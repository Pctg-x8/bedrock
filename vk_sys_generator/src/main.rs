use std::{collections::HashMap, io::Write};

use parts::{
    Bitmask, Command, Enum, ExtensionHeaderConstants, FuncPointer, Object, Struct, StructUsage, TypeAlias, Union,
    emit_c_enum_type, emit_const,
};

use crate::{
    extensions::{VK_KHR_DISPLAY_SWAPCHAIN, VK_KHR_SURFACE, VK_KHR_SWAPCHAIN},
    parts::EnumType,
    rs_item::{
        CompilationCondition, Constant, ConstantSymbol, FeatureName, FnSymbol, FunctionPtrNewtype, FunctionStub,
        RustCodeEmitter, TypeSymbol,
    },
    v1_1::{
        VK_KHR_BIND_MEMORY_2, VK_KHR_DEVICE_GROUP, VK_KHR_EXTERNAL_MEMORY, VK_KHR_MAINTENANCE_1,
        VK_KHR_SAMPLER_YCBCR_CONVERSION,
    },
    v1_4::VK_KHR_MAINTENANCE_5,
};

mod extensions;
mod parts;
mod rs_item;
mod v1_1;
mod v1_2;
mod v1_3;
mod v1_4;

fn main() -> std::io::Result<()> {
    let mut generator = CodeGenerator::new();
    let mut o = std::io::stdout().lock();
    o.write_all(
        HEADER
            .replace("**HEADER_MINOR_VERSION**", "4")
            .replace("**HEADER_VERSION**", "305")
            .as_bytes(),
    )?;

    for ta in TYPE_ALIASES {
        ta.emit(&mut o)?;
    }

    for f in FLAGS {
        f.emit(&mut generator);
    }

    // flags extra constants
    emit_const(
        &mut o,
        "VK_SHADER_STAGE_ALL_GRAPHICS",
        "VkShaderStageFlags",
        &format!("0x{:08x}", (1u32 << 5) - 1),
    )?;
    emit_const(&mut o, "VK_SHADER_STAGE_ALL", "VkShaderStageFlags", "0x7fffffff")?;
    emit_const(&mut o, "VK_CULL_MODE_NONE", "VkCullModeFlags", "0")?;
    emit_const(&mut o, "VK_CULL_MODE_FRONT_AND_BACK", "VkCullModeFlags", "3")?;
    emit_const(&mut o, "VK_STENCIL_FACE_FRONT_AND_BACK", "VkStencilFaceFlags", "3")?;
    o.write_all(b"#[cfg(feature = \"VK_KHR_synchronization2\")]\n")?;
    emit_const(
        &mut o,
        "VK_PIPELINE_STAGE_2_NONE_KHR",
        "VkPipelineStageFlagBits2KHR",
        "0",
    )?;
    o.write_all(b"#[cfg(feature = \"Allow1_3APIs\")]\n")?;
    emit_const(&mut o, "VK_PIPELINE_STAGE_2_NONE", "VkPipelineStageFlagBits2", "0")?;
    o.write_all(b"#[cfg(feature = \"VK_KHR_synchronization2\")]\n")?;
    emit_const(&mut o, "VK_ACCESS_2_NONE_KHR", "VkAccessFlagBits2KHR", "0")?;
    o.write_all(b"#[cfg(feature = \"Allow1_3APIs\")]\n")?;
    emit_const(&mut o, "VK_ACCESS_2_NONE", "VkAccessFlagBits2", "0")?;

    for obj in OBJECTS {
        o.write_all(b"\n")?;
        obj.emit(&mut o)?;
    }

    emit_const(&mut o, "VK_LOD_CLAMP_NONE", "f32", "1000.0")?;
    emit_const(&mut o, "VK_REMAINING_MIP_LEVELS", "u32", "0xffff_ffff")?;
    emit_const(&mut o, "VK_REMAINING_ARRAY_LAYERS", "u32", "0xffff_ffff")?;
    emit_const(&mut o, "VK_WHOLE_SIZE", "u64", "!0")?;
    emit_const(&mut o, "VK_ATTACHMENT_UNUSED", "u32", "0xffff_ffff")?;
    emit_const(&mut o, "VK_TRUE", "VkBool32", "1")?;
    emit_const(&mut o, "VK_FALSE", "VkBool32", "0")?;
    emit_const(&mut o, "VK_QUEUE_FAMILY_IGNORED", "u32", "0xffff_ffff")?;
    emit_const(&mut o, "VK_SUBPASS_EXTERNAL", "u32", "0xffff_ffff")?;
    emit_const(&mut o, "VK_MAX_PHYSICAL_DEVICE_NAME_SIZE", "usize", "256")?;
    emit_const(&mut o, "VK_UUID_SIZE", "usize", "16")?;
    emit_const(&mut o, "VK_MAX_MEMORY_TYPES", "usize", "32")?;
    emit_const(&mut o, "VK_MAX_MEMORY_HEAPS", "usize", "16")?;
    emit_const(&mut o, "VK_MAX_EXTENSION_NAME_SIZE", "usize", "256")?;
    emit_const(&mut o, "VK_MAX_DESCRIPTION_SIZE", "usize", "256")?;

    writeln!(
        o,
        r#"#[cfg(any(feature = "VK_KHR_external_fence_capabilities", feature = "VK_KHR_external_memory_capabilities", feature = "VK_KHR_external_semaphore_capabilities"))]"#
    )?;
    emit_const(&mut o, "VK_LUID_SIZE_KHR", "usize", "8")?;
    writeln!(o, r#"#[cfg(feature = "Allow1_1APIs")]"#)?;
    emit_const(&mut o, "VK_LUID_SIZE", "usize", "8")?;

    o.write_all(b"#[cfg(feature = \"VK_KHR_external_memory\")]")?;
    emit_const(&mut o, "VK_QUEUE_FAMILY_EXTERNAL_KHR", "u32", "!1")?;
    o.write_all(b"#[cfg(feature = \"Allow1_1APIs\")]")?;
    emit_const(&mut o, "VK_QUEUE_FAMILY_EXTERNAL", "u32", "!1")?;
    o.write_all(b"#[cfg(feature = \"VK_EXT_queue_family_foreign\")]")?;
    emit_const(&mut o, "VK_QUEUE_FAMILY_FOREIGN_EXT", "u32", "!0u32 - 2")?;

    o.write_all(b"#[cfg(feature = \"VK_KHR_device_group_creation\")]")?;
    emit_const(&mut o, "VK_MAX_DEVICE_GROUP_SIZE_KHR", "usize", "32")?;
    o.write_all(b"#[cfg(feature = \"Allow1_1APIs\")]")?;
    emit_const(&mut o, "VK_MAX_DEVICE_GROUP_SIZE", "usize", "32")?;

    o.write_all(b"#[cfg(feature = \"VK_KHR_global_priority\")]")?;
    emit_const(&mut o, "VK_MAX_GLOBAL_PRIORITY_SIZE_KHR", "usize", "16")?;
    o.write_all(b"#[cfg(feature = \"Allow1_4APIs\")]")?;
    emit_const(&mut o, "VK_MAX_GLOBAL_PRIORITY_SIZE", "usize", "16")?;

    o.write_all(b"#[cfg(feature = \"VK_KHR_depth_stencil_resolve\")] #[rustfmt::skip] pub const VK_RESOLVE_MODE_NONE_KHR: VkResolveModeFlagBitsKHR = 0;\n")?;
    o.write_all(b"#[cfg(feature = \"Allow1_2APIs\")] #[rustfmt::skip] pub const VK_RESOLVE_MODE_NONE: VkResolveModeFlagBitsKHR = 0;\n")?;

    o.write_all(b"\n")?;
    emit_c_enum_type(&mut o, "VkStructureType")?;
    emit_c_enum_type(&mut o, "VkObjectType")?;

    emit_enums(&mut generator);
    emit_result_type(&mut generator);
    emit_format_enum(&mut generator);

    for f in FUNC_POINTERS {
        o.write_all(b"\n")?;
        f.emit(&mut o)?;
    }

    for s in STRUCTS {
        s.emit(&mut generator);
    }

    for x in COMMANDS {
        x.emit(&mut generator);
    }

    // chaotic requirement structure
    Struct::typed(
        "PhysicalDeviceIDPropertiesKHR",
        "PHYSICAL_DEVICE_ID_PROPERTIES_KHR",
        vk_ext_enum(72, 4) as _,
        StructUsage::Sink,
        &const {
            [
                Struct::member("deviceUUID", "[u8; VK_UUID_SIZE]"),
                Struct::member("driverUUID", "[u8; VK_UUID_SIZE]"),
                Struct::member("deviceLUID", "[u8; VK_LUID_SIZE_KHR]"),
                Struct::member("deviceNodeMask", "u32"),
                Struct::member("deviceLUIDValid", "VkBool32"),
            ]
        },
    )
    .emit_extra_cfg(
        &mut generator,
        CompilationCondition::any([
            CompilationCondition::Feature(FeatureName::VulkanExt {
                tag: "KHR",
                name: "external_fence_capabilities",
            }),
            CompilationCondition::Feature(FeatureName::VulkanExt {
                tag: "KHR",
                name: "external_memory_capabilities",
            }),
            CompilationCondition::Feature(FeatureName::VulkanExt {
                tag: "KHR",
                name: "external_semaphore_capabilities",
            }),
        ]),
    );
    generator.emit_type_alias(rs_item::TypeAlias {
        compilation_condition: CompilationCondition::Feature(FeatureName::AllowApiVersion("1_1")),
        target_name: TypeSymbol {
            stem: "PhysicalDeviceIDProperties",
            suffix: None,
        },
        source_name: rs_item::Type::Defined(TypeSymbol {
            stem: "PhysicalDeviceIDProperties",
            suffix: Some("KHR"),
        }),
    });
    // TODO: const aliasing(そもそもこれ普通にpromoteできないか？)
    o.write_all(b"\n")?;
    o.write_all(b"#[cfg(feature = \"Allow1_1APIs\")]\n")?;
    o.write_all(b"#[rustfmt::skip]\n")?;
    o.write_all(b"pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR;\n")?;

    // struct aliasing
    generator.emit_type_alias(rs_item::TypeAlias {
        compilation_condition: CompilationCondition::Feature(FeatureName::VulkanExt {
            tag: "KHR",
            name: "variable_pointers",
        }),
        target_name: TypeSymbol {
            stem: "PhysicalDeviceVariablePointerFeatures",
            suffix: Some("KHR"),
        },
        source_name: rs_item::Type::Defined(TypeSymbol {
            stem: "PhysicalDeviceVariablePointersFeatures",
            suffix: Some("KHR"),
        }),
    });
    // TODO: const aliasing
    o.write_all(b"#[cfg(feature = \"VK_KHR_variable_pointers\")]\n")?;
    o.write_all(b"#[rustfmt::skip]\n")?;
    o.write_all(b"pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;\n")?;

    for s in UNIONS {
        o.write_all(b"\n")?;
        s.emit(&mut o)?;
    }

    for c in EXTENSION_HEADER_CONSTANTS {
        o.write_all(b"\n")?;
        c.emit(&mut o)?;
    }

    v1_1::emit(&mut generator);
    for x in v1_1::ELEMENTS {
        x.emit(&mut generator, &mut o)?;
    }

    for x in v1_2::ELEMENTS {
        x.emit(&mut generator, &mut o)?;
    }

    for x in v1_3::ELEMENTS {
        x.emit(&mut generator, &mut o)?;
    }

    for x in v1_4::ELEMENTS {
        x.emit(&mut generator, &mut o)?;
    }

    extensions::emit(&mut generator);
    for x in extensions::ELEMENTS {
        x.emit(&mut generator, &mut o)?;
    }

    generator.generate(&mut o)?;

    Ok(())
}

enum TypeDef<'s> {
    Struct(rs_item::Struct<'s>),
    Alias(rs_item::TypeAlias<'s>),
}
impl TypeDef<'_> {
    const fn define_name(&self) -> &TypeSymbol<'_> {
        match self {
            Self::Struct(x) => &x.name,
            Self::Alias(x) => &x.target_name,
        }
    }

    #[inline(always)]
    fn emit(&self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        match self {
            Self::Struct(x) => x.emit(w),
            Self::Alias(x) => x.emit(w),
        }
    }
}

struct CodeGenerator {
    fntable: HashMap<FnSymbol<'static>, FunctionStub<'static>>,
    function_ptr_newtypes: HashMap<FnSymbol<'static>, FunctionPtrNewtype<'static>>,
    consts: HashMap<ConstantSymbol<'static>, Constant<'static>>,
    types: HashMap<TypeSymbol<'static>, TypeDef<'static>>,
}
impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            fntable: HashMap::new(),
            function_ptr_newtypes: HashMap::new(),
            consts: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub fn generate(self, w: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        let mut sorted = self.consts.into_values().collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
        for c in sorted {
            c.emit(w)?;
            w.write_all(b"\n")?;
        }

        let mut sorted = self.types.into_values().collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.define_name().cmp(b.define_name()));
        for c in sorted {
            c.emit(w)?;
            w.write_all(b"\n")?;
        }

        let mut sorted = self.function_ptr_newtypes.into_values().collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
        for x in sorted {
            x.emit(w)?;
            w.write_all(b"\n")?;
        }

        w.write_all(b"\n")?;
        w.write_all(b"#[cfg(all(feature = \"Implements\", not(feature = \"DynamicLoaded\")))]\n")?;
        w.write_all(b"#[cfg_attr(all(not(feature = \"DynamicLoaded\"), feature = \"Implements\", not(windows)), link(name = \"vulkan\"))]\n")?;
        w.write_all(b"#[cfg_attr(all(not(feature = \"DynamicLoaded\"), feature = \"Implements\", windows), link(name = \"vulkan-1\"))]\n")?;
        w.write_all(b"#[rustfmt::skip]\n")?;
        w.write_all(b"unsafe extern \"system\" {\n")?;
        let mut sorted = self.fntable.into_values().collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
        for f in sorted {
            w.write_all(b"    ")?;
            f.emit(w)?;
            w.write_all(b"\n")?;
        }
        w.write_all(b"}\n")?;

        Ok(())
    }
}
impl RustCodeEmitter for CodeGenerator {
    fn emit_const(&mut self, e: Constant<'static>) {
        match self.consts.entry(e.name.clone()) {
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(e);
            }
            std::collections::hash_map::Entry::Occupied(mut o) => {
                o.get_mut().compilation_condition =
                    core::mem::replace(&mut o.get_mut().compilation_condition, CompilationCondition::Empty)
                        .or(e.compilation_condition);
            }
        }
    }

    fn emit_struct(&mut self, e: rs_item::Struct<'static>) {
        match self.types.entry(e.name.clone()) {
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(TypeDef::Struct(e));
            }
            std::collections::hash_map::Entry::Occupied(mut o) => match o.get_mut() {
                TypeDef::Struct(o) => {
                    o.compilation_condition =
                        core::mem::replace(&mut o.compilation_condition, CompilationCondition::Empty)
                            .or(e.compilation_condition);
                }
                _ => panic!("different kind of type item defined by same name"),
            },
        }
    }

    fn emit_type_alias(&mut self, e: rs_item::TypeAlias<'static>) {
        match self.types.entry(e.target_name.clone()) {
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(TypeDef::Alias(e));
            }
            std::collections::hash_map::Entry::Occupied(mut o) => match o.get_mut() {
                TypeDef::Alias(o) => {
                    // TODO: source_nameの型が同型かみたほうがいいかも
                    o.compilation_condition =
                        core::mem::replace(&mut o.compilation_condition, CompilationCondition::Empty)
                            .or(e.compilation_condition);
                }
                _ => panic!("different kind of type item defined by same name"),
            },
        }
    }

    fn emit_function_ptr_newtype(&mut self, e: rs_item::FunctionPtrNewtype<'static>) {
        match self.function_ptr_newtypes.entry(e.name.clone()) {
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(e);
            }
            std::collections::hash_map::Entry::Occupied(mut o) => {
                o.get_mut().compilation_condition =
                    core::mem::replace(&mut o.get_mut().compilation_condition, CompilationCondition::Empty)
                        .or(e.compilation_condition);
            }
        }
    }

    fn emit_function_stub(&mut self, e: rs_item::FunctionStub<'static>) {
        match self.fntable.entry(e.name.clone()) {
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(e);
            }
            std::collections::hash_map::Entry::Occupied(mut o) => {
                o.get_mut().compilation_condition =
                    core::mem::replace(&mut o.get_mut().compilation_condition, CompilationCondition::Empty)
                        .or(e.compilation_condition);
            }
        }
    }
}

#[allow(clippy::inconsistent_digit_grouping)]
const fn vk_ext_enum(extnumber: i32, offset: i32) -> i32 {
    if extnumber == 0 {
        return offset;
    }

    100_0000_000 + (extnumber - 1) * 1000 + offset
}

const HEADER: &str = include_str!("../res/common_header.rs.txt");

const DEVICE_SIZE_TYPE: &str = "VkDeviceSize";
const DEVICE_ADDR_TYPE: &str = "VkDeviceAddress";

const EXTENSION_HEADER_CONSTANTS: &[ExtensionHeaderConstants] = &[
    ExtensionHeaderConstants::new("VK_KHR_surface", 25),
    ExtensionHeaderConstants::new("VK_KHR_swapchain", 68),
    ExtensionHeaderConstants::new("VK_KHR_display", 21),
    ExtensionHeaderConstants::new("VK_KHR_xlib_surface", 6),
    ExtensionHeaderConstants::new("VK_KHR_xcb_surface", 6),
    ExtensionHeaderConstants::new("VK_KHR_wayland_surface", 6),
    ExtensionHeaderConstants::new("VK_KHR_android_surface", 6),
    ExtensionHeaderConstants::new("VK_KHR_win32_surface", 6),
    ExtensionHeaderConstants::new("VK_EXT_metal_surface", 1),
    ExtensionHeaderConstants::new("VK_KHR_win32_keyed_mutex", 1),
    ExtensionHeaderConstants::new("VK_KHR_get_surface_capabilities2", 1),
    ExtensionHeaderConstants::new("VK_KHR_device_group_creation", 1),
    ExtensionHeaderConstants::new("VK_KHR_multiview", 1),
    ExtensionHeaderConstants::new("VK_KHR_shader_draw_parameters", 1),
    ExtensionHeaderConstants::new("VK_KHR_portability_enumeration", 1),
    ExtensionHeaderConstants::new("VK_KHR_relaxed_block_layout", 1),
    ExtensionHeaderConstants::new("VK_KHR_storage_buffer_storage_class", 1),
    ExtensionHeaderConstants::new("VK_KHR_variable_pointers", 1),
    ExtensionHeaderConstants::new("VK_KHR_16bit_storage", 1),
    ExtensionHeaderConstants::new("VK_KHR_maintenance1", 2),
    ExtensionHeaderConstants::new("VK_KHR_maintenance2", 1),
    ExtensionHeaderConstants::new("VK_KHR_maintenance3", 1),
    ExtensionHeaderConstants::new("VK_KHR_synchronization2", 1),
];

const TYPE_ALIASES: &[TypeAlias] = &[
    TypeAlias::new("VkSampleMask", "u32"),
    TypeAlias::new("VkBool32", "u32"),
    TypeAlias::new("VkFlags", "u32"),
    TypeAlias::new("VkFlags64", "u64"),
    TypeAlias::new(DEVICE_SIZE_TYPE, "u64"),
    TypeAlias::new(DEVICE_ADDR_TYPE, "u64"),
];

const OBJECTS: &[Object] = &[
    Object::new("VkInstance", "INSTANCE", 1).dispatchable(),
    Object::new("VkPhysicalDevice", "PHYSICAL_DEVICE", 2).dispatchable(),
    Object::new("VkDevice", "DEVICE", 3).dispatchable(),
    Object::new("VkQueue", "QUEUE", 4).dispatchable(),
    Object::new("VkCommandBuffer", "COMMAND_BUFFER", 6).dispatchable(),
    Object::new("VkDeviceMemory", "DEVICE_MEMORY", 8),
    Object::new("VkCommandPool", "COMMAND_POOL", 25),
    Object::new("VkBuffer", "BUFFER", 9),
    Object::new("VkBufferView", "BUFFER_VIEW", 13),
    Object::new("VkImage", "IMAGE", 10),
    Object::new("VkImageView", "IMAGE_VIEW", 14),
    Object::new("VkShaderModule", "SHADER_MODULE", 15),
    Object::new("VkPipeline", "PIPELINE", 19),
    Object::new("VkPipelineLayout", "PIPELINE_LAYOUT", 17),
    Object::new("VkSampler", "SAMPLER", 21),
    Object::new("VkDescriptorSet", "DESCRIPTOR_SET", 23),
    Object::new("VkDescriptorSetLayout", "DESCRIPTOR_SET_LAYOUT", 20),
    Object::new("VkDescriptorPool", "DESCRIPTOR_POOL", 22),
    Object::new(
        "VkDescriptorUpdateTemplateKHR",
        "DESCRIPTOR_UPDATE_TEMPLATE_KHR",
        vk_ext_enum(86, 0),
    )
    .extension_old("VK_KHR_descriptor_update_template")
    .promoted("1_1", "VkDescriptorUpdateTemplate", "DESCRIPTOR_UPDATE_TEMPLATE"),
    Object::new("VkFence", "FENCE", 7),
    Object::new("VkSemaphore", "SEMAPHORE", 5),
    Object::new("VkEvent", "EVENT", 11),
    Object::new("VkQueryPool", "QUERY_POOL", 12),
    Object::new("VkFramebuffer", "FRAMEBUFFER", 24),
    Object::new("VkRenderPass", "RENDER_PASS", 18),
    Object::new("VkPipelineCache", "PIPELINE_CACHE", 16),
    // WSI extensions
    Object::new("VkDisplayKHR", "DISPLAY_KHR", vk_ext_enum(3, 0)).extension_old("VK_KHR_display"),
    Object::new("VkDisplayModeKHR", "DISPLAY_MODE_KHR", vk_ext_enum(3, 1)).extension_old("VK_KHR_display"),
    Object::new("VkSurfaceKHR", "SURFACE_KHR", vk_ext_enum(1, 0)).extension_old("VK_KHR_surface"),
    Object::new("VkSwapchainKHR", "SWAPCHAIN_KHR", vk_ext_enum(2, 0)).extension_old("VK_KHR_swapchain"),
    // debug report
    Object::new(
        "VkDebugReportCallbackEXT",
        "DEBUG_REPORT_CALLBACK_EXT",
        vk_ext_enum(12, 0),
    )
    .extension_old("VK_EXT_debug_report"),
    Object::new(
        "VkDebugUtilsMessengerEXT",
        "DEBUG_UTILS_MESSENGER_EXT",
        vk_ext_enum(129, 0),
    )
    .extension_old("VK_EXT_debug_utils"),
];

