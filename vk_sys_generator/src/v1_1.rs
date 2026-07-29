//! v1.1 promoted elements

use crate::{
    DEPENDENCY_FLAGS, ERROR, FORMAT, FORMAT_FEATURE_FLAGS, IMAGE_ASPECT_FLAGS, IMAGE_CREATE_FLAGS, IMAGE_LAYOUT,
    MEMORY_HEAP_FLAGS, PIPELINE_CREATE_FLAGS,
    extensions::{DEBUG_REPORT_OBJECT_TYPE, SWAPCHAIN_CREATE_FLAGS, VK_KHR_SURFACE, VK_KHR_SWAPCHAIN},
    parts::*,
    rs_item::RustCodeEmitter,
};

pub fn emit(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    MEMORY_HEAP_FLAGS
        .entry("MULTI_INSTANCE", 1)
        .version_since(VERSION)
        .emit(emitter);

    emit_maintenance1(emitter);
    emit_maintenance2(emitter);

    emit_multiview(emitter);
    emit_get_physical_device_properties2(emitter);
    emit_device_group(emitter);
    emit_shader_subgroup_vote(emitter);
    emit_external_memory_capabilities(emitter);
    emit_external_memory(emitter);
    emit_external_semaphore_capabilities(emitter);
    emit_external_semaphore(emitter);
    emit_descriptor_update_template(emitter);
    emit_external_fence_capabilities(emitter);
    emit_external_fence(emitter);
    emit_dedicated_allocation(emitter);
    emit_get_memory_requirements2(emitter);
    emit_sampler_ycbcr_conversion(emitter);
    emit_bind_memory2(emitter);
}

const VERSION: &str = "1_1";

pub const VK_KHR_MAINTENANCE_1: &Extension = &Extension::khr("maintenance1", 2, 70).promoted(VERSION);
const COMMAND_POOL_TRIM_FLAGS: &BitmaskType =
    &BitmaskType::new("CommandPoolTrimFlags", "CommandPoolTrimFlagBits", "COMMAND_POOL_TRIM")
        .extension(VK_KHR_MAINTENANCE_1);
fn emit_maintenance1(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_MAINTENANCE_1.header_constants().emit(emitter);

    ERROR
        .member("OUT_OF_POOL_MEMORY", 0)
        .neg()
        .extension(VK_KHR_MAINTENANCE_1)
        .emit(emitter);
    FORMAT_FEATURE_FLAGS
        .entry("TRANSFER_SRC", 14)
        .extension(VK_KHR_MAINTENANCE_1)
        .emit(emitter);
    FORMAT_FEATURE_FLAGS
        .entry("TRANSFER_DST", 15)
        .extension(VK_KHR_MAINTENANCE_1)
        .emit(emitter);
    IMAGE_CREATE_FLAGS
        .entry("2D_ARRAY_COMPATIBLE", 5)
        .extension(VK_KHR_MAINTENANCE_1)
        .emit(emitter);

    COMMAND_POOL_TRIM_FLAGS.emit(emitter);

    Command::new(
        "TrimCommandPool",
        &[
            ("device", "VkDevice"),
            ("commandPool", "VkCommandPool"),
            ("flags", "VkCommandPoolTrimFlagsKHR"),
        ],
    )
    .extension(VK_KHR_MAINTENANCE_1)
    .emit(emitter);
}

pub const VK_KHR_MAINTENANCE_2: &Extension = &Extension::khr("maintenance2", 1, 118).promoted(VERSION);
pub const POINT_CLIPPING_BEHAVIOR: &EnumType =
    &EnumType::new("PointClippingBehavior", "POINT_CLIPPING_BEHAVIOR").extension(VK_KHR_MAINTENANCE_2);
pub const TESSELLATION_DOMAIN_ORIGIN: &EnumType =
    &EnumType::new("TessellationDomainOrigin", "TESSELLATION_DOMAIN_ORIGIN").extension(VK_KHR_MAINTENANCE_2);
const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: &Struct = &Struct::typed(
    "PhysicalDevicePointClippingProperties",
    "PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES",
    VK_KHR_MAINTENANCE_2.ext_enum(0) as _,
    StructUsage::Sink,
    &[Struct::member("pointClippingBehavior", "VkPointClippingBehaviorKHR")],
)
.extensions(&[VK_KHR_MAINTENANCE_2])
.promoted(VERSION);
const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: &Struct = &Struct::typed(
    "RenderPassInputAttachmentAspectCreateInfo",
    "RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO",
    VK_KHR_MAINTENANCE_2.ext_enum(1) as _,
    StructUsage::Source,
    &[
        Struct::member("aspectReferenceCount", "u32"),
        Struct::member("pAspectReferences", "*const VkInputAttachmentAspectReferenceKHR"),
    ],
)
.extensions(&[VK_KHR_MAINTENANCE_2])
.promoted(VERSION);
const IMAGE_VIEW_USAGE_CREATE_INFO: &Struct = &Struct::typed(
    "ImageViewUsageCreateInfo",
    "IMAGE_VIEW_USAGE_CREATE_INFO",
    VK_KHR_MAINTENANCE_2.ext_enum(2) as _,
    StructUsage::Source,
    &[
        Struct::member("sliceOffset", "u32"),
        Struct::member("sliceCount", "u32"),
    ],
)
.extensions(&[VK_KHR_MAINTENANCE_2])
.promoted(VERSION);
const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: &Struct = &Struct::typed(
    "PipelineTessellationDomainOriginStateCreateInfo",
    "PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO",
    VK_KHR_MAINTENANCE_2.ext_enum(3) as _,
    StructUsage::Source,
    &[Struct::member("domainOrigin", "VkTessellationDomainOriginKHR")],
)
.extensions(&[VK_KHR_MAINTENANCE_2])
.promoted(VERSION);
const INPUT_ATTACHMENT_ASPECT_REFERENCE: &Struct = &Struct::new(
    "InputAttachmentAspectReference",
    &[
        Struct::member("subpass", "u32"),
        Struct::member("inputAttachmentIndex", "u32"),
        Struct::member("aspectMask", "VkImageAspectFlags"),
    ],
)
.extensions(&[VK_KHR_MAINTENANCE_2])
.promoted(VERSION);
fn emit_maintenance2(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_MAINTENANCE_2.header_constants().emit(emitter);

    IMAGE_LAYOUT
        .member("DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL", 0)
        .extension(VK_KHR_MAINTENANCE_2)
        .emit(emitter);
    IMAGE_LAYOUT
        .member("DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL", 1)
        .extension(VK_KHR_MAINTENANCE_2)
        .emit(emitter);

    IMAGE_CREATE_FLAGS
        .entry("BLOCK_TEXEL_VIEW_COMPATIBLE", 7)
        .extension(VK_KHR_MAINTENANCE_2)
        .emit(emitter);
    IMAGE_CREATE_FLAGS
        .entry("EXTENDED_USAGE", 8)
        .extension(VK_KHR_MAINTENANCE_2)
        .emit(emitter);

    POINT_CLIPPING_BEHAVIOR.emit(emitter);
    POINT_CLIPPING_BEHAVIOR
        .member("ALL_CLIP_PLANES", 0)
        .override_extension_number(0)
        .emit(emitter);
    POINT_CLIPPING_BEHAVIOR
        .member("USER_CLIP_PLANES", 1)
        .override_extension_number(0)
        .emit(emitter);

    TESSELLATION_DOMAIN_ORIGIN.emit(emitter);
    TESSELLATION_DOMAIN_ORIGIN
        .member("UPPER_LEFT", 0)
        .override_extension_number(0)
        .emit(emitter);
    TESSELLATION_DOMAIN_ORIGIN
        .member("LOWER_LEFT", 1)
        .override_extension_number(0)
        .emit(emitter);

    PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES.emit(emitter);
    RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO.emit(emitter);
    IMAGE_VIEW_USAGE_CREATE_INFO.emit(emitter);
    PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO.emit(emitter);
    INPUT_ATTACHMENT_ASPECT_REFERENCE.emit(emitter);
}

