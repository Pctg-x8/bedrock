use std::collections::{HashMap, HashSet};

use crate::rs_item::{
    CompilationCondition, Constant, ConstantSymbol, ConstantValue, FeatureName, FnSymbol, FunctionPtrNewtype,
    FunctionPtrNewtypeDerives, FunctionStub, RustCodeEmitter, StructDefault, StructDerives,
    StructTypedVulkanSinkStructureImpl, StructTypedVulkanStructureImpl, Type, TypeSymbol,
};

pub const TY_VK_BOOL: &str = "VkBool32";

pub const fn ex_khr(name: &'static str) -> (&'static str, &'static str) {
    ("KHR", name)
}
pub const fn ex_ext(name: &'static str) -> (&'static str, &'static str) {
    ("EXT", name)
}

pub fn emit_type_alias(w: &mut impl std::io::Write, name: &str, alias_type: &str) -> std::io::Result<()> {
    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(w, "pub type {name} = {alias_type};")
}

pub fn emit_c_enum_type(w: &mut impl std::io::Write, name: &str) -> std::io::Result<()> {
    emit_type_alias(w, name, "i32")
}

pub fn emit_const(w: &mut impl std::io::Write, name: &str, r#type: &str, value: &str) -> std::io::Result<()> {
    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(w, "pub const {name}: {type} = {value};")
}

#[allow(clippy::inconsistent_digit_grouping)]
pub fn emit_result_const(w: &mut impl std::io::Write, name: &str, ext_number: u32, value: u32) -> std::io::Result<()> {
    let value = if ext_number == 0 {
        value
    } else {
        100_0000_000 + (ext_number - 1) * 1_000 + value
    };

    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(w, "pub const {name}: VkResult = VkResult({value});")
}

#[allow(clippy::inconsistent_digit_grouping)]
pub fn emit_result_err_const(
    w: &mut impl std::io::Write,
    name: &str,
    ext_number: u32,
    value: u32,
) -> std::io::Result<()> {
    let value = if ext_number == 0 {
        value
    } else {
        100_0000_000 + (ext_number - 1) * 1_000 + value
    };

    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(w, "pub const {name}: VkResult = VkResult(-{value});")
}

pub struct TypeAlias {
    name: &'static str,
    org_type: &'static str,
}
impl TypeAlias {
    pub const fn new(name: &'static str, org_type: &'static str) -> Self {
        Self { name, org_type }
    }

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(w, "pub type {} = {};", self.name, self.org_type)
    }
}

pub struct FuncPointer {
    name: &'static str,
    args: &'static [(&'static str, &'static str)],
    return_ty: Option<&'static str>,
    extension: Option<(&'static str, &'static str)>,
}
impl FuncPointer {
    pub const fn new(name: &'static str, args: &'static [(&'static str, &'static str)]) -> Self {
        Self {
            name,
            args,
            return_ty: None,
            extension: None,
        }
    }

    pub const fn returns(mut self, t: &'static str) -> Self {
        self.return_ty = Some(t);
        self
    }

    pub const fn extension(mut self, tag: &'static str, name: &'static str) -> Self {
        self.extension = Some((tag, name));
        self
    }

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        if let Some((tag, name)) = self.extension {
            writeln!(w, "#[cfg(feature = \"VK_{tag}_{name}\")]")?;
            writeln!(w, "#[rustfmt::skip]")?;
            write!(w, "pub type PFN_vk{}{tag} = extern \"system\" fn(", self.name)?;
        } else {
            writeln!(w, "#[rustfmt::skip]")?;
            write!(w, "pub type PFN_vk{} = extern \"system\" fn(", self.name)?;
        }
        let mut cont = false;
        for (n, t) in self.args {
            if cont {
                w.write_all(b", ")?;
            }
            write!(w, "{n}: {t}")?;
            cont = true;
        }
        if let Some(t) = self.return_ty {
            writeln!(w, ") -> {t};")
        } else {
            writeln!(w, ");")
        }
    }
}

pub struct EnumMember {
    name: &'static str,
    value: isize,
    extension_old: Option<(&'static str, &'static str)>,
    extension: Option<&'static Extension<'static>>,
    promoted: Option<&'static str>,
}
impl EnumMember {
    #[deprecated = "use new extension specifier"]
    pub const fn extension_old(mut self, suffix: &'static str, name: &'static str) -> Self {
        self.extension_old = Some((name, suffix));
        self
    }

    pub const fn extension(mut self, ext: &'static Extension<'static>) -> Self {
        self.extension = Some(ext);
        self
    }

    pub const fn promoted(mut self, version: &'static str) -> Self {
        self.promoted = Some(version);
        self
    }
}

pub struct Enum {
    name: &'static str,
    extending: bool,
    prefix: &'static str,
    members: &'static [EnumMember],
    extension_old: Option<(&'static str, &'static str)>,
    extension: Option<&'static Extension<'static>>,
    promoted: Option<&'static str>,
}
impl Enum {
    pub const fn new(name: &'static str, prefix: &'static str, members: &'static [EnumMember]) -> Self {
        Self {
            name,
            prefix,
            extending: false,
            members,
            extension_old: None,
            extension: None,
            promoted: None,
        }
    }

    pub const fn extending(name: &'static str, prefix: &'static str, members: &'static [EnumMember]) -> Self {
        Self {
            name,
            prefix,
            extending: true,
            members,
            extension_old: None,
            extension: None,
            promoted: None,
        }
    }

    pub const fn extending_error(members: &'static [EnumMember]) -> Self {
        Self::extending("Result", "ERROR", members)
    }

    #[deprecated = "use new extension specifier"]
    pub const fn extension_old(mut self, suffix: &'static str, name: &'static str) -> Self {
        self.extension_old = Some((name, suffix));
        self
    }

    pub const fn extension(mut self, extension: &'static Extension<'static>) -> Self {
        self.extension = Some(extension);
        self
    }

    pub const fn promoted(mut self, version: &'static str) -> Self {
        self.promoted = Some(version);
        self
    }

    pub const fn member(name: &'static str, value: isize) -> EnumMember {
        EnumMember {
            name,
            value,
            extension_old: None,
            extension: None,
            promoted: None,
        }
    }

    pub const fn into_element(self) -> Element {
        Element::Enum(self)
    }