pub static ATTACHMENT_LOAD_OP: EnumType = EnumType::new("AttachmentLoadOp", "ATTACHMENT_LOAD_OP");
pub static ATTACHMENT_STORE_OP: EnumType = EnumType::new("AttachmentStoreOp", "ATTACHMENT_STORE_OP");
pub static BLEND_FACTOR: EnumType = EnumType::new("BlendFactor", "BLEND_FACTOR");
pub static BLEND_OP: EnumType = EnumType::new("BlendOp", "BLEND_OP");
pub static BORDER_COLOR: EnumType = EnumType::new("BorderColor", "BORDER_COLOR");
pub static COMMAND_BUFFER_LEVEL: EnumType = EnumType::new("CommandBufferLevel", "COMMAND_BUFFER_LEVEL");
pub static COMPARE_OP: EnumType = EnumType::new("CompareOp", "COMPARE_OP");
pub static COMPONENT_SWIZZLE: EnumType = EnumType::new("ComponentSwizzle", "COMPONENT_SWIZZLE");
pub static DESCRIPTOR_TYPE: EnumType = EnumType::new("DescriptorType", "DESCRIPTOR_TYPE");
pub static DYNAMIC_STATE: EnumType = EnumType::new("DynamicState", "DYNAMIC_STATE");
pub static FILTER: EnumType = EnumType::new("Filter", "FILTER");
pub static FRONT_FACE: EnumType = EnumType::new("FrontFace", "FRONT_FACE");
pub static IMAGE_LAYOUT: EnumType = EnumType::new("ImageLayout", "IMAGE_LAYOUT");
pub static IMAGE_TILING: EnumType = EnumType::new("ImageTiling", "IMAGE_TILING");
pub static IMAGE_TYPE: EnumType = EnumType::new("ImageType", "IMAGE_TYPE");
pub static IMAGE_VIEW_TYPE: EnumType = EnumType::new("ImageViewType", "IMAGE_VIEW_TYPE");
pub static INDEX_TYPE: EnumType = EnumType::new("IndexType", "INDEX_TYPE");
pub static LOGIC_OP: EnumType = EnumType::new("LogicOp", "LOGIC_OP");
pub static INTERNAL_ALLOCATION_TYPE: EnumType = EnumType::new("InternalAllocationType", "INTERNAL_ALLOCATION_TYPE");
pub static PHYSICAL_DEVICE_TYPE: EnumType = EnumType::new("PhysicalDeviceType", "PHYSICAL_DEVICE_TYPE");
pub static PIPELINE_BIND_POINT: EnumType = EnumType::new("PipelineBindPoint", "PIPELINE_BIND_POINT");
pub static POLYGON_MODE: EnumType = EnumType::new("PolygonMode", "POLYGON_MODE");
pub static PRIMITIVE_TOPOLOGY: EnumType = EnumType::new("PrimitiveTopology", "PRIMITIVE_TOPOLOGY");
pub static QUERY_TYPE: EnumType = EnumType::new("QueryType", "QUERY_TYPE");
pub static SAMPLER_ADDRESS_MODE: EnumType = EnumType::new("SamplerAddressMode", "SAMPLER_ADDRESS_MODE");
pub static SAMPLER_MIPMAP_MODE: EnumType = EnumType::new("SamplerMipmapMode", "SAMPLER_MIPMAP_MODE");
pub static SHARING_MODE: EnumType = EnumType::new("SharingMode", "SHARING_MODE");
pub static STENCIL_OP: EnumType = EnumType::new("StencilOp", "STENCIL_OP");
pub static SUBPASS_CONTENTS: EnumType = EnumType::new("SubpassContents", "SUBPASS_CONTENTS");
pub static SYSTEM_ALLOCATION_SCOPE: EnumType = EnumType::new("SystemAllocationScope", "SYSTEM_ALLOCATION_SCOPE");
pub static VERTEX_INPUT_RATE: EnumType = EnumType::new("VertexInputRate", "VERTEX_INPUT_RATE");

fn emit_enums(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    ATTACHMENT_LOAD_OP.emit(emitter);
    ATTACHMENT_LOAD_OP.member("LOAD", 0).emit(emitter);
    ATTACHMENT_LOAD_OP.member("CLEAR", 1).emit(emitter);
    ATTACHMENT_LOAD_OP.member("DONT_CARE", 2).emit(emitter);

    ATTACHMENT_STORE_OP.emit(emitter);
    ATTACHMENT_STORE_OP.member("STORE", 0).emit(emitter);
    ATTACHMENT_STORE_OP.member("DONT_CARE", 1).emit(emitter);

    BLEND_FACTOR.emit(emitter);
    BLEND_FACTOR.member("ZERO", 0).emit(emitter);
    BLEND_FACTOR.member("ONE", 1).emit(emitter);
    BLEND_FACTOR.member("SRC_COLOR", 2).emit(emitter);
    BLEND_FACTOR.member("ONE_MINUS_SRC_COLOR", 3).emit(emitter);
    BLEND_FACTOR.member("DST_COLOR", 4).emit(emitter);
    BLEND_FACTOR.member("ONE_MINUS_DST_COLOR", 5).emit(emitter);
    BLEND_FACTOR.member("SRC_ALPHA", 6).emit(emitter);
    BLEND_FACTOR.member("ONE_MINUS_SRC_ALPHA", 7).emit(emitter);
    BLEND_FACTOR.member("DST_ALPHA", 8).emit(emitter);
    BLEND_FACTOR.member("ONE_MINUS_DST_ALPHA", 9).emit(emitter);
    BLEND_FACTOR.member("CONSTANT_COLOR", 10).emit(emitter);
    BLEND_FACTOR.member("ONE_MINUS_CONSTANT_COLOR", 11).emit(emitter);
    BLEND_FACTOR.member("CONSTANT_ALPHA", 12).emit(emitter);
    BLEND_FACTOR.member("ONE_MINUS_CONSTANT_ALPHA", 13).emit(emitter);
    BLEND_FACTOR.member("SRC_ALPHA_STAURATE", 14).emit(emitter);
    BLEND_FACTOR.member("SRC1_COLOR", 15).emit(emitter);
    BLEND_FACTOR.member("ONE_MINUS_SRC1_COLOR", 16).emit(emitter);
    BLEND_FACTOR.member("SRC1_ALPHA", 17).emit(emitter);
    BLEND_FACTOR.member("ONE_MINUS_SRC1_ALPHA", 18).emit(emitter);

    BLEND_OP.emit(emitter);
    BLEND_OP.member("ADD", 0).emit(emitter);
    BLEND_OP.member("SUBTRACT", 1).emit(emitter);
    BLEND_OP.member("REVERSE_SUBTRACT", 2).emit(emitter);
    BLEND_OP.member("MIN", 3).emit(emitter);
    BLEND_OP.member("MAX", 4).emit(emitter);

    BORDER_COLOR.emit(emitter);
    BORDER_COLOR.member("FLOAT_TRANSPARENT_BLACK", 0).emit(emitter);
    BORDER_COLOR.member("INT_TRANSPARENT_BLACK", 1).emit(emitter);
    BORDER_COLOR.member("FLOAT_OPAQUE_BLACK", 2).emit(emitter);
    BORDER_COLOR.member("INT_OPAQUE_BLACK", 3).emit(emitter);
    BORDER_COLOR.member("FLOAT_OPAQUE_WHITE", 4).emit(emitter);
    BORDER_COLOR.member("INT_OPAQUE_WHITE", 5).emit(emitter);

    COMMAND_BUFFER_LEVEL.emit(emitter);
    COMMAND_BUFFER_LEVEL.member("PRIMARY", 0).emit(emitter);
    COMMAND_BUFFER_LEVEL.member("SECONDARY", 1).emit(emitter);

    COMPARE_OP.emit(emitter);
    COMPARE_OP.member("NEVER", 0).emit(emitter);
    COMPARE_OP.member("LESS", 1).emit(emitter);
    COMPARE_OP.member("EQUAL", 2).emit(emitter);
    COMPARE_OP.member("LESS_OR_EQUAL", 3).emit(emitter);
    COMPARE_OP.member("GREATER", 4).emit(emitter);
    COMPARE_OP.member("NOT_EQUAL", 5).emit(emitter);
    COMPARE_OP.member("GREATER_OR_EQUAL", 6).emit(emitter);
    COMPARE_OP.member("ALWAYS", 7).emit(emitter);

    COMPONENT_SWIZZLE.emit(emitter);
    COMPONENT_SWIZZLE.member("IDENTITY", 0).emit(emitter);
    COMPONENT_SWIZZLE.member("ZERO", 1).emit(emitter);
    COMPONENT_SWIZZLE.member("ONE", 2).emit(emitter);
    COMPONENT_SWIZZLE.member("R", 3).emit(emitter);
    COMPONENT_SWIZZLE.member("G", 4).emit(emitter);
    COMPONENT_SWIZZLE.member("B", 5).emit(emitter);
    COMPONENT_SWIZZLE.member("A", 6).emit(emitter);

    DESCRIPTOR_TYPE.emit(emitter);
    DESCRIPTOR_TYPE.member("SAMPLER", 0).emit(emitter);
    DESCRIPTOR_TYPE.member("COMBINED_IMAGE_SAMPLER", 1).emit(emitter);
    DESCRIPTOR_TYPE.member("SAMPLED_IMAGE", 2).emit(emitter);
    DESCRIPTOR_TYPE.member("STORAGE_IMAGE", 3).emit(emitter);
    DESCRIPTOR_TYPE.member("UNIFORM_TEXEL_BUFFER", 4).emit(emitter);
    DESCRIPTOR_TYPE.member("STORAGE_TEXEL_BUFFER", 5).emit(emitter);
    DESCRIPTOR_TYPE.member("UNIFORM_BUFFER", 6).emit(emitter);
    DESCRIPTOR_TYPE.member("STORAGE_BUFFER", 7).emit(emitter);
    DESCRIPTOR_TYPE.member("UNIFORM_BUFFER_DYNAMIC", 8).emit(emitter);
    DESCRIPTOR_TYPE.member("STORAGE_BUFFER_DYNAMIC", 9).emit(emitter);
    DESCRIPTOR_TYPE.member("INPUT_ATTACHMENT", 10).emit(emitter);

    DYNAMIC_STATE.emit(emitter);
    DYNAMIC_STATE.member("VIEWPORT", 0).emit(emitter);
    DYNAMIC_STATE.member("SCISSOR", 1).emit(emitter);
    DYNAMIC_STATE.member("LINE_WIDTH", 2).emit(emitter);
    DYNAMIC_STATE.member("DEPTH_BIAS", 3).emit(emitter);
    DYNAMIC_STATE.member("BLEND_CONSTANTS", 4).emit(emitter);
    DYNAMIC_STATE.member("DEPTH_BOUNDS", 5).emit(emitter);
    DYNAMIC_STATE.member("STENCIL_COMPARE_MASK", 6).emit(emitter);
    DYNAMIC_STATE.member("STENCIL_WRITE_MASK", 7).emit(emitter);
    DYNAMIC_STATE.member("STENCIL_REFERENCE", 8).emit(emitter);

    FILTER.emit(emitter);
    FILTER.member("NEAREST", 0).emit(emitter);
    FILTER.member("LINEAR", 1).emit(emitter);

    FRONT_FACE.emit(emitter);
    FRONT_FACE.member("COUNTER_CLOCKWISE", 0).emit(emitter);
    FRONT_FACE.member("CLOCKWISE", 1).emit(emitter);

    IMAGE_LAYOUT.emit(emitter);
    IMAGE_LAYOUT.member("UNDEFINED", 0).emit(emitter);
    IMAGE_LAYOUT.member("GENERAL", 1).emit(emitter);
    IMAGE_LAYOUT.member("COLOR_ATTACHMENT_OPTIMAL", 2).emit(emitter);
    IMAGE_LAYOUT.member("DEPTH_STENCIL_ATTACHMENT_OPTIMAL", 3).emit(emitter);
    IMAGE_LAYOUT.member("DEPTH_STENCIL_READ_ONLY_OPTIMAL", 4).emit(emitter);
    IMAGE_LAYOUT.member("SHADER_READ_ONLY_OPTIMAL", 5).emit(emitter);
    IMAGE_LAYOUT.member("TRANSFER_SRC_OPTIMAL", 6).emit(emitter);
    IMAGE_LAYOUT.member("TRANSFER_DST_OPTIMAL", 7).emit(emitter);
    IMAGE_LAYOUT.member("PREINITIALIZED", 8).emit(emitter);

    IMAGE_TILING.emit(emitter);
    IMAGE_TILING.member("OPTIMAL", 0).emit(emitter);
    IMAGE_TILING.member("LINEAR", 1).emit(emitter);

    IMAGE_TYPE.emit(emitter);
    IMAGE_TYPE.member("1D", 0).emit(emitter);
    IMAGE_TYPE.member("2D", 1).emit(emitter);
    IMAGE_TYPE.member("3D", 2).emit(emitter);

    IMAGE_VIEW_TYPE.emit(emitter);
    IMAGE_VIEW_TYPE.member("1D", 0).emit(emitter);
    IMAGE_VIEW_TYPE.member("2D", 1).emit(emitter);
    IMAGE_VIEW_TYPE.member("3D", 2).emit(emitter);
    IMAGE_VIEW_TYPE.member("CUBE", 3).emit(emitter);
    IMAGE_VIEW_TYPE.member("1D_ARRAY", 4).emit(emitter);
    IMAGE_VIEW_TYPE.member("2D_ARRAY", 5).emit(emitter);
    IMAGE_VIEW_TYPE.member("CUBE_ARRAY", 6).emit(emitter);

    INDEX_TYPE.emit(emitter);
    INDEX_TYPE.member("UINT16", 0).emit(emitter);
    INDEX_TYPE.member("UINT32", 1).emit(emitter);

    LOGIC_OP.emit(emitter);
    LOGIC_OP.member("CLEAR", 0).emit(emitter);
    LOGIC_OP.member("AND", 1).emit(emitter);
    LOGIC_OP.member("AND_REVERSE", 2).emit(emitter);
    LOGIC_OP.member("COPY", 3).emit(emitter);
    LOGIC_OP.member("AND_INVERTED", 4).emit(emitter);
    LOGIC_OP.member("NO_OP", 5).emit(emitter);
    LOGIC_OP.member("XOR", 6).emit(emitter);
    LOGIC_OP.member("OR", 7).emit(emitter);
    LOGIC_OP.member("NOR", 8).emit(emitter);
    LOGIC_OP.member("EQUIVALENT", 9).emit(emitter);
    LOGIC_OP.member("INVERT", 10).emit(emitter);
    LOGIC_OP.member("OR_REVERSE", 11).emit(emitter);
    LOGIC_OP.member("COPY_INVERTED", 12).emit(emitter);
    LOGIC_OP.member("OR_INVERTED", 13).emit(emitter);
    LOGIC_OP.member("NAND", 14).emit(emitter);
    LOGIC_OP.member("SET", 15).emit(emitter);

    INTERNAL_ALLOCATION_TYPE.emit(emitter);
    INTERNAL_ALLOCATION_TYPE.member("EXECUTABLE", 0).emit(emitter);

    PHYSICAL_DEVICE_TYPE.emit(emitter);
    PHYSICAL_DEVICE_TYPE.member("OTHER", 0).emit(emitter);
    PHYSICAL_DEVICE_TYPE.member("INTEGRATED_GPU", 1).emit(emitter);
    PHYSICAL_DEVICE_TYPE.member("DISCRETE_GPU", 2).emit(emitter);
    PHYSICAL_DEVICE_TYPE.member("VIRTUAL_GPU", 3).emit(emitter);
    PHYSICAL_DEVICE_TYPE.member("CPU", 4).emit(emitter);

    PIPELINE_BIND_POINT.emit(emitter);
    PIPELINE_BIND_POINT.member("GRAPHICS", 0).emit(emitter);
    PIPELINE_BIND_POINT.member("COMPUTE", 1).emit(emitter);

    POLYGON_MODE.emit(emitter);
    POLYGON_MODE.member("FILL", 0).emit(emitter);
    POLYGON_MODE.member("LINE", 1).emit(emitter);
    POLYGON_MODE.member("POINT", 2).emit(emitter);

    PRIMITIVE_TOPOLOGY.emit(emitter);
    PRIMITIVE_TOPOLOGY.member("POINT_LIST", 0).emit(emitter);
    PRIMITIVE_TOPOLOGY.member("LINE_LIST", 1).emit(emitter);
    PRIMITIVE_TOPOLOGY.member("LINE_STRIP", 2).emit(emitter);
    PRIMITIVE_TOPOLOGY.member("TRIANGLE_LIST", 3).emit(emitter);
    PRIMITIVE_TOPOLOGY.member("TRIANGLE_STRIP", 4).emit(emitter);
    PRIMITIVE_TOPOLOGY.member("TRIANGLE_FAN", 5).emit(emitter);
    PRIMITIVE_TOPOLOGY.member("LINE_LIST_WITH_ADJACENCY", 6).emit(emitter);
    PRIMITIVE_TOPOLOGY.member("LINE_STRIP_WITH_ADJACENCY", 7).emit(emitter);
    PRIMITIVE_TOPOLOGY
        .member("TRIANGLE_LIST_WITH_ADJACENCY", 8)
        .emit(emitter);
    PRIMITIVE_TOPOLOGY
        .member("TRIANGLE_STRIP_WITH_ADJACENCY", 9)
        .emit(emitter);
    PRIMITIVE_TOPOLOGY.member("PATCH_LIST", 10).emit(emitter);

    QUERY_TYPE.emit(emitter);
    QUERY_TYPE.member("OCCLUSION", 0).emit(emitter);
    QUERY_TYPE.member("PIPELINE_STATISTICS", 1).emit(emitter);
    QUERY_TYPE.member("TIMESTAMP", 2).emit(emitter);

    SAMPLER_ADDRESS_MODE.emit(emitter);
    SAMPLER_ADDRESS_MODE.member("REPEAT", 0).emit(emitter);
    SAMPLER_ADDRESS_MODE.member("MIRRORED_REPEAT", 1).emit(emitter);
    SAMPLER_ADDRESS_MODE.member("CLAMP_TO_EDGE", 2).emit(emitter);
    SAMPLER_ADDRESS_MODE.member("CLAMP_TO_BORDER", 3).emit(emitter);

    SAMPLER_MIPMAP_MODE.emit(emitter);
    SAMPLER_MIPMAP_MODE.member("NEAREST", 0).emit(emitter);
    SAMPLER_MIPMAP_MODE.member("LINEAR", 1).emit(emitter);

    SHARING_MODE.emit(emitter);
    SHARING_MODE.member("EXCLUSIVE", 0).emit(emitter);
    SHARING_MODE.member("CONCURRENT", 1).emit(emitter);

    STENCIL_OP.emit(emitter);
    STENCIL_OP.member("KEEP", 0).emit(emitter);
    STENCIL_OP.member("ZERO", 1).emit(emitter);
    STENCIL_OP.member("REPLACE", 2).emit(emitter);
    STENCIL_OP.member("INCREMENT_AND_CLAMP", 3).emit(emitter);
    STENCIL_OP.member("DECREMENT_AND_CLAMP", 4).emit(emitter);
    STENCIL_OP.member("INVERT", 5).emit(emitter);
    STENCIL_OP.member("INCREMENT_AND_WRAP", 6).emit(emitter);
    STENCIL_OP.member("DECREMENT_AND_WRAP", 7).emit(emitter);

    SUBPASS_CONTENTS.emit(emitter);
    SUBPASS_CONTENTS.member("INLINE", 0).emit(emitter);
    SUBPASS_CONTENTS.member("SECONDARY_COMMAND_BUFFERS", 1).emit(emitter);

    SYSTEM_ALLOCATION_SCOPE.emit(emitter);
    SYSTEM_ALLOCATION_SCOPE.member("COMMAND", 0).emit(emitter);
    SYSTEM_ALLOCATION_SCOPE.member("OBJECT", 1).emit(emitter);
    SYSTEM_ALLOCATION_SCOPE.member("CACHE", 2).emit(emitter);
    SYSTEM_ALLOCATION_SCOPE.member("DEVICE", 3).emit(emitter);
    SYSTEM_ALLOCATION_SCOPE.member("INSTANCE", 4).emit(emitter);

    VERTEX_INPUT_RATE.emit(emitter);
    VERTEX_INPUT_RATE.member("VERTEX", 0).emit(emitter);
    VERTEX_INPUT_RATE.member("INSTANCE", 1).emit(emitter);
}

