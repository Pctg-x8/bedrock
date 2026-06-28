//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_host_image_copy.html

pub const VK_EXT_HOST_IMAGE_COPY_SPEC_VERISON: usize = 1;
pub const VK_EXT_HOST_IMAGE_COPY_EXTENSION_NAME: &str = "VK_EXT_host_image_copy";

use crate::*;
use derives::{TypedVulkanSinkStructure, TypedVulkanStructure, promote_1_4, vk_ext_command};

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT: VkStructureType = ext_enum_value(271, 0) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT: VkStructureType =
    ext_enum_value(271, 1) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_MEMORY_TO_IMAGE_COPY_EXT: VkStructureType = ext_enum_value(271, 2) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_IMAGE_TO_MEMORY_COPY_EXT: VkStructureType = ext_enum_value(271, 3) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_MEMORY_INFO_EXT: VkStructureType = ext_enum_value(271, 4) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_COPY_MEMORY_TO_IMAGE_INFO_EXT: VkStructureType = ext_enum_value(271, 5) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_HOST_IMAGE_LAYOUT_TRANSITION_INFO_EXT: VkStructureType = ext_enum_value(271, 6) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_IMAGE_INFO_EXT: VkStructureType = ext_enum_value(271, 7) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_SUBRESOURCE_HOST_MEMCPY_SIZE_EXT: VkStructureType = ext_enum_value(271, 8) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT: VkStructureType = ext_enum_value(271, 9) as _;
#[cfg(not(feature = "VK_KHR_maintenance5"))]
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_EXT: VkStructureType = ext_enum_value(339, 2) as _;
#[cfg(not(feature = "VK_KHR_maintenance5"))]
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT: VkStructureType = ext_enum_value(339, 3) as _;

vk_bitmask! {
    extending enum VkFormatFeatureFlagBits2KHR {
        #[promote_1_4]
        pub VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT_EXT: 34,
    }
}

vk_bitmask! {
    extending enum VkImageUsageFlagBits {
        #[promote_1_4]
        pub VK_IMAGE_USAGE_HOST_TRANSFER_BIT_EXT: 22
    }
}

#[promote_1_4(suffix = "EXT")]
pub type VkHostImageCopyFlagsEXT = VkFlags;
vk_bitmask! {
    #[promote_1_4(suffix = "EXT")]
    pub enum VkHostImageCopyFlagBitsEXT {
        #[promote_1_4]
        pub VK_HOST_IMAGE_COPY_MEMCPY_EXT: 0,
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure, TypedVulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkPhysicalDeviceHostImageCopyFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub hostImageCopy: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkPhysicalDeviceHostImageCopyPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub copySrcLayoutCount: u32,
    pub pCopySrcLayouts: *mut VkImageLayout,
    pub copyDstLayoutCount: u32,
    pub pCopyDstLayouts: *mut VkImageLayout,
    pub optimalTilingLayoutUUID: [u8; VK_UUID_SIZE],
    pub identicalMemoryTypeRequirements: VkBool32,
}

#[repr(C)]
#[derive(Clone, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_TO_IMAGE_COPY_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkMemoryToImageCopyEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pHostPointer: *const core::ffi::c_void,
    pub memoryRowLength: u32,
    pub memoryImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[repr(C)]
#[derive(Clone, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_TO_MEMORY_COPY_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkImageToMemoryCopyEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pHostPointer: *mut core::ffi::c_void,
    pub memoryRowLength: u32,
    pub memoryImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COPY_MEMORY_TO_IMAGE_INFO_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkCopyMemoryToImageInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkHostImageCopyFlagsEXT,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkMemoryToImageCopyEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COPY_IMAGE_TO_MEMORY_INFO_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkCopyImageToMemoryInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkHostImageCopyFlagsEXT,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageToMemoryCopyEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COPY_IMAGE_TO_IMAGE_INFO_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkCopyImageToImageInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkHostImageCopyFlagsEXT,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageCopy2KHR,
}

#[repr(C)]
#[derive(Clone, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_HOST_IMAGE_LAYOUT_TRANSITION_INFO_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkHostImageLayoutTransitionInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_SUBRESOURCE_HOST_MEMCPY_SIZE_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkSubresourceHostMemcpySizeEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkHostImageCopyDevicePerformanceQueryEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub optimalDeviceAccess: VkBool32,
    pub identicalMemoryLayout: VkBool32,
}

#[cfg(not(feature = "VK_KHR_maintenance5"))]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure, TypedVulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_EXT)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkSubresourceLayout2EXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub subresourceLayout: VkSubresourceLayout,
}

#[cfg(not(feature = "VK_KHR_maintenance5"))]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure, TypedVulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkImageSubresource2EXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub imageSubresource: VkImageSubresource,
}

vk_ext_command! {
    pub fn vkCopyMemoryToImageEXT(device: VkDevice, pCopyMemoryToImageInfo: *const VkCopyMemoryToImageInfoEXT) -> VkResult;
    suffix = "EXT";
    promote = "1.4";
}

vk_ext_command! {
    pub fn vkCopyImageToMemoryEXT(device: VkDevice, pCopyImageToMemoryInfo: *const VkCopyImageToMemoryInfoEXT) -> VkResult;
    suffix = "EXT";
    promote = "1.4";
}

vk_ext_command! {
    pub fn vkCopyImageToImageEXT(device: VkDevice, pCopyImageToImageInfo: *const VkCopyImageToImageInfoEXT) -> VkResult;
    suffix = "EXT";
    promote = "1.4";
}

vk_ext_command! {
    pub fn vkTransitionImageLayoutEXT(device: VkDevice, transitionCount: u32, pTransitions: *const VkHostImageLayoutTransitionInfoEXT) -> VkResult;
    suffix = "EXT";
    promote = "1.4";
}

#[cfg(not(feature = "VK_KHR_maintenance5"))]
vk_ext_command! {
    pub fn vkGetImageSubresourceLayout2EXT(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource2EXT, pLayout: *mut VkSubresourceLayout2EXT);
    suffix = "EXT";
    promote = "1.4";
}
