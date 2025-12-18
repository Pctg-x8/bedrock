use std::io::Write;

use parts::{
    Bitmask, Command, Enum, ExtensionHeaderConstants, FuncPointer, Object, Struct, StructUsage, TypeAlias, Union,
    emit_c_enum_type, emit_const, emit_result_const, emit_result_err_const,
};

mod parts;
mod v1_2;

fn main() -> std::io::Result<()> {
    let mut o = std::io::stdout().lock();
    o.write(HEADER.replace("**VER**", "1.4.305").as_bytes())?;

    o.write(b"\n")?;
    writeln!(o, "/// Vulkan 1.0 version number")?;
    writeln!(o, "pub const VK_API_VERSION_1_0: u32 = VK_MAKE_VERSION(0, 1, 0, 0);")?;
    o.write(b"\n")?;
    writeln!(o, "/// Version of this file")?;
    writeln!(o, "pub const VK_HEADER_VERSION: u16 = {};", 305)?;
    writeln!(
        o,
        "pub const VK_HEADER_VERSION_COMPLETE: u32 = VK_MAKE_VERSION(0, 1, {}, VK_HEADER_VERSION);",
        4
    )?;

    o.write(b"\n")?;

    for ta in TYPE_ALIASES {
        ta.emit(&mut o)?;
    }

    for f in FLAGS {
        o.write(b"\n")?;
        f.emit(&mut o)?;
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
    o.write(b"#[cfg(feature = \"VK_KHR_synchronization2\")]\n")?;
    emit_const(
        &mut o,
        "VK_PIPELINE_STAGE_2_NONE_KHR",
        "VkPipelineStageFlagBits2KHR",
        "0",
    )?;
    o.write(b"#[cfg(feature = \"Allow1_3APIs\")]\n")?;
    emit_const(&mut o, "VK_PIPELINE_STAGE_2_NONE", "VkPipelineStageFlagBits2", "0")?;
    o.write(b"#[cfg(feature = \"VK_KHR_synchronization2\")]\n")?;
    emit_const(&mut o, "VK_ACCESS_2_NONE_KHR", "VkAccessFlagBits2KHR", "0")?;
    o.write(b"#[cfg(feature = \"Allow1_3APIs\")]\n")?;
    emit_const(&mut o, "VK_ACCESS_2_NONE", "VkAccessFlagBits2", "0")?;

    for obj in OBJECTS {
        o.write(b"\n")?;
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

    o.write(b"#[cfg(feature = \"VK_KHR_external_memory\")]\n")?;
    emit_const(&mut o, "VK_QUEUE_FAMILY_EXTERNAL_KHR", "u32", "!1")?;
    o.write(b"#[cfg(feature = \"Allow1_1APIs\")]")?;
    emit_const(&mut o, "VK_QUEUE_FAMILY_EXTERNAL", "u32", "!1")?;

    o.write(b"#[cfg(feature = \"VK_KHR_device_group_creation\")]\n")?;
    emit_const(&mut o, "VK_MAX_DEVICE_GROUP_SIZE_KHR", "usize", "32")?;
    o.write(b"#[cfg(feature = \"Allow1_1APIs\")]\n")?;
    emit_const(&mut o, "VK_MAX_DEVICE_GROUP_SIZE", "usize", "32")?;

    o.write(b"\n")?;

    emit_result_type(&mut o)?;

    o.write(b"\n")?;
    emit_c_enum_type(&mut o, "VkStructureType")?;
    emit_c_enum_type(&mut o, "VkObjectType")?;

    for e in ENUMS {
        o.write(b"\n")?;
        e.emit(&mut o)?;
    }

    o.write(b"\n")?;
    emit_format_enum(&mut o)?;

    for f in FUNC_POINTERS {
        o.write(b"\n")?;
        f.emit(&mut o)?;
    }

    for s in STRUCTS {
        o.write(b"\n")?;
        s.emit(&mut o)?;
    }

    // chaotic requirement structure
    Struct::new(
        "PhysicalDeviceIDPropertiesKHR",
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
    .stype(
        "PHYSICAL_DEVICE_ID_PROPERTIES_KHR",
        vk_ext_enum(72, 4) as _,
        StructUsage::Sink,
    )
    .emit_extra_cfg(&mut o, "any(feature = \"VK_KHR_external_fence_capabilities\", feature = \"VK_KHR_external_memory_capabilities\", feature = \"VK_KHR_external_semaphore_capabilities\")")?;
    o.write(b"#[cfg(feature = \"Allow1_1APIs\")]\n")?;
    o.write(b"#[rustfmt::skip]\n")?;
    o.write(b"pub type VkPhysicalDeviceIDProperties = VkPhysicalDeviceIDPropertiesKHR;\n")?;
    o.write(b"#[cfg(feature = \"Allow1_1APIs\")]\n")?;
    o.write(b"#[rustfmt::skip]\n")?;
    o.write(b"pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR;\n")?;

    // struct aliasing
    o.write(b"#[cfg(feature = \"VK_KHR_variable_pointers\")]\n")?;
    o.write(b"#[rustfmt::skip]\n")?;
    o.write(b"pub type VkPhysicalDeviceVariablePointerFeaturesKHR = VkPhysicalDeviceVariablePointersFeaturesKHR;\n")?;
    o.write(b"#[cfg(feature = \"VK_KHR_variable_pointers\")]\n")?;
    o.write(b"#[rustfmt::skip]\n")?;
    o.write(b"pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;\n")?;

    for s in UNIONS {
        o.write(b"\n")?;
        s.emit(&mut o)?;
    }

    for c in COMMANDS {
        o.write(b"\n")?;
        c.emit(&mut o)?;
    }

    o.write(b"\n")?;
    o.write(b"#[cfg(all(feature = \"Implements\", not(feature = \"DynamicLoaded\")))]\n")?;
    o.write(b"#[cfg_attr(all(not(windows), not(target_os = \"macos\"), not(feature = \"DynamicLoaded\")), link(name = \"vulkan\"))]\n")?;
    o.write(b"#[cfg_attr(all(windows, not(feature = \"DynamicLoaded\"), feature = \"Implements\"), link(name = \"vulkan-1\"))]\n")?;
    o.write(b"#[rustfmt::skip]\n")?;
    o.write(b"unsafe extern \"system\" {\n")?;
    for c in COMMANDS {
        c.emit_static_symbol(&mut o)?;
    }
    o.write(b"}\n")?;

    for c in EXTENSION_HEADER_CONSTANTS {
        o.write(b"\n")?;
        c.emit(&mut o)?;
    }

    for x in v1_2::ELEMENTS {
        o.write(b"\n")?;
        x.emit(&mut o)?;
    }

    Ok(())
}

const fn vk_ext_enum(extnumber: i32, offset: i32) -> i32 {
    100_0000_000 + (extnumber - 1) * 1000 + offset
}

const HEADER: &'static str = include_str!("../res/common_header.rs");

const DEVICE_SIZE_TYPE: &'static str = "VkDeviceSize";
const DEVICE_ADDR_TYPE: &'static str = "VkDeviceAddress";

const EXTENSION_HEADER_CONSTANTS: &'static [ExtensionHeaderConstants] = &[
    ExtensionHeaderConstants::new("VK_KHR_surface", 25),
    ExtensionHeaderConstants::new("VK_KHR_swapchain", 68),
    ExtensionHeaderConstants::new("VK_KHR_display", 21),
    ExtensionHeaderConstants::new("VK_KHR_display_swapchain", 1),
    ExtensionHeaderConstants::new("VK_KHR_xlib_surface", 6),
    ExtensionHeaderConstants::new("VK_KHR_xcb_surface", 6),
    ExtensionHeaderConstants::new("VK_KHR_wayland_surface", 6),
    ExtensionHeaderConstants::new("VK_KHR_android_surface", 6),
    ExtensionHeaderConstants::new("VK_KHR_win32_surface", 6),
    ExtensionHeaderConstants::new("VK_EXT_metal_surface", 1),
    ExtensionHeaderConstants::new("VK_KHR_sampler_mirror_clamp_to_edge", 1),
    ExtensionHeaderConstants::new("VK_KHR_win32_keyed_mutex", 1),
    ExtensionHeaderConstants::new("VK_KHR_external_memory", 1),
    ExtensionHeaderConstants::new("VK_KHR_external_memory_capabilities", 1),
    ExtensionHeaderConstants::new("VK_KHR_external_memory_win32", 1),
    ExtensionHeaderConstants::new("VK_KHR_external_memory_fd", 1),
    ExtensionHeaderConstants::new("VK_KHR_external_semaphore", 1),
    ExtensionHeaderConstants::new("VK_KHR_external_semaphore_win32", 1),
    ExtensionHeaderConstants::new("VK_KHR_external_semaphore_fd", 1),
    ExtensionHeaderConstants::new("VK_KHR_external_fence_win32", 1),
    ExtensionHeaderConstants::new("VK_KHR_external_fence_fd", 1),
    ExtensionHeaderConstants::new("VK_KHR_get_surface_capabilities2", 1),
    ExtensionHeaderConstants::new("VK_EXT_debug_report", 8),
    ExtensionHeaderConstants::new("VK_EXT_debug_utils", 2),
    ExtensionHeaderConstants::new("VK_KHR_device_group", 4),
    ExtensionHeaderConstants::new("VK_KHR_device_group_creation", 1),
    ExtensionHeaderConstants::new("VK_KHR_multiview", 1),
    ExtensionHeaderConstants::new("VK_KHR_get_physical_device_properties2", 1),
    ExtensionHeaderConstants::new("VK_KHR_shader_draw_parameters", 1),
    ExtensionHeaderConstants::new("VK_KHR_portability_enumeration", 1),
    ExtensionHeaderConstants::new("VK_KHR_get_memory_requirements2", 1),
    ExtensionHeaderConstants::new("VK_KHR_relaxed_block_layout", 1),
    ExtensionHeaderConstants::new("VK_KHR_storage_buffer_storage_class", 1),
    ExtensionHeaderConstants::new("VK_KHR_bind_memory2", 1),
    ExtensionHeaderConstants::new("VK_KHR_variable_pointers", 1),
    ExtensionHeaderConstants::new("VK_KHR_dedicated_allocation", 3),
    ExtensionHeaderConstants::new("VK_KHR_16bit_storage", 1),
    ExtensionHeaderConstants::new("VK_KHR_sampler_ycbcr_conversion", 14),
    ExtensionHeaderConstants::new("VK_KHR_maintenance1", 2),
    ExtensionHeaderConstants::new("VK_KHR_maintenance2", 1),
    ExtensionHeaderConstants::new("VK_KHR_maintenance3", 1),
    ExtensionHeaderConstants::new("VK_KHR_synchronization2", 1),
];

const TYPE_ALIASES: &'static [TypeAlias] = &[
    TypeAlias::new("VkSampleMask", "u32"),
    TypeAlias::new("VkBool32", "u32"),
    TypeAlias::new("VkFlags", "u32"),
    TypeAlias::new("VkFlags64", "u64"),
    TypeAlias::new(DEVICE_SIZE_TYPE, "u64"),
    TypeAlias::new(DEVICE_ADDR_TYPE, "u64"),
];

const OBJECTS: &'static [Object] = &[
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
    Object::new(
        "VkSamplerYcbcrConversionKHR",
        "SAMPLER_YCBCR_CONVERSION_KHR",
        vk_ext_enum(157, 0),
    )
    .extension("VK_KHR_sampler_ycbcr_conversion")
    .promoted("1_1", "VkSamplerYcbcrConversion", "SAMPLER_YCBCR_CONVERSION"),
    Object::new("VkDescriptorSet", "DESCRIPTOR_SET", 23),
    Object::new("VkDescriptorSetLayout", "DESCRIPTOR_SET_LAYOUT", 20),
    Object::new("VkDescriptorPool", "DESCRIPTOR_POOL", 22),
    Object::new(
        "VkDescriptorUpdateTemplateKHR",
        "DESCRIPTOR_UPDATE_TEMPLATE_KHR",
        vk_ext_enum(86, 0),
    )
    .extension("VK_KHR_descriptor_update_template")
    .promoted("1_1", "VkDescriptorUpdateTemplate", "DESCRIPTOR_UPDATE_TEMPLATE"),
    Object::new("VkFence", "FENCE", 7),
    Object::new("VkSemaphore", "SEMAPHORE", 5),
    Object::new("VkEvent", "EVENT", 11),
    Object::new("VkQueryPool", "QUERY_POOL", 12),
    Object::new("VkFramebuffer", "FRAMEBUFFER", 24),
    Object::new("VkRenderPass", "RENDER_PASS", 18),
    Object::new("VkPipelineCache", "PIPELINE_CACHE", 16),
    // WSI extensions
    Object::new("VkDisplayKHR", "DISPLAY_KHR", vk_ext_enum(3, 0)).extension("VK_KHR_display"),
    Object::new("VkDisplayModeKHR", "DISPLAY_MODE_KHR", vk_ext_enum(3, 1)).extension("VK_KHR_display"),
    Object::new("VkSurfaceKHR", "SURFACE_KHR", vk_ext_enum(1, 0)).extension("VK_KHR_surface"),
    Object::new("VkSwapchainKHR", "SWAPCHAIN_KHR", vk_ext_enum(2, 0)).extension("VK_KHR_swapchain"),
    // debug report
    Object::new(
        "VkDebugReportCallbackEXT",
        "DEBUG_REPORT_CALLBACK_EXT",
        vk_ext_enum(12, 0),
    )
    .extension("VK_EXT_debug_report"),
    Object::new(
        "VkDebugUtilsMessengerEXT",
        "DEBUG_UTILS_MESSENGER_EXT",
        vk_ext_enum(129, 0),
    )
    .extension("VK_EXT_debug_utils"),
];

