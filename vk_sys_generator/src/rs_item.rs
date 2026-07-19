use std::collections::BTreeSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeatureName<'s> {
    Raw(&'s str),
    VulkanExt { tag: &'s str, name: &'s str },
    AllowApiVersion(&'s str),
}
impl core::fmt::Display for FeatureName<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(x) => f.write_str(x),
            Self::VulkanExt { tag, name } => write!(f, "VK_{tag}_{name}"),
            Self::AllowApiVersion(v) => write!(f, "Allow{v}APIs"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompilationCondition<'s> {
    Empty,
    /// 原則使わない(available_conditionsの仕組みは将来的に廃止する)
    Raw(&'s str),
    Feature(FeatureName<'s>),
    All(BTreeSet<CompilationCondition<'s>>),
    Any(BTreeSet<CompilationCondition<'s>>),
}
impl CompilationCondition<'_> {
    fn emit_content(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        match self {
            Self::Empty => Ok(()),
            Self::Raw(s) => w.write_all(s.as_bytes()),
            Self::Feature(f) => write!(w, "feature = \"{f}\""),
            Self::All(xs) => {
                w.write_all(b"all(")?;
                let mut first = true;
                for x in xs {
                    if !first {
                        w.write_all(b", ")?;
                    }

                    x.emit_content(w)?;
                    first = false;
                }
                w.write_all(b")")
            }
            Self::Any(xs) => {
                w.write_all(b"any(")?;
                let mut first = true;
                for x in xs {
                    if !first {
                        w.write_all(b", ")?;
                    }

                    x.emit_content(w)?;
                    first = false;
                }
                w.write_all(b")")
            }
        }
    }

    pub fn emit_attr_content(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        w.write_all(b"cfg(")?;
        self.emit_content(w)?;
        w.write_all(b")")
    }

    pub fn emit_single_attr(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        if matches!(self, Self::Empty) {
            // no emit for empty
            return Ok(());
        }

        w.write_all(b"#[")?;
        self.emit_attr_content(w)?;
        w.write_all(b"]")
    }

    #[inline]
    pub fn all(xs: impl IntoIterator<Item = Self>) -> Self {
        Self::All(FromIterator::from_iter(xs)).reduce()
    }

    #[inline]
    pub fn any(xs: impl IntoIterator<Item = Self>) -> Self {
        Self::Any(FromIterator::from_iter(xs)).reduce()
    }

    pub fn reduce(self) -> Self {
        match self {
            Self::All(mut xs) => match xs.len() {
                0 => Self::Empty,
                1 => unsafe { xs.pop_first().unwrap_unchecked() },
                _ => Self::All(xs),
            },
            Self::Any(mut xs) => match xs.len() {
                0 => Self::Empty,
                1 => unsafe { xs.pop_first().unwrap_unchecked() },
                _ => Self::Any(xs),
            },
            a => a,
        }
    }

    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Empty, other) => other,
            (this, Self::Empty) => this,
            (Self::All(mut xs), Self::All(ys)) => {
                xs.extend(ys);
                Self::All(xs)
            }
            (Self::All(mut xs), y) => {
                xs.insert(y);
                Self::All(xs)
            }
            (x, Self::All(mut ys)) => {
                ys.insert(x);
                Self::All(ys)
            }
            (a, b) => Self::all([a, b]),
        }
    }

    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::Empty, other) => other,
            (this, Self::Empty) => this,
            (Self::Any(mut xs), Self::Any(ys)) => {
                xs.extend(ys);
                Self::Any(xs)
            }
            (Self::All(mut xs), Self::All(mut ys)) => {
                // any(all(a, b), all(a, c)) => all(a, any(b, c))
                let mut new_sets = BTreeSet::new();
                new_sets.extend(xs.extract_if(.., |e| ys.contains(e)));
                for c in new_sets.iter() {
                    ys.remove(c);
                }

                new_sets.insert(Self::any(xs.into_iter().chain(ys)));
                Self::All(new_sets)
            }
            (Self::Any(mut xs), y) => {
                xs.insert(y);
                Self::Any(xs)
            }
            (x, Self::Any(mut ys)) => {
                ys.insert(x);
                Self::Any(ys)
            }
            (a, b) => Self::any([a, b]),
        }
    }
}

pub enum FixedArrayLength<'s> {
    Imm(usize),
    Named(&'s str),
}

