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
];