const FLAGS: &[Bitmask] = &[
    Bitmask::new(
        "AccessFlags",
        "AccessFlagBits",
        "ACCESS",
        &[
            Bitmask::entry("INDIRECT_COMMAND_READ", 0),
            Bitmask::entry("INDEX_READ", 1),
            Bitmask::entry("VERTEX_ATTRIBUTE_READ", 2),
            Bitmask::entry("UNIFORM_READ", 3),
            Bitmask::entry("INPUT_ATTACHMENT_READ", 4),
            Bitmask::entry("SHADER_READ", 5),
            Bitmask::entry("SHADER_WRITE", 6),
            Bitmask::entry("COLOR_ATTACHMENT_READ", 7),
            Bitmask::entry("COLOR_ATTACHMENT_WRITE", 8),
            Bitmask::entry("DEPTH_STENCIL_ATTACHMENT_READ", 9),
            Bitmask::entry("DEPTH_STENCIL_ATTACHMENT_WRITE", 10),
            Bitmask::entry("TRANSFER_READ", 11),
            Bitmask::entry("TRANSFER_WRITE", 12),
            Bitmask::entry("HOST_READ", 13),
            Bitmask::entry("HOST_WRITE", 14),
            Bitmask::entry("MEMORY_READ", 15),
            Bitmask::entry("MEMORY_WRITE", 16),
            Bitmask::entry("COLOR_ATTACHMENT_READ_NONCOHERENT", 19).extension_old("EXT", "blend_operation_advanced"),
        ],
    ),
    Bitmask::new(
        "AccessFlags2",
        "AccessFlagBits2",
        "ACCESS_2",
        &[
            Bitmask::entry("INDIRECT_COMMAND_READ", 0)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("INDEX_READ", 1)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("VERTEX_ATTRIBUTE_READ", 2)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("UNIFORM_READ", 3)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("INPUT_ATTACHMENT_READ", 4)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("SHADER_READ", 5)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("SHADER_WRITE", 6)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("COLOR_ATTACHMENT_READ", 7)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("COLOR_ATTACHMENT_WRITE", 8)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("DEPTH_STENCIL_ATTACHMENT_READ", 9)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("DEPTH_STENCIL_ATTACHMENT_WRITE", 10)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("TRANSFER_READ", 11)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("TRANSFER_WRITE", 12)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("HOST_READ", 13)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("HOST_WRITE", 14)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("MEMORY_READ", 15)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("MEMORY_WRITE", 16)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("SHADER_SAMPLED_READ", 32)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("SHADER_STORAGE_READ", 33)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("SHADER_STORAGE_WRITE", 34)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
        ],
    )
    .long()
    .extension_old("KHR", "synchronization2")
    .promoted("1_3"),
    Bitmask::new(
        "AttachmentDescriptionFlags",
        "AttachmentDescriptionFlagBits",
        "ATTACHMENT_DESCRIPTION",
        &[Bitmask::entry("MAY_ALIAS", 0)],
    ),
    Bitmask::new(
        "BufferCreateFlags",
        "BufferCreateFlagBits",
        "BUFFER_CREATE",
        &[
            Bitmask::entry("SPARSE_BINDING", 0),
            Bitmask::entry("SPARSE_RESIDENCY", 1),
            Bitmask::entry("SPARSE_ALIASED", 2),
            Bitmask::entry("PROTECTED", 3).version_since("1_1"),
        ],
    ),
    Bitmask::new(
        "BufferUsageFlags",
        "BufferUsageFlagBits",
        "BUFFER_USAGE",
        &[
            Bitmask::entry("TRANSFER_SRC", 0),
            Bitmask::entry("TRANSFER_DST", 1),
            Bitmask::entry("UNIFORM_TEXEL_BUFFER", 2),
            Bitmask::entry("STORAGE_TEXEL_BUFFER", 3),
            Bitmask::entry("UNIFORM_BUFFER", 4),
            Bitmask::entry("STORAGE_BUFFER", 5),
            Bitmask::entry("INDEX_BUFFER", 6),
            Bitmask::entry("VERTEX_BUFFER", 7),
            Bitmask::entry("INDIRECT_BUFFER", 8),
        ],
    ),
    Bitmask::new(
        "BufferViewCreateFlags",
        "BufferViewCreateFlagBits",
        "BUFFER_VIEW_CREATE",
        &[],
    ),
    Bitmask::new(
        "ColorComponentFlags",
        "ColorComponentFlagBits",
        "COLOR_COMPONENT",
        &[
            Bitmask::entry("R", 0),
            Bitmask::entry("G", 1),
            Bitmask::entry("B", 2),
            Bitmask::entry("A", 3),
        ],
    ),
    Bitmask::new(
        "CommandBufferResetFlags",
        "CommandBufferResetFlagBits",
        "COMMAND_BUFFER_RESET",
        &[Bitmask::entry("RELEASE_RESOURCES", 0)],
    ),
    Bitmask::new(
        "CommandBufferUsageFlags",
        "CommandBufferUsageFlagBits",
        "COMMAND_BUFFER_USAGE",
        &[
            Bitmask::entry("ONE_TIME_SUBMIT", 0),
            Bitmask::entry("RENDER_PASS_CONTINUE", 1),
            Bitmask::entry("SIMULTANEOUS_USE", 2),
        ],
    ),
    Bitmask::new(
        "CommandPoolCreateFlags",
        "CommandPoolCreateFlagBits",
        "COMMAND_POOL_CREATE",
        &[
            Bitmask::entry("TRANSIENT", 0),
            Bitmask::entry("RESET_COMMAND_BUFFER", 1),
            Bitmask::entry("PROTECTED", 2).version_since("1_1"),
        ],
    ),
    Bitmask::new(
        "CommandPoolResetFlags",
        "CommandPoolResetFlagBits",
        "COMMAND_POOL_RESET",
        &[Bitmask::entry("RELEASE_RESOURCES", 0)],
    ),
    Bitmask::new(
        "CullModeFlags",
        "CullModeFlagBits",
        "CULL_MODE",
        &[Bitmask::entry("FRONT", 0), Bitmask::entry("BACK", 1)],
    ),
    Bitmask::new(
        "DependencyFlags",
        "DependencyFlagBits",
        "DEPENDENCY",
        &[
            Bitmask::entry("BY_REGION", 0),
            Bitmask::entry("VIEW_LOCAL", 1)
                .extension_old("KHR", "multiview")
                .promoted("1_1"),
        ],
    ),
    Bitmask::new(
        "DescriptorPoolCreateFlags",
        "DescriptorPoolCreateFlagBits",
        "DESCRIPTOR_POOL_CREATE",
        &[Bitmask::entry("FREE_DESCRIPTOR_SET", 0)],
    ),
    Bitmask::new(
        "DescriptorPoolResetFlags",
        "DescriptorPoolResetFlagBits",
        "DESCRIPTOR_POOL_RESET",
        &[],
    ),
    Bitmask::new(
        "DescriptorSetLayoutCreateFlags",
        "DescriptorSetLayoutCreateFlagBits",
        "DESCRIPTOR_SET_LAYOUT_CREATE",
        &[],
    ),
    Bitmask::new("DeviceCreateFlags", "DeviceCreateFlagBits", "DEVICE_CREATE", &[]),
    Bitmask::new(
        "DeviceGroupPresentModeFlags",
        "DeviceGroupPresentModeFlagBits",
        "DEVICE_GROUP_PRESENT_MODE",
        &[],
    )
    .extension(VK_KHR_DEVICE_GROUP)
    .promoted("1_1")
    .side_extensions(&[VK_KHR_SURFACE]),
    Bitmask::new(
        "DeviceQueueCreateFlags",
        "DeviceQueueCreateFlagBits",
        "DEVICE_QUEUE_CREATE",
        &[Bitmask::entry("PROTECTED", 0)],
    ),
    Bitmask::new(
        "DisplayModeCreateFlags",
        "DisplayModeCreateFlagBits",
        "DISPLAY_MODE_CREATE",
        &[],
    )
    .extension_old("KHR", "display"),
    Bitmask::new(
        "DisplayPlaneAlphaFlags",
        "DisplayPlaneAlphaFlagBits",
        "DISPLAY_PLANE_ALPHA",
        &[
            Bitmask::entry("OPAQUE", 0).extension_old("KHR", "display"),
            Bitmask::entry("GLOBAL", 1).extension_old("KHR", "display"),
            Bitmask::entry("PER_PIXEL", 2).extension_old("KHR", "display"),
            Bitmask::entry("PER_PIXEL_PREMULTIPLIED", 3).extension_old("KHR", "display"),
        ],
    )
    .extension_old("KHR", "display"),
    Bitmask::new(
        "DisplaySurfaceCreateFlags",
        "DisplaySurfaceCreateFlagBits",
        "DISPLAY_SURFACE_CREATE",
        &[],
    )
    .extension_old("KHR", "display"),
    Bitmask::new("EventCreateFlags", "EventCreateFlagBits", "EVENT_CREATE", &[]),
    Bitmask::new(
        "FenceCreateFlags",
        "FenceCreateFlagBits",
        "FENCE_CREATE",
        &[Bitmask::entry("SIGNALED", 0)],
    ),
    Bitmask::new(
        "FormatFeatureFlags",
        "FormatFeatureFlagBits",
        "FORMAT_FEATURE",
        &[
            Bitmask::entry("SAMPLED_IMAGE", 0),
            Bitmask::entry("STORAGE_IMAGE", 1),
            Bitmask::entry("STORAGE_IMAGE_ATOMIC", 2),
            Bitmask::entry("UNIFORM_TEXEL_BUFFER", 3),
            Bitmask::entry("STORAGE_TEXEL_BUFFER", 4),
            Bitmask::entry("STORAGE_TEXEL_BUFFER_ATOMIC", 5),
            Bitmask::entry("VERTEX_BUFFER", 6),
            Bitmask::entry("COLOR_ATTACHMENT", 7),
            Bitmask::entry("COLOR_ATTACHMENT_BLEND", 8),
            Bitmask::entry("DEPTH_STENCIL_ATTACHMENT", 9),
            Bitmask::entry("BLIT_SRC", 10),
            Bitmask::entry("BLIT_DST", 11),
            Bitmask::entry("SAMPLED_IMAGE_FILTER_LINEAR", 12),
            Bitmask::entry("TRANSFER_SRC", 14)
                .extension_old("KHR", "maintenance1")
                .promoted("1_1"),
            Bitmask::entry("TRANSFER_DST", 15)
                .extension_old("KHR", "maintenance1")
                .promoted("1_1"),
        ],
    ),
    Bitmask::new(
        "FramebufferCreateFlags",
        "FramebufferCreateFlagBits",
        "FRAMEBUFFER_CREATE",
        &[],
    ),
    Bitmask::new(
        "InstanceCreateFlags",
        "InstanceCreateFlagBits",
        "INSTANCE_CREATE",
        &[Bitmask::entry("ENUMERATE_PORTABILITY", 0)
            .extension_old("KHR", "portability_enumeration")
            .promoted("1_1")],
    ),
    Bitmask::new(
        "ImageAspectFlags",
        "ImageAspectFlagBits",
        "IMAGE_ASPECT",
        &[
            Bitmask::entry("COLOR", 0),
            Bitmask::entry("DEPTH", 1),
            Bitmask::entry("STENCIL", 2),
            Bitmask::entry("METADATA", 3),
        ],
    ),
    Bitmask::new(
        "ImageUsageFlags",
        "ImageUsageFlagBits",
        "IMAGE_USAGE",
        &[
            Bitmask::entry("TRANSFER_SRC", 0),
            Bitmask::entry("TRANSFER_DST", 1),
            Bitmask::entry("SAMPLED", 2),
            Bitmask::entry("STORAGE", 3),
            Bitmask::entry("COLOR_ATTACHMENT", 4),
            Bitmask::entry("DEPTH_STENCIL_ATTACHMENT", 5),
            Bitmask::entry("TRANSIENT_ATTACHMENT", 6),
            Bitmask::entry("INPUT_ATTACHMENT", 7),
        ],
    ),
    Bitmask::new(
        "ImageCreateFlags",
        "ImageCreateFlagBits",
        "IMAGE_CREATE",
        &[
            Bitmask::entry("SPARSE_BINDING", 0),
            Bitmask::entry("SPARSE_RESIDENCY", 1),
            Bitmask::entry("SPARSE_ALIASED", 2),
            Bitmask::entry("MUTABLE_FORMAT", 3),
            Bitmask::entry("CUBE_COMPATIBLE", 4),
            Bitmask::entry("2D_ARRAY_COMPATIBLE", 5)
                .extension_old("KHR", "maintenance1")
                .promoted("1_1"),
            Bitmask::entry("SPLIT_INSTANCE_BIND_REGIONS", 6)
                .extension(VK_KHR_DEVICE_GROUP)
                .side_extensions(&[VK_KHR_BIND_MEMORY_2])
                .promoted("1_1"),
            Bitmask::entry("BLOCK_TEXEL_VIEW_COMPATIBLE", 7)
                .extension_old("KHR", "maintenance2")
                .promoted("1_1"),
            Bitmask::entry("EXTENDED_USAGE", 8)
                .extension_old("KHR", "maintenance2")
                .promoted("1_1"),
            Bitmask::entry("PROTECTED", 11),
        ],
    ),
    Bitmask::new(
        "ImageViewCreateFlags",
        "ImageViewCreateFlagBits",
        "IMAGE_VIEW_CREATE",
        &[],
    ),
    Bitmask::new(
        "MemoryHeapFlags",
        "MemoryHeapFlagBits",
        "MEMORY_HEAP",
        &[
            Bitmask::entry("DEVICE_LOCAL", 0),
            Bitmask::entry("MULTI_INSTANCE", 1).version_since("1_1"),
        ],
    ),
    Bitmask::new(
        "MemoryPropertyFlags",
        "MemoryPropertyFlagBits",
        "MEMORY_PROPERTY",
        &[
            Bitmask::entry("DEVICE_LOCAL", 0),
            Bitmask::entry("HOST_VISIBLE", 1),
            Bitmask::entry("HOST_COHERENT", 2),
            Bitmask::entry("HOST_CACHED", 3),
            Bitmask::entry("LAZILY_ALLOCATED", 4),
            Bitmask::entry("PROTECTED", 5),
        ],
    ),
    Bitmask::new("MemoryMapFlags", "MemoryMapFlagBits", "MEMORY_MAP", &[]),
    Bitmask::new(
        "PipelineCacheCreateFlags",
        "PipelineCacheCreateFlagBits",
        "PIPELINE_CACHE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineCreateFlags",
        "PipelineCreateFlagBits",
        "PIPELINE_CREATE",
        &[
            Bitmask::entry("DISABLE_OPTIMIZATION", 0),
            Bitmask::entry("ALLOW_DERIVATIVES", 1),
            Bitmask::entry("DERIVATIVE", 2),
        ],
    ),
    Bitmask::new(
        "PipelineLayoutCreateFlags",
        "PipelineLayoutCreateFlagBits",
        "PIPELINE_LAYOUT_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineDepthStencilStateCreateFlags",
        "PipelineDepthStencilStateCreateFlagBits",
        "PIPELINE_DEPTH_STENCIL_STATE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineDynamicStateCreateFlags",
        "PipelineDynamicStateCreateFlagBits",
        "PIPELINE_DYNAMIC_STATE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineColorBlendStateCreateFlags",
        "PipelineColorBlendStateCreateFlagBits",
        "PIPELINE_COLOR_BLEND_STATE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineMultisampleStateCreateFlags",
        "PipelineMultisampleStateCreateFlagBits",
        "PIPELINE_MULTISAMPLE_STATE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineRasterizationStateCreateFlags",
        "PipelineRasterizationStateCreateFlagBits",
        "PIPELINE_RASTERIZATION_STATE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineViewportStateCreateFlags",
        "PipelineViewportStateCreateFlagBits",
        "PIPELINE_VIEWPORT_STATE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineTessellationStateCreateFlags",
        "PipelineTessellationStateCreateFlagBits",
        "PIPELINE_TESSELLATION_STATE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineInputAssemblyStateCreateFlags",
        "PipelineInputAssemblyStateCreateFlagBits",
        "PIPELINE_INPUT_ASSEMBLY_STATE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineVertexInputStateCreateFlags",
        "PipelineVertexInputStateCreateFlagBits",
        "PIPELINE_VERTEX_INPUT_STATE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineShaderStageCreateFlags",
        "PipelineShaderStageCreateFlagBits",
        "PIPELINE_SHADER_STAGE_CREATE",
        &[],
    ),
    Bitmask::new(
        "PipelineStageFlags",
        "PipelineStageFlagBits",
        "PIPELINE_STAGE",
        &[
            Bitmask::entry("TOP_OF_PIPE", 0),
            Bitmask::entry("DRAW_INDIRECT", 1),
            Bitmask::entry("VERTEX_INPUT", 2),
            Bitmask::entry("VERTEX_SHADER", 3),
            Bitmask::entry("TESSELLATION_CONTROL_SHADER", 4),
            Bitmask::entry("TESSELLATION_EVALUATION_SHADER", 5),
            Bitmask::entry("GEOMETRY_SHADER", 6),
            Bitmask::entry("FRAGMENT_SHADER", 7),
            Bitmask::entry("EARLY_FRAGMENT_TESTS", 8),
            Bitmask::entry("LATE_FRAGMENT_TESTS", 9),
            Bitmask::entry("COLOR_ATTACHMENT_OUTPUT", 10),
            Bitmask::entry("COMPUTE_SHADER", 11),
            Bitmask::entry("TRANSFER", 12),
            Bitmask::entry("BOTTOM_OF_PIPE", 13),
            Bitmask::entry("HOST", 14),
            Bitmask::entry("ALL_GRAPHICS", 15),
            Bitmask::entry("ALL_COMMANDS", 16),
        ],
    ),
    Bitmask::new(
        "PipelineStageFlags2",
        "PipelineStageFlagBits2",
        "PIPELINE_STAGE_2",
        &[
            Bitmask::entry("TOP_OF_PIPE", 0)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("DRAW_INDIRECT", 1)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("VERTEX_INPUT", 2)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("VERTEX_SHADER", 3)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("TESSELLATION_CONTROL_SHADER", 4)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("TESSELLATION_EVALUATION_SHADER", 5)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("GEOMETRY_SHADER", 6)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("FRAGMENT_SHADER", 7)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("EARLY_FRAGMENT_TESTS", 8)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("LATE_FRAGMENT_TESTS", 9)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("COLOR_ATTACHMENT_OUTPUT", 10)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("COMPUTE_SHADER", 11)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("ALL_TRANSFER", 12)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("BOTTOM_OF_PIPE", 13)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("HOST", 14)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("ALL_GRAPHICS", 15)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("ALL_COMMANDS", 16)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("COPY", 32)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("RESOLVE", 33)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("BLIT", 34)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("CLEAR", 35)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("INDEX_INPUT", 36)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("VERTEX_ATTRIBUTE_INPUT", 37)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("PRE_RASTERIZATION_SHADERS", 38)
                .extension_old("KHR", "synchronization2")
                .promoted("1_3"),
        ],
    )
    .long()
    .extension_old("KHR", "synchronization2")
    .promoted("1_3"),
    Bitmask::new(
        "QueryControlFlags",
        "QueryControlFlagBits",
        "QUERY_CONTROL",
        &[Bitmask::entry("PRECISE", 0)],
    ),
    Bitmask::new(
        "QueryPipelineStatisticFlags",
        "QueryPipelineStatisticFlagBits",
        "QUERY_PIPELINE_STATISTIC",
        &[
            Bitmask::entry("INPUT_ASSEMBLY_VERTICES", 0),
            Bitmask::entry("INPUT_ASSEMBLY_PRIMITIVES", 1),
            Bitmask::entry("VERTEX_SHADER_INVOCATIONS", 2),
            Bitmask::entry("GEOMETRY_SHADER_INVOCATIONS", 3),
            Bitmask::entry("GEOMETRY_SHADER_PRIMITIVES", 4),
            Bitmask::entry("CLIPPING_INVOCATIONS", 5),
            Bitmask::entry("CLIPPING_PRIMITIVES", 6),
            Bitmask::entry("FRAGMENT_SHADER_INVOCATIONS", 7),
            Bitmask::entry("TESSELLATION_CONTROL_SHADER_PATCHES", 8),
            Bitmask::entry("TESSELLATION_EVALUATION_SHADER_INVOCATIONS", 9),
            Bitmask::entry("COMPUTE_SHADER_INVOCATIONS", 10),
        ],
    ),
    Bitmask::new(
        "QueryPoolCreateFlags",
        "QueryPoolCreateFlagBits",
        "QUERY_POOL_CREATE",
        &[],
    ),
    Bitmask::new(
        "QueryResultFlags",
        "QueryResultFlagBits",
        "QUERY_RESULT",
        &[
            Bitmask::entry("64", 0),
            Bitmask::entry("WAIT", 1),
            Bitmask::entry("WITH_AVAILABILITY", 2),
            Bitmask::entry("PARTIAL", 3),
        ],
    ),
    Bitmask::new(
        "QueueFlags",
        "QueueFlagBits",
        "QUEUE",
        &[
            Bitmask::entry("GRAPHICS", 0),
            Bitmask::entry("COMPUTE", 1),
            Bitmask::entry("TRANSFER", 2),
            Bitmask::entry("SPARSE_BINDING", 3),
            Bitmask::entry("PROTECTED", 4),
        ],
    ),
    Bitmask::new(
        "RenderPassCreateFlags",
        "RenderPassCreateFlagBits",
        "RENDER_PASS_CREATE",
        &[],
    ),
    Bitmask::new(
        "SampleCountFlags",
        "SampleCountFlagBits",
        "SAMPLE_COUNT",
        &[
            Bitmask::entry("1", 0),
            Bitmask::entry("2", 1),
            Bitmask::entry("4", 2),
            Bitmask::entry("8", 3),
            Bitmask::entry("16", 4),
            Bitmask::entry("32", 5),
            Bitmask::entry("64", 6),
        ],
    ),
    Bitmask::new("SamplerCreateFlags", "SamplerCreateFlagBits", "SAMPLER_CREATE", &[]),
    Bitmask::new(
        "SemaphoreCreateFlags",
        "SemaphoreCreateFlagBits",
        "SEMAPHORE_CREATE",
        &[],
    ),
    Bitmask::new(
        "ShaderModuleCreateFlags",
        "ShaderModuleCreateFlagBits",
        "SHADER_MODULE_CREATE",
        &[],
    ),
    Bitmask::new(
        "ShaderStageFlags",
        "ShaderStageFlagBits",
        "SHADER_STAGE",
        &[
            Bitmask::entry("VERTEX", 0),
            Bitmask::entry("TESSELLATION_CONTROL", 1),
            Bitmask::entry("TESSELLATION_EVALUATION", 2),
            Bitmask::entry("GEOMETRY", 3),
            Bitmask::entry("FRAGMENT", 4),
            Bitmask::entry("COMPUTE", 5),
        ],
    ),
    Bitmask::new(
        "SparseMemoryBindFlags",
        "SparseMemoryBindFlagBits",
        "SPARSE_MEMORY_BIND",
        &[Bitmask::entry("METADATA", 0)],
    ),
    Bitmask::new(
        "SparseImageFormatFlags",
        "SparseImageFormatFlagBits",
        "SPARSE_IMAGE_FORMAT",
        &[
            Bitmask::entry("SINGLE_MIPTAIL", 0),
            Bitmask::entry("ALIGNED_MIP_SIZE", 1),
            Bitmask::entry("NONSTANDARD_BLOCK_SIZE", 2),
        ],
    ),
    Bitmask::new(
        "StencilFaceFlags",
        "StencilFaceFlagBits",
        "STENCIL_FACE",
        &[Bitmask::entry("FRONT", 0), Bitmask::entry("BACK", 1)],
    ),
    Bitmask::new(
        "SubgroupFeatureFlags",
        "SubgroupFeatureFlagBits",
        "SUBGROUP_FEATURE",
        &[
            Bitmask::entry("BASIC", 0),
            Bitmask::entry("VOTE", 1),
            Bitmask::entry("ARITHMETIC", 2),
            Bitmask::entry("BALLOT", 3),
            Bitmask::entry("SHUFFLE", 4),
            Bitmask::entry("SHUFFLE_RELATIVE", 5),
            Bitmask::entry("CLUSTERED", 6),
            Bitmask::entry("QUAD", 7),
        ],
    )
    .version_since("1_1"),
    Bitmask::new(
        "SubmitFlags",
        "SubmitFlagBits",
        "SUBMIT",
        &[Bitmask::entry("PROTECTED", 0)
            .extension_old("KHR", "synchronization2")
            .promoted("1_3")],
    )
    .extension_old("KHR", "synchronization2")
    .promoted("1_3"),
    Bitmask::new(
        "SubpassDescriptionFlags",
        "SubpassDescriptionFlagBits",
        "SUBPASS_DESCRIPTION",
        &[],
    ),
];

const FUNC_POINTERS: &[FuncPointer] = &[
    FuncPointer::new(
        "InternalAllocationNotification",
        &[
            ("pUserData", "*mut core::ffi::c_void"),
            ("size", "usize"),
            ("allocationType", "VkInternalAllocationType"),
            ("allocationScope", "VkSystemAllocationScope"),
        ],
    ),
    FuncPointer::new(
        "InternalFreeNotification",
        &[
            ("pUserData", "*mut core::ffi::c_void"),
            ("size", "usize"),
            ("allocationType", "VkInternalAllocationType"),
            ("allocationScope", "VkSystemAllocationScope"),
        ],
    ),
    FuncPointer::new(
        "ReallocationFunction",
        &[
            ("pUserData", "*mut core::ffi::c_void"),
            ("pOriginal", "*mut core::ffi::c_void"),
            ("size", "usize"),
            ("alignment", "usize"),
            ("allocationScope", "VkSystemAllocationScope"),
        ],
    )
    .returns("*mut core::ffi::c_void"),
    FuncPointer::new(
        "AllocationFunction",
        &[
            ("pUserData", "*mut core::ffi::c_void"),
            ("size", "usize"),
            ("alignment", "usize"),
            ("allocationScope", "VkSystemAllocationScope"),
        ],
    )
    .returns("*mut core::ffi::c_void"),
    FuncPointer::new(
        "FreeFunction",
        &[
            ("pUserData", "*mut core::ffi::c_void"),
            ("pMemory", "*mut core::ffi::c_void"),
        ],
    ),
    FuncPointer::new("VoidFunction", &[]),
];