    pub fn emit(&self, emitter: &mut (impl RustCodeEmitter + ?Sized)) {
        let type_sym = if let Some(x) = self.extension {
            TypeSymbol {
                stem: self.name,
                suffix: Some(x.tag),
            }
        } else if let Some((_, s)) = self.extension_old {
            TypeSymbol {
                stem: self.name,
                suffix: Some(s),
            }
        } else {
            TypeSymbol {
                stem: self.name,
                suffix: None,
            }
        };
        let is_newtyped = type_sym
            == TypeSymbol {
                stem: "Result",
                suffix: None,
            };
        let ty_promoted = self
            .promoted
            .or(self.extension.map(|x| x.promoted_since).filter(|x| !x.is_empty()));

        if !self.extending {
            // not extending(first seen in this definition)
            emitter.emit_type_alias(crate::rs_item::TypeAlias {
                compilation_condition: CompilationCondition::all(
                    [
                        self.extension.map(|x| {
                            CompilationCondition::Feature(FeatureName::VulkanExt {
                                tag: x.tag,
                                name: x.name,
                            })
                        }),
                        self.extension_old
                            .map(|(name, tag)| CompilationCondition::Feature(FeatureName::VulkanExt { tag, name })),
                    ]
                    .into_iter()
                    .flatten(),
                ),
                target_name: type_sym.clone(),
                source_name: Type::Raw("i32"),
            });
            if let Some(v) = ty_promoted {
                emitter.emit_type_alias(crate::rs_item::TypeAlias {
                    compilation_condition: CompilationCondition::Feature(FeatureName::AllowApiVersion(v)),
                    target_name: TypeSymbol {
                        stem: self.name,
                        suffix: None,
                    },
                    source_name: Type::Raw("i32"),
                });
            }
        }

        for member in self.members {
            let m_promoted = member
                .promoted
                .or(member.extension.map(|x| x.promoted_since).filter(|x| !x.is_empty()));

            let ty = Type::Defined(type_sym.clone());
            let value = if is_newtyped {
                ConstantValue::SignedNewtyped {
                    ctor: type_sym.clone(),
                    value: member.value,
                }
            } else {
                ConstantValue::Signed(member.value)
            };

            if self.extension.is_some() || member.extension.is_some() {
                // use ext2
                emitter.emit_const(Constant {
                    compilation_condition: CompilationCondition::all(
                        [
                            self.extension.map(|x| {
                                CompilationCondition::Feature(FeatureName::VulkanExt {
                                    tag: x.tag,
                                    name: x.name,
                                })
                            }),
                            member.extension.map(|x| {
                                CompilationCondition::Feature(FeatureName::VulkanExt {
                                    tag: x.tag,
                                    name: x.name,
                                })
                            }),
                        ]
                        .into_iter()
                        .flatten(),
                    ),
                    name: ConstantSymbol::Enum {
                        prefix: self.prefix,
                        stem: member.name,
                        // prefer member's extension tag(overriding)
                        suffix: member.extension.map(|x| x.tag).or(self.extension.map(|x| x.tag)),
                    },
                    ty: ty.clone(),
                    value: value.clone(),
                });
            } else {
                // use old(deprecated) extension specifier
                emitter.emit_const(Constant {
                    compilation_condition: CompilationCondition::all(
                        [
                            self.extension_old
                                .map(|(name, tag)| CompilationCondition::Feature(FeatureName::VulkanExt { tag, name })),
                            member
                                .extension_old
                                .map(|(name, tag)| CompilationCondition::Feature(FeatureName::VulkanExt { tag, name })),
                        ]
                        .into_iter()
                        .flatten(),
                    ),
                    name: ConstantSymbol::Enum {
                        prefix: self.prefix,
                        stem: member.name,
                        // prefer member's extension tag(overriding)
                        suffix: member.extension_old.map(|x| x.1).or(self.extension_old.map(|x| x.1)),
                    },
                    ty: ty.clone(),
                    value: value.clone(),
                });
            }

            if let Some(v) = m_promoted {
                emitter.emit_const(Constant {
                    compilation_condition: CompilationCondition::all(
                        [
                            self.extension_old
                                .map(|(name, tag)| CompilationCondition::Feature(FeatureName::VulkanExt { name, tag })),
                            self.extension.map(|x| {
                                CompilationCondition::Feature(FeatureName::VulkanExt {
                                    tag: x.tag,
                                    name: x.name,
                                })
                            }),
                            Some(CompilationCondition::Feature(FeatureName::AllowApiVersion(v))),
                        ]
                        .into_iter()
                        .flatten(),
                    ),
                    name: ConstantSymbol::Enum {
                        prefix: self.prefix,
                        stem: member.name,
                        suffix: None,
                    },
                    ty: Type::Defined(TypeSymbol {
                        stem: self.name,
                        suffix: None,
                    }),
                    value,
                });
            }
        }
    }
}

pub struct BitmaskEntry {
    name: &'static str,
    bitpos: usize,
    extension_old: Option<(&'static str, &'static str)>,
    extension: Option<&'static Extension<'static>>,
    side_extensions: &'static [&'static Extension<'static>],
    promoted: Option<&'static str>,
    version_since: Option<&'static str>,
}
impl BitmaskEntry {
    #[deprecated = "use new extension specifier"]
    pub const fn extension_old(mut self, tag: &'static str, name: &'static str) -> Self {
        self.extension_old = Some((tag, name));
        self
    }

    pub const fn extension(mut self, ext: &'static Extension<'static>) -> Self {
        self.extension = Some(ext);
        self
    }

    pub const fn side_extensions(mut self, exts: &'static [&'static Extension<'static>]) -> Self {
        self.side_extensions = exts;
        self
    }

    pub const fn promoted(mut self, name: &'static str) -> Self {
        self.promoted = Some(name);
        self
    }

    pub const fn version_since(mut self, v: &'static str) -> Self {
        self.version_since = Some(v);
        self
    }
}

pub struct Bitmask {
    name: &'static str,
    bits_name: &'static str,
    prefix: &'static str,
    extending: bool,
    entries: &'static [BitmaskEntry],
    version_since: Option<&'static str>,
    extension_old: Option<(&'static str, &'static str)>,
    extension: Option<&'static Extension<'static>>,
    side_extensions: &'static [&'static Extension<'static>],
    promoted: Option<&'static str>,
    long: bool,
}
impl Bitmask {
    pub const fn new(
        name: &'static str,
        bits_name: &'static str,
        prefix: &'static str,
        entries: &'static [BitmaskEntry],
    ) -> Self {
        Self {
            name,
            bits_name,
            prefix,
            extending: false,
            entries,
            version_since: None,
            extension_old: None,
            extension: None,
            side_extensions: &[],
            promoted: None,
            long: false,
        }
    }

