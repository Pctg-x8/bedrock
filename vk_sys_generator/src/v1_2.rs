//! v1.2 promoted elements

use crate::{parts::*, vk_ext_enum};

const VERSION: &str = "1_2";
const VK_KHR_BUFFER_DEVICE_ADDRESS: &Extension = &Extension::khr("buffer_device_address", 1, 258);
const VK_KHR_TIMELINE_SEMAPHORE: &Extension = &Extension::khr("timeline_semaphore", 2, 208);
const VK_KHR_IMAGE_FORMAT_LIST: &Extension = &Extension::khr("image_format_list", 1, 148);
const VK_EXT_SAMPLER_FILTER_MINMAX: &Extension = &Extension::ext("sampler_filter_minmax", 1, 131);
const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE: &Extension = &Extension::khr("sampler_mirror_clamp_to_edge", 3, 15);
const VK_KHR_SHADER_FLOAT_CONTROLS: &Extension = &Extension::khr("shader_float_controls", 4, 198);
const VK_EXT_SHADER_VIEWPORT_INDEX_LAYER: &Extension = &Extension::ext("shader_viewport_index_layer", 1, 163);
pub const VK_KHR_CREATE_RENDERPASS_2: &Extension = &Extension::khr("create_renderpass2", 1, 110);
pub const VK_KHR_DEPTH_STENCIL_RESOLVE: &Extension = &Extension::khr("depth_stencil_resolve", 1, 200);
pub const VK_EXT_DESCRIPTOR_INDEXING: &Extension = &Extension::ext("descriptor_indexing", 2, 162);

