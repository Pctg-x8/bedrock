use std::collections::{HashMap, HashSet};

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

pub fn emit_result_const(w: &mut impl std::io::Write, name: &str, ext_number: u32, value: u32) -> std::io::Result<()> {
    let value = if ext_number == 0 {
        value
    } else {
        100_0000_000 + (ext_number - 1) * 1_000 + value
    };

    writeln!(w, "#[rustfmt::skip]")?;
    writeln!(w, "pub const {name}: VkResult = VkResult({value});")
}

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
                w.write(b", ")?;
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
    extension: Option<(&'static str, &'static str)>,
    promoted: Option<&'static str>,
}
impl EnumMember {
    pub const fn extension(mut self, ext: &'static str, suffix: &'static str) -> Self {
        self.extension = Some((ext, suffix));
        self
    }

    pub const fn promoted(mut self, version: &'static str) -> Self {
        self.promoted = Some(version);
        self
    }
}

pub struct Enum {
    name: &'static str,
    prefix: &'static str,
    members: &'static [EnumMember],
    extension: Option<(&'static str, &'static str)>,
    promoted: Option<&'static str>,
}
impl Enum {
    pub const fn new(name: &'static str, prefix: &'static str, members: &'static [EnumMember]) -> Self {
        Self {
            name,
            prefix,
            members,
            extension: None,
            promoted: None,
        }
    }

    pub const fn extension(mut self, name: &'static str, suffix: &'static str) -> Self {
        self.extension = Some((name, suffix));
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
            extension: None,
            promoted: None,
        }
    }

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        let type_name = if let Some((_, s)) = self.extension {
            format!("Vk{}{s}", self.name)
        } else {
            format!("Vk{}", self.name)
        };
        if let Some((x, _)) = self.extension {
            writeln!(w, "#[cfg(feature = {x:?})]")?;
        }
        emit_c_enum_type(w, &type_name)?;
        if let Some(v) = self.promoted {
            writeln!(w, "#[cfg(feature = \"Allow{v}APIs\")]")?;
            writeln!(w, "#[rustfmt::skip]")?;
            emit_c_enum_type(w, &format!("Vk{}", self.name))?;
        }

        for member in self.members {
            match (self.extension, member.extension) {
                (None, None) => {
                    writeln!(w, "#[rustfmt::skip]")?;
                    writeln!(
                        w,
                        "pub const VK_{}_{}: {type_name} = {};",
                        self.prefix, member.name, member.value
                    )?;
                }
                (Some((x, s)), None) | (None, Some((x, s))) => {
                    writeln!(w, "#[cfg(feature = {x:?})]")?;
                    writeln!(w, "#[rustfmt::skip]")?;
                    writeln!(
                        w,
                        "pub const VK_{}_{}_{s}: {type_name} = {};",
                        self.prefix, member.name, member.value
                    )?;
                }
                (Some(a), Some(b)) if a.0 == b.0 => {
                    writeln!(w, "#[cfg(feature = {:?})]", a.0)?;
                    writeln!(w, "#[rustfmt::skip]")?;
                    writeln!(
                        w,
                        "pub const VK_{}_{}_{}: {type_name} = {};",
                        self.prefix, member.name, a.1, member.value
                    )?;
                }
                (Some(a), Some(b)) if a.1 == b.1 => {
                    writeln!(w, "#[cfg(feature = {:?})]", a.0)?;
                    writeln!(w, "#[cfg(feature = {:?})]", b.0)?;
                    writeln!(w, "#[rustfmt::skip]")?;
                    writeln!(
                        w,
                        "pub const VK_{}_{}_{}: {type_name} = {};",
                        self.prefix, member.name, a.1, member.value
                    )?;
                }
                (Some(a), Some(b)) => {
                    // both extension required, using member extension's suffix
                    writeln!(w, "#[cfg(feature = {:?})]", a.0)?;
                    writeln!(w, "#[cfg(feature = {:?})]", b.0)?;
                    writeln!(w, "#[rustfmt::skip]")?;
                    writeln!(
                        w,
                        "pub const VK_{}_{}_{}: {type_name} = {};",
                        self.prefix, member.name, b.1, member.value
                    )?;
                }
            }

            if let Some(v) = member.promoted {
                if let Some((x, _)) = self.extension {
                    writeln!(w, "#[cfg(feature = {x:?})]")?;
                }
                writeln!(w, "#[cfg(feature = \"Allow{v}APIs\")]")?;
                writeln!(w, "#[rustfmt::skip]")?;
                writeln!(
                    w,
                    "pub const VK_{}_{}: {type_name} = {};",
                    self.prefix, member.name, member.value
                )?;
            }
        }

        Ok(())
    }
}