    pub const fn extending(bits_name: &'static str, prefix: &'static str, entries: &'static [BitmaskEntry]) -> Self {
        Self {
            name: "",
            bits_name,
            prefix,
            extending: true,
            entries,
            version_since: None,
            extension_old: None,
            extension: None,
            side_extensions: &[],
            promoted: None,
            long: false,
        }
    }

    pub const fn long(mut self) -> Self {
        self.long = true;
        self
    }

    pub const fn version_since(mut self, v: &'static str) -> Self {
        self.version_since = Some(v);
        self
    }

    #[deprecated = "use new extension specifier"]
    pub const fn extension_old(mut self, tag: &'static str, name: &'static str) -> Self {
        self.extension_old = Some((tag, name));
        self
    }

    pub const fn extension(mut self, ext: &'static Extension<'static>) -> Self {
        self.extension = Some(ext);
        self
    }

    pub const fn side_extensions(mut self, exts: &'static [&'static Extension<'static>]) -> Self {
        self.side_extensions = exts;
        self
    }

    pub const fn promoted(mut self, promoted: &'static str) -> Self {
        self.promoted = Some(promoted);
        self
    }

    pub const fn entry(name: &'static str, bitpos: usize) -> BitmaskEntry {
        BitmaskEntry {
            name,
            bitpos,
            extension_old: None,
            extension: None,
            side_extensions: &[],
            promoted: None,
            version_since: None,
        }
    }

    pub const fn into_element(self) -> Element {
        Element::Bitmask(self)
    }

    pub fn emit(&self, emitter: &mut (impl RustCodeEmitter + ?Sized)) {
        let bits_sym = TypeSymbol {
            stem: self.bits_name,
            suffix: self.extension.map(|x| x.tag).or(self.extension_old.map(|(tag, _)| tag)),
        };

        if !self.extending {
            // not extending(first seen in this definition)
            let cond = CompilationCondition::all(
                [
                    self.version_since
                        .map(|v| CompilationCondition::Feature(FeatureName::AllowApiVersion(v))),
                    self.extension_old
                        .map(|(tag, name)| CompilationCondition::Feature(FeatureName::VulkanExt { tag, name })),
                    self.extension.map(|x| {
                        CompilationCondition::Feature(FeatureName::VulkanExt {
                            tag: x.tag,
                            name: x.name,
                        })
                    }),
                ]
                .into_iter()
                .flatten()
                .chain(self.side_extensions.iter().map(|x| {
                    CompilationCondition::Feature(FeatureName::VulkanExt {
                        tag: x.tag,
                        name: x.name,
                    })
                })),
            );
            let source_ty = if self.long {
                Type::Raw("VkFlags64")
            } else {
                Type::Raw("VkFlags")
            };

            emitter.emit_type_alias(crate::rs_item::TypeAlias {
                compilation_condition: cond.clone(),
                target_name: TypeSymbol {
                    stem: self.name,
                    suffix: self.extension.map(|x| x.tag).or(self.extension_old.map(|(tag, _)| tag)),
                },
                source_name: source_ty.clone(),
            });
            emitter.emit_type_alias(crate::rs_item::TypeAlias {
                compilation_condition: cond,
                target_name: bits_sym.clone(),
                source_name: source_ty.clone(),
            });

            if let Some(p) = self.promoted {
                let cond = CompilationCondition::Feature(FeatureName::AllowApiVersion(p)).and(
                    CompilationCondition::all(self.side_extensions.iter().map(|x| {
                        CompilationCondition::Feature(FeatureName::VulkanExt {
                            tag: x.tag,
                            name: x.name,
                        })
                    })),
                );

                emitter.emit_type_alias(crate::rs_item::TypeAlias {
                    compilation_condition: cond.clone(),
                    target_name: TypeSymbol {
                        stem: self.name,
                        suffix: None,
                    },
                    source_name: source_ty.clone(),
                });
                emitter.emit_type_alias(crate::rs_item::TypeAlias {
                    compilation_condition: cond,
                    target_name: TypeSymbol {
                        stem: self.bits_name,
                        suffix: None,
                    },
                    source_name: source_ty,
                });
            }
        }

        for e in self.entries {
            let cond = CompilationCondition::all(
                self.version_since
                    .map(|v| CompilationCondition::Feature(FeatureName::AllowApiVersion(v)))
                    .into_iter()
                    .chain(
                        e.version_since
                            .map(|v| CompilationCondition::Feature(FeatureName::AllowApiVersion(v))),
                    )
                    .chain(
                        e.extension_old
                            .map(|(tag, name)| CompilationCondition::Feature(FeatureName::VulkanExt { tag, name })),
                    )
                    .chain(e.extension.map(|x| {
                        CompilationCondition::Feature(FeatureName::VulkanExt {
                            tag: x.tag,
                            name: x.name,
                        })
                    }))
                    .chain(e.side_extensions.iter().map(|x| {
                        CompilationCondition::Feature(FeatureName::VulkanExt {
                            tag: x.tag,
                            name: x.name,
                        })
                    }))
                    // guard by typedef's extension(because define uses the type)
                    .chain(
                        self.extension_old
                            .map(|(tag, name)| CompilationCondition::Feature(FeatureName::VulkanExt { tag, name })),
                    )
                    .chain(self.extension.map(|x| {
                        CompilationCondition::Feature(FeatureName::VulkanExt {
                            tag: x.tag,
                            name: x.name,
                        })
                    }))
                    .chain(self.side_extensions.iter().map(|x| {
                        CompilationCondition::Feature(FeatureName::VulkanExt {
                            tag: x.tag,
                            name: x.name,
                        })
                    })),
            );

            emitter.emit_const(Constant {
                compilation_condition: cond,
                name: ConstantSymbol::Bitmask {
                    prefix: self.prefix,
                    stem: e.name,
                    suffix: e.extension.map(|x| x.tag).or(e.extension_old.map(|(tag, _)| tag)),
                },
                ty: Type::Defined(bits_sym.clone()),
                value: if self.long {
                    ConstantValue::Bits64(1u64 << e.bitpos)
                } else {
                    ConstantValue::Bits32(1u32 << e.bitpos)
                },
            });

            if let Some(p) = e.promoted {
                emitter.emit_const(Constant {
                    compilation_condition: CompilationCondition::Feature(FeatureName::AllowApiVersion(p)).and(
                        CompilationCondition::all(
                            e.side_extensions
                                .iter()
                                .map(|x| {
                                    CompilationCondition::Feature(FeatureName::VulkanExt {
                                        tag: x.tag,
                                        name: x.name,
                                    })
                                })
                                .chain(self.side_extensions.iter().map(|x| {
                                    CompilationCondition::Feature(FeatureName::VulkanExt {
                                        tag: x.tag,
                                        name: x.name,
                                    })
                                })),
                        ),
                    ),
                    name: ConstantSymbol::Bitmask {
                        prefix: self.prefix,
                        stem: e.name,
                        suffix: None,
                    },
                    ty: Type::Defined(TypeSymbol {
                        stem: self.bits_name,
                        suffix: None,
                    }),
                    value: if self.long {
                        ConstantValue::Bits64(1u64 << e.bitpos)
                    } else {
                        ConstantValue::Bits32(1u32 << e.bitpos)
                    },
                });
            }
        }
    }
}