const STRUCTS: &[Struct] = &[
    Struct::new(
        "AllocationCallbacks",
        &[
            Struct::member("pUserData", "*mut core::ffi::c_void"),
            Struct::member("pfnAllocation", "PFN_vkAllocationFunction"),
            Struct::member("pfnReallocation", "PFN_vkReallocationFunction"),
            Struct::member("pfnFree", "PFN_vkFreeFunction"),
            Struct::member("pfnInternalAllocation", "Option<PFN_vkInternalAllocationNotification>"),
            Struct::member("pfnInternalFree", "Option<PFN_vkInternalFreeNotification>"),
        ],
    ),
    Struct::new(
        "ApplicationInfo",
        &[
            Struct::member("pApplicationName", "*const core::ffi::c_char"),
            Struct::member("applicationVersion", "u32"),
            Struct::member("pEngineName", "*const core::ffi::c_char"),
            Struct::member("engineVersion", "u32"),
            Struct::member("apiVersion", "u32"),
        ],
    )
    .stype("APPLICATION_INFO", 0, StructUsage::Source),
    Struct::new(
        "AttachmentDescription",
        &[
            Struct::member("flags", "VkAttachmentDescriptionFlags"),
            Struct::member("format", "VkFormat"),
            Struct::member("samples", "VkSampleCountFlagBits"),
            Struct::member("loadOp", "VkAttachmentLoadOp"),
            Struct::member("storeOp", "VkAttachmentStoreOp"),
            Struct::member("stencilLoadOp", "VkAttachmentLoadOp"),
            Struct::member("stencilStoreOp", "VkAttachmentStoreOp"),
            Struct::member("initialLayout", "VkImageLayout"),
            Struct::member("finalLayout", "VkImageLayout"),
        ],
    ),
    Struct::new(
        "AttachmentReference",
        &[
            Struct::member("attachment", "u32"),
            Struct::member("layout", "VkImageLayout"),
        ],
    )
    .copyable(),
    Struct::new(
        "BindSparseInfo",
        &[
            Struct::member("waitSemaphoreCount", "u32"),
            Struct::member("pWaitSemaphores", "*const VkSemaphore"),
            Struct::member("bufferBindCount", "u32"),
            Struct::member("pBufferBinds", "*const VkSparseBufferMemoryBindInfo"),
            Struct::member("imageOpaqueBindCount", "u32"),
            Struct::member("pImageOpaqueBinds", "*const VkSparseImageOpaqueMemoryBindInfo"),
            Struct::member("imageBindCount", "u32"),
            Struct::member("pImageBinds", "*const VkSparseImageMemoryBindInfo"),
            Struct::member("signalSemaphoreCount", "u32"),
            Struct::member("pSignalSemaphores", "*const VkSemaphore"),
        ],
    )
    .stype("BIND_SPARSE_INFO", 7, StructUsage::Source),
    Struct::new(
        "BufferCopy",
        &[
            Struct::member("srcOffset", DEVICE_SIZE_TYPE),
            Struct::member("dstOffset", DEVICE_SIZE_TYPE),
            Struct::member("size", DEVICE_SIZE_TYPE),
        ],
    ),
    Struct::new(
        "BufferCreateInfo",
        &[
            Struct::member("flags", "VkBufferCreateFlags"),
            Struct::member("size", DEVICE_SIZE_TYPE),
            Struct::member("usage", "VkBufferUsageFlags"),
            Struct::member("sharingMode", "VkSharingMode"),
            Struct::member("queueFamilyIndexCount", "u32"),
            Struct::member("pQueueFamilyIndices", "*const u32"),
        ],
    )
    .stype("BUFFER_CREATE_INFO", 12, StructUsage::Source),
    Struct::new(
        "BufferImageCopy",
        &[
            Struct::member("bufferOffset", DEVICE_SIZE_TYPE),
            Struct::member("bufferRowLength", "u32"),
            Struct::member("bufferImageHeight", "u32"),
            Struct::member("imageSubresource", "VkImageSubresourceLayers"),
            Struct::member("imageOffset", "VkOffset3D"),
            Struct::member("imageExtent", "VkExtent3D"),
        ],
    ),
    Struct::new(
        "BufferMemoryBarrier",
        &[
            Struct::member("srcAccessMask", "VkAccessFlags"),
            Struct::member("dstAccessMask", "VkAccessFlags"),
            Struct::member("srcQueueFamilyIndex", "u32"),
            Struct::member("dstQueueFamilyIndex", "u32"),
            Struct::member("buffer", "VkBuffer"),
            Struct::member("offset", "VkDeviceSize"),
            Struct::member("size", "VkDeviceSize"),
        ],
    )
    .stype("BUFFER_MEMORY_BARRIER", 44, StructUsage::Source),
    Struct::new(
        "BufferViewCreateInfo",
        &[
            Struct::member("flags", "VkBufferViewCreateFlags"),
            Struct::member("buffer", "VkBuffer"),
            Struct::member("format", "VkFormat"),
            Struct::member("offset", DEVICE_SIZE_TYPE),
            Struct::member("range", DEVICE_SIZE_TYPE),
        ],
    )
    .stype("BUFFER_VIEW_CREATE_INFO", 13, StructUsage::Source),
    Struct::new(
        "ClearAttachment",
        &[
            Struct::member("aspectMask", "VkImageAspectFlags"),
            Struct::member("colorAttachment", "u32"),
            Struct::member("clearValue", "VkClearValue"),
        ],
    )
    .non_debuggable(),
    Struct::new(
        "ClearDepthStencilValue",
        &[
            Struct::member("depth", "core::ffi::c_float"),
            Struct::member("stencil", "u32"),
        ],
    )
    .copyable(),
    Struct::new(
        "ClearRect",
        &[
            Struct::member("rect", "VkRect2D"),
            Struct::member("baseArrayLayer", "u32"),
            Struct::member("layerCount", "u32"),
        ],
    ),
    Struct::new(
        "CommandBufferAllocateInfo",
        &[
            Struct::member("commandPool", "VkCommandPool"),
            Struct::member("level", "VkCommandBufferLevel"),
            Struct::member("commandBufferCount", "u32"),
        ],
    )
    .stype("COMMAND_BUFFER_ALLOCATE_INFO", 40, StructUsage::Source),
    Struct::new(
        "CommandBufferBeginInfo",
        &[
            Struct::member("flags", "VkCommandBufferUsageFlags"),
            Struct::member("pInheritanceInfo", "*const VkCommandBufferInheritanceInfo"),
        ],
    )
    .stype("COMMAND_BUFFER_BEGIN_INFO", 42, StructUsage::Source),
    Struct::new(
        "CommandBufferInheritanceInfo",
        &[
            Struct::member("renderPass", "Option<VkRenderPass>"),
            Struct::member("subpass", "u32"),
            Struct::member("framebuffer", "Option<VkFramebuffer>"),
            Struct::member("occlusionQueryEnable", "VkBool32"),
            Struct::member("queryFlags", "VkQueryControlFlags"),
            Struct::member("pipelineStatistics", "VkQueryPipelineStatisticFlags"),
        ],
    )
    .stype("COMMAND_BUFFER_INHERITANCE_INFO", 41, StructUsage::Source),
    Struct::new(
        "CommandPoolCreateInfo",
        &[
            Struct::member("flags", "VkCommandPoolCreateFlags"),
            Struct::member("queueFamilyIndex", "u32"),
        ],
    )
    .stype("COMMAND_POOL_CREATE_INFO", 39, StructUsage::Source),
    Struct::new(
        "ComponentMapping",
        &[
            Struct::member("r", "VkComponentSwizzle"),
            Struct::member("g", "VkComponentSwizzle"),
            Struct::member("b", "VkComponentSwizzle"),
            Struct::member("a", "VkComponentSwizzle"),
        ],
    )
    .copyable(),
    Struct::new(
        "ComputePipelineCreateInfo",
        &[
            Struct::member("flags", "VkPipelineCreateFlags"),
            Struct::member("stage", "VkPipelineShaderStageCreateInfo"),
            Struct::member("layout", "Option<VkPipelineLayout>"),
            Struct::member("basePipelineHandle", "Option<VkPipeline>"),
            Struct::member("basePipelineIndex", "i32"),
        ],
    )
    .stype("COMPUTE_PIPELINE_CREATE_INFO", 29, StructUsage::Source),
    Struct::new(
        "CopyDescriptorSet",
        &[
            Struct::member("srcSet", "VkDescriptorSet"),
            Struct::member("srcBinding", "u32"),
            Struct::member("srcArrayElement", "u32"),
            Struct::member("dstSet", "VkDescriptorSet"),
            Struct::member("dstBinding", "u32"),
            Struct::member("dstArrayElement", "u32"),
            Struct::member("descriptorCount", "u32"),
        ],
    )
    .stype("COPY_DESCRIPTOR_SET", 36, StructUsage::Source),
    Struct::new(
        "DescriptorBufferInfo",
        &[
            Struct::member("buffer", "VkBuffer"),
            Struct::member("offset", DEVICE_SIZE_TYPE),
            Struct::member("range", DEVICE_SIZE_TYPE),
        ],
    ),
    Struct::new(
        "DescriptorImageInfo",
        &[
            Struct::member("sampler", "Option<VkSampler>"),
            Struct::member("imageView", "VkImageView"),
            Struct::member("imageLayout", "VkImageLayout"),
        ],
    ),
    Struct::new(
        "DescriptorPoolCreateInfo",
        &[
            Struct::member("flags", "VkDescriptorPoolCreateFlags"),
            Struct::member("maxSets", "u32"),
            Struct::member("poolSizeCount", "u32"),
            Struct::member("pPoolSizes", "*const VkDescriptorPoolSize"),
        ],
    )
    .stype("DESCRIPTOR_POOL_CREATE_INFO", 33, StructUsage::Source),
    Struct::new(
        "DescriptorPoolSize",
        &[
            Struct::member("r#type", "VkDescriptorType"),
            Struct::member("descriptorCount", "u32"),
        ],
    ),
    Struct::new(
        "DescriptorSetAllocateInfo",
        &[
            Struct::member("descriptorPool", "VkDescriptorPool"),
            Struct::member("descriptorSetCount", "u32"),
            Struct::member("pSetLayouts", "*const VkDescriptorSetLayout"),
        ],
    )
    .stype("DESCRIPTOR_SET_ALLOCATE_INFO", 34, StructUsage::Source),
    Struct::new(
        "DescriptorSetLayoutBinding",
        &[
            Struct::member("binding", "u32"),
            Struct::member("descriptorType", "VkDescriptorType"),
            Struct::member("descriptorCount", "u32"),
            Struct::member("stageFlags", "VkShaderStageFlags"),
            Struct::member("pImmutableSamplers", "*const VkSampler"),
        ],
    ),
    Struct::new(
        "DescriptorSetLayoutCreateInfo",
        &[
            Struct::member("flags", "VkDescriptorSetLayoutCreateFlags"),
            Struct::member("bindingCount", "u32"),
            Struct::member("pBindings", "*const VkDescriptorSetLayoutBinding"),
        ],
    )
    .stype("DESCRIPTOR_SET_LAYOUT_CREATE_INFO", 32, StructUsage::Source),
    Struct::typed(
        "DescriptorSetLayoutSupport",
        "DESCRIPTOR_SET_LAYOUT_SUPPORT",
        vk_ext_enum(169, 1) as _,
        StructUsage::Sink,
        &[Struct::member("supported", "VkBool32")],
    )
    .extensions_old(&[("KHR", "maintenance3")])
    .promoted("1_1"),
    Struct::new(
        "DeviceCreateInfo",
        &[
            Struct::member("flags", "VkDeviceCreateFlags"),
            Struct::member("queueCreateInfoCount", "u32"),
            Struct::member("pQueueCreateInfos", "*const VkDeviceQueueCreateInfo"),
            Struct::member("enabledLayerCount", "u32"),
            Struct::member("ppEnabledLayerNames", "*const *const core::ffi::c_char"),
            Struct::member("enabledExtensionCount", "u32"),
            Struct::member("ppEnabledExtensionNames", "*const *const core::ffi::c_char"),
            Struct::member("pEnabledFeatures", "*const VkPhysicalDeviceFeatures"),
        ],
    )
    .stype("DEVICE_CREATE_INFO", 3, StructUsage::Source),
    Struct::typed(
        "DeviceGroupCreateInfo",
        "DEVICE_GROUP_CREATE_INFO",
        vk_ext_enum(71, 1) as _,
        StructUsage::Source,
        &[
            Struct::member("physicalDeviceCount", "u32"),
            Struct::member("pPhysicalDevices", "*const VkPhysicalDevice"),
        ],
    )
    .extensions_old(&[("KHR", "device_group_creation")])
    .promoted("1_1"),
    Struct::new(
        "DeviceQueueCreateInfo",
        &[
            Struct::member("flags", "VkDeviceQueueCreateFlags"),
            Struct::member("queueFamilyIndex", "u32"),
            Struct::member("queueCount", "u32"),
            Struct::member("pQueuePriorities", "*const core::ffi::c_float"),
        ],
    )
    .stype("DEVICE_QUEUE_CREATE_INFO", 2, StructUsage::Source),
    Struct::new(
        "DispatchIndirectCommand",
        &[
            Struct::member("x", "u32"),
            Struct::member("y", "u32"),
            Struct::member("z", "u32"),
        ],
    ),
    Struct::new(
        "DisplayModeCreateInfo",
        &[
            Struct::member("flags", "VkDisplayModeCreateFlagsKHR"),
            Struct::member("parameters", "VkDisplayModeParametersKHR"),
        ],
    )
    .stype("DISPLAY_MODE_CREATE_INFO", vk_ext_enum(3, 0) as _, StructUsage::Source)
    .extensions_old(&[("KHR", "display")]),
    Struct::new(
        "DisplayModeParameters",
        &[
            Struct::member("visibleRegion", "VkExtent2D"),
            Struct::member("refreshRate", "u32"),
        ],
    )
    .extensions_old(&[("KHR", "display")]),
    Struct::new(
        "DisplayModeProperties",
        &[
            Struct::member("displayMode", "VkDisplayModeKHR"),
            Struct::member("parameters", "VkDisplayModeParametersKHR"),
        ],
    )
    .extensions_old(&[("KHR", "display")]),
    Struct::new(
        "DisplayPlaneCapabilities",
        &[
            Struct::member("supportedAlpha", "VkDisplayPlaneAlphaFlagsKHR"),
            Struct::member("minSrcPosition", "VkOffset2D"),
            Struct::member("maxSrcPosition", "VkOffset2D"),
            Struct::member("minSrcExtent", "VkExtent2D"),
            Struct::member("maxSrcExtent", "VkExtent2D"),
            Struct::member("minDstPosition", "VkOffset2D"),
            Struct::member("maxDstPosition", "VkOffset2D"),
            Struct::member("minDstExtent", "VkExtent2D"),
            Struct::member("maxDstExtent", "VkExtent2D"),
        ],
    )
    .extensions_old(&[("KHR", "display")]),
    Struct::new(
        "DisplayPlaneProperties",
        &[
            Struct::member("currentDisplay", "VkDisplayKHR"),
            Struct::member("currentStackIndex", "u32"),
        ],
    )
    .extensions_old(&[("KHR", "display")]),
    Struct::new(
        "DisplayProperties",
        &[
            Struct::member("display", "VkDisplayKHR"),
            Struct::member("displayName", "*const core::ffi::c_char"),
            Struct::member("physicalDimensions", "VkExtent2D"),
            Struct::member("physicalResolution", "VkExtent2D"),
            Struct::member("supportedTransforms", "VkSurfaceTransformFlagsKHR"),
            Struct::member("planeReorderPossible", "VkBool32"),
            Struct::member("persistentContent", "VkBool32"),
        ],
    )
    .extensions_old(&[("KHR", "display")]),
    Struct::new(
        "DisplaySurfaceCreateInfo",
        &[
            Struct::member("flags", "VkDisplaySurfaceCreateFlagsKHR"),
            Struct::member("displayMode", "VkDisplayModeKHR"),
            Struct::member("planeIndex", "u32"),
            Struct::member("planeStackIndex", "u32"),
            Struct::member("transform", "VkSurfaceTransformFlagBitsKHR"),
            Struct::member("globalAlpha", "core::ffi::c_float"),
            Struct::member("alphaMode", "VkDisplayPlaneAlphaFlagBitsKHR"),
            Struct::member("imageExtent", "VkExtent2D"),
        ],
    )
    .stype(
        "DISPLAY_SURFACE_CREATE_INFO",
        vk_ext_enum(3, 1) as _,
        StructUsage::Source,
    )
    .extensions_old(&[("KHR", "display")]),
    Struct::new(
        "DrawIndexedIndirectCommand",
        &[
            Struct::member("indexCount", "u32"),
            Struct::member("instanceCount", "u32"),
            Struct::member("firstIndex", "u32"),
            Struct::member("vertexOffset", "i32"),
            Struct::member("firstInstance", "u32"),
        ],
    ),
    Struct::new(
        "DrawIndirectCommand",
        &[
            Struct::member("vertexCount", "u32"),
            Struct::member("instanceCount", "u32"),
            Struct::member("firstVertex", "u32"),
            Struct::member("firstInstance", "u32"),
        ],
    ),
    Struct::new("EventCreateInfo", &[Struct::member("flags", "VkEventCreateFlags")]).stype(
        "EVENT_CREATE_INFO",
        10,
        StructUsage::Source,
    ),
    Struct::new(
        "ExtensionProperties",
        &[
            Struct::member("extensionName", "crate::FixedCStrBuffer<VK_MAX_EXTENSION_NAME_SIZE>"),
            Struct::member("specVersion", "u32"),
        ],
    ),
    Struct::new(
        "Extent2D",
        &[Struct::member("width", "u32"), Struct::member("height", "u32")],
    )
    .copyable()
    .equatable()
    .hashable(),
    Struct::new(
        "Extent3D",
        &[
            Struct::member("width", "u32"),
            Struct::member("height", "u32"),
            Struct::member("depth", "u32"),
        ],
    )
    .copyable()
    .equatable()
    .hashable(),
    Struct::new("FenceCreateInfo", &[Struct::member("flags", "VkFenceCreateFlags")]).stype(
        "FENCE_CREATE_INFO",
        8,
        StructUsage::Source,
    ),
    Struct::new(
        "FormatProperties",
        &[
            Struct::member("linearTilingFeatures", "VkFormatFeatureFlags"),
            Struct::member("optimalTilingFeatures", "VkFormatFeatureFlags"),
            Struct::member("bufferFeatures", "VkFormatFeatureFlags"),
        ],
    ),
    Struct::new(
        "FramebufferCreateInfo",
        &[
            Struct::member("flags", "VkFramebufferCreateFlags"),
            Struct::member("renderPass", "VkRenderPass"),
            Struct::member("attachmentCount", "u32"),
            Struct::member("pAttachments", "*const VkImageView"),
            Struct::member("width", "u32"),
            Struct::member("height", "u32"),
            Struct::member("layers", "u32"),
        ],
    )
    .stype("FRAMEBUFFER_CREATE_INFO", 37, StructUsage::Source),
    Struct::new(
        "GraphicsPipelineCreateInfo",
        &[
            Struct::member("flags", "VkPipelineCreateFlags"),
            Struct::member("stageCount", "u32"),
            Struct::member("pStages", "*const VkPipelineShaderStageCreateInfo"),
            Struct::member("pVertexInputState", "*const VkPipelineVertexInputStateCreateInfo"),
            Struct::member("pInputAssemblyState", "*const VkPipelineInputAssemblyStateCreateInfo"),
            Struct::member("pTessellationState", "*const VkPipelineTessellationStateCreateInfo"),
            Struct::member("pViewportState", "*const VkPipelineViewportStateCreateInfo"),
            Struct::member("pRasterizationState", "*const VkPipelineRasterizationStateCreateInfo"),
            Struct::member("pMultisampleState", "*const VkPipelineMultisampleStateCreateInfo"),
            Struct::member("pDepthStencilState", "*const VkPipelineDepthStencilStateCreateInfo"),
            Struct::member("pColorBlendState", "*const VkPipelineColorBlendStateCreateInfo"),
            Struct::member("pDynamicState", "*const VkPipelineDynamicStateCreateInfo"),
            Struct::member("layout", "Option<VkPipelineLayout>"),
            Struct::member("renderPass", "Option<VkRenderPass>"),
            Struct::member("subpass", "u32"),
            Struct::member("basePipelineHandle", "Option<VkPipeline>"),
            Struct::member("basePipelineIndex", "i32"),
        ],
    )
    .stype("GRAPHICS_PIPELINE_CREATE_INFO", 28, StructUsage::Source),
    Struct::new(
        "ImageBlit",
        &[
            Struct::member("srcSubresource", "VkImageSubresourceLayers"),
            Struct::member("srcOffsets", "[VkOffset3D; 2]"),
            Struct::member("dstSubresource", "VkImageSubresourceLayers"),
            Struct::member("dstOffsets", "[VkOffset3D; 2]"),
        ],
    ),
    Struct::new(
        "ImageCopy",
        &[
            Struct::member("srcSubresource", "VkImageSubresourceLayers"),
            Struct::member("srcOffset", "VkOffset3D"),
            Struct::member("dstSubresource", "VkImageSubresourceLayers"),
            Struct::member("dstOffset", "VkOffset3D"),
            Struct::member("extent", "VkExtent3D"),
        ],
    ),
    Struct::new(
        "ImageCreateInfo",
        &[
            Struct::member("flags", "VkImageCreateFlags"),
            Struct::member("imageType", "VkImageType"),
            Struct::member("format", "VkFormat"),
            Struct::member("extent", "VkExtent3D"),
            Struct::member("mipLevels", "u32"),
            Struct::member("arrayLayers", "u32"),
            Struct::member("samples", "VkSampleCountFlagBits"),
            Struct::member("tiling", "VkImageTiling"),
            Struct::member("usage", "VkImageUsageFlags"),
            Struct::member("sharingMode", "VkSharingMode"),
            Struct::member("queueFamilyIndexCount", "u32"),
            Struct::member("pQueueFamilyIndices", "*const u32"),
            Struct::member("initialLayout", "VkImageLayout"),
        ],
    )
    .stype("IMAGE_CREATE_INFO", 14, StructUsage::Source),
    Struct::new(
        "ImageFormatProperties",
        &[
            Struct::member("maxExtent", "VkExtent3D"),
            Struct::member("maxMipLevels", "u32"),
            Struct::member("maxArrayLayers", "u32"),
            Struct::member("sampleCounts", "VkSampleCountFlags"),
            Struct::member("maxResourceSize", "VkDeviceSize"),
        ],
    ),
    Struct::typed(
        "ImageMemoryBarrier",
        "IMAGE_MEMORY_BARRIER",
        45,
        StructUsage::Source,
        &[
            Struct::member("srcAccessMask", "VkAccessFlags"),
            Struct::member("dstAccessMask", "VkAccessFlags"),
            Struct::member("oldLayout", "VkImageLayout"),
            Struct::member("newLayout", "VkImageLayout"),
            Struct::member("srcQueueFamilyIndex", "u32"),
            Struct::member("dstQueueFamilyIndex", "u32"),
            Struct::member("image", "VkImage"),
            Struct::member("subresourceRange", "VkImageSubresourceRange"),
        ],
    ),
    Struct::new(
        "ImageResolve",
        &[
            Struct::member("srcSubresource", "VkImageSubresourceLayers"),
            Struct::member("srcOffset", "VkOffset3D"),
            Struct::member("dstSubresource", "VkImageSubresourceLayers"),
            Struct::member("dstOffset", "VkOffset3D"),
            Struct::member("extent", "VkExtent3D"),
        ],
    ),
    Struct::new(
        "ImageSubresource",
        &[
            Struct::member("aspectMask", "VkImageAspectFlags"),
            Struct::member("mipLevel", "u32"),
            Struct::member("arrayLayer", "u32"),
        ],
    ),
    Struct::new(
        "ImageSubresourceLayers",
        &[
            Struct::member("aspectMask", "VkImageAspectFlags"),
            Struct::member("mipLevel", "u32"),
            Struct::member("baseArrayLayer", "u32"),
            Struct::member("layerCount", "u32"),
        ],
    ),
    Struct::new(
        "ImageSubresourceRange",
        &[
            Struct::member("aspectMask", "VkImageAspectFlags"),
            Struct::member("baseMipLevel", "u32"),
            Struct::member("levelCount", "u32"),
            Struct::member("baseArrayLayer", "u32"),
            Struct::member("layerCount", "u32"),
        ],
    ),
    Struct::new(
        "ImageViewCreateInfo",
        &[
            Struct::member("flags", "VkImageViewCreateFlags"),
            Struct::member("image", "VkImage"),
            Struct::member("viewType", "VkImageViewType"),
            Struct::member("format", "VkFormat"),
            Struct::member("components", "VkComponentMapping"),
            Struct::member("subresourceRange", "VkImageSubresourceRange"),
        ],
    )
    .stype("IMAGE_VIEW_CREATE_INFO", 15, StructUsage::Source),
    Struct::new(
        "InstanceCreateInfo",
        &[
            Struct::member("flags", "VkInstanceCreateFlags"),
            Struct::member("pApplicationInfo", "*const VkApplicationInfo"),
            Struct::member("enabledLayerCount", "u32"),
            Struct::member("ppEnabledLayerNames", "*const *const core::ffi::c_char"),
            Struct::member("enabledExtensionCount", "u32"),
            Struct::member("ppEnabledExtensionNames", "*const *const core::ffi::c_char"),
        ],
    )
    .stype("INSTANCE_CREATE_INFO", 1, StructUsage::Source),
    Struct::new(
        "LayerProperties",
        &[
            Struct::member("layerName", "crate::FixedCStrBuffer<VK_MAX_EXTENSION_NAME_SIZE>"),
            Struct::member("specVersion", "u32"),
            Struct::member("implementationVersion", "u32"),
            Struct::member("description", "crate::FixedCStrBuffer<VK_MAX_DESCRIPTION_SIZE>"),
        ],
    ),
    Struct::new(
        "MappedMemoryRange",
        &[
            Struct::member("memory", "VkDeviceMemory"),
            Struct::member("offset", DEVICE_SIZE_TYPE),
            Struct::member("size", DEVICE_SIZE_TYPE),
        ],
    )
    .stype("MAPPED_MEMORY_RANGE", 6, StructUsage::Source),
    Struct::new(
        "MemoryAllocateInfo",
        &[
            Struct::member("allocationSize", DEVICE_SIZE_TYPE),
            Struct::member("memoryTypeIndex", "u32"),
        ],
    )
    .stype("MEMORY_ALLOCATE_INFO", 5, StructUsage::Source),
    Struct::new(
        "MemoryBarrier",
        &[
            Struct::member("srcAccessMask", "VkAccessFlags"),
            Struct::member("dstAccessMask", "VkAccessFlags"),
        ],
    )
    .stype("MEMORY_BARRIER", 46, StructUsage::Source),
    Struct::new(
        "MemoryRequirements",
        &[
            Struct::member("size", DEVICE_SIZE_TYPE),
            Struct::member("alignment", DEVICE_SIZE_TYPE),
            Struct::member("memoryTypeBits", "u32"),
        ],
    ),
    Struct::new(
        "MemoryType",
        &[
            Struct::member("propertyFlags", "VkMemoryPropertyFlags"),
            Struct::member("heapIndex", "u32"),
        ],
    ),
    Struct::new(
        "MemoryHeap",
        &[
            Struct::member("size", "VkDeviceSize"),
            Struct::member("flags", "VkMemoryHeapFlags"),
        ],
    ),
    Struct::new("Offset2D", &[Struct::member("x", "i32"), Struct::member("y", "i32")])
        .copyable()
        .equatable()
        .hashable(),
    Struct::new(
        "Offset3D",
        &[
            Struct::member("x", "i32"),
            Struct::member("y", "i32"),
            Struct::member("z", "i32"),
        ],
    )
    .copyable()
    .equatable()
    .hashable(),
    Struct::typed(
        "PhysicalDevice16BitStorageFeatures",
        "PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES",
        vk_ext_enum(84, 0) as _,
        StructUsage::Both,
        &[
            Struct::member("storageBuffer16BitAccess", "VkBool32"),
            Struct::member("uniformAndStorageBuffer16BitAccess", "VkBool32"),
            Struct::member("storagePushConstant16", "VkBool32"),
            Struct::member("storageInputOutput16", "VkBool32"),
        ],
    )
    .extensions_old(&[("KHR", "16bit_storage")])
    .promoted("1_1"),
    Struct::new(
        "PhysicalDeviceFeatures",
        &[
            Struct::member("robustBufferAccess", "VkBool32"),
            Struct::member("fullDrawIndexUint32", "VkBool32"),
            Struct::member("imageCubeArray", "VkBool32"),
            Struct::member("independentBlend", "VkBool32"),
            Struct::member("geometryShader", "VkBool32"),
            Struct::member("tessellationShader", "VkBool32"),
            Struct::member("sampleRateShading", "VkBool32"),
            Struct::member("dualSrcBlend", "VkBool32"),
            Struct::member("logicOp", "VkBool32"),
            Struct::member("multiDrawIndirect", "VkBool32"),
            Struct::member("drawIndirectFirstInstance", "VkBool32"),
            Struct::member("depthClamp", "VkBool32"),
            Struct::member("depthBiasClamp", "VkBool32"),
            Struct::member("fillModeNonSolid", "VkBool32"),
            Struct::member("depthBounds", "VkBool32"),
            Struct::member("wideLines", "VkBool32"),
            Struct::member("largePoints", "VkBool32"),
            Struct::member("alphaToOne", "VkBool32"),
            Struct::member("multiViewport", "VkBool32"),
            Struct::member("samplerAnisotropy", "VkBool32"),
            Struct::member("textureCompressionETC2", "VkBool32"),
            Struct::member("textureCompressionASTC_LDR", "VkBool32"),
            Struct::member("textureCompressionBC", "VkBool32"),
            Struct::member("occlusionQueryPrecise", "VkBool32"),
            Struct::member("pipelineStatisticsQuery", "VkBool32"),
            Struct::member("vertexPipelineStoresAndAtomics", "VkBool32"),
            Struct::member("fragmentStoresAndAtomics", "VkBool32"),
            Struct::member("shaderTessellationAndGeometryPointSize", "VkBool32"),
            Struct::member("shaderImageGatherExtended", "VkBool32"),
            Struct::member("shaderStorageImageExtendedFormats", "VkBool32"),
            Struct::member("shaderStorageImageMultisample", "VkBool32"),
            Struct::member("shaderStorageImageReadWithoutFormat", "VkBool32"),
            Struct::member("shaderStorageImageWriteWithoutFormat", "VkBool32"),
            Struct::member("shaderUniformBufferArrayDynamicIndexing", "VkBool32"),
            Struct::member("shaderSampledImageArrayDynamicIndexing", "VkBool32"),
            Struct::member("shaderStorageBufferArrayDynamicIndexing", "VkBool32"),
            Struct::member("shaderStorageImageArrayDynamicIndexing", "VkBool32"),
            Struct::member("shaderClipDistance", "VkBool32"),
            Struct::member("shaderCullDistance", "VkBool32"),
            Struct::member("shaderFloat64", "VkBool32"),
            Struct::member("shaderInt64", "VkBool32"),
            Struct::member("shaderInt16", "VkBool32"),
            Struct::member("shaderResourceResidency", "VkBool32"),
            Struct::member("shaderResourceMinLod", "VkBool32"),
            Struct::member("sparseBinding", "VkBool32"),
            Struct::member("sparseResidencyBuffer", "VkBool32"),
            Struct::member("sparseResidencyImage2D", "VkBool32"),
            Struct::member("sparseResidencyImage3D", "VkBool32"),
            Struct::member("sparseResidency2Samples", "VkBool32"),
            Struct::member("sparseResidency4Samples", "VkBool32"),
            Struct::member("sparseResidency8Samples", "VkBool32"),
            Struct::member("sparseResidency16Samples", "VkBool32"),
            Struct::member("sparseResidencyAliased", "VkBool32"),
            Struct::member("variableMultisampleRate", "VkBool32"),
            Struct::member("inheritedQueries", "VkBool32"),
        ],
    )
    .default_zero(),
    Struct::typed(
        "PhysicalDeviceGroupProperties",
        "PHYSICAL_DEVICE_GROUP_PROPERTIES",
        vk_ext_enum(71, 0) as _,
        StructUsage::Sink,
        &[
            Struct::member("physicalDeviceCount", "u32"),
            Struct::member("physicalDevices", "[VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE_KHR]"),
            Struct::member("subsetAllocation", "VkBool32"),
        ],
    )
    .extensions_old(&[("KHR", "device_group_creation")])
    .promoted("1_1"),
    Struct::new(
        "PhysicalDeviceLimits",
        &[
            Struct::member("maxImageDimension1D", "u32"),
            Struct::member("maxImageDimension2D", "u32"),
            Struct::member("maxImageDimension3D", "u32"),
            Struct::member("maxImageDimensionCube", "u32"),
            Struct::member("maxImageArrayLayers", "u32"),
            Struct::member("maxTexelBufferElements", "u32"),
            Struct::member("maxUniformBufferRange", "u32"),
            Struct::member("maxStorageBufferRange", "u32"),
            Struct::member("maxPushConstantsSize", "u32"),
            Struct::member("maxMemoryAllocationCount", "u32"),
            Struct::member("maxSamplerAllocationCount", "u32"),
            Struct::member("bufferImageGranularity", DEVICE_SIZE_TYPE),
            Struct::member("sparseAddressSpaceSize", DEVICE_SIZE_TYPE),
            Struct::member("maxBoundDescriptorSets", "u32"),
            Struct::member("maxPerStageDescriptorSamplers", "u32"),
            Struct::member("maxPerStageDescriptorUniformBuffers", "u32"),
            Struct::member("maxPerStageDescriptorStorageBuffers", "u32"),
            Struct::member("maxPerStageDescriptorSampledImages", "u32"),
            Struct::member("maxPerStageDescriptorStorageImages", "u32"),
            Struct::member("maxPerStageDescriptorInputAttachments", "u32"),
            Struct::member("maxPerStageResources", "u32"),
            Struct::member("maxDescriptorSetSamplers", "u32"),
            Struct::member("maxDescriptorSetUniformBuffers", "u32"),
            Struct::member("maxDescriptorSetUniformBuffersDynamic", "u32"),
            Struct::member("maxDescriptorSetStorageBuffers", "u32"),
            Struct::member("maxDescriptorSetStorageBuffersDynamic", "u32"),
            Struct::member("maxDescriptorSetSampledImages", "u32"),
            Struct::member("maxDescriptorSetStorageImages", "u32"),
            Struct::member("maxDescriptorSetInputAttachments", "u32"),
            Struct::member("maxVertexInputAttributes", "u32"),
            Struct::member("maxVertexInputBindings", "u32"),
            Struct::member("maxVertexInputAttributeOffset", "u32"),
            Struct::member("maxVertexInputBindingStride", "u32"),
            Struct::member("maxVertexOutputComponents", "u32"),
            Struct::member("maxTessellationGenerationLevel", "u32"),
            Struct::member("maxTessellationPatchSize", "u32"),
            Struct::member("maxTessellationControlPerVertexInputComponents", "u32"),
            Struct::member("maxTessellationControlPerVertexOutputComponents", "u32"),
            Struct::member("maxTessellationControlPerPatchOutputComponents", "u32"),
            Struct::member("maxTessellationControlTotalOutputComponents", "u32"),
            Struct::member("maxTessellationEvaluationInputComponents", "u32"),
            Struct::member("maxTessellationEvaluationOutputComponents", "u32"),
            Struct::member("maxGeometryShaderInvocations", "u32"),
            Struct::member("maxGeometryInputComponents", "u32"),
            Struct::member("maxGeometryOutputComponents", "u32"),
            Struct::member("maxGeometryOutputVertices", "u32"),
            Struct::member("maxGeometryTotalOutputComponents", "u32"),
            Struct::member("maxFragmentInputComponents", "u32"),
            Struct::member("maxFragmentOutputAttachments", "u32"),
            Struct::member("maxFragmentDualSrcAttachments", "u32"),
            Struct::member("maxFragmentCombinedOutputResources", "u32"),
            Struct::member("maxComputeSharedMemorySize", "u32"),
            Struct::member("maxComputeWorkGroupCount", "[u32; 3]"),
            Struct::member("maxComputeWorkGroupInvocations", "u32"),
            Struct::member("maxComputeWorkGroupSize", "[u32; 3]"),
            Struct::member("subPixelPrecisionBits", "u32"),
            Struct::member("subTexelPrecisionBits", "u32"),
            Struct::member("mipmapPrecisionBits", "u32"),
            Struct::member("maxDrawIndexedIndexValue", "u32"),
            Struct::member("maxDrawIndirectCount", "u32"),
            Struct::member("maxSamplerLodBias", "core::ffi::c_float"),
            Struct::member("maxSamplerAnisotropy", "core::ffi::c_float"),
            Struct::member("maxViewports", "u32"),
            Struct::member("maxViewportDimensions", "[u32; 2]"),
            Struct::member("viewportBoundsRange", "[core::ffi::c_float; 2]"),
            Struct::member("viewportSubPixelBits", "u32"),
            Struct::member("minMemoryMapAlignment", "usize"),
            Struct::member("minTexelBufferOffsetAlignment", DEVICE_SIZE_TYPE),
            Struct::member("minUniformBufferOffsetAlignment", DEVICE_SIZE_TYPE),
            Struct::member("minStorageBufferOffsetAlignment", DEVICE_SIZE_TYPE),
            Struct::member("minTexelOffset", "i32"),
            Struct::member("maxTexelOffset", "u32"),
            Struct::member("minTexelGatherOffset", "i32"),
            Struct::member("maxTexelGatherOffset", "u32"),
            Struct::member("minInterpolationOffset", "core::ffi::c_float"),
            Struct::member("maxInterpolationOffset", "core::ffi::c_float"),
            Struct::member("subPixelInterpolationOffsetBits", "u32"),
            Struct::member("maxFramebufferWidth", "u32"),
            Struct::member("maxFramebufferHeight", "u32"),
            Struct::member("maxFramebufferLayers", "u32"),
            Struct::member("framebufferColorSampleCounts", "VkSampleCountFlags"),
            Struct::member("framebufferDepthSampleCounts", "VkSampleCountFlags"),
            Struct::member("framebufferStencilSampleCounts", "VkSampleCountFlags"),
            Struct::member("framebufferNoAttachmentsSampleCounts", "VkSampleCountFlags"),
            Struct::member("maxColorAttachments", "u32"),
            Struct::member("sampledImageColorSampleCounts", "VkSampleCountFlags"),
            Struct::member("sampledImageIntegerSampleCounts", "VkSampleCountFlags"),
            Struct::member("sampledImageDepthSampleCounts", "VkSampleCountFlags"),
            Struct::member("sampledImageStencilSampleCounts", "VkSampleCountFlags"),
            Struct::member("storageImageSampleCounts", "VkSampleCountFlags"),
            Struct::member("maxSampleMaskWords", "u32"),
            Struct::member("timestampComputeAndGraphics", "VkBool32"),
            Struct::member("timestampPeriod", "core::ffi::c_float"),
            Struct::member("maxClipDistances", "u32"),
            Struct::member("maxCullDistances", "u32"),
            Struct::member("maxCombinedClipAndCullDistances", "u32"),
            Struct::member("discreteQueuePriorities", "u32"),
            Struct::member("pointSizeRange", "[core::ffi::c_float; 2]"),
            Struct::member("lineWidthRange", "[core::ffi::c_float; 2]"),
            Struct::member("pointSizeGranularity", "core::ffi::c_float"),
            Struct::member("lineWidthGranularity", "core::ffi::c_float"),
            Struct::member("strictLines", "VkBool32"),
            Struct::member("standardSampleLocations", "VkBool32"),
            Struct::member("optimalBufferCopyOffsetAlignment", DEVICE_SIZE_TYPE),
            Struct::member("optimalBufferCopyRowPitchAlignment", DEVICE_SIZE_TYPE),
            Struct::member("nonCoherentAtomSize", DEVICE_SIZE_TYPE),
        ],
    ),
    Struct::typed(
        "PhysicalDeviceMaintenance3Properties",
        "PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES",
        vk_ext_enum(169, 0) as _,
        StructUsage::Sink,
        &[
            Struct::member("maxPerSetDescriptors", "u32"),
            Struct::member("maxMemoryAllocationSize", DEVICE_SIZE_TYPE),
        ],
    )
    .extensions_old(&[("KHR", "maintenance3")])
    .promoted("1_1"),
    Struct::new(
        "PhysicalDeviceMemoryProperties",
        &[
            Struct::member("memoryTypeCount", "u32"),
            Struct::member("memoryTypes", "[VkMemoryType; VK_MAX_MEMORY_TYPES]"),
            Struct::member("memoryHeapCount", "u32"),
            Struct::member("memoryHeaps", "[VkMemoryHeap; VK_MAX_MEMORY_HEAPS]"),
        ],
    ),
    Struct::typed(
        "PhysicalDeviceMultiviewFeatures",
        "PHYSICAL_DEVICE_MULTIVIEW_FEATURES",
        vk_ext_enum(54, 1) as _,
        StructUsage::Both,
        &[
            Struct::member("multiview", "VkBool32"),
            Struct::member("multiviewGeometryShader", "VkBool32"),
            Struct::member("multiviewTessellationShader", "VkBool32"),
        ],
    )
    .extensions_old(&[("KHR", "multiview")])
    .promoted("1_1"),
    Struct::typed(
        "PhysicalDeviceMultiviewProperties",
        "PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES",
        vk_ext_enum(54, 2) as _,
        StructUsage::Sink,
        &[
            Struct::member("maxMultiviewViewCount", "u32"),
            Struct::member("maxMultiviewInstanceIndex", "u32"),
        ],
    )
    .extensions_old(&[("KHR", "multiview")])
    .promoted("1_1"),
    Struct::new(
        "PhysicalDeviceProperties",
        &[
            Struct::member("apiVersion", "u32"),
            Struct::member("driverVersion", "u32"),
            Struct::member("vendorID", "u32"),
            Struct::member("deviceID", "u32"),
            Struct::member("deviceType", "VkPhysicalDeviceType"),
            Struct::member("deviceName", "crate::FixedCStrBuffer<VK_MAX_PHYSICAL_DEVICE_NAME_SIZE>"),
            Struct::member("pipelineCacheUUID", "[u8; VK_UUID_SIZE]"),
            Struct::member("limits", "VkPhysicalDeviceLimits"),
            Struct::member("sparseProperties", "VkPhysicalDeviceSparseProperties"),
        ],
    ),
    Struct::new(
        "PhysicalDeviceSparseProperties",
        &[
            Struct::member("residencyStandard2DBlockShape", "VkBool32"),
            Struct::member("residencyStandard2DMultisampleBlockShape", "VkBool32"),
            Struct::member("residencyStandard3DBlockShape", "VkBool32"),
            Struct::member("residencyAlignedMipSize", "VkBool32"),
            Struct::member("residencyNonResidentStrict", "VkBool32"),
        ],
    ),
    Struct::new(
        "PhysicalDeviceSurfaceInfo2",
        &[Struct::member("surface", "VkSurfaceKHR")],
    )
    .stype(
        "PHYSICAL_DEVICE_SURFACE_INFO_2",
        vk_ext_enum(120, 0) as _,
        StructUsage::Source,
    )
    .extensions_old(&[("KHR", "get_surface_capabilities2")]),
    Struct::typed(
        "PhysicalDeviceVariablePointersFeatures",
        "PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES",
        vk_ext_enum(121, 0) as _,
        StructUsage::Both,
        &[
            Struct::member("variablePointersStorageBuffer", "VkBool32"),
            Struct::member("variablePointers", "VkBool32"),
        ],
    )
    .extensions_old(&[("KHR", "variable_pointers")])
    .promoted("1_1"),
    Struct::new(
        "PipelineCacheCreateInfo",
        &[
            Struct::member("flags", "VkPipelineCacheCreateFlags"),
            Struct::member("initialDataSize", "usize"),
            Struct::member("pInitialData", "*const core::ffi::c_void"),
        ],
    )
    .stype("PIPELINE_CACHE_CREATE_INFO", 17, StructUsage::Source),
    Struct::new(
        "PipelineColorBlendAttachmentState",
        &[
            Struct::member("blendEnable", "VkBool32"),
            Struct::member("srcColorBlendFactor", "VkBlendFactor"),
            Struct::member("dstColorBlendFactor", "VkBlendFactor"),
            Struct::member("colorBlendOp", "VkBlendOp"),
            Struct::member("srcAlphaBlendFactor", "VkBlendFactor"),
            Struct::member("dstAlphaBlendFactor", "VkBlendFactor"),
            Struct::member("alphaBlendOp", "VkBlendOp"),
            Struct::member("colorWriteMask", "VkColorComponentFlags"),
        ],
    ),
    Struct::new(
        "PipelineColorBlendStateCreateInfo",
        &[
            Struct::member("flags", "VkPipelineColorBlendStateCreateFlags"),
            Struct::member("logicOpEnable", "VkBool32"),
            Struct::member("logicOp", "VkLogicOp"),
            Struct::member("attachmentCount", "u32"),
            Struct::member("pAttachments", "*const VkPipelineColorBlendAttachmentState"),
            Struct::member("blendConstants", "[core::ffi::c_float; 4]"),
        ],
    )
    .stype("PIPELINE_COLOR_BLEND_STATE_CREATE_INFO", 26, StructUsage::Source),
    Struct::new(
        "PipelineDepthStencilStateCreateInfo",
        &[
            Struct::member("flags", "VkPipelineDepthStencilStateCreateFlags"),
            Struct::member("depthTestEnable", "VkBool32"),
            Struct::member("depthWriteEnable", "VkBool32"),
            Struct::member("depthCompareOp", "VkCompareOp"),
            Struct::member("depthBoundsTestEnable", "VkBool32"),
            Struct::member("stencilTestEnable", "VkBool32"),
            Struct::member("front", "VkStencilOpState"),
            Struct::member("back", "VkStencilOpState"),
            Struct::member("minDepthBounds", "core::ffi::c_float"),
            Struct::member("maxDepthBounds", "core::ffi::c_float"),
        ],
    )
    .stype("PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO", 25, StructUsage::Source),
    Struct::new(
        "PipelineDynamicStateCreateInfo",
        &[
            Struct::member("flags", "VkPipelineDynamicStateCreateFlags"),
            Struct::member("dynamicStateCount", "u32"),
            Struct::member("pDynamicStates", "*const VkDynamicState"),
        ],
    )
    .stype("PIPELINE_DYNAMIC_STATE_CREATE_INFO", 27, StructUsage::Source),
    Struct::new(
        "PipelineInputAssemblyStateCreateInfo",
        &[
            Struct::member("flags", "VkPipelineInputAssemblyStateCreateFlags"),
            Struct::member("topology", "VkPrimitiveTopology"),
            Struct::member("primitiveRestartEnable", "VkBool32"),
        ],
    )
    .stype("PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO", 20, StructUsage::Source),
    Struct::new(
        "PipelineLayoutCreateInfo",
        &[
            Struct::member("flags", "VkPipelineLayoutCreateFlags"),
            Struct::member("setLayoutCount", "u32"),
            Struct::member("pSetLayouts", "*const VkDescriptorSetLayout"),
            Struct::member("pushConstantRangeCount", "u32"),
            Struct::member("pPushConstantRanges", "*const VkPushConstantRange"),
        ],
    )
    .stype("PIPELINE_LAYOUT_CREATE_INFO", 30, StructUsage::Source),
    Struct::new(
        "PipelineMultisampleStateCreateInfo",
        &[
            Struct::member("flags", "VkPipelineMultisampleStateCreateFlags"),
            Struct::member("rasterizationSamples", "VkSampleCountFlagBits"),
            Struct::member("sampleShadingEnable", "VkBool32"),
            Struct::member("minSampleShading", "core::ffi::c_float"),
            Struct::member("pSampleMask", "*const VkSampleMask"),
            Struct::member("alphaToCoverageEnable", "VkBool32"),
            Struct::member("alphaToOneEnable", "VkBool32"),
        ],
    )
    .stype("PIPELINE_MULTISAMPLE_STATE_CREATE_INFO", 24, StructUsage::Source),
    Struct::new(
        "PipelineRasterizationStateCreateInfo",
        &[
            Struct::member("flags", "VkPipelineRasterizationStateCreateFlags"),
            Struct::member("depthClampEnable", "VkBool32"),
            Struct::member("rasterizerDiscardEnable", "VkBool32"),
            Struct::member("polygonMode", "VkPolygonMode"),
            Struct::member("cullMode", "VkCullModeFlags"),
            Struct::member("frontFace", "VkFrontFace"),
            Struct::member("depthBiasEnable", "VkBool32"),
            Struct::member("depthBiasConstantFactor", "core::ffi::c_float"),
            Struct::member("depthBiasClamp", "core::ffi::c_float"),
            Struct::member("depthBiasSlopeFactor", "core::ffi::c_float"),
            Struct::member("lineWidth", "core::ffi::c_float"),
        ],
    )
    .stype("PIPELINE_RASTERIZATION_STATE_CREATE_INFO", 23, StructUsage::Source),
    Struct::new(
        "PipelineShaderStageCreateInfo",
        &[
            Struct::member("flags", "VkPipelineShaderStageCreateFlags"),
            Struct::member("stage", "VkShaderStageFlagBits"),
            Struct::member("module", "Option<VkShaderModule>"),
            Struct::member("pName", "*const core::ffi::c_char"),
            Struct::member("pSpecializationInfo", "*const VkSpecializationInfo"),
        ],
    )
    .stype("PIPELINE_SHADER_STAGE_CREATE_INFO", 18, StructUsage::Source),
    Struct::new(
        "PipelineTessellationStateCreateInfo",
        &[
            Struct::member("flags", "VkPipelineTessellationStateCreateFlags"),
            Struct::member("patchControlPoints", "u32"),
        ],
    )
    .stype("PIPELINE_TESSELLATION_STATE_CREATE_INFO", 21, StructUsage::Source),
    Struct::new(
        "PipelineVertexInputStateCreateInfo",
        &[
            Struct::member("flags", "VkPipelineVertexInputStateCreateFlags"),
            Struct::member("vertexBindingDescriptionCount", "u32"),
            Struct::member("pVertexBindingDescriptions", "*const VkVertexInputBindingDescription"),
            Struct::member("vertexAttributeDescriptionCount", "u32"),
            Struct::member(
                "pVertexAttributeDescriptions",
                "*const VkVertexInputAttributeDescription",
            ),
        ],
    )
    .stype("PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO", 19, StructUsage::Source),
    Struct::new(
        "PipelineViewportStateCreateInfo",
        &[
            Struct::member("flags", "VkPipelineViewportStateCreateFlags"),
            Struct::member("viewportCount", "u32"),
            Struct::member("pViewports", "*const VkViewport"),
            Struct::member("scissorCount", "u32"),
            Struct::member("pScissors", "*const VkRect2D"),
        ],
    )
    .stype("PIPELINE_VIEWPORT_STATE_CREATE_INFO", 22, StructUsage::Source),
    Struct::new(
        "PushConstantRange",
        &[
            Struct::member("stageFlags", "VkShaderStageFlags"),
            Struct::member("offset", "u32"),
            Struct::member("size", "u32"),
        ],
    ),
    Struct::new(
        "QueryPoolCreateInfo",
        &[
            Struct::member("flags", "VkQueryPoolCreateFlags"),
            Struct::member("queryType", "VkQueryType"),
            Struct::member("queryCount", "u32"),
            Struct::member("pipelineStatistics", "VkQueryPipelineStatisticFlags"),
        ],
    )
    .stype("QUERY_POOL_CREATE_INFO", 11, StructUsage::Source),
    Struct::new(
        "QueueFamilyProperties",
        &[
            Struct::member("queueFlags", "VkQueueFlags"),
            Struct::member("queueCount", "u32"),
            Struct::member("timestampValidBits", "u32"),
            Struct::member("minImageTransferGranularity", "VkExtent3D"),
        ],
    ),
    Struct::new(
        "Rect2D",
        &[
            Struct::member("offset", "VkOffset2D"),
            Struct::member("extent", "VkExtent2D"),
        ],
    )
    .copyable()
    .equatable()
    .hashable(),
    Struct::new(
        "RenderPassBeginInfo",
        &[
            Struct::member("renderPass", "VkRenderPass"),
            Struct::member("framebuffer", "VkFramebuffer"),
            Struct::member("renderArea", "VkRect2D"),
            Struct::member("clearValueCount", "u32"),
            Struct::member("pClearValues", "*const VkClearValue"),
        ],
    )
    .stype("RENDER_PASS_BEGIN_INFO", 43, StructUsage::Source),
    Struct::new(
        "RenderPassCreateInfo",
        &[
            Struct::member("flags", "VkRenderPassCreateFlags"),
            Struct::member("attachmentCount", "u32"),
            Struct::member("pAttachments", "*const VkAttachmentDescription"),
            Struct::member("subpassCount", "u32"),
            Struct::member("pSubpasses", "*const VkSubpassDescription"),
            Struct::member("dependencyCount", "u32"),
            Struct::member("pDependencies", "*const VkSubpassDependency"),
        ],
    )
    .stype("RENDER_PASS_CREATE_INFO", 38, StructUsage::Source),
    Struct::typed(
        "RenderPassMultiviewCreateInfo",
        "RENDER_PASS_MULTIVIEW_CREATE_INFO",
        vk_ext_enum(54, 0) as _,
        StructUsage::Source,
        &[
            Struct::member("subpassCount", "u32"),
            Struct::member("pViewMasks", "*const u32"),
            Struct::member("dependencyCount", "u32"),
            Struct::member("pViewOffsets", "*const i32"),
            Struct::member("correlationMaskCount", "u32"),
            Struct::member("pCorrelationMasks", "*const u32"),
        ],
    )
    .extensions_old(&[("KHR", "multiview")])
    .promoted("1_1"),
    Struct::new(
        "SamplerCreateInfo",
        &[
            Struct::member("flags", "VkSamplerCreateFlags"),
            Struct::member("magFilter", "VkFilter"),
            Struct::member("minFilter", "VkFilter"),
            Struct::member("mipmapMode", "VkSamplerMipmapMode"),
            Struct::member("addressModeU", "VkSamplerAddressMode"),
            Struct::member("addressModeV", "VkSamplerAddressMode"),
            Struct::member("addressModeW", "VkSamplerAddressMode"),
            Struct::member("mipLodBias", "core::ffi::c_float"),
            Struct::member("anisotropyEnable", "VkBool32"),
            Struct::member("maxAnisotropy", "core::ffi::c_float"),
            Struct::member("compareEnable", "VkBool32"),
            Struct::member("compareOp", "VkCompareOp"),
            Struct::member("minLod", "core::ffi::c_float"),
            Struct::member("maxLod", "core::ffi::c_float"),
            Struct::member("borderColor", "VkBorderColor"),
            Struct::member("unnormalizedCoordinates", "VkBool32"),
        ],
    )
    .stype("SAMPLER_CREATE_INFO", 31, StructUsage::Source),
    Struct::new(
        "SemaphoreCreateInfo",
        &[Struct::member("flags", "VkSemaphoreCreateFlags")],
    )
    .stype("SEMAPHORE_CREATE_INFO", 9, StructUsage::Source),
    Struct::new(
        "ShaderModuleCreateInfo",
        &[
            Struct::member("flags", "VkShaderModuleCreateFlags"),
            Struct::member("codeSize", "usize"),
            Struct::member("pCode", "*const u32"),
        ],
    )
    .stype("SHADER_MODULE_CREATE_INFO", 16, StructUsage::Source),
    Struct::new(
        "SparseBufferMemoryBindInfo",
        &[
            Struct::member("buffer", "VkBuffer"),
            Struct::member("bindCount", "u32"),
            Struct::member("pBinds", "*const VkSparseMemoryBind"),
        ],
    ),
    Struct::new(
        "SparseImageFormatProperties",
        &[
            Struct::member("aspectMask", "VkImageAspectFlags"),
            Struct::member("imageGranularity", "VkExtent3D"),
            Struct::member("flags", "VkSparseImageFormatFlags"),
        ],
    ),
    Struct::new(
        "SparseImageMemoryBind",
        &[
            Struct::member("subresource", "VkImageSubresource"),
            Struct::member("offset", "VkOffset3D"),
            Struct::member("extent", "VkExtent3D"),
            Struct::member("memory", "VkDeviceMemory"),
            Struct::member("memoryOffset", DEVICE_SIZE_TYPE),
            Struct::member("flags", "VkSparseMemoryBindFlags"),
        ],
    ),
    Struct::new(
        "SparseImageMemoryBindInfo",
        &[
            Struct::member("image", "VkImage"),
            Struct::member("bindCount", "u32"),
            Struct::member("pBinds", "*const VkSparseImageMemoryBind"),
        ],
    ),
    Struct::new(
        "SparseImageMemoryRequirements",
        &[
            Struct::member("formatProperties", "VkSparseImageFormatProperties"),
            Struct::member("imageMipTailFirstLod", "u32"),
            Struct::member("imageMipTailSize", DEVICE_SIZE_TYPE),
            Struct::member("imageMipTailOffset", DEVICE_SIZE_TYPE),
            Struct::member("imageMipTailStride", DEVICE_SIZE_TYPE),
        ],
    ),
    Struct::new(
        "SparseImageOpaqueMemoryBindInfo",
        &[
            Struct::member("image", "VkImage"),
            Struct::member("bindCount", "u32"),
            Struct::member("pBinds", "*const VkSparseMemoryBind"),
        ],
    ),
    Struct::new(
        "SparseMemoryBind",
        &[
            Struct::member("resourceOffset", DEVICE_SIZE_TYPE),
            Struct::member("size", DEVICE_SIZE_TYPE),
            Struct::member("memory", "VkDeviceMemory"),
            Struct::member("memoryOffset", DEVICE_SIZE_TYPE),
            Struct::member("flags", "VkSparseMemoryBindFlags"),
        ],
    ),
    Struct::new(
        "SpecializationInfo",
        &[
            Struct::member("mapEntryCount", "u32"),
            Struct::member("pMapEntries", "*const VkSpecializationMapEntry"),
            Struct::member("dataSize", "usize"),
            Struct::member("pData", "*const core::ffi::c_void"),
        ],
    ),
    Struct::new(
        "SpecializationMapEntry",
        &[
            Struct::member("constantID", "u32"),
            Struct::member("offset", "u32"),
            Struct::member("size", "usize"),
        ],
    ),
    Struct::new(
        "StencilOpState",
        &[
            Struct::member("failOp", "VkStencilOp"),
            Struct::member("passOp", "VkStencilOp"),
            Struct::member("depthFailOp", "VkStencilOp"),
            Struct::member("compareOp", "VkCompareOp"),
            Struct::member("compareMask", "u32"),
            Struct::member("writeMask", "u32"),
            Struct::member("reference", "u32"),
        ],
    ),
    Struct::new(
        "SubmitInfo",
        &[
            Struct::member("waitSemaphoreCount", "u32"),
            Struct::member("pWaitSemaphores", "*const VkSemaphore"),
            Struct::member("pWaitDstStageMask", "*const VkPipelineStageFlags"),
            Struct::member("commandBufferCount", "u32"),
            Struct::member("pCommandBuffers", "*const VkCommandBuffer"),
            Struct::member("signalSemaphoreCount", "u32"),
            Struct::member("pSignalSemaphores", "*const VkSemaphore"),
        ],
    )
    .stype("SUBMIT_INFO", 4, StructUsage::Source),
    Struct::new(
        "SubpassDependency",
        &[
            Struct::member("srcSubpass", "u32"),
            Struct::member("dstSubpass", "u32"),
            Struct::member("srcStageMask", "VkPipelineStageFlags"),
            Struct::member("dstStageMask", "VkPipelineStageFlags"),
            Struct::member("srcAccessMask", "VkAccessFlags"),
            Struct::member("dstAccessMask", "VkAccessFlags"),
            Struct::member("dependencyFlags", "VkDependencyFlags"),
        ],
    ),
    Struct::new(
        "SubpassDescription",
        &[
            Struct::member("flags", "VkSubpassDescriptionFlags"),
            Struct::member("pipelineBindPoint", "VkPipelineBindPoint"),
            Struct::member("inputAttachmentCount", "u32"),
            Struct::member("pInputAttachments", "*const VkAttachmentReference"),
            Struct::member("colorAttachmentCount", "u32"),
            Struct::member("pColorAttachments", "*const VkAttachmentReference"),
            Struct::member("pResolveAttachments", "*const VkAttachmentReference"),
            Struct::member("pDepthStencilAttachment", "*const VkAttachmentReference"),
            Struct::member("preserveAttachmentCount", "u32"),
            Struct::member("pPreserveAttachments", "*const u32"),
        ],
    ),
    Struct::new(
        "SubresourceLayout",
        &[
            Struct::member("offset", DEVICE_SIZE_TYPE),
            Struct::member("size", DEVICE_SIZE_TYPE),
            Struct::member("rowPitch", DEVICE_SIZE_TYPE),
            Struct::member("arrayPitch", DEVICE_SIZE_TYPE),
            Struct::member("depthPitch", DEVICE_SIZE_TYPE),
        ],
    ),
    Struct::typed(
        "SurfaceCapabilities2",
        "SURFACE_CAPABILITIES_2",
        vk_ext_enum(120, 1) as _,
        StructUsage::Sink,
        &[Struct::member("surfaceCapabilities", "VkSurfaceCapabilitiesKHR")],
    )
    .extensions_old(&[("KHR", "get_surface_capabilities2")]),
    Struct::typed(
        "SurfaceFormat2",
        "SURFACE_FORMAT_2",
        vk_ext_enum(120, 2) as _,
        StructUsage::Sink,
        &[Struct::member("surfaceFormat", "VkSurfaceFormatKHR")],
    )
    .extensions_old(&[("KHR", "get_surface_capabilities2")]),
    Struct::new(
        "VertexInputAttributeDescription",
        &[
            Struct::member("location", "u32"),
            Struct::member("binding", "u32"),
            Struct::member("format", "VkFormat"),
            Struct::member("offset", "u32"),
        ],
    )
    .copyable(),
    Struct::new(
        "VertexInputBindingDescription",
        &[
            Struct::member("binding", "u32"),
            Struct::member("stride", "u32"),
            Struct::member("inputRate", "VkVertexInputRate"),
        ],
    )
    .copyable(),
    Struct::new(
        "Viewport",
        &[
            Struct::member("x", "core::ffi::c_float"),
            Struct::member("y", "core::ffi::c_float"),
            Struct::member("width", "core::ffi::c_float"),
            Struct::member("height", "core::ffi::c_float"),
            Struct::member("minDepth", "core::ffi::c_float"),
            Struct::member("maxDepth", "core::ffi::c_float"),
        ],
    ),
    Struct::new(
        "Win32KeyedMutexAcquireReleaseInfo",
        &[
            Struct::member("acquireCount", "u32"),
            Struct::member("pAcquireSyncs", "*const VkDeviceMemory"),
            Struct::member("pAcquireKeys", "*const u64"),
            Struct::member("pAcquireTimeouts", "*const u32"),
            Struct::member("releaseCount", "u32"),
            Struct::member("pReleaseSyncs", "*const VkDeviceMemory"),
            Struct::member("pReleaseKeys", "*const u64"),
        ],
    )
    .stype(
        "WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO",
        vk_ext_enum(76, 0) as _,
        StructUsage::Source,
    )
    .extensions_old(&[("KHR", "win32_keyed_mutex")]),
    Struct::new(
        "WriteDescriptorSet",
        &[
            Struct::member("dstSet", "VkDescriptorSet"),
            Struct::member("dstBinding", "u32"),
            Struct::member("dstArrayElement", "u32"),
            Struct::member("descriptorCount", "u32"),
            Struct::member("descriptorType", "VkDescriptorType"),
            Struct::member("pImageInfo", "*const VkDescriptorImageInfo"),
            Struct::member("pBufferInfo", "*const VkDescriptorBufferInfo"),
            Struct::member("pTexelBufferView", "*const VkBufferView"),
        ],
    )
    .stype("WRITE_DESCRIPTOR_SET", 35, StructUsage::Source),
    Struct::typed(
        "MemoryBarrier2",
        "MEMORY_BARRIER_2",
        vk_ext_enum(315, 0) as _,
        StructUsage::Source,
        &[
            Struct::member("srcStageMask", "VkPipelineStageFlags2KHR"),
            Struct::member("srcAccessMask", "VkAccessFlags2KHR"),
            Struct::member("dstStageMask", "VkPipelineStageFlags2KHR"),
            Struct::member("dstAccessMask", "VkAccessFlags2KHR"),
        ],
    )
    .extensions_old(&[("KHR", "synchronization2")])
    .promoted("1_3"),
    Struct::typed(
        "BufferMemoryBarrier2",
        "BUFFER_MEMORY_BARRIER_2",
        vk_ext_enum(315, 1) as _,
        StructUsage::Source,
        &[
            Struct::member("srcStageMask", "VkPipelineStageFlags2KHR"),
            Struct::member("srcAccessMask", "VkAccessFlags2KHR"),
            Struct::member("dstStageMask", "VkPipelineStageFlags2KHR"),
            Struct::member("dstAccessMask", "VkAccessFlags2KHR"),
            Struct::member("srcQueueFamilyIndex", "u32"),
            Struct::member("dstQueueFamilyIndex", "u32"),
            Struct::member("buffer", "VkBuffer"),
            Struct::member("offset", DEVICE_SIZE_TYPE),
            Struct::member("size", DEVICE_SIZE_TYPE),
        ],
    )
    .extensions_old(&[("KHR", "synchronization2")])
    .promoted("1_3"),
    Struct::typed(
        "ImageMemoryBarrier2",
        "IMAGE_MEMORY_BARRIER_2",
        vk_ext_enum(315, 2) as _,
        StructUsage::Source,
        &[
            Struct::member("srcStageMask", "VkPipelineStageFlags2KHR"),
            Struct::member("srcAccessMask", "VkAccessFlags2KHR"),
            Struct::member("dstStageMask", "VkPipelineStageFlags2KHR"),
            Struct::member("dstAccessMask", "VkAccessFlags2KHR"),
            Struct::member("oldLayout", "VkImageLayout"),
            Struct::member("newLayout", "VkImageLayout"),
            Struct::member("srcQueueFamilyIndex", "u32"),
            Struct::member("dstQueueFamilyIndex", "u32"),
            Struct::member("image", "VkImage"),
            Struct::member("subresourceRange", "VkImageSubresourceRange"),
        ],
    )
    .extensions_old(&[("KHR", "synchronization2")])
    .promoted("1_3"),
    Struct::typed(
        "DependencyInfo",
        "DEPENDENCY_INFO",
        vk_ext_enum(315, 3) as _,
        StructUsage::Source,
        &[
            Struct::member("dependencyFlags", "VkDependencyFlags"),
            Struct::member("memoryBarrierCount", "u32"),
            Struct::member("pMemoryBarriers", "*const VkMemoryBarrier2KHR"),
            Struct::member("bufferMemoryBarrierCount", "u32"),
            Struct::member("pBufferMemoryBarriers", "*const VkBufferMemoryBarrier2KHR"),
            Struct::member("imageMemoryBarrierCount", "u32"),
            Struct::member("pImageMemoryBarriers", "*const VkImageMemoryBarrier2KHR"),
        ],
    )
    .extensions_old(&[("KHR", "synchronization2")])
    .promoted("1_3"),
    Struct::typed(
        "SubmitInfo2",
        "SUBMIT_INFO_2",
        vk_ext_enum(315, 4) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkSubmitFlagsKHR"),
            Struct::member("waitSemaphoreInfoCount", "u32"),
            Struct::member("pWaitSemaphoreInfos", "*const VkSemaphoreSubmitInfoKHR"),
            Struct::member("commandBufferInfoCount", "u32"),
            Struct::member("pCommandBufferInfos", "*const VkCommandBufferSubmitInfoKHR"),
            Struct::member("signalSemaphoreInfoCount", "u32"),
            Struct::member("pSignalSemaphoreInfos", "*const VkSemaphoreSubmitInfoKHR"),
        ],
    )
    .extensions_old(&[("KHR", "synchronization2")])
    .promoted("1_3"),
    Struct::typed(
        "SemaphoreSubmitInfo",
        "SEMAPHORE_SUBMIT_INFO",
        vk_ext_enum(315, 5) as _,
        StructUsage::Source,
        &[
            Struct::member("semaphore", "VkSemaphore"),
            Struct::member("value", "u64"),
            Struct::member("stageMask", "VkPipelineStageFlags2KHR"),
            Struct::member("deviceIndex", "u32"),
        ],
    )
    .extensions_old(&[("KHR", "synchronization2")])
    .promoted("1_3"),
    Struct::typed(
        "CommandBufferSubmitInfo",
        "COMMAND_BUFFER_SUBMIT_INFO",
        vk_ext_enum(315, 6) as _,
        StructUsage::Source,
        &[
            Struct::member("commandBuffer", "VkCommandBuffer"),
            Struct::member("deviceMask", "u32"),
        ],
    )
    .extensions_old(&[("KHR", "synchronization2")])
    .promoted("1_3"),
    Struct::typed(
        "PhysicalDeviceSynchronization2Features",
        "PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES",
        vk_ext_enum(315, 7) as _,
        StructUsage::Both,
        &[Struct::member("synchronization2", "VkBool32")],
    )
    .extensions_old(&[("KHR", "synchronization2")])
    .promoted("1_3"),
];

