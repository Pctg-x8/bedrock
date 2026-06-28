use crate::{parts::*, vk_ext_enum};

const VERSION: &str = "1_4";

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
        .extensions(&[ex_khr("vertex_attribute_divisor")])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::new(
            "VertexInputBindingDivisorDescription",
            &[Struct::member("binding", "u32"), Struct::member("divisor", "u32")],
        )
        .extensions(&[ex_khr("vertex_attribute_divisor")])
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
        .extensions(&[ex_khr("vertex_attribute_divisor")])
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
        .extensions(&[ex_khr("vertex_attribute_divisor")])
        .promoted(VERSION),
    ),
    // VK_KHR_global_priority
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_global_priority", 1)),
    Element::Enum(Enum::extending_error(&[Enum::member(
        "NOT_PERMITTED_KHR",
        -vk_ext_enum(175, 1) as _,
    )
    .extension("VK_KHR_global_priority", "KHR")
    .promoted(VERSION)])),
    Element::Enum(
        Enum::new(
            "QueueGlobalPriority",
            "QUEUE_GLOBAL_PRIORITY",
            &[
                Enum::member("LOW", 128)
                    .extension("VK_KHR_global_priority", "KHR")
                    .promoted(VERSION),
                Enum::member("MEDIUM", 256)
                    .extension("VK_KHR_global_priority", "KHR")
                    .promoted(VERSION),
                Enum::member("HIGH", 512)
                    .extension("VK_KHR_global_priority", "KHR")
                    .promoted(VERSION),
                Enum::member("REALTIME", 1024)
                    .extension("VK_KHR_global_priority", "KHR")
                    .promoted(VERSION),
            ],
        )
        .extension("global_priority", "KHR")
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
        .extensions(&[ex_khr("global_priority")])
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
        .extensions(&[ex_khr("global_priority")])
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
        .extensions(&[ex_khr("global_priority")])
        .promoted(VERSION),
    ),
];