pub struct Object {
    dispatchable: bool,
    name: &'static str,
    object_type_const_name: &'static str,
    object_type_const_value: i32,
    extension_old: Option<&'static str>,
    extension: Option<&'static Extension<'static>>,
    promoted: Option<(&'static str, &'static str, &'static str)>,
}
impl Object {
    pub const fn new(name: &'static str, const_suffix: &'static str, object_type_value: i32) -> Self {
        Self {
            dispatchable: false,
            name,
            object_type_const_name: const_suffix,
            object_type_const_value: object_type_value,
            extension_old: None,
            extension: None,
            promoted: None,
        }
    }

    pub const fn dispatchable(mut self) -> Self {
        self.dispatchable = true;
        self
    }

    #[deprecated = "use new extensions specifier"]
    pub const fn extension_old(mut self, name: &'static str) -> Self {
        self.extension_old = Some(name);
        self
    }

    pub const fn extension(mut self, x: &'static Extension<'static>) -> Self {
        self.extension = Some(x);
        self
    }

    pub const fn promoted(
        mut self,
        version: &'static str,
        promoted_name: &'static str,
        promoted_otype_name: &'static str,
    ) -> Self {
        self.promoted = Some((version, promoted_name, promoted_otype_name));
        self
    }

    pub const fn into_element(self) -> Element {
        Element::Object(self)
    }

    fn emit_ext_item_prefixes(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        if let Some(x) = self.extension_old {
            writeln!(w, "#[cfg(feature = {x:?})]")?;
        }
        if let Some(Extension { tag, name, .. }) = self.extension {
            writeln!(w, "#[cfg(feature = \"VK_{tag}_{name}\")]")?;
        }
        writeln!(w, "#[rustfmt::skip]")?;

        Ok(())
    }

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        let (value_type, raw_expr);
        if self.dispatchable {
            value_type = "core::ptr::NonNull<core::ffi::c_void>";
            raw_expr = "self.0.as_ptr().addr() as _";
        } else {
            value_type = "core::num::NonZeroU64";
            raw_expr = "self.0.get()";
        }

        self.emit_ext_item_prefixes(w)?;
        writeln!(w, "#[repr(transparent)]")?;
        writeln!(w, "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]")?;
        writeln!(
            w,
            "pub struct {}(pub {value_type}, pub core::marker::PhantomData<*mut u8>);",
            self.name
        )?;

