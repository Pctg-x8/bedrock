//! v1.1 promoted elements

use crate::{
    extensions::{VK_EXT_DEBUG_REPORT, VK_KHR_SURFACE, VK_KHR_SWAPCHAIN},
    parts::*,
    vk_ext_enum,
};

const VERSION: &str = "1_1";
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2: &Extension =
    &Extension::khr("get_physical_device_properties2", 2, 60).promoted(VERSION);
pub const VK_KHR_DEVICE_GROUP: &Extension = &Extension::khr("device_group", 4, 61).promoted(VERSION);
const VK_EXT_SHADER_SUBGROUP_VOTE: &Extension = &Extension::ext("shader_subgroup_vote", 1, 66).promoted(VERSION);
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE: &Extension =
    &Extension::khr("descriptor_update_template", 1, 86).promoted(VERSION);
pub const VK_KHR_DEDICATED_ALLOCATION: &Extension = &Extension::khr("dedicated_allocation", 1, 128).promoted(VERSION);
pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2: &Extension =
    &Extension::khr("get_memory_requirements2", 1, 147).promoted(VERSION);
const VK_KHR_SAMPLER_YCBCR_CONVERSION: &Extension =
    &Extension::khr("sampler_ycbcr_conversion", 14, 157).promoted(VERSION);
pub const VK_KHR_BIND_MEMORY_2: &Extension = &Extension::khr("bind_memory2", 1, 158).promoted(VERSION);