pub const VK_KHR_MULTIVIEW: &Extension = &Extension::khr("multiview", 1, 54).promoted(VERSION);
pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: &Struct = &Struct::typed(
    "RenderPassMultiviewCreateInfo",
    "RENDER_PASS_MULTIVIEW_CREATE_INFO",
    VK_KHR_MULTIVIEW.ext_enum(0) as _,
    StructUsage::Source,
    &[
        Struct::member("subpassCount", "u32"),
        Struct::member("pViewMasks", "*const u32"),
        Struct::member("dependencyCount", "u32"),
        Struct::member("pViewOffsets", "*const i32"),
        Struct::member("correlationMaskCount", "u32"),
        Struct::member("pCorrelationMasks", "*const u32"),
    ],
)
.extensions(&[VK_KHR_MULTIVIEW])
.promoted(VERSION);
pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: &Struct = &Struct::typed(
    "PhysicalDeviceMultiviewFeatures",
    "PHYSICAL_DEVICE_MULTIVIEW_FEATURES",
    VK_KHR_MULTIVIEW.ext_enum(1) as _,
    StructUsage::Both,
    &[
        Struct::member("multiview", "VkBool32"),
        Struct::member("multiviewGeometryShader", "VkBool32"),
        Struct::member("multiviewTessellationShader", "VkBool32"),
    ],
)
.extensions(&[VK_KHR_MULTIVIEW])
.promoted(VERSION);
pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: &Struct = &Struct::typed(
    "PhysicalDeviceMultiviewProperties",
    "PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES",
    VK_KHR_MULTIVIEW.ext_enum(2) as _,
    StructUsage::Sink,
    &[
        Struct::member("maxMultiviewViewCount", "u32"),
        Struct::member("maxMultiviewInstanceIndex", "u32"),
    ],
)
.extensions(&[VK_KHR_MULTIVIEW])
.promoted(VERSION);
fn emit_multiview(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_MULTIVIEW.header_constants().emit(emitter);

    DEPENDENCY_FLAGS
        .entry("VIEW_LOCAL", 1)
        .extension(VK_KHR_MULTIVIEW)
        .emit(emitter);

    RENDER_PASS_MULTIVIEW_CREATE_INFO.emit(emitter);
    PHYSICAL_DEVICE_MULTIVIEW_FEATURES.emit(emitter);
    PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES.emit(emitter);
}

pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2: &Extension =
    &Extension::khr("get_physical_device_properties2", 2, 60).promoted(VERSION);