        self.emit_ext_item_prefixes(w)?;
        writeln!(w, "impl crate::VkRawHandle for {} {{", self.name)?;
        writeln!(
            w,
            "    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_{};",
            self.object_type_const_name
        )?;
        w.write_all(b"\n")?;
        writeln!(w, "    #[inline(always)]")?;
        writeln!(w, "    fn raw_handle_value(&self) -> u64 {{")?;
        writeln!(w, "        {raw_expr}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;

        self.emit_ext_item_prefixes(w)?;
        writeln!(
            w,
            "pub const VK_OBJECT_TYPE_{}: VkObjectType = {};",
            self.object_type_const_name, self.object_type_const_value
        )?;

        if let Some((v, n, on)) = self.promoted {
            writeln!(w, "#[cfg(feature = \"Allow{v}APIs\")]")?;
            writeln!(w, "#[rustfmt::skip]")?;
            writeln!(w, "pub type {n} = {};", self.name)?;
            writeln!(w, "#[cfg(feature = \"Allow{v}APIs\")]")?;
            writeln!(w, "#[rustfmt::skip]")?;
            writeln!(
                w,
                "pub const VK_OBJECT_TYPE_{on}: VkObjectType = {};",
                self.object_type_const_value
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StructUsage {
    Source,
    Sink,
    Both,
}
impl StructUsage {
    #[inline(always)]
    const fn is_source(&self) -> bool {
        matches!(self, StructUsage::Source | StructUsage::Both)
    }

    #[inline(always)]
    const fn is_sink(&self) -> bool {
        matches!(self, StructUsage::Sink | StructUsage::Both)
    }
}

pub struct Struct {
    name: &'static str,
    stype: Option<(&'static str, u32, StructUsage)>,
    members: &'static [StructMember],
    debuggable: bool,
    cloneable: bool,
    copyable: bool,
    equatable: bool,
    hashable: bool,
    default_zero: bool,
    extensions_old: &'static [(&'static str, &'static str)],
    extensions: &'static [&'static Extension<'static>],
    side_extensions: &'static [&'static Extension<'static>],
    promoted: Option<&'static str>,
    version_since: Option<&'static str>,
}
impl Struct {
    pub const fn new(name: &'static str, members: &'static [StructMember]) -> Self {
        Self {
            name,
            stype: None,
            members,
            debuggable: true,
            cloneable: true,
            copyable: false,
            equatable: false,
            hashable: false,
            default_zero: false,
            extensions_old: &[],
            extensions: &[],
            side_extensions: &[],
            promoted: None,
            version_since: None,
        }
    }

    pub const fn typed(
        name: &'static str,
        sty_suffix: &'static str,
        sty_value: u32,
        sty_usage: StructUsage,
        members: &'static [StructMember],
    ) -> Self {
        Self {
            stype: Some((sty_suffix, sty_value, sty_usage)),
            ..Self::new(name, members)
        }
    }

    #[deprecated = "use Struct::typed constructor"]
    pub const fn stype(mut self, unique_part: &'static str, value: u32, usage: StructUsage) -> Self {
        self.stype = Some((unique_part, value, usage));
        self
    }

    pub const fn non_debuggable(mut self) -> Self {
        self.debuggable = false;
        self
    }

    pub const fn copyable(mut self) -> Self {
        // implies cloneable
        self.cloneable = true;
        self.copyable = true;
        self
    }

    pub const fn equatable(mut self) -> Self {
        self.equatable = true;
        self
    }

    pub const fn hashable(mut self) -> Self {
        self.hashable = true;
        self
    }

    pub const fn default_zero(mut self) -> Self {
        self.default_zero = true;
        self
    }

    #[deprecated = "use new extensions specifier"]
    pub const fn extensions_old(mut self, extensions: &'static [(&'static str, &'static str)]) -> Self {
        self.extensions_old = extensions;
        self
    }

    pub const fn extensions(mut self, extensions: &'static [&'static Extension<'static>]) -> Self {
        self.extensions = extensions;
        self
    }

    pub const fn side_extensions(mut self, side_extensions: &'static [&'static Extension<'static>]) -> Self {
        self.side_extensions = side_extensions;
        self
    }

    pub const fn promoted(mut self, version: &'static str) -> Self {
        self.promoted = Some(version);
        self
    }

    pub const fn version_since(mut self, version: &'static str) -> Self {
        self.version_since = Some(version);
        self
    }

    pub const fn member(name: &'static str, r#type: &'static str) -> StructMember {
        StructMember::new(name, r#type)
    }

    pub const fn into_element(self) -> Element {
        Element::Struct(self)
    }

    fn derives(&self) -> StructDerives {
        let mut derives = StructDerives::empty();
        if self.debuggable {
            derives |= StructDerives::DEBUG;
        }
        if self.cloneable {
            derives |= StructDerives::CLONE;
        }
        if self.copyable {
            derives |= StructDerives::COPY;
        }
        if self.equatable {
            derives |= StructDerives::EQ;
        }
        if self.hashable {
            derives |= StructDerives::HASH;
        }

        derives
    }

    fn build_all_rs_members(
        direct_members: &[StructMember],
        usage: Option<StructUsage>,
    ) -> Vec<crate::rs_item::StructMember<'static>> {
        let mut emit_members = Vec::with_capacity(direct_members.len() + 2);
        if let Some(u) = usage {
            emit_members.extend([
                crate::rs_item::StructMember {
                    name: "sType",
                    ty: Type::Raw("VkStructureType"),
                },
                crate::rs_item::StructMember {
                    name: "pNext",
                    ty: match u {
                        StructUsage::Source => Type::Raw("core::ffi::c_void").const_ptr(),
                        StructUsage::Sink => Type::Raw("core::ffi::c_void").mut_ptr(),
                        // prefer mut pointer
                        StructUsage::Both => Type::Raw("core::ffi::c_void").mut_ptr(),
                    },
                },
            ]);
        }
        emit_members.extend(direct_members.iter().map(|m| crate::rs_item::StructMember {
            name: m.name,
            ty: Type::Raw(m.r#type),
        }));

        emit_members
    }

    pub fn emit(&self, emitter: &mut (impl RustCodeEmitter + ?Sized)) {
        if self.extensions_old.is_empty() && self.extensions.is_empty() {
            assert!(self.promoted.is_none());
            // no extensions: simple define
            let mut cond = CompilationCondition::Empty;
            if let Some(v) = self.version_since {
                cond = cond.and(CompilationCondition::Feature(FeatureName::AllowApiVersion(v)));
            }

            let mut derives = self.derives();
            let mut typed_vulkan_structure_impl = None;
            let mut typed_vulkan_sink_structure_impl = None;
            let mut default = StructDefault::None;
            if let Some((up, v, u)) = self.stype {
                let st_const_sym = ConstantSymbol::Enum {
                    prefix: "STRUCTURE_TYPE",
                    stem: up,
                    suffix: None,
                };
                emitter.emit_const(Constant {
                    compilation_condition: cond.clone(),
                    name: st_const_sym.clone(),
                    ty: Type::Raw("VkStructureType"),
                    value: ConstantValue::UnsignedLong(v as _),
                });

                if u.is_source() {
                    derives |= StructDerives::VULKAN_STRUCTURE;
                    typed_vulkan_structure_impl = Some(StructTypedVulkanStructureImpl {
                        const_name: st_const_sym.clone(),
                    });
                }
                if u.is_sink() {
                    derives |= StructDerives::VULKAN_SINK_STRUCTURE;
                    typed_vulkan_sink_structure_impl = Some(StructTypedVulkanSinkStructureImpl {
                        const_name: st_const_sym.clone(),
                    });
                }

                if self.default_zero {
                    default = StructDefault::ZeroTyped(st_const_sym.clone())
                }
            } else if self.default_zero {
                default = StructDefault::Zero;
            }

            emitter.emit_struct(crate::rs_item::Struct {
                compilation_condition: cond.clone(),
                name: TypeSymbol {
                    stem: self.name,
                    suffix: None,
                },
                derives,
                members: Self::build_all_rs_members(self.members, self.stype.map(|(_, _, u)| u)),
                typed_vulkan_structure_impl,
                typed_vulkan_sink_structure_impl,
                default,
            });

            return;
        }

        let mut extensions_suffixes = HashMap::new();
        for &(tag, name) in self.extensions_old {
            extensions_suffixes.entry(tag).or_insert_with(Vec::new).push(name);
        }
        for x in self.extensions.iter() {
            extensions_suffixes.entry(x.tag).or_insert_with(Vec::new).push(x.name);
        }

        if extensions_suffixes.len() == 1 {
            // single extension with probably promoted
            let (tag, names) = unsafe { extensions_suffixes.drain().next().unwrap_unchecked() };
            let mut cond = match &names[..] {
                &[name] => CompilationCondition::Feature(FeatureName::VulkanExt { tag, name }),
                xs => CompilationCondition::all(
                    xs.iter()
                        .map(|name| CompilationCondition::Feature(FeatureName::VulkanExt { tag, name })),
                ),
            };
            if !self.side_extensions.is_empty() {
                cond = cond.and(CompilationCondition::all(self.side_extensions.iter().map(|x| {
                    CompilationCondition::Feature(FeatureName::VulkanExt {
                        tag: x.tag,
                        name: x.name,
                    })
                })));
            }
            if let Some(v) = self.version_since {
                cond = cond.and(CompilationCondition::Feature(FeatureName::AllowApiVersion(v)));
            }

            let mut derives = self.derives();
            let mut typed_vulkan_structure_impl = None;
            let mut typed_vulkan_sink_structure_impl = None;
            let mut default = StructDefault::None;
            if let Some((up, v, u)) = self.stype {
                let st_const_sym = ConstantSymbol::Enum {
                    prefix: "STRUCTURE_TYPE",
                    stem: up,
                    suffix: Some(tag),
                };
                emitter.emit_const(Constant {
                    compilation_condition: cond.clone(),
                    name: st_const_sym.clone(),
                    ty: Type::Raw("VkStructureType"),
                    value: ConstantValue::UnsignedLong(v as _),
                });

                if u.is_source() {
                    derives |= StructDerives::VULKAN_STRUCTURE;
                    typed_vulkan_structure_impl = Some(StructTypedVulkanStructureImpl {
                        const_name: st_const_sym.clone(),
                    });
                }
                if u.is_sink() {
                    derives |= StructDerives::VULKAN_SINK_STRUCTURE;
                    typed_vulkan_sink_structure_impl = Some(StructTypedVulkanSinkStructureImpl {
                        const_name: st_const_sym.clone(),
                    });
                }

                if self.default_zero {
                    default = StructDefault::ZeroTyped(st_const_sym.clone());
                }
            } else if self.default_zero {
                default = StructDefault::Zero;
            }

            emitter.emit_struct(crate::rs_item::Struct {
                compilation_condition: cond.clone(),
                name: TypeSymbol {
                    stem: self.name,
                    suffix: Some(tag),
                },
                derives,
                members: Self::build_all_rs_members(self.members, self.stype.map(|(_, _, u)| u)),
                typed_vulkan_structure_impl,
                typed_vulkan_sink_structure_impl,
                default,
            });

            if let Some(pv) = self.promoted {
                emitter.emit_type_alias(crate::rs_item::TypeAlias {
                    compilation_condition: CompilationCondition::Feature(FeatureName::AllowApiVersion(pv)),
                    target_name: TypeSymbol {
                        stem: self.name,
                        suffix: None,
                    },
                    source_name: Type::Defined(TypeSymbol {
                        stem: self.name,
                        suffix: Some(tag),
                    }),
                });
                if let Some((up, v, _)) = self.stype {
                    emitter.emit_const(Constant {
                        compilation_condition: CompilationCondition::Feature(FeatureName::AllowApiVersion(pv)),
                        name: ConstantSymbol::Enum {
                            prefix: "STRUCTURE_TYPE",
                            stem: up,
                            suffix: None,
                        },
                        ty: Type::Raw("VkStructureType"),
                        value: ConstantValue::UnsignedLong(v as _),
                    });
                }
            }

            return;
        }

        unimplemented!();
    }

    pub fn emit_extra_cfg(&self, w: &mut impl std::io::Write, cfg: &str) -> std::io::Result<()> {
        assert!(
            self.extensions_old.is_empty() && self.extensions.is_empty(),
            "struct def has some extensions"
        );
        assert!(self.promoted.is_none());
        // no extensions: simple define
        let mut derives = self.derives();
        let mut typed_vulkan_structure_impl = None;
        let mut typed_vulkan_sink_structure_impl = None;
        if let Some((up, v, u)) = self.stype {
            let st_const_sym = ConstantSymbol::Enum {
                prefix: "STRUCTURE_TYPE",
                stem: up,
                suffix: None,
            };

            Constant {
                compilation_condition: CompilationCondition::Raw(cfg),
                name: st_const_sym.clone(),
                ty: Type::Raw("VkStructureType"),
                value: ConstantValue::UnsignedLong(v as _),
            }
            .emit(w)?;
            w.write_all(b"\n")?;

            if u.is_source() {
                derives |= StructDerives::VULKAN_STRUCTURE;
                typed_vulkan_structure_impl = Some(StructTypedVulkanStructureImpl {
                    const_name: st_const_sym.clone(),
                });
            }
            if u.is_sink() {
                derives |= StructDerives::VULKAN_SINK_STRUCTURE;
                typed_vulkan_sink_structure_impl = Some(StructTypedVulkanSinkStructureImpl {
                    const_name: st_const_sym.clone(),
                });
            }
        }

        crate::rs_item::Struct {
            compilation_condition: CompilationCondition::Raw(cfg),
            name: TypeSymbol {
                stem: self.name,
                suffix: None,
            },
            members: Self::build_all_rs_members(self.members, self.stype.map(|(_, _, u)| u)),
            derives,
            typed_vulkan_structure_impl,
            typed_vulkan_sink_structure_impl,
            default: StructDefault::None,
        }
        .emit(w)?;

        Ok(())
    }
}

pub struct StructMember {
    name: &'static str,
    r#type: &'static str,
    comment: Option<&'static str>,
}
impl StructMember {
    pub const fn new(name: &'static str, r#type: &'static str) -> Self {
        Self {
            name,
            r#type,
            comment: None,
        }
    }

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        if let Some(c) = self.comment {
            writeln!(w, "    /// {c}")?;
        }
        writeln!(w, "    pub {}: {},", self.name, self.r#type)?;

        Ok(())
    }
}

pub struct Union {
    name: &'static str,
    comment: Option<&'static str>,
    members: &'static [StructMember],
    cloneable: bool,
    copyable: bool,
    extension: Option<(&'static str, &'static str)>,
}
impl Union {
    pub const fn new(name: &'static str, members: &'static [StructMember]) -> Self {
        Self {
            name,
            comment: None,
            members,
            cloneable: true,
            copyable: true,
            extension: None,
        }
    }

    // いまのところない
    pub const fn extension(mut self, suffix: &'static str, name: &'static str) -> Self {
        self.extension = Some((suffix, name));
        self
    }

    pub const fn member(name: &'static str, r#type: &'static str) -> StructMember {
        StructMember::new(name, r#type)
    }

    fn emit_ident(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        w.write_all(b"Vk")?;
        w.write_all(self.name.as_bytes())?;
        if let Some((suffix, _)) = self.extension {
            w.write_all(suffix.as_bytes())?;
        }

        Ok(())
    }

    fn emit_extension_gate(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        if let Some((suffix, name)) = self.extension {
            writeln!(w, "#[cfg(feature = \"VK_{suffix}_{name}\")]")?;
        }

        Ok(())
    }

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        if let Some(c) = self.comment {
            writeln!(w, "/// {c}")?;
        }

        self.emit_extension_gate(w)?;
        let mut derives = Vec::with_capacity(8);
        if self.cloneable {
            derives.push("Clone");
        }
        if self.copyable {
            derives.push("Copy");
        }
        if !derives.is_empty() {
            write!(w, "#[derive(")?;
            let mut cont = false;
            for d in derives {
                if cont {
                    w.write_all(b", ")?;
                }
                w.write_all(d.as_bytes())?;
                cont = true;
            }
            w.write_all(b")]\n")?;
        }

        writeln!(w, "#[repr(C)]")?;
        writeln!(w, "#[rustfmt::skip]")?;
        w.write_all(b"pub union ")?;
        self.emit_ident(w)?;
        w.write_all(b" {\n")?;
        for m in self.members {
            m.emit(w)?;
        }
        writeln!(w, "}}")?;

        Ok(())
    }
}

pub struct Command {
    is_command_buffer_inst: bool,
    name: &'static str,
    args: &'static [(&'static str, &'static str)],
    return_type: Option<&'static str>,
    static_callable: bool,
    version_since: Option<&'static str>,
    extension_old: Option<(&'static str, &'static str)>,
    extension: Option<&'static Extension<'static>>,
    side_extensions: &'static [&'static Extension<'static>],
    available_condition: &'static str,
    promoted: Option<&'static str>,
}
impl Command {
    pub const fn new(name: &'static str, args: &'static [(&'static str, &'static str)]) -> Self {
        Self {
            is_command_buffer_inst: false,
            name,
            args,
            return_type: None,
            static_callable: false,
            version_since: None,
            extension_old: None,
            extension: None,
            side_extensions: &[],
            available_condition: "",
            promoted: None,
        }
    }

    pub const fn inst(name: &'static str, args: &'static [(&'static str, &'static str)]) -> Self {
        Self {
            is_command_buffer_inst: true,
            name,
            args,
            return_type: None,
            static_callable: false,
            version_since: None,
            extension_old: None,
            extension: None,
            side_extensions: &[],
            available_condition: "",
            promoted: None,
        }
    }

    pub const fn failable(self) -> Self {
        self.returns("VkResult")
    }

    pub const fn returns(mut self, return_type: &'static str) -> Self {
        self.return_type = Some(return_type);
        self
    }

    pub const fn static_callable(mut self) -> Self {
        self.static_callable = true;
        self
    }

    pub const fn version_since(mut self, version_since: &'static str) -> Self {
        self.version_since = Some(version_since);
        self
    }

    #[deprecated = "use new extension specifier"]
    pub const fn extension_old(mut self, tag: &'static str, name: &'static str) -> Self {
        self.extension_old = Some((tag, name));
        self
    }

    pub const fn extension(mut self, extension: &'static Extension<'static>) -> Self {
        self.extension = Some(extension);
        self
    }

    pub const fn side_extensions(mut self, side_extensions: &'static [&'static Extension<'static>]) -> Self {
        self.side_extensions = side_extensions;
        self
    }

    pub const fn available_condition(mut self, cond: &'static str) -> Self {
        self.available_condition = cond;
        self
    }

    pub const fn promoted(mut self, promoted: &'static str) -> Self {
        self.promoted = Some(promoted);
        self
    }

    pub const fn into_element(self) -> Element {
        Element::Command(self)
    }

    pub fn emit(&self, emitter: &mut (impl RustCodeEmitter + ?Sized)) {
        let fn_name = if let Some(Extension { tag, .. }) = self.extension {
            if self.is_command_buffer_inst {
                FnSymbol::CmdSuffixed {
                    stem: self.name,
                    suffix: tag,
                }
            } else {
                FnSymbol::Suffixed {
                    stem: self.name,
                    suffix: tag,
                }
            }
        } else {
            match (self.is_command_buffer_inst, self.extension_old) {
                (false, None) => FnSymbol::Raw(self.name),
                (true, None) => FnSymbol::Cmd(self.name),
                (false, Some((tag, _))) => FnSymbol::Suffixed {
                    stem: self.name,
                    suffix: tag,
                },
                (true, Some((tag, _))) => FnSymbol::CmdSuffixed {
                    stem: self.name,
                    suffix: tag,
                },
            }
        };

        let mut cond = CompilationCondition::Empty;
        if let Some((tag, name)) = self.extension_old {
            cond = cond.and(CompilationCondition::Feature(FeatureName::VulkanExt { tag, name }));
        }
        if let Some(Extension { tag, name, .. }) = self.extension {
            cond = cond.and(CompilationCondition::Feature(FeatureName::VulkanExt { tag, name }));
        }
        if !self.side_extensions.is_empty() {
            cond = cond.and(CompilationCondition::all(self.side_extensions.iter().map(|x| {
                CompilationCondition::Feature(FeatureName::VulkanExt {
                    tag: x.tag,
                    name: x.name,
                })
            })));
        }
        if let Some(v) = self.version_since {
            cond = cond.and(CompilationCondition::Feature(FeatureName::AllowApiVersion(v)));
        }

        let mut args = Vec::with_capacity(self.args.len() + 1);
        if self.is_command_buffer_inst {
            args.push((
                "commandBuffer",
                Type::Defined(TypeSymbol {
                    stem: "CommandBuffer",
                    suffix: None,
                }),
            ));
        }
        args.extend(self.args.iter().map(|&(n, t)| (n, Type::Raw(t))));

        let mut derives = FunctionPtrNewtypeDerives::PFN | FunctionPtrNewtypeDerives::FROM_PTR;
        if self.static_callable {
            derives |= FunctionPtrNewtypeDerives::STATIC_CALLABLE;
            emitter.emit_function_stub(FunctionStub {
                compilation_condition: cond.clone(),
                name: fn_name.clone(),
                args: args.clone(),
                return_type: self.return_type.map(Type::Raw),
            });
        }

        emitter.emit_function_ptr_newtype(FunctionPtrNewtype {
            compilation_condition: cond,
            name: fn_name,
            args: args.clone(),
            return_type: self.return_type.map(Type::Raw),
            derives,
        });

        if let Some(p) = self.promoted {
            // promoted symbols always static callable
            let cond = CompilationCondition::Feature(FeatureName::AllowApiVersion(p));
            let name = match self.is_command_buffer_inst {
                false => FnSymbol::Raw(self.name),
                true => FnSymbol::Cmd(self.name),
            };

            emitter.emit_function_ptr_newtype(FunctionPtrNewtype {
                compilation_condition: cond.clone(),
                name: name.clone(),
                args: args.clone(),
                return_type: self.return_type.map(Type::Raw),
                derives: FunctionPtrNewtypeDerives::PFN
                    | FunctionPtrNewtypeDerives::FROM_PTR
                    | FunctionPtrNewtypeDerives::STATIC_CALLABLE,
            });
            emitter.emit_function_stub(FunctionStub {
                compilation_condition: cond,
                name,
                args,
                return_type: self.return_type.map(Type::Raw),
            });
        }
    }
}

pub struct FunctionStubArgsIterator<'s> {
    require_target_command_buffer: bool,
    args: &'s [(&'s str, &'s str)],
    args_iter_ptr: usize,
}
impl<'s> Iterator for FunctionStubArgsIterator<'s> {
    type Item = (&'s str, Type<'s>);

    fn next(&mut self) -> Option<Self::Item> {
        if core::mem::replace(&mut self.require_target_command_buffer, false) {
            return Some(("commandBuffer", Type::Raw("VkCommandBuffer")));
        }

        let (n, t) = self.args.get(self.args_iter_ptr)?;
        self.args_iter_ptr += 1;
        Some((n, Type::Raw(t)))
    }
}

pub struct ExtensionHeaderConstants2<'e, 's>(pub &'e Extension<'s>);
impl ExtensionHeaderConstants2<'static, 'static> {
    pub const fn into_element(self) -> Element {
        Element::ExtensionHeaderConstants2(self)
    }
}
impl<'e, 's> ExtensionHeaderConstants2<'e, 's> {
    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        let Extension {
            tag,
            name,
            revision: version,
            ..
        } = self.0;
        let name_up = name.to_uppercase();

        writeln!(w, "#[cfg(feature = \"VK_{tag}_{name}\")]")?;
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(
            w,
            "pub const VK_{tag}_{name_up}_EXTENSION_NAME: &str = \"VK_{tag}_{name}\";",
        )?;
        writeln!(w, "#[cfg(feature = \"VK_{tag}_{name}\")]")?;
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(w, "pub const VK_{tag}_{name_up}_SPEC_VERSION: usize = {version};",)?;
        Ok(())
    }
}

pub struct ExtensionHeaderConstants {
    name: &'static str,
    spec_version: usize,
}
impl ExtensionHeaderConstants {
    pub const fn new(name: &'static str, spec_version: usize) -> Self {
        Self { name, spec_version }
    }

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        writeln!(w, "#[cfg(feature = \"{}\")]", self.name)?;
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(
            w,
            "pub const {}_EXTENSION_NAME: &str = \"{}\";",
            self.name.to_uppercase(),
            self.name
        )?;
        writeln!(w, "#[cfg(feature = \"{}\")]", self.name)?;
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(
            w,
            "pub const {}_SPEC_VERSION: usize = {};",
            self.name.to_uppercase(),
            self.spec_version
        )?;
        Ok(())
    }
}

pub enum Element {
    ExtensionHeaderConstants(ExtensionHeaderConstants),
    ExtensionHeaderConstants2(ExtensionHeaderConstants2<'static, 'static>),
    Bitmask(Bitmask),
    Enum(Enum),
    FuncPointer(FuncPointer),
    Object(Object),
    Struct(Struct),
    Union(Union),
    Command(Command),
}
impl Element {
    pub fn emit(
        &self,
        emitter: &mut (impl RustCodeEmitter + ?Sized),
        w: &mut impl std::io::Write,
    ) -> std::io::Result<()> {
        match self {
            Self::ExtensionHeaderConstants(x) => {
                w.write_all(b"\n")?;
                x.emit(w)
            }
            Self::ExtensionHeaderConstants2(x) => {
                w.write_all(b"\n")?;
                x.emit(w)
            }
            Self::Bitmask(x) => {
                x.emit(emitter);
                Ok(())
            }
            Self::Enum(x) => {
                x.emit(emitter);
                Ok(())
            }
            Self::FuncPointer(x) => {
                w.write_all(b"\n")?;
                x.emit(w)
            }
            Self::Object(x) => {
                w.write_all(b"\n")?;
                x.emit(w)
            }
            Self::Struct(x) => {
                x.emit(emitter);
                Ok(())
            }
            Self::Union(x) => {
                w.write_all(b"\n")?;
                x.emit(w)
            }
            Self::Command(x) => {
                x.emit(emitter);
                Ok(())
            }
        }
    }
}

pub struct Extension<'s> {
    pub tag: &'s str,
    pub name: &'s str,
    pub revision: usize,
    /// Registered Extension Number
    pub number: i32,
    pub promoted_since: &'s str,
}
impl<'s> Extension<'s> {
    pub const fn new(tag: &'s str, name: &'s str, revision: usize, number: i32) -> Self {
        Self {
            tag,
            name,
            revision,
            number,
            promoted_since: "",
        }
    }

    pub const fn khr(name: &'s str, revision: usize, number: i32) -> Self {
        Self::new("KHR", name, revision, number)
    }

    pub const fn ext(name: &'s str, revision: usize, number: i32) -> Self {
        Self::new("EXT", name, revision, number)
    }

    pub const fn promoted(mut self, since: &'s str) -> Self {
        self.promoted_since = since;
        self
    }

    pub const fn ext_enum(&self, index: i32) -> i32 {
        crate::vk_ext_enum(self.number, index)
    }

    pub const fn header_constants<'e>(&'e self) -> ExtensionHeaderConstants2<'e, 's> {
        ExtensionHeaderConstants2(self)
    }
}