pub struct BitmaskEntry {
    name: &'static str,
    bitpos: usize,
    extension: Option<(&'static str, &'static str)>,
    extra_requirements: &'static [&'static str],
    promoted: Option<&'static str>,
    version_since: Option<&'static str>,
}
impl BitmaskEntry {
    pub const fn extension(mut self, tag: &'static str, name: &'static str) -> Self {
        self.extension = Some((tag, name));
        self
    }

    pub const fn promoted(mut self, name: &'static str) -> Self {
        self.promoted = Some(name);
        self
    }

    pub const fn extra_requirements(mut self, requirements: &'static [&'static str]) -> Self {
        self.extra_requirements = requirements;
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
    entries: &'static [BitmaskEntry],
    version_since: Option<&'static str>,
    extension: Option<(&'static str, &'static str)>,
    extra_requirements: &'static [&'static str],
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
            entries,
            version_since: None,
            extension: None,
            extra_requirements: &[],
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

    pub const fn extension(mut self, tag: &'static str, name: &'static str) -> Self {
        self.extension = Some((tag, name));
        self
    }

    pub const fn extra_requirements(mut self, requirements: &'static [&'static str]) -> Self {
        self.extra_requirements = requirements;
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
            extension: None,
            extra_requirements: &[],
            promoted: None,
            version_since: None,
        }
    }

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        let (type_name, bits_name);
        if let Some((tag, _)) = self.extension {
            type_name = format!("Vk{}{tag}", self.name);
            bits_name = format!("Vk{}{tag}", self.bits_name);
        } else {
            type_name = format!("Vk{}", self.name);
            bits_name = format!("Vk{}", self.bits_name);
        }

        if let Some(x) = self.version_since {
            writeln!(w, "#[cfg(feature = \"Allow{x}APIs\")]")?;
        }
        if let Some((tag, name)) = self.extension {
            writeln!(w, "#[cfg(feature = \"VK_{tag}_{name}\")]")?;
        }
        for x in self.extra_requirements {
            writeln!(w, "#[cfg(feature = \"{x}\")]")?;
        }
        emit_type_alias(w, &type_name, if self.long { "VkFlags64" } else { "VkFlags" })?;
        if let Some(p) = self.promoted {
            writeln!(w, "#[cfg(feature = \"Allow{p}APIs\")]")?;
            emit_type_alias(w, &format!("Vk{}", self.name), &type_name)?;
        }
        if let Some(x) = self.version_since {
            writeln!(w, "#[cfg(feature = \"Allow{x}APIs\")]")?;
        }
        if let Some((tag, name)) = self.extension {
            writeln!(w, "#[cfg(feature = \"VK_{tag}_{name}\")]")?;
        }
        for x in self.extra_requirements {
            writeln!(w, "#[cfg(feature = \"{x}\")]")?;
        }
        emit_type_alias(w, &bits_name, if self.long { "VkFlags64" } else { "VkFlags" })?;
        if let Some(p) = self.promoted {
            writeln!(w, "#[cfg(feature = \"Allow{p}APIs\")]")?;
            emit_type_alias(w, &format!("Vk{}", self.bits_name), &bits_name)?;
        }
        for e in self.entries {
            let mut feature_requirements = HashSet::<String>::new();
            feature_requirements.extend(self.version_since.map(|v| format!("Allow{v}APIs")));
            feature_requirements.extend(e.version_since.map(|v| format!("Allow{v}APIs")));
            // guard by typedef's extension because define uses the type
            feature_requirements.extend(self.extension.map(|(tag, name)| format!("VK_{tag}_{name}")));
            feature_requirements.extend(e.extension.map(|(tag, name)| format!("VK_{tag}_{name}")));
            feature_requirements.extend(self.extra_requirements.iter().copied().map(String::from));
            feature_requirements.extend(e.extra_requirements.iter().copied().map(String::from));

            // sort for less diff
            let mut feature_requirements = feature_requirements.into_iter().collect::<Vec<_>>();
            feature_requirements.sort();
            for x in feature_requirements {
                writeln!(w, "#[cfg(feature = \"{x}\")]")?;
            }

            match e.extension {
                Some((tag, _)) => {
                    writeln!(w, "#[rustfmt::skip]")?;
                    write!(w, "pub const VK_{}_{}_BIT_{tag}: {bits_name} = 0x", self.prefix, e.name)?;
                }
                None => {
                    writeln!(w, "#[rustfmt::skip]")?;
                    write!(w, "pub const VK_{}_{}_BIT: {bits_name} = 0x", self.prefix, e.name)?;
                }
            }

            if self.long {
                writeln!(w, "{:016x};", 1u64 << e.bitpos)?;
            } else {
                writeln!(w, "{:08x};", 1u32 << e.bitpos)?;
            }

            if let Some(promoted) = e.promoted {
                writeln!(w, "#[cfg(feature = \"Allow{promoted}APIs\")]")?;
                writeln!(w, "#[rustfmt::skip]")?;
                write!(w, "pub const VK_{}_{}_BIT: {bits_name} = 0x", self.prefix, e.name)?;

                if self.long {
                    writeln!(w, "{:016x};", 1u64 << e.bitpos)?;
                } else {
                    writeln!(w, "{:08x};", 1u32 << e.bitpos)?;
                }
            }
        }

        Ok(())
    }
}

