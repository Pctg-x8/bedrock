use crate::{parts::*, vk_ext_enum};

pub const ELEMENTS: &[Element] = &[
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_acquire_drm_display", 1)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_acquire_xlib_display", 1)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_blend_operation_advanced", 2)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_AMD_buffer_marker", 1)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_NV_acquire_winrt_display", 1)),
    Element::Enum(Enum::extending(
        "BlendOp",
        "BLEND_OP",
        &[
            Enum::member("ZERO", vk_ext_enum(149, 0) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SRC", vk_ext_enum(149, 1) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DST", vk_ext_enum(149, 2) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SRC_OVER", vk_ext_enum(149, 3) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DST_OVER", vk_ext_enum(149, 4) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SRC_IN", vk_ext_enum(149, 5) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DST_IN", vk_ext_enum(149, 6) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SRC_OUT", vk_ext_enum(149, 7) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DST_OUT", vk_ext_enum(149, 8) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SRC_ATOP", vk_ext_enum(149, 9) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DST_ATOP", vk_ext_enum(149, 10) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("XOR", vk_ext_enum(149, 11) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("MULTIPLY", vk_ext_enum(149, 12) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SCREEN", vk_ext_enum(149, 13) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("OVERLAY", vk_ext_enum(149, 14) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DARKEN", vk_ext_enum(149, 15) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("LIGHTEN", vk_ext_enum(149, 16) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("COLORDODGE", vk_ext_enum(149, 17) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("COLORBURN", vk_ext_enum(149, 18) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HARDLIGHT", vk_ext_enum(149, 19) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SOFTLIGHT", vk_ext_enum(149, 20) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DIFFERENCE", vk_ext_enum(149, 21) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("EXCLUSION", vk_ext_enum(149, 22) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("INVERT", vk_ext_enum(149, 23) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("INVERT_RGB", vk_ext_enum(149, 24) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("LINEARDODGE", vk_ext_enum(149, 25) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("LINEARBURN", vk_ext_enum(149, 26) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("VIVIDLIGHT", vk_ext_enum(149, 27) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("LINEARLIGHT", vk_ext_enum(149, 28) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("PINLIGHT", vk_ext_enum(149, 29) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HARDMIX", vk_ext_enum(149, 30) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HSL_HUE", vk_ext_enum(149, 31) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HSL_SATURATION", vk_ext_enum(149, 32) as _)
                .extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HSL_COLOR", vk_ext_enum(149, 33) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HSL_LUMINOSITY", vk_ext_enum(149, 34) as _)
                .extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("PLUS", vk_ext_enum(149, 35) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("PLUS_CLAMPED", vk_ext_enum(149, 36) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("PLUS_CLAMPED_ALHPA", vk_ext_enum(149, 37) as _)
                .extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("PLUS_DARKER", vk_ext_enum(149, 38) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("MINUS", vk_ext_enum(149, 39) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("MINUS_CLAMPED", vk_ext_enum(149, 40) as _)
                .extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("CONTRAST", vk_ext_enum(149, 41) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("INVERT_OVG", vk_ext_enum(149, 42) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("RED", vk_ext_enum(149, 43) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("GREEN", vk_ext_enum(149, 44) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("BLUE", vk_ext_enum(149, 45) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
        ],
    )),
    Element::Enum(
        Enum::new(
            "BlendOverlap",
            "BLEND_OVERLAP",
            &[
                Enum::member("UNCORRELATED", 0).extension("VK_EXT_blend_operation_advanced", "EXT"),
                Enum::member("DISJOINT", 1).extension("VK_EXT_blend_operation_advanced", "EXT"),
                Enum::member("CONJOINT", 2).extension("VK_EXT_blend_operation_advanced", "EXT"),
            ],
        )
        .extension("VK_EXT_blend_operation_advanced", "EXT"),
    ),
    Element::Struct(
        Struct::new(
            "PhysicalDeviceBlendOperationAdvanccedFeatures",
            &[Struct::member("advancedBlendCoherentOperations", "VkBool32")],
        )
        .stype(
            "PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES",
            vk_ext_enum(149, 0) as _,
            StructUsage::Both,
        )
        .extensions(&[("EXT", "blend_operation_advanced")]),
    ),
    Element::Struct(
        Struct::new(
            "PhysicalDeviceBlendOperationAdvancedProperties",
            &[
                Struct::member("advancedBlendMaxColorAttachments", "u32"),
                Struct::member("advancedBlendIndependentBlend", "VkBool32"),
                Struct::member("advancedBlendNonPremultipliedSrcColor", "VkBool32"),
                Struct::member("advancedBlendNonPremultipliedDstColor", "VkBool32"),
                Struct::member("advancedBlendCorrelatedOverlap", "VkBool32"),
                Struct::member("advancedBlendAllOperations", "VkBool32"),
            ],
        )
        .stype(
            "PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES",
            vk_ext_enum(149, 1) as _,
            StructUsage::Sink,
        )
        .extensions(&[("EXT", "blend_operation_advanced")]),
    ),
    Element::Struct(
        Struct::new(
            "PipelineColorBlendAdvancedStateCreateInfo",
            &[
                Struct::member("srcPremultiplied", "VkBool32"),
                Struct::member("dstPremultiplied", "VkBool32"),
                Struct::member("blendOverlap", "VkBlendOverlapEXT"),
            ],
        )
        .stype(
            "PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO",
            vk_ext_enum(149, 2) as _,
            StructUsage::Source,
        )
        .extensions(&[("EXT", "blend_operation_advanced")]),
    ),
    Element::Command(
        Command::new(
            "AcquireDrmDisplay",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("drmFd", "i32"),
                ("display", "VkDisplayKHR"),
            ],
        )
        .failable()
        .extension("EXT", "acquire_drm_display"),
    ),
    Element::Command(
        Command::new(
            "AcquireWinrtDisplay",
            &[("physicalDevice", "VkPhysicalDevice"), ("display", "VkDisplayKHR")],
        )
        .failable()
        .extension("NV", "acquire_winrt_display"),
    ),
    Element::Command(
        Command::new(
            "AcquireXlibDisplay",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("dpy", "*mut x11::xlib::Display"),
                ("display", "VkDisplayKHR"),
            ],
        )
        .failable()
        .extension("EXT", "acquire_xlib_display"),
    ),
    Element::Command(
        Command::new(
            "GetDrmDisplay",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("drmFd", "i32"),
                ("connectorId", "u32"),
                ("pDisplay", "*mut VkDisplayKHR"),
            ],
        )
        .failable()
        .extension("EXT", "acquire_drm_display"),
    ),
    Element::Command(
        Command::new(
            "GetRandROutputDisplay",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("dpy", "*mut x11::xlib::Display"),
                ("rrOutput", "x11::xrandr::RROutput"),
                ("pDisplay", "*mut VkDisplayKHR"),
            ],
        )
        .failable()
        .extension("EXT", "acquire_xlib_display"),
    ),
    Element::Command(
        Command::new(
            "GetWinrtDisplay",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("deviceRelativeId", "u32"),
                ("pDisplay", "*mut VkDisplayKHR"),
            ],
        )
        .failable()
        .extension("NV", "acquire_winrt_display"),
    ),
    Element::Command(
        Command::inst(
            "WriteBufferMarker",
            &[
                ("pipelineStage", "VkPipelineStageFlags"),
                ("dstBuffer", "VkBuffer"),
                ("dstOffset", "VkDeviceSize"),
                ("marker", "u32"),
            ],
        )
        .extension("AMD", "buffer_marker"),
    ),
];
