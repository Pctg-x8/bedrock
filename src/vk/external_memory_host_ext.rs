//! VK_EXT_external_memory_host extension

pub const VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: usize = 1;
pub const VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: &str = "VK_EXT_external_memory_host";

use super::*;
use crate::PFN;

pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT: VkStructureType = ext_enum_value(179, 0) as _;
pub const VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT: VkStructureType = ext_enum_value(179, 1) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: VkStructureType =
    ext_enum_value(179, 2) as _;

vk_bitmask! {
    extending enum VkExternalMemoryHandleTypeFlagBitsKHR {
        pub VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT: 7,
        pub VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT: 8
    }
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT)]
pub struct VkImportMemoryHostPointerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
    pub pHostPointer: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT)]
pub struct VkMemoryHostPointerPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT)]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minImportedHostPointerAlignment: VkDeviceSize,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetMemoryHostPointerPropertiesEXT)]
pub struct PFN_vkGetMemoryHostPointerPropertiesEXT(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        handleType: VkExternalMemoryHandleTypeFlagsKHR,
        pHostPointer: *const c_void,
        pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT,
    ) -> VkResult,
);