const UNIONS: &[Union] = &[
    Union::new(
        "ClearColorValue",
        &[
            Union::member("float32", "[core::ffi::c_float; 4]"),
            Union::member("int32", "[i32; 4]"),
            Union::member("uint32", "[u32; 4]"),
        ],
    ),
    Union::new(
        "ClearValue",
        &[
            Union::member("color", "VkClearColorValue"),
            Union::member("depthStencil", "VkClearDepthStencilValue"),
        ],
    ),
];

const COMMANDS: &[Command] = &[
    Command::new(
        "CreateInstance",
        &[
            ("pCreateInfo", "*const VkInstanceCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pInstance", "*mut VkInstance"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyInstance",
        &[
            ("instance", "VkInstance"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "EnumeratePhysicalDevices",
        &[
            ("instance", "VkInstance"),
            ("pPhysicalDeviceCount", "*mut u32"),
            ("pPhysicalDevices", "*mut VkPhysicalDevice"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "GetPhysicalDeviceFeatures",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pFeatures", "*mut VkPhysicalDeviceFeatures"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetPhysicalDeviceFormatProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("format", "VkFormat"),
            ("pFormatProperties", "*mut VkFormatProperties"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetPhysicalDeviceImageFormatProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("format", "VkFormat"),
            ("r#type", "VkImageType"),
            ("tiling", "VkImageTiling"),
            ("usage", "VkImageUsageFlags"),
            ("flags", "VkImageCreateFlags"),
            ("pImageFormatProperties", "*mut VkImageFormatProperties"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "GetPhysicalDeviceProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pProperties", "*mut VkPhysicalDeviceProperties"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetPhysicalDeviceQueueFamilyProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pQueueFamilyPropertyCount", "*mut u32"),
            ("pQueueFamilyProperties", "*mut VkQueueFamilyProperties"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetPhysicalDeviceMemoryProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pMemoryProperties", "*mut VkPhysicalDeviceMemoryProperties"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetInstanceProcAddr",
        &[("instance", "VkInstance"), ("pName", "*const core::ffi::c_char")],
    )
    .returns("Option<PFN_vkVoidFunction>")
    .static_callable(),
    Command::new(
        "GetDeviceProcAddr",
        &[("device", "VkDevice"), ("pName", "*const core::ffi::c_char")],
    )
    .returns("Option<PFN_vkVoidFunction>")
    .static_callable(),
    Command::new(
        "CreateDevice",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pCreateInfo", "*const VkDeviceCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pDevice", "*mut VkDevice"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyDevice",
        &[("device", "VkDevice"), ("pAllocator", "*const VkAllocationCallbacks")],
    )
    .static_callable(),
    Command::new(
        "EnumerateInstanceExtensionProperties",
        &[
            ("pLayerName", "*const core::ffi::c_char"),
            ("pPropertyCount", "*mut u32"),
            ("pProperties", "*mut VkExtensionProperties"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "EnumerateDeviceExtensionProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pLayerName", "*const core::ffi::c_char"),
            ("pPropertyCount", "*mut u32"),
            ("pProperties", "*mut VkExtensionProperties"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "EnumerateInstanceLayerProperties",
        &[
            ("pPropertyCount", "*mut u32"),
            ("pProperties", "*mut VkLayerProperties"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "EnumerateDeviceLayerProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pPropertyCount", "*mut u32"),
            ("pProperties", "*mut VkLayerProperties"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "GetDeviceQueue",
        &[
            ("device", "VkDevice"),
            ("queueFamilyIndex", "u32"),
            ("queueIndex", "u32"),
            ("pQueue", "*mut VkQueue"),
        ],
    )
    .static_callable(),
    Command::new(
        "QueueSubmit",
        &[
            ("queue", "VkQueue"),
            ("submitCount", "u32"),
            ("pSubmits", "*const VkSubmitInfo"),
            ("fence", "Option<VkFence>"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new("QueueWaitIdle", &[("queue", "VkQueue")])
        .failable()
        .static_callable(),
    Command::new("DeviceWaitIdle", &[("device", "VkDevice")])
        .failable()
        .static_callable(),
    Command::new(
        "AllocateMemory",
        &[
            ("device", "VkDevice"),
            ("pAllocateInfo", "*const VkMemoryAllocateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pMemory", "*mut VkDeviceMemory"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "FreeMemory",
        &[
            ("device", "VkDevice"),
            ("memory", "VkDeviceMemory"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "MapMemory",
        &[
            ("device", "VkDevice"),
            ("memory", "VkDeviceMemory"),
            ("offset", "VkDeviceSize"),
            ("size", "VkDeviceSize"),
            ("flags", "VkMemoryMapFlags"),
            ("ppData", "*mut *mut core::ffi::c_void"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new("UnmapMemory", &[("device", "VkDevice"), ("memory", "VkDeviceMemory")]).static_callable(),
    Command::new(
        "FlushMappedMemoryRanges",
        &[
            ("device", "VkDevice"),
            ("memoryRangeCount", "u32"),
            ("pMemoryRanges", "*const VkMappedMemoryRange"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "InvalidateMappedMemoryRanges",
        &[
            ("device", "VkDevice"),
            ("memoryRangeCount", "u32"),
            ("pMemoryRanges", "*const VkMappedMemoryRange"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "GetDeviceMemoryCommitment",
        &[
            ("device", "VkDevice"),
            ("memory", "VkDeviceMemory"),
            ("pCommitmentMemoryInBytes", "*mut VkDeviceSize"),
        ],
    )
    .static_callable(),
    Command::new(
        "BindBufferMemory",
        &[
            ("device", "VkDevice"),
            ("buffer", "VkBuffer"),
            ("memory", "VkDeviceMemory"),
            ("memoryOffset", "VkDeviceSize"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "BindImageMemory",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("memory", "VkDeviceMemory"),
            ("memoryOffset", "VkDeviceSize"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "GetBufferMemoryRequirements",
        &[
            ("device", "VkDevice"),
            ("buffer", "VkBuffer"),
            ("pMemoryRequirements", "*mut VkMemoryRequirements"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetImageMemoryRequirements",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("pMemoryRequirements", "*mut VkMemoryRequirements"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetImageSparseMemoryRequirements",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("pSparseMemoryRequirementsCount", "*mut u32"),
            ("pSparseMemoryRequirements", "*mut VkSparseImageMemoryRequirements"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetPhysicalDeviceSparseImageFormatProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("format", "VkFormat"),
            ("r#type", "VkImageType"),
            ("samples", "VkSampleCountFlags"),
            ("usage", "VkImageUsageFlags"),
            ("tiling", "VkImageTiling"),
            ("pPropertyCount", "*mut u32"),
            ("pProperties", "*mut VkSparseImageFormatProperties"),
        ],
    )
    .static_callable(),
    Command::new(
        "QueueBindSparse",
        &[
            ("queue", "VkQueue"),
            ("bindInfoCount", "u32"),
            ("pBindInfos", "*const VkBindSparseInfo"),
            ("fence", "Option<VkFence>"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "CreateFence",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkFenceCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pFence", "*mut VkFence"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyFence",
        &[
            ("device", "VkDevice"),
            ("fence", "VkFence"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "ResetFences",
        &[
            ("device", "VkDevice"),
            ("fenceCount", "u32"),
            ("pFences", "*const VkFence"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new("GetFenceStatus", &[("device", "VkDevice"), ("fence", "VkFence")])
        .failable()
        .static_callable(),
    Command::new(
        "WaitForFences",
        &[
            ("device", "VkDevice"),
            ("fenceCount", "u32"),
            ("pFences", "*const VkFence"),
            ("waitAll", "VkBool32"),
            ("timeout", "u64"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "CreateSemaphore",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkSemaphoreCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSemaphore", "*mut VkSemaphore"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroySemaphore",
        &[
            ("device", "VkDevice"),
            ("semaphore", "VkSemaphore"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateEvent",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkEventCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pEvent", "*mut VkEvent"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyEvent",
        &[
            ("device", "VkDevice"),
            ("event", "VkEvent"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new("GetEventStatus", &[("device", "VkDevice"), ("event", "VkEvent")])
        .failable()
        .static_callable(),
    Command::new("SetEvent", &[("device", "VkDevice"), ("event", "VkEvent")])
        .failable()
        .static_callable(),
    Command::new("ResetEvent", &[("device", "VkDevice"), ("event", "VkEvent")])
        .failable()
        .static_callable(),
    Command::new(
        "CreateQueryPool",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkQueryPoolCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pQueryPool", "*mut VkQueryPool"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyQueryPool",
        &[
            ("device", "VkDevice"),
            ("queryPool", "VkQueryPool"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetQueryPoolResults",
        &[
            ("device", "VkDevice"),
            ("queryPool", "VkQueryPool"),
            ("firstQuery", "u32"),
            ("queryCount", "u32"),
            ("dataSize", "usize"),
            ("pData", "*mut core::ffi::c_void"),
            ("stride", "VkDeviceSize"),
            ("flags", "VkQueryResultFlags"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "CreateBuffer",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkBufferCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pBuffer", "*mut VkBuffer"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyBuffer",
        &[
            ("device", "VkDevice"),
            ("buffer", "VkBuffer"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateBufferView",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkBufferViewCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pView", "*mut VkBufferView"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyBufferView",
        &[
            ("device", "VkDevice"),
            ("bufferView", "VkBufferView"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateImage",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkImageCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pImage", "*mut VkImage"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyImage",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetImageSubresourceLayout",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("pSubresource", "*const VkImageSubresource"),
            ("pLayout", "*mut VkSubresourceLayout"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateImageView",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkImageViewCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pView", "*mut VkImageView"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyImageView",
        &[
            ("device", "VkDevice"),
            ("view", "VkImageView"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateShaderModule",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkShaderModuleCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pModule", "*mut VkShaderModule"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyShaderModule",
        &[
            ("device", "VkDevice"),
            ("module", "VkShaderModule"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreatePipelineCache",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkPipelineCacheCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pPipelineCache", "*mut VkPipelineCache"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyPipelineCache",
        &[
            ("device", "VkDevice"),
            ("pipelineCache", "VkPipelineCache"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetPipelineCacheData",
        &[
            ("device", "VkDevice"),
            ("pipelineCache", "VkPipelineCache"),
            ("pDataSize", "*mut usize"),
            ("pData", "*mut core::ffi::c_void"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "MergePipelineCaches",
        &[
            ("device", "VkDevice"),
            ("dstCache", "VkPipelineCache"),
            ("srcCacheCount", "u32"),
            ("pSrcCaches", "*const VkPipelineCache"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "CreateGraphicsPipelines",
        &[
            ("device", "VkDevice"),
            ("pipelineCache", "Option<VkPipelineCache>"),
            ("createInfoCount", "u32"),
            ("pCreateInfos", "*const VkGraphicsPipelineCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pPipelines", "*mut VkPipeline"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "CreateComputePipelines",
        &[
            ("device", "VkDevice"),
            ("pipelineCache", "Option<VkPipelineCache>"),
            ("createInfoCount", "u32"),
            ("pCreateInfos", "*const VkComputePipelineCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pPipelines", "*mut VkPipeline"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyPipeline",
        &[
            ("device", "VkDevice"),
            ("pipeline", "VkPipeline"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreatePipelineLayout",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkPipelineLayoutCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pPipelineLayout", "*mut VkPipelineLayout"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyPipelineLayout",
        &[
            ("device", "VkDevice"),
            ("pipelineLayout", "VkPipelineLayout"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateSampler",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkSamplerCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSampler", "*mut VkSampler"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroySampler",
        &[
            ("device", "VkDevice"),
            ("sampler", "VkSampler"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateDescriptorSetLayout",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkDescriptorSetLayoutCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSetLayout", "*mut VkDescriptorSetLayout"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyDescriptorSetLayout",
        &[
            ("device", "VkDevice"),
            ("descriptorSetLayout", "VkDescriptorSetLayout"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateDescriptorPool",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkDescriptorPoolCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pPool", "*mut VkDescriptorPool"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyDescriptorPool",
        &[
            ("device", "VkDevice"),
            ("descriptorPool", "VkDescriptorPool"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "ResetDescriptorPool",
        &[
            ("device", "VkDevice"),
            ("descriptorPool", "VkDescriptorPool"),
            ("flags", "VkDescriptorPoolResetFlags"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "AllocateDescriptorSets",
        &[
            ("device", "VkDevice"),
            ("pAllocateInfo", "*const VkDescriptorSetAllocateInfo"),
            ("pDescriptorSets", "*mut VkDescriptorSet"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "FreeDescriptorSets",
        &[
            ("device", "VkDevice"),
            ("descriptorPool", "VkDescriptorPool"),
            ("descriptorSetCount", "u32"),
            ("pDescriptorSets", "*const VkDescriptorSet"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "UpdateDescriptorSets",
        &[
            ("device", "VkDevice"),
            ("descriptorWriteCount", "u32"),
            ("pDescriptorWrites", "*const VkWriteDescriptorSet"),
            ("descriptorCopyCount", "u32"),
            ("pDescriptorCopies", "*const VkCopyDescriptorSet"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateFramebuffer",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkFramebufferCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pFramebuffer", "*mut VkFramebuffer"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyFramebuffer",
        &[
            ("device", "VkDevice"),
            ("framebuffer", "VkFramebuffer"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateRenderPass",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkRenderPassCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pRenderPass", "*mut VkRenderPass"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyRenderPass",
        &[
            ("device", "VkDevice"),
            ("renderPass", "VkRenderPass"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "GetRenderAreaGranularity",
        &[
            ("device", "VkDevice"),
            ("renderPass", "VkRenderPass"),
            ("pGranularity", "*mut VkExtent2D"),
        ],
    )
    .static_callable(),
    Command::new(
        "CreateCommandPool",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkCommandPoolCreateInfo"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pCommandPool", "*mut VkCommandPool"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "DestroyCommandPool",
        &[
            ("device", "VkDevice"),
            ("commandPool", "VkCommandPool"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable(),
    Command::new(
        "ResetCommandPool",
        &[
            ("device", "VkDevice"),
            ("commandPool", "VkCommandPool"),
            ("flags", "VkCommandPoolResetFlags"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "AllocateCommandBuffers",
        &[
            ("device", "VkDevice"),
            ("pAllocateInfo", "*const VkCommandBufferAllocateInfo"),
            ("pCommandBuffers", "*mut VkCommandBuffer"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new(
        "FreeCommandBuffers",
        &[
            ("device", "VkDevice"),
            ("commandPool", "VkCommandPool"),
            ("commandBufferCount", "u32"),
            ("pCommandBuffers", "*const VkCommandBuffer"),
        ],
    )
    .static_callable(),
    Command::new(
        "BeginCommandBuffer",
        &[
            ("commandBuffer", "VkCommandBuffer"),
            ("pBeginInfo", "*const VkCommandBufferBeginInfo"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new("EndCommandBuffer", &[("commandBuffer", "VkCommandBuffer")])
        .failable()
        .static_callable(),
    Command::new(
        "ResetCommandBuffer",
        &[
            ("commandBuffer", "VkCommandBuffer"),
            ("flags", "VkCommandBufferResetFlags"),
        ],
    )
    .failable()
    .static_callable(),
    Command::new("EnumerateInstanceVersion", &[("pApiVersion", "*mut u32")])
        .failable()
        .static_callable()
        .version_since("1_1"),
    Command::new(
        "GetPhysicalDeviceDisplayProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pPropertyCount", "*mut u32"),
            ("pProperties", "*mut VkDisplayPropertiesKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension_old("KHR", "display"),
    Command::new(
        "GetPhysicalDeviceDisplayPlaneProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pPropertyCount", "*mut u32"),
            ("pProperties", "*mut VkDisplayPlanePropertiesKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension_old("KHR", "display"),
    Command::new(
        "GetDisplayPlaneSupportedDisplays",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("planeIndex", "u32"),
            ("pDisplayCount", "*mut u32"),
            ("pDisplays", "*mut VkDisplayKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension_old("KHR", "display"),
    Command::new(
        "GetDisplayModeProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("display", "VkDisplayKHR"),
            ("pPropertyCount", "*mut u32"),
            ("pProperties", "*mut VkDisplayModePropertiesKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension_old("KHR", "display"),
    Command::new(
        "CreateDisplayMode",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("display", "VkDisplayKHR"),
            ("pCreateInfo", "*const VkDisplayModeCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pMode", "*mut VkDisplayModeKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension_old("KHR", "display"),
    Command::new(
        "GetDisplayPlaneCapabilities",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("mode", "VkDisplayModeKHR"),
            ("planeIndex", "u32"),
            ("pCapabilities", "*mut VkDisplayPlaneCapabilitiesKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension_old("KHR", "display"),
    Command::new(
        "CreateDisplayPlaneSurface",
        &[
            ("instance", "VkInstance"),
            ("pCreateInfo", "*const VkDisplaySurfaceCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSurface", "*mut VkSurfaceKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension_old("KHR", "display"),
    Command::new(
        "GetPhysicalDeviceSurfaceCapabilities2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pSurfaceInfo", "*const VkPhysicalDeviceSurfaceInfo2KHR"),
            ("pSurfaceCapabilities", "*mut VkSurfaceCapabilities2KHR"),
        ],
    )
    .failable()
    .extension_old("KHR", "get_surface_capabilities2"),
    Command::new(
        "GetPhysicalDeviceSurfaceFormats2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pSurfaceInfo", "*const VkPhysicalDeviceSurfaceInfo2KHR"),
            ("pSurfaceFormatsCount", "*mut u32"),
            ("pSurfaceFormats", "*mut VkSurfaceFormat2KHR"),
        ],
    )
    .failable()
    .extension_old("KHR", "get_surface_capabilities2"),
    Command::new(
        "EnumeratePhysicalDeviceGroup",
        &[
            ("instance", "VkInstance"),
            ("pPhysicalDeviceGroupCount", "*mut u32"),
            (
                "pPhysicalDeviceGroupProperties",
                "*mut VkPhysicalDeviceGroupPropertiesKHR",
            ),
        ],
    )
    .failable()
    .extension_old("KHR", "device_group_creation")
    .promoted("1_1"),
    Command::new(
        "GetDescriptorSetLayoutSupport",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkDescriptorSetLayoutCreateInfo"),
            ("pSupport", "*mut VkDescriptorSetLayoutSupport"),
        ],
    )
    .extension_old("KHR", "maintenance3")
    .promoted("1_1"),
    Command::new(
        "QueueSubmit2",
        &[
            ("queue", "VkQueue"),
            ("submitCount", "u32"),
            ("pSubmits", "*const VkSubmitInfo2KHR"),
            ("fence", "Option<VkFence>"),
        ],
    )
    .failable()
    .extension_old("KHR", "synchronization2")
    .promoted("1_3"),
    // command buffer instructions
    Command::inst(
        "BindPipeline",
        &[("pipelineBindPoint", "VkPipelineBindPoint"), ("pipeline", "VkPipeline")],
    )
    .static_callable(),
    Command::inst(
        "SetViewport",
        &[
            ("firstViewport", "u32"),
            ("viewportCount", "u32"),
            ("pViewports", "*const VkViewport"),
        ],
    )
    .static_callable(),
    Command::inst(
        "SetScissor",
        &[
            ("firstScissor", "u32"),
            ("scissorCount", "u32"),
            ("pScissors", "*const VkRect2D"),
        ],
    )
    .static_callable(),
    Command::inst("SetLineWidth", &[("lineWidth", "core::ffi::c_float")]).static_callable(),
    Command::inst(
        "SetDepthBias",
        &[
            ("depthBiasConstantFactor", "core::ffi::c_float"),
            ("depthBiasClamp", "core::ffi::c_float"),
            ("depthBiasSlopeFactor", "core::ffi::c_float"),
        ],
    )
    .static_callable(),
    Command::inst("SetBlendConstants", &[("blendConstants", "*const core::ffi::c_float")]).static_callable(),
    Command::inst(
        "SetDepthBounds",
        &[
            ("minDepthBounds", "core::ffi::c_float"),
            ("maxDepthBounds", "core::ffi::c_float"),
        ],
    )
    .static_callable(),
    Command::inst(
        "SetStencilCompareMask",
        &[("faceMask", "VkStencilFaceFlags"), ("compareMask", "u32")],
    )
    .static_callable(),
    Command::inst(
        "SetStencilWriteMask",
        &[("faceMask", "VkStencilFaceFlags"), ("writeMask", "u32")],
    )
    .static_callable(),
    Command::inst(
        "SetStencilReference",
        &[("faceMask", "VkStencilFaceFlags"), ("reference", "u32")],
    )
    .static_callable(),
    Command::inst(
        "BindDescriptorSets",
        &[
            ("pipelineBindPoint", "VkPipelineBindPoint"),
            ("layout", "VkPipelineLayout"),
            ("firstSet", "u32"),
            ("descriptorSetCount", "u32"),
            ("pDescriptorSets", "*const VkDescriptorSet"),
            ("dynamicOffsetCount", "u32"),
            ("pDynamicOffsets", "*const u32"),
        ],
    )
    .static_callable(),
    Command::inst(
        "BindIndexBuffer",
        &[
            ("buffer", "VkBuffer"),
            ("offset", "VkDeviceSize"),
            ("indexType", "VkIndexType"),
        ],
    )
    .static_callable(),
    Command::inst(
        "BindVertexBuffers",
        &[
            ("firstBinding", "u32"),
            ("bindingCount", "u32"),
            ("pBuffers", "*const VkBuffer"),
            ("pOffsets", "*const VkDeviceSize"),
        ],
    )
    .static_callable(),
    Command::inst(
        "Draw",
        &[
            ("vertexCount", "u32"),
            ("instanceCount", "u32"),
            ("firstVertex", "u32"),
            ("firstInstance", "u32"),
        ],
    )
    .static_callable(),
    Command::inst(
        "DrawIndexed",
        &[
            ("indexCount", "u32"),
            ("instanceCount", "u32"),
            ("firstIndex", "u32"),
            ("vertexOffset", "i32"),
            ("firstInstance", "u32"),
        ],
    )
    .static_callable(),
    Command::inst(
        "DrawIndirect",
        &[
            ("buffer", "VkBuffer"),
            ("offset", "VkDeviceSize"),
            ("drawCount", "u32"),
            ("stride", "u32"),
        ],
    )
    .static_callable(),
    Command::inst(
        "DrawIndexedIndirect",
        &[
            ("buffer", "VkBuffer"),
            ("offset", "VkDeviceSize"),
            ("drawCount", "u32"),
            ("stride", "u32"),
        ],
    )
    .static_callable(),
    Command::inst(
        "Dispatch",
        &[("groupCountX", "u32"), ("groupCountY", "u32"), ("groupCountZ", "u32")],
    )
    .static_callable(),
    Command::inst(
        "DispatchIndirect",
        &[("buffer", "VkBuffer"), ("offset", "VkDeviceSize")],
    )
    .static_callable(),
    Command::inst(
        "CopyBuffer",
        &[
            ("srcBuffer", "VkBuffer"),
            ("dstBuffer", "VkBuffer"),
            ("regionCount", "u32"),
            ("pRegions", "*const VkBufferCopy"),
        ],
    )
    .static_callable(),
    Command::inst(
        "CopyImage",
        &[
            ("srcImage", "VkImage"),
            ("srcImageLayout", "VkImageLayout"),
            ("dstImage", "VkImage"),
            ("dstImageLayout", "VkImageLayout"),
            ("regionCount", "u32"),
            ("pRegions", "*const VkImageCopy"),
        ],
    )
    .static_callable(),
    Command::inst(
        "BlitImage",
        &[
            ("srcImage", "VkImage"),
            ("srcImageLayout", "VkImageLayout"),
            ("dstImage", "VkImage"),
            ("dstImageLayout", "VkImageLayout"),
            ("regionCount", "u32"),
            ("pRegions", "*const VkImageBlit"),
            ("filters", "VkFilter"),
        ],
    )
    .static_callable(),
    Command::inst(
        "CopyBufferToImage",
        &[
            ("srcBuffer", "VkBuffer"),
            ("dstImage", "VkImage"),
            ("dstImageLayout", "VkImageLayout"),
            ("regionCount", "u32"),
            ("pRegions", "*const VkBufferImageCopy"),
        ],
    )
    .static_callable(),
    Command::inst(
        "CopyImageToBuffer",
        &[
            ("srcImage", "VkImage"),
            ("srcImageLayout", "VkImageLayout"),
            ("dstBuffer", "VkBuffer"),
            ("regionCount", "u32"),
            ("pRegions", "*const VkBufferImageCopy"),
        ],
    )
    .static_callable(),
    Command::inst(
        "UpdateBuffer",
        &[
            ("dstBuffer", "VkBuffer"),
            ("dstOffset", "VkDeviceSize"),
            ("dataSize", "VkDeviceSize"),
            ("pData", "*const core::ffi::c_void"),
        ],
    )
    .static_callable(),
    Command::inst(
        "FillBuffer",
        &[
            ("dstBuffer", "VkBuffer"),
            ("dstOffset", "VkDeviceSize"),
            ("dataSize", "VkDeviceSize"),
            ("data", "u32"),
        ],
    )
    .static_callable(),
    Command::inst(
        "ClearColorImage",
        &[
            ("image", "VkImage"),
            ("imageLayout", "VkImageLayout"),
            ("pColor", "*const VkClearColorValue"),
            ("rangeCount", "u32"),
            ("pRanges", "*const VkImageSubresourceRange"),
        ],
    )
    .static_callable(),
    Command::inst(
        "ClearDepthStencilImage",
        &[
            ("image", "VkImage"),
            ("imageLayout", "VkImageLayout"),
            ("pDepthStencil", "*const VkClearDepthStencilValue"),
            ("rangeCount", "u32"),
            ("pRanges", "*const VkImageSubresourceRange"),
        ],
    )
    .static_callable(),
    Command::inst(
        "ClearAttachments",
        &[
            ("attachmentCount", "u32"),
            ("pAttachments", "*const VkClearAttachment"),
            ("rectCount", "u32"),
            ("pRects", "*const VkClearRect"),
        ],
    )
    .static_callable(),
    Command::inst(
        "ResolveImage",
        &[
            ("srcImage", "VkImage"),
            ("srcImageLayout", "VkImageLayout"),
            ("dstImage", "VkImage"),
            ("dstImageLayout", "VkImageLayout"),
            ("regionCount", "u32"),
            ("pRegions", "*const VkImageResolve"),
        ],
    )
    .static_callable(),
    Command::inst(
        "SetEvent",
        &[("event", "VkEvent"), ("stageMask", "VkPipelineStageFlags")],
    )
    .static_callable(),
    Command::inst(
        "ResetEvent",
        &[("event", "VkEvent"), ("stageMask", "VkPipelineStageFlags")],
    )
    .static_callable(),
    Command::inst(
        "WaitEvents",
        &[
            ("eventCount", "u32"),
            ("pEvents", "*const VkEvent"),
            ("srcStageMask", "VkPipelineStageFlags"),
            ("dstStageMask", "VkPipelineStageFlags"),
            ("memoryBarrierCount", "u32"),
            ("pMemoryBarriers", "*const VkMemoryBarrier"),
            ("bufferMemoryBarrierCount", "u32"),
            ("pBufferMemoryBarriers", "*const VkBufferMemoryBarrier"),
            ("imageMemoryBarrierCount", "u32"),
            ("pImageMemoryBarriers", "*const VkImageMemoryBarrier"),
        ],
    )
    .static_callable(),
    Command::inst(
        "PipelineBarrier",
        &[
            ("srcStageMask", "VkPipelineStageFlags"),
            ("dstStageMask", "VkPipelineStageFlags"),
            ("dependencyFlags", "VkDependencyFlags"),
            ("memoryBarrierCount", "u32"),
            ("pMemoryBarriers", "*const VkMemoryBarrier"),
            ("bufferMemoryBarrierCount", "u32"),
            ("pBufferMemoryBarriers", "*const VkBufferMemoryBarrier"),
            ("imageMemoryBarrierCount", "u32"),
            ("pImageMemoryBarriers", "*const VkImageMemoryBarrier"),
        ],
    )
    .static_callable(),
    Command::inst(
        "BeginQuery",
        &[
            ("queryPool", "VkQueryPool"),
            ("query", "u32"),
            ("flags", "VkQueryControlFlags"),
        ],
    )
    .static_callable(),
    Command::inst("EndQuery", &[("queryPool", "VkQueryPool"), ("query", "u32")]).static_callable(),
    Command::inst(
        "ResetQueryPool",
        &[
            ("queryPool", "VkQueryPool"),
            ("firstQuery", "u32"),
            ("queryCount", "u32"),
        ],
    )
    .static_callable(),
    Command::inst(
        "WriteTimestamp",
        &[
            ("pipelineStage", "VkPipelineStageFlags"),
            ("queryPool", "VkQueryPool"),
            ("query", "u32"),
        ],
    )
    .static_callable(),
    Command::inst(
        "CopyQueryPoolResults",
        &[
            ("queryPool", "VkQueryPool"),
            ("firstQuery", "u32"),
            ("queryCount", "u32"),
            ("dstBuffer", "VkBuffer"),
            ("dstOffset", "VkDeviceSize"),
            ("stride", "VkDeviceSize"),
            ("flags", "VkQueryResultFlags"),
        ],
    )
    .static_callable(),
    Command::inst(
        "PushConstants",
        &[
            ("pipelineLayout", "VkPipelineLayout"),
            ("stageFlags", "VkShaderStageFlags"),
            ("offset", "u32"),
            ("size", "u32"),
            ("pValues", "*const core::ffi::c_void"),
        ],
    )
    .static_callable(),
    Command::inst(
        "BeginRenderPass",
        &[
            ("pRenderPassBegin", "*const VkRenderPassBeginInfo"),
            ("contents", "VkSubpassContents"),
        ],
    )
    .static_callable(),
    Command::inst("NextSubpass", &[("contents", "VkSubpassContents")]).static_callable(),
    Command::inst("EndRenderPass", &[]).static_callable(),
    Command::inst(
        "ExecuteCommands",
        &[
            ("commandBufferCount", "u32"),
            ("pCommandBuffers", "*const VkCommandBuffer"),
        ],
    )
    .static_callable(),
    Command::inst(
        "SetEvent2",
        &[("event", "VkEvent"), ("pDependencyInfo", "*const VkDependencyInfoKHR")],
    )
    .extension_old("KHR", "synchronization2")
    .promoted("1_3"),
    Command::inst(
        "ResetEvent2",
        &[("event", "VkEvent"), ("stageMask", "VkPipelineStageFlags2KHR")],
    )
    .extension_old("KHR", "synchronization2")
    .promoted("1_3"),
    Command::inst(
        "WaitEvents2",
        &[
            ("eventCount", "u32"),
            ("pEvents", "*const VkEvent"),
            ("pDependencyInfos", "*const VkDependencyInfoKHR"),
        ],
    )
    .extension_old("KHR", "synchronization2")
    .promoted("1_3"),
    Command::inst("PipelineBarrier2", &[("pDependencyInfo", "*const VkDependencyInfoKHR")])
        .extension_old("KHR", "synchronization2")
        .promoted("1_3"),
    Command::inst(
        "WriteTimestamp2",
        &[
            ("stage", "VkPipelineStageFlags2KHR"),
            ("queryPool", "VkQueryPool"),
            ("query", "u32"),
        ],
    )
    .extension_old("KHR", "synchronization2")
    .promoted("1_3"),
];

pub static RESULT: EnumType = EnumType::new("Result", "");
pub static ERROR: EnumType = EnumType::new("Result", "ERROR");
fn emit_result_type(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    RESULT.emit(emitter);

    // base defines
    RESULT.member("SUCCESS", 0).emit(emitter);
    RESULT.member("NOT_READY", 1).emit(emitter);
    RESULT.member("TIMEOUT", 2).emit(emitter);
    RESULT.member("EVENT_SET", 3).emit(emitter);
    RESULT.member("EVENT_RESET", 4).emit(emitter);
    RESULT.member("INCOMPLETE", 5).emit(emitter);
    ERROR.member("OUT_OF_HOST_MEMORY", 1).neg().emit(emitter);
    ERROR.member("OUT_OF_DEVICE_MEMORY", 2).neg().emit(emitter);
    ERROR.member("INITIALIZATION_FAILED", 3).neg().emit(emitter);
    ERROR.member("DEVICE_LOST", 4).neg().emit(emitter);
    ERROR.member("MEMORY_MAP_FAILED", 5).neg().emit(emitter);
    ERROR.member("LAYER_NOT_PRESENT", 6).neg().emit(emitter);
    ERROR.member("EXTENSION_NOT_PRESENT", 7).neg().emit(emitter);
    ERROR.member("FEATURE_NOT_PRESENT", 8).neg().emit(emitter);
    ERROR.member("INCOMPATIBLE_DRIVER", 9).neg().emit(emitter);
    ERROR.member("TOO_MANY_OBJECTS", 10).neg().emit(emitter);
    ERROR.member("FORMAT_NOT_SUPPORTED", 11).neg().emit(emitter);
    ERROR.member("FRAGMENTED_POOL", 12).neg().emit(emitter);
    ERROR.member("UNKNOWN", 13).neg().emit(emitter);

    // from extensions
    RESULT
        .member("SUBOPTIMAL", 3)
        .extension(&VK_KHR_SWAPCHAIN)
        .emit(emitter);
    ERROR
        .member("OUT_OF_DATE", 4)
        .neg()
        .extension(&VK_KHR_SWAPCHAIN)
        .emit(emitter);
    ERROR
        .member("INCOMPATIBLE_DISPLAY", 1)
        .neg()
        .extension(VK_KHR_DISPLAY_SWAPCHAIN)
        .emit(emitter);

    // from promoted extensions
    ERROR
        .member("INVALID_EXTERNAL_HANDLE", 3)
        .neg()
        .extension(VK_KHR_EXTERNAL_MEMORY)
        .emit(emitter);
    ERROR
        .member("OUT_OF_POOL_MEMORY", 0)
        .neg()
        .extension(VK_KHR_MAINTENANCE_1)
        .emit(emitter);
}

pub static FORMAT: EnumType = EnumType::new("Format", "FORMAT");
fn emit_format_enum(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    FORMAT.emit(emitter);
    FORMAT.member("UNDEFINED", 0).emit(emitter);

    // packed format
    FORMAT.member("R4G4_UNORM_PACK8", 1).emit(emitter);
    FORMAT.member("R4G4B4A4_UNORM_PACK16", 2).emit(emitter);
    FORMAT.member("B4G4R4A4_UNORM_PACK16", 3).emit(emitter);
    FORMAT.member("R5G6B5_UNORM_PACK16", 4).emit(emitter);
    FORMAT.member("B5G6R5_UNORM_PACK16", 5).emit(emitter);
    FORMAT.member("R5G5B5A1_UNORM_PACK16", 6).emit(emitter);
    FORMAT.member("B5G5R5A1_UNORM_PACK16", 7).emit(emitter);
    FORMAT.member("A1R5G5B5_UNORM_PACK16", 8).emit(emitter);

    // straight format
    FORMAT.member("R8_UNORM", 9).emit(emitter);
    FORMAT.member("R8_SNORM", 10).emit(emitter);
    FORMAT.member("R8_USCALED", 11).emit(emitter);
    FORMAT.member("R8_SSCALED", 12).emit(emitter);
    FORMAT.member("R8_UINT", 13).emit(emitter);
    FORMAT.member("R8_SINT", 14).emit(emitter);
    FORMAT.member("R8_SRGB", 15).emit(emitter);
    FORMAT.member("R8G8_UNORM", 16).emit(emitter);
    FORMAT.member("R8G8_SNORM", 17).emit(emitter);
    FORMAT.member("R8G8_USCALED", 18).emit(emitter);
    FORMAT.member("R8G8_SSCALED", 19).emit(emitter);
    FORMAT.member("R8G8_UINT", 20).emit(emitter);
    FORMAT.member("R8G8_SINT", 21).emit(emitter);
    FORMAT.member("R8G8_SRGB", 22).emit(emitter);
    FORMAT.member("R8G8B8_UNORM", 23).emit(emitter);
    FORMAT.member("R8G8B8_SNORM", 24).emit(emitter);
    FORMAT.member("R8G8B8_USCALED", 25).emit(emitter);
    FORMAT.member("R8G8B8_SSCALED", 26).emit(emitter);
    FORMAT.member("R8G8B8_UINT", 27).emit(emitter);
    FORMAT.member("R8G8B8_SINT", 28).emit(emitter);
    FORMAT.member("R8G8B8_SRGB", 29).emit(emitter);
    FORMAT.member("B8G8R8_UNORM", 30).emit(emitter);
    FORMAT.member("B8G8R8_SNORM", 31).emit(emitter);
    FORMAT.member("B8G8R8_USCALED", 32).emit(emitter);
    FORMAT.member("B8G8R8_SSCALED", 33).emit(emitter);
    FORMAT.member("B8G8R8_UINT", 34).emit(emitter);
    FORMAT.member("B8G8R8_SINT", 35).emit(emitter);
    FORMAT.member("B8G8R8_SRGB", 36).emit(emitter);
    FORMAT.member("R8G8B8A8_UNORM", 37).emit(emitter);
    FORMAT.member("R8G8B8A8_SNORM", 38).emit(emitter);
    FORMAT.member("R8G8B8A8_USCALED", 39).emit(emitter);
    FORMAT.member("R8G8B8A8_SSCALED", 40).emit(emitter);
    FORMAT.member("R8G8B8A8_UINT", 41).emit(emitter);
    FORMAT.member("R8G8B8A8_SINT", 42).emit(emitter);
    FORMAT.member("R8G8B8A8_SRGB", 43).emit(emitter);
    FORMAT.member("B8G8R8A8_UNORM", 44).emit(emitter);
    FORMAT.member("B8G8R8A8_SNORM", 45).emit(emitter);
    FORMAT.member("B8G8R8A8_USCALED", 46).emit(emitter);
    FORMAT.member("B8G8R8A8_SSCALED", 47).emit(emitter);
    FORMAT.member("B8G8R8A8_UINT", 48).emit(emitter);
    FORMAT.member("B8G8R8A8_SINT", 49).emit(emitter);
    FORMAT.member("B8G8R8A8_SRGB", 50).emit(emitter);
    FORMAT.member("A8B8G8R8_UNORM_PACK32", 51).emit(emitter);
    FORMAT.member("A8B8G8R8_SNORM_PACK32", 52).emit(emitter);
    FORMAT.member("A8B8G8R8_USCALED_PACK32", 53).emit(emitter);
    FORMAT.member("A8B8G8R8_SSCALED_PACK32", 54).emit(emitter);
    FORMAT.member("A8B8G8R8_UINT_PACK32", 55).emit(emitter);
    FORMAT.member("A8B8G8R8_SINT_PACK32", 56).emit(emitter);
    FORMAT.member("A8B8G8R8_SRGB_PACK32", 57).emit(emitter);
    FORMAT.member("A2R10G10B10_UNORM_PACK32", 58).emit(emitter);
    FORMAT.member("A2R10G10B10_SNORM_PACK32", 59).emit(emitter);
    FORMAT.member("A2R10G10B10_USCALED_PACK32", 60).emit(emitter);
    FORMAT.member("A2R10G10B10_SSCALED_PACK32", 61).emit(emitter);
    FORMAT.member("A2R10G10B10_UINT_PACK32", 62).emit(emitter);
    FORMAT.member("A2R10G10B10_SINT_PACK32", 63).emit(emitter);
    FORMAT.member("A2B10G10R10_UNORM_PACK32", 64).emit(emitter);
    FORMAT.member("A2B10G10R10_SNORM_PACK32", 65).emit(emitter);
    FORMAT.member("A2B10G10R10_USCALED_PACK32", 66).emit(emitter);
    FORMAT.member("A2B10G10R10_SSCALED_PACK32", 67).emit(emitter);
    FORMAT.member("A2B10G10R10_UINT_PACK32", 68).emit(emitter);
    FORMAT.member("A2B10G10R10_SINT_PACK32", 69).emit(emitter);
    FORMAT.member("R16_UNORM", 70).emit(emitter);
    FORMAT.member("R16_SNORM", 71).emit(emitter);
    FORMAT.member("R16_USCALED", 72).emit(emitter);
    FORMAT.member("R16_SSCALED", 73).emit(emitter);
    FORMAT.member("R16_UINT", 74).emit(emitter);
    FORMAT.member("R16_SINT", 75).emit(emitter);
    FORMAT.member("R16_SFLOAT", 76).emit(emitter);
    FORMAT.member("R16G16_UNORM", 77).emit(emitter);
    FORMAT.member("R16G16_SNORM", 78).emit(emitter);
    FORMAT.member("R16G16_USCALED", 79).emit(emitter);
    FORMAT.member("R16G16_SSCALED", 80).emit(emitter);
    FORMAT.member("R16G16_UINT", 81).emit(emitter);
    FORMAT.member("R16G16_SINT", 82).emit(emitter);
    FORMAT.member("R16G16_SFLOAT", 83).emit(emitter);
    FORMAT.member("R16G16B16_UNORM", 84).emit(emitter);
    FORMAT.member("R16G16B16_SNORM", 85).emit(emitter);
    FORMAT.member("R16G16B16_USCALED", 86).emit(emitter);
    FORMAT.member("R16G16B16_SSCALED", 87).emit(emitter);
    FORMAT.member("R16G16B16_UINT", 88).emit(emitter);
    FORMAT.member("R16G16B16_SINT", 89).emit(emitter);
    FORMAT.member("R16G16B16_SFLOAT", 90).emit(emitter);
    FORMAT.member("R16G16B16A16_UNORM", 91).emit(emitter);
    FORMAT.member("R16G16B16A16_SNORM", 92).emit(emitter);
    FORMAT.member("R16G16B16A16_USCALED", 93).emit(emitter);
    FORMAT.member("R16G16B16A16_SSCALED", 94).emit(emitter);
    FORMAT.member("R16G16B16A16_UINT", 95).emit(emitter);
    FORMAT.member("R16G16B16A16_SINT", 96).emit(emitter);
    FORMAT.member("R16G16B16A16_SFLOAT", 97).emit(emitter);
    FORMAT.member("R32_UINT", 98).emit(emitter);
    FORMAT.member("R32_SINT", 99).emit(emitter);
    FORMAT.member("R32_SFLOAT", 100).emit(emitter);
    FORMAT.member("R32G32_UINT", 101).emit(emitter);
    FORMAT.member("R32G32_SINT", 102).emit(emitter);
    FORMAT.member("R32G32_SFLOAT", 103).emit(emitter);
    FORMAT.member("R32G32B32_UINT", 104).emit(emitter);
    FORMAT.member("R32G32B32_SINT", 105).emit(emitter);
    FORMAT.member("R32G32B32_SFLOAT", 106).emit(emitter);
    FORMAT.member("R32G32B32A32_UINT", 107).emit(emitter);
    FORMAT.member("R32G32B32A32_SINT", 108).emit(emitter);
    FORMAT.member("R32G32B32A32_SFLOAT", 109).emit(emitter);
    FORMAT.member("R64_UINT", 110).emit(emitter);
    FORMAT.member("R64_SINT", 111).emit(emitter);
    FORMAT.member("R64_SFLOAT", 112).emit(emitter);
    FORMAT.member("R64G64_UINT", 113).emit(emitter);
    FORMAT.member("R64G64_SINT", 114).emit(emitter);
    FORMAT.member("R64G64_SFLOAT", 115).emit(emitter);
    FORMAT.member("R64G64B64_UINT", 116).emit(emitter);
    FORMAT.member("R64G64B64_SINT", 117).emit(emitter);
    FORMAT.member("R64G64B64_SFLOAT", 118).emit(emitter);
    FORMAT.member("R64G64B64A64_UINT", 119).emit(emitter);
    FORMAT.member("R64G64B64A64_SINT", 120).emit(emitter);
    FORMAT.member("R64G64B64A64_SFLOAT", 121).emit(emitter);
    FORMAT.member("B10G11R11_UFLOAT_PACK32", 122).emit(emitter);
    FORMAT.member("E5B9G9R9_UFLOAT_PACK32", 123).emit(emitter);
    FORMAT.member("D16_UNORM", 124).emit(emitter);
    FORMAT.member("X8_D24_UNORM_PACK32", 125).emit(emitter);
    FORMAT.member("D32_SFLOAT", 126).emit(emitter);
    FORMAT.member("S8_UINT", 127).emit(emitter);
    FORMAT.member("D16_UNORM_S8_UINT", 128).emit(emitter);
    FORMAT.member("D24_UNORM_S8_UINT", 129).emit(emitter);
    FORMAT.member("D32_SFLOAT_S8_UINT", 130).emit(emitter);
    FORMAT
        .member("A1B5G5R4_UNORM_PACK16", 0)
        .extension(VK_KHR_MAINTENANCE_5)
        .emit(emitter);
    FORMAT
        .member("A8_UNORM", 1)
        .extension(VK_KHR_MAINTENANCE_5)
        .emit(emitter);

    // compressed formats
    FORMAT.member("BC1_RGB_UNORM_BLOCK", 131).emit(emitter);
    FORMAT.member("BC1_RGB_SRGB_BLOCK", 132).emit(emitter);
    FORMAT.member("BC1_RGBA_UNORM_BLOCK", 133).emit(emitter);
    FORMAT.member("BC1_RGBA_SRGB_BLOCK", 134).emit(emitter);
    FORMAT.member("BC2_UNORM_BLOCK", 135).emit(emitter);
    FORMAT.member("BC2_SRGB_BLOCK", 136).emit(emitter);
    FORMAT.member("BC3_UNORM_BLOCK", 137).emit(emitter);
    FORMAT.member("BC3_SRGB_BLOCK", 138).emit(emitter);
    FORMAT.member("BC4_UNORM_BLOCK", 139).emit(emitter);
    FORMAT.member("BC4_SNORM_BLOCK", 140).emit(emitter);
    FORMAT.member("BC5_UNORM_BLOCK", 141).emit(emitter);
    FORMAT.member("BC5_SNORM_BLOCK", 142).emit(emitter);
    FORMAT.member("BC6H_UFLOAT_BLOCK", 143).emit(emitter);
    FORMAT.member("BC6H_SFLOAT_BLOCK", 144).emit(emitter);
    FORMAT.member("BC7_UNORM_BLOCK", 145).emit(emitter);
    FORMAT.member("BC7_SRGB_BLOCK", 146).emit(emitter);

    FORMAT.member("ETC2_R8G8B8_UNORM_BLOCK", 147).emit(emitter);
    FORMAT.member("ETC2_R8G8B8_SRGB_BLOCK", 148).emit(emitter);
    FORMAT.member("ETC2_R8G8B8A1_UNORM_BLOCK", 149).emit(emitter);
    FORMAT.member("ETC2_R8G8B8A1_SRGB_BLOCK", 150).emit(emitter);
    FORMAT.member("ETC2_R8G8B8A8_UNORM_BLOCK", 151).emit(emitter);
    FORMAT.member("ETC2_R8G8B8A8_SRGB_BLOCK", 152).emit(emitter);

    FORMAT.member("EAC_R11_UNORM_BLOCK", 153).emit(emitter);
    FORMAT.member("EAC_R11_SNORM_BLOCK", 154).emit(emitter);
    FORMAT.member("EAC_R11G11_UNORM_BLOCK", 155).emit(emitter);
    FORMAT.member("EAC_R11G11_SNORM_BLOCK", 156).emit(emitter);

    FORMAT.member("ASTC_4x4_UNORM_BLOCK", 157).emit(emitter);
    FORMAT.member("ASTC_4x4_SRGB_BLOCK", 158).emit(emitter);
    FORMAT.member("ASTC_5x4_UNORM_BLOCK", 159).emit(emitter);
    FORMAT.member("ASTC_5x4_SRGB_BLOCK", 160).emit(emitter);
    FORMAT.member("ASTC_5x5_UNORM_BLOCK", 161).emit(emitter);
    FORMAT.member("ASTC_5x5_SRGB_BLOCK", 162).emit(emitter);
    FORMAT.member("ASTC_6x5_UNORM_BLOCK", 163).emit(emitter);
    FORMAT.member("ASTC_6x5_SRGB_BLOCK", 164).emit(emitter);
    FORMAT.member("ASTC_6x6_UNORM_BLOCK", 165).emit(emitter);
    FORMAT.member("ASTC_6x6_SRGB_BLOCK", 166).emit(emitter);
    FORMAT.member("ASTC_8x5_UNORM_BLOCK", 167).emit(emitter);
    FORMAT.member("ASTC_8x5_SRGB_BLOCK", 168).emit(emitter);
    FORMAT.member("ASTC_8x6_UNORM_BLOCK", 169).emit(emitter);
    FORMAT.member("ASTC_8x6_SRGB_BLOCK", 170).emit(emitter);
    FORMAT.member("ASTC_8x8_UNORM_BLOCK", 171).emit(emitter);
    FORMAT.member("ASTC_8x8_SRGB_BLOCK", 172).emit(emitter);
    FORMAT.member("ASTC_10x5_UNORM_BLOCK", 173).emit(emitter);
    FORMAT.member("ASTC_10x5_SRGB_BLOCK", 174).emit(emitter);
    FORMAT.member("ASTC_10x6_UNORM_BLOCK", 175).emit(emitter);
    FORMAT.member("ASTC_10x6_SRGB_BLOCK", 176).emit(emitter);
    FORMAT.member("ASTC_10x8_UNORM_BLOCK", 177).emit(emitter);
    FORMAT.member("ASTC_10x8_SRGB_BLOCK", 178).emit(emitter);
    FORMAT.member("ASTC_10x10_UNORM_BLOCK", 179).emit(emitter);
    FORMAT.member("ASTC_10x10_SRGB_BLOCK", 180).emit(emitter);
    FORMAT.member("ASTC_12x10_UNORM_BLOCK", 181).emit(emitter);
    FORMAT.member("ASTC_12x10_SRGB_BLOCK", 182).emit(emitter);
    FORMAT.member("ASTC_12x12_UNORM_BLOCK", 183).emit(emitter);
    FORMAT.member("ASTC_12x12_SRGB_BLOCK", 184).emit(emitter);

    // sampler_ycbcr_conversion
    FORMAT
        .member("G8B8G8R8_422_UNORM", 0)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("B8G8R8G8_422_UNORM", 1)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G8_B8_R8_3PLANE_420_UNORM", 2)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G8_B8R8_2PLANE_420_UNORM", 3)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G8_B8_R8_3PLANE_422_UNORM", 4)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G8_B8R8_2PLANE_422_UNORM", 5)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G8_B8_R8_3PLANE_444_UNORM", 6)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R10X6_UNORM_PACK16", 7)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R10X6G10X6_UNORM_2PACK16", 8)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R10X6G10X6B10X6A10X6_UNORM_4PACK16", 9)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6B10X6G10X6R10X6_422_UNORM_4PACK16", 10)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("B10X6G10X6R10X6G10X6_422_UNORM_4PACK16", 11)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16", 12)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16", 13)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16", 14)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16", 15)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16", 16)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R12X4_UNORM_PACK16", 17)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R12X4G12X4_UNORM_2PACK16", 18)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R12X4G12X4B12X4A12X4_UNORM_4PACK16", 19)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4B12X4G12X4R12X4_422_UNORM_4PACK16", 20)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("B12X4G12X4R12X4G12X4_422_UNORM_4PACK16", 21)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16", 22)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16", 23)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16", 24)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16", 25)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16", 26)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16B16G16R16_422_UNORM", 27)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("B16G16R16G16_422_UNORM", 28)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16_B16_R16_3PLANE_420_UNORM", 29)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16_B16R16_2PLANE_420_UNORM", 30)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16_B16_R16_3PLANE_422_UNORM", 31)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16_B16R16_2PLANE_422_UNORM", 32)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16_B16_R16_3PLANE_444_UNORM", 33)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
}