pub const PHYSICAL_DEVICE_FEATURES_2: &Struct = &Struct::typed(
    "PhysicalDeviceFeatures2",
    "PHYSICAL_DEVICE_FEATURES_2",
    VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(0) as _,
    StructUsage::Both,
    &[Struct::member("features", "VkPhysicalDeviceFeatures")],
)
.extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
.promoted(VERSION);
pub const PHYSICAL_DEVICE_PROPERTIES_2: &Struct = &Struct::typed(
    "PhysicalDeviceProperties2",
    "PHYSICAL_DEVICE_PROPERTIES_2",
    VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(1) as _,
    StructUsage::Sink,
    &[Struct::member("properties", "VkPhysicalDeviceProperties")],
)
.extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
.promoted(VERSION);
pub const FORMAT_PROPERTIES_2: &Struct = &Struct::typed(
    "FormatProperties2",
    "FORMAT_PROPERTIES_2",
    VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(2) as _,
    StructUsage::Sink,
    &[Struct::member("formatProperties", "VkFormatProperties")],
)
.extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
.promoted(VERSION);
pub const IMAGE_FORMAT_PROPERTIES_2: &Struct = &Struct::typed(
    "ImageFormatProperties2",
    "IMAGE_FORMAT_PROPERTIES_2",
    VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(3) as _,
    StructUsage::Sink,
    &[Struct::member("imageFormatProperties", "VkImageFormatProperties")],
)
.extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
.promoted(VERSION);
pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const QUEUE_FAMILY_PROPERTIES_2: &Struct = &Struct::typed(
    "QueueFamilyProperties2",
    "QUEUE_FAMILY_PROPERTIES_2",
    VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(5) as _,
    StructUsage::Sink,
    &[Struct::member("queueFamilyProperties", "VkQueueFamilyProperties")],
)
.extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
.promoted(VERSION);
pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: &Struct = &Struct::typed(
    "PhysicalDeviceMemoryProperties2",
    "PHYSICAL_DEVICE_MEMORY_PROPERTIES_2",
    VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(6) as _,
    StructUsage::Sink,
    &[Struct::member("memoryProperties", "VkPhysicalDeviceMemoryProperties")],
)
.extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
.promoted(VERSION);
pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: &Struct = &Struct::typed(
    "SparseImageFormatProperties2",
    "SPARSE_IMAGE_FORMAT_PROPERTIES_2",
    VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.ext_enum(7) as _,
    StructUsage::Sink,
    &[Struct::member("properties", "VkSparseImageFormatProperties")],
)
.extensions(&[VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2])
.promoted(VERSION);
pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: &Struct = &Struct::typed(
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
.promoted(VERSION);
fn emit_get_physical_device_properties2(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2.header_constants().emit(emitter);

    PHYSICAL_DEVICE_FEATURES_2.emit(emitter);
    PHYSICAL_DEVICE_PROPERTIES_2.emit(emitter);
    FORMAT_PROPERTIES_2.emit(emitter);
    IMAGE_FORMAT_PROPERTIES_2.emit(emitter);
    PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2.emit(emitter);
    QUEUE_FAMILY_PROPERTIES_2.emit(emitter);
    PHYSICAL_DEVICE_MEMORY_PROPERTIES_2.emit(emitter);
    SPARSE_IMAGE_FORMAT_PROPERTIES_2.emit(emitter);
    PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2.emit(emitter);

    Command::new(
        "GetPhysicalDeviceFeatures2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pFeatures", "*mut VkPhysicalDeviceFeatures2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .emit(emitter);
    Command::new(
        "GetPhysicalDeviceProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pProperties", "*mut VkPhysicalDeviceProperties2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .emit(emitter);
    Command::new(
        "GetPhysicalDeviceFormatProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("format", "VkFormat"),
            ("pFormatProperties", "*mut VkFormatProperties2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .emit(emitter);
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
    .emit(emitter);
    Command::new(
        "GetPhysicalDeviceQueueFamilyProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pQueueFamilyPropertyCount", "*mut u32"),
            ("pQueueFamilyProperties", "*mut VkQueueFamilyProperties2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .emit(emitter);
    Command::new(
        "GetPhysicalDeviceMemoryProperties2",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pMemoryProperties", "*mut VkPhysicalDeviceMemoryProperties2KHR"),
        ],
    )
    .extension(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2)
    .emit(emitter);
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
    .emit(emitter);
}

pub const VK_KHR_DEVICE_GROUP: &Extension = &Extension::khr("device_group", 4, 61).promoted(VERSION);
pub const DEVICE_GROUP_PRESENT_MODE_FLAGS: &BitmaskType = &BitmaskType::new(
    "DeviceGroupPresentModeFlags",
    "DeviceGroupPresentModeFlagBits",
    "DEVICE_GROUP_PRESENT_MODE",
)
.extension(VK_KHR_DEVICE_GROUP)
.side_extensions(&[VK_KHR_SURFACE]);
pub const MEMORY_ALLOCATE_FLAGS: &BitmaskType =
    &BitmaskType::new("MemoryAllocateFlags", "MemoryAllocateFlagBits", "MEMORY_ALLOCATE")
        .extension(VK_KHR_DEVICE_GROUP);
pub const PEER_MEMORY_FEATURE_FLAGS: &BitmaskType = &BitmaskType::new(
    "PeerMemoryFeatureFlags",
    "PeerMemoryFeatureFlagBits",
    "PEER_MEMORY_FEATURE",
)
.extension(VK_KHR_DEVICE_GROUP);
pub const MEMORY_ALLOCATE_FLAGS_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: &Struct = &Struct::typed(
    "DeviceGroupCommandBufferBeginInfo",
    "DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO",
    VK_KHR_DEVICE_GROUP.ext_enum(4) as _,
    StructUsage::Source,
    &[Struct::member("deviceMask", "u32")],
)
.extensions(&[VK_KHR_DEVICE_GROUP])
.promoted(VERSION);
pub const DEVICE_GROUP_SUBMIT_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const DEVICE_GROUP_BIND_SPARSE_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const DEVICE_GROUP_PRESENT_CAPABILITIES: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const IMAGE_SWAPCHAIN_CREATE_INFO: &Struct = &Struct::typed(
    "ImageSwapchainCreateInfo",
    "IMAGE_SWAPCHAIN_CREATE_INFO",
    VK_KHR_DEVICE_GROUP.ext_enum(8) as _,
    StructUsage::Source,
    &[Struct::member("swapchain", "VkSwapchainKHR")],
)
.extensions(&[VK_KHR_DEVICE_GROUP])
.side_extensions(&[VK_KHR_SWAPCHAIN])
.promoted(VERSION);
pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const ACQUIRE_NEXT_IMAGE_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const DEVICE_GROUP_PRESENT_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO: &Struct = &Struct::typed(
    "DeviceGroupSwapchainCreateInfo",
    "DEVICE_GROUP_SWAPCHAIN_CREATE_INFO",
    VK_KHR_DEVICE_GROUP.ext_enum(12) as _,
    StructUsage::Source,
    &[Struct::member("modes", "VkDeviceGroupPresentModeFlagsKHR")],
)
.extensions(&[VK_KHR_DEVICE_GROUP])
.side_extensions(&[VK_KHR_SWAPCHAIN])
.promoted(VERSION);
pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
fn emit_device_group(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_DEVICE_GROUP.header_constants().emit(emitter);

    DEPENDENCY_FLAGS
        .entry("DEVICE_GROUP", 2)
        .extension(VK_KHR_DEVICE_GROUP)
        .emit(emitter);
    PIPELINE_CREATE_FLAGS
        .entry("VIEW_INDEX_FROM_DEVICE_INDEX", 3)
        .extension(VK_KHR_DEVICE_GROUP)
        .emit(emitter);
    PIPELINE_CREATE_FLAGS
        .entry("DISPATCH_BASE", 4)
        .extension(VK_KHR_DEVICE_GROUP)
        .emit(emitter);
    SWAPCHAIN_CREATE_FLAGS
        .entry("SPLIT_INSTANCE_BIND_REGIONS", 0)
        .extension(VK_KHR_DEVICE_GROUP)
        .emit(emitter);
    IMAGE_CREATE_FLAGS
        .entry("SPLIT_INSTANCE_BIND_REGIONS", 6)
        .extension(VK_KHR_DEVICE_GROUP)
        .side_extensions(&[VK_KHR_BIND_MEMORY_2])
        .emit(emitter);

    DEVICE_GROUP_PRESENT_MODE_FLAGS.emit(emitter);

    MEMORY_ALLOCATE_FLAGS.emit(emitter);
    MEMORY_ALLOCATE_FLAGS.entry("DEVICE_MASK", 0).emit(emitter);

    PEER_MEMORY_FEATURE_FLAGS.emit(emitter);
    PEER_MEMORY_FEATURE_FLAGS.entry("COPY_SRC", 0).emit(emitter);
    PEER_MEMORY_FEATURE_FLAGS.entry("COPY_DST", 1).emit(emitter);
    PEER_MEMORY_FEATURE_FLAGS.entry("GENERIC_SRC", 2).emit(emitter);
    PEER_MEMORY_FEATURE_FLAGS.entry("GENERIC_DST", 3).emit(emitter);

    MEMORY_ALLOCATE_FLAGS_INFO.emit(emitter);
    DEVICE_GROUP_RENDER_PASS_BEGIN_INFO.emit(emitter);
    DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO.emit(emitter);
    DEVICE_GROUP_SUBMIT_INFO.emit(emitter);
    DEVICE_GROUP_BIND_SPARSE_INFO.emit(emitter);
    DEVICE_GROUP_PRESENT_CAPABILITIES.emit(emitter);
    IMAGE_SWAPCHAIN_CREATE_INFO.emit(emitter);
    BIND_IMAGE_MEMORY_SWAPCHAIN_INFO.emit(emitter);
    ACQUIRE_NEXT_IMAGE_INFO.emit(emitter);
    DEVICE_GROUP_PRESENT_INFO.emit(emitter);
    DEVICE_GROUP_SWAPCHAIN_CREATE_INFO.emit(emitter);
    BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO.emit(emitter);
    BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO.emit(emitter);

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
    .emit(emitter);
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
    .emit(emitter);
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
    .emit(emitter);
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
    .emit(emitter);
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
    .emit(emitter);
    Command::inst("SetDeviceMask", &[("deviceMask", "u32")])
        .extension(VK_KHR_DEVICE_GROUP)
        .emit(emitter);
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
    .emit(emitter);
}

pub const VK_EXT_SHADER_SUBGROUP_VOTE: &Extension = &Extension::ext("shader_subgroup_vote", 1, 66).promoted(VERSION);
fn emit_shader_subgroup_vote(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_EXT_SHADER_SUBGROUP_VOTE.header_constants().emit(emitter);
}

pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES: &Extension =
    &Extension::khr("external_memory_capabilities", 1, 72).promoted(VERSION);
pub const EXTERNAL_MEMORY_FEATURE_FLAGS: &BitmaskType = &BitmaskType::new(
    "ExternalMemoryFeatureFlags",
    "ExternalMemoryFeatureFlagBits",
    "EXTERNAL_MEMORY_FEATURE",
)
.extension(VK_KHR_EXTERNAL_MEMORY_CAPABILITIES);
pub const EXTERNAL_MEMORY_HANDLE_TYPE_FLAGS: &BitmaskType = &BitmaskType::new(
    "ExternalMemoryHandleTypeFlags",
    "ExternalMemoryHandleTypeFlagBits",
    "EXTERNAL_MEMORY_HANDLE_TYPE",
)
.extension(VK_KHR_EXTERNAL_MEMORY_CAPABILITIES);
pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: &Struct = &Struct::typed(
    "PhysicalDeviceExternalImageFormatInfo",
    "PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO",
    VK_KHR_EXTERNAL_MEMORY_CAPABILITIES.ext_enum(0) as _,
    StructUsage::Source,
    &[Struct::member("handleType", "VkExternalMemoryHandleTypeFlagsKHR")],
)
.extensions(&[VK_KHR_EXTERNAL_MEMORY_CAPABILITIES])
.promoted(VERSION);
pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: &Struct = &Struct::typed(
    "ExternalImageFormatProperties",
    "EXTERANL_IMAGE_FORMAT_PROPERTIES",
    VK_KHR_EXTERNAL_MEMORY_CAPABILITIES.ext_enum(1) as _,
    StructUsage::Sink,
    &[Struct::member(
        "externalMemoryProperties",
        "VkExternalMemoryPropertiesKHR",
    )],
)
.extensions(&[VK_KHR_EXTERNAL_MEMORY_CAPABILITIES])
.promoted(VERSION);
pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: &Struct = &Struct::typed(
    "PhysicalDeviceExternalBufferInfo",
    "PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO",
    VK_KHR_EXTERNAL_MEMORY_CAPABILITIES.ext_enum(2) as _,
    StructUsage::Source,
    &[
        Struct::member("flags", "VkBufferCreateFlags"),
        Struct::member("usage", "VkBufferUsageFlags"),
        Struct::member("handleType", "VkExternalMemoryHandleTypeFlagsKHR"),
    ],
)
.extensions(&[VK_KHR_EXTERNAL_MEMORY_CAPABILITIES])
.promoted(VERSION);
pub const EXTERNAL_BUFFER_PROPERTIES: &Struct = &Struct::typed(
    "ExternalBufferProperties",
    "EXTERNAL_BUFFER_PROPERTIES",
    VK_KHR_EXTERNAL_MEMORY_CAPABILITIES.ext_enum(3) as _,
    StructUsage::Sink,
    &[Struct::member(
        "externalMemoryProperties",
        "VkExternalMemoryPropertiesKHR",
    )],
)
.extensions(&[VK_KHR_EXTERNAL_MEMORY_CAPABILITIES])
.promoted(VERSION);
pub const EXTERNAL_MEMORY_PROPERTIES: &Struct = &Struct::new(
    "ExternalMemoryProperties",
    &[
        Struct::member("externalMemoryFeatures", "VkExternalMemoryFeatureFlagsKHR"),
        Struct::member("exportFromImportedHandleTypes", "VkExternalMemoryHandleTypeFlagsKHR"),
        Struct::member("compatibleHandleTypes", "VkExternalMemoryHandleTypeFlagsKHR"),
    ],
)
.extensions(&[VK_KHR_EXTERNAL_MEMORY_CAPABILITIES])
.promoted(VERSION);
fn emit_external_memory_capabilities(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_EXTERNAL_MEMORY_CAPABILITIES.header_constants().emit(emitter);

    EXTERNAL_MEMORY_FEATURE_FLAGS.emit(emitter);
    EXTERNAL_MEMORY_FEATURE_FLAGS.entry("DEDICATED_ONLY", 0).emit(emitter);
    EXTERNAL_MEMORY_FEATURE_FLAGS.entry("EXPORTABLE", 1).emit(emitter);
    EXTERNAL_MEMORY_FEATURE_FLAGS.entry("IMPORTABLE", 2).emit(emitter);

    EXTERNAL_MEMORY_HANDLE_TYPE_FLAGS.emit(emitter);
    EXTERNAL_MEMORY_HANDLE_TYPE_FLAGS.entry("OPAQUE_FD", 0).emit(emitter);
    EXTERNAL_MEMORY_HANDLE_TYPE_FLAGS.entry("OPAQUE_WIN32", 1).emit(emitter);
    EXTERNAL_MEMORY_HANDLE_TYPE_FLAGS
        .entry("OPAQUE_WIN32_KMT", 2)
        .emit(emitter);
    EXTERNAL_MEMORY_HANDLE_TYPE_FLAGS
        .entry("D3D11_TEXTURE", 3)
        .emit(emitter);
    EXTERNAL_MEMORY_HANDLE_TYPE_FLAGS
        .entry("D3D11_TEXTURE_KMT", 4)
        .emit(emitter);
    EXTERNAL_MEMORY_HANDLE_TYPE_FLAGS.entry("D3D12_HEAP", 5).emit(emitter);
    EXTERNAL_MEMORY_HANDLE_TYPE_FLAGS
        .entry("D3D12_RESOURCE", 6)
        .emit(emitter);

    PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO.emit(emitter);
    EXTERNAL_IMAGE_FORMAT_PROPERTIES.emit(emitter);
    PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO.emit(emitter);
    EXTERNAL_BUFFER_PROPERTIES.emit(emitter);
    EXTERNAL_MEMORY_PROPERTIES.emit(emitter);

    Command::new(
        "GetPhysicalDeviceExternalBufferProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pExternalBufferInfo", "*const VkPhysicalDeviceExternalBufferInfoKHR"),
            ("pExternalBufferProperties", "*mut VkExternalBufferPropertiesKHR"),
        ],
    )
    .extension(VK_KHR_EXTERNAL_MEMORY_CAPABILITIES)
    .emit(emitter);
}

pub const VK_KHR_EXTERNAL_MEMORY: &Extension = &Extension::khr("external_memory", 1, 73).promoted(VERSION);
pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: &Struct = &Struct::typed(
    "ExternalMemoryBufferCreateInfo",
    "EXTERNAL_MEMORY_BUFFER_CREATE_INFO",
    VK_KHR_EXTERNAL_MEMORY.ext_enum(0) as _,
    StructUsage::Source,
    &[Struct::member("handleTypes", "VkExternalMemoryHandleTypeFlagsKHR")],
)
.extensions(&[VK_KHR_EXTERNAL_MEMORY])
.promoted(VERSION);
pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: &Struct = &Struct::typed(
    "ExternalMemoryImageCreateInfo",
    "EXTERNAL_MEMORY_IMAGE_CREATE_INFO",
    VK_KHR_EXTERNAL_MEMORY.ext_enum(1) as _,
    StructUsage::Source,
    &[Struct::member("handleTypes", "VkExternalMemoryHandleTypeFlagsKHR")],
)
.extensions(&[VK_KHR_EXTERNAL_MEMORY])
.promoted(VERSION);
pub const EXPORT_MEMORY_ALLOCATE_INFO: &Struct = &Struct::typed(
    "ExportMemoryAllocateInfo",
    "EXPORT_MEMORY_ALLOCATE_INFO",
    VK_KHR_EXTERNAL_MEMORY.ext_enum(2) as _,
    StructUsage::Source,
    &[Struct::member("handleTypes", "VkExternalMemoryHandleTypeFlagsKHR")],
)
.extensions(&[VK_KHR_EXTERNAL_MEMORY])
.promoted(VERSION);
fn emit_external_memory(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_EXTERNAL_MEMORY.header_constants().emit(emitter);

    ERROR
        .member("INVALID_EXTERNAL_HANDLE", 3)
        .neg()
        .extension(VK_KHR_EXTERNAL_MEMORY)
        .emit(emitter);

    EXTERNAL_MEMORY_BUFFER_CREATE_INFO.emit(emitter);
    EXTERNAL_MEMORY_IMAGE_CREATE_INFO.emit(emitter);
    EXPORT_MEMORY_ALLOCATE_INFO.emit(emitter);
}

pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES: &Extension =
    &Extension::khr("external_semaphore_capabilities", 1, 77).promoted(VERSION);
pub const EXTERNAL_SEMAPHORE_FEATURE_FLAGS: &BitmaskType = &BitmaskType::new(
    "ExternalSemaphoreFeatureFlags",
    "ExternalSemaphoreFeatureFlagBits",
    "EXTERNAL_SEMAPHORE_FEATURE",
)
.extension(VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES);
pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAGS: &BitmaskType = &BitmaskType::new(
    "ExternalSemaphoreHandleTypeFlags",
    "ExternalSemaphoreHandleTypeFlagBits",
    "EXTERNAL_SEMAPHORE_HANDLE_TYPE",
)
.extension(VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES);
pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: &Struct = &Struct::typed(
    "PhysicalDeviceExternalSemaphoreInfo",
    "PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO",
    VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES.ext_enum(0) as _,
    StructUsage::Source,
    &[Struct::member("handleType", "VkExternalSemaphoreHandleTypeFlagsKHR")],
)
.extensions(&[VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES])
.promoted(VERSION);
pub const EXTERNAL_SEMAPHORE_PROPERTIES: &Struct = &Struct::typed(
    "ExternalSemaphoreProperties",
    "EXTERNAL_SEMAPHORE_PROPERTIES",
    VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES.ext_enum(1) as _,
    StructUsage::Sink,
    &[
        Struct::member("exportFromImportedHandleTypes", "VkExternalSemaphoreHandleTypeFlagsKHR"),
        Struct::member("compatibleHandleTypes", "VkExternalSemaphoreHandleTypeFlagsKHR"),
        Struct::member("externalSemaphoreFeatures", "VkExternalSemaphoreFeatureFlagsKHR"),
    ],
)
.extensions(&[VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES])
.promoted(VERSION);
fn emit_external_semaphore_capabilities(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES.header_constants().emit(emitter);

    EXTERNAL_SEMAPHORE_FEATURE_FLAGS.emit(emitter);
    EXTERNAL_SEMAPHORE_FEATURE_FLAGS.entry("EXPORTABLE", 0).emit(emitter);
    EXTERNAL_SEMAPHORE_FEATURE_FLAGS.entry("IMPORTABLE", 1).emit(emitter);

    EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAGS.emit(emitter);
    EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAGS.entry("OPAQUE_FD", 0).emit(emitter);
    EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAGS
        .entry("OPAQUE_WIN32", 1)
        .emit(emitter);
    EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAGS
        .entry("OPAQUE_WIN32_KMT", 2)
        .emit(emitter);
    EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAGS
        .entry("D3D12_FENCE", 3)
        .emit(emitter);
    // promoted special alias
    EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAGS
        .entry("D3D11_FENCE", 3)
        .version_since(VERSION)
        .emit(emitter);
    EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAGS.entry("SYNC_FD", 4).emit(emitter);

    PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO.emit(emitter);
    EXTERNAL_SEMAPHORE_PROPERTIES.emit(emitter);

    Command::new(
        "GetPhysicalDeviceExternalSemaphoreProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            (
                "pExternalSemaphoreInfo",
                "*const VkPhysicalDeviceExternalSemaphoreInfoKHR",
            ),
            ("pExternalSemaphoreProperties", "*mut VkExternalSemaphorePropertiesKHR"),
        ],
    )
    .extension(VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES)
    .emit(emitter);
}

pub const VK_KHR_EXTERNAL_SEMAPHORE: &Extension = &Extension::khr("external_semaphore", 1, 78).promoted(VERSION);
pub const SEMAPHORE_IMPORT_FLAGS: &BitmaskType =
    &BitmaskType::new("SemaphoreImportFlags", "SemaphoreImportFlagBits", "SEMAPHORE_IMPORT")
        .extension(VK_KHR_EXTERNAL_SEMAPHORE);
pub const EXPORT_SEMAPHORE_CREATE_INFO: &Struct = &Struct::typed(
    "ExportSemaphoreCreateInfo",
    "EXPORT_SEMAPHORE_CREATE_INFO",
    VK_KHR_EXTERNAL_SEMAPHORE.ext_enum(0) as _,
    StructUsage::Source,
    &[Struct::member("handleType", "VkExternalSemaphoreHandleTypeFlagsKHR")],
)
.extensions(&[VK_KHR_EXTERNAL_SEMAPHORE])
.promoted(VERSION);
fn emit_external_semaphore(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_EXTERNAL_SEMAPHORE.header_constants().emit(emitter);

    SEMAPHORE_IMPORT_FLAGS.emit(emitter);
    SEMAPHORE_IMPORT_FLAGS.entry("TEMPORARY", 0).emit(emitter);

    EXPORT_SEMAPHORE_CREATE_INFO.emit(emitter);
}

pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE: &Extension =
    &Extension::khr("descriptor_update_template", 1, 86).promoted(VERSION);
pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_FLAGS: &BitmaskType = &BitmaskType::new(
    "DescriptorUpdateTemplateCreateFlags",
    "DescriptorUpdateTemplateCreateFlagBits",
    "DESCRIPTOR_UPDATE_TEMPLATE_CREATE",
)
.extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE);
pub const DESCRIPTOR_UPDATE_TEMPLATE_TYPE: &EnumType =
    &EnumType::new("DescriptorUpdateTemplateType", "DESCRIPTOR_UPDATE_TEMPLATE_TYPE")
        .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE);
pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const DESCRIPTOR_UPDATE_TEMPLATE_ENTRY: &Struct = &Struct::new(
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
.promoted(VERSION);
fn emit_descriptor_update_template(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE.header_constants().emit(emitter);

    DEBUG_REPORT_OBJECT_TYPE
        .member("DESCRIPTOR_UPDATE_TEMPLATE", 0)
        .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)
        .emit(emitter);

    DESCRIPTOR_UPDATE_TEMPLATE_CREATE_FLAGS.emit(emitter);

    DESCRIPTOR_UPDATE_TEMPLATE_TYPE.emit(emitter);
    DESCRIPTOR_UPDATE_TEMPLATE_TYPE
        .member("DESCRIPTOR_SET", 0)
        .override_extension_number(0)
        .emit(emitter);

    DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO.emit(emitter);
    DESCRIPTOR_UPDATE_TEMPLATE_ENTRY.emit(emitter);

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
    .emit(emitter);
    Command::new(
        "DestroyDescriptorUpdateTemplate",
        &[
            ("device", "VkDevice"),
            ("descriptorUpdateTemplate", "VkDescriptorUpdateTemplateKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)
    .emit(emitter);
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
    .emit(emitter);
}

pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES: &Extension =
    &Extension::khr("external_fence_capabilities", 1, 113).promoted(VERSION);
pub const EXTERNAL_FENCE_FEATURE_FLAGS: &BitmaskType = &BitmaskType::new(
    "ExternalFenceFeatureFlags",
    "ExternalFenceFeatureFlagBits",
    "EXTERNAL_FENCE_FEATURE",
)
.extension(VK_KHR_EXTERNAL_FENCE_CAPABILITIES);
pub const EXTERNAL_FENCE_HANDLE_TYPE_FLAGS: &BitmaskType = &BitmaskType::new(
    "ExternalFenceHandleTypeFlags",
    "ExternalFenceHandleTypeFlagBits",
    "EXTERNAL_FENCE_HANDLE_TYPE",
)
.extension(VK_KHR_EXTERNAL_FENCE_CAPABILITIES);
pub const EXTERNAL_FENCE_PROPERTIES: &Struct = &Struct::typed(
    "ExternalFenceProperties",
    "EXTERNAL_FENCE_PROPERTIES",
    VK_KHR_EXTERNAL_FENCE_CAPABILITIES.ext_enum(1) as _,
    StructUsage::Sink,
    &[
        Struct::member("exportFromImportedHandleTypes", "VkExternalFenceHandleTypeFlagsKHR"),
        Struct::member("compatibleHandleTypes", "VkExternalFenceHandleTypeFlagsKHR"),
        Struct::member("externalFenceFeatures", "VkExternalFenceFeatureFlagsKHR"),
    ],
)
.extensions(&[VK_KHR_EXTERNAL_FENCE_CAPABILITIES])
.promoted(VERSION);
pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: &Struct = &Struct::typed(
    "PhysicalDeviceExternalFenceInfo",
    "PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO",
    VK_KHR_EXTERNAL_FENCE_CAPABILITIES.ext_enum(0) as _,
    StructUsage::Source,
    &[Struct::member("handleType", "VkExternalFenceHandleTypeFlagsKHR")],
)
.extensions(&[VK_KHR_EXTERNAL_FENCE_CAPABILITIES])
.promoted(VERSION);
fn emit_external_fence_capabilities(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_EXTERNAL_FENCE_CAPABILITIES.header_constants().emit(emitter);

    EXTERNAL_FENCE_FEATURE_FLAGS.emit(emitter);
    EXTERNAL_FENCE_FEATURE_FLAGS.entry("EXPORTABLE", 0).emit(emitter);
    EXTERNAL_FENCE_FEATURE_FLAGS.entry("IMPORTABLE", 1).emit(emitter);

    EXTERNAL_FENCE_HANDLE_TYPE_FLAGS.emit(emitter);
    EXTERNAL_FENCE_HANDLE_TYPE_FLAGS.entry("OPAQUE_FD", 0).emit(emitter);
    EXTERNAL_FENCE_HANDLE_TYPE_FLAGS.entry("OPAQUE_WIN32", 1).emit(emitter);
    EXTERNAL_FENCE_HANDLE_TYPE_FLAGS
        .entry("OPAQUE_WIN32_KMT", 2)
        .emit(emitter);
    EXTERNAL_FENCE_HANDLE_TYPE_FLAGS.entry("SYNC_FD", 3).emit(emitter);

    EXTERNAL_FENCE_PROPERTIES.emit(emitter);
    PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO.emit(emitter);

    Command::new(
        "GetPhysicalDeviceExternalFenceProperties",
        &[
            ("physicalDevice", "VkPhysicalDevice"),
            ("pExternalFenceInfo", "*const VkPhysicalDeviceExternalFenceInfoKHR"),
            ("pExternalFenceProperties", "*mut VkExternalFencePropertiesKHR"),
        ],
    )
    .extension(VK_KHR_EXTERNAL_FENCE_CAPABILITIES)
    .emit(emitter);
}

pub const VK_KHR_EXTERNAL_FENCE: &Extension = &Extension::khr("external_fence", 1, 114).promoted(VERSION);
pub const FENCE_IMPORT_FLAGS: &BitmaskType =
    &BitmaskType::new("FenceImportFlags", "FenceImportFlagBits", "FENCE_IMPORT").extension(VK_KHR_EXTERNAL_FENCE);
pub const EXPORT_FENCE_CREATE_INFO: &Struct = &Struct::new(
    "ExportFenceCreateInfo",
    &[Struct::member("handleTypes", "VkExternalFenceHandleTypeFlagsKHR")],
)
.extensions(&[VK_KHR_EXTERNAL_FENCE])
.promoted(VERSION);
fn emit_external_fence(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_EXTERNAL_FENCE.header_constants().emit(emitter);

    FENCE_IMPORT_FLAGS.emit(emitter);
    FENCE_IMPORT_FLAGS.entry("TEMPORARY", 0).emit(emitter);

    EXPORT_FENCE_CREATE_INFO.emit(emitter);
}

pub const VK_KHR_DEDICATED_ALLOCATION: &Extension = &Extension::khr("dedicated_allocation", 1, 128).promoted(VERSION);
pub const MEMORY_DEDICATED_REQUIREMENTS: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const MEMORY_DEDICATED_ALLOCATE_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
fn emit_dedicated_allocation(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_DEDICATED_ALLOCATION.header_constants().emit(emitter);

    MEMORY_DEDICATED_REQUIREMENTS.emit(emitter);
    MEMORY_DEDICATED_ALLOCATE_INFO.emit(emitter);
}

pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2: &Extension =
    &Extension::khr("get_memory_requirements2", 1, 147).promoted(VERSION);
pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: &Struct = &Struct::typed(
    "BufferMemoryRequirementsInfo2",
    "BUFFER_MEMORY_REQUIREMENTS_INFO_2",
    VK_KHR_GET_MEMORY_REQUIREMENTS_2.ext_enum(0) as _,
    StructUsage::Source,
    &[Struct::member("buffer", "VkBuffer")],
)
.extensions(&[VK_KHR_GET_MEMORY_REQUIREMENTS_2])
.promoted(VERSION);
pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: &Struct = &Struct::typed(
    "ImageMemoryRequirementsInfo2",
    "IMAGE_MEMORY_REQUIREMENTS_INFO_2",
    VK_KHR_GET_MEMORY_REQUIREMENTS_2.ext_enum(1) as _,
    StructUsage::Source,
    &[Struct::member("image", "VkImage")],
)
.extensions(&[VK_KHR_GET_MEMORY_REQUIREMENTS_2])
.promoted(VERSION);
pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: &Struct = &Struct::typed(
    "ImageSparseMemoryRequirementsInfo2",
    "IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2",
    VK_KHR_GET_MEMORY_REQUIREMENTS_2.ext_enum(2) as _,
    StructUsage::Source,
    &[Struct::member("image", "VkImage")],
)
.extensions(&[VK_KHR_GET_MEMORY_REQUIREMENTS_2])
.promoted(VERSION);
pub const MEMORY_REQUIREMENTS_2: &Struct = &Struct::typed(
    "MemoryRequirements2",
    "MEMORY_REQUIREMENTS_2",
    VK_KHR_GET_MEMORY_REQUIREMENTS_2.ext_enum(3) as _,
    StructUsage::Sink,
    &[Struct::member("memoryRequirements", "VkMemoryRequirements")],
)
.extensions(&[VK_KHR_GET_MEMORY_REQUIREMENTS_2])
.promoted(VERSION);
pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: &Struct = &Struct::typed(
    "SparseImageMemoryRequirements2",
    "SPARSE_IMAGE_MEMORY_REQUIREMENTS_2",
    VK_KHR_GET_MEMORY_REQUIREMENTS_2.ext_enum(4) as _,
    StructUsage::Sink,
    &[Struct::member("memoryRequirements", "VkSparseImageMemoryRequirements")],
)
.extensions(&[VK_KHR_GET_MEMORY_REQUIREMENTS_2])
.promoted(VERSION);
fn emit_get_memory_requirements2(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_GET_MEMORY_REQUIREMENTS_2.header_constants().emit(emitter);

    BUFFER_MEMORY_REQUIREMENTS_INFO_2.emit(emitter);
    IMAGE_MEMORY_REQUIREMENTS_INFO_2.emit(emitter);
    IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2.emit(emitter);
    MEMORY_REQUIREMENTS_2.emit(emitter);
    SPARSE_IMAGE_MEMORY_REQUIREMENTS_2.emit(emitter);

    Command::new(
        "GetImageMemoryRequirements2",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkImageMemoryRequirementsInfo2KHR"),
            ("pMemoryRequirements", "*mut VkMemoryRequirements2KHR"),
        ],
    )
    .extension(VK_KHR_GET_MEMORY_REQUIREMENTS_2)
    .emit(emitter);
    Command::new(
        "GetBufferMemoryRequirements2",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkBufferMemoryRequirementsInfo2KHR"),
            ("pMemoryRequirements", "*mut VkMemoryRequirements2KHR"),
        ],
    )
    .extension(VK_KHR_GET_MEMORY_REQUIREMENTS_2)
    .emit(emitter);
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
    .emit(emitter);
}

pub const VK_KHR_SAMPLER_YCBCR_CONVERSION: &Extension =
    &Extension::khr("sampler_ycbcr_conversion", 14, 157).promoted(VERSION);
pub const SAMPLER_YCBCR_MODEL_CONVERSION: &EnumType =
    &EnumType::new("SamplerYcbcrModelConversion", "SAMPLER_YCBCR_MODEL_CONVERSION")
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION);
pub const CHROMA_LOCATION: &EnumType =
    &EnumType::new("ChromaLocation", "CHROMA_LOCATION").extension(VK_KHR_SAMPLER_YCBCR_CONVERSION);
