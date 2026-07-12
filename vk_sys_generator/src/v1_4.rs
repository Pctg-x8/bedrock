use crate::{parts::*, vk_ext_enum};

const VERSION: &str = "1_4";
const VK_KHR_LOAD_STORE_OP_NONE: &Extension = &Extension::khr("load_store_op_none", 1);
const VK_KHR_SHADER_EXPECT_ASSUME: &Extension = &Extension::khr("shader_expect_assume", 1);
const VK_KHR_SHADER_FLOAT_CONTROLS2: &Extension = &Extension::khr("shader_float_controls2", 1);

pub const ELEMENTS: &[Element] = &[
    // VK_KHR_vertex_attribute_divisor
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_vertex_attribute_divisor", 1)),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceVertexAttributeDivisorProperties",
            "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES",
            vk_ext_enum(191, 0) as _,
            StructUsage::Sink,
            &[
                Struct::member("maxVertexAttribDivisor", "u32"),
                Struct::member("supportsNonZeroFirstInstance", TY_VK_BOOL),
            ],
        )
        .extensions_old(&[ex_khr("vertex_attribute_divisor")])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::new(
            "VertexInputBindingDivisorDescription",
            &[Struct::member("binding", "u32"), Struct::member("divisor", "u32")],
        )
        .extensions_old(&[ex_khr("vertex_attribute_divisor")])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "PipelineVertexInputDivisorStateCreateInfo",
            "PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO",
            vk_ext_enum(191, 1) as _,
            StructUsage::Source,
            &[
                Struct::member("vertexBindingDivisorCount", "u32"),
                Struct::member(
                    "pVertexBindingDivisors",
                    "*const VkVertexInputBindingDivisorDescriptionKHR",
                ),
            ],
        )
        .extensions_old(&[ex_khr("vertex_attribute_divisor")])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceVertexAttributeDivisorFeatures",
            "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES",
            vk_ext_enum(191, 2) as _,
            StructUsage::Both,
            &[
                Struct::member("vertexAttributeInstanceRateDivisor", TY_VK_BOOL),
                Struct::member("vertexAttributeInstanceRateZeroDivisor", TY_VK_BOOL),
            ],
        )
        .extensions_old(&[ex_khr("vertex_attribute_divisor")])
        .promoted(VERSION),
    ),
    // VK_KHR_global_priority
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_global_priority", 1)),
    Element::Enum(Enum::extending_error(&[Enum::member(
        "NOT_PERMITTED_KHR",
        -vk_ext_enum(175, 1) as _,
    )
    .extension_old("KHR", "global_priority")
    .promoted(VERSION)])),
    Element::Enum(
        Enum::new(
            "QueueGlobalPriority",
            "QUEUE_GLOBAL_PRIORITY",
            &[
                Enum::member("LOW", 128)
                    .extension_old("KHR", "global_priority")
                    .promoted(VERSION),
                Enum::member("MEDIUM", 256)
                    .extension_old("KHR", "global_priority")
                    .promoted(VERSION),
                Enum::member("HIGH", 512)
                    .extension_old("KHR", "global_priority")
                    .promoted(VERSION),
                Enum::member("REALTIME", 1024)
                    .extension_old("KHR", "global_priority")
                    .promoted(VERSION),
            ],
        )
        .extension_old("KHR", "global_priority")
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "DeviceQueueGlobalPriorityCreateInfo",
            "DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO",
            vk_ext_enum(175, 0) as _,
            StructUsage::Source,
            &[Struct::member("globalPriority", "VkQueueGlobalPriorityKHR")],
        )
        .extensions_old(&[ex_khr("global_priority")])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceGlobalPriorityQueryFeatures",
            "PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES",
            vk_ext_enum(389, 0) as _,
            StructUsage::Both,
            &[Struct::member("globalPriorityQuery", TY_VK_BOOL)],
        )
        .extensions_old(&[ex_khr("global_priority")])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "QueueFamilyGlobalPriorityProperties",
            "QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES",
            vk_ext_enum(389, 1) as _,
            StructUsage::Sink,
            &[
                Struct::member("priorityCount", "u32"),
                Struct::member(
                    "priorities",
                    "[VkQueueGlobalPriorityKHR; VK_MAX_GLOBAL_PRIORITY_SIZE_KHR]",
                ),
            ],
        )
        .extensions_old(&[ex_khr("global_priority")])
        .promoted(VERSION),
    ),
    // VK_KHR_load_store_op_none
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_LOAD_STORE_OP_NONE)),
    Element::Enum(Enum::extending(
        "AttachmentLoadOp",
        "ATTACHMENT_LOAD_OP",
        &[Enum::member("NONE", vk_ext_enum(401, 0) as _)
            .extension(VK_KHR_LOAD_STORE_OP_NONE)
            .promoted(VERSION)],
    )),
    Element::Enum(Enum::extending(
        "AttachmentStoreOp",
        "ATTACHMENT_STORE_OP",
        &[Enum::member("NONE", vk_ext_enum(302, 0) as _)
            .extension(VK_KHR_LOAD_STORE_OP_NONE)
            .extra_requirements("not(feature = \"VK_KHR_dynamic_rendering\")") // conflicting definition
            .promoted(VERSION)],
    )),
    // VK_KHR_shader_expect_assume
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_SHADER_EXPECT_ASSUME)),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceShaderExpectAssumeFeatures",
            "PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES",
            vk_ext_enum(545, 0) as _,
            StructUsage::Both,
            &[Struct::member("shaderExpectAssume", TY_VK_BOOL)],
        )
        .extensions(&[VK_KHR_SHADER_EXPECT_ASSUME])
        .promoted(VERSION),
    ),
    // VK_KHR_shader_float_controls2
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_SHADER_FLOAT_CONTROLS2)),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceShaderFloatControls2Features",
            "PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES",
            vk_ext_enum(529, 0) as _,
            StructUsage::Both,
            &[Struct::member("shaderFloatControls2", TY_VK_BOOL)],
        )
        .extensions(&[VK_KHR_SHADER_FLOAT_CONTROLS2])
        .promoted(VERSION),
    ),
];