pub const ELEMENTS: &[Element] = &[
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceVulkan11Features",
            "PHYSICAL_DEVICE_VULKAN_1_1_FEATURES",
            49,
            StructUsage::Both,
            &[
                Struct::member("storageBuffer16BitAccess", "VkBool32"),
                Struct::member("uniformAndStorageBuffer16BitAccess", "VkBool32"),
                Struct::member("storagePushConstant16", "VkBool32"),
                Struct::member("storageInputOutput16", "VkBool32"),
                Struct::member("multiview", "VkBool32"),
                Struct::member("multiviewGeometryShader", "VkBool32"),
                Struct::member("multiviewTessellationShader", "VkBool32"),
                Struct::member("variablePointersStorageBuffer", "VkBool32"),
                Struct::member("variablePointers", "VkBool32"),
                Struct::member("protectedMemory", "VkBool32"),
                Struct::member("samplerYcbcrConversion", "VkBool32"),
                Struct::member("shaderDrawParameters", "VkBool32"),
            ],
        )
        .available_condition("feature = \"Allow1_2APIs\"")
        .default_zero(),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceVulkan11Properties",
            "PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES",
            50,
            StructUsage::Sink,
            &[
                Struct::member("deviceUUID", "[u8; VK_UUID_SIZE]"),
                Struct::member("driverUUID", "[u8; VK_UUID_SIZE]"),
                Struct::member("deviceLUID", "[u8; VK_LUID_SIZE]"),
                Struct::member("deviceNodeMask", "u32"),
                Struct::member("deviceLUIDValid", "VkBool32"),
                Struct::member("subgroupSize", "u32"),
                Struct::member("subgroupSupportedStages", "VkShaderStageFlags"),
                Struct::member("subgroupSupportedOperations", "VkSubgroupFeatureFlags"),
                Struct::member("subgroupQuadOperationsInAllStages", "VkBool32"),
                Struct::member("pointClippingBehavior", "VkPointClippingBehavior"),
                Struct::member("maxMultiviewViewCount", "u32"),
                Struct::member("maxMultiviewInstanceIndex", "u32"),
                Struct::member("protectedNoFault", "VkBool32"),
                Struct::member("maxPerSetDescriptors", "u32"),
                Struct::member("maxMemoryAllocationSize", "VkDeviceSize"),
            ],
        )
        .available_condition("feature = \"Allow1_2APIs\""),
    ),
    // VK_KHR_buffer_device_address
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_BUFFER_DEVICE_ADDRESS)),
    Element::Enum(Enum::extending_error(&[Enum::member(
        "INVALID_OPAQUE_CAPTURE_ADDRESS",
        -vk_ext_enum(258, 0) as _,
    )
    .extension(VK_KHR_BUFFER_DEVICE_ADDRESS)
    .promoted(VERSION)])),
    Element::Bitmask(Bitmask::extending(
        "BufferCreateFlagBits",
        "BUFFER_CREATE",
        &[Bitmask::entry("DEVICE_ADDRESS_CAPTURE_REPLAY", 4)
            .extension(VK_KHR_BUFFER_DEVICE_ADDRESS)
            .promoted(VERSION)],
    )),
    Element::Bitmask(Bitmask::extending(
        "BufferUsageFlagBits",
        "BUFFER_USAGE",
        &[Bitmask::entry("SHADER_DEVICE_ADDRESS", 17)
            .extension(VK_KHR_BUFFER_DEVICE_ADDRESS)
            .promoted(VERSION)],
    )),
    Element::Bitmask(Bitmask::extending(
        "MemoryAllocateFlagBits",
        "MEMORY_ALLOCATE",
        &[
            Bitmask::entry("DEVICE_ADDRESS", 1)
                .extension(VK_KHR_BUFFER_DEVICE_ADDRESS)
                .promoted(VERSION),
            Bitmask::entry("DEVICE_ADDRESS_CAPTURE_REPLAY", 2)
                .extension(VK_KHR_BUFFER_DEVICE_ADDRESS)
                .promoted(VERSION),
        ],
    )),
    Element::Struct(
        Struct::typed(
            "BufferDeviceAddressInfo",
            "BUFFER_DEVICE_ADDRESS_INFO",
            vk_ext_enum(245, 1) as _,
            StructUsage::Source,
            &[Struct::member("buffer", "VkBuffer")],
        )
        .extensions(&[VK_KHR_BUFFER_DEVICE_ADDRESS])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "BufferOpaqueCaptureAddressCreateInfo",
            "BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO",
            vk_ext_enum(258, 2) as _,
            StructUsage::Source,
            &[Struct::member("opaqueCaptureAddress", "u64")],
        )
        .extensions(&[VK_KHR_BUFFER_DEVICE_ADDRESS])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "DeviceMemoryOpaqueCaptureAddressInfo",
            "DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO",
            vk_ext_enum(258, 4) as _,
            StructUsage::Source,
            &[Struct::member("memory", "VkDeviceMemory")],
        )
        .extensions(&[VK_KHR_BUFFER_DEVICE_ADDRESS])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "MemoryOpaqueCaptureAddressAllocateInfo",
            "MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO",
            vk_ext_enum(258, 3) as _,
            StructUsage::Source,
            &[Struct::member("opaqueCaptureAddress", "u64")],
        )
        .extensions(&[VK_KHR_BUFFER_DEVICE_ADDRESS])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceBufferDeviceAddressFeatures",
            "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES",
            vk_ext_enum(258, 0) as _,
            StructUsage::Both,
            &[
                Struct::member("bufferDeviceAddress", "VkBool32"),
                Struct::member("bufferDeviceAddressCaptureReplay", "VkBool32"),
                Struct::member("bufferDeviceAddressMultiDevice", "VkBool32"),
            ],
        )
        .extensions(&[VK_KHR_BUFFER_DEVICE_ADDRESS])
        .promoted(VERSION),
    ),
    Element::Command(
        Command::new(
            "GetBufferDeviceAddress",
            &[("device", "VkDevice"), ("pInfo", "*const VkBufferDeviceAddressInfoKHR")],
        )
        .returns("VkDeviceAddress")
        .extension(VK_KHR_BUFFER_DEVICE_ADDRESS)
        .promoted(VERSION),
    ),
    Element::Command(
        Command::new(
            "GetBufferOpaqueCaptureAddress",
            &[("device", "VkDevice"), ("pInfo", "*const VkBufferDeviceAddressInfoKHR")],
        )
        .returns("u64")
        .extension(VK_KHR_BUFFER_DEVICE_ADDRESS)
        .promoted(VERSION),
    ),
    Element::Command(
        Command::new(
            "GetDeviceMemoryOpaqueCaptureAddress",
            &[
                ("device", "VkDevice"),
                ("pInfo", "*const VkDeviceMemoryOpaqueCaptureAddressInfoKHR"),
            ],
        )
        .returns("u64")
        .extension(VK_KHR_BUFFER_DEVICE_ADDRESS)
        .promoted(VERSION),
    ),
    // VK_KHR_timeline_semaphore
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_TIMELINE_SEMAPHORE)),
    Element::Enum(
        Enum::new(
            "SemaphoreType",
            "SEMAPHORE_TYPE",
            &[
                Enum::member("BINARY", 0)
                    .extension(VK_KHR_TIMELINE_SEMAPHORE)
                    .promoted(VERSION),
                Enum::member("TIMELINE", 1)
                    .extension(VK_KHR_TIMELINE_SEMAPHORE)
                    .promoted(VERSION),
            ],
        )
        .extension(VK_KHR_TIMELINE_SEMAPHORE)
        .promoted(VERSION),
    ),
    Element::Bitmask(
        Bitmask::new(
            "SemaphoreWaitFlags",
            "SemaphoreWaitFlagBits",
            "SEMAPHORE_WAIT",
            &[Bitmask::entry("ANY", 0)
                .extension(VK_KHR_TIMELINE_SEMAPHORE)
                .promoted(VERSION)],
        )
        .extension(VK_KHR_TIMELINE_SEMAPHORE)
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceTimelineSemaphoreFeatures",
            "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES",
            vk_ext_enum(208, 0) as _,
            StructUsage::Both,
            &[Struct::member("timelineSemaphore", "VkBool32")],
        )
        .extensions(&[VK_KHR_TIMELINE_SEMAPHORE])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceTimelineSemaphoreProperties",
            "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES",
            vk_ext_enum(208, 1) as _,
            StructUsage::Sink,
            &[Struct::member("maxTimelineSemaphoreValueDifference", "u64")],
        )
        .extensions(&[VK_KHR_TIMELINE_SEMAPHORE])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "SemaphoreSignalInfo",
            "SEMAPHORE_SIGNAL_INFO",
            vk_ext_enum(208, 5) as _,
            StructUsage::Source,
            &[
                Struct::member("semaphore", "VkSemaphore"),
                Struct::member("value", "u64"),
            ],
        )
        .extensions(&[VK_KHR_TIMELINE_SEMAPHORE])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "SemaphoreTypeCreateInfo",
            "SEMAPHORE_TYPE_CREATE_INFO",
            vk_ext_enum(208, 2) as _,
            StructUsage::Source,
            &[
                Struct::member("semaphoreType", "VkSemaphoreTypeKHR"),
                Struct::member("initialValue", "u64"),
            ],
        )
        .extensions(&[VK_KHR_TIMELINE_SEMAPHORE])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "SemaphoreWaitInfo",
            "SEMAPHORE_WAIT_INFO",
            vk_ext_enum(208, 4) as _,
            StructUsage::Source,
            &[
                Struct::member("flags", "VkSemaphoreWaitFlagsKHR"),
                Struct::member("semaphoreCount", "u32"),
                Struct::member("pSemaphores", "*const VkSemaphore"),
                Struct::member("pValues", "*const u64"),
            ],
        )
        .extensions(&[VK_KHR_TIMELINE_SEMAPHORE])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "TimelineSemaphoreSubmitInfo",
            "TIMELINE_SEMAPHORE_SUBMIT_INFO",
            vk_ext_enum(208, 3) as _,
            StructUsage::Source,
            &[
                Struct::member("waitSemaphoreValueCount", "u32"),
                Struct::member("pWaitSemaphoreValues", "*const u64"),
                Struct::member("signalSemaphoreValueCount", "u32"),
                Struct::member("pSignalSemaphoreValues", "*const u64"),
            ],
        )
        .extensions(&[VK_KHR_TIMELINE_SEMAPHORE])
        .promoted(VERSION),
    ),
    Element::Command(
        Command::new(
            "GetSemaphoreCounterValue",
            &[
                ("device", "VkDevice"),
                ("semaphore", "VkSemaphore"),
                ("pValue", "*mut u64"),
            ],
        )
        .failable()
        .extension(VK_KHR_TIMELINE_SEMAPHORE)
        .promoted(VERSION),
    ),
    Element::Command(
        Command::new(
            "SignalSemaphore",
            &[
                ("device", "VkDevice"),
                ("pSignalInfo", "*const VkSemaphoreSignalInfoKHR"),
            ],
        )
        .failable()
        .extension(VK_KHR_TIMELINE_SEMAPHORE)
        .promoted(VERSION),
    ),
    Element::Command(
        Command::new(
            "WaitSemaphores",
            &[
                ("device", "VkDevice"),
                ("pWaitInfo", "*const VkSemaphoreWaitInfoKHR"),
                ("timeout", "u64"),
            ],
        )
        .failable()
        .extension(VK_KHR_TIMELINE_SEMAPHORE)
        .promoted(VERSION),
    ),
    // VK_KHR_image_format_list
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_IMAGE_FORMAT_LIST)),
    Element::Struct(
        Struct::typed(
            "ImageFormatListCreateInfo",
            "IMAGE_FORMAT_LIST_CREATE_INFO",
            vk_ext_enum(148, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("viewFormatCount", "u32"),
                Struct::member("pViewFormats", "*const VkFormat"),
            ],
        )
        .extensions(&[VK_KHR_IMAGE_FORMAT_LIST])
        .promoted(VERSION),
    ),
    // VK_EXT_sampler_filter_minmax
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_EXT_SAMPLER_FILTER_MINMAX)),
    Element::Enum(
        Enum::new(
            "SamplerReductionMode",
            "SAMPLER_REDUCTION_MODE",
            &[
                Enum::member("WEIGHTED_AVERAGE", 0)
                    .extension(VK_EXT_SAMPLER_FILTER_MINMAX)
                    .promoted(VERSION),
                Enum::member("MIN", 1)
                    .extension(VK_EXT_SAMPLER_FILTER_MINMAX)
                    .promoted(VERSION),
                Enum::member("MAX", 2)
                    .extension(VK_EXT_SAMPLER_FILTER_MINMAX)
                    .promoted(VERSION),
            ],
        )
        .extension(VK_EXT_SAMPLER_FILTER_MINMAX)
        .promoted(VERSION),
    ),
    Element::Bitmask(Bitmask::extending(
        "FormatFeatureFlagBits",
        "FORMAT_FEATURE",
        &[Bitmask::entry("SAMPLED_IMAGE_FILTER_MINMAX", 16)
            .extension(VK_EXT_SAMPLER_FILTER_MINMAX)
            .promoted(VERSION)],
    )),
    Element::Struct(
        Struct::typed(
            "SamplerReductionModeCreateInfo",
            "SAMPLER_REDUCTION_MODE_CREATE_INFO",
            vk_ext_enum(131, 1) as _,
            StructUsage::Source,
            &[Struct::member("reductionMode", "VkSamplerReductionModeEXT")],
        )
        .extensions(&[VK_EXT_SAMPLER_FILTER_MINMAX])
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceSamplerFilterMinmaxProperties",
            "PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES",
            vk_ext_enum(131, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("filterMinmaxSingleComponentFormats", TY_VK_BOOL),
                Struct::member("filterMinmaxImageComponentMapping", TY_VK_BOOL),
            ],
        )
        .extensions(&[VK_EXT_SAMPLER_FILTER_MINMAX])
        .promoted(VERSION),
    ),
    // VK_KHR_sampler_mirror_clamp_to_edge
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE)),
    Element::Enum(Enum::extending(
        "SamplerAddressMode",
        "SAMPLER_ADDRESS_MODE",
        &[Enum::member("MIRROR_CLAMP_TO_EDGE", 4)
            .extension(VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE)
            .promoted(VERSION)],
    )),
    // VK_KHR_shader_float_controls
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_SHADER_FLOAT_CONTROLS)),
    Element::Enum(
        Enum::new(
            "ShaderFloatControlsIndependence",
            "SHADER_FLOAT_CONTROLS_INDEPENDENCE",
            &[
                Enum::member("32_BIT_ONLY", 0)
                    .extension(VK_KHR_SHADER_FLOAT_CONTROLS)
                    .promoted(VERSION),
                Enum::member("ALL", 1)
                    .extension(VK_KHR_SHADER_FLOAT_CONTROLS)
                    .promoted(VERSION),
                Enum::member("NONE", 2)
                    .extension(VK_KHR_SHADER_FLOAT_CONTROLS)
                    .promoted(VERSION),
            ],
        )
        .extension(VK_KHR_SHADER_FLOAT_CONTROLS),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceFloatControlsProperties",
            "PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES",
            vk_ext_enum(198, 0) as _,
            StructUsage::Sink,
            &[
                Struct::member("denormBehaviorIndependence", "VkShaderFloatControlsIndependenceKHR"),
                Struct::member("roundingModeIndependence", "VkShaderFloatControlsIndependenceKHR"),
                Struct::member("shaderSignedZeroInfNanPreserveFloat16", TY_VK_BOOL),
                Struct::member("shaderSignedZeroInfNanPreserveFloat32", TY_VK_BOOL),
                Struct::member("shaderSignedZeroInfNanPreserveFloat64", TY_VK_BOOL),
                Struct::member("shaderDenormPreserveFloat16", TY_VK_BOOL),
                Struct::member("shaderDenormPreserveFloat32", TY_VK_BOOL),
                Struct::member("shaderDenormPreserveFloat64", TY_VK_BOOL),
                Struct::member("shaderDenormFlushToZeroFloat16", TY_VK_BOOL),
                Struct::member("shaderDenormFlushToZeroFloat32", TY_VK_BOOL),
                Struct::member("shaderDenormFlushToZeroFloat64", TY_VK_BOOL),
                Struct::member("shaderRoundingModeRTEFloat16", TY_VK_BOOL),
                Struct::member("shaderRoundingModeRTEFloat32", TY_VK_BOOL),
                Struct::member("shaderRoundingModeRTEFloat64", TY_VK_BOOL),
                Struct::member("shaderRoundingModeRTZFloat16", TY_VK_BOOL),
                Struct::member("shaderRoundingModeRTZFloat32", TY_VK_BOOL),
                Struct::member("shaderRoundingModeRTZFloat64", TY_VK_BOOL),
            ],
        )
        .extensions(&[VK_KHR_SHADER_FLOAT_CONTROLS])
        .promoted(VERSION),
    ),
    // VK_EXT_shader_viewport_index_layer
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_EXT_SHADER_VIEWPORT_INDEX_LAYER)),
    // VK_KHR_create_renderpass2
    VK_KHR_CREATE_RENDERPASS_2.header_constants().into_element(),
    Struct::typed(
        "RenderPassCreateInfo2",
        "RENDER_PASS_CREATE_INFO_2",
        VK_KHR_CREATE_RENDERPASS_2.ext_enum(4) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkRenderPassCreateFlags"),
            Struct::member("attachmentCount", "u32"),
            Struct::member("pAttachments", "*const VkAttachmentDescription2KHR"),
            Struct::member("subpassCount", "u32"),
            Struct::member("pSubpasses", "*const VkSubpassDescription2KHR"),
            Struct::member("ependencyCount", "u32"),
            Struct::member("pDependencies", "*const VkSubpassDependency2KHR"),
            Struct::member("correlatedViewMaskCount", "u32"),
            Struct::member("pCorrellatedViewMasks", "*const u32"),
        ],
    )
    .extensions(&[VK_KHR_CREATE_RENDERPASS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "AttachmentDescription2",
        "ATTACHMENT_DESCRIPTION_2",
        VK_KHR_CREATE_RENDERPASS_2.ext_enum(0) as _,
        StructUsage::Source,
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
    )
    .extensions(&[VK_KHR_CREATE_RENDERPASS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "AttachmentReference2",
        "ATTACHMENT_REFERENCE_2",
        VK_KHR_CREATE_RENDERPASS_2.ext_enum(1) as _,
        StructUsage::Source,
        &[
            Struct::member("attachment", "u32"),
            Struct::member("layout", "VkImageLayout"),
            Struct::member("aspectMask", "VkImageAspectFlags"),
        ],
    )
    .extensions(&[VK_KHR_CREATE_RENDERPASS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SubpassDescription2",
        "SUBPASS_DESCRIPTION_2",
        VK_KHR_CREATE_RENDERPASS_2.ext_enum(2) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkSubpassDescriptionFlags"),
            Struct::member("pipelineBindPoint", "VkPipelineBindPoint"),
            Struct::member("viewMask", "u32"),
            Struct::member("inputAttachmentCount", "u32"),
            Struct::member("pInputAttachments", "*const VkAttachmentReference2KHR"),
            Struct::member("colorAttachmentCount", "u32"),
            Struct::member("pColorAttachments", "*const VkAttachmentReference2KHR"),
            Struct::member("pResolveAttachments", "*const VkAttachmentReference2KHR"),
            Struct::member("pDepthStencilAtachment", "*const VkAttachmentReference2KHR"),
            Struct::member("preserveAttachmentCount", "u32"),
            Struct::member("pPreserveAttachments", "*const u32"),
        ],
    )
    .extensions(&[VK_KHR_CREATE_RENDERPASS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SubpasDependency2",
        "SUBPASS_DEPENDENCY_2",
        VK_KHR_CREATE_RENDERPASS_2.ext_enum(3) as _,
        StructUsage::Source,
        &[
            Struct::member("srcSubpass", "u32"),
            Struct::member("dstSubpass", "u32"),
            Struct::member("srcStageMask", "VkPipelineStageFlags"),
            Struct::member("dstStageMask", "VkPipelineStageFlags"),
            Struct::member("srcAccessMask", "VkAccessFlags"),
            Struct::member("dstAccessMask", "VkAccessFlags"),
            Struct::member("dependencyFlags", "VkDependencyFlags"),
            Struct::member("viewOffset", "i32"),
        ],
    )
    .extensions(&[VK_KHR_CREATE_RENDERPASS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SubpassBeginInfo",
        "SUBPASS_BEGIN_INFO",
        VK_KHR_CREATE_RENDERPASS_2.ext_enum(5) as _,
        StructUsage::Source,
        &[Struct::member("contents", "VkSubpassContents")],
    )
    .extensions(&[VK_KHR_CREATE_RENDERPASS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SubpassEndInfo",
        "SUBPASS_END_INFO",
        VK_KHR_CREATE_RENDERPASS_2.ext_enum(6) as _,
        StructUsage::Source,
        &[],
    )
    .extensions(&[VK_KHR_CREATE_RENDERPASS_2])
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "CreateRenderPass2",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "*const VkRenderPassCreateInfo2KHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pRenderPass", "*mut VkRenderPass"),
        ],
    )
    .failable()
    .extension(VK_KHR_CREATE_RENDERPASS_2)
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "BeginRenderPass2",
        &[
            ("pRenderPassBegin", "*const VkRenderPassBeginInfo"),
            ("pSubpassBeginInfo", "*const VkSubpassBeginInfoKHR"),
        ],
    )
    .extension(VK_KHR_CREATE_RENDERPASS_2)
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "NextSubpass2",
        &[
            ("pSubpassBeginInfo", "*const VkSubpassBeginInfoKHR"),
            ("pSubpassEndInfo", "*const VkSubpassEndInfoKHR"),
        ],
    )
    .extension(VK_KHR_CREATE_RENDERPASS_2)
    .promoted(VERSION)
    .into_element(),
    Command::inst("EndRenderPass2", &[("pSubpassEndInfo", "*const VkSubpassEndInfoKHR")])
        .extension(VK_KHR_CREATE_RENDERPASS_2)
        .promoted(VERSION)
        .into_element(),
    // VK_KHR_depth_stencil_resolve
    VK_KHR_DEPTH_STENCIL_RESOLVE.header_constants().into_element(),
    Bitmask::new(
        "ResolveModeFlags",
        "ResolveModeFlagBits",
        "RESOLVE_MODE",
        &[
            Bitmask::entry("SAMPLE_ZERO", 0)
                .extension(VK_KHR_DEPTH_STENCIL_RESOLVE)
                .promoted(VERSION),
            Bitmask::entry("AVERAGE", 1)
                .extension(VK_KHR_DEPTH_STENCIL_RESOLVE)
                .promoted(VERSION),
            Bitmask::entry("MIN", 2)
                .extension(VK_KHR_DEPTH_STENCIL_RESOLVE)
                .promoted(VERSION),
            Bitmask::entry("MAX", 3)
                .extension(VK_KHR_DEPTH_STENCIL_RESOLVE)
                .promoted(VERSION),
        ],
    )
    .extension(VK_KHR_DEPTH_STENCIL_RESOLVE)
    .promoted(VERSION)
    .into_element(),
    // VK_RESOLVE_MODE_NONE_KHR defined in main
    Struct::typed(
        "PhysicalDeviceDepthStencilResolveProperties",
        "PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES",
        VK_KHR_DEPTH_STENCIL_RESOLVE.ext_enum(0) as _,
        StructUsage::Sink,
        &[
            Struct::member("supportedDepthREsolveModes", "VkResolveModeFlagsKHR"),
            Struct::member("supportedStencilResolveModes", "VkResolveModeFlagsKHR"),
            Struct::member("independentResolveNone", TY_VK_BOOL),
            Struct::member("independentREsolve", TY_VK_BOOL),
        ],
    )
    .extensions(&[VK_KHR_DEPTH_STENCIL_RESOLVE])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SubpassDescriptionDepthStencilResolve",
        "SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE",
        VK_KHR_DEPTH_STENCIL_RESOLVE.ext_enum(1) as _,
        StructUsage::Source,
        &[
            Struct::member("depthResolveMode", "VkResolveModeFlagBitsKHR"),
            Struct::member("stencilResolveMode", "VkResolveModeFlagBitsKHR"),
            Struct::member("pDepthStencilResolveAttachment", "*const VkAttachmentReference2KHR"),
        ],
    )
    .extensions(&[VK_KHR_DEPTH_STENCIL_RESOLVE])
    .promoted(VERSION)
    .into_element(),
    // VK_EXT_descriptor_indexing
    VK_EXT_DESCRIPTOR_INDEXING.header_constants().into_element(),
    Enum::extending_error(&[
        Enum::member("FRAGMENTATION", -VK_EXT_DESCRIPTOR_INDEXING.ext_enum(0) as _)
            .extension(VK_EXT_DESCRIPTOR_INDEXING)
            .promoted(VERSION),
    ])
    .into_element(),
    Bitmask::new(
        "DescriptorBindingFlags",
        "DescriptorBindingFlagBits",
        "DESCRIPTOR_BINDING",
        &[
            Bitmask::entry("UPDATE_AFTER_BIND", 0)
                .extension(VK_EXT_DESCRIPTOR_INDEXING)
                .promoted(VERSION),
            Bitmask::entry("UPDATE_UNUSED_WHILE_PENDING", 1)
                .extension(VK_EXT_DESCRIPTOR_INDEXING)
                .promoted(VERSION),
            Bitmask::entry("PARTIALLY_BOUND", 2)
                .extension(VK_EXT_DESCRIPTOR_INDEXING)
                .promoted(VERSION),
            Bitmask::entry("VARIABLE_DESCRIPTOR_COUNT", 3)
                .extension(VK_EXT_DESCRIPTOR_INDEXING)
                .promoted(VERSION),
        ],
    )
    .extension(VK_EXT_DESCRIPTOR_INDEXING)
    .promoted(VERSION)
    .into_element(),
    Bitmask::extending(
        "DescriptorPoolCreateFlagBits",
        "DESCRIPTOR_POOL_CREATE",
        &[Bitmask::entry("UPDATE_AFTER_BIND", 2)
            .extension(VK_EXT_DESCRIPTOR_INDEXING)
            .promoted(VERSION)],
    )
    .into_element(),
    Bitmask::extending(
        "DescriptorSetLayoutCreateFlagBits",
        "DESCRIPTOR_SET_LAYOUT_CREATE",
        &[Bitmask::entry("UPDATE_AFTER_BIND_POOL", 1)
            .extension(VK_EXT_DESCRIPTOR_INDEXING)
            .promoted(VERSION)],
    )
    .into_element(),
    Struct::typed(
        "DescriptorSetLayoutBindingFlagsCreateInfo",
        "DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO",
        VK_EXT_DESCRIPTOR_INDEXING.ext_enum(0) as _,
        StructUsage::Source,
        &[
            Struct::member("bindingCount", "u32"),
            Struct::member("pBindingFlags", "*const VkDescriptorBindingFlagsEXT"),
        ],
    )
    .extensions(&[VK_EXT_DESCRIPTOR_INDEXING])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceDescriptorIndexingFeatures",
        "PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES",
        VK_EXT_DESCRIPTOR_INDEXING.ext_enum(1) as _,
        StructUsage::Both,
        &[
            Struct::member("shaderInputAttachmentArrayDynamicIndexing", TY_VK_BOOL),
            Struct::member("shaderUniformTexelBufferArrayDynamicIndexing", TY_VK_BOOL),
            Struct::member("shaderStorageTexelBufferArrayDynamicIndexing", TY_VK_BOOL),
            Struct::member("shaderUniformBufferArrayNonUniformIndexing", TY_VK_BOOL),
            Struct::member("shaderSampledImageArrayNonUniformIndexing", TY_VK_BOOL),
            Struct::member("shaderStorageBufferArrayNonUniformIndexing", TY_VK_BOOL),
            Struct::member("shaderStorageImageArrayNonUniformIndexing", TY_VK_BOOL),
            Struct::member("shaderInputAttachmentArrayNonUniformIndexing", TY_VK_BOOL),
            Struct::member("shaderUniformTexelBufferArrayNonUniformIndexing", TY_VK_BOOL),
            Struct::member("shaderStorageTexelBufferArrayNonUniformIndexing", TY_VK_BOOL),
            Struct::member("descriptorBindingUniformBufferUpdateAfterBind", TY_VK_BOOL),
            Struct::member("descriptorBindingSampledImageUpdateAfterBind", TY_VK_BOOL),
            Struct::member("descriptorBindingStorageImageUpdateAfterBind", TY_VK_BOOL),
            Struct::member("descriptorBindingStorageBufferUpdateAfterBind", TY_VK_BOOL),
            Struct::member("descriptorBindingUniformTexelBufferUpdateAfterBind", TY_VK_BOOL),
            Struct::member("descriptorBindingStorageTexelBufferUpdateAfterBind", TY_VK_BOOL),
            Struct::member("descriptorBindingUpdateUnusedWhilePending", TY_VK_BOOL),
            Struct::member("descriptorBindingPartiallyBound", TY_VK_BOOL),
            Struct::member("descriptorBindingVariableDescriptorCount", TY_VK_BOOL),
            Struct::member("runtimeDescriptorArray", TY_VK_BOOL),
        ],
    )
    .extensions(&[VK_EXT_DESCRIPTOR_INDEXING])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceDescriptorIndexingProperties",
        "PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES",
        VK_EXT_DESCRIPTOR_INDEXING.ext_enum(2) as _,
        StructUsage::Sink,
        &[
            Struct::member("maxUpdateAfterBindDescriptorsInAllPools", "u32"),
            Struct::member("shaderUniformBufferArrayNonUniformIndexingNative", TY_VK_BOOL),
            Struct::member("shaderSampledImageArrayNonUniformIndexingNative", TY_VK_BOOL),
            Struct::member("shaderStorageBufferArrayNonUniformIndexingNative", TY_VK_BOOL),
            Struct::member("shaderStorageImageArrayNonUniformIndexingNative", TY_VK_BOOL),
            Struct::member("shaderInputAttachmentArrayNonUniformIndexingNative", TY_VK_BOOL),
            Struct::member("robustBufferAccessUpdateAfterBind", TY_VK_BOOL),
            Struct::member("quadDivergentImplicitLod", TY_VK_BOOL),
            Struct::member("maxPerStageDescriptorUpdateAfterBindSamplers", "u32"),
            Struct::member("maxPerStageDescriptorUpdateAfterBindUniformBuffers", "u32"),
            Struct::member("maxPerStageDescriptorUpdateAfterBindStorageBuffers", "u32"),
            Struct::member("maxPerStageDescriptorUpdateAfterBindSampledImages", "u32"),
            Struct::member("maxPerStageDescriptorUpdateAfterBindStorageImages", "u32"),
            Struct::member("maxPerStageDescriptorUpdateAfterBindInputAttachments", "u32"),
            Struct::member("maxPerStageUpdateAfterBindResources", "u32"),
            Struct::member("maxDescriptorSetUpdateAfterBindSamplers", "u32"),
            Struct::member("maxDescriptorSetUpdateAfterBindUniformBuffers", "u32"),
            Struct::member("maxDescriptorSetUpdateAfterBindUniformBuffersDynamic", "u32"),
            Struct::member("maxDescriptorSetUpdateAfterBindStorageBuffers", "u32"),
            Struct::member("maxDescriptorSetUpdateAfterBindStorageBuffersDynamic", "u32"),
            Struct::member("maxDescriptorSetUpdateAfterBindSampledImages", "u32"),
            Struct::member("maxDescriptorSetUpdateAfterBindStorageImages", "u32"),
            Struct::member("maxDescriptorSetUpdateAfterBindInputAttachments", "u32"),
        ],
    )
    .extensions(&[VK_EXT_DESCRIPTOR_INDEXING])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DescriptorSetVariableDescriptorCountAllocateInfo",
        "DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO",
        VK_EXT_DESCRIPTOR_INDEXING.ext_enum(3) as _,
        StructUsage::Source,
        &[
            Struct::member("descriptorSetCount", "u32"),
            Struct::member("pDescriptorCounts", "*const u32"),
        ],
    )
    .extensions(&[VK_EXT_DESCRIPTOR_INDEXING])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DescriptorSetVariableDescriptorCountLayoutSupport",
        "DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT",
        VK_EXT_DESCRIPTOR_INDEXING.ext_enum(4) as _,
        StructUsage::Sink,
        &[Struct::member("maxVariableDescriptorCount", "u32")],
    )
    .extensions(&[VK_EXT_DESCRIPTOR_INDEXING])
    .promoted(VERSION)
    .into_element(),
];
