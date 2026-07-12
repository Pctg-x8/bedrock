//! v1.2 promoted elements

use crate::{parts::*, vk_ext_enum};

const VERSION: &str = "1_2";
const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE: &Extension = &Extension::khr("sampler_mirror_clamp_to_edge", 3);
const VK_KHR_SHADER_FLOAT_CONTROLS: &Extension = &Extension::khr("shader_float_controls", 4);
const VK_EXT_SHADER_VIEWPORT_INDEX_LAYER: &Extension = &Extension::ext("shader_viewport_index_layer", 1);

pub const ELEMENTS: &[Element] = &[
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_buffer_device_address", 1)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_timeline_semaphore", 2)),
    Element::Enum(Enum::extending_error(&[Enum::member(
        "INVALID_OPAQUE_CAPTURE_ADDRESS",
        -vk_ext_enum(258, 0) as _,
    )
    .extension("KHR", "buffer_device_address")
    .promoted("1_2")])),
    Element::Enum(
        Enum::new(
            "SemaphoreType",
            "SEMAPHORE_TYPE",
            &[
                Enum::member("BINARY", 0)
                    .extension("KHR", "timeline_semaphore")
                    .promoted("1_2"),
                Enum::member("TIMELINE", 1)
                    .extension("KHR", "timeline_semaphore")
                    .promoted("1_2"),
            ],
        )
        .extension("KHR", "timeline_semaphore")
        .promoted("1_2"),
    ),
    Element::Bitmask(Bitmask::extending(
        "BufferCreateFlagBits",
        "BUFFER_CREATE",
        &[Bitmask::entry("DEVICE_ADDRESS_CAPTURE_REPLAY", 4)
            .extension("KHR", "buffer_device_address")
            .promoted("1_2")],
    )),
    Element::Bitmask(Bitmask::extending(
        "BufferUsageFlagBits",
        "BUFFER_USAGE",
        &[Bitmask::entry("SHADER_DEVICE_ADDRESS", 17)
            .extension("KHR", "buffer_device_address")
            .promoted("1_2")],
    )),
    Element::Bitmask(Bitmask::extending(
        "MemoryAllocateFlagBits",
        "MEMORY_ALLOCATE",
        &[
            Bitmask::entry("DEVICE_ADDRESS", 1)
                .extension("KHR", "buffer_device_address")
                .promoted("1_2"),
            Bitmask::entry("DEVICE_ADDRESS_CAPTURE_REPLAY", 2)
                .extension("KHR", "buffer_device_address")
                .promoted("1_2"),
        ],
    )),
    Element::Bitmask(
        Bitmask::new(
            "SemaphoreWaitFlags",
            "SemaphoreWaitFlagBits",
            "SEMAPHORE_WAIT",
            &[Bitmask::entry("ANY", 0)
                .extension("KHR", "timeline_semaphore")
                .promoted("1_2")],
        )
        .extension("KHR", "timeline_semaphore")
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new("BufferDeviceAddressInfo", &[Struct::member("buffer", "VkBuffer")])
            .stype(
                "BUFFER_DEVICE_ADDRESS_INFO",
                vk_ext_enum(245, 1) as _,
                StructUsage::Source,
            )
            .extensions(&[("KHR", "buffer_device_address")])
            .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "BufferOpaqueCaptureAddressCreateInfo",
            &[Struct::member("opaqueCaptureAddress", "u64")],
        )
        .stype(
            "BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO",
            vk_ext_enum(258, 2) as _,
            StructUsage::Source,
        )
        .extensions(&[("KHR", "buffer_device_address")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "DeviceMemoryOpaqueCaptureAddressInfo",
            &[Struct::member("memory", "VkDeviceMemory")],
        )
        .stype(
            "DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO",
            vk_ext_enum(258, 4) as _,
            StructUsage::Source,
        )
        .extensions(&[("KHR", "buffer_device_address")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "MemoryOpaqueCaptureAddressAllocateInfo",
            &[Struct::member("opaqueCaptureAddress", "u64")],
        )
        .stype(
            "MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO",
            vk_ext_enum(258, 3) as _,
            StructUsage::Source,
        )
        .extensions(&[("KHR", "buffer_device_address")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "PhysicalDeviceBufferDeviceAddressFeatures",
            &[
                Struct::member("bufferDeviceAddress", "VkBool32"),
                Struct::member("bufferDeviceAddressCaptureReplay", "VkBool32"),
                Struct::member("bufferDeviceAddressMultiDevice", "VkBool32"),
            ],
        )
        .stype(
            "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES",
            vk_ext_enum(258, 0) as _,
            StructUsage::Both,
        )
        .extensions(&[("KHR", "buffer_device_address")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "PhysicalDeviceTimelineSemaphoreFeatures",
            &[Struct::member("timelineSemaphore", "VkBool32")],
        )
        .stype(
            "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES",
            vk_ext_enum(208, 0) as _,
            StructUsage::Both,
        )
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "PhysicalDeviceTimelineSemaphoreProperties",
            &[Struct::member("maxTimelineSemaphoreValueDifference", "u64")],
        )
        .stype(
            "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES",
            vk_ext_enum(208, 1) as _,
            StructUsage::Sink,
        )
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
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
    Element::Struct(
        Struct::new(
            "SemaphoreSignalInfo",
            &[
                Struct::member("semaphore", "VkSemaphore"),
                Struct::member("value", "u64"),
            ],
        )
        .stype("SEMAPHORE_SIGNAL_INFO", vk_ext_enum(208, 5) as _, StructUsage::Source)
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "SemaphoreTypeCreateInfo",
            &[
                Struct::member("semaphoreType", "VkSemaphoreTypeKHR"),
                Struct::member("initialValue", "u64"),
            ],
        )
        .stype(
            "SEMAPHORE_TYPE_CREATE_INFO",
            vk_ext_enum(208, 2) as _,
            StructUsage::Source,
        )
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "SemaphoreWaitInfo",
            &[
                Struct::member("flags", "VkSemaphoreWaitFlagsKHR"),
                Struct::member("semaphoreCount", "u32"),
                Struct::member("pSemaphores", "*const VkSemaphore"),
                Struct::member("pValues", "*const u64"),
            ],
        )
        .stype("SEMAPHORE_WAIT_INFO", vk_ext_enum(208, 4) as _, StructUsage::Source)
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Struct(
        Struct::new(
            "TimelineSemaphoreSubmitInfo",
            &[
                Struct::member("waitSemaphoreValueCount", "u32"),
                Struct::member("pWaitSemaphoreValues", "*const u64"),
                Struct::member("signalSemaphoreValueCount", "u32"),
                Struct::member("pSignalSemaphoreValues", "*const u64"),
            ],
        )
        .stype(
            "TIMELINE_SEMAPHORE_SUBMIT_INFO",
            vk_ext_enum(208, 3) as _,
            StructUsage::Source,
        )
        .extensions(&[("KHR", "timeline_semaphore")])
        .promoted("1_2"),
    ),
    Element::Command(
        Command::new(
            "GetBufferDeviceAddress",
            &[("device", "VkDevice"), ("pInfo", "*const VkBufferDeviceAddressInfoKHR")],
        )
        .returns("VkDeviceAddress")
        .extension("KHR", "buffer_device_address")
        .promoted("1_2"),
    ),
    Element::Command(
        Command::new(
            "GetBufferOpaqueCaptureAddress",
            &[("device", "VkDevice"), ("pInfo", "*const VkBufferDeviceAddressInfoKHR")],
        )
        .returns("u64")
        .extension("KHR", "buffer_device_address")
        .promoted("1_2"),
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
        .extension("KHR", "buffer_device_address")
        .promoted("1_2"),
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
        .extension("KHR", "timeline_semaphore")
        .promoted("1_2"),
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
        .extension("KHR", "timeline_semaphore")
        .promoted("1_2"),
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
        .extension("KHR", "timeline_semaphore")
        .promoted("1_2"),
    ),
    // VK_KHR_image_format_list
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_image_format_list", 1)),
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
        .extensions(&[ex_khr("image_format_list")])
        .promoted(VERSION),
    ),
    // VK_EXT_sampler_filter_minmax
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_sampler_filter_minmax", 1)),
    Element::Enum(
        Enum::new(
            "SamplerReductionMode",
            "SAMPLER_REDUCTION_MODE",
            &[
                Enum::member("WEIGHTED_AVERAGE", 0)
                    .extension("EXT", "sampler_filter_minmax")
                    .promoted(VERSION),
                Enum::member("MIN", 1)
                    .extension("EXT", "sampler_filter_minmax")
                    .promoted(VERSION),
                Enum::member("MAX", 2)
                    .extension("EXT", "sampler_filter_minmax")
                    .promoted(VERSION),
            ],
        )
        .extension("EXT", "sampler_filter_minmax")
        .promoted(VERSION),
    ),
    Element::Bitmask(Bitmask::extending(
        "FormatFeatureFlagBits",
        "FORMAT_FEATURE",
        &[Bitmask::entry("SAMPLED_IMAGE_FILTER_MINMAX", 16)
            .extension("EXT", "sampler_filter_minmax")
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
        .extensions(&[ex_ext("sampler_filter_minmax")])
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
        .extensions(&[ex_ext("sampler_filter_minmax")])
        .promoted(VERSION),
    ),
    // VK_KHR_sampler_mirror_clamp_to_edge
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE)),
    Element::Enum(Enum::extending(
        "SamplerAddressMode",
        "SMAPLER_ADDRESS_MODE",
        &[Enum::member("MIRROR_CLAMP_TO_EDGE", 4)
            .extension2(VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE)
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
                    .extension2(VK_KHR_SHADER_FLOAT_CONTROLS)
                    .promoted(VERSION),
                Enum::member("ALL", 1)
                    .extension2(VK_KHR_SHADER_FLOAT_CONTROLS)
                    .promoted(VERSION),
                Enum::member("NONE", 2)
                    .extension2(VK_KHR_SHADER_FLOAT_CONTROLS)
                    .promoted(VERSION),
            ],
        )
        .extension2(VK_KHR_SHADER_FLOAT_CONTROLS),
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
        .extensions2(&[VK_KHR_SHADER_FLOAT_CONTROLS])
        .promoted(VERSION),
    ),
    // VK_EXT_shader_viewport_index_layer
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_EXT_SHADER_VIEWPORT_INDEX_LAYER)),
];