pub struct Object {
    dispatchable: bool,
    name: &'static str,
    object_type_const_name: &'static str,
    object_type_const_value: i32,
    extension: Option<&'static str>,
    promoted: Option<(&'static str, &'static str, &'static str)>,
}
impl Object {
    pub const fn new(name: &'static str, const_suffix: &'static str, object_type_value: i32) -> Self {
        Self {
            dispatchable: false,
            name,
            object_type_const_name: const_suffix,
            object_type_const_value: object_type_value,
            extension: None,
            promoted: None,
        }
    }

    pub const fn dispatchable(mut self) -> Self {
        self.dispatchable = true;
        self
    }

    pub const fn extension(mut self, name: &'static str) -> Self {
        self.extension = Some(name);
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

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        let (value_type, null_expr, raw_expr);
        if self.dispatchable {
            value_type = "*mut core::ffi::c_void";
            null_expr = "core::ptr::null_mut()";
            raw_expr = "self.0 as usize as _";
        } else {
            value_type = "u64";
            null_expr = "0";
            raw_expr = "self.0";
        }

        if let Some(x) = self.extension {
            writeln!(w, "#[cfg(feature = {x:?})]")?;
        }

        writeln!(w, "#[repr(transparent)]")?;
        writeln!(w, "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]")?;
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(
            w,
            "pub struct {}(pub {value_type}, pub core::marker::PhantomData<*mut u8>);",
            self.name
        )?;

        if let Some(x) = self.extension {
            writeln!(w, "#[cfg(feature = {x:?})]")?;
        }
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(w, "impl crate::VkRawHandle for {} {{", self.name)?;
        writeln!(
            w,
            "    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_{};",
            self.object_type_const_name
        )?;
        writeln!(
            w,
            "    const NULL: Self = Self({null_expr}, core::marker::PhantomData);"
        )?;
        w.write(b"\n")?;
        writeln!(w, "    #[inline(always)]")?;
        writeln!(w, "    fn raw_handle_value(&self) -> u64 {{")?;
        writeln!(w, "        {raw_expr}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;

        if let Some(x) = self.extension {
            writeln!(w, "#[cfg(feature = {x:?})]")?;
        }
        writeln!(w, "#[rustfmt::skip]")?;
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

pub struct Struct {
    name: &'static str,
    stype: Option<(&'static str, u32, StructUsage)>,
    members: &'static [StructMember],
    debuggable: bool,
    cloneable: bool,
    copyable: bool,
    equatable: bool,
    hashable: bool,
    extensions: &'static [(&'static str, &'static str)],
    promoted: Option<&'static str>,
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
            extensions: &[],
            promoted: None,
        }
    }

    pub const fn typed(
        name: &'static str,
        sty_suffix: &'static str,
        sty_value: u32,
        sty_usage: StructUsage,
        members: &'static [StructMember],
    ) -> Self {
        Self::new(name, members).stype(sty_suffix, sty_value, sty_usage)
    }

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

    pub const fn extensions(mut self, extensions: &'static [(&'static str, &'static str)]) -> Self {
        self.extensions = extensions;
        self
    }

    pub const fn promoted(mut self, version: &'static str) -> Self {
        self.promoted = Some(version);
        self
    }

    pub const fn member(name: &'static str, r#type: &'static str) -> StructMember {
        StructMember::new(name, r#type)
    }

    fn emit_core(
        w: &mut impl std::io::Write,
        type_name: &str,
        members: &[StructMember],
        usage: Option<StructUsage>,
        debuggable: bool,
        cloneable: bool,
        copyable: bool,
        equatable: bool,
        hashable: bool,
    ) -> std::io::Result<()> {
        let mut derives = Vec::with_capacity(8);
        if debuggable {
            derives.push("Debug");
        }
        if cloneable {
            derives.push("Clone");
        }
        if copyable {
            derives.push("Copy");
        }
        if equatable {
            derives.extend(["Eq", "PartialEq"]);
        }
        if hashable {
            derives.push("Hash");
        }
        if !derives.is_empty() {
            write!(w, "#[derive(")?;
            let mut cont = false;
            for d in derives {
                if cont {
                    w.write(b", ")?;
                }
                w.write(d.as_bytes())?;
                cont = true;
            }
            w.write(b")]\n")?;
        }

        writeln!(w, "#[repr(C)]")?;
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(w, "pub struct {type_name} {{")?;
        if let Some(u) = usage {
            // common headers
            StructMember::new("sType", "VkStructureType").emit(w)?;
            StructMember::new(
                "pNext",
                match u {
                    StructUsage::Source => "*const core::ffi::c_void",
                    StructUsage::Sink => "*mut core::ffi::c_void",
                    // prefer mut pointer
                    StructUsage::Both => "*mut core::ffi::c_void",
                },
            )
            .emit(w)?;
        }
        for m in members {
            m.emit(w)?;
        }
        writeln!(w, "}}")?;

        Ok(())
    }

    fn emit_vulkan_structure_impl(w: &mut impl std::io::Write, type_name: &str) -> std::io::Result<()> {
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(w, "unsafe impl crate::VulkanStructure for {type_name} {{")?;
        writeln!(w, "    #[inline(always)]")?;
        writeln!(w, "    fn as_generic(&self) -> &crate::GenericVulkanStructure {{")?;
        writeln!(w, "        unsafe {{ core::mem::transmute(self) }}")?;
        writeln!(w, "    }}")?;
        writeln!(w)?;
        writeln!(w, "    #[inline(always)]")?;
        writeln!(
            w,
            "    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {{"
        )?;
        writeln!(w, "        unsafe {{ core::mem::transmute(self) }}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;

        Ok(())
    }

    fn emit_vulkan_sink_structure_impl(w: &mut impl std::io::Write, type_name: &str) -> std::io::Result<()> {
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(w, "unsafe impl crate::VulkanSinkStructure for {type_name} {{")?;
        writeln!(w, "    #[inline(always)]")?;
        writeln!(w, "    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure {{")?;
        writeln!(w, "        unsafe {{ core::mem::transmute(self) }}")?;
        writeln!(w, "    }}")?;
        writeln!(w)?;
        writeln!(w, "    #[inline(always)]")?;
        writeln!(
            w,
            "    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure {{"
        )?;
        writeln!(w, "        unsafe {{ core::mem::transmute(self) }}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;

        Ok(())
    }

    fn emit_typed_vulkan_structure_impl(
        w: &mut impl std::io::Write,
        type_name: &str,
        structure_type_name: &str,
    ) -> std::io::Result<()> {
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(w, "unsafe impl crate::TypedVulkanStructure for {type_name} {{")?;
        writeln!(
            w,
            "    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_{structure_type_name};"
        )?;
        writeln!(w, "}}")?;

        Ok(())
    }

    fn emit_typed_vulkan_sink_structure_impl(
        w: &mut impl std::io::Write,
        type_name: &str,
        structure_type_name: &str,
    ) -> std::io::Result<()> {
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(w, "unsafe impl crate::TypedVulkanSinkStructure for {type_name} {{")?;
        writeln!(
            w,
            "    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_{structure_type_name};"
        )?;
        writeln!(w, "}}")?;

        Ok(())
    }

    fn emit_structure_type_const(
        w: &mut impl std::io::Write,
        structure_type_name: &str,
        value: u32,
    ) -> std::io::Result<()> {
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(
            w,
            "pub const VK_STRUCTURE_TYPE_{structure_type_name}: VkStructureType = {value};"
        )
    }

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        if self.extensions.is_empty() {
            assert!(self.promoted.is_none());
            // no extensions: simple define
            let type_name = format!("Vk{}", self.name);

            Self::emit_core(
                w,
                &type_name,
                &self.members,
                self.stype.map(|(_, _, u)| u),
                self.debuggable,
                self.cloneable,
                self.copyable,
                self.equatable,
                self.hashable,
            )?;
            if let Some((up, v, u)) = self.stype {
                Self::emit_structure_type_const(w, up, v)?;
                if u == StructUsage::Source || u == StructUsage::Both {
                    Self::emit_vulkan_structure_impl(w, &type_name)?;
                    Self::emit_typed_vulkan_structure_impl(w, &type_name, up)?;
                }
                if u == StructUsage::Sink || u == StructUsage::Both {
                    Self::emit_vulkan_sink_structure_impl(w, &type_name)?;
                    Self::emit_typed_vulkan_sink_structure_impl(w, &type_name, up)?;
                }
            }

            return Ok(());
        }

        let mut extensions_suffixes = HashMap::new();
        for &(tag, name) in self.extensions {
            extensions_suffixes.entry(tag).or_insert_with(Vec::new).push(name);
        }

        if extensions_suffixes.len() == 1 {
            // single extension with probably promoted
            let (tag, names) = unsafe { extensions_suffixes.drain().next().unwrap_unchecked() };
            let type_name = format!("Vk{}{tag}", self.name);
            let feature_gate = format!(
                "#[cfg(all({}))]",
                names
                    .iter()
                    .map(|name| format!("feature = \"VK_{tag}_{name}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            );

            writeln!(w, "{feature_gate}")?;
            Self::emit_core(
                w,
                &type_name,
                &self.members,
                self.stype.map(|(_, _, u)| u),
                self.debuggable,
                self.cloneable,
                self.copyable,
                self.equatable,
                self.hashable,
            )?;
            if let Some((up, v, u)) = self.stype {
                let structure_type_name = format!("{up}_{tag}");

                writeln!(w, "{feature_gate}")?;
                Self::emit_structure_type_const(w, &structure_type_name, v)?;
                if u == StructUsage::Source || u == StructUsage::Both {
                    writeln!(w, "{feature_gate}")?;
                    Self::emit_vulkan_structure_impl(w, &type_name)?;
                    writeln!(w, "{feature_gate}")?;
                    Self::emit_typed_vulkan_structure_impl(w, &type_name, &structure_type_name)?;
                }
                if u == StructUsage::Sink || u == StructUsage::Both {
                    writeln!(w, "{feature_gate}")?;
                    Self::emit_vulkan_sink_structure_impl(w, &type_name)?;
                    writeln!(w, "{feature_gate}")?;
                    Self::emit_typed_vulkan_sink_structure_impl(w, &type_name, &structure_type_name)?;
                }
            }

            if let Some(pv) = self.promoted {
                let promoted_type_name = format!("Vk{}", self.name);

                writeln!(w, "#[cfg(feature = \"Allow{pv}APIs\")]")?;
                writeln!(w, "#[rustfmt::skip]")?;
                writeln!(w, "pub type {promoted_type_name} = {type_name};")?;

                if let Some((up, _, _)) = self.stype {
                    writeln!(w, "#[cfg(feature = \"Allow{pv}APIs\")]")?;
                    writeln!(w, "#[rustfmt::skip]")?;
                    writeln!(
                        w,
                        "pub const VK_STRUCTURE_TYPE_{up}: VkStructureType = VK_STRUCTURE_TYPE_{up}_{tag};"
                    )?;
                }
            }

            return Ok(());
        }

        unimplemented!();
    }

    pub fn emit_extra_cfg(&self, w: &mut impl std::io::Write, cfg: &str) -> std::io::Result<()> {
        assert!(self.extensions.is_empty(), "struct def has some extensions");
        assert!(self.promoted.is_none());
        // no extensions: simple define
        let type_name = format!("Vk{}", self.name);

        writeln!(w, "#[cfg({cfg})]")?;
        Self::emit_core(
            w,
            &type_name,
            &self.members,
            self.stype.map(|(_, _, u)| u),
            self.debuggable,
            self.cloneable,
            self.copyable,
            self.equatable,
            self.hashable,
        )?;
        if let Some((up, v, u)) = self.stype {
            writeln!(w, "#[cfg({cfg})]")?;
            Self::emit_structure_type_const(w, up, v)?;
            if u == StructUsage::Source || u == StructUsage::Both {
                writeln!(w, "#[cfg({cfg})]")?;
                Self::emit_vulkan_structure_impl(w, &type_name)?;
                writeln!(w, "#[cfg({cfg})]")?;
                Self::emit_typed_vulkan_structure_impl(w, &type_name, up)?;
            }
            if u == StructUsage::Sink || u == StructUsage::Both {
                writeln!(w, "#[cfg({cfg})]")?;
                Self::emit_vulkan_sink_structure_impl(w, &type_name)?;
                writeln!(w, "#[cfg({cfg})]")?;
                Self::emit_typed_vulkan_sink_structure_impl(w, &type_name, up)?;
            }
        }

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
    // pub const fn extension(mut self, name: &'static str, suffix: &'static str) -> Self {
    //     self.extension = Some((name, suffix));
    //     self
    // }

    pub const fn member(name: &'static str, r#type: &'static str) -> StructMember {
        StructMember::new(name, r#type)
    }

    fn emit_ident(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        w.write(b"Vk")?;
        w.write(self.name.as_bytes())?;
        if let Some((_, suffix)) = self.extension {
            w.write(suffix.as_bytes())?;
        }

        Ok(())
    }

    fn emit_extension_gate(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        if let Some((name, _)) = self.extension {
            writeln!(w, "#[cfg(feature = \"{name}\")]")?;
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
                    w.write(b", ")?;
                }
                w.write(d.as_bytes())?;
                cont = true;
            }
            w.write(b")]\n")?;
        }

        writeln!(w, "#[repr(C)]")?;
        writeln!(w, "#[rustfmt::skip]")?;
        w.write(b"pub union ")?;
        self.emit_ident(w)?;
        w.write(b" {\n")?;
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
    extension: Option<(&'static str, &'static str)>,
    extra_requirements: &'static [&'static str],
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
            extension: None,
            extra_requirements: &[],
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
            extension: None,
            extra_requirements: &[],
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

    pub const fn extension(mut self, tag: &'static str, name: &'static str) -> Self {
        self.extension = Some((tag, name));
        self
    }

    pub const fn extra_requirements(mut self, extra_requirements: &'static [&'static str]) -> Self {
        self.extra_requirements = extra_requirements;
        self
    }

    pub const fn promoted(mut self, promoted: &'static str) -> Self {
        self.promoted = Some(promoted);
        self
    }

    fn emit_pfn(w: &mut impl std::io::Write, type_name: &str, org_fn_name: &str) -> std::io::Result<()> {
        writeln!(w, "#[rustfmt::skip]")?;
        writeln!(w, "unsafe impl crate::resolver::PFN for {type_name} {{")?;
        writeln!(w, "    const NAME_CSTR: &'static core::ffi::CStr = c\"{org_fn_name}\";")?;
        writeln!(w)?;
        writeln!(w, "    #[inline(always)]")?;
        writeln!(w, "    unsafe fn from_ptr(p: *const core::ffi::c_void) -> Self {{")?;
        writeln!(w, "        unsafe {{ core::mem::transmute(p) }}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "    #[inline(always)]")?;
        writeln!(w, "    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {{")?;
        writeln!(w, "        unsafe {{ core::mem::transmute(p) }}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;

        Ok(())
    }

    fn emit_feature_gate(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        w.write(b"#[cfg(feature = \"Implements\")]\n")?;
        if let Some((tag, name)) = self.extension {
            writeln!(w, "#[cfg(feature = \"VK_{tag}_{name}\")]")?;
        }
        if let Some(v) = self.version_since {
            writeln!(w, "#[cfg(feature = \"Allow{v}APIs\")]")?;
        }

        for x in self.extra_requirements {
            writeln!(w, "#[cfg(feature = \"{x}\")]")?;
        }

        Ok(())
    }

    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        let (type_name, org_fn_name);
        match (self.is_command_buffer_inst, self.extension) {
            (false, None) => {
                type_name = format!("PFN_vk{}", self.name);
                org_fn_name = format!("vk{}", self.name);
            }
            (true, None) => {
                type_name = format!("PFN_vkCmd{}", self.name);
                org_fn_name = format!("vkCmd{}", self.name);
            }
            (false, Some((tag, _))) => {
                type_name = format!("PFN_vk{}{tag}", self.name);
                org_fn_name = format!("vk{}{tag}", self.name);
            }
            (true, Some((tag, _))) => {
                type_name = format!("PFN_vkCmd{}{tag}", self.name);
                org_fn_name = format!("vkCmd{}{tag}", self.name);
            }
        }

        self.emit_feature_gate(w)?;
        writeln!(w, "#[repr(transparent)]")?;
        writeln!(w, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]")?;
        writeln!(w, "#[rustfmt::skip]")?;
        write!(w, "pub struct {type_name}(pub unsafe extern \"system\" fn(")?;
        let mut cont = false;
        if self.is_command_buffer_inst {
            // command buffer instruction always takes a VkCommandBuffer as the first argument
            write!(w, "commandBuffer: VkCommandBuffer")?;
            cont = true;
        }
        for (n, t) in self.args {
            if cont {
                w.write(b", ")?;
            }
            write!(w, "{n}: {t}")?;
            cont = true;
        }
        w.write(b")")?;
        if let Some(return_type) = self.return_type {
            write!(w, " -> {return_type}")?;
        }
        w.write(b");\n")?;

        self.emit_feature_gate(w)?;
        Self::emit_pfn(w, &type_name, &org_fn_name)?;

        if self.static_callable {
            w.write(b"#[cfg(not(feature = \"DynamicLoaded\"))]\n")?;
            self.emit_feature_gate(w)?;
            writeln!(w, "#[rustfmt::skip]")?;
            writeln!(w, "impl crate::resolver::StaticCallable for {type_name} {{")?;
            writeln!(w, "    const STATIC: Self = Self({org_fn_name});")?;
            writeln!(w, "}}")?;
        }

        if let Some(p) = self.promoted {
            let (type_name, org_fn_name);
            match self.is_command_buffer_inst {
                false => {
                    type_name = format!("PFN_vk{}", self.name);
                    org_fn_name = format!("vk{}", self.name);
                }
                true => {
                    type_name = format!("PFN_vkCmd{}", self.name);
                    org_fn_name = format!("vkCmd{}", self.name);
                }
            }

            writeln!(w, "#[cfg(feature = \"Implements\")]")?;
            writeln!(w, "#[cfg(feature = \"Allow{p}APIs\")]")?;
            writeln!(w, "#[repr(transparent)]")?;
            writeln!(w, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]")?;
            writeln!(w, "#[rustfmt::skip]")?;
            write!(w, "pub struct {type_name}(pub unsafe extern \"system\" fn(")?;
            let mut cont = false;
            if self.is_command_buffer_inst {
                // command buffer instruction always takes a VkCommandBuffer as the first argument
                write!(w, "commandBuffer: VkCommandBuffer")?;
                cont = true;
            }
            for (n, t) in self.args {
                if cont {
                    w.write(b", ")?;
                }
                write!(w, "{n}: {t}")?;
                cont = true;
            }
            w.write(b")")?;
            if let Some(return_type) = self.return_type {
                write!(w, " -> {return_type}")?;
            }
            w.write(b");\n")?;

            writeln!(w, "#[cfg(feature = \"Implements\")]")?;
            writeln!(w, "#[cfg(feature = \"Allow{p}APIs\")]")?;
            Self::emit_pfn(w, &type_name, &org_fn_name)?;

            // promoted symbols always static callable
            writeln!(w, "#[cfg(feature = \"Implements\")]")?;
            w.write(b"#[cfg(not(feature = \"DynamicLoaded\"))]\n")?;
            writeln!(w, "#[cfg(feature = \"Allow{p}APIs\")]")?;
            writeln!(w, "#[rustfmt::skip]")?;
            writeln!(w, "impl crate::resolver::StaticCallable for {type_name} {{")?;
            writeln!(w, "    const STATIC: Self = Self({org_fn_name});")?;
            writeln!(w, "}}")?;
        }

        Ok(())
    }

    pub fn emit_static_symbol(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        if self.static_callable {
            if let Some((tag, name)) = self.extension {
                writeln!(w, "    #[cfg(feature = \"VK_{tag}_{name}\")]")?;
            }

            if let Some(v) = self.version_since {
                writeln!(w, "    #[cfg(feature = \"Allow{v}APIs\")]")?;
            }

            match (self.is_command_buffer_inst, self.extension) {
                (false, None) => write!(w, "    pub fn vk{}(", self.name)?,
                (true, None) => write!(w, "    pub fn vkCmd{}(", self.name)?,
                (false, Some((tag, _))) => write!(w, "    pub fn vk{}{tag}(", self.name)?,
                (true, Some((tag, _))) => write!(w, "    pub fn vkCmd{}{tag}(", self.name)?,
            }
            let mut cont = false;
            if self.is_command_buffer_inst {
                // command buffer instruction always takes a VkCommandBuffer as the first argument
                write!(w, "commandBuffer: VkCommandBuffer")?;
                cont = true;
            }
            for (n, t) in self.args {
                if cont {
                    w.write(b", ")?;
                }
                write!(w, "{n}: {t}")?;
                cont = true;
            }
            w.write(b")")?;
            if let Some(return_type) = self.return_type {
                write!(w, " -> {return_type}")?;
            }
            w.write(b";\n")?;
        }

        if let Some(p) = self.promoted {
            writeln!(w, "    #[cfg(feature = \"Allow{p}APIs\")]")?;

            match self.is_command_buffer_inst {
                false => write!(w, "    pub fn vk{}(", self.name)?,
                true => write!(w, "    pub fn vkCmd{}(", self.name)?,
            }
            let mut cont = false;
            if self.is_command_buffer_inst {
                // command buffer instruction always takes a VkCommandBuffer as the first argument
                write!(w, "commandBuffer: VkCommandBuffer")?;
                cont = true;
            }
            for (n, t) in self.args {
                if cont {
                    w.write(b", ")?;
                }
                write!(w, "{n}: {t}")?;
                cont = true;
            }
            w.write(b")")?;
            if let Some(return_type) = self.return_type {
                write!(w, " -> {return_type}")?;
            }
            w.write(b";\n")?;
        }

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
            "pub const {}_EXTENSION_NAME: &'static str = \"{}\";",
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
    Bitmask(Bitmask),
    Enum(Enum),
    FuncPointer(FuncPointer),
    Object(Object),
    Struct(Struct),
    Union(Union),
    Command(Command),
}
impl Element {
    pub fn emit(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            Self::ExtensionHeaderConstants(x) => x.emit(w),
            Self::Bitmask(x) => x.emit(w),
            Self::Enum(x) => x.emit(w),
            Self::FuncPointer(x) => x.emit(w),
            Self::Object(x) => x.emit(w),
            Self::Struct(x) => x.emit(w),
            Self::Union(x) => x.emit(w),
            Self::Command(x) => x.emit(w),
        }
    }
}
