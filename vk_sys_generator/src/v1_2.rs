//! v1.2 promoted elements

use crate::{parts::*, vk_ext_enum};

const VERSION: &str = "1_2";

pub const ELEMENTS: &[Element] = &[
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_buffer_device_address", 1)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_timeline_semaphore", 2)),
    Element::Enum(Enum::extending_error(&[Enum::member(
        "INVALID_OPAQUE_CAPTURE_ADDRESS",
        -vk_ext_enum(258, 0) as _,
    )
    .extension("VK_KHR_buffer_device_address", "KHR")
    .promoted("1_2")])),
    Element::Enum(
        Enum::new(
            "SemaphoreType",
            "SEMAPHORE_TYPE",
            &[
                Enum::member("BINARY", 0)
                    .extension("VK_KHR_timeline_semaphore", "KHR")
                    .promoted("1_2"),
                Enum::member("TIMELINE", 1)
                    .extension("VK_KHR_timeline_semaphore", "KHR")
                    .promoted("1_2"),
            ],
        )
        .extension("VK_KHR_timeline_semaphore", "KHR")
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
];