const ENUMS: &'static [Enum] = &[
    Enum::new(
        "AttachmentLoadOp",
        "ATTACHMENT_LOAD_OP",
        &[
            Enum::member("LOAD", 0),
            Enum::member("CLEAR", 1),
            Enum::member("DONT_CARE", 2),
        ],
    ),
    Enum::new(
        "AttachmentStoreOp",
        "ATTACHMENT_STORE_OP",
        &[Enum::member("STORE", 0), Enum::member("DONT_CARE", 1)],
    ),
    Enum::new(
        "BlendFactor",
        "BLEND_FACTOR",
        &[
            Enum::member("ZERO", 0),
            Enum::member("ONE", 1),
            Enum::member("SRC_COLOR", 2),
            Enum::member("ONE_MINUS_SRC_COLOR", 3),
            Enum::member("DST_COLOR", 4),
            Enum::member("ONE_MINUS_DST_COLOR", 5),
            Enum::member("SRC_ALPHA", 6),
            Enum::member("ONE_MINUS_SRC_ALPHA", 7),
            Enum::member("DST_ALPHA", 8),
            Enum::member("ONE_MINUS_DST_ALPHA", 9),
            Enum::member("CONSTANT_COLOR", 10),
            Enum::member("ONE_MINUS_CONSTANT_COLOR", 11),
            Enum::member("CONSTANT_ALPHA", 12),
            Enum::member("ONE_MINUS_CONSTANT_ALPHA", 13),
            Enum::member("SRC_ALPHA_STAURATE", 14),
            Enum::member("SRC1_COLOR", 15),
            Enum::member("ONE_MINUS_SRC1_COLOR", 16),
            Enum::member("SRC1_ALPHA", 17),
            Enum::member("ONE_MINUS_SRC1_ALPHA", 18),
        ],
    ),
    Enum::new(
        "BlendOp",
        "BLEND_OP",
        &[
            Enum::member("ADD", 0),
            Enum::member("SUBTRACT", 1),
            Enum::member("REVERSE_SUBTRACT", 2),
            Enum::member("MIN", 3),
            Enum::member("MAX", 4),
        ],
    ),
    Enum::new(
        "BorderColor",
        "BORDER_COLOR",
        &[
            Enum::member("FLOAT_TRANSPARENT_BLACK", 0),
            Enum::member("INT_TRANSPARENT_BLACK", 1),
            Enum::member("FLOAT_OPAQUE_BLACK", 2),
            Enum::member("INT_OPAQUE_BLACK", 3),
            Enum::member("FLOAT_OPAQUE_WHITE", 4),
            Enum::member("INT_OPAQUE_WHITE", 5),
        ],
    ),
    Enum::new(
        "ChromaLocation",
        "CHROMA_LOCATION",
        &[
            Enum::member("COSITED_EVEN", 0)
                .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
                .promoted("1_1"),
            Enum::member("MIDPOINT", 1)
                .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
                .promoted("1_1"),
        ],
    )
    .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
    .promoted("1_1"),
    Enum::new(
        "ColorSpace",
        "COLOR_SPACE",
        &[Enum::member("SRGB_NONLINEAR", 0).extension("VK_KHR_surface", "KHR")],
    )
    .extension("VK_KHR_surface", "KHR"),
    Enum::new(
        "CommandBufferLevel",
        "COMMAND_BUFFER_LEVEL",
        &[Enum::member("PRIMARY", 0), Enum::member("SECONDARY", 1)],
    ),
    Enum::new(
        "CompareOp",
        "COMPARE_OP",
        &[
            Enum::member("NEVER", 0),
            Enum::member("LESS", 1),
            Enum::member("EQUAL", 2),
            Enum::member("LESS_OR_EQUAL", 3),
            Enum::member("GREATER", 4),
            Enum::member("NOT_EQUAL", 5),
            Enum::member("GREATER_OR_EQUAL", 6),
            Enum::member("ALWAYS", 7),
        ],
    ),
    Enum::new(
        "ComponentSwizzle",
        "COMPONENT_SWIZZLE",
        &[
            Enum::member("IDENTITY", 0),
            Enum::member("ZERO", 1),
            Enum::member("ONE", 2),
            Enum::member("R", 3),
            Enum::member("G", 4),
            Enum::member("B", 5),
            Enum::member("A", 6),
        ],
    ),
    Enum::new(
        "DebugReportObjectType",
        "DEBUG_REPORT_OBJECT_TYPE",
        &[
            Enum::member("UNKNOWN", 0),
            Enum::member("INSTANCE", 1),
            Enum::member("PHYSICAL_DEVICE", 2),
            Enum::member("DEVICE", 3),
            Enum::member("QUEUE", 4),
            Enum::member("SEMAPHORE", 5),
            Enum::member("COMMAND_BUFFER", 6),
            Enum::member("FENCE", 7),
            Enum::member("DEVICE_MEMORY", 8),
            Enum::member("BUFFER", 9),
            Enum::member("IMAGE", 10),
            Enum::member("EVENT", 11),
            Enum::member("QUERY_POOL", 12),
            Enum::member("BUFFER_VIEW", 13),
            Enum::member("IMAGE_VIEW", 14),
            Enum::member("SHADER_MODULE", 15),
            Enum::member("PIPELINE_CACHE", 16),
            Enum::member("PIPELINE_LAYOUT", 17),
            Enum::member("RENDER_PASS", 18),
            Enum::member("PIPELINE", 19),
            Enum::member("DESCRIPTOR_SET_LAYOUT", 20),
            Enum::member("SAMPLER", 21),
            Enum::member("DESCRIPTOR_POOL", 22),
            Enum::member("DESCRIPTOR_SET", 23),
            Enum::member("FRAMEBUFFER", 24),
            Enum::member("COMMAND_POOL", 25),
            Enum::member("SURFACE_KHR", 26),
            Enum::member("SWAPCHAIN_KHR", 27),
            Enum::member("DEBUG_REPORT_CALLBACK_EXT", 28),
            Enum::member("DISPLAY_KHR", 29),
            Enum::member("DISPLAY_MODE_KHR", 30),
            Enum::member("OBJECT_TABLE_NVX", 31),
            Enum::member("INDIRECT_COMMANDS_LAYOUT_NVX", 32),
            Enum::member("VALIDATION_CACHE_EXT", 33),
            Enum::member("DESCRIPTOR_UPDATE_TEMPLATE", vk_ext_enum(86, 0) as _)
                .extension("VK_KHR_descriptor_update_template", "KHR")
                .promoted("1_1"),
            Enum::member("SAMPLER_YCBCR_CONVERSION", vk_ext_enum(157, 0) as _)
                .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
                .promoted("1_1"),
        ],
    )
    .extension("VK_EXT_debug_report", "EXT"),
    Enum::new(
        "DescriptorType",
        "DESCRIPTOR_TYPE",
        &[
            Enum::member("SAMPLER", 0),
            Enum::member("COMBINED_IMAGE_SAMPLER", 1),
            Enum::member("SAMPLED_IMAGE", 2),
            Enum::member("STORAGE_IMAGE", 3),
            Enum::member("UNIFORM_TEXEL_BUFFER", 4),
            Enum::member("STORAGE_TEXEL_BUFFER", 5),
            Enum::member("UNIFORM_BUFFER", 6),
            Enum::member("STORAGE_BUFFER", 7),
            Enum::member("UNIFORM_BUFFER_DYNAMIC", 8),
            Enum::member("STORAGE_BUFFER_DYNAMIC", 9),
            Enum::member("INPUT_ATTACHMENT", 10),
        ],
    ),
    Enum::new(
        "DescriptorUpdateTemplateType",
        "DESCRIPTOR_UPDATE_TEMPLATE_TYPE",
        &[Enum::member("DESCRIPTOR_SET", 0)
            .extension("VK_KHR_descriptor_update_template", "KHR")
            .promoted("1_1")],
    )
    .extension("VK_KHR_descriptor_update_template", "KHR")
    .promoted("1_1"),
    Enum::new(
        "DynamicState",
        "DYNAMIC_STATE",
        &[
            Enum::member("VIEWPORT", 0),
            Enum::member("SCISSOR", 1),
            Enum::member("LINE_WIDTH", 2),
            Enum::member("DEPTH_BIAS", 3),
            Enum::member("BLEND_CONSTANTS", 4),
            Enum::member("DEPTH_BOUNDS", 5),
            Enum::member("STENCIL_COMPARE_MASK", 6),
            Enum::member("STENCIL_WRITE_MASK", 7),
            Enum::member("STENCIL_REFERENCE", 8),
        ],
    ),
    Enum::new(
        "Filter",
        "FILTER",
        &[Enum::member("NEAREST", 0), Enum::member("LINEAR", 1)],
    ),
    Enum::new(
        "FrontFace",
        "FRONT_FACE",
        &[Enum::member("COUNTER_CLOCKWISE", 0), Enum::member("CLOCKWISE", 1)],
    ),
    Enum::new(
        "ImageLayout",
        "IMAGE_LAYOUT",
        &[
            Enum::member("UNDEFINED", 0),
            Enum::member("GENERAL", 1),
            Enum::member("COLOR_ATTACHMENT_OPTIMAL", 2),
            Enum::member("DEPTH_STENCIL_ATTACHMENT_OPTIMAL", 3),
            Enum::member("DEPTH_STENCIL_READ_ONLY_OPTIMAL", 4),
            Enum::member("SHADER_READ_ONLY_OPTIMAL", 5),
            Enum::member("TRANSFER_SRC_OPTIMAL", 6),
            Enum::member("TRANSFER_DST_OPTIMAL", 7),
            Enum::member("PREINITIALIZED", 8),
            Enum::member("PRESENT_SRC", vk_ext_enum(2, 2) as _).extension("VK_KHR_swapchain", "KHR"),
            Enum::member("DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL", vk_ext_enum(118, 0) as _)
                .extension("VK_KHR_maintenance2", "KHR")
                .promoted("1_1"),
            Enum::member("DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL", vk_ext_enum(118, 1) as _)
                .extension("VK_KHR_maintenance2", "KHR")
                .promoted("1_1"),
        ],
    ),
    Enum::new(
        "ImageTiling",
        "IMAGE_TILING",
        &[Enum::member("OPTIMAL", 0), Enum::member("LINEAR", 1)],
    ),
    Enum::new(
        "ImageType",
        "IMAGE_TYPE",
        &[Enum::member("1D", 0), Enum::member("2D", 1), Enum::member("3D", 2)],
    ),
    Enum::new(
        "ImageViewType",
        "IMAGE_VIEW_TYPE",
        &[
            Enum::member("1D", 0),
            Enum::member("2D", 1),
            Enum::member("3D", 2),
            Enum::member("CUBE", 3),
            Enum::member("1D_ARRAY", 4),
            Enum::member("2D_ARRAY", 5),
            Enum::member("CUBE_ARRAY", 6),
        ],
    ),
    Enum::new(
        "IndexType",
        "INDEX_TYPE",
        &[Enum::member("UINT16", 0), Enum::member("UINT32", 1)],
    ),
    Enum::new(
        "LogicOp",
        "LOGIC_OP",
        &[
            Enum::member("CLEAR", 0),
            Enum::member("AND", 1),
            Enum::member("AND_REVERSE", 2),
            Enum::member("COPY", 3),
            Enum::member("AND_INVERTED", 4),
            Enum::member("NO_OP", 5),
            Enum::member("XOR", 6),
            Enum::member("OR", 7),
            Enum::member("NOR", 8),
            Enum::member("EQUIVALENT", 9),
            Enum::member("INVERT", 10),
            Enum::member("OR_REVERSE", 11),
            Enum::member("COPY_INVERTED", 12),
            Enum::member("OR_INVERTED", 13),
            Enum::member("NAND", 14),
            Enum::member("SET", 15),
        ],
    ),
    Enum::new(
        "InternalAllocationType",
        "INTERNAL_ALLOCATION_TYPE",
        &[Enum::member("EXECUTABLE", 0)],
    ),
    Enum::new(
        "PhysicalDeviceType",
        "PHYSICAL_DEVICE_TYPE",
        &[
            Enum::member("OTHER", 0),
            Enum::member("INTEGRATED_GPU", 1),
            Enum::member("DISCRETE_GPU", 2),
            Enum::member("VIRTUAL_GPU", 3),
            Enum::member("CPU", 4),
        ],
    ),
    Enum::new(
        "PipelineBindPoint",
        "PIPELINE_BIND_POINT",
        &[Enum::member("GRAPHICS", 0), Enum::member("COMPUTE", 1)],
    ),
    Enum::new(
        "PointClippingBehavior",
        "POINT_CLIPPING_BEHAVIOR",
        &[
            Enum::member("ALL_CLIP_PLANES", 0)
                .extension("VK_KHR_maintenance2", "KHR")
                .promoted("1_1"),
            Enum::member("USER_CLIP_PLANES", 1)
                .extension("VK_KHR_maintenance2", "KHR")
                .promoted("1_1"),
        ],
    )
    .extension("VK_KHR_maintenance2", "KHR")
    .promoted("1_1"),
    Enum::new(
        "PolygonMode",
        "POLYGON_MODE",
        &[
            Enum::member("FILL", 0),
            Enum::member("LINE", 1),
            Enum::member("POINT", 2),
        ],
    ),
    Enum::new(
        "PresentMode",
        "PRESENT_MODE",
        &[
            Enum::member("IMMEDIATE", 0).extension("VK_KHR_surface", "KHR"),
            Enum::member("MAILBOX", 1).extension("VK_KHR_surface", "KHR"),
            Enum::member("FIFO", 2).extension("VK_KHR_surface", "KHR"),
            Enum::member("FIFO_RELAXED", 3).extension("VK_KHR_surface", "KHR"),
        ],
    )
    .extension("VK_KHR_surface", "KHR"),
    Enum::new(
        "PrimitiveTopology",
        "PRIMITIVE_TOPOLOGY",
        &[
            Enum::member("POINT_LIST", 0),
            Enum::member("LINE_LIST", 1),
            Enum::member("LINE_STRIP", 2),
            Enum::member("TRIANGLE_LIST", 3),
            Enum::member("TRIANGLE_STRIP", 4),
            Enum::member("TRIANGLE_FAN", 5),
            Enum::member("LINE_LIST_WITH_ADJACENCY", 6),
            Enum::member("LINE_STRIP_WITH_ADJACENCY", 7),
            Enum::member("TRIANGLE_LIST_WITH_ADJACENCY", 8),
            Enum::member("TRIANGLE_STRIP_WITH_ADJACENCY", 9),
            Enum::member("PATCH_LIST", 10),
        ],
    ),
    Enum::new(
        "QueryType",
        "QUERY_TYPE",
        &[
            Enum::member("OCCLUSION", 0),
            Enum::member("PIPELINE_STATISTICS", 1),
            Enum::member("TIMESTAMP", 2),
        ],
    ),
    Enum::new(
        "SamplerAddressMode",
        "SAMPLER_ADDRESS_MODE",
        &[
            Enum::member("REPEAT", 0),
            Enum::member("MIRRORED_REPEAT", 1),
            Enum::member("CLAMP_TO_EDGE", 2),
            Enum::member("CLAMP_TO_BORDER", 3),
            Enum::member("MIRROR_CLAMP_TO_EDGE", 4),
        ],
    ),
    Enum::new(
        "SamplerMipmapMode",
        "SAMPLER_MIPMAP_MODE",
        &[Enum::member("NEAREST", 0), Enum::member("LINEAR", 1)],
    ),
    Enum::new(
        "SamplerYcbcrModelConversion",
        "SAMPLER_YCBCR_MODEL_CONVERSION",
        &[
            Enum::member("RGB_IDENTITY", 0)
                .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
                .promoted("1_1"),
            Enum::member("YCBCR_IDENTITY", 1)
                .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
                .promoted("1_1"),
            Enum::member("YCBCR_709", 2)
                .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
                .promoted("1_1"),
            Enum::member("YCBCR_601", 3)
                .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
                .promoted("1_1"),
            Enum::member("YCBCR_2020", 4)
                .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
                .promoted("1_1"),
        ],
    )
    .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
    .promoted("1_1"),
    Enum::new(
        "SamplerYcbcrRange",
        "SAMPLER_YCBCR_RANGE",
        &[
            Enum::member("ITU_FULL", 0)
                .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
                .promoted("1_1"),
            Enum::member("ITU_NARROW", 1)
                .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
                .promoted("1_1"),
        ],
    )
    .extension("VK_KHR_sampler_ycbcr_conversion", "KHR")
    .promoted("1_1"),
    Enum::new(
        "SharingMode",
        "SHARING_MODE",
        &[Enum::member("EXCLUSIVE", 0), Enum::member("CONCURRENT", 1)],
    ),
    Enum::new(
        "StencilOp",
        "STENCIL_OP",
        &[
            Enum::member("KEEP", 0),
            Enum::member("ZERO", 1),
            Enum::member("REPLACE", 2),
            Enum::member("INCREMENT_AND_CLAMP", 3),
            Enum::member("DECREMENT_AND_CLAMP", 4),
            Enum::member("INVERT", 5),
            Enum::member("INCREMENT_AND_WRAP", 6),
            Enum::member("DECREMENT_AND_WRAP", 7),
        ],
    ),
    Enum::new(
        "SubpassContents",
        "SUBPASS_CONTENTS",
        &[Enum::member("INLINE", 0), Enum::member("SECONDARY_COMMAND_BUFFERS", 1)],
    ),
    Enum::new(
        "SystemAllocationScope",
        "SYSTEM_ALLOCATION_SCOPE",
        &[
            Enum::member("COMMAND", 0),
            Enum::member("OBJECT", 1),
            Enum::member("CACHE", 2),
            Enum::member("DEVICE", 3),
            Enum::member("INSTANCE", 4),
        ],
    ),
    Enum::new(
        "TessellationDomainOrigin",
        "TESSELLATION_DOMAIN_ORIGIN",
        &[
            Enum::member("UPPER_LEFT", 0)
                .extension("VK_KHR_maintenance2", "KHR")
                .promoted("1_1"),
            Enum::member("LOWER_LEFT", 1)
                .extension("VK_KHR_maintenance2", "KHR")
                .promoted("1_1"),
        ],
    )
    .extension("VK_KHR_maintenance2", "KHR")
    .promoted("1_1"),
    Enum::new(
        "VertexInputRate",
        "VERTEX_INPUT_RATE",
        &[Enum::member("VERTEX", 0), Enum::member("INSTANCE", 1)],
    ),
];