pub enum Type<'s> {
    Raw(&'s str),
    ConstPtr(Box<Type<'s>>),
    MutPtr(Box<Type<'s>>),
    FixedArray(Box<Type<'s>>, FixedArrayLength<'s>),
}
impl Type<'_> {
    pub fn emit(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        match self {
            Self::Raw(r) => w.write_all(r.as_bytes()),
            Self::ConstPtr(t) => {
                w.write_all(b"*const ")?;
                t.emit(w)
            }
            Self::MutPtr(t) => {
                w.write_all(b"*mut ")?;
                t.emit(w)
            }
            Self::FixedArray(t, FixedArrayLength::Imm(n)) => {
                w.write_all(b"[")?;
                t.emit(w)?;
                write!(w, "; {n}]")
            }
            Self::FixedArray(t, FixedArrayLength::Named(n)) => {
                w.write_all(b"[")?;
                t.emit(w)?;
                write!(w, "; {n}]")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FnSymbol<'s> {
    Raw(&'s str),
    Suffixed { stem: &'s str, suffix: &'s str },
    Cmd(&'s str),
    CmdSuffixed { stem: &'s str, suffix: &'s str },
}
impl core::fmt::Display for FnSymbol<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(r) => write!(f, "vk{r}"),
            Self::Suffixed { stem, suffix } => write!(f, "vk{stem}{suffix}"),
            Self::Cmd(r) => write!(f, "vkCmd{r}"),
            Self::CmdSuffixed { stem, suffix } => write!(f, "vkCmd{stem}{suffix}"),
        }
    }
}

pub struct FunctionStub<'s, Args> {
    pub compilation_condition: CompilationCondition<'s>,
    pub name: FnSymbol<'s>,
    pub args: Args,
    pub return_type: Option<Type<'s>>,
}
impl<'s, Args: Iterator<Item = (&'s str, Type<'s>)>> FunctionStub<'s, Args> {
    pub fn emit(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(w, "pub fn {}(", self.name)?;
        let mut first = true;
        for a in self.args {
            if !first {
                w.write_all(b", ")?;
            }

            w.write_all(a.0.as_bytes())?;
            w.write_all(b": ")?;
            a.1.emit(w)?;
            first = false;
        }
        w.write_all(b")")?;
        if let Some(r) = self.return_type {
            w.write_all(b" -> ")?;
            r.emit(w)?;
        }
        w.write_all(b";")?;

        Ok(())
    }
}

pub struct StaticCallableImpl<'s> {
    pub compilation_condition: CompilationCondition<'s>,
    pub fn_name: FnSymbol<'s>,
}
impl StaticCallableImpl<'_> {
    pub fn emit(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[cfg(all(feature = \"Implements\", not(feature = \"DynamicLoaded\")))] #[rustfmt::skip] impl crate::StaticCallable for PFN_{n} {{ const STATIC: Self = Self({n}); }}",
            n = self.fn_name
        )?;

        Ok(())
    }
}

pub struct FromPtrImpl<'s> {
    pub compilation_condition: CompilationCondition<'s>,
    pub fn_name: FnSymbol<'s>,
}
impl FromPtrImpl<'_> {
    pub fn emit(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[rustfmt::skip] unsafe impl crate::FromPtr for PFN_{n} {{ #[inline(always)] unsafe fn from_ptr(p: *const core::ffi::c_void) -> Self {{ unsafe {{ core::mem::transmute::<*const core::ffi::c_void, Self>(p) }} }} }}",
            n = self.fn_name
        )?;

        Ok(())
    }
}

pub struct PFNImpl<'s> {
    pub compilation_condition: CompilationCondition<'s>,
    pub fn_name: FnSymbol<'s>,
}
impl PFNImpl<'_> {
    pub fn emit(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[rustfmt::skip] unsafe impl crate::PFN for PFN_{n} {{ const NAME_CSTR: &'static core::ffi::CStr = c\"{n}\"; #[inline(always)] unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {{ unsafe {{ core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }} }} }}",
            n = self.fn_name
        )?;

        Ok(())
    }
}

pub struct FunctionPtrNewtype<'s, Args> {
    pub compilation_condition: CompilationCondition<'s>,
    pub name: FnSymbol<'s>,
    pub args: Args,
    pub return_type: Option<Type<'s>>,
}
impl<'s, Args: Iterator<Item = (&'s str, Type<'s>)>> FunctionPtrNewtype<'s, Args> {
    pub fn emit(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[repr(transparent)] #[derive(Debug, Clone, Copy)] #[rustfmt::skip] pub struct PFN_{}(pub unsafe extern \"system\" fn(",
            self.name
        )?;
        let mut first = true;
        for x in self.args {
            if !first {
                w.write_all(b", ")?;
            }

            w.write_all(x.0.as_bytes())?;
            w.write_all(b": ")?;
            x.1.emit(w)?;
            first = false;
        }
        w.write_all(b")")?;
        if let Some(r) = self.return_type {
            w.write_all(b" -> ")?;
            r.emit(w)?;
        }
        w.write_all(b");")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstantSymbol<'s> {
    StructureType(&'s str),
}
impl core::fmt::Display for ConstantSymbol<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StructureType(s) => write!(f, "VK_STRUCTURE_TYPE_{s}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConstantValue {
    Signed(i64),
    Unsigned(u64),
}
impl core::fmt::Display for ConstantValue {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signed(v) => write!(f, "{v}"),
            Self::Unsigned(v) => write!(f, "{v}"),
        }
    }
}

pub struct Constant<'s> {
    pub compilation_condition: CompilationCondition<'s>,
    pub name: ConstantSymbol<'s>,
    pub ty: Type<'s>,
    pub value: ConstantValue,
}
impl Constant<'_> {
    pub fn emit(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(w, "#[rustfmt::skip] pub const {}: ", self.name)?;
        self.ty.emit(w)?;
        write!(w, " = {};", self.value)?;

        Ok(())
    }
}
