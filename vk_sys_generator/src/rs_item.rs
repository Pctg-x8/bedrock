use std::collections::BTreeSet;

use bitflags::bitflags;

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
    #[deprecated = "原則使わない(available_conditionsの仕組みは将来的に廃止する)"]
    Raw(&'s str),
    Feature(FeatureName<'s>),
    All(BTreeSet<CompilationCondition<'s>>),
    Any(BTreeSet<CompilationCondition<'s>>),
}
impl CompilationCondition<'_> {
    fn emit_content(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
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

    pub fn emit_attr_content(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        w.write_all(b"cfg(")?;
        self.emit_content(w)?;
        w.write_all(b")")
    }

    pub fn emit_single_attr(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
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

#[derive(Clone)]
pub enum FixedArrayLength<'s> {
    Imm(usize),
    Named(&'s str),
}

#[derive(Clone)]
pub enum Type<'s> {
    Raw(&'s str),
    Defined(TypeSymbol<'s>),
    ConstPtr(Box<Type<'s>>),
    MutPtr(Box<Type<'s>>),
    FixedArray(Box<Type<'s>>, FixedArrayLength<'s>),
}
impl core::fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(r) => f.write_str(r),
            Self::Defined(t) => t.fmt(f),
            Self::ConstPtr(t) => write!(f, "*const {t}"),
            Self::MutPtr(t) => write!(f, "*mut {t}"),
            Self::FixedArray(t, FixedArrayLength::Imm(n)) => write!(f, "[{t}; {n}]"),
            Self::FixedArray(t, FixedArrayLength::Named(n)) => write!(f, "[{t}; {n}]"),
        }
    }
}
impl Type<'_> {
    pub fn const_ptr(self) -> Self {
        Self::ConstPtr(Box::new(self))
    }

    pub fn mut_ptr(self) -> Self {
        Self::MutPtr(Box::new(self))
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

pub struct FunctionStub<'s> {
    pub compilation_condition: CompilationCondition<'s>,
    pub name: FnSymbol<'s>,
    pub args: Vec<(&'s str, Type<'s>)>,
    pub return_type: Option<Type<'s>>,
}
impl FunctionStub<'_> {
    pub fn emit(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(w, "pub fn {}(", self.name)?;
        let mut first = true;
        for a in &self.args {
            if !first {
                w.write_all(b",")?;
            }

            write!(w, "{}:{}", a.0, a.1)?;
            first = false;
        }
        w.write_all(b")")?;
        if let Some(r) = self.return_type {
            write!(w, "->{r}")?;
        }
        w.write_all(b";")?;

        Ok(())
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct FunctionPtrNewtypeDerives : u8 {
        const FROM_PTR = 0x01;
        const PFN = 0x02;
        const STATIC_CALLABLE = 0x04;
    }
}

pub struct FunctionPtrNewtype<'s> {
    pub compilation_condition: CompilationCondition<'s>,
    pub name: FnSymbol<'s>,
    pub args: Vec<(&'s str, Type<'s>)>,
    pub return_type: Option<Type<'s>>,
    pub derives: FunctionPtrNewtypeDerives,
}
impl FunctionPtrNewtype<'_> {
    pub fn emit(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_{}(pub unsafe extern \"system\" fn(",
            self.name
        )?;
        let mut first = true;
        for x in &self.args {
            if !first {
                w.write_all(b",")?;
            }

            write!(w, "{}:{}", x.0, x.1)?;
            first = false;
        }
        w.write_all(b")")?;
        if let Some(ref r) = self.return_type {
            write!(w, "->{r}")?;
        }
        w.write_all(b");")?;

        if self.derives.contains(FunctionPtrNewtypeDerives::FROM_PTR) {
            w.write_all(b"\n")?;
            self.emit_from_ptr(w)?;
        }
        if self.derives.contains(FunctionPtrNewtypeDerives::PFN) {
            w.write_all(b"\n")?;
            self.emit_pfn(w)?;
        }
        if self.derives.contains(FunctionPtrNewtypeDerives::STATIC_CALLABLE) {
            w.write_all(b"\n")?;
            self.emit_static_callable(w)?;
        }

        Ok(())
    }

    fn emit_from_ptr(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_{n}{{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{{unsafe{{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}}}}",
            n = self.name
        )
    }

    fn emit_pfn(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[rustfmt::skip]unsafe impl crate::PFN for PFN_{n}{{const NAME_CSTR:&'static core::ffi::CStr=c\"{n}\";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{{unsafe{{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}}}}",
            n = self.name
        )
    }

    fn emit_static_callable(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[cfg(all(feature=\"Implements\",not(feature=\"DynamicLoaded\")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_{n}{{const STATIC:Self=Self({n});}}",
            n = self.name
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstantSymbol<'s> {
    Enum {
        prefix: &'s str,
        stem: &'s str,
        suffix: Option<&'s str>,
    },
    Bitmask {
        prefix: &'s str,
        stem: &'s str,
        suffix: Option<&'s str>,
    },
}
impl core::fmt::Display for ConstantSymbol<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enum {
                prefix,
                stem,
                suffix: None,
            } => write!(f, "VK_{prefix}_{stem}"),
            Self::Enum {
                prefix,
                stem,
                suffix: Some(suffix),
            } => write!(f, "VK_{prefix}_{stem}_{suffix}"),
            Self::Bitmask {
                prefix,
                stem,
                suffix: None,
            } => write!(f, "VK_{prefix}_{stem}_BIT"),
            Self::Bitmask {
                prefix,
                stem,
                suffix: Some(suffix),
            } => write!(f, "VK_{prefix}_{stem}_BIT_{suffix}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConstantValue<'s> {
    UnsignedLong(u64),
    Signed(isize),
    SignedNewtyped { ctor: TypeSymbol<'s>, value: isize },
    Bits64(u64),
    Bits32(u32),
}
impl core::fmt::Display for ConstantValue<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsignedLong(v) => write!(f, "{v}"),
            Self::Signed(v) => write!(f, "{v}"),
            Self::SignedNewtyped { ctor, value } => write!(f, "{ctor}({value})"),
            // +2 for heading "0x"
            Self::Bits64(v) => write!(f, "{v:#018x}"),
            Self::Bits32(v) => write!(f, "{v:#010x}"),
        }
    }
}

pub struct Constant<'s> {
    pub compilation_condition: CompilationCondition<'s>,
    pub name: ConstantSymbol<'s>,
    pub ty: Type<'s>,
    pub value: ConstantValue<'s>,
}
impl Constant<'_> {
    pub fn emit(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(w, "#[rustfmt::skip]pub const {}:{}={};", self.name, self.ty, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeSymbol<'s> {
    pub stem: &'s str,
    pub suffix: Option<&'s str>,
}
impl core::fmt::Display for TypeSymbol<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vk{}", self.stem)?;
        if let Some(suffix) = self.suffix {
            f.write_str(suffix)?;
        }
        Ok(())
    }
}

bitflags! {
    pub struct StructDerives : u8 {
        const DEBUG = 0x01;
        const CLONE = 0x02;
        const COPY = 0x04;
        const EQ = 0x08;
        const HASH = 0x10;
        const VULKAN_STRUCTURE = 0x20;
        const VULKAN_SINK_STRUCTURE = 0x40;
    }
}

pub struct StructMember<'s> {
    pub name: &'s str,
    pub ty: Type<'s>,
}

#[derive(Debug)]
pub enum StructDefault<'s> {
    None,
    Zero,
    ZeroTyped(ConstantSymbol<'s>),
}

pub struct Struct<'s> {
    pub compilation_condition: CompilationCondition<'s>,
    pub name: TypeSymbol<'s>,
    pub derives: StructDerives,
    pub members: Vec<StructMember<'s>>,
    pub typed_vulkan_structure_impl: Option<StructTypedVulkanStructureImpl<'s>>,
    pub typed_vulkan_sink_structure_impl: Option<StructTypedVulkanSinkStructureImpl<'s>>,
    pub default: StructDefault<'s>,
}
impl Struct<'_> {
    pub fn emit(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        if !self.derives.is_empty() {
            write!(w, "#[derive(")?;
            if self.derives.contains(StructDerives::DEBUG) {
                write!(w, "Debug,")?;
            }
            if self.derives.contains(StructDerives::CLONE) {
                write!(w, "Clone,")?;
            }
            if self.derives.contains(StructDerives::COPY) {
                write!(w, "Copy,")?;
            }
            if self.derives.contains(StructDerives::EQ) {
                write!(w, "PartialEq,Eq,")?;
            }
            if self.derives.contains(StructDerives::HASH) {
                write!(w, "Hash,")?;
            }
            write!(w, ")]")?;
        }
        write!(w, "#[rustfmt::skip]#[repr(C)]pub struct {}{{", self.name)?;
        for member in self.members.iter() {
            write!(w, "pub {}:{},", member.name, member.ty)?;
        }
        write!(w, "}}")?;

        if self.derives.contains(StructDerives::VULKAN_STRUCTURE) {
            w.write_all(b"\n")?;
            self.emit_vulkan_structure_impl(w)?;
        }
        if self.derives.contains(StructDerives::VULKAN_SINK_STRUCTURE) {
            w.write_all(b"\n")?;
            self.emit_vulkan_sink_structure_impl(w)?;
        }

        if let Some(ref x) = self.typed_vulkan_structure_impl {
            w.write_all(b"\n")?;
            x.emit(self, w)?;
        }
        if let Some(ref x) = self.typed_vulkan_sink_structure_impl {
            w.write_all(b"\n")?;
            x.emit(self, w)?;
        }

        match self.default {
            StructDefault::None => (),
            StructDefault::Zero => {
                w.write_all(b"\n")?;
                self.emit_default_zero(w)?;
            }
            StructDefault::ZeroTyped(ref sty) => {
                w.write_all(b"\n")?;
                self.emit_default_zero_typed(sty, w)?;
            }
        }

        Ok(())
    }

    fn emit_vulkan_structure_impl(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[rustfmt::skip]unsafe impl crate::VulkanStructure for {}{{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{{unsafe{{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{{unsafe{{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}}}}",
            self.name
        )?;

        Ok(())
    }

    fn emit_vulkan_sink_structure_impl(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for {}{{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{{unsafe{{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{{unsafe{{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}}}}",
            self.name
        )?;

        Ok(())
    }

    fn emit_default_zero(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[rustfmt::skip]impl Default for {}{{#[inline(always)]fn default()->Self{{unsafe{{core::mem::MaybeUninit::zeroed().assume_init()}}}}}}",
            self.name
        )?;

        Ok(())
    }

    fn emit_default_zero_typed(
        &self,
        sty: &ConstantSymbol<'_>,
        w: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[rustfmt::skip]impl Default for {}{{#[inline(always)]fn default()->Self{{let mut p=core::mem::MaybeUninit::<Self>::zeroed();unsafe{{core::ptr::addr_of_mut!((*p.as_mut_ptr()).sType).write({sty});p.assume_init()}}}}}}",
            self.name
        )?;

        Ok(())
    }
}

pub struct StructTypedVulkanStructureImpl<'s> {
    pub const_name: ConstantSymbol<'s>,
}
impl StructTypedVulkanStructureImpl<'_> {
    fn emit(&self, source_ty: &Struct<'_>, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        source_ty.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[rustfmt::skip]impl crate::TypedVulkanStructure for {}{{const TYPE: VkStructureType={};}}",
            source_ty.name, self.const_name
        )
    }
}

pub struct StructTypedVulkanSinkStructureImpl<'s> {
    pub const_name: ConstantSymbol<'s>,
}
impl StructTypedVulkanSinkStructureImpl<'_> {
    fn emit(&self, source_ty: &Struct<'_>, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        source_ty.compilation_condition.emit_single_attr(w)?;
        write!(
            w,
            "#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for {}{{const TYPE: VkStructureType={};}}",
            source_ty.name, self.const_name
        )
    }
}

pub struct TypeAlias<'s> {
    pub compilation_condition: CompilationCondition<'s>,
    pub target_name: TypeSymbol<'s>,
    pub source_name: Type<'s>,
}
impl TypeAlias<'_> {
    pub fn emit(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        self.compilation_condition.emit_single_attr(w)?;
        write!(w, "#[rustfmt::skip]pub type {}={};", self.target_name, self.source_name)
    }
}

pub trait RustCodeEmitter {
    fn emit_const(&mut self, e: Constant<'static>);
    fn emit_struct(&mut self, e: Struct<'static>);
    fn emit_type_alias(&mut self, e: TypeAlias<'static>);
    fn emit_function_ptr_newtype(&mut self, e: FunctionPtrNewtype<'static>);
    fn emit_function_stub(&mut self, e: FunctionStub<'static>);
}
