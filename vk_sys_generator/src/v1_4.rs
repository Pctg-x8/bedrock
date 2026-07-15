use crate::{parts::*, v1_1::VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE, vk_ext_enum};

const VERSION: &str = "1_4";
pub const VK_KHR_MAINTENANCE_5: &Extension = &Extension::khr("maintenance5", 1, 471);
const VK_KHR_MAINTENANCE_6: &Extension = &Extension::khr("maintenance6", 1, 546);
const VK_KHR_VERTEX_ATTRIBUTE_DIVISOR: &Extension = &Extension::khr("vertex_attribute_divisor", 1, 191);
const VK_KHR_GLOBAL_PRIORITY: &Extension = &Extension::khr("global_priority", 1, 175);
const VK_KHR_LOAD_STORE_OP_NONE: &Extension = &Extension::khr("load_store_op_none", 1, 527);
const VK_KHR_SHADER_EXPECT_ASSUME: &Extension = &Extension::khr("shader_expect_assume", 1, 545);
const VK_KHR_SHADER_FLOAT_CONTROLS2: &Extension = &Extension::khr("shader_float_controls2", 1, 529);
const VK_KHR_PUSH_DESCRIPTOR: &Extension = &Extension::khr("push_descriptor", 1, 81);

pub const ELEMENTS: &[Element] = &[
    // VK_KHR_maintenace5
    VK_KHR_MAINTENANCE_5.header_constants().into_element(),
    Bitmask::new(
        "BufferUsageFlags2",
        "BufferUsageFlagBits2",
        "BUFFER_USAGE_2",
        &[
            Bitmask::entry("TRANSFER_SRC", 0)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("TRANSFER_DST", 1)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("UNIFORM_TEXEL_BUFFER", 2)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("STORAGE_TEXEL_BUFFER", 3)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("UNIFORM_BUFFER", 4)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("STORAGE_BUFFER", 5)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("INDEX_BUFFER", 6)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("VERTEX_BUFFER", 7)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("INDIRECT_BUFFER", 8)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
        ],
    )
    .long()
    .extension(VK_KHR_MAINTENANCE_5)
    .promoted(VERSION)
    .into_element(),
    Bitmask::new(
        "PipelineCreateFlags2",
        "PipelineCreateFlagBits2",
        "PIPELINE_CREATE_2",
        &[
            Bitmask::entry("DISABLE_OPTIMIZATION", 0)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("ALLOW_DERIVATIVES", 1)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("DERIVATIVE", 2)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("VIEW_INDEX_FROM_DEVICE_INDEX", 3)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
            Bitmask::entry("DISPATCH_BASE", 4)
                .extension(VK_KHR_MAINTENANCE_5)
                .promoted(VERSION),
        ],
    )
    .long()
    .extension(VK_KHR_MAINTENANCE_5)
    .promoted(VERSION)
    .into_element(),
    // VK_FORMAT_A1B5G5R4_UNORM_PACK16_KHR defined in main
    // VK_FORMAT_A8_UNORM_KHR defined in main
    Struct::typed(
        "PhysicalDeviceMaintenance5Features",
        "PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES",
        VK_KHR_MAINTENANCE_5.ext_enum(0) as _,
        StructUsage::Both,
        &[Struct::member("maintenance5", TY_VK_BOOL)],
    )
    .extensions(&[VK_KHR_MAINTENANCE_5])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceMaintenance5Properties",
        "PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES",
        VK_KHR_MAINTENANCE_5.ext_enum(1) as _,
        StructUsage::Sink,
        &[
            Struct::member("earlyFragmentMultisampleCoverageAfterSampleCounting", TY_VK_BOOL),
            Struct::member("earlyFragmentSampleMaskTestBeforeSampleCounting", TY_VK_BOOL),
            Struct::member("depthStencilSwizzleOneSupport", TY_VK_BOOL),
            Struct::member("polygonModePointSize", TY_VK_BOOL),
            Struct::member("nonStrictSinglePixelWideLinesUseParallelogram", TY_VK_BOOL),
            Struct::member("nonStrictWideLinesUseParallelogram", TY_VK_BOOL),
        ],
    )
    .extensions(&[VK_KHR_MAINTENANCE_5])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "RenderingAreaInfo",
        "RENDERING_AREA_INFO",
        VK_KHR_MAINTENANCE_5.ext_enum(3) as _,
        StructUsage::Source,
        &[
            Struct::member("viewMask", "u32"),
            Struct::member("colorAttachmentCount", "u32"),
            Struct::member("pColorAttachmentFormats", "*const VkFormat"),
            Struct::member("depthAttachmentFormat", "VkFormat"),
            Struct::member("stencilAttachmentFormat", "VkFormat"),
        ],
    )
    .extensions(&[VK_KHR_MAINTENANCE_5])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImageSubresource2",
        "IMAGE_SUBRESOURCE_2",
        vk_ext_enum(339, 3) as _,
        StructUsage::Source,
        &[Struct::member("imageSubresource", "VkImageSubresource")],
    )
    .extensions(&[VK_KHR_MAINTENANCE_5])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DeviceImageSubresourceInfo",
        "DEVICE_IMAGE_SUBRESOURCE_INFO",
        VK_KHR_MAINTENANCE_5.ext_enum(4) as _,
        StructUsage::Source,
        &[
            Struct::member("pCreateInfo", "*const VkImageCreateInfo"),
            Struct::member("pSubresource", "*const VkImageSubresource2KHR"),
        ],
    )
    .extensions(&[VK_KHR_MAINTENANCE_5])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SubresourceLayout2",
        "SUBRESOURCE_LAYOUT_2",
        vk_ext_enum(339, 2) as _,
        StructUsage::Sink,
        &[Struct::member("subresourceLayout", "VkSubresourceLayout")],
    )
    .extensions(&[VK_KHR_MAINTENANCE_5])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PipelineCreateFlags2CreateInfo",
        "PIPELINE_CREATE_FLAGS_2_CREATE_INFO",
        VK_KHR_MAINTENANCE_5.ext_enum(5) as _,
        StructUsage::Source,
        &[Struct::member("flags", "VkPipelineCreateFlags2KHR")],
    )
    .extensions(&[VK_KHR_MAINTENANCE_5])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BufferUsageFlags2CreateInfo",
        "BUFFER_USAGE_FLAGS_2_CREATE_INFO",
        VK_KHR_MAINTENANCE_5.ext_enum(6) as _,
        StructUsage::Source,
        &[Struct::member("usage", "VkBufferUsageFlags2KHR")],
    )
    .extensions(&[VK_KHR_MAINTENANCE_5])
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "BindIndexBuffer2",
        &[
            ("buffer", "VkBuffer"),
            ("offset", "VkDeviceSize"),
            ("size", "VkDeviceSize"),
            ("indexType", "VkIndexType"),
        ],
    )
    .extension(VK_KHR_MAINTENANCE_5)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "GetRenderingAreaGranularity",
        &[
            ("device", "VkDevice"),
            ("pRenderingAreaInfo", "*const VkRenderingAreaInfoKHR"),
            ("pGranularity", "*mut VkExtent2D"),
        ],
    )
    .extension(VK_KHR_MAINTENANCE_5)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "GetDeviceImageSubresourceLayout",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkDeviceImageSubresourceInfoKHR"),
            ("pLayout", "*mut VkSubresourceLayout2KHR"),
        ],
    )
    .extension(VK_KHR_MAINTENANCE_5)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "GetImageSubresourceLayout2",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("pSubresource", "*const VkImageSubresource2KHR"),
            ("pLayout", "*mut VkSubresourceLayout2KHR"),
        ],
    )
    .extension(VK_KHR_MAINTENANCE_5)
    .promoted(VERSION)
    .into_element(),
    // VK_KHR_maintenance6
    VK_KHR_MAINTENANCE_6.header_constants().into_element(),
    Struct::typed(
        "PhysicalDeviceMaintenance6Features",
        "PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES",
        VK_KHR_MAINTENANCE_6.ext_enum(0) as _,
        StructUsage::Both,
        &[Struct::member("maintenance6", TY_VK_BOOL)],
    )
    .extensions(&[VK_KHR_MAINTENANCE_6])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceMaintenance6Properties",
        "PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES",
        VK_KHR_MAINTENANCE_6.ext_enum(1) as _,
        StructUsage::Sink,
        &[
            Struct::member("blockTexelViewCompatibleMultipleLayers", TY_VK_BOOL),
            Struct::member("maxCombinedImageSamplerDescriptorCount", "u32"),
            Struct::member("fragmentShadingRateClampCombinerInputs", TY_VK_BOOL),
        ],
    )
    .extensions(&[VK_KHR_MAINTENANCE_6])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BindMemoryStatus",
        "BIND_MEMORY_STATUS",
        VK_KHR_MAINTENANCE_6.ext_enum(2) as _,
        StructUsage::Source,
        &[Struct::member("pResult", "*mut VkResult")],
    )
    .extensions(&[VK_KHR_MAINTENANCE_6])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BindDescriptionSetsInfo",
        "BIND_DESCRIPTOR_SETS_INFO",
        VK_KHR_MAINTENANCE_6.ext_enum(3) as _,
        StructUsage::Source,
        &[
            Struct::member("stageFlags", "VkShaderStageFlags"),
            Struct::member("layout", "VkPipelineLayout"),
            Struct::member("firstSet", "u32"),
            Struct::member("descriptorSetCount", "u32"),
            Struct::member("pDescriptorSets", "*const VkDescriptorSet"),
            Struct::member("dynamicOffsetCount", "u32"),
            Struct::member("pDynamicOffsets", "*const u32"),
        ],
    )
    .extensions(&[VK_KHR_MAINTENANCE_6])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PushConstantsInfo",
        "PUSH_CONSTANTS_INFO",
        VK_KHR_MAINTENANCE_6.ext_enum(4) as _,
        StructUsage::Source,
        &[
            Struct::member("layout", "VkPipelineLayout"),
            Struct::member("stageFlags", "VkShaderStageFlags"),
            Struct::member("offset", "u32"),
            Struct::member("size", "u32"),
            Struct::member("pValues", "*const core::ffi::c_void"),
        ],
    )
    .extensions(&[VK_KHR_MAINTENANCE_6])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PushDescriptorSetInfo",
        "PUSH_DESCRIPTOR_SET_INFO",
        VK_KHR_MAINTENANCE_6.ext_enum(5) as _,
        StructUsage::Source,
        &[
            Struct::member("stageFlags", "VkShaderStageFlags"),
            Struct::member("layout", "VkPipelineLayout"),
            Struct::member("set", "u32"),
            Struct::member("descriptorWriteCount", "u32"),
            Struct::member("pDescriptorWrites", "*const VkWriteDescriptorSet"),
        ],
    )
    .extensions(&[VK_KHR_MAINTENANCE_6, VK_KHR_PUSH_DESCRIPTOR])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PushDescriptorSetWithTemplateInfo",
        "PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO",
        VK_KHR_MAINTENANCE_6.ext_enum(6) as _,
        StructUsage::Source,
        &[
            Struct::member("descriptorUpdateTemplate", "VkDescriptorUpdateTemplateKHR"),
            Struct::member("layout", "VkPipelineLayout"),
            Struct::member("set", "u32"),
            Struct::member("pData", "*const core::ffi::c_void"),
        ],
    )
    .extensions(&[VK_KHR_MAINTENANCE_6, VK_KHR_PUSH_DESCRIPTOR])
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "BindDescriptorSets2",
        &[("pBindDescriptorSetsInfo", "*const VkBindDescriptorSetsInfoKHR")],
    )
    .extension(VK_KHR_MAINTENANCE_6)
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "PushConstants2",
        &[("pPushConstantsInfo", "*const VkPushConstantsInfoKHR")],
    )
    .extension(VK_KHR_MAINTENANCE_6)
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "PushDescriptorSet2",
        &[("pPushDescriptorSetInfo", "*const VkPushDescriptorSetInfoKHR")],
    )
    .extension(VK_KHR_MAINTENANCE_6)
    .extra_requirements(&["VK_KHR_push_descriptor"])
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "PushDescriptorSetWithTemplate2",
        &[(
            "pPushDescriptorSetWithTemplateInfo",
            "*const VkPushDescriptorSetWithTemplateInfoKHR",
        )],
    )
    .extension(VK_KHR_MAINTENANCE_6)
    .extra_requirements(&["VK_KHR_push_descriptor"])
    .promoted(VERSION)
    .into_element(),
    // VK_KHR_vertex_attribute_divisor
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_VERTEX_ATTRIBUTE_DIVISOR)),
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
        .extensions(&[VK_KHR_VERTEX_ATTRIBUTE_DIVISOR])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::new(
            "VertexInputBindingDivisorDescription",
            &[Struct::member("binding", "u32"), Struct::member("divisor", "u32")],
        )
        .extensions(&[VK_KHR_VERTEX_ATTRIBUTE_DIVISOR])
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
        .extensions(&[VK_KHR_VERTEX_ATTRIBUTE_DIVISOR])
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
        .extensions(&[VK_KHR_VERTEX_ATTRIBUTE_DIVISOR])
        .promoted(VERSION),
    ),
    // VK_KHR_global_priority
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_GLOBAL_PRIORITY)),
    Element::Enum(Enum::extending_error(&[Enum::member(
        "NOT_PERMITTED_KHR",
        -vk_ext_enum(175, 1) as _,
    )
    .extension(VK_KHR_GLOBAL_PRIORITY)
    .promoted(VERSION)])),
    Element::Enum(
        Enum::new(
            "QueueGlobalPriority",
            "QUEUE_GLOBAL_PRIORITY",
            &[
                Enum::member("LOW", 128)
                    .extension(VK_KHR_GLOBAL_PRIORITY)
                    .promoted(VERSION),
                Enum::member("MEDIUM", 256)
                    .extension(VK_KHR_GLOBAL_PRIORITY)
                    .promoted(VERSION),
                Enum::member("HIGH", 512)
                    .extension(VK_KHR_GLOBAL_PRIORITY)
                    .promoted(VERSION),
                Enum::member("REALTIME", 1024)
                    .extension(VK_KHR_GLOBAL_PRIORITY)
                    .promoted(VERSION),
            ],
        )
        .extension(VK_KHR_GLOBAL_PRIORITY)
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
        .extensions(&[VK_KHR_GLOBAL_PRIORITY])
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
        .extensions(&[VK_KHR_GLOBAL_PRIORITY])
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
        .extensions(&[VK_KHR_GLOBAL_PRIORITY])
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
    // VK_KHR_push_descriptor
    VK_KHR_PUSH_DESCRIPTOR.header_constants().into_element(),
    Bitmask::extending(
        "DescriptorSetLayoutCreateFlagBits",
        "DESCRIPTOR_SET_LAYOUT_CREATE",
        &[Bitmask::entry("PUSH_DESCRIPTOR", 0)
            .extension(VK_KHR_PUSH_DESCRIPTOR)
            .promoted(VERSION)],
    )
    .into_element(),
    Struct::typed(
        "PhysicalDevicePushDescriptorProperties",
        "PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES",
        VK_KHR_PUSH_DESCRIPTOR.ext_enum(0) as _,
        StructUsage::Sink,
        &[Struct::member("maxPushDescriptors", "u32")],
    )
    .extensions(&[VK_KHR_PUSH_DESCRIPTOR])
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "PushDescriptorSet",
        &[
            ("pipelineBindPoint", "VkPipelineBindPoint"),
            ("layout", "VkPipelineLayout"),
            ("set", "u32"),
            ("descriptorWriteCount", "u32"),
            ("pDescriptorWrites", "*const VkWriteDescriptorSet"),
        ],
    )
    .extension(VK_KHR_PUSH_DESCRIPTOR)
    .promoted(VERSION)
    .into_element(),
    Enum::extending(
        "DescriptorUpdateTemplateType",
        "DESCRIPTOR_UPDATE_TEMPLATE_TYPE",
        &[Enum::member("PUSH_DESCRIPTORS", 1)
            .extension(VK_KHR_PUSH_DESCRIPTOR)
            .promoted(VERSION)],
    )
    .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)
    .into_element(),
    Command::inst(
        "PushDescriptorSetWithTemplate",
        &[
            ("descriptorUpdateTemplate", "VkDescriptorUpdateTemplateKHR"),
            ("layout", "VkPipelineLayout"),
            ("set", "u32"),
            ("pData", "*const core::ffi::c_void"),
        ],
    )
    .extension(VK_KHR_PUSH_DESCRIPTOR)
    .extra_requirements(&["VK_KHR_descriptor_update_template"])
    .promoted(VERSION)
    .into_element(),
];
