//! VK_KHR_external_memory_fd extensions

pub const VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &'static str = "VK_KHR_external_memory";

use super::*;
use crate::PFN;

pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR: VkStructureType = ext_enum_value(75, 0) as _;
pub const VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR: VkStructureType = ext_enum_value(75, 1) as _;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR: VkStructureType = ext_enum_value(75, 2) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR)]
pub struct VkImportMemoryFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
    pub fd: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR)]
pub struct VkMemoryFdPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR)]
pub struct VkMemoryGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetMemoryFdKHR)]
pub struct PFN_vkGetMemoryFdKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pGetFdInfo: *const VkMemoryGetFdInfoKHR,
        pFd: *mut c_int,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetMemoryFdPropertiesKHR)]
pub struct PFN_vkGetMemoryFdPropertiesKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        handleType: VkExternalMemoryHandleTypeFlagsKHR,
        fd: c_int,
        pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR,
    ) -> VkResult,
);