pub const ELEMENTS: &[Element] = &[
    // VK_KHR_get_physical_device_properties2
    VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2
        .header_constants()
        .into_element(),
    Struct::typed(
        "PhysicalDeviceFeatures2",
        "PHYSICAL_DEVICE_FEATURES_2",
        VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(0) as _,
        StructUsage::Both,
        &[Struct::member("features", "VkPhysicalDeviceFeatures")],
    )
    .extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceProperties2",
        "PHYSICAL_DEVICE_PROPERTIES_2",
        VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(1) as _,
        StructUsage::Sink,
        &[Struct::member("properties", "VkPhysicalDeviceProperties")],
    )
    .extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "FormatProperties2",
        "FORMAT_PROPERTIES_2",
        VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(2) as _,
        StructUsage::Sink,
        &[Struct::member("formatProperties", "VkFormatProperties")],
    )
    .extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImageFormatProperties2",
        "IMAGE_FORMAT_PROPERTIES_2",
        VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(3) as _,
        StructUsage::Sink,
        &[Struct::member("imageFormatProperties", "VkImageFormatProperties")],
    )
    .extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceImageFormatInfo2",
        "PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2",
        VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(4) as _,
        StructUsage::Source,
        &[
            Struct::member("format", "VkFormat"),
            Struct::member("r#type", "VkImageType"),
            Struct::member("tiling", "VkImageTiling"),
            Struct::member("usage", "VkImageUsageFlags"),
            Struct::member("flags", "VkImageCreateFlags"),
        ],
    )
    .extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "QueueFamilyProperties2",
        "QUEUE_FAMILY_PROPERTIES_2",
        VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(5) as _,
        StructUsage::Sink,
        &[Struct::member("queueFamilyProperties", "VkQueueFamilyProperties")],
    )
    .extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceMemoryProperties2",
        "PHYSICAL_DEVICE_MEMORY_PROPERTIES_2",
        VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(6) as _,
        StructUsage::Sink,
        &[Struct::member("memoryProperties", "VkPhysicalDeviceMemoryProperties")],
    )
    .extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SparseImageFormatProperties2",
        "SPARSE_IMAGE_FORMAT_PROPERTIES_2",
        VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(7) as _,
        StructUsage::Sink,
        &[Struct::member("properties", "VkSparseImageFormatProperties")],
    )
    .extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceSparseImageFormatInfo2",
        "PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2",
        VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(8) as _,
        StructUsage::Source,
        &[
            Struct::member("format", "VkFormat"),
            Struct::member("r#type", "VkImageType"),
            Struct::member("samples", "VkSampleCountFlagBits"),
            Struct::member("usage", "VkImageUsageFlags"),
            Struct::member("tiling", "VkImageTiling"),
        ],
    )
    .extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "GetPhysicalDeviceFeatures2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pFeatures", "*mut VkPhysicalDeviceFeatures2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .into_element(),
    Command::new(
        "GetPhysicalDeviceProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pProperties", "*mut VkPhysicalDeviceProperties2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .into_element(),
    Command::new(
        "GetPhysicalDeviceFormatProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("format", "VkFormat"),
            ("pFormatProperties", "*mut VkFormatProperties2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .into_element(),
    Command::new(
        "GetPhysicalDeviceImageFormatProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pImageFormatInfo", "*const VkPhysicalDeviceImageFormatInfo2KHR"),
            ("pImageFormatProperties", "*mut VkImageFormatProperties2KHR"),
        ],
    )
    .failable()
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .into_element(),
    Command::new(
        "GetPhysicalDeviceQueueFamilyProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pQueueFamilyPropertyCount", "*mut u32"),
            ("pQueueFamilyProperties", "*mut VkQueueFamilyProperties2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .into_element(),
    Command::new(
        "GetPhysicalDeviceMemoryProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pMemoryProperties", "*mut VkPhysicalDeviceMemoryProperties2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .into_element(),
    Command::new(
        "GetPhysicalDeviceSparseImageFormatProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pFormatInfo", "*const VkPhysicalDeviceSparseImageFormatInfo2KHR"),
            ("pPropertyCount", "*mut u32"),
            ("pProperties", "*mut VkSparseImageFormatProperties2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .into_element(),
    // VK_KHR_device_group
    VK_KHR_DEVICE_GROUP.header_constants().into_element(),
    Bitmask::extending(
        "DependencyFlagBits",
        "DEPENDENCY",
        &[Bitmask::entry("DEVICE_GROUP", 2).extension(VK_KHR_DEVICE_GROUP)],
    )
    .into_element(),
    Bitmask::extending(
        "PipelineCreateFlagBits",
        "PIPELINE_CREATE",
        &[
            Bitmask::entry("VIEW_INDEX_FROM_DEVICE_INDEX", 3).extension(VK_KHR_DEVICE_GROUP),
            Bitmask::entry("DISPATCH_BASE", 4).extension(VK_KHR_DEVICE_GROUP),
        ],
    )
    .into_element(),
    Bitmask::extending(
        "SwapchainCreateFlagBits",
        "SWAPCHAIN_CREATE",
        &[Bitmask::entry("SPLIT_INSTNACE_BIND_REGIONS", 0).extension(VK_KHR_DEVICE_GROUP)],
    )
    .extension(VK_KHR_SWAPCHAIN)
    .into_element(),
    Bitmask::new(
        "MemoryAllocateFlags",
        "MemoryAllocateFlagBits",
        "MEMORY_ALLOCATE",
        &[Bitmask::entry("DEVICE_MASK", 0).extension(VK_KHR_DEVICE_GROUP)],
    )
    .extension(VK_KHR_DEVICE_GROUP)
    .into_element(),
    Bitmask::new(
        "PeerMemoryFeatureFlags",
        "PeerMemoryFeatureFlagBits",
        "PEER_MEMORY_FEATURE",
        &[
            Bitmask::entry("COPY_SRC", 0).extension(VK_KHR_DEVICE_GROUP),
            Bitmask::entry("COPY_DST", 1).extension(VK_KHR_DEVICE_GROUP),
            Bitmask::entry("GENERIC_SRC", 2).extension(VK_KHR_DEVICE_GROUP),
            Bitmask::entry("GENERIC_DST", 3).extension(VK_KHR_DEVICE_GROUP),
        ],
    )
    .extension(VK_KHR_DEVICE_GROUP)
    .into_element(),
    Struct::typed(
        "MemoryAllocateFlagsInfo",
        "MEMORY_ALLOCATE_FLAGS_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(0) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkMemoryAllocateFlags"),
            Struct::member("deviceMask", "u32"),
        ],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DeviceGroupRenderPassBeginInfo",
        "DEVICE_GROUP_RENDER_PASS_BEGIN_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(3) as _,
        StructUsage::Source,
        &[
            Struct::member("deviceMask", "u32"),
            Struct::member("deviceRenderAreaCount", "u32"),
            Struct::member("pDeviceRenderAreas", "*const VkRect2D"),
        ],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DeviceGroupCommandBufferBeginInfo",
        "DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(4) as _,
        StructUsage::Source,
        &[Struct::member("deviceMask", "u32")],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DeviceGroupSubmitInfo",
        "DEVICE_GROUP_SUBMIT_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(5) as _,
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
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DeviceGroupBindSparseInfo",
        "DEVICE_GROUP_BIND_SPARSE_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(6) as _,
        StructUsage::Source,
        &[
            Struct::member("resourceDeviceIndex", "u32"),
            Struct::member("memoryDeviceIndex", "u32"),
        ],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DeviceGroupPresentCapabilities",
        "DEVICE_GROUP_PRESENT_CAPABILITIES",
        VK_KHR_DEVICE_GROUP.ext_enum(7) as _,
        StructUsage::Sink,
        &[
            Struct::member("presentMask", "[u32; VK_MAX_DEVICE_GROUP_SIZE_KHR]"),
            Struct::member("modes", "VkDeviceGroupPresentModeFlagsKHR"),
        ],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .side_extensions(&[VK_KHR_SURFACE])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImageSwapchainCreateInfo",
        "IMAGE_SWAPCHAIN_CREATE_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(8) as _,
        StructUsage::Source,
        &[Struct::member("swapchain", "VkSwapchainKHR")],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .side_extensions(&[VK_KHR_SWAPCHAIN])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BindImageMemorySwapchainInfo",
        "BIND_IMAGE_MEMORY_SWAPCHAIN_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(9) as _,
        StructUsage::Source,
        &[
            Struct::member("swapchain", "VkSwapchainKHR"),
            Struct::member("imageIndex", "u32"),
        ],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .side_extensions(&[VK_KHR_SWAPCHAIN])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "AcquireNextImageInfo",
        "ACQUIRE_NEXT_IMAGE_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(10) as _,
        StructUsage::Source,
        &[
            Struct::member("swapchain", "VkSwapchainKHR"),
            Struct::member("timeout", "u64"),
            Struct::member("semaphore", "VkSemaphore"),
            Struct::member("fence", "VkFence"),
            Struct::member("deviceMask", "u32"),
        ],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .side_extensions(&[VK_KHR_SWAPCHAIN])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DeviceGroupPresentInfo",
        "DEVICE_GROUP_PRESENT_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(11) as _,
        StructUsage::Source,
        &[
            Struct::member("swapchainCount", "u32"),
            Struct::member("pDeviceMasks", "*const u32"),
            Struct::member("mode", "VkDeviceGroupPresentModeFlagBitsKHR"),
        ],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .side_extensions(&[VK_KHR_SWAPCHAIN])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DeviceGroupSwapchainCreateInfo",
        "DEVICE_GROUP_SWAPCHAIN_CREATE_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(12) as _,
        StructUsage::Source,
        &[Struct::member("modes", "VkDeviceGroupPresentModeFlagsKHR")],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .side_extensions(&[VK_KHR_SWAPCHAIN])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BindBufferMemoryDeviceGroupInfo",
        "BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(13) as _,
        StructUsage::Source,
        &[
            Struct::member("deviceIndexCount", "u32"),
            Struct::member("pDeviceIndices", "*const u32"),
        ],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .side_extensions(&[VK_KHR_BIND_MEMORY_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BindImageMemoryDeviceGroupInfo",
        "BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO",
        VK_KHR_DEVICE_GROUP.ext_enum(14) as _,
        StructUsage::Source,
        &[
            Struct::member("deviceIndexCount", "u32"),
            Struct::member("pDeviceIndices", "*const u32"),
            Struct::member("splitInstanceBindRegionCount", "u32"),
            Struct::member("pSplitInstanceBindRegions", "*const VkRect2D"),
        ],
    )
    .extensions(&[VK_KHR_DEVICE_GROUP])
    .side_extensions(&[VK_KHR_BIND_MEMORY_2])
    .promoted(VERSION)
    .into_element(),
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
    .extension(VK_KHR_DEVICE_GROUP)
    .into_element(),
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
    .extension(VK_KHR_DEVICE_GROUP)
    .side_extensions(&[VK_KHR_SURFACE])
    .into_element(),
    Command::new(
        "GetDeviceGroupSurfacePresentModes",
        &[
            ("device", "VkDevice"),
            ("surface", "VkSurfaceKHR"),
            ("pModes", "*mut VkDeviceGroupPresentModeFlagsKHR"),
        ],
    )
    .failable()
    .extension(VK_KHR_DEVICE_GROUP)
    .side_extensions(&[VK_KHR_SURFACE])
    .into_element(),
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
    .extension(VK_KHR_DEVICE_GROUP)
    .side_extensions(&[VK_KHR_SURFACE])
    .into_element(),
    Command::new(
        "AcquireNextImage2",
        &[
            ("device", "VkDevice"),
            ("pAcquireInfo", "*const VkAcquireNextImageInfoKHR"),
            ("pImageIndex", "*mut u32"),
        ],
    )
    .failable()
    .extension(VK_KHR_DEVICE_GROUP)
    .side_extensions(&[VK_KHR_SWAPCHAIN])
    .into_element(),
    Command::inst("SetDeviceMask", &[("deviceMask", "u32")])
        .extension(VK_KHR_DEVICE_GROUP)
        .into_element(),
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
    .extension(VK_KHR_DEVICE_GROUP)
    .into_element(),
    // VK_EXT_shader_subgroup_vote
    VK_EXT_SHADER_SUBGROUP_VOTE.header_constants().into_element(),
    // VK_KHR_descriptor_update_template
    VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE.header_constants().into_element(),
    Bitmask::new(
        "DescriptorUpdateTemplateCreateFlags",
        "DescriptorUpdateTemplateCreateFlagBits",
        "DESCRIPTOR_UPDATE_TEMPLATE_CREATE",
        &[],
    )
    .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)
    .into_element(),
    Enum::extending(
        "DebugReportObjectType",
        "DEBUG_REPORT_OBJECT_TYPE",
        &[Enum::member(
            "DESCRIPTOR_UPDATE_TEMPLATE",
            VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE.ext_enum(0) as _,
        )
        .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)],
    )
    .extension(VK_EXT_DEBUG_REPORT)
    .into_element(),
    Enum::new(
        "DescriptorUpdateTemplateType",
        "DESCRIPTOR_UPDATE_TEMPLATE_TYPE",
        &[Enum::member("DESCRIPTOR_SET", 0).extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)],
    )
    .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)
    .into_element(),
    Struct::typed(
        "DescriptorUpdateTemplateCreateInfo",
        "DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO",
        VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE.ext_enum(0) as _,
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
    .extensions(&[VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE])
    .promoted(VERSION)
    .into_element(),
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
    .extensions(&[VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE])
    .promoted(VERSION)
    .into_element(),
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
    .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)
    .into_element(),
    Command::new(
        "DestroyDescriptorUpdateTemplate",
        &[
            ("device", "VkDevice"),
            ("descriptorUpdateTemplate", "VkDescriptorUpdateTemplateKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)
    .into_element(),
    Command::new(
        "UpdateDescriptorSetWithTemplate",
        &[
            ("device", "VkDevice"),
            ("descriptorSet", "VkDescriptorSet"),
            ("descriptorUpdateTemplate", "VkDescriptorUpdateTemplateKHR"),
            ("pData", "*const core::ffi::c_void"),
        ],
    )
    .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)
    .into_element(),
    // VK_KHR_dedicated_allocation
    VK_KHR_DEDICATED_ALLOCATION.header_constants().into_element(),
    Struct::typed(
        "MemoryDedicatedRequirements",
        "MEMORY_DEDICATED_REQUIREMENTS",
        VK_KHR_DEDICATED_ALLOCATION.ext_enum(0) as _,
        StructUsage::Sink,
        &[
            Struct::member("prefersDedicatedAllocation", "VkBool32"),
            Struct::member("requiresDedicatedAllocation", "VkBool32"),
        ],
    )
    .extensions(&[VK_KHR_DEDICATED_ALLOCATION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "MemoryDedicatedAllocateInfo",
        "MEMORY_DEDICATED_ALLOCATE_INFO",
        VK_KHR_DEDICATED_ALLOCATION.ext_enum(1) as _,
        StructUsage::Source,
        &[
            Struct::member("image", "Option<VkImage>"),
            Struct::member("buffer", "Option<VkBuffer>"),
        ],
    )
    .extensions(&[VK_KHR_DEDICATED_ALLOCATION])
    .promoted(VERSION)
    .into_element(),
    // VK_KHR_get_memory_requirements2
    VK_KHR_GET_MEMORY_REQUIREMENTS_2.header_constants().into_element(),
    Struct::typed(
        "BufferMemoryRequirementsInfo2",
        "BUFFER_MEMORY_REQUIREMENTS_INFO_2",
        VK_KHR_GET_MEMORY_REQUIREMENTS_2.ext_enum(0) as _,
        StructUsage::Source,
        &[Struct::member("buffer", "VkBuffer")],
    )
    .extensions(&[VK_KHR_GET_MEMORY_REQUIREMENTS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImageMemoryRequirementsInfo2",
        "IMAGE_MEMORY_REQUIREMENTS_INFO_2",
        VK_KHR_GET_MEMORY_REQUIREMENTS_2.ext_enum(1) as _,
        StructUsage::Source,
        &[Struct::member("image", "VkImage")],
    )
    .extensions(&[VK_KHR_GET_MEMORY_REQUIREMENTS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImageSparseMemoryRequirementsInfo2",
        "IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2",
        VK_KHR_GET_MEMORY_REQUIREMENTS_2.ext_enum(2) as _,
        StructUsage::Source,
        &[Struct::member("image", "VkImage")],
    )
    .extensions(&[VK_KHR_GET_MEMORY_REQUIREMENTS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "MemoryRequirements2",
        "MEMORY_REQUIREMENTS_2",
        VK_KHR_GET_MEMORY_REQUIREMENTS_2.ext_enum(3) as _,
        StructUsage::Sink,
        &[Struct::member("memoryRequirements", "VkMemoryRequirements")],
    )
    .extensions(&[VK_KHR_GET_MEMORY_REQUIREMENTS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SparseImageMemoryRequirements2",
        "SPARSE_IMAGE_MEMORY_REQUIREMENTS_2",
        VK_KHR_GET_MEMORY_REQUIREMENTS_2.ext_enum(4) as _,
        StructUsage::Sink,
        &[Struct::member("memoryRequirements", "VkSparseImageMemoryRequirements")],
    )
    .extensions(&[VK_KHR_GET_MEMORY_REQUIREMENTS_2])
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "GetImageMemoryRequirements2",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkImageMemoryRequirementsInfo2KHR"),
            ("pMemoryRequirements", "*mut VkMemoryRequirements2KHR"),
        ],
    )
    .extension(VK_KHR_GET_MEMORY_REQUIREMENTS_2)
    .into_element(),
    Command::new(
        "GetBufferMemoryRequirements2",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkBufferMemoryRequirementsInfo2KHR"),
            ("pMemoryRequirements", "*mut VkMemoryRequirements2KHR"),
        ],
    )
    .extension(VK_KHR_GET_MEMORY_REQUIREMENTS_2)
    .into_element(),
    Command::new(
        "GetImageSparseMemoryRequirements2",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkImageSparseMemoryRequirementsInfo2KHR"),
            ("pSparseMemoryRequirementCount", "*mut u32"),
            ("pSparseMemoryRequirements", "*mut VkSparseImageMemoryRequirements2KHR"),
        ],
    )
    .extension(VK_KHR_GET_MEMORY_REQUIREMENTS_2)
    .into_element(),
    // VK_KHR_sampler_ycbcr_conversion
    VK_KHR_SAMPLER_YCBCR_CONVERSION.header_constants().into_element(),
    Object::new(
        "VkSamplerYcbcrConversionKHR",
        "SAMPLER_YCBCR_CONVERSION_KHR",
        vk_ext_enum(157, 0),
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .promoted(VERSION, "VkSamplerYcbcrConversion", "SAMPLER_YCBCR_CONVERSION")
    .into_element(),
    Bitmask::extending(
        "FormatFeatureFlagBits",
        "FORMAT_FEATURE",
        &[
            Bitmask::entry("MIDPOINT_CHROMA_SAMPLES", 17).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER", 18)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER", 19)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT", 20)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Bitmask::entry(
                "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE",
                21,
            )
            .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Bitmask::entry("DISJOINT", 22).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Bitmask::entry("COSITED_CHROMA_SAMPLES", 23).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
        ],
    )
    .into_element(),
    Bitmask::extending(
        "ImageAspectFlagBits",
        "IMAGE_ASPECT",
        &[
            Bitmask::entry("PLANE_0", 4).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Bitmask::entry("PLANE_1", 5).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Bitmask::entry("PLANE_2", 6).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
        ],
    )
    .into_element(),
    Bitmask::extending(
        "ImageCreateFlagBits",
        "IMAGE_CREATE",
        &[Bitmask::entry("DISJOINT", 9).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)],
    )
    .into_element(),
    Enum::new(
        "SamplerYcbcrModelConversion",
        "SAMPLER_YCBCR_MODEL_CONVERSION",
        &[
            Enum::member("RGB_IDENTITY", 0).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Enum::member("YCBCR_IDENTITY", 1).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Enum::member("YCBCR_709", 2).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Enum::member("YCBCR_601", 3).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Enum::member("YCBCR_2020", 4).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
        ],
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .into_element(),
    Enum::new(
        "ChromaLocation",
        "CHROMA_LOCATION",
        &[
            Enum::member("COSITED_EVEN", 0).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Enum::member("MIDPOINT", 1).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
        ],
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .into_element(),
    Enum::new(
        "SamplerYcbcrRange",
        "SAMPLER_YCBCR_RANGE",
        &[
            Enum::member("ITU_FULL", 0).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
            Enum::member("ITU_NARROW", 1).extension(VK_KHR_SAMPLER_YCBCR_CONVERSION),
        ],
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .into_element(),
    Enum::extending(
        "DebugReportObjectType",
        "DEBUG_REPORT_OBJECT_TYPE",
        &[Enum::member("SAMPLER_YCBCR_CONVERSION", vk_ext_enum(157, 0) as _)
            .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)],
    )
    .extension(VK_EXT_DEBUG_REPORT)
    .into_element(),
    Struct::typed(
        "SamplerYcbcrConversionCreateInfo",
        "SAMPLER_YCBCR_CONVERSION_CREATE_INFO",
        VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(0) as _,
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
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SamplerYcbcrConversionInfo",
        "SAMPLER_YCBCR_CONVERSION_INFO",
        VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(1) as _,
        StructUsage::Source,
        &[Struct::member("conversion", "VkSamplerYcbcrConversion")],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BindImagePlaneMemoryInfo",
        "BIND_IMAGE_PLANE_MEMORY_INFO",
        VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(2) as _,
        StructUsage::Source,
        &[Struct::member("planeAspect", "VkImageAspectFlags")],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImagePlaneMemoryRequirementsInfo",
        "IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO",
        VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(3) as _,
        StructUsage::Source,
        &[Struct::member("planeAspect", "VkImageAspectFlagBits")],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceSamplerYcbcrConversionFeatures",
        "PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES",
        VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(4) as _,
        StructUsage::Both,
        &[Struct::member("samplerYcbcrConversion", TY_VK_BOOL)],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SamplerYcbcrConversionImageFormatProperties",
        "SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES",
        VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(5) as _,
        StructUsage::Sink,
        &[Struct::member("combinedImageSamplerDescriptorCount", "u32")],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
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
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .into_element(),
    Command::new(
        "DestroySamplerYcbcrConversion",
        &[
            ("device", "VkDevice"),
            ("ycbcrConversion", "VkSamplerYcbcrConversionKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .into_element(),
    // VK_KHR_bind_memory2
    VK_KHR_BIND_MEMORY_2.header_constants().into_element(),
    Bitmask::extending(
        "ImageCreateFlagBits",
        "IMAGE_CREATE",
        &[Bitmask::entry("ALIAS", 10).extension(VK_KHR_BIND_MEMORY_2)],
    )
    .into_element(),
    Struct::typed(
        "BindBufferMemoryInfo",
        "BIND_BUFFER_MEMORY_INFO",
        VK_KHR_BIND_MEMORY_2.ext_enum(0) as _,
        StructUsage::Source,
        &[
            Struct::member("buffer", "VkBuffer"),
            Struct::member("memory", "VkDeviceMemory"),
            Struct::member("memoryOffset", "VkDeviceSize"),
        ],
    )
    .extensions(&[VK_KHR_BIND_MEMORY_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BindImageMemoryInfo",
        "BIND_IMAGE_MEMORY_INFO",
        VK_KHR_BIND_MEMORY_2.ext_enum(1) as _,
        StructUsage::Source,
        &[
            Struct::member("image", "VkImage"),
            Struct::member("memory", "VkDeviceMemory"),
            Struct::member("memoryOffset", "VkDeviceSize"),
        ],
    )
    .extensions(&[VK_KHR_BIND_MEMORY_2])
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "BindBufferMemory2",
        &[
            ("device", "VkDevice"),
            ("bindInfoCount", "u32"),
            ("pBindInfos", "*const VkBindBufferMemoryInfoKHR"),
        ],
    )
    .failable()
    .extension(VK_KHR_BIND_MEMORY_2)
    .into_element(),
    Command::new(
        "BindImageMemory2",
        &[
            ("device", "VkDevice"),
            ("bindInfoCount", "u32"),
            ("pBindInfos", "*const VkBindImageMemoryInfoKHR"),
        ],
    )
    .failable()
    .extension(VK_KHR_BIND_MEMORY_2)
    .into_element(),
];