const FLAGS: &'static [Bitmask] = &[
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
            Bitmask::entry("COLOR_ATTACHMENT_READ_NONCOHERENT", 19).extension("EXT", "blend_operation_advanced"),
        ],
    ),
    Bitmask::new(
        "AccessFlags2",
        "AccessFlagBits2",
        "ACCESS_2",
        &[
            Bitmask::entry("INDIRECT_COMMAND_READ", 0)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("INDEX_READ", 1)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("VERTEX_ATTRIBUTE_READ", 2)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("UNIFORM_READ", 3)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("INPUT_ATTACHMENT_READ", 4)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("SHADER_READ", 5)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("SHADER_WRITE", 6)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("COLOR_ATTACHMENT_READ", 7)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("COLOR_ATTACHMENT_WRITE", 8)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("DEPTH_STENCIL_ATTACHMENT_READ", 9)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("DEPTH_STENCIL_ATTACHMENT_WRITE", 10)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("TRANSFER_READ", 11)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("TRANSFER_WRITE", 12)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("HOST_READ", 13)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("HOST_WRITE", 14)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("MEMORY_READ", 15)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("MEMORY_WRITE", 16)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("SHADER_SAMPLED_READ", 32)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("SHADER_STORAGE_READ", 33)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("SHADER_STORAGE_WRITE", 34)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
        ],
    )
    .long()
    .extension("KHR", "synchronization2")
    .promoted("1_3"),
    Bitmask::new(
        "AndroidSurfaceCreateFlags",
        "AndroidSurfaceCreateFlagBits",
        "ANDROID_SURFACE_CREATE",
        &[],
    )
    .extension("KHR", "android_surface"),
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
        "CommandPoolTrimFlags",
        "CommandPoolTrimFlagBits",
        "COMMAND_POOL_TRIM",
        &[],
    )
    .extension("KHR", "maintenance1")
    .promoted("1_1"),
    Bitmask::new(
        "CompositeAlphaFlags",
        "CompositeAlphaFlagBits",
        "COMPOSITE_ALPHA",
        &[
            Bitmask::entry("OPAQUE", 0).extension("KHR", "surface"),
            Bitmask::entry("PRE_MULTIPLIED", 1).extension("KHR", "surface"),
            Bitmask::entry("POST_MULTIPLIED", 2).extension("KHR", "surface"),
            Bitmask::entry("INHERIT", 3).extension("KHR", "surface"),
        ],
    )
    .extension("KHR", "surface"),
    Bitmask::new(
        "CullModeFlags",
        "CullModeFlagBits",
        "CULL_MODE",
        &[Bitmask::entry("FRONT", 0), Bitmask::entry("BACK", 1)],
    ),
    Bitmask::new(
        "DebugReportFlags",
        "DebugReportFlagBits",
        "DEBUG_REPORT",
        &[
            Bitmask::entry("INFORMATION", 0).extension("EXT", "debug_report"),
            Bitmask::entry("WARNING", 1).extension("EXT", "debug_report"),
            Bitmask::entry("PERFORMANCE_WARNING", 2).extension("EXT", "debug_report"),
            Bitmask::entry("ERROR", 3).extension("EXT", "debug_report"),
            Bitmask::entry("DEBUG", 4).extension("EXT", "debug_report"),
        ],
    )
    .extension("EXT", "debug_report"),
    Bitmask::new(
        "DebugUtilsMessageSeverityFlags",
        "DebugUtilsMessageSeverityFlagBits",
        "DEBUG_UTILS_MESSAGE_SEVERITY",
        &[
            Bitmask::entry("VERBOSE", 0).extension("EXT", "debug_utils"),
            Bitmask::entry("INFO", 4).extension("EXT", "debug_utils"),
            Bitmask::entry("WARNING", 8).extension("EXT", "debug_utils"),
            Bitmask::entry("ERROR", 12).extension("EXT", "debug_utils"),
        ],
    )
    .extension("EXT", "debug_utils"),
    Bitmask::new(
        "DebugUtilsMessageTypeFlags",
        "DebugUtilsMessageTypeFlagBits",
        "DEBUG_UTILS_MESSAGE_TYPE",
        &[
            Bitmask::entry("GENERAL", 0).extension("EXT", "debug_utils"),
            Bitmask::entry("VALIDATION", 1).extension("EXT", "debug_utils"),
            Bitmask::entry("PERFORMANCE", 2).extension("EXT", "debug_utils"),
        ],
    )
    .extension("EXT", "debug_utils"),
    Bitmask::new(
        "DebugUtilsMessengerCallbackDataFlags",
        "DebugUtilsMessengerCallbackDataFlagBits",
        "DEBUG_UTILS_MESSENGER_CALLBACK_DATA",
        &[],
    )
    .extension("EXT", "debug_utils"),
    Bitmask::new(
        "DebugUtilsMessengerCreateFlags",
        "DebugUtilsMessengerCreateFlagBits",
        "DEBUG_UTILS_MESSENGER_CREATE",
        &[],
    )
    .extension("EXT", "debug_utils"),
    Bitmask::new(
        "DependencyFlags",
        "DependencyFlagBits",
        "DEPENDENCY",
        &[
            Bitmask::entry("BY_REGION", 0),
            Bitmask::entry("VIEW_LOCAL", 1)
                .extension("KHR", "multiview")
                .promoted("1_1"),
            Bitmask::entry("DEVICE_GROUP", 2)
                .extension("KHR", "device_group")
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
    Bitmask::new(
        "DescriptorUpdateTemplateCreateFlags",
        "DescriptorUpdateTemplateCreateFlagBits",
        "DESCRIPTOR_UPDATE_TEMPLATE_CREATE",
        &[],
    )
    .extension("KHR", "descriptor_update_template")
    .promoted("1_1"),
    Bitmask::new("DeviceCreateFlags", "DeviceCreateFlagBits", "DEVICE_CREATE", &[]),
    Bitmask::new(
        "DeviceGroupPresentModeFlags",
        "DeviceGroupPresentModeFlagBits",
        "DEVICE_GROUP_PRESENT_MODE",
        &[],
    )
    .extension("KHR", "device_group")
    .extra_requirements(&["VK_KHR_surface"]),
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
    .extension("KHR", "display"),
    Bitmask::new(
        "DisplayPlaneAlphaFlags",
        "DisplayPlaneAlphaFlagBits",
        "DISPLAY_PLANE_ALPHA",
        &[
            Bitmask::entry("OPAQUE", 0).extension("KHR", "display"),
            Bitmask::entry("GLOBAL", 1).extension("KHR", "display"),
            Bitmask::entry("PER_PIXEL", 2).extension("KHR", "display"),
            Bitmask::entry("PER_PIXEL_PREMULTIPLIED", 3).extension("KHR", "display"),
        ],
    )
    .extension("KHR", "display"),
    Bitmask::new(
        "DisplaySurfaceCreateFlags",
        "DisplaySurfaceCreateFlagBits",
        "DISPLAY_SURFACE_CREATE",
        &[],
    )
    .extension("KHR", "display"),
    Bitmask::new("EventCreateFlags", "EventCreateFlagBits", "EVENT_CREATE", &[]),
    Bitmask::new(
        "ExternalFenceFeatureFlags",
        "ExternalFenceFeatureFlagBits",
        "EXTERNAL_FENCE_FEATURE",
        &[
            Bitmask::entry("EXPORTABLE", 0)
                .extension("KHR", "external_fence_capabilities")
                .promoted("1_1"),
            Bitmask::entry("IMPORTABLE", 1)
                .extension("KHR", "external_fence_capabilities")
                .promoted("1_1"),
        ],
    )
    .extension("KHR", "external_fence_capabilities")
    .promoted("1_1"),
    Bitmask::new(
        "ExternalFenceHandleTypeFlags",
        "ExternalFenceHandleTypeFlagBits",
        "EXTERNAL_FENCE_HANDLE_TYPE",
        &[
            Bitmask::entry("OPAQUE_FD", 0)
                .extension("KHR", "external_fence_capabilities")
                .promoted("1_1"),
            Bitmask::entry("OPAQUE_WIN32", 1)
                .extension("KHR", "external_fence_capabilities")
                .promoted("1_1"),
            Bitmask::entry("OPAQUE_WIN32_KMT", 2)
                .extension("KHR", "external_fence_capabilities")
                .promoted("1_1"),
            Bitmask::entry("SYNC_FD", 3)
                .extension("KHR", "external_fence_capabilities")
                .promoted("1_1"),
        ],
    )
    .extension("KHR", "external_fence_capabilities")
    .promoted("1_1"),
    Bitmask::new(
        "ExternalMemoryFeatureFlags",
        "ExternalMemoryFeatureFlagBits",
        "EXTERNAL_MEMORY_FEATURE",
        &[
            Bitmask::entry("DEDICATED_ONLY", 0)
                .extension("KHR", "external_memory_capabilities")
                .promoted("1_1"),
            Bitmask::entry("EXPORTABLE", 1)
                .extension("KHR", "external_memory_capabilities")
                .promoted("1_1"),
            Bitmask::entry("IMPORTABLE", 2)
                .extension("KHR", "external_memory_capabilities")
                .promoted("1_1"),
        ],
    )
    .extension("KHR", "external_memory_capabilities")
    .promoted("1_1"),
    Bitmask::new(
        "ExternalMemoryHandleTypeFlags",
        "ExternalMemoryHandleTypeFlagBits",
        "EXTERNAL_MEMORY_HANDLE_TYPE",
        &[
            Bitmask::entry("OPAQUE_FD", 0)
                .extension("KHR", "external_memory_capabilities")
                .promoted("1_1"),
            Bitmask::entry("OPAQUE_WIN32", 1)
                .extension("KHR", "external_memory_capabilities")
                .promoted("1_1"),
            Bitmask::entry("OPAQUE_WIN32_KMT", 2)
                .extension("KHR", "external_memory_capabilities")
                .promoted("1_1"),
            Bitmask::entry("D3D11_TEXTURE", 3)
                .extension("KHR", "external_memory_capabilities")
                .promoted("1_1"),
            Bitmask::entry("D3D11_TEXTURE_KMT", 4)
                .extension("KHR", "external_memory_capabilities")
                .promoted("1_1"),
            Bitmask::entry("D3D12_HEAP", 5)
                .extension("KHR", "external_memory_capabilities")
                .promoted("1_1"),
            Bitmask::entry("D3D12_RESOURCE", 6)
                .extension("KHR", "external_memory_capabilities")
                .promoted("1_1"),
        ],
    )
    .extension("KHR", "external_memory_capabilities")
    .promoted("1_1"),
    Bitmask::new(
        "ExternalSemaphoreFeatureFlags",
        "ExternalSemaphoreFeatureFlagBits",
        "EXTERNAL_SEMAPHORE_FEATURE",
        &[
            Bitmask::entry("EXPORTABLE", 0)
                .extension("KHR", "external_semaphore_capabilities")
                .promoted("1_1"),
            Bitmask::entry("IMPORTABLE", 1)
                .extension("KHR", "external_semaphore_capabilities")
                .promoted("1_1"),
        ],
    )
    .extension("KHR", "external_semaphore_capabilities")
    .promoted("1_1"),
    Bitmask::new(
        "ExternalSemaphoreHandleTypeFlags",
        "ExternalSemaphoreHandleTypeFlagBits",
        "EXTERNAL_SEMAPHORE_HANDLE_TYPE",
        &[
            Bitmask::entry("OPAQUE_FD", 0)
                .extension("KHR", "external_semaphore_capabilities")
                .promoted("1_1"),
            Bitmask::entry("OPAQUE_WIN32", 1)
                .extension("KHR", "external_semaphore_capabilities")
                .promoted("1_1"),
            Bitmask::entry("OPAQUE_WIN32_KMT", 2)
                .extension("KHR", "external_semaphore_capabilities")
                .promoted("1_1"),
            Bitmask::entry("D3D12_FENCE", 3)
                .extension("KHR", "external_semaphore_capabilities")
                .promoted("1_1"),
            // promoted special alias
            Bitmask::entry("D3D11_FENCE", 3).version_since("1_1"),
            Bitmask::entry("SYNC_FD", 4)
                .extension("KHR", "external_semaphore_capabilities")
                .promoted("1_1"),
        ],
    )
    .extension("KHR", "external_semaphore_capabilities")
    .promoted("1_1"),
    Bitmask::new(
        "FenceCreateFlags",
        "FenceCreateFlagBits",
        "FENCE_CREATE",
        &[Bitmask::entry("SIGNALED", 0)],
    ),
    Bitmask::new(
        "FenceImportFlags",
        "FenceImportFlagBits",
        "FENCE_IMPORT",
        &[Bitmask::entry("TEMPORARY", 0)
            .extension("KHR", "external_fence")
            .promoted("1_1")],
    )
    .extension("KHR", "external_fence")
    .promoted("1_1"),
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
                .extension("KHR", "maintenance1")
                .promoted("1_1"),
            Bitmask::entry("TRANSFER_DST", 15)
                .extension("KHR", "maintenance1")
                .promoted("1_1"),
            Bitmask::entry("MIDPOINT_CHROMA_SAMPLES", 17)
                .extension("KHR", "sampler_ycbcr_conversion")
                .promoted("1_1"),
            Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER", 18)
                .extension("KHR", "sampler_ycbcr_conversion")
                .promoted("1_1"),
            Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER", 19)
                .extension("KHR", "sampler_ycbcr_conversion")
                .promoted("1_1"),
            Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT", 20)
                .extension("KHR", "sampler_ycbcr_conversion")
                .promoted("1_1"),
            Bitmask::entry(
                "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE",
                21,
            )
            .extension("KHR", "sampler_ycbcr_conversion")
            .promoted("1_1"),
            Bitmask::entry("DISJOINT", 22)
                .extension("KHR", "sampler_ycbcr_conversion")
                .promoted("1_1"),
            Bitmask::entry("COSITED_CHROMA_SAMPLES", 23)
                .extension("KHR", "sampler_ycbcr_conversion")
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
            .extension("KHR", "portability_enumeration")
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
            Bitmask::entry("PLANE_0", 4)
                .extension("KHR", "sampler_ycbcr_conversion")
                .promoted("1_1"),
            Bitmask::entry("PLANE_1", 5)
                .extension("KHR", "sampler_ycbcr_conversion")
                .promoted("1_1"),
            Bitmask::entry("PLANE_2", 6)
                .extension("KHR", "sampler_ycbcr_conversion")
                .promoted("1_1"),
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
                .extension("KHR", "maintenance1")
                .promoted("1_1"),
            Bitmask::entry("SPLIT_INSTANCE_BIND_REGIONS", 6)
                .extension("KHR", "device_group")
                .extra_requirements(&["VK_KHR_bind_memory2"])
                .promoted("1_1"),
            Bitmask::entry("BLOCK_TEXEL_VIEW_COMPATIBLE", 7)
                .extension("KHR", "maintenance2")
                .promoted("1_1"),
            Bitmask::entry("EXTENDED_USAGE", 8)
                .extension("KHR", "maintenance2")
                .promoted("1_1"),
            Bitmask::entry("DISJOINT", 9)
                .extension("KHR", "sampler_ycbcr_conversion")
                .promoted("1_1"),
            Bitmask::entry("ALIAS", 10)
                .extension("KHR", "bind_memory2")
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
        "MemoryAllocateFlags",
        "MemoryAllocateFlagBits",
        "MEMORY_ALLOCATE",
        &[Bitmask::entry("DEVICE_MASK", 0)
            .extension("KHR", "device_group")
            .promoted("1_1")],
    )
    .extension("KHR", "device_group")
    .promoted("1_1"),
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
        "MetalSurfaceCreateFlags",
        "MetalSurfaceCreateFlagBits",
        "METAL_SURFACE_CREATE",
        &[],
    )
    .extension("EXT", "metal_surface"),
    Bitmask::new(
        "PeerMemoryFeatureFlags",
        "PeerMemoryFeatureFlagBits",
        "PEER_MEMORY_FEATURE",
        &[
            Bitmask::entry("COPY_SRC", 0)
                .extension("KHR", "device_group")
                .promoted("1_1"),
            Bitmask::entry("COPY_DST", 1)
                .extension("KHR", "device_group")
                .promoted("1_1"),
            Bitmask::entry("GENERIC_SRC", 2)
                .extension("KHR", "device_group")
                .promoted("1_1"),
            Bitmask::entry("GENERIC_DST", 3)
                .extension("KHR", "device_group")
                .promoted("1_1"),
        ],
    )
    .extension("KHR", "device_group")
    .promoted("1_1"),
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
            Bitmask::entry("VIEW_INDEX_FROM_DEVICE_INDEX", 3)
                .extension("KHR", "device_group")
                .promoted("1_1"),
            Bitmask::entry("DISPATCH_BASE", 4)
                .extension("KHR", "device_group")
                .promoted("1_1"),
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
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("DRAW_INDIRECT", 1)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("VERTEX_INPUT", 2)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("VERTEX_SHADER", 3)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("TESSELLATION_CONTROL_SHADER", 4)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("TESSELLATION_EVALUATION_SHADER", 5)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("GEOMETRY_SHADER", 6)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("FRAGMENT_SHADER", 7)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("EARLY_FRAGMENT_TESTS", 8)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("LATE_FRAGMENT_TESTS", 9)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("COLOR_ATTACHMENT_OUTPUT", 10)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("COMPUTE_SHADER", 11)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("ALL_TRANSFER", 12)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("BOTTOM_OF_PIPE", 13)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("HOST", 14)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("ALL_GRAPHICS", 15)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("ALL_COMMANDS", 16)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("COPY", 32)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("RESOLVE", 33)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("BLIT", 34)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("CLEAR", 35)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("INDEX_INPUT", 36)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("VERTEX_ATTRIBUTE_INPUT", 37)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
            Bitmask::entry("PRE_RASTERIZATION_SHADERS", 38)
                .extension("KHR", "synchronization2")
                .promoted("1_3"),
        ],
    )
    .long()
    .extension("KHR", "synchronization2")
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
        "SemaphoreImportFlags",
        "SemaphoreImportFlagBits",
        "SEMAPHORE_IMPORT",
        &[Bitmask::entry("TEMPORARY", 0)
            .extension("KHR", "external_semaphore")
            .promoted("1_1")],
    )
    .extension("KHR", "external_semaphore")
    .promoted("1_1"),
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
            .extension("KHR", "synchronization2")
            .promoted("1_3")],
    )
    .extension("KHR", "synchronization2")
    .promoted("1_3"),
    Bitmask::new(
        "SubpassDescriptionFlags",
        "SubpassDescriptionFlagBits",
        "SUBPASS_DESCRIPTION",
        &[],
    ),
    Bitmask::new(
        "SurfaceTransformFlags",
        "SurfaceTransformFlagBits",
        "SURFACE_TRANSFORM",
        &[
            Bitmask::entry("IDENTITY", 0).extension("KHR", "surface"),
            Bitmask::entry("ROTATE_90", 1).extension("KHR", "surface"),
            Bitmask::entry("ROTATE_180", 2).extension("KHR", "surface"),
            Bitmask::entry("ROTATE_270", 3).extension("KHR", "surface"),
            Bitmask::entry("HORIZONTAL_MIRROR", 4).extension("KHR", "surface"),
            Bitmask::entry("HORIZONTAL_MIRROR_ROTATE_90", 5).extension("KHR", "surface"),
            Bitmask::entry("HORIZONTAL_MIRROR_ROTATE_180", 6).extension("KHR", "surface"),
            Bitmask::entry("HORIZONTAL_MIRROR_ROTATE_270", 7).extension("KHR", "surface"),
            Bitmask::entry("INHERIT", 8).extension("KHR", "surface"),
        ],
    )
    .extension("KHR", "surface"),
    Bitmask::new(
        "SwapchainCreateFlags",
        "SwapchainCreateFlagBits",
        "SWAPCHAIN_CREATE",
        &[Bitmask::entry("SPLIT_INSTNACE_BIND_REGIONS", 0).extension("KHR", "device_group")],
    )
    .extension("KHR", "swapchain"),
    Bitmask::new(
        "WaylandSurfaceCreateFlags",
        "WaylandSurfaceCreateFlagBits",
        "WAYLAND_SURFACE_CREATE",
        &[],
    )
    .extension("KHR", "wayland_surface"),
    Bitmask::new(
        "Win32SurfaceCreateFlags",
        "Win32SurfaceCreateFlagBits",
        "WIN32_SURFACE_CREATE",
        &[],
    )
    .extension("KHR", "win32_surface"),
    Bitmask::new(
        "XcbSurfaceCreateFlags",
        "XcbSurfaceCreateFlagBits",
        "XCB_SURFACE_CREATE",
        &[],
    )
    .extension("KHR", "xcb_surface"),
    Bitmask::new(
        "XlibSurfaceCreateFlags",
        "XlibSurfaceCreateFlagBits",
        "XLIB_SURFACE_CREATE",
        &[],
    )
    .extension("KHR", "xlib_surface"),
];

const FUNC_POINTERS: &'static [FuncPointer] = &[
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
    FuncPointer::new(
        "DebugReportCallback",
        &[
            ("flags", "VkDebugReportFlagsEXT"),
            ("objectType", "VkDebugReportObjectTypeEXT"),
            ("object", "u64"),
            ("location", "usize"),
            ("messageCode", "i32"),
            ("pLayerPrefix", "*const core::ffi::c_char"),
            ("pMessage", "*const core::ffi::c_char"),
            ("pUserData", "*mut core::ffi::c_void"),
        ],
    )
    .returns("VkBool32")
    .extension("EXT", "debug_report"),
    FuncPointer::new(
        "DebugUtilsMessengerCallback",
        &[
            ("messageSeverity", "VkDebugUtilsMessageSeverityFlagBitsEXT"),
            ("messageTypes", "VkDebugUtilsMessageTypeFlagsEXT"),
            ("pCallbackData", "*const VkDebugUtilsMessengerCallbackDataEXT"),
            ("pUserData", "*mut core::ffi::c_void"),
        ],
    )
    .returns("VkBool32")
    .extension("EXT", "debug_utils"),
];

