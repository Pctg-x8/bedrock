use crate::{parts::*, v1_1::VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE, vk_ext_enum};

const VERSION: &str = "1_4";
pub const VK_KHR_MAINTENANCE_5: &Extension = &Extension::khr("maintenance5", 1, 471).promoted(VERSION);
const VK_KHR_MAINTENANCE_6: &Extension = &Extension::khr("maintenance6", 1, 546);
const VK_KHR_VERTEX_ATTRIBUTE_DIVISOR: &Extension = &Extension::khr("vertex_attribute_divisor", 1, 191);
const VK_KHR_GLOBAL_PRIORITY: &Extension = &Extension::khr("global_priority", 1, 175);
const VK_KHR_LOAD_STORE_OP_NONE: &Extension = &Extension::khr("load_store_op_none", 1, 527);
const VK_KHR_SHADER_EXPECT_ASSUME: &Extension = &Extension::khr("shader_expect_assume", 1, 545);
const VK_KHR_SHADER_FLOAT_CONTROLS2: &Extension = &Extension::khr("shader_float_controls2", 1, 529);
const VK_KHR_PUSH_DESCRIPTOR: &Extension = &Extension::khr("push_descriptor", 1, 81);
const VK_KHR_DYNAMIC_RENDERING_LOCAL_READ: &Extension = &Extension::khr("dynamic_rendering_local_read", 1, 233);
const VK_KHR_INDEX_TYPE_UINT8: &Extension = &Extension::khr("index_type_uint8", 1, 534);
const VK_KHR_LINE_RASTERIZATION: &Extension = &Extension::khr("line_rasterization", 1, 535);
const VK_KHR_MAP_MEMORY_2: &Extension = &Extension::khr("map_memory2", 1, 272);
const VK_KHR_SHADER_SUBGROUP_ROTATE: &Extension = &Extension::khr("shader_subgroup_rotate", 2, 417);
const VK_EXT_HOST_IMAGE_COPY: &Extension = &Extension::ext("host_image_copy", 1, 271);
const VK_EXT_PIPELINE_PROTECTED_ACCESS: &Extension = &Extension::ext("pipeline_protected_access", 1, 467);
const VK_EXT_PIPELINE_ROBUSTNESS: &Extension = &Extension::ext("pipeline_robustness", 1, 69).promoted(VERSION);

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
        "BindDescriptorSetsInfo",
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
    .available_condition("feature = \"VK_KHR_push_descriptor\"")
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
    .available_condition("feature = \"VK_KHR_push_descriptor\"")
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
    .available_condition("feature = \"VK_KHR_descriptor_update_template\"")
    .promoted(VERSION)
    .into_element(),
    // VK_KHR_dynamic_rendering_local_read
    VK_KHR_DYNAMIC_RENDERING_LOCAL_READ.header_constants().into_element(),
    Enum::extending(
        "ImageLayout",
        "IMAGE_LAYOUT",
        &[Enum::member(
            "RENDERING_LOCAL_READ",
            VK_KHR_DYNAMIC_RENDERING_LOCAL_READ.ext_enum(0) as _,
        )
        .extension(VK_KHR_DYNAMIC_RENDERING_LOCAL_READ)
        .promoted(VERSION)],
    )
    .into_element(),
    Struct::typed(
        "PhysicalDeviceDynamicRenderingLocalReadFeatures",
        "PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES",
        VK_KHR_DYNAMIC_RENDERING_LOCAL_READ.ext_enum(0) as _,
        StructUsage::Source,
        &[Struct::member("dynamicRenderingLocalRead", TY_VK_BOOL)],
    )
    .extensions(&[VK_KHR_DYNAMIC_RENDERING_LOCAL_READ])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "RenderingAttachmentLocationInfo",
        "RENDERING_ATTACHMENT_LOCATION_INFO",
        VK_KHR_DYNAMIC_RENDERING_LOCAL_READ.ext_enum(1) as _,
        StructUsage::Source,
        &[
            Struct::member("colorAttachmentCount", "u32"),
            Struct::member("pColorAttachmentLocations", "*const u32"),
        ],
    )
    .extensions(&[VK_KHR_DYNAMIC_RENDERING_LOCAL_READ])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "RenderingInputAttachmentIndexInfo",
        "RENDERING_INPUT_ATTACHMENT_INDEX_INFO",
        VK_KHR_DYNAMIC_RENDERING_LOCAL_READ.ext_enum(2) as _,
        StructUsage::Source,
        &[
            Struct::member("colorAttachmentCount", "u32"),
            Struct::member("pColorAttachmentInputIndices", "*const u32"),
            Struct::member("pDepthInputAttachmentIndex", "*const u32"),
            Struct::member("pStencilInputAttachmentIndex", "*const u32"),
        ],
    )
    .extensions(&[VK_KHR_DYNAMIC_RENDERING_LOCAL_READ])
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "SetRenderingAttachmentLocations",
        &[("pLocationInfo", "*const VkRenderingAttachmentLocationInfoKHR")],
    )
    .extension(VK_KHR_DYNAMIC_RENDERING_LOCAL_READ)
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "SetRenderingInputAttachmentIndices",
        &[(
            "pInputAttachmentIndexInfo",
            "*const VkRenderingInputAttachmentIndexInfoKHR",
        )],
    )
    .extension(VK_KHR_DYNAMIC_RENDERING_LOCAL_READ)
    .promoted(VERSION)
    .into_element(),
    // VK_KHR_index_type_uint8
    VK_KHR_INDEX_TYPE_UINT8.header_constants().into_element(),
    Enum::extending(
        "IndexType",
        "INDEX_TYPE",
        &[Enum::member("UINT8", vk_ext_enum(266, 0) as _)
            .extension(VK_KHR_INDEX_TYPE_UINT8)
            .promoted(VERSION)],
    )
    .into_element(),
    Struct::typed(
        "PhysicalDeviceIndexTypeUint8Features",
        "PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES",
        vk_ext_enum(266, 0) as _,
        StructUsage::Both,
        &[Struct::member("indexTypeUint8", TY_VK_BOOL)],
    )
    .extensions(&[VK_KHR_INDEX_TYPE_UINT8])
    .promoted(VERSION)
    .into_element(),
    // VK_KHR_line_rasterization
    VK_KHR_LINE_RASTERIZATION.header_constants().into_element(),
    Enum::new(
        "LineRasterizationMode",
        "LINE_RASTERIZATION_MODE",
        &[
            Enum::member("DEFAULT", 0)
                .extension(VK_KHR_LINE_RASTERIZATION)
                .promoted(VERSION),
            Enum::member("RECTANGULAR", 1)
                .extension(VK_KHR_LINE_RASTERIZATION)
                .promoted(VERSION),
            Enum::member("BRESENHAM", 2)
                .extension(VK_KHR_LINE_RASTERIZATION)
                .promoted(VERSION),
            Enum::member("RECTANGULAR_SMOOTH", 3)
                .extension(VK_KHR_LINE_RASTERIZATION)
                .promoted(VERSION),
        ],
    )
    .extension(VK_KHR_LINE_RASTERIZATION)
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceLineRasterizationFeatures",
        "PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES",
        VK_KHR_LINE_RASTERIZATION.ext_enum(0) as _,
        StructUsage::Both,
        &[
            Struct::member("rectangularLines", TY_VK_BOOL),
            Struct::member("bresenhamLines", TY_VK_BOOL),
            Struct::member("smoothLines", TY_VK_BOOL),
            Struct::member("stippledRectangularLines", TY_VK_BOOL),
            Struct::member("stippledBresenhamLines", TY_VK_BOOL),
            Struct::member("stippledSmoothLines", TY_VK_BOOL),
        ],
    )
    .extensions(&[VK_KHR_LINE_RASTERIZATION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceLineRasterizationProperties",
        "PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES",
        VK_KHR_LINE_RASTERIZATION.ext_enum(2) as _,
        StructUsage::Sink,
        &[Struct::member("lineSubPixelPrecisionBits", "u32")],
    )
    .extensions(&[VK_KHR_LINE_RASTERIZATION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PipelineRasterizationLineStateCreateInfo",
        "PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO",
        VK_KHR_LINE_RASTERIZATION.ext_enum(1) as _,
        StructUsage::Source,
        &[
            Struct::member("lineRasterizationMode", "VkLineRasterizationModeKHR"),
            Struct::member("stippledLineEnable", TY_VK_BOOL),
            Struct::member("lineStippleFactor", "u32"),
            Struct::member("lineStipplePattern", "u16"),
        ],
    )
    .extensions(&[VK_KHR_LINE_RASTERIZATION])
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "SetLineStipple",
        &[("lineStippleFactor", "u32"), ("lineStipplePattern", "u16")],
    )
    .extension(VK_KHR_LINE_RASTERIZATION)
    .promoted(VERSION)
    .into_element(),
    // VK_KHR_map_memory2
    VK_KHR_MAP_MEMORY_2.header_constants().into_element(),
    Bitmask::new(
        "MemoryUnmapFlags",
        "MemoryUnmapFlagBits",
        "MEMORY_UNMAP",
        &[Bitmask::entry("RESERVE", 0)
            .extension(VK_KHR_MAP_MEMORY_2)
            .promoted(VERSION)],
    )
    .extension(VK_KHR_MAP_MEMORY_2)
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "MemoryMapInfo",
        "MEMORY_MAP_INFO",
        VK_KHR_MAP_MEMORY_2.ext_enum(0) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkMemoryUnmapFlagsKHR"),
            Struct::member("memory", "VkDeviceMemory"),
        ],
    )
    .extensions(&[VK_KHR_MAP_MEMORY_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "MemoryUnmapInfo",
        "MEMORY_UNMAP_INFO",
        VK_KHR_MAP_MEMORY_2.ext_enum(1) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkMemoryUnmapFlagsKHR"),
            Struct::member("memory", "VkDeviceMemory"),
        ],
    )
    .extensions(&[VK_KHR_MAP_MEMORY_2])
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "MapMemory2",
        &[
            ("device", "VkDevice"),
            ("pMemoryMapInfo", "*const VkMemoryMapInfoKHR"),
            ("ppData", "*mut *mut core::ffi::c_void"),
        ],
    )
    .failable()
    .extension(VK_KHR_MAP_MEMORY_2)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "UnmapMemory2",
        &[
            ("device", "VkDevice"),
            ("pMemoryUnmapInfo", "*const VkMemoryUnmapInfoKHR"),
        ],
    )
    .failable()
    .extension(VK_KHR_MAP_MEMORY_2)
    .promoted(VERSION)
    .into_element(),
    // VK_KHR_shader_subgroup_rotate
    VK_KHR_SHADER_SUBGROUP_ROTATE.header_constants().into_element(),
    Bitmask::extending(
        "SubgroupFeatureFlagBits",
        "SUBGROUP_FEATURE",
        &[
            Bitmask::entry("ROTATE", 9)
                .extension(VK_KHR_SHADER_SUBGROUP_ROTATE)
                .promoted(VERSION),
            Bitmask::entry("ROTATE_CLUSTERED", 10)
                .extension(VK_KHR_SHADER_SUBGROUP_ROTATE)
                .promoted(VERSION),
        ],
    )
    .into_element(),
    Struct::typed(
        "PhysicalDeviceShaderSubgroupRotateFeatures",
        "PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES",
        VK_KHR_SHADER_SUBGROUP_ROTATE.ext_enum(0) as _,
        StructUsage::Both,
        &[
            Struct::member("shaderSubgroupRotate", TY_VK_BOOL),
            Struct::member("shaderSubgroupRotateClustered", TY_VK_BOOL),
        ],
    )
    .extensions(&[VK_KHR_SHADER_SUBGROUP_ROTATE])
    .promoted(VERSION)
    .into_element(),
    // VK_EXT_host_image_copy
    VK_EXT_HOST_IMAGE_COPY.header_constants().into_element(),
    Bitmask::extending(
        "FormatFeatureFlagBits2",
        "FORMAT_FEATURE_2",
        &[Bitmask::entry("HOST_IMAGE_TRANSFER", 34)
            .extension(VK_EXT_HOST_IMAGE_COPY)
            .promoted(VERSION)],
    )
    .long()
    .into_element(),
    Bitmask::extending(
        "ImageUsageFlagBits",
        "IMAGE_USAGE",
        &[Bitmask::entry("HOST_TRANSFER", 22)
            .extension(VK_EXT_HOST_IMAGE_COPY)
            .promoted(VERSION)],
    )
    .into_element(),
    Bitmask::new(
        "HostImageCopyFlags",
        "HostImageCopyFlagBits",
        "HOST_IMAGE_COPY",
        &[Bitmask::entry("MEMCPY", 0)
            .extension(VK_EXT_HOST_IMAGE_COPY)
            .promoted(VERSION)],
    )
    .extension(VK_EXT_HOST_IMAGE_COPY)
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceHostImageCopyFeatures",
        "PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES",
        VK_EXT_HOST_IMAGE_COPY.ext_enum(0) as _,
        StructUsage::Both,
        &[Struct::member("hostImageCopy", TY_VK_BOOL)],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceHostImageCopyProperites",
        "PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES",
        VK_EXT_HOST_IMAGE_COPY.ext_enum(1) as _,
        StructUsage::Sink,
        &[
            Struct::member("copySrcLayoutCount", "u32"),
            Struct::member("pCopySrcLayouts", "*mut VkImageLayout"),
            Struct::member("copyDstLayoutCount", "u32"),
            Struct::member("pCopyDstLayouts", "*mut VkImageLayout"),
            Struct::member("optimalTilingLayoutUUID", "[u8; VK_UUID_SIZE]"),
            Struct::member("identicalMemoryTypeRequirements", TY_VK_BOOL),
        ],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "MemoryToImageCopy",
        "MEMORY_TO_IMAGE_COPY",
        VK_EXT_HOST_IMAGE_COPY.ext_enum(2) as _,
        StructUsage::Source,
        &[
            Struct::member("pHostPointer", "*const core::ffi::c_void"),
            Struct::member("memoryRowLength", "u32"),
            Struct::member("memoryImageHeight", "u32"),
            Struct::member("imageSubresource", "VkImageSubresourceLayers"),
            Struct::member("imageOffset", "VkOffset3D"),
            Struct::member("imageExtent", "VkExtent3D"),
        ],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImageToMemoryCopy",
        "IMAGE_TO_MEMORY_COPY",
        VK_EXT_HOST_IMAGE_COPY.ext_enum(3) as _,
        StructUsage::Source,
        &[
            Struct::member("pHostPointer", "*mut core::ffi::c_void"),
            Struct::member("memoryRowLength", "u32"),
            Struct::member("memoryImageHeight", "u32"),
            Struct::member("imageSubresource", "VkImageSubresourceLayers"),
            Struct::member("imageOffset", "VkOffset3D"),
            Struct::member("imageExtent", "VkExtent3D"),
        ],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "CopyMemoryToImageInfo",
        "COPY_MEMORY_TO_IMAGE_INFO",
        VK_EXT_HOST_IMAGE_COPY.ext_enum(5) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkHostImageCopyFlagsEXT"),
            Struct::member("dstImage", "VkImage"),
            Struct::member("dstImageLayout", "VkImageLayout"),
            Struct::member("regionCount", "u32"),
            Struct::member("pRegions", "*const VkMemoryToImageCopyEXT"),
        ],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "CopyImageToMemoryInfo",
        "COPY_IMAGE_TO_MEMORY_INFO",
        VK_EXT_HOST_IMAGE_COPY.ext_enum(4) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkHostImageCopyFlagsEXT"),
            Struct::member("srcImage", "VkImage"),
            Struct::member("srcImageLayout", "VkImageLayout"),
            Struct::member("regionCount", "u32"),
            Struct::member("pRegions", "*const VkImageToMemoryCopyEXT"),
        ],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "CopyImageToImageInfo",
        "COPY_IMAGE_TO_IMAGE_INFO",
        VK_EXT_HOST_IMAGE_COPY.ext_enum(7) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkHostImageCopyFlagsEXT"),
            Struct::member("srcImage", "VkImage"),
            Struct::member("srcImageLayout", "VkImageLayout"),
            Struct::member("dstImage", "VkImage"),
            Struct::member("dstImageLayout", "VkImageLayout"),
            Struct::member("regionCount", "u32"),
            Struct::member("pRegions", "*const VkImageCopy2KHR"),
        ],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "HostImageLayoutTransitionInfo",
        "HOST_IMAGE_LAYOUT_TRANSITION",
        VK_EXT_HOST_IMAGE_COPY.ext_enum(6) as _,
        StructUsage::Source,
        &[
            Struct::member("image", "VkImage"),
            Struct::member("oldLayout", "VkImageLayout"),
            Struct::member("newLayout", "VkImageLayout"),
            Struct::member("subresourceRange", "VkImageSubresourceRange"),
        ],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SubresourceHostMemcpySize",
        "SUBRESOURCE_HOST_MEMCPY_SIZE",
        VK_EXT_HOST_IMAGE_COPY.ext_enum(8) as _,
        StructUsage::Sink,
        &[Struct::member("size", "VkDeviceSize")],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "HostImageCopyDevicePerformanceQuery",
        "HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY",
        VK_EXT_HOST_IMAGE_COPY.ext_enum(9) as _,
        StructUsage::Sink,
        &[
            Struct::member("optimalDeviceAccess", TY_VK_BOOL),
            Struct::member("identicalMemoryLayout", TY_VK_BOOL),
        ],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SubresourceLayout2",
        "SURESOURCE_LAYOUT_2",
        vk_ext_enum(339, 2) as _,
        StructUsage::Both,
        &[Struct::member("subresourceLayout", "VkSubresourceLayout")],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImageSubresource2",
        "IMAGE_SUBRESOURCE_2",
        vk_ext_enum(339, 3) as _,
        StructUsage::Both,
        &[Struct::member("imageSubresource", "VkImageSubresource")],
    )
    .extensions(&[VK_EXT_HOST_IMAGE_COPY])
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "CopyMemoryToImage",
        &[
            ("device", "VkDevice"),
            ("pCopyMemoryToImageInfo", "*const VkCopyMemoryToImageInfoEXT"),
        ],
    )
    .failable()
    .extension(VK_EXT_HOST_IMAGE_COPY)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "CopyImageToMemory",
        &[
            ("device", "VkDevice"),
            ("pCopyImageToMemoryInfo", "*const VkCopyImageToMemoryInfoEXT"),
        ],
    )
    .failable()
    .extension(VK_EXT_HOST_IMAGE_COPY)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "CopyImageToImage",
        &[
            ("device", "VkDevice"),
            ("pCopyImageToImageInfo", "*const VkCopyImageToImageInfoEXT"),
        ],
    )
    .failable()
    .extension(VK_EXT_HOST_IMAGE_COPY)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "TransitionImageLayout",
        &[
            ("device", "VkDevice"),
            ("transitionCount", "u32"),
            ("pTransitions", "*const VkHostImageLayoutTransitionInfoEXT"),
        ],
    )
    .failable()
    .extension(VK_EXT_HOST_IMAGE_COPY)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "GetImageSubresourceLayout2",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("pSubresource", "*const VkImageSubresource2EXT"),
            ("pLayout", "*mut VkSubresourceLayout2EXT"),
        ],
    )
    .extension(VK_EXT_HOST_IMAGE_COPY)
    .promoted(VERSION)
    .into_element(),
    // VK_EXT_pipeline_protected_access
    VK_EXT_PIPELINE_PROTECTED_ACCESS.header_constants().into_element(),
    Bitmask::extending(
        "PipelineCreateFlagBits",
        "PIPELINE_CREATE",
        &[
            Bitmask::entry("NO_PROTECTED_ACCESS", 27)
                .extension(VK_EXT_PIPELINE_PROTECTED_ACCESS)
                .promoted(VERSION),
            Bitmask::entry("PROTECTED_ACCESS_ONLY", 30)
                .extension(VK_EXT_PIPELINE_PROTECTED_ACCESS)
                .promoted(VERSION),
        ],
    )
    .into_element(),
    Struct::typed(
        "PhysicalDevicePipelineProtectedAccessFeatures",
        "PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES",
        VK_EXT_PIPELINE_PROTECTED_ACCESS.ext_enum(0) as _,
        StructUsage::Both,
        &[Struct::member("pipelineProtectedAccess", TY_VK_BOOL)],
    )
    .extensions(&[VK_EXT_PIPELINE_PROTECTED_ACCESS])
    .promoted(VERSION)
    .into_element(),
    // VK_EXT_pipeline_robustness
    VK_EXT_PIPELINE_ROBUSTNESS.header_constants().into_element(),
    Enum::new(
        "PipelineRobustnessBufferBehavior",
        "PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR",
        &[
            Enum::member("DEVICE_DEFAULT", 0)
                .extension(VK_EXT_PIPELINE_ROBUSTNESS)
                .promoted(VERSION),
            Enum::member("DISABLED", 1)
                .extension(VK_EXT_PIPELINE_ROBUSTNESS)
                .promoted(VERSION),
            Enum::member("ROBUST_BUFFER_ACCESS", 2)
                .extension(VK_EXT_PIPELINE_ROBUSTNESS)
                .promoted(VERSION),
            Enum::member("ROBUST_BUFFER_ACCESS_2", 3)
                .extension(VK_EXT_PIPELINE_ROBUSTNESS)
                .promoted(VERSION),
        ],
    )
    .extension(VK_EXT_PIPELINE_PROTECTED_ACCESS)
    .promoted(VERSION)
    .into_element(),
    Enum::new(
        "PipelineRobustnessImageBehavior",
        "PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR",
        &[
            Enum::member("DEVICE_DEFAULT", 0)
                .extension(VK_EXT_PIPELINE_ROBUSTNESS)
                .promoted(VERSION),
            Enum::member("DISABLED", 1)
                .extension(VK_EXT_PIPELINE_ROBUSTNESS)
                .promoted(VERSION),
            Enum::member("ROBUST_IMAGE_ACCESS", 2)
                .extension(VK_EXT_PIPELINE_ROBUSTNESS)
                .promoted(VERSION),
            Enum::member("ROBUST_IMAGE_ACCESS_2", 3)
                .extension(VK_EXT_PIPELINE_ROBUSTNESS)
                .promoted(VERSION),
        ],
    )
    .extension(VK_EXT_PIPELINE_PROTECTED_ACCESS)
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDevicePipelineRobustnessFeatures",
        "PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES",
        VK_EXT_PIPELINE_ROBUSTNESS.ext_enum(1) as _,
        StructUsage::Both,
        &[Struct::member("pipelineRobustness", TY_VK_BOOL)],
    )
    .extensions(&[VK_EXT_PIPELINE_ROBUSTNESS])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDevicePipelineRobustnessProperties",
        "PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES",
        VK_EXT_PIPELINE_ROBUSTNESS.ext_enum(2) as _,
        StructUsage::Sink,
        &[
            Struct::member(
                "defaultRobustnessStorageBuffers",
                "VkPipelineRobustnessBufferBehaviorEXT",
            ),
            Struct::member(
                "defaultRobustnessUniformBuffers",
                "VkPipelineRobustnessBufferBehaviorEXT",
            ),
            Struct::member("defaultRobustnessVertexInputs", "VkPipelineRobustnessBufferBehaviorEXT"),
            Struct::member("defaultRobustnessImage", "VkPipelineRobustnessImageBehaviorEXT"),
        ],
    )
    .extensions(&[VK_EXT_PIPELINE_ROBUSTNESS])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PipelineRobustnessCreateInfo",
        "PIPELINE_ROBUSTNESS_CREATE_INFO",
        VK_EXT_PIPELINE_ROBUSTNESS.ext_enum(0) as _,
        StructUsage::Source,
        &[
            Struct::member("storageBuffers", "VkPipelineRobustnessBufferBehaviorEXT"),
            Struct::member("uniformBuffers", "VkPipelineRobustnessBufferBehaviorEXT"),
            Struct::member("vertexInputs", "VkPipelineRobustnessBufferBehaviorEXT"),
            Struct::member("images", "VkPipelineRobustnessImageBehaviorEXT"),
        ],
    )
    .extensions(&[VK_EXT_PIPELINE_ROBUSTNESS])
    .promoted(VERSION)
    .into_element(),
];