pub const SAMPLER_YCBCR_RANGE: &EnumType =
    &EnumType::new("SamplerYcbcrRange", "SAMPLER_YCBCR_RANGE").extension(VK_KHR_SAMPLER_YCBCR_CONVERSION);
pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const SAMPLER_YCBCR_CONVERSION_INFO: &Struct = &Struct::typed(
    "SamplerYcbcrConversionInfo",
    "SAMPLER_YCBCR_CONVERSION_INFO",
    VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(1) as _,
    StructUsage::Source,
    &[Struct::member("conversion", "VkSamplerYcbcrConversion")],
)
.extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
.promoted(VERSION);
pub const BIND_IMAGE_PLANE_MEMORY_INFO: &Struct = &Struct::typed(
    "BindImagePlaneMemoryInfo",
    "BIND_IMAGE_PLANE_MEMORY_INFO",
    VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(2) as _,
    StructUsage::Source,
    &[Struct::member("planeAspect", "VkImageAspectFlags")],
)
.extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
.promoted(VERSION);
pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: &Struct = &Struct::typed(
    "ImagePlaneMemoryRequirementsInfo",
    "IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO",
    VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(3) as _,
    StructUsage::Source,
    &[Struct::member("planeAspect", "VkImageAspectFlagBits")],
)
.extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
.promoted(VERSION);
pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: &Struct = &Struct::typed(
    "PhysicalDeviceSamplerYcbcrConversionFeatures",
    "PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES",
    VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(4) as _,
    StructUsage::Both,
    &[Struct::member("samplerYcbcrConversion", TY_VK_BOOL)],
)
.extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
.promoted(VERSION);
pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: &Struct = &Struct::typed(
    "SamplerYcbcrConversionImageFormatProperties",
    "SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES",
    VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(5) as _,
    StructUsage::Sink,
    &[Struct::member("combinedImageSamplerDescriptorCount", "u32")],
)
.extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
.promoted(VERSION);
fn emit_sampler_ycbcr_conversion(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_SAMPLER_YCBCR_CONVERSION.header_constants().emit(emitter);

    FORMAT
        .member("G8B8G8R8_422_UNORM", 0)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("B8G8R8G8_422_UNORM", 1)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G8_B8_R8_3PLANE_420_UNORM", 2)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G8_B8R8_2PLANE_420_UNORM", 3)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G8_B8_R8_3PLANE_422_UNORM", 4)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G8_B8R8_2PLANE_422_UNORM", 5)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G8_B8_R8_3PLANE_444_UNORM", 6)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R10X6_UNORM_PACK16", 7)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R10X6G10X6_UNORM_2PACK16", 8)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R10X6G10X6B10X6A10X6_UNORM_4PACK16", 9)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6B10X6G10X6R10X6_422_UNORM_4PACK16", 10)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("B10X6G10X6R10X6G10X6_422_UNORM_4PACK16", 11)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16", 12)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16", 13)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16", 14)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16", 15)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16", 16)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R12X4_UNORM_PACK16", 17)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R12X4G12X4_UNORM_2PACK16", 18)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("R12X4G12X4B12X4A12X4_UNORM_4PACK16", 19)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4B12X4G12X4R12X4_422_UNORM_4PACK16", 20)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("B12X4G12X4R12X4G12X4_422_UNORM_4PACK16", 21)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16", 22)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16", 23)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16", 24)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16", 25)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16", 26)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16B16G16R16_422_UNORM", 27)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("B16G16R16G16_422_UNORM", 28)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16_B16_R16_3PLANE_420_UNORM", 29)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16_B16R16_2PLANE_420_UNORM", 30)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16_B16_R16_3PLANE_422_UNORM", 31)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16_B16R16_2PLANE_422_UNORM", 32)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT
        .member("G16_B16_R16_3PLANE_444_UNORM", 33)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);

    FORMAT_FEATURE_FLAGS
        .entry("MIDPOINT_CHROMA_SAMPLES", 17)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT_FEATURE_FLAGS
        .entry("SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER", 18)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT_FEATURE_FLAGS
        .entry("SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER", 19)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT_FEATURE_FLAGS
        .entry("SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT", 20)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT_FEATURE_FLAGS
        .entry(
            "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE",
            21,
        )
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT_FEATURE_FLAGS
        .entry("DISJOINT", 22)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    FORMAT_FEATURE_FLAGS
        .entry("COSITED_CHROMA_SAMPLES", 23)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);

    IMAGE_ASPECT_FLAGS
        .entry("PLANE_0", 4)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    IMAGE_ASPECT_FLAGS
        .entry("PLANE_1", 5)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);
    IMAGE_ASPECT_FLAGS
        .entry("PLANE_2", 6)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);

    IMAGE_CREATE_FLAGS
        .entry("DISJOINT", 9)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);

    DEBUG_REPORT_OBJECT_TYPE
        .member("SAMPLER_YCBCR_CONVERSION", 0)
        .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
        .emit(emitter);

    SAMPLER_YCBCR_MODEL_CONVERSION.emit(emitter);
    SAMPLER_YCBCR_MODEL_CONVERSION
        .member("RGB_IDENTITY", 0)
        .override_extension_number(0)
        .emit(emitter);
    SAMPLER_YCBCR_MODEL_CONVERSION
        .member("YCBCR_IDENTITY", 1)
        .override_extension_number(0)
        .emit(emitter);
    SAMPLER_YCBCR_MODEL_CONVERSION
        .member("YCBCR_709", 2)
        .override_extension_number(0)
        .emit(emitter);
    SAMPLER_YCBCR_MODEL_CONVERSION
        .member("YCBCR_601", 3)
        .override_extension_number(0)
        .emit(emitter);
    SAMPLER_YCBCR_MODEL_CONVERSION
        .member("YCBCR_2020", 4)
        .override_extension_number(0)
        .emit(emitter);

    CHROMA_LOCATION.emit(emitter);
    CHROMA_LOCATION
        .member("COSITED_EVEN", 0)
        .override_extension_number(0)
        .emit(emitter);
    CHROMA_LOCATION
        .member("MIDPOINT", 1)
        .override_extension_number(0)
        .emit(emitter);

    SAMPLER_YCBCR_RANGE.emit(emitter);
    SAMPLER_YCBCR_RANGE
        .member("ITU_FULL", 0)
        .override_extension_number(0)
        .emit(emitter);
    SAMPLER_YCBCR_RANGE
        .member("ITU_NARROW", 1)
        .override_extension_number(0)
        .emit(emitter);

    SAMPLER_YCBCR_CONVERSION_CREATE_INFO.emit(emitter);
    SAMPLER_YCBCR_CONVERSION_INFO.emit(emitter);
    BIND_IMAGE_PLANE_MEMORY_INFO.emit(emitter);
    IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO.emit(emitter);
    PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES.emit(emitter);
    SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES.emit(emitter);

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
    .emit(emitter);
    Command::new(
        "DestroySamplerYcbcrConversion",
        &[
            ("device", "VkDevice"),
            ("ycbcrConversion", "VkSamplerYcbcrConversionKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .emit(emitter);
}

pub const VK_KHR_BIND_MEMORY_2: &Extension = &Extension::khr("bind_memory2", 1, 158).promoted(VERSION);
pub const BIND_BUFFER_MEMORY_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
pub const BIND_IMAGE_MEMORY_INFO: &Struct = &Struct::typed(
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
.promoted(VERSION);
fn emit_bind_memory2(emitter: &mut (impl RustCodeEmitter + ?Sized)) {
    VK_KHR_BIND_MEMORY_2.header_constants().emit(emitter);

    IMAGE_CREATE_FLAGS
        .entry("ALIAS", 10)
        .extension(VK_KHR_BIND_MEMORY_2)
        .emit(emitter);

    BIND_BUFFER_MEMORY_INFO.emit(emitter);
    BIND_IMAGE_MEMORY_INFO.emit(emitter);

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
    .emit(emitter);
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
    .emit(emitter);
}

pub fn emit_elements(w: &mut impl std::io::Write) -> std::io::Result<()> {
    // VK_KHR_descriptor_update_template
    Object::new(
        "VkDescriptorUpdateTemplateKHR",
        "DESCRIPTOR_UPDATE_TEMPLATE_KHR",
        VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE.ext_enum(0),
    )
    .extension(VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE)
    .promoted(VERSION, "VkDescriptorUpdateTemplate", "DESCRIPTOR_UPDATE_TEMPLATE")
    .emit(w)?;
    // VK_KHR_sampler_ycbcr_conversion
    Object::new(
        "VkSamplerYcbcrConversionKHR",
        "SAMPLER_YCBCR_CONVERSION_KHR",
        VK_KHR_SAMPLER_YCBCR_CONVERSION.ext_enum(0),
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .promoted(VERSION, "VkSamplerYcbcrConversion", "SAMPLER_YCBCR_CONVERSION")
    .emit(w)?;

    Ok(())
}