const STRUCTS: &'static [Struct] = &[
    Struct::typed(
        "AcquireNextImageInfo",
        "ACQUIRE_NEXT_IMAGE_INFO",
        vk_ext_enum(61, 10) as _,
        StructUsage::Source,
        &[
            Struct::member("swapchain", "VkSwapchainKHR"),
            Struct::member("timeout", "u64"),
            Struct::member("semaphore", "VkSemaphore"),
            Struct::member("fence", "VkFence"),
            Struct::member("deviceMask", "u32"),
        ],
    )
    .extensions(&[("KHR", "device_group"), ("KHR", "swapchain")]),
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
        "AndroidSurfaceCreateInfo",
        &[
            Struct::member("flags", "VkAndroidSurfaceCreateFlagsKHR"),
            Struct::member("window", "*mut android::ANativeWindow"),
        ],
    )
    .stype(
        "ANDROID_SURFACE_CREATE_INFO",
        vk_ext_enum(9, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "android_surface")]),
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
    Struct::typed(
        "BindBufferMemoryDeviceGroupInfo",
        "BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO",
        vk_ext_enum(61, 13) as _,
        StructUsage::Source,
        &[
            Struct::member("deviceIndexCount", "u32"),
            Struct::member("pDeviceIndices", "*const u32"),
        ],
    )
    .extensions(&[("KHR", "device_group"), ("KHR", "bind_memory2")])
    .promoted("1_1"),
    Struct::typed(
        "BindBufferMemoryInfo",
        "BIND_BUFFER_MEMORY_INFO",
        vk_ext_enum(158, 0) as _,
        StructUsage::Source,
        &[
            Struct::member("buffer", "VkBuffer"),
            Struct::member("memory", "VkDeviceMemory"),
            Struct::member("memoryOffset", DEVICE_SIZE_TYPE),
        ],
    )
    .extensions(&[("KHR", "bind_memory2")])
    .promoted("1_1"),
    Struct::typed(
        "BindImageMemoryDeviceGroupInfo",
        "BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO",
        vk_ext_enum(61, 14) as _,
        StructUsage::Source,
        &[
            Struct::member("deviceIndexCount", "u32"),
            Struct::member("pDeviceIndices", "*const u32"),
            Struct::member("splitInstanceBindRegionCount", "u32"),
            Struct::member("pSplitInstanceBindRegions", "*const VkRect2D"),
        ],
    )
    .extensions(&[("KHR", "device_group"), ("KHR", "bind_memory2")])
    .promoted("1_1"),
    Struct::typed(
        "BindImageMemoryInfo",
        "BIND_IMAGE_MEMORY_INFO",
        vk_ext_enum(158, 1) as _,
        StructUsage::Source,
        &[
            Struct::member("image", "VkImage"),
            Struct::member("memory", "VkDeviceMemory"),
            Struct::member("memoryOffset", DEVICE_SIZE_TYPE),
        ],
    )
    .extensions(&[("KHR", "bind_memory2")])
    .promoted("1_1"),
    Struct::typed(
        "BindImageMemorySwapchainInfo",
        "BIND_IMAGE_MEMORY_SWAPCHAIN_INFO",
        vk_ext_enum(61, 9) as _,
        StructUsage::Source,
        &[
            Struct::member("swapchain", "VkSwapchainKHR"),
            Struct::member("imageIndex", "u32"),
        ],
    )
    .extensions(&[("KHR", "device_group"), ("KHR", "swapchain")]),
    Struct::typed(
        "BindImagePlaneMemoryInfo",
        "BIND_IMAGE_PLANE_MEMORY_INFO",
        vk_ext_enum(157, 2) as _,
        StructUsage::Source,
        &[Struct::member("planeAspect", "VkImageAspectFlags")],
    )
    .extensions(&[("KHR", "sampler_ycbcr_conversion")])
    .promoted("1_1"),
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
    Struct::typed(
        "BufferMemoryRequirementsInfo2",
        "BUFFER_MEMORY_REQUIREMENTS_INFO_2",
        vk_ext_enum(147, 0) as _,
        StructUsage::Source,
        &[Struct::member("buffer", "VkBuffer")],
    )
    .extensions(&[("KHR", "get_memory_requirements2")])
    .promoted("1_1"),
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
            Struct::member("renderPass", "VkRenderPass"),
            Struct::member("subpass", "u32"),
            Struct::member("framebuffer", "VkFramebuffer"),
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
            Struct::member("layout", "VkPipelineLayout"),
            Struct::member("basePipelineHandle", "VkPipeline"),
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
        "D3D12FenceSubmitInfo",
        &[
            Struct::member("waitSemaphoreValuesCount", "u32"),
            Struct::member("pWaitSemaphoreValues", "*const u64"),
            Struct::member("signalSemaphoreValuesCount", "u32"),
            Struct::member("pSignalSemaphoreValues", "*const u64"),
        ],
    )
    .stype("D3D12_FENCE_SUBMIT_INFO", vk_ext_enum(79, 2) as _, StructUsage::Source)
    .extensions(&[("KHR", "external_semaphore_win32")]),
    Struct::new(
        "DebugReportCallbackCreateInfo",
        &[
            Struct::member("flags", "VkDebugReportFlagsEXT"),
            Struct::member("pfnCallback", "PFN_vkDebugReportCallbackEXT"),
            Struct::member("pUserData", "*mut core::ffi::c_void"),
        ],
    )
    .stype(
        "DEBUG_REPORT_CALLBACK_CREATE_INFO",
        vk_ext_enum(12, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("EXT", "debug_report")]),
    Struct::new(
        "DebugUtilsLabel",
        &[
            Struct::member("pLabelName", "*const core::ffi::c_char"),
            Struct::member("pColor", "[core::ffi::c_float; 4]"),
        ],
    )
    .stype("DEBUG_UTILS_LABEL", vk_ext_enum(129, 2) as _, StructUsage::Source)
    .extensions(&[("EXT", "debug_utils")]),
    Struct::new(
        "DebugUtilsMessengerCallbackData",
        &[
            Struct::member("flags", "VkDebugUtilsMessengerCallbackDataFlagsEXT"),
            Struct::member("pMessageIdName", "*const core::ffi::c_char"),
            Struct::member("messageIdNumber", "i32"),
            Struct::member("pMessage", "*const core::ffi::c_char"),
            Struct::member("queueLabelCount", "u32"),
            Struct::member("pQueueLabels", "*const VkDebugUtilsLabelEXT"),
            Struct::member("cmdBufLabelCount", "u32"),
            Struct::member("pCmdBufLabels", "*const VkDebugUtilsLabelEXT"),
            Struct::member("objectCount", "u32"),
            Struct::member("pObjects", "*const VkDebugUtilsObjectNameInfoEXT"),
        ],
    )
    .stype(
        "DEBUG_UTILS_MESSENGER_CALLBACK_DATA",
        vk_ext_enum(129, 3) as _,
        StructUsage::Source,
    )
    .extensions(&[("EXT", "debug_utils")]),
    Struct::new(
        "DebugUtilsMessengerCreateInfo",
        &[
            Struct::member("flags", "VkDebugUtilsMessengerCreateFlagsEXT"),
            Struct::member("messageSeverity", "VkDebugUtilsMessageSeverityFlagsEXT"),
            Struct::member("messageType", "VkDebugUtilsMessageTypeFlagsEXT"),
            Struct::member("pfnUserCallback", "PFN_vkDebugUtilsMessengerCallbackEXT"),
            Struct::member("pUserData", "*mut core::ffi::c_void"),
        ],
    )
    .stype(
        "DEBUG_UTILS_MESSENGER_CREATE_INFO",
        vk_ext_enum(129, 4) as _,
        StructUsage::Source,
    )
    .extensions(&[("EXT", "debug_utils")]),
    Struct::new(
        "DebugUtilsObjectNameInfo",
        &[
            Struct::member("objectType", "VkObjectType"),
            Struct::member("objectHandle", "u64"),
            Struct::member("pObjectName", "*const core::ffi::c_char"),
        ],
    )
    .stype(
        "DEBUG_UTILS_OBJECT_NAME_INFO",
        vk_ext_enum(129, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("EXT", "debug_utils")]),
    Struct::new(
        "DebugUtilsObjectTagInfo",
        &[
            Struct::member("objectType", "VkObjectType"),
            Struct::member("objectHandle", "u64"),
            Struct::member("tagName", "u64"),
            Struct::member("tagSize", "u64"),
            Struct::member("pTag", "*const core::ffi::c_void"),
        ],
    )
    .stype(
        "DEBUG_UTILS_OBJECT_TAG_INFO",
        vk_ext_enum(129, 1) as _,
        StructUsage::Source,
    )
    .extensions(&[("EXT", "debug_utils")]),
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
            Struct::member("sampler", "VkSampler"),
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
    .extensions(&[("KHR", "maintenance3")])
    .promoted("1_1"),
    Struct::typed(
        "DescriptorUpdateTemplateCreateInfo",
        "DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO",
        vk_ext_enum(86, 0) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkDescriptorUpdateTemplateCreateFlagsKHR"),
            Struct::member("descriptorUpdateEntryCount", "u32"),
            Struct::member("pDescriptorUpdateEntries", "*const VkDescriptorUpdateTemplateEntryKHR"),
            Struct::member("templateType", "VkDescriptorUpdateTemplateTypeKHR"),
            Struct::member("descriptorSetLayout", "VkDescriptorSetLayout"),
            Struct::member("pipelineBindPoint", "VkPipelineBindPoint"),
            Struct::member("pipelineLayout", "VkPipelineLayout"),
            Struct::member("set", "u32"),
        ],
    )
    .extensions(&[("KHR", "descriptor_update_template")])
    .promoted("1_1"),
    Struct::new(
        "DescriptorUpdateTemplateEntry",
        &[
            Struct::member("dstBinding", "u32"),
            Struct::member("dstArrayElement", "u32"),
            Struct::member("descriptorCount", "u32"),
            Struct::member("descriptorType", "VkDescriptorType"),
            Struct::member("offset", "usize"),
            Struct::member("stride", "usize"),
        ],
    )
    .extensions(&[("KHR", "descriptor_update_template")])
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
        "DeviceGroupBindSparseInfo",
        "DEVICE_GROUP_BIND_SPARSE_INFO",
        vk_ext_enum(61, 6) as _,
        StructUsage::Source,
        &[
            Struct::member("resourceDeviceIndex", "u32"),
            Struct::member("memoryDeviceIndex", "u32"),
        ],
    )
    .extensions(&[("KHR", "device_group")])
    .promoted("1_1"),
    Struct::typed(
        "DeviceGroupCommandBufferBeginInfo",
        "DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO",
        vk_ext_enum(61, 4) as _,
        StructUsage::Source,
        &[Struct::member("deviceMask", "u32")],
    )
    .extensions(&[("KHR", "device_group")])
    .promoted("1_1"),
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
    .extensions(&[("KHR", "device_group_creation")])
    .promoted("1_1"),
    Struct::typed(
        "DeviceGroupPresentCapabilities",
        "DEVICE_GROUP_PRESENT_CAPABILITIES",
        vk_ext_enum(61, 7) as _,
        StructUsage::Sink,
        &[
            Struct::member("presentMask", "[u32; VK_MAX_DEVICE_GROUP_SIZE_KHR]"),
            Struct::member("modes", "VkDeviceGroupPresentModeFlagsKHR"),
        ],
    )
    .extensions(&[("KHR", "device_group"), ("KHR", "surface")]),
    Struct::typed(
        "DeviceGroupPresentInfo",
        "DEVICE_GROUP_PRESENT_INFO",
        vk_ext_enum(61, 11) as _,
        StructUsage::Source,
        &[
            Struct::member("swapchainCount", "u32"),
            Struct::member("pDeviceMasks", "*const u32"),
            Struct::member("mode", "VkDeviceGroupPresentModeFlagBitsKHR"),
        ],
    )
    .extensions(&[("KHR", "device_group"), ("KHR", "swapchain")]),
    Struct::typed(
        "DeviceGroupRenderPassBeginInfo",
        "DEVICE_GROUP_RENDER_PASS_BEGIN_INFO",
        vk_ext_enum(61, 3) as _,
        StructUsage::Source,
        &[
            Struct::member("deviceMask", "u32"),
            Struct::member("deviceRenderAreaCount", "u32"),
            Struct::member("pDeviceRenderAreas", "*const VkRect2D"),
        ],
    )
    .extensions(&[("KHR", "device_group")])
    .promoted("1_1"),
    Struct::typed(
        "DeviceGroupSubmitInfo",
        "DEVICE_GROUP_SUBMIT_INFO",
        vk_ext_enum(61, 5) as _,
        StructUsage::Source,
        &[
            Struct::member("waitSemaphoreCount", "u32"),
            Struct::member("pWaitSemaphoreDeviceIndices", "*const u32"),
            Struct::member("commandBufferCount", "u32"),
            Struct::member("pCommandBufferDeviceMasks", "*const u32"),
            Struct::member("signalSemaphoreCount", "u32"),
            Struct::member("pSignalSemaphoreDeviceIndices", "*const u32"),
        ],
    )
    .extensions(&[("KHR", "device_group")])
    .promoted("1_1"),
    Struct::typed(
        "DeviceGroupSwapchainCreateInfo",
        "DEVICE_GROUP_SWAPCHAIN_CREATE_INFO",
        vk_ext_enum(61, 12) as _,
        StructUsage::Source,
        &[Struct::member("modes", "VkDeviceGroupPresentModeFlagsKHR")],
    )
    .extensions(&[("KHR", "device_group"), ("KHR", "swapchain")]),
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
    .extensions(&[("KHR", "display")]),
    Struct::new(
        "DisplayModeParameters",
        &[
            Struct::member("visibleRegion", "VkExtent2D"),
            Struct::member("refreshRate", "u32"),
        ],
    )
    .extensions(&[("KHR", "display")]),
    Struct::new(
        "DisplayModeProperties",
        &[
            Struct::member("displayMode", "VkDisplayModeKHR"),
            Struct::member("parameters", "VkDisplayModeParametersKHR"),
        ],
    )
    .extensions(&[("KHR", "display")]),
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
    .extensions(&[("KHR", "display")]),
    Struct::new(
        "DisplayPlaneProperties",
        &[
            Struct::member("currentDisplay", "VkDisplayKHR"),
            Struct::member("currentStackIndex", "u32"),
        ],
    )
    .extensions(&[("KHR", "display")]),
    Struct::new(
        "DisplayPresentInfo",
        &[
            Struct::member("srcRect", "VkRect2D"),
            Struct::member("dstRect", "VkRect2D"),
            Struct::member("persistent", "VkBool32"),
        ],
    )
    .stype("DISPLAY_PRESENT_INFO", vk_ext_enum(4, 0) as _, StructUsage::Source)
    .extensions(&[("KHR", "display_swapchain")]),
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
    .extensions(&[("KHR", "display")]),
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
    .extensions(&[("KHR", "display")]),
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
        "ExportFenceCreateInfo",
        &[Struct::member("handleTypes", "VkExternalFenceHandleTypeFlagsKHR")],
    )
    .extensions(&[("KHR", "external_fence")])
    .promoted("1_1"),
    Struct::new(
        "ExportFenceWin32HandleInfo",
        &[
            Struct::member("pAttributes", "*const windows::Win32::Security::SECURITY_ATTRIBUTES"),
            Struct::member("dwAccess", "u32"),
            Struct::member("name", "windows::core::PCWSTR"),
        ],
    )
    .stype(
        "EXPORT_FENCE_WIN32_HANDLE_INFO",
        vk_ext_enum(115, 1) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_fence_win32")]),
    Struct::new(
        "VkExportMemoryAllocateInfo",
        &[Struct::member("handleTypes", "VkExternalMemoryHandleTypeFlagsKHR")],
    )
    .stype(
        "EXPORT_MEMORY_ALLOCATE_INFO",
        vk_ext_enum(73, 2) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_memory")])
    .promoted("1_1"),
    Struct::new(
        "ExportMemoryWin32HandleInfo",
        &[
            Struct::member("pAttributes", "*const windows::Win32::Security::SECURITY_ATTRIBUTES"),
            Struct::member("dwAccess", "u32"),
            Struct::member("name", "windows::core::PCWSTR"),
        ],
    )
    .stype(
        "EXPORT_MEMORY_WIN32_HANDLE_INFO",
        vk_ext_enum(74, 1) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_memory_win32")]),
    Struct::new(
        "ExportSemaphoreCreateInfo",
        &[Struct::member("handleType", "VkExternalSemaphoreHandleTypeFlagsKHR")],
    )
    .stype(
        "EXPORT_SEMAPHORE_CREATE_INFO",
        vk_ext_enum(78, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_semaphore")])
    .promoted("1_1"),
    Struct::new(
        "ExportSemaphoreWin32HandleInfo",
        &[
            Struct::member("pAttributes", "*const windows::Win32::Security::SECURITY_ATTRIBUTES"),
            Struct::member("dwAccess", "u32"),
            Struct::member("name", "windows::core::PCWSTR"),
        ],
    )
    .stype(
        "EXPORT_SEMAPHORE_WIN32_HANDLE_INFO",
        vk_ext_enum(79, 1) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_semaphore_win32")]),
    Struct::new(
        "ExtensionProperties",
        &[
            Struct::member(
                "extensionName",
                "crate::ffi_helper::FixedCStrBuffer<VK_MAX_EXTENSION_NAME_SIZE>",
            ),
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
    Struct::new(
        "ExternalBufferProperties",
        &[Struct::member(
            "externalMemoryProperties",
            "VkExternalMemoryPropertiesKHR",
        )],
    )
    .stype("EXTERNAL_BUFFER_PROPERTIES", vk_ext_enum(72, 3) as _, StructUsage::Sink)
    .extensions(&[("KHR", "external_memory_capabilities")])
    .promoted("1_1"),
    Struct::new(
        "ExternalFenceProperties",
        &[
            Struct::member("exportFromImportedHandleTypes", "VkExternalFenceHandleTypeFlagsKHR"),
            Struct::member("compatibleHandleTypes", "VkExternalFenceHandleTypeFlagsKHR"),
            Struct::member("externalFenceFeatures", "VkExternalFenceFeatureFlagsKHR"),
        ],
    )
    .stype("EXTERNAL_FENCE_PROPERTIES", vk_ext_enum(113, 1) as _, StructUsage::Sink)
    .extensions(&[("KHR", "external_fence_capabilities")])
    .promoted("1_1"),
    Struct::new(
        "ExternalImageFormatProperties",
        &[Struct::member(
            "externalMemoryProperties",
            "VkExternalMemoryPropertiesKHR",
        )],
    )
    .stype(
        "EXTERANL_IMAGE_FORMAT_PROPERTIES",
        vk_ext_enum(72, 1) as _,
        StructUsage::Sink,
    )
    .extensions(&[("KHR", "external_memory_capabilities")])
    .promoted("1_1"),
    Struct::new(
        "ExternalMemoryBufferCreateInfo",
        &[Struct::member("handleTypes", "VkExternalMemoryHandleTypeFlagsKHR")],
    )
    .stype(
        "EXTERNAL_MEMORY_BUFFER_CREATE_INFO",
        vk_ext_enum(73, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_memory")])
    .promoted("1_1"),
    Struct::new(
        "ExternalMemoryImageCreateInfo",
        &[Struct::member("handleTypes", "VkExternalMemoryHandleTypeFlagsKHR")],
    )
    .stype(
        "EXTERNAL_MEMORY_IMAGE_CREATE_INFO",
        vk_ext_enum(73, 1) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_memory")])
    .promoted("1_1"),
    Struct::new(
        "ExternalMemoryProperties",
        &[
            Struct::member("externalMemoryFeatures", "VkExternalMemoryFeatureFlagsKHR"),
            Struct::member("exportFromImportedHandleTypes", "VkExternalMemoryHandleTypeFlagsKHR"),
            Struct::member("compatibleHandleTypes", "VkExternalMemoryHandleTypeFlagsKHR"),
        ],
    )
    .extensions(&[("KHR", "external_memory_capabilities")])
    .promoted("1_1"),
    Struct::new(
        "ExternalSemaphoreProperties",
        &[
            Struct::member("exportFromImportedHandleTypes", "VkExternalSemaphoreHandleTypeFlagsKHR"),
            Struct::member("compatibleHandleTypes", "VkExternalSemaphoreHandleTypeFlagsKHR"),
            Struct::member("externalSemaphoreFeatures", "VkExternalSemaphoreFeatureFlagsKHR"),
        ],
    )
    .stype(
        "EXTERNAL_SEMAPHORE_PROPERTIES",
        vk_ext_enum(77, 1) as _,
        StructUsage::Sink,
    )
    .extensions(&[("KHR", "external_semaphore_capabilities")])
    .promoted("1_1"),
    Struct::new("FenceCreateInfo", &[Struct::member("flags", "VkFenceCreateFlags")]).stype(
        "FENCE_CREATE_INFO",
        8,
        StructUsage::Source,
    ),
    Struct::new(
        "FenceGetFdInfo",
        &[
            Struct::member("fence", "VkFence"),
            Struct::member("handleType", "VkExternalFenceHandleTypeFlagsKHR"),
        ],
    )
    .stype("FENCE_GET_FD_INFO", vk_ext_enum(116, 1) as _, StructUsage::Source)
    .extensions(&[("KHR", "external_fence_fd")]),
    Struct::new(
        "FenceGetWin32HandleInfo",
        &[
            Struct::member("fence", "VkFence"),
            Struct::member("handleType", "VkExternalFenceHandleTypeFlagsKHR"),
        ],
    )
    .stype(
        "FENCE_GET_WIN32_HANDLE_INFO",
        vk_ext_enum(115, 2) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_fence_win32")]),
    Struct::new(
        "FormatProperties",
        &[
            Struct::member("linearTilingFeatures", "VkFormatFeatureFlags"),
            Struct::member("optimalTilingFeatures", "VkFormatFeatureFlags"),
            Struct::member("bufferFeatures", "VkFormatFeatureFlags"),
        ],
    ),
    Struct::typed(
        "FormatProperties2",
        "FORMAT_PROPERTIES_2",
        vk_ext_enum(60, 2) as _,
        StructUsage::Sink,
        &[Struct::member("formatProperties", "VkFormatProperties")],
    )
    .extensions(&[("KHR", "get_physical_device_properties2")])
    .promoted("1_1"),
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
            Struct::member("layout", "VkPipelineLayout"),
            Struct::member("renderPass", "VkRenderPass"),
            Struct::member("subpass", "u32"),
            Struct::member("basePipelineHandle", "VkPipeline"),
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
        "ImageFormatProperties2",
        "IMAGE_FORMAT_PROPERTIES_2",
        vk_ext_enum(60, 3) as _,
        StructUsage::Sink,
        &[Struct::member("imageFormatProperties", "VkImageFormatProperties")],
    )
    .extensions(&[("KHR", "get_physical_device_properties2")])
    .promoted("1_1"),
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
    Struct::typed(
        "ImageMemoryRequirementsInfo2",
        "IMAGE_MEMORY_REQUIREMENTS_INFO_2",
        vk_ext_enum(147, 1) as _,
        StructUsage::Source,
        &[Struct::member("image", "VkImage")],
    )
    .extensions(&[("KHR", "get_memory_requirements2")])
    .promoted("1_1"),
    Struct::typed(
        "ImagePlaneMemoryRequirementsInfo",
        "IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO",
        vk_ext_enum(157, 2) as _,
        StructUsage::Source,
        &[Struct::member("planeAspect", "VkImageAspectFlagBits")],
    )
    .extensions(&[("KHR", "sampler_ycbcr_conversion")])
    .promoted("1_1"),
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
    Struct::typed(
        "ImageSparseMemoryRequirementsInfo2",
        "IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2",
        vk_ext_enum(147, 2) as _,
        StructUsage::Source,
        &[Struct::member("image", "VkImage")],
    )
    .extensions(&[("KHR", "get_memory_requirements2")])
    .promoted("1_1"),
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
    Struct::typed(
        "ImageSwapchainCreateInfo",
        "IMAGE_SWAPCHAIN_CREATE_INFO",
        vk_ext_enum(61, 8) as _,
        StructUsage::Source,
        &[Struct::member("swapchain", "VkSwapchainKHR")],
    )
    .extensions(&[("KHR", "device_group"), ("KHR", "swapchain")]),
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
    Struct::typed(
        "ImageViewUsageCreateInfo",
        "IMAGE_VIEW_USAGE_CREATE_INFO",
        vk_ext_enum(118, 2) as _,
        StructUsage::Source,
        &[
            Struct::member("sliceOffset", "u32"),
            Struct::member("sliceCount", "u32"),
        ],
    )
    .extensions(&[("KHR", "maintenance2")])
    .promoted("1_1"),
    Struct::new(
        "ImportFenceFdInfo",
        &[
            Struct::member("fence", "VkFence"),
            Struct::member("flags", "VkFenceImportFlagsKHR"),
            Struct::member("handleType", "VkExternalFenceHandleTypeFlagsKHR"),
            Struct::member("fd", "core::ffi::c_int"),
        ],
    )
    .stype("IMPORT_FENCE_FD_INFO", vk_ext_enum(116, 0) as _, StructUsage::Source)
    .extensions(&[("KHR", "external_fence_fd")]),
    Struct::new(
        "ImportFenceWin32HandleInfo",
        &[
            Struct::member("fence", "VkFence"),
            Struct::member("flags", "VkFenceImportFlagsKHR"),
            Struct::member("handleType", "VkExternalFenceHandleTypeFlagsKHR"),
            Struct::member("handle", "windows::Win32::Foundation::HANDLE"),
            Struct::member("name", "windows::core::PCWSTR"),
        ],
    )
    .stype(
        "IMPORT_FENCE_WIN32_HANDLE_INFO",
        vk_ext_enum(115, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_fence_win32")]),
    Struct::new(
        "ImportMemoryFdInfo",
        &[
            Struct::member("handleType", "VkExternalMemoryHandleTypeFlagsKHR"),
            Struct::member("fd", "core::ffi::c_int"),
        ],
    )
    .stype("IMPORT_MEMORY_FD_INFO", vk_ext_enum(75, 0) as _, StructUsage::Source)
    .extensions(&[("KHR", "external_memory_fd")]),
    Struct::new(
        "ImportMemoryWin32HandleInfo",
        &[
            Struct::member("handleType", "VkExternalMemoryHandleTypeFlagsKHR"),
            Struct::member("handle", "windows::Win32::Foundation::HANDLE"),
            Struct::member("name", "windows::core::PCWSTR"),
        ],
    )
    .stype(
        "IMPORT_MEMORY_WIN32_HANDLE_INFO",
        vk_ext_enum(74, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_memory_win32")]),
    Struct::new(
        "ImportSemaphoreFdInfo",
        &[
            Struct::member("semaphore", "VkSemaphore"),
            Struct::member("flags", "VkSemaphoreImportFlagsKHR"),
            Struct::member("handleType", "VkExternalSemaphoreHandleTypeFlagsKHR"),
            Struct::member("fd", "core::ffi::c_int"),
        ],
    )
    .stype("IMPORT_SEMAPHORE_FD_INFO", vk_ext_enum(80, 0) as _, StructUsage::Source)
    .extensions(&[("KHR", "external_semaphore_fd")]),
    Struct::new(
        "ImportSemaphoreWin32HandleInfo",
        &[
            Struct::member("semaphore", "VkSemaphore"),
            Struct::member("flags", "VkSemaphoreImportFlagsKHR"),
            Struct::member("handleType", "VkExternalSemaphoreHandleTypeFlagsKHR"),
            Struct::member("handle", "windows::Win32::Foundation::HANDLE"),
            Struct::member("name", "windows::core::PCWSTR"),
        ],
    )
    .stype(
        "IMPORT_SEMAPHORE_WIN32_HANDLE_INFO",
        vk_ext_enum(79, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_semaphore_win32")]),
    Struct::new(
        "InputAttachmentAspectReference",
        &[
            Struct::member("subpass", "u32"),
            Struct::member("inputAttachmentIndex", "u32"),
            Struct::member("aspectMask", "VkImageAspectFlags"),
        ],
    )
    .extensions(&[("KHR", "maintenance2")])
    .promoted("1_1"),
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
            Struct::member(
                "layerName",
                "crate::ffi_helper::FixedCStrBuffer<VK_MAX_EXTENSION_NAME_SIZE>",
            ),
            Struct::member("specVersion", "u32"),
            Struct::member("implementationVersion", "u32"),
            Struct::member(
                "description",
                "crate::ffi_helper::FixedCStrBuffer<VK_MAX_DESCRIPTION_SIZE>",
            ),
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
        "MemoryAllocateFlagsInfo",
        &[
            Struct::member("flags", "VkMemoryAllocateFlags"),
            Struct::member("deviceMask", "u32"),
        ],
    )
    .stype(
        "MEMORY_ALLOCATE_FLAGS_INFO",
        vk_ext_enum(61, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "device_group")])
    .promoted("1_1"),
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
    Struct::typed(
        "MemoryDedicatedAllocateInfo",
        "MEMORY_DEDICATED_ALLOCATE_INFO",
        vk_ext_enum(128, 1) as _,
        StructUsage::Source,
        &[Struct::member("image", "VkImage"), Struct::member("buffer", "VkBuffer")],
    )
    .extensions(&[("KHR", "dedicated_allocation")])
    .promoted("1_1"),
    Struct::typed(
        "MemoryDedicatedRequirements",
        "MEMORY_DEDICATED_REQUIREMENTS",
        vk_ext_enum(128, 0) as _,
        StructUsage::Sink,
        &[
            Struct::member("prefersDedicatedAllocation", "VkBool32"),
            Struct::member("requiresDedicatedAllocation", "VkBool32"),
        ],
    )
    .extensions(&[("KHR", "dedicated_allocation")])
    .promoted("1_1"),
    Struct::new("MemoryFdProperties", &[Struct::member("memoryTypeBits", "u32")])
        .stype("MEMORY_FD_PROPERTIES", vk_ext_enum(75, 1) as _, StructUsage::Sink)
        .extensions(&[("KHR", "external_memory_fd")]),
    Struct::new(
        "MemoryGetFdInfo",
        &[
            Struct::member("memory", "VkDeviceMemory"),
            Struct::member("handleType", "VkExternalMemoryHandleTypeFlagsKHR"),
        ],
    )
    .stype("MEMORY_GET_FD_INFO", vk_ext_enum(75, 2) as _, StructUsage::Source)
    .extensions(&[("KHR", "external_memory_fd")]),
    Struct::new(
        "MemoryGetWin32HandleInfo",
        &[
            Struct::member("memory", "VkDeviceMemory"),
            Struct::member("handleType", "VkExternalMemoryHandleTypeFlagsKHR"),
        ],
    )
    .stype(
        "MEMORY_GET_WIN32_HANDLE_INFO",
        vk_ext_enum(74, 3) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_memory_win32")]),
    Struct::new(
        "MemoryRequirements",
        &[
            Struct::member("size", DEVICE_SIZE_TYPE),
            Struct::member("alignment", DEVICE_SIZE_TYPE),
            Struct::member("memoryTypeBits", "u32"),
        ],
    ),
    Struct::typed(
        "MemoryRequirements2",
        "MEMORY_REQUIREMENTS_2",
        vk_ext_enum(147, 3) as _,
        StructUsage::Sink,
        &[Struct::member("memoryRequirements", "VkMemoryRequirements")],
    )
    .extensions(&[("KHR", "get_memory_requirements2")])
    .promoted("1_1"),
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
    Struct::new(
        "MemoryWin32HandleProperties",
        &[Struct::member("memoryTypeBits", "u32")],
    )
    .stype(
        "MEMORY_WIN32_HANDLE_PROPERTIES",
        vk_ext_enum(74, 2) as _,
        StructUsage::Sink,
    )
    .extensions(&[("KHR", "external_memory_win32")]),
    Struct::typed(
        "MetalSurfaceCreateInfo",
        "METAL_SURFACE_CREATE_INFO",
        vk_ext_enum(218, 0) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkMetalSurfaceCreateFlagsEXT"),
            Struct::member("pLayer", "*const core::ffi::c_void"),
        ],
    )
    .extensions(&[("EXT", "metal_surface")]),
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
    .extensions(&[("KHR", "16bit_storage")])
    .promoted("1_1"),
    Struct::new(
        "PhysicalDeviceExternalBufferInfo",
        &[
            Struct::member("flags", "VkBufferCreateFlags"),
            Struct::member("usage", "VkBufferUsageFlags"),
            Struct::member("handleType", "VkExternalMemoryHandleTypeFlagsKHR"),
        ],
    )
    .stype(
        "PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO",
        vk_ext_enum(72, 2) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_memory_capabilities")])
    .promoted("1_1"),
    Struct::new(
        "PhysicalDeviceExternalFenceInfo",
        &[Struct::member("handleType", "VkExternalFenceHandleTypeFlagsKHR")],
    )
    .stype(
        "PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO",
        vk_ext_enum(113, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_fence_capabilities")])
    .promoted("1_1"),
    Struct::new(
        "PhysicalDeviceExternalImageFormatInfo",
        &[Struct::member("handleType", "VkExternalMemoryHandleTypeFlagsKHR")],
    )
    .stype(
        "PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO",
        vk_ext_enum(72, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_memory_capabilities")])
    .promoted("1_1"),
    Struct::new(
        "PhysicalDeviceExternalSemaphoreInfo",
        &[Struct::member("handleType", "VkExternalSemaphoreHandleTypeFlagsKHR")],
    )
    .stype(
        "PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO",
        vk_ext_enum(77, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_semaphore_capabilities")])
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
    ),
    Struct::typed(
        "PhysicalDeviceFeatures2",
        "PHYSICAL_DEVICE_FEATURES_2",
        vk_ext_enum(60, 0) as _,
        StructUsage::Both,
        &[Struct::member("features", "VkPhysicalDeviceFeatures")],
    )
    .extensions(&[("KHR", "get_physical_device_properties2")])
    .promoted("1_1"),
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
    .extensions(&[("KHR", "device_group_creation")])
    .promoted("1_1"),
    Struct::typed(
        "PhysicalDeviceImageFormatInfo2",
        "PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2",
        vk_ext_enum(60, 4) as _,
        StructUsage::Source,
        &[
            Struct::member("format", "VkFormat"),
            Struct::member("r#type", "VkImageType"),
            Struct::member("tiling", "VkImageTiling"),
            Struct::member("usage", "VkImageUsageFlags"),
            Struct::member("flags", "VkImageCreateFlags"),
        ],
    )
    .extensions(&[("KHR", "get_physical_device_properties2")])
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
    .extensions(&[("KHR", "maintenance3")])
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
        "PhysicalDeviceMemoryProperties2",
        "PHYSICAL_DEVICE_MEMORY_PROPERTIES_2",
        vk_ext_enum(60, 6) as _,
        StructUsage::Sink,
        &[Struct::member("memoryProperties", "VkPhysicalDeviceMemoryProperties")],
    )
    .extensions(&[("KHR", "get_physical_device_properties2")])
    .promoted("1_1"),
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
    .extensions(&[("KHR", "multiview")])
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
    .extensions(&[("KHR", "multiview")])
    .promoted("1_1"),
    Struct::typed(
        "PhysicalDevicePointClippingProperties",
        "PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES",
        vk_ext_enum(118, 0) as _,
        StructUsage::Sink,
        &[Struct::member("pointClippingBehavior", "VkPointClippingBehaviorKHR")],
    )
    .extensions(&[("KHR", "maintenance2")])
    .promoted("1_1"),
    Struct::new(
        "PhysicalDeviceProperties",
        &[
            Struct::member("apiVersion", "u32"),
            Struct::member("driverVersion", "u32"),
            Struct::member("vendorID", "u32"),
            Struct::member("deviceID", "u32"),
            Struct::member("deviceType", "VkPhysicalDeviceType"),
            Struct::member(
                "deviceName",
                "crate::ffi_helper::FixedCStrBuffer<VK_MAX_PHYSICAL_DEVICE_NAME_SIZE>",
            ),
            Struct::member("pipelineCacheUUID", "[u8; VK_UUID_SIZE]"),
            Struct::member("limits", "VkPhysicalDeviceLimits"),
            Struct::member("sparseProperties", "VkPhysicalDeviceSparseProperties"),
        ],
    ),
    Struct::typed(
        "PhysicalDeviceProperties2",
        "PHYSICAL_DEVICE_PROPERTIES_2",
        vk_ext_enum(60, 1) as _,
        StructUsage::Sink,
        &[Struct::member("properties", "VkPhysicalDeviceProperties")],
    )
    .extensions(&[("KHR", "get_physical_device_properties2")])
    .promoted("1_1"),
    Struct::typed(
        "PhysicalDeviceSamplerYcbcrConversionFeatures",
        "PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES",
        vk_ext_enum(157, 4) as _,
        StructUsage::Both,
        &[Struct::member("samplerYcbcrConversion", "VkBool32")],
    )
    .extensions(&[("KHR", "sampler_ycbcr_conversion")])
    .promoted("1_1"),
    Struct::typed(
        "PhysicalDeviceSparseImageFormatInfo2",
        "PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2",
        vk_ext_enum(60, 8) as _,
        StructUsage::Source,
        &[
            Struct::member("format", "VkFormat"),
            Struct::member("r#type", "VkImageType"),
            Struct::member("samples", "VkSampleCountFlagBits"),
            Struct::member("usage", "VkImageUsageFlags"),
            Struct::member("tiling", "VkImageTiling"),
        ],
    )
    .extensions(&[("KHR", "get_physical_device_properties2")])
    .promoted("1_1"),
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
    .extensions(&[("KHR", "get_surface_capabilities2")]),
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
    .extensions(&[("KHR", "variable_pointers")])
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
            Struct::member("module", "VkShaderModule"),
            Struct::member("pName", "*const core::ffi::c_char"),
            Struct::member("pSpecializationInfo", "*const VkSpecializationInfo"),
        ],
    )
    .stype("PIPELINE_SHADER_STAGE_CREATE_INFO", 18, StructUsage::Source),
    Struct::typed(
        "PipelineTessellationDomainOriginStateCreateInfo",
        "PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO",
        vk_ext_enum(118, 3) as _,
        StructUsage::Source,
        &[Struct::member("domainOrigin", "VkTessellationDomainOriginKHR")],
    )
    .extensions(&[("KHR", "maintenance2")])
    .promoted("1_1"),
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
        "PresentInfo",
        &[
            Struct::member("waitSemaphoreCount", "u32"),
            Struct::member("pWaitSemaphores", "*const VkSemaphore"),
            Struct::member("swapchainCount", "u32"),
            Struct::member("pSwapchains", "*const VkSwapchainKHR"),
            Struct::member("pImageIndices", "*const u32"),
            Struct::member("pResults", "*mut VkResult"),
        ],
    )
    .stype("PRESENT_INFO", vk_ext_enum(2, 1) as _, StructUsage::Source)
    .extensions(&[("KHR", "swapchain")]),
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
    Struct::typed(
        "QueueFamilyProperties2",
        "QUEUE_FAMILY_PROPERTIES_2",
        vk_ext_enum(60, 5) as _,
        StructUsage::Sink,
        &[Struct::member("queueFamilyProperties", "VkQueueFamilyProperties")],
    )
    .extensions(&[("KHR", "get_physical_device_properties2")])
    .promoted("1_1"),
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
        "RenderPassInputAttachmentAspectCreateInfo",
        "RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO",
        vk_ext_enum(118, 1) as _,
        StructUsage::Source,
        &[
            Struct::member("aspectReferenceCount", "u32"),
            Struct::member("pAspectReferences", "*const VkInputAttachmentAspectReferenceKHR"),
        ],
    )
    .extensions(&[("KHR", "maintenance2")])
    .promoted("1_1"),
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
    .extensions(&[("KHR", "multiview")])
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
    Struct::typed(
        "SamplerYcbcrConversionCreateInfo",
        "SAMPLER_YCBCR_CONVERSION_CREATE_INFO",
        vk_ext_enum(157, 0) as _,
        StructUsage::Source,
        &[
            Struct::member("format", "VkFormat"),
            Struct::member("ycbcrModel", "VkSamplerYcbcrModelConversionKHR"),
            Struct::member("ycbcrRange", "VkSamplerYcbcrRangeKHR"),
            Struct::member("components", "VkComponentMapping"),
            Struct::member("xChromaOffset", "VkChromaLocationKHR"),
            Struct::member("yChromaOffset", "VkChromaLocationKHR"),
            Struct::member("chromaFilter", "VkFilter"),
            Struct::member("forceExplicitReconstruction", "VkBool32"),
        ],
    )
    .extensions(&[("KHR", "sampler_ycbcr_conversion")])
    .promoted("1_1"),
    Struct::typed(
        "SamplerYcbcrConversionImageFormatProperties",
        "SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES",
        vk_ext_enum(157, 5) as _,
        StructUsage::Sink,
        &[Struct::member("combinedImageSamplerDescriptorCount", "u32")],
    )
    .extensions(&[("KHR", "sampler_ycbcr_conversion")])
    .promoted("1_1"),
    Struct::typed(
        "SamplerYcbcrConversionInfo",
        "SAMPLER_YCBCR_CONVERSION_INFO",
        vk_ext_enum(157, 1) as _,
        StructUsage::Source,
        &[Struct::member("conversion", "VkSamplerYcbcrConversion")],
    )
    .extensions(&[("KHR", "sampler_ycbcr_conversion")])
    .promoted("1_1"),
    Struct::new(
        "SemaphoreCreateInfo",
        &[Struct::member("flags", "VkSemaphoreCreateFlags")],
    )
    .stype("SEMAPHORE_CREATE_INFO", 9, StructUsage::Source),
    Struct::new(
        "SemaphoreGetFdInfo",
        &[
            Struct::member("semaphore", "VkSemaphore"),
            Struct::member("handleType", "VkExternalSemaphoreHandleTypeFlagBitsKHR"),
        ],
    )
    .stype("SEMAPHORE_GET_FD_INFO", vk_ext_enum(80, 1) as _, StructUsage::Source)
    .extensions(&[("KHR", "external_semaphore_fd")]),
    Struct::new(
        "SemaphoreGetWin32HandleInfo",
        &[
            Struct::member("semaphore", "VkSemaphore"),
            Struct::member("handleType", "VkExternalSemaphoreHandleTypeFlagBitsKHR"),
        ],
    )
    .stype(
        "SEMAPHORE_GET_WIN32_HANDLE_INFO",
        vk_ext_enum(79, 3) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "external_semaphore_win32")]),
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
    Struct::typed(
        "SparseImageFormatProperties2",
        "SPARSE_IMAGE_FORMAT_PROPERTIES_2",
        vk_ext_enum(60, 7) as _,
        StructUsage::Sink,
        &[Struct::member("properties", "VkSparseImageFormatProperties")],
    )
    .extensions(&[("KHR", "get_physical_device_properties2")])
    .promoted("1_1"),
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
    Struct::typed(
        "SparseImageMemoryRequirements2",
        "SPARSE_IMAGE_MEMORY_REQUIREMENTS_2",
        vk_ext_enum(147, 4) as _,
        StructUsage::Sink,
        &[Struct::member("memoryRequirements", "VkSparseImageMemoryRequirements")],
    )
    .extensions(&[("KHR", "get_memory_requirements2")])
    .promoted("1_1"),
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
            Struct::member("reosurceOffset", DEVICE_SIZE_TYPE),
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
    Struct::new(
        "SurfaceCapabilities",
        &[
            Struct::member("minImageCount", "u32"),
            Struct::member("maxImageCount", "u32"),
            Struct::member("currentExtent", "VkExtent2D"),
            Struct::member("minImageExtent", "VkExtent2D"),
            Struct::member("maxImageExtent", "VkExtent2D"),
            Struct::member("maxImageArrayLayers", "u32"),
            Struct::member("supportedTransforms", "VkSurfaceTransformFlagsKHR"),
            Struct::member("currentTransform", "VkSurfaceTransformFlagBitsKHR"),
            Struct::member("supportedCompositeAlpha", "VkCompositeAlphaFlagsKHR"),
            Struct::member("supportedUsageFlags", "VkImageUsageFlags"),
        ],
    )
    .extensions(&[("KHR", "surface")]),
    Struct::typed(
        "SurfaceCapabilities2",
        "SURFACE_CAPABILITIES_2",
        vk_ext_enum(120, 1) as _,
        StructUsage::Sink,
        &[Struct::member("surfaceCapabilities", "VkSurfaceCapabilitiesKHR")],
    )
    .extensions(&[("KHR", "get_surface_capabilities2")]),
    Struct::new(
        "SurfaceFormat",
        &[
            Struct::member("format", "VkFormat"),
            Struct::member("colorSpace", "VkColorSpaceKHR"),
        ],
    )
    .copyable()
    .extensions(&[("KHR", "surface")]),
    Struct::typed(
        "SurfaceFormat2",
        "SURFACE_FORMAT_2",
        vk_ext_enum(120, 2) as _,
        StructUsage::Sink,
        &[Struct::member("surfaceFormat", "VkSurfaceFormatKHR")],
    )
    .extensions(&[("KHR", "get_surface_capabilities2")]),
    Struct::new(
        "SwapchainCreateInfo",
        &[
            Struct::member("flags", "VkSwapchainCreateFlagsKHR"),
            Struct::member("surface", "VkSurfaceKHR"),
            Struct::member("minImageCount", "u32"),
            Struct::member("imageFormat", "VkFormat"),
            Struct::member("imageColorSpace", "VkColorSpaceKHR"),
            Struct::member("imageExtent", "VkExtent2D"),
            Struct::member("imageArrayLayers", "u32"),
            Struct::member("imageUsage", "VkImageUsageFlags"),
            Struct::member("imageSharingMode", "VkSharingMode"),
            Struct::member("queueFamilyIndexCount", "u32"),
            Struct::member("pQueueFamilyIndices", "*const u32"),
            Struct::member("preTransform", "VkSurfaceTransformFlagBitsKHR"),
            Struct::member("compositeAlpha", "VkCompositeAlphaFlagBitsKHR"),
            Struct::member("presentMode", "VkPresentModeKHR"),
            Struct::member("clipped", "VkBool32"),
            Struct::member("oldSwapchain", "VkSwapchainKHR"),
        ],
    )
    .stype("SWAPCHAIN_CREATE_INFO", vk_ext_enum(2, 0) as _, StructUsage::Source)
    .extensions(&[("KHR", "swapchain")]),
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
        "WaylandSurfaceCreateInfo",
        &[
            Struct::member("flags", "VkWaylandSurfaceCreateFlagsKHR"),
            Struct::member("display", "*mut core::ffi::c_void"),
            Struct::member("surface", "*mut core::ffi::c_void"),
        ],
    )
    .stype(
        "WAYLAND_SURFACE_CREATE_INFO",
        vk_ext_enum(7, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "wayland_surface")]),
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
    .extensions(&[("KHR", "win32_keyed_mutex")]),
    Struct::new(
        "Win32SurfaceCreateInfo",
        &[
            Struct::member("flags", "VkWin32SurfaceCreateFlagsKHR"),
            Struct::member("hinstance", "windows::Win32::Foundation::HINSTANCE"),
            Struct::member("hwnd", "windows::Win32::Foundation::HWND"),
        ],
    )
    .stype(
        "WIN32_SURFACE_CREATE_INFO",
        vk_ext_enum(10, 0) as _,
        StructUsage::Source,
    )
    .extensions(&[("KHR", "win32_surface")]),
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
    Struct::new(
        "XcbSurfaceCreateInfo",
        &[
            Struct::member("flags", "VkXcbSurfaceCreateFlagsKHR"),
            Struct::member("connection", "*mut xcb::ffi::xcb_connection_t"),
            Struct::member("window", "xcb::x::Window"),
        ],
    )
    .stype("XCB_SURFACE_CREATE_INFO", vk_ext_enum(6, 0) as _, StructUsage::Source)
    .extensions(&[("KHR", "xcb_surface")]),
    Struct::new(
        "XlibSurfaceCreateInfo",
        &[
            Struct::member("flags", "VkXlibSurfaceCreateFlagsKHR"),
            Struct::member("dpy", "*mut x11::xlib::Display"),
            Struct::member("window", "x11::xlib::Window"),
        ],
    )
    .stype("XLIB_SURFACE_CREATE_INFO", vk_ext_enum(5, 0) as _, StructUsage::Source)
    .extensions(&[("KHR", "xlib_surface")]),
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
    .extensions(&[("KHR", "synchronization2")])
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
    .extensions(&[("KHR", "synchronization2")])
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
    .extensions(&[("KHR", "synchronization2")])
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
    .extensions(&[("KHR", "synchronization2")])
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
    .extensions(&[("KHR", "synchronization2")])
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
    .extensions(&[("KHR", "synchronization2")])
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
    .extensions(&[("KHR", "synchronization2")])
    .promoted("1_3"),
    Struct::typed(
        "PhysicalDeviceSynchronization2Features",
        "PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES",
        vk_ext_enum(315, 7) as _,
        StructUsage::Both,
        &[Struct::member("synchronization2", "VkBool32")],
    )
    .extensions(&[("KHR", "synchronization2")])
    .promoted("1_3"),
];

const UNIONS: &'static [Union] = &[
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

const COMMANDS: &'static [Command] = &[
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
            ("fence", "VkFence"),
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
            ("fence", "VkFence"),
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
            ("pipelineCache", "VkPipelineCache"),
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
            ("pipelineCache", "VkPipelineCache"),
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
        "DestroySurface",
        &[
            ("instance", "VkInstance"),
            ("surface", "VkSurfaceKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable()
    .extension("KHR", "surface"),
    Command::new(
        "GetPhysicalDeviceSurfaceSupport",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("queueFamilyIndex", "u32"),
            ("surface", "VkSurfaceKHR"),
            ("pSupported", "*mut VkBool32"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "surface"),
    Command::new(
        "GetPhysicalDeviceSurfaceCapabilities",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("surface", "VkSurfaceKHR"),
            ("pSurfaceCapabilities", "*mut VkSurfaceCapabilitiesKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "surface"),
    Command::new(
        "GetPhysicalDeviceSurfaceFormats",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("surface", "VkSurfaceKHR"),
            ("pSurfaceFormatsCount", "*mut u32"),
            ("pSurfaceFormats", "*mut VkSurfaceFormatKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "surface"),
    Command::new(
        "GetPhysicalDeviceSurfacePresentModes",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("surface", "VkSurfaceKHR"),
            ("pPresentModeCount", "*mut u32"),
            ("pPresentModes", "*mut VkPresentModeKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "surface"),
    Command::new(
        "CreateSwapchain",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkSwapchainCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSwapchain", "*mut VkSwapchainKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "swapchain"),
    Command::new(
        "DestroySwapchain",
        &[
            ("device", "VkDevice"),
            ("swapchain", "VkSwapchainKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .static_callable()
    .extension("KHR", "swapchain"),
    Command::new(
        "GetSwapchainImages",
        &[
            ("device", "VkDevice"),
            ("swapchain", "VkSwapchainKHR"),
            ("pSwapchainImageCount", "*mut u32"),
            ("pSwapchainImages", "*mut VkImage"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "swapchain"),
    Command::new(
        "AcquireNextImage",
        &[
            ("device", "VkDevice"),
            ("swapchain", "VkSwapchainKHR"),
            ("timeout", "u64"),
            ("semaphore", "VkSemaphore"),
            ("fence", "VkFence"),
            ("pImageIndex", "*mut u32"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "swapchain"),
    Command::new(
        "QueuePresent",
        &[("queue", "VkQueue"), ("pPresentInfo", "*const VkPresentInfoKHR")],
    )
    .failable()
    .static_callable()
    .extension("KHR", "swapchain"),
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
    .extension("KHR", "display"),
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
    .extension("KHR", "display"),
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
    .extension("KHR", "display"),
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
    .extension("KHR", "display"),
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
    .extension("KHR", "display"),
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
    .extension("KHR", "display"),
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
    .extension("KHR", "display"),
    Command::new(
        "CreateSharedSwapchain",
        &[
            ("device", "VkDevice"),
            ("swapchainCount", "u32"),
            ("pCreateInfos", "*const VkSwapchainCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSwapchains", "*mut VkSwapchainKHR"),
        ],
    )
    .failable()
    .extension("KHR", "display_swapchain"),
    Command::new(
        "CreateXlibSurface",
        &[
            ("instance", "VkInstance"),
            ("pCreateInfo", "*const VkXlibSurfaceCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSurface", "*mut VkSurfaceKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "xlib_surface"),
    Command::new(
        "GetPhysicalDeviceXlibPresentationSupport",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("queueFamilyIndex", "u32"),
            ("dpy", "*mut x11::xlib::Display"),
            ("visualID", "x11::xlib::VisualID"),
        ],
    )
    .returns("VkBool32")
    .static_callable()
    .extension("KHR", "xlib_surface"),
    Command::new(
        "CreateXcbSurface",
        &[
            ("instance", "VkInstance"),
            ("pCreateInfo", "*const VkXcbSurfaceCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSurface", "*mut VkSurfaceKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "xcb_surface"),
    Command::new(
        "GetPhysicalDeviceXcbPresentationSupport",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("queueFamilyIndex", "u32"),
            ("connection", "*mut xcb::ffi::xcb_connection_t"),
            ("visual_id", "xcb::x::Visualid"),
        ],
    )
    .returns("VkBool32")
    .static_callable()
    .extension("KHR", "xcb_surface"),
    Command::new(
        "CreateWaylandSurface",
        &[
            ("instance", "VkInstance"),
            ("pCreateInfo", "*const VkWaylandSurfaceCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSurface", "*mut VkSurfaceKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "wayland_surface"),
    Command::new(
        "GetPhysicalDeviceWaylandPresentationSupport",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("queueFamilyIndex", "u32"),
            ("display", "*mut core::ffi::c_void"),
        ],
    )
    .returns("VkBool32")
    .static_callable()
    .extension("KHR", "wayland_surface"),
    Command::new(
        "CreateAndroidSurface",
        &[
            ("instance", "VkInstance"),
            ("pCreateInfo", "*const VkAndroidSurfaceCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSurface", "*mut VkSurfaceKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "android_surface"),
    Command::new(
        "CreateWin32Surface",
        &[
            ("instance", "VkInstance"),
            ("pCreateInfo", "*const VkWin32SurfaceCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSurface", "*mut VkSurfaceKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension("KHR", "win32_surface"),
    Command::new(
        "GetPhysicalDeviceWin32PresentationSupport",
        &[("physicalDevice", "VkPhysicalDevice"), ("queueFamilyIndex", "u32")],
    )
    .returns("VkBool32")
    .static_callable()
    .extension("KHR", "win32_surface"),
    Command::new(
        "CreateMetalSurface",
        &[
            ("instance", "VkInstance"),
            ("pCreateInfo", "*const VkMetalSurfaceCreateInfoEXT"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pSurface", "*mut VkSurfaceKHR"),
        ],
    )
    .failable()
    .static_callable()
    .extension("EXT", "metal_surface"),
    Command::new(
        "GetMemoryWin32Handle",
        &[
            ("device", "VkDevice"),
            ("pGetWin32HandleInfo", "*const VkMemoryGetWin32HandleInfoKHR"),
            ("pHandle", "*mut windows::Win32::Foundation::HANDLE"),
        ],
    )
    .failable()
    .extension("KHR", "external_memory_win32"),
    Command::new(
        "GetMemoryWin32HandleProperties",
        &[
            ("device", "VkDevice"),
            ("handleType", "VkExternalMemoryHandleTypeFlagsKHR"),
            ("handle", "windows::Win32::Foundation::HANDLE"),
            ("pMemoryWin32HandleProperties", "*mut VkMemoryWin32HandlePropertiesKHR"),
        ],
    )
    .failable()
    .extension("KHR", "external_memory_win32"),
    Command::new(
        "GetMemoryFd",
        &[
            ("device", "VkDevice"),
            ("pGetFdInfo", "*const VkMemoryGetFdInfoKHR"),
            ("pFd", "*mut core::ffi::c_int"),
        ],
    )
    .failable()
    .extension("KHR", "external_memory_fd"),
    Command::new(
        "GetMemoryFdProperties",
        &[
            ("device", "VkDevice"),
            ("handleType", "VkExternalMemoryHandleTypeFlagsKHR"),
            ("fd", "core::ffi::c_int"),
            ("pMemoryFdProperties", "*mut VkMemoryFdPropertiesKHR"),
        ],
    )
    .failable()
    .extension("KHR", "external_memory_fd"),
    Command::new(
        "ImportSemaphoreWin32Handle",
        &[
            ("device", "VkDevice"),
            (
                "pImportSemaphoreWin32HandleInfo",
                "*const VkImportSemaphoreWin32HandleInfoKHR",
            ),
        ],
    )
    .failable()
    .extension("KHR", "external_semaphore_win32"),
    Command::new(
        "GetSemaphoreWin32Handle",
        &[
            ("device", "VkDevice"),
            ("pGetWin32HandleInfo", "*const VkSemaphoreGetWin32HandleInfoKHR"),
            ("pHandle", "*mut windows::Win32::Foundation::HANDLE"),
        ],
    )
    .failable()
    .extension("KHR", "external_semaphore_win32"),
    Command::new(
        "ImportSemaphoreFd",
        &[
            ("device", "VkDevice"),
            ("pImportSemaphoreFdInfo", "*const VkImportSemaphoreFdInfoKHR"),
        ],
    )
    .failable()
    .extension("KHR", "external_semaphore_fd"),
    Command::new(
        "GetSemaphoreFd",
        &[
            ("device", "VkDevice"),
            ("pGetFdInfo", "*const VkSemaphoreGetFdInfoKHR"),
            ("pFd", "*mut core::ffi::c_int"),
        ],
    )
    .failable()
    .extension("KHR", "external_semaphore_fd"),
    Command::new(
        "ImportFenceWin32Handle",
        &[
            ("device", "VkDevice"),
            ("pImportFenceWin32HandleInfo", "*const VkImportFenceWin32HandleInfoKHR"),
        ],
    )
    .failable()
    .extension("KHR", "external_fence_win32"),
    Command::new(
        "GetFenceWin32Handle",
        &[
            ("device", "VkDevice"),
            ("pGetWin32HandleInfo", "*const VkFenceGetWin32HandleInfoKHR"),
            ("pHandle", "*mut windows::Win32::Foundation::HANDLE"),
        ],
    )
    .failable()
    .extension("KHR", "external_fence_win32"),
    Command::new(
        "ImportFenceFd",
        &[
            ("device", "VkDevice"),
            ("pImportFenceFdInfo", "*const VkImportFenceFdInfoKHR"),
        ],
    )
    .failable()
    .extension("KHR", "external_fence_fd"),
    Command::new(
        "GetFenceFd",
        &[
            ("device", "VkDevice"),
            ("pGetFdInfo", "*const VkFenceGetFdInfoKHR"),
            ("pFd", "*mut core::ffi::c_int"),
        ],
    )
    .failable()
    .extension("KHR", "external_fence_fd"),
    Command::new(
        "GetPhysicalDeviceSurfaceCapabilities2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pSurfaceInfo", "*const VkPhysicalDeviceSurfaceInfo2KHR"),
            ("pSurfaceCapabilities", "*mut VkSurfaceCapabilities2KHR"),
        ],
    )
    .failable()
    .extension("KHR", "get_surface_capabilities2"),
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
    .extension("KHR", "get_surface_capabilities2"),
    Command::new(
        "CreateDebugReportCallback",
        &[
            ("instance", "VkInstance"),
            ("pCreateInfo", "*const VkDebugReportCallbackCreateInfoEXT"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pCallback", "*mut VkDebugReportCallbackEXT"),
        ],
    )
    .failable()
    .extension("EXT", "debug_report"),
    Command::new(
        "DestroyDebugReportCallback",
        &[
            ("instance", "VkInstance"),
            ("callback", "VkDebugReportCallbackEXT"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .extension("EXT", "debug_report"),
    Command::new(
        "DebugReportMessage",
        &[
            ("instance", "VkInstance"),
            ("flags", "VkDebugReportFlagsEXT"),
            ("objectType", "VkDebugReportObjectTypeEXT"),
            ("object", "u64"),
            ("location", "usize"),
            ("messageCode", "i32"),
            ("pLayerPrefix", "*const core::ffi::c_char"),
            ("pMessage", "*const core::ffi::c_char"),
        ],
    )
    .extension("EXT", "debug_report"),
    Command::new(
        "SetDebugUtilsObjectName",
        &[
            ("device", "VkDevice"),
            ("pNameInfo", "*const VkDebugUtilsObjectNameInfoEXT"),
        ],
    )
    .failable()
    .extension("EXT", "debug_utils"),
    Command::new(
        "SetDebugUtilsObjectTag",
        &[
            ("device", "VkDevice"),
            ("pTagInfo", "*const VkDebugUtilsObjectTagInfoEXT"),
        ],
    )
    .failable()
    .extension("EXT", "debug_utils"),
    Command::new(
        "QueueBeginDebugUtilsLabel",
        &[("queue", "VkQueue"), ("pLabelInfo", "*const VkDebugUtilsLabelEXT")],
    )
    .extension("EXT", "debug_utils"),
    Command::new("QueueEndDebugUtilsLabel", &[("queue", "VkQueue")]).extension("EXT", "debug_utils"),
    Command::new(
        "QueueInsertDebugUtilsLabel",
        &[("queue", "VkQueue"), ("pLabelInfo", "*const VkDebugUtilsLabelEXT")],
    )
    .extension("EXT", "debug_utils"),
    Command::new(
        "CreateDebugUtilsMessenger",
        &[
            ("instance", "VkInstance"),
            ("pCreateInfo", "*const VkDebugUtilsMessengerCreateInfoEXT"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pDebugUtilsMessenger", "*mut VkDebugUtilsMessengerEXT"),
        ],
    )
    .failable()
    .extension("EXT", "debug_utils"),
    Command::new(
        "DestroyDebugUtilsMessenger",
        &[
            ("instance", "VkInstance"),
            ("debugUtilsMessenger", "VkDebugUtilsMessengerEXT"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .extension("EXT", "debug_utils"),
    Command::new(
        "SubmitDebugUtilsMessage",
        &[
            ("instance", "VkInstance"),
            ("messageSeverity", "VkDebugUtilsMessageSeverityFlagBitsEXT"),
            ("messageTypes", "VkDebugUtilsMessageTypeFlagsEXT"),
            ("pCallbackData", "*const VkDebugUtilsMessengerCallbackDataEXT"),
        ],
    )
    .extension("EXT", "debug_utils"),
    Command::new(
        "GetPhysicalDeviceExternalBufferProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pExternalBufferInfo", "*const VkPhysicalDeviceExternalBufferInfoKHR"),
            ("pExternalBufferProperties", "*mut VkExternalBufferPropertiesKHR"),
        ],
    )
    .extension("KHR", "external_memory_capabilities")
    .promoted("1_1"),
    Command::new(
        "GetPhysicalDeviceExternalSemaphoreProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            (
                "pExternalSemaphoreInfo",
                "*const VkPhysicalDeviceExternalSemaphoreInfoKHR",
            ),
            ("pExternalSemaphoreProperties", "*mut VkExternalSemaphorePropertiesKHR"),
        ],
    )
    .extension("KHR", "external_semaphore_capabilities")
    .promoted("1_1"),
    Command::new(
        "GetPhysicalDeviceExternalFenceProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pExternalFenceInfo", "*const VkPhysicalDeviceExternalFenceInfoKHR"),
            ("pExternalFenceProperties", "*mut VkExternalFencePropertiesKHR"),
        ],
    )
    .extension("KHR", "external_fence_capabilities")
    .promoted("1_1"),
    Command::new(
        "GetDeviceGroupPeerMemoryFeatures",
        &[
            ("device", "VkDevice"),
            ("heapIndex", "u32"),
            ("localDeviceIndex", "u32"),
            ("remoteDeviceIndex", "u32"),
            ("pPeerMemoryFeatures", "*mut VkPeerMemoryFeatureFlags"),
        ],
    )
    .extension("KHR", "device_group")
    .promoted("1_1"),
    Command::new(
        "GetDeviceGroupPresentCapabilities",
        &[
            ("device", "VkDevice"),
            (
                "pDeviceGroupPresentCapabilities",
                "*mut VkDeviceGroupPresentCapabilitiesKHR",
            ),
        ],
    )
    .failable()
    .extension("KHR", "device_group")
    .extra_requirements(&["VK_KHR_surface"]),
    Command::new(
        "GetDeviceGroupSurfacePresentModes",
        &[
            ("device", "VkDevice"),
            ("surface", "VkSurfaceKHR"),
            ("pModes", "*mut VkDeviceGroupPresentModeFlagsKHR"),
        ],
    )
    .failable()
    .extension("KHR", "device_group")
    .extra_requirements(&["VK_KHR_surface"]),
    Command::new(
        "GetPhysicalDevicePresentRectangles",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("surface", "VkSurfaceKHR"),
            ("pRectCount", "*mut u32"),
            ("pRects", "*mut VkRect2D"),
        ],
    )
    .failable()
    .extension("KHR", "device_group")
    .extra_requirements(&["VK_KHR_surface"]),
    Command::new(
        "AcquireNextImage2",
        &[
            ("device", "VkDevice"),
            ("pAcquireInfo", "*const VkAcquireNextImageInfoKHR"),
            ("pImageIndex", "*mut u32"),
        ],
    )
    .failable()
    .extension("KHR", "device_group")
    .extra_requirements(&["VK_KHR_swapchain"]),
    Command::new(
        "GetPhysicalDeviceFeatures2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pFeatures", "*mut VkPhysicalDeviceFeatures2KHR"),
        ],
    )
    .extension("KHR", "get_physical_device_properties2")
    .promoted("1_1"),
    Command::new(
        "GetPhysicalDeviceProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pProperties", "*mut VkPhysicalDeviceProperties2KHR"),
        ],
    )
    .extension("KHR", "get_physical_device_properties2")
    .promoted("1_1"),
    Command::new(
        "GetPhysicalDeviceFormatProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("format", "VkFormat"),
            ("pFormatProperties", "*mut VkFormatProperties2KHR"),
        ],
    )
    .extension("KHR", "get_physical_device_properties2")
    .promoted("1_1"),
    Command::new(
        "GetPhysicalDeviceImageFormatProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pImageFormatInfo", "*const VkPhysicalDeviceImageFormatInfo2KHR"),
            ("pImageFormatProperties", "*mut VkImageFormatProperties2KHR"),
        ],
    )
    .failable()
    .extension("KHR", "get_physical_device_properties2")
    .promoted("1_1"),
    Command::new(
        "GetPhysicalDeviceQueueFamilyProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pQueueFamilyPropertyCount", "*mut u32"),
            ("pQueueFamilyProperties", "*mut VkQueueFamilyProperties2KHR"),
        ],
    )
    .extension("KHR", "get_physical_device_properties2")
    .promoted("1_1"),
    Command::new(
        "GetPhysicalDeviceMemoryProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pMemoryProperties", "*mut VkPhysicalDeviceMemoryProperties2KHR"),
        ],
    )
    .extension("KHR", "get_physical_device_properties2")
    .promoted("1_1"),
    Command::new(
        "GetPhysicalDeviceSparseImageFormatProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pFormatInfo", "*const VkPhysicalDeviceSparseImageFormatInfo2KHR"),
            ("pPropertyCount", "*mut u32"),
            ("pProperties", "*mut VkSparseImageFormatProperties2KHR"),
        ],
    )
    .extension("KHR", "get_physical_device_properties2")
    .promoted("1_1"),
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
    .extension("KHR", "device_group_creation")
    .promoted("1_1"),
    Command::new(
        "GetImageMemoryRequirements2",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkImageMemoryRequirementsInfo2KHR"),
            ("pMemoryRequirements", "*mut VkMemoryRequirements2KHR"),
        ],
    )
    .extension("KHR", "get_memory_requirements2")
    .promoted("1_1"),
    Command::new(
        "GetBufferMemoryRequirements2",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkBufferMemoryRequirementsInfo2KHR"),
            ("pMemoryRequirements", "*mut VkMemoryRequirements2KHR"),
        ],
    )
    .extension("KHR", "get_memory_requirements2")
    .promoted("1_1"),
    Command::new(
        "GetImageSparseMemoryRequirements2",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkImageSparseMemoryRequirementsInfo2KHR"),
            ("pSparseMemoryRequirementCount", "*mut u32"),
            ("pSparseMemoryRequirements", "*mut VkSparseImageMemoryRequirements2KHR"),
        ],
    )
    .extension("KHR", "get_memory_requirements2")
    .promoted("1_1"),
    Command::new(
        "BindBufferMemory2",
        &[
            ("device", "VkDevice"),
            ("bindInfoCount", "u32"),
            ("pBindInfos", "*const VkBindBufferMemoryInfoKHR"),
        ],
    )
    .failable()
    .extension("KHR", "bind_memory2")
    .promoted("1_1"),
    Command::new(
        "BindImageMemory2",
        &[
            ("device", "VkDevice"),
            ("bindInfoCount", "u32"),
            ("pBindInfos", "*const VkBindImageMemoryInfoKHR"),
        ],
    )
    .failable()
    .extension("KHR", "bind_memory2")
    .promoted("1_1"),
    Command::new(
        "CreateDescriptorUpdateTemplate",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkDescriptorUpdateTemplateCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pDescriptorUpdateTemplate", "*mut VkDescriptorUpdateTemplateKHR"),
        ],
    )
    .failable()
    .extension("KHR", "descriptor_update_template")
    .promoted("1_1"),
    Command::new(
        "DestroyDescriptorUpdateTemplate",
        &[
            ("device", "VkDevice"),
            ("descriptorUpdateTemplate", "VkDescriptorUpdateTemplateKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .extension("KHR", "descriptor_update_template")
    .promoted("1_1"),
    Command::new(
        "UpdateDescriptorSetWithTemplate",
        &[
            ("device", "VkDevice"),
            ("descriptorSet", "VkDescriptorSet"),
            ("descriptorUpdateTemplate", "VkDescriptorUpdateTemplateKHR"),
            ("pData", "*const core::ffi::c_void"),
        ],
    )
    .extension("KHR", "descriptor_update_template")
    .promoted("1_1"),
    Command::new(
        "CreateSamplerYcbcrConversion",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "VkSamplerYcbcrConversionCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pYcbcrConversion", "*mut VkSamplerYcbcrConversionKHR"),
        ],
    )
    .failable()
    .extension("KHR", "sampler_ycbcr_conversion")
    .promoted("1_1"),
    Command::new(
        "DestroySamplerYcbcrConversion",
        &[
            ("device", "VkDevice"),
            ("ycbcrConversion", "VkSamplerYcbcrConversionKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .extension("KHR", "sampler_ycbcr_conversion")
    .promoted("1_1"),
    Command::new(
        "TrimCommandPool",
        &[
            ("device", "VkDevice"),
            ("commandPool", "VkCommandPool"),
            ("flags", "VkCommandPoolTrimFlagsKHR"),
        ],
    )
    .extension("KHR", "maintenance1")
    .promoted("1_1"),
    Command::new(
        "GetDescriptorSetLayoutSupport",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkDescriptorSetLayoutCreateInfo"),
            ("pSupport", "*mut VkDescriptorSetLayoutSupport"),
        ],
    )
    .extension("KHR", "maintenance3")
    .promoted("1_1"),
    Command::new(
        "QueueSubmit2",
        &[
            ("queue", "VkQueue"),
            ("submitCount", "u32"),
            ("pSubmits", "*const VkSubmitInfo2KHR"),
            ("fence", "VkFence"),
        ],
    )
    .failable()
    .extension("KHR", "synchronization2")
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
    Command::inst("BeginDebugUtilsLabel", &[("pLabelInfo", "*const VkDebugUtilsLabelEXT")])
        .extension("EXT", "debug_utils"),
    Command::inst("EndDebugUtilsLabel", &[]).extension("EXT", "debug_utils"),
    Command::inst(
        "InsertDebugUtilsLabel",
        &[("pLabelInfo", "*const VkDebugUtilsLabelEXT")],
    )
    .extension("EXT", "debug_utils"),
    Command::inst("SetDeviceMask", &[("deviceMask", "u32")])
        .extension("KHR", "device_group")
        .promoted("1_1"),
    Command::inst(
        "DispatchBase",
        &[
            ("baseGroupX", "u32"),
            ("baseGroupY", "u32"),
            ("baseGroupZ", "u32"),
            ("groupCountX", "u32"),
            ("groupCountY", "u32"),
            ("groupCountZ", "u32"),
        ],
    )
    .extension("KHR", "device_group")
    .promoted("1_1"),
    Command::inst(
        "SetEvent2",
        &[("event", "VkEvent"), ("pDependencyInfo", "*const VkDependencyInfoKHR")],
    )
    .extension("KHR", "synchronization2")
    .promoted("1_3"),
    Command::inst(
        "ResetEvent2",
        &[("event", "VkEvent"), ("stageMask", "VkPipelineStageFlags2KHR")],
    )
    .extension("KHR", "synchronization2")
    .promoted("1_3"),
    Command::inst(
        "WaitEvents2",
        &[
            ("eventCount", "u32"),
            ("pEvents", "*const VkEvent"),
            ("pDependencyInfos", "*const VkDependencyInfoKHR"),
        ],
    )
    .extension("KHR", "synchronization2")
    .promoted("1_3"),
    Command::inst("PipelineBarrier2", &[("pDependencyInfo", "*const VkDependencyInfoKHR")])
        .extension("KHR", "synchronization2")
        .promoted("1_3"),
    Command::inst(
        "WriteTimestamp2",
        &[
            ("stage", "VkPipelineStageFlags2KHR"),
            ("queryPool", "VkQueryPool"),
            ("query", "u32"),
        ],
    )
    .extension("KHR", "synchronization2")
    .promoted("1_3"),
];

fn emit_result_type(w: &mut impl std::io::Write) -> std::io::Result<()> {
    writeln!(w, "#[repr(transparent)]")?;
    writeln!(w, "#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]")?;
    writeln!(w, "pub struct VkResult(pub i32);")?;

    // base defines
    emit_result_const(w, "VK_SUCCESS", 0, 0)?;
    emit_result_const(w, "VK_NOT_READY", 0, 1)?;
    emit_result_const(w, "VK_TIMEOUT", 0, 2)?;
    emit_result_const(w, "VK_EVENT_SET", 0, 3)?;
    emit_result_const(w, "VK_EVENT_RESET", 0, 4)?;
    emit_result_const(w, "VK_INCOMPLETE", 0, 5)?;
    emit_result_err_const(w, "VK_ERROR_OUT_OF_HOST_MEMORY", 0, 1)?;
    emit_result_err_const(w, "VK_ERROR_OUT_OF_DEVICE_MEMORY", 0, 2)?;
    emit_result_err_const(w, "VK_ERROR_INITIALIZATION_FAILED", 0, 3)?;
    emit_result_err_const(w, "VK_ERROR_DEVICE_LOST", 0, 4)?;
    emit_result_err_const(w, "VK_ERROR_MEMORY_MAP_FAILED", 0, 5)?;
    emit_result_err_const(w, "VK_ERROR_LAYER_NOT_PRESENT", 0, 6)?;
    emit_result_err_const(w, "VK_ERROR_EXTENSION_NOT_PRESENT", 0, 7)?;
    emit_result_err_const(w, "VK_ERROR_FEATURE_NOT_PRESENT", 0, 8)?;
    emit_result_err_const(w, "VK_ERROR_INCOMPATIBLE_DRIVER", 0, 9)?;
    emit_result_err_const(w, "VK_ERROR_TOO_MANY_OBJECTS", 0, 10)?;
    emit_result_err_const(w, "VK_ERROR_FORMAT_NOT_SUPPORTED", 0, 11)?;
    emit_result_err_const(w, "VK_ERROR_FRAGMENTED_POOL", 0, 12)?;
    emit_result_err_const(w, "VK_ERROR_UNKNOWN", 0, 13)?;

    // from extensions
    w.write(b"#[cfg(feature = \"VK_KHR_surface\")]\n")?;
    emit_result_err_const(w, "VK_ERROR_SURFACE_LOST_KHR", 1, 0)?;
    w.write(b"#[cfg(feature = \"VK_KHR_surface\")]\n")?;
    emit_result_err_const(w, "VK_ERROR_NATIVE_WINDOW_IN_USE_KHR", 1, 1)?;
    w.write(b"#[cfg(feature = \"VK_KHR_swapchain\")]\n")?;
    emit_result_const(w, "VK_SUBOPTIMAL_KHR", 2, 3)?;
    w.write(b"#[cfg(feature = \"VK_KHR_swapchain\")]\n")?;
    emit_result_err_const(w, "VK_ERROR_OUT_OF_DATE_KHR", 2, 4)?;
    w.write(b"#[cfg(feature = \"VK_KHR_display_swapchain\")]\n")?;
    emit_result_err_const(w, "VK_ERROR_INCOMPATIBLE_DISPLAY_KHR", 4, 1)?;

    // from promoted extensions
    w.write(b"#[cfg(feature = \"VK_KHR_external_memory\")]\n")?;
    emit_result_err_const(w, "VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR", 73, 3)?;
    w.write(b"#[cfg(feature = \"Allow1_1APIs\")]\n")?;
    emit_result_err_const(w, "VK_ERROR_INVALID_EXTERNAL_HANDLE", 73, 3)?;
    w.write(b"#[cfg(feature = \"VK_KHR_maintenance1\")]\n")?;
    emit_result_err_const(w, "VK_ERROR_OUT_OF_POOL_MEMORY_KHR", 70, 0)?;
    w.write(b"#[cfg(feature = \"Allow1_1APIs\")]\n")?;
    emit_result_err_const(w, "VK_ERROR_OUT_OF_POOL_MEMORY", 70, 0)?;

    Ok(())
}

fn emit_format_enum(w: &mut impl Write) -> std::io::Result<()> {
    emit_c_enum_type(w, "VkFormat")?;
    emit_const(w, "VK_FORMAT_UNDEFINED", "VkFormat", "0")?;

    // packed format
    fn packed(w: &mut impl Write, bit_assign: &str, r#repr: &str, size: usize, value: usize) -> std::io::Result<()> {
        writeln!(
            w,
            "pub const VK_FORMAT_{bit_assign}_{repr}_PACK{size}: VkFormat = {value};"
        )
    }
    fn packed_s(
        w: &mut impl Write,
        bit_assign: &str,
        r#repr: &str,
        size: usize,
        suffix: &str,
        value: usize,
    ) -> std::io::Result<()> {
        writeln!(
            w,
            "pub const VK_FORMAT_{bit_assign}_{repr}_PACK{size}_{suffix}: VkFormat = {value};"
        )
    }

    packed(w, "R4G4", "UNORM", 8, 1)?;
    packed(w, "R4G4B4A4", "UNORM", 16, 2)?;
    packed(w, "B4G4R4A4", "UNORM", 16, 3)?;
    packed(w, "R5G6B5", "UNORM", 16, 4)?;
    packed(w, "B5G6R5", "UNORM", 16, 5)?;
    packed(w, "R5G5B5A1", "UNORM", 16, 6)?;
    packed(w, "B5G5R5A1", "UNORM", 16, 7)?;
    packed(w, "A1R5G5B5", "UNORM", 16, 8)?;

    // straight format
    fn f(w: &mut impl Write, bit_assign: &str, r#repr: &str, value: usize) -> std::io::Result<()> {
        writeln!(w, "pub const VK_FORMAT_{bit_assign}_{repr}: VkFormat = {value};")
    }
    fn f2(
        w: &mut impl Write,
        bit_assign1: &str,
        r#repr1: &str,
        bit_assign2: &str,
        r#repr2: &str,
        value: usize,
    ) -> std::io::Result<()> {
        writeln!(
            w,
            "pub const VK_FORMAT_{bit_assign1}_{repr1}_{bit_assign2}_{repr2}: VkFormat = {value};"
        )
    }
    f(w, "R8", "UNORM", 9)?;
    f(w, "R8", "SNORM", 10)?;
    f(w, "R8", "USCALED", 11)?;
    f(w, "R8", "SSCALED", 12)?;
    f(w, "R8", "UINT", 13)?;
    f(w, "R8", "SINT", 14)?;
    f(w, "R8", "SRGB", 15)?;
    f(w, "R8G8", "UNORM", 16)?;
    f(w, "R8G8", "SNORM", 17)?;
    f(w, "R8G8", "USCALED", 18)?;
    f(w, "R8G8", "SSCALED", 19)?;
    f(w, "R8G8", "UINT", 20)?;
    f(w, "R8G8", "SINT", 21)?;
    f(w, "R8G8", "SRGB", 22)?;
    f(w, "R8G8B8", "UNORM", 23)?;
    f(w, "R8G8B8", "SNORM", 24)?;
    f(w, "R8G8B8", "USCALED", 25)?;
    f(w, "R8G8B8", "SSCALED", 26)?;
    f(w, "R8G8B8", "UINT", 27)?;
    f(w, "R8G8B8", "SINT", 28)?;
    f(w, "R8G8B8", "SRGB", 29)?;
    f(w, "B8G8R8", "UNORM", 30)?;
    f(w, "B8G8R8", "SNORM", 31)?;
    f(w, "B8G8R8", "USCALED", 32)?;
    f(w, "B8G8R8", "SSCALED", 33)?;
    f(w, "B8G8R8", "UINT", 34)?;
    f(w, "B8G8R8", "SINT", 35)?;
    f(w, "B8G8R8", "SRGB", 36)?;
    f(w, "R8G8B8A8", "UNORM", 37)?;
    f(w, "R8G8B8A8", "SNORM", 38)?;
    f(w, "R8G8B8A8", "USCALED", 39)?;
    f(w, "R8G8B8A8", "SSCALED", 40)?;
    f(w, "R8G8B8A8", "UINT", 41)?;
    f(w, "R8G8B8A8", "SINT", 42)?;
    f(w, "R8G8B8A8", "SRGB", 43)?;
    f(w, "B8G8R8A8", "UNORM", 44)?;
    f(w, "B8G8R8A8", "SNORM", 45)?;
    f(w, "B8G8R8A8", "USCALED", 46)?;
    f(w, "B8G8R8A8", "SSCALED", 47)?;
    f(w, "B8G8R8A8", "UINT", 48)?;
    f(w, "B8G8R8A8", "SINT", 49)?;
    f(w, "B8G8R8A8", "SRGB", 50)?;
    packed(w, "A8B8G8R8", "UNORM", 32, 51)?;
    packed(w, "A8B8G8R8", "SNORM", 32, 52)?;
    packed(w, "A8B8G8R8", "USCALED", 32, 53)?;
    packed(w, "A8B8G8R8", "SSCALED", 32, 54)?;
    packed(w, "A8B8G8R8", "UINT", 32, 55)?;
    packed(w, "A8B8G8R8", "SINT", 32, 56)?;
    packed(w, "A8B8G8R8", "SRGB", 32, 57)?;
    packed(w, "A2R10G10B10", "UNORM", 32, 58)?;
    packed(w, "A2R10G10B10", "SNORM", 32, 59)?;
    packed(w, "A2R10G10B10", "USCALED", 32, 60)?;
    packed(w, "A2R10G10B10", "SSCALED", 32, 61)?;
    packed(w, "A2R10G10B10", "UINT", 32, 62)?;
    packed(w, "A2R10G10B10", "SINT", 32, 63)?;
    packed(w, "A2B10G10R10", "UNORM", 32, 64)?;
    packed(w, "A2B10G10R10", "SNORM", 32, 65)?;
    packed(w, "A2B10G10R10", "USCALED", 32, 66)?;
    packed(w, "A2B10G10R10", "SSCALED", 32, 67)?;
    packed(w, "A2B10G10R10", "UINT", 32, 68)?;
    packed(w, "A2B10G10R10", "SINT", 32, 69)?;
    f(w, "R16", "UNORM", 70)?;
    f(w, "R16", "SNORM", 71)?;
    f(w, "R16", "USCALED", 72)?;
    f(w, "R16", "SSCALED", 73)?;
    f(w, "R16", "UINT", 74)?;
    f(w, "R16", "SINT", 75)?;
    f(w, "R16", "SFLOAT", 76)?;
    f(w, "R16G16", "UNORM", 77)?;
    f(w, "R16G16", "SNORM", 78)?;
    f(w, "R16G16", "USCALED", 79)?;
    f(w, "R16G16", "SSCALED", 80)?;
    f(w, "R16G16", "UINT", 81)?;
    f(w, "R16G16", "SINT", 82)?;
    f(w, "R16G16", "SFLOAT", 83)?;
    f(w, "R16G16B16", "UNORM", 84)?;
    f(w, "R16G16B16", "SNORM", 85)?;
    f(w, "R16G16B16", "USCALED", 86)?;
    f(w, "R16G16B16", "SSCALED", 87)?;
    f(w, "R16G16B16", "UINT", 88)?;
    f(w, "R16G16B16", "SINT", 89)?;
    f(w, "R16G16B16", "SFLOAT", 90)?;
    f(w, "R16G16B16A16", "UNORM", 91)?;
    f(w, "R16G16B16A16", "SNORM", 92)?;
    f(w, "R16G16B16A16", "USCALED", 93)?;
    f(w, "R16G16B16A16", "SSCALED", 94)?;
    f(w, "R16G16B16A16", "UINT", 95)?;
    f(w, "R16G16B16A16", "SINT", 96)?;
    f(w, "R16G16B16A16", "SFLOAT", 97)?;
    f(w, "R32", "UINT", 98)?;
    f(w, "R32", "SINT", 99)?;
    f(w, "R32", "SFLOAT", 100)?;
    f(w, "R32G32", "UINT", 101)?;
    f(w, "R32G32", "SINT", 102)?;
    f(w, "R32G32", "SFLOAT", 103)?;
    f(w, "R32G32B32", "UINT", 104)?;
    f(w, "R32G32B32", "SINT", 105)?;
    f(w, "R32G32B32", "SFLOAT", 106)?;
    f(w, "R32G32B32A32", "UINT", 107)?;
    f(w, "R32G32B32A32", "SINT", 108)?;
    f(w, "R32G32B32A32", "SFLOAT", 109)?;
    f(w, "R64", "UINT", 110)?;
    f(w, "R64", "SINT", 111)?;
    f(w, "R64", "SFLOAT", 112)?;
    f(w, "R64G64", "UINT", 113)?;
    f(w, "R64G64", "SINT", 114)?;
    f(w, "R64G64", "SFLOAT", 115)?;
    f(w, "R64G64B64", "UINT", 116)?;
    f(w, "R64G64B64", "SINT", 117)?;
    f(w, "R64G64B64", "SFLOAT", 118)?;
    f(w, "R64G64B64A64", "UINT", 119)?;
    f(w, "R64G64B64A64", "SINT", 120)?;
    f(w, "R64G64B64A64", "SFLOAT", 121)?;
    packed(w, "B10G11R11", "UFLOAT", 32, 122)?;
    packed(w, "E5B9G9R9", "UFLOAT", 32, 123)?;
    f(w, "D16", "UNORM", 124)?;
    packed(w, "X8_D24", "UNORM", 32, 125)?;
    f(w, "D32", "SFLOAT", 126)?;
    f(w, "S8", "UINT", 127)?;
    f2(w, "D16", "UNORM", "S8", "UINT", 128)?;
    f2(w, "D24", "UNORM", "S8", "UINT", 129)?;
    f2(w, "D32", "SFLOAT", "S8", "UINT", 130)?;

    // compressed formats
    fn bc(w: &mut impl std::io::Write, variant: u8, order_repr: &str, value: usize) -> std::io::Result<()> {
        writeln!(
            w,
            "pub const VK_FORMAT_BC{variant}_{order_repr}_BLOCK: VkFormat = {value};",
        )
    }
    bc(w, 1, "RGB_UNORM", 131)?;
    bc(w, 1, "RGB_SRGB", 132)?;
    bc(w, 1, "RGBA_UNORM", 133)?;
    bc(w, 1, "RGBA_SRGB", 134)?;
    bc(w, 2, "UNORM", 135)?;
    bc(w, 2, "SRGB", 136)?;
    bc(w, 3, "UNORM", 137)?;
    bc(w, 3, "SRGB", 138)?;
    bc(w, 4, "UNORM", 139)?;
    bc(w, 4, "SNORM", 140)?;
    bc(w, 5, "UNORM", 141)?;
    bc(w, 5, "SNORM", 142)?;
    w.write(b"pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: VkFormat = 143;\n")?;
    w.write(b"pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: VkFormat = 144;\n")?;
    bc(w, 7, "UNORM", 145)?;
    bc(w, 7, "SRGB", 146)?;

    fn etc2(w: &mut impl std::io::Write, bit_assign: &str, r#repr: &str, value: usize) -> std::io::Result<()> {
        writeln!(
            w,
            "pub const VK_FORMAT_ETC2_{bit_assign}_{repr}_BLOCK: VkFormat = {value};"
        )
    }
    etc2(w, "R8G8B8", "UNORM", 147)?;
    etc2(w, "R8G8B8", "SRGB", 148)?;
    etc2(w, "R8G8B8A1", "UNORM", 149)?;
    etc2(w, "R8G8B8A1", "SRGB", 150)?;
    etc2(w, "R8G8B8A8", "UNORM", 151)?;
    etc2(w, "R8G8B8A8", "SRGB", 152)?;

    fn eac(w: &mut impl std::io::Write, bit_assign: &str, r#repr: &str, value: usize) -> std::io::Result<()> {
        writeln!(
            w,
            "pub const VK_FORMAT_EAC_{bit_assign}_{repr}_BLOCK: VkFormat = {value};"
        )
    }
    eac(w, "R11", "UNORM", 153)?;
    eac(w, "R11", "SNORM", 154)?;
    eac(w, "R11G11", "UNORM", 155)?;
    eac(w, "R11G11", "SNORM", 156)?;

    fn astc(w: &mut impl std::io::Write, bw: u8, bh: u8, r#repr: &str, value: usize) -> std::io::Result<()> {
        writeln!(
            w,
            "pub const VK_FORMAT_ASTC_{bw}x{bh}_{repr}_BLOCK: VkFormat = {value};"
        )
    }
    astc(w, 4, 4, "UNORM", 157)?;
    astc(w, 4, 4, "SRGB", 158)?;
    astc(w, 5, 4, "UNORM", 159)?;
    astc(w, 5, 4, "SRGB", 160)?;
    astc(w, 5, 5, "UNORM", 161)?;
    astc(w, 5, 5, "SRGB", 162)?;
    astc(w, 6, 5, "UNORM", 163)?;
    astc(w, 6, 5, "SRGB", 164)?;
    astc(w, 6, 6, "UNORM", 165)?;
    astc(w, 6, 6, "SRGB", 166)?;
    astc(w, 8, 5, "UNORM", 167)?;
    astc(w, 8, 5, "SRGB", 168)?;
    astc(w, 8, 6, "UNORM", 169)?;
    astc(w, 8, 6, "SRGB", 170)?;
    astc(w, 8, 8, "UNORM", 171)?;
    astc(w, 8, 8, "SRGB", 172)?;
    astc(w, 10, 5, "UNORM", 173)?;
    astc(w, 10, 5, "SRGB", 174)?;
    astc(w, 10, 6, "UNORM", 175)?;
    astc(w, 10, 6, "SRGB", 176)?;
    astc(w, 10, 8, "UNORM", 177)?;
    astc(w, 10, 8, "SRGB", 178)?;
    astc(w, 10, 10, "UNORM", 179)?;
    astc(w, 10, 10, "SRGB", 180)?;
    astc(w, 12, 10, "UNORM", 181)?;
    astc(w, 12, 10, "SRGB", 182)?;
    astc(w, 12, 12, "UNORM", 183)?;
    astc(w, 12, 12, "SRGB", 184)?;

    // sampler_ycbcr_conversion
    fn ycbcr(
        w: &mut impl std::io::Write,
        bit_assign: &str,
        dim: &str,
        r#repr: &str,
        value: i32,
    ) -> std::io::Result<()> {
        w.write(b"#[cfg(feature = \"VK_KHR_sampler_ycbcr_conversion\")]\n")?;
        writeln!(
            w,
            "pub const VK_FORMAT_{bit_assign}_{dim}_{repr}_KHR: VkFormat = {value};"
        )?;
        w.write(b"#[cfg(feature = \"Allow1_1APIs\")]\n")?;
        writeln!(w, "pub const VK_FORMAT_{bit_assign}_{dim}_{repr}: VkFormat = {value};")?;

        Ok(())
    }
    fn ycbcr_planes(
        w: &mut impl std::io::Write,
        bit_assign_per_plane: &[&str],
        dim: &str,
        r#repr: &str,
        value: i32,
    ) -> std::io::Result<()> {
        let bit_assign = bit_assign_per_plane.join("_");
        let plane_count = bit_assign_per_plane.len();
        w.write(b"#[cfg(feature = \"VK_KHR_sampler_ycbcr_conversion\")]\n")?;
        writeln!(
            w,
            "pub const VK_FORMAT_{bit_assign}_{plane_count}PLANE_{dim}_{repr}_KHR: VkFormat = {value};"
        )?;
        w.write(b"#[cfg(feature = \"Allow1_1APIs\")]\n")?;
        writeln!(
            w,
            "pub const VK_FORMAT_{bit_assign}_{plane_count}PLANE_{dim}_{repr}: VkFormat = {value};"
        )?;

        Ok(())
    }
    ycbcr(w, "G8B8G8R8", "422", "UNORM", vk_ext_enum(157, 0))?;
    ycbcr(w, "B8G8R8G8", "422", "UNORM", vk_ext_enum(157, 1))?;
    ycbcr_planes(w, &["G8", "B8", "R8"], "420", "UNORM", vk_ext_enum(157, 2))?;
    ycbcr_planes(w, &["G8", "B8R8"], "420", "UNORM", vk_ext_enum(157, 3))?;
    ycbcr_planes(w, &["G8", "B8", "R8"], "422", "UNORM", vk_ext_enum(157, 4))?;
    ycbcr_planes(w, &["G8", "B8R8"], "422", "UNORM", vk_ext_enum(157, 5))?;
    ycbcr_planes(w, &["G8", "B8", "R8"], "444", "UNORM", vk_ext_enum(157, 6))?;
    w.write(b"#[cfg(feature = \"VK_KHR_sampler_ycbcr_conversion\")]\n")?;
    packed_s(w, "R10X6", "UNORM", 16, "KHR", vk_ext_enum(157, 7) as _)?;
    w.write(b"#[cfg(feature = \"Allow1_1APIs\")]\n")?;
    packed(w, "R10X6", "UNORM", 16, vk_ext_enum(157, 7) as _)?;
    writeln!(
        w,
        r#"#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR: VkFormat = {v};
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: VkFormat = {v};"#,
        v = vk_ext_enum(157, 8)
    )?;
    writeln!(
        w,
        r#"#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: VkFormat = {v};
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: VkFormat = {v};"#,
        v = vk_ext_enum(157, 9)
    )?;
    writeln!(
        w,
        r#"#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: VkFormat = {v};
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: VkFormat = {v};"#,
        v = vk_ext_enum(157, 10)
    )?;
    writeln!(
        w,
        r#"#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: VkFormat = {v};
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: VkFormat = {v};"#,
        v = vk_ext_enum(157, 11)
    )?;
    ycbcr_planes(
        w,
        &["G10X6", "B10X6", "R10X6"],
        "420",
        "UNORM_3PACK16",
        vk_ext_enum(157, 12),
    )?;
    ycbcr_planes(
        w,
        &["G10X6", "B10X6R10X6"],
        "420",
        "UNORM_3PACK16",
        vk_ext_enum(157, 13),
    )?;
    ycbcr_planes(
        w,
        &["G10X6", "B10X6", "R10X6"],
        "422",
        "UNORM_3PACK16",
        vk_ext_enum(157, 14),
    )?;
    ycbcr_planes(
        w,
        &["G10X6", "B10X6R10X6"],
        "422",
        "UNORM_3PACK16",
        vk_ext_enum(157, 15),
    )?;
    ycbcr_planes(
        w,
        &["G10X6", "B10X6", "R10X6"],
        "444",
        "UNORM_3PACK16",
        vk_ext_enum(157, 16),
    )?;
    w.write(b"#[cfg(feature = \"VK_KHR_sampler_ycbcr_conversion\")]\n")?;
    packed_s(w, "R12X4", "UNORM", 16, "KHR", vk_ext_enum(157, 17) as _)?;
    w.write(b"#[cfg(feature = \"Allow1_1APIs\")]\n")?;
    packed(w, "R12X4", "UNORM", 16, vk_ext_enum(157, 17) as _)?;
    writeln!(
        w,
        r#"#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR: VkFormat = {v};
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: VkFormat = {v};"#,
        v = vk_ext_enum(157, 18)
    )?;
    writeln!(
        w,
        r#"#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: VkFormat = {v};
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: VkFormat = {v};"#,
        v = vk_ext_enum(157, 19)
    )?;
    writeln!(
        w,
        r#"#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: VkFormat = {v};
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: VkFormat = {v};"#,
        v = vk_ext_enum(157, 20)
    )?;
    writeln!(
        w,
        r#"#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: VkFormat = {v};
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: VkFormat = {v};"#,
        v = vk_ext_enum(157, 21)
    )?;
    ycbcr_planes(
        w,
        &["G12X4", "B12X4", "R12X4"],
        "420",
        "UNORM_3PACK16",
        vk_ext_enum(157, 22),
    )?;
    ycbcr_planes(
        w,
        &["G12X4", "B12X4R12X4"],
        "420",
        "UNORM_3PACK16",
        vk_ext_enum(157, 23),
    )?;
    ycbcr_planes(
        w,
        &["G12X4", "B12X4", "R12X4"],
        "422",
        "UNORM_3PACK16",
        vk_ext_enum(157, 24),
    )?;
    ycbcr_planes(
        w,
        &["G12X4", "B12X4R12X4"],
        "422",
        "UNORM_3PACK16",
        vk_ext_enum(157, 25),
    )?;
    ycbcr_planes(
        w,
        &["G12X4", "B12X4", "R12X4"],
        "444",
        "UNORM_3PACK16",
        vk_ext_enum(157, 26),
    )?;
    ycbcr(w, "G16B16G16R16", "422", "UNORM", vk_ext_enum(157, 27))?;
    ycbcr(w, "B16G16R16G16", "422", "UNORM", vk_ext_enum(157, 28))?;
    ycbcr_planes(w, &["G16", "B16", "R16"], "420", "UNORM", vk_ext_enum(157, 29))?;
    ycbcr_planes(w, &["G16", "B16R16"], "420", "UNORM", vk_ext_enum(157, 30))?;
    ycbcr_planes(w, &["G16", "B16", "R16"], "422", "UNORM", vk_ext_enum(157, 31))?;
    ycbcr_planes(w, &["G16", "B16R16"], "422", "UNORM", vk_ext_enum(157, 32))?;
    ycbcr_planes(w, &["G16", "B16", "R16"], "444", "UNORM", vk_ext_enum(157, 33))?;

    Ok(())
}
