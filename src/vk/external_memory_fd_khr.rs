//! VK_KHR_external_memory_fd extensions

pub const VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &'static str = "VK_KHR_external_memory";

use super::*;
use libc::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR"]
pub struct VkImportMemoryFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlags,
    pub fd: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR"]
pub struct VkMemoryFdPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR"]
pub struct VkMemoryGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlags,
}

pub type PFN_vkGetMemoryFdKHR =
    extern "system" fn(device: VkDevice, pGetFdInfo: *const VkMemoryGetFdInfoKHR, pFd: *mut c_int) -> VkResult;
pub type PFN_vkGetMemoryFdPropertiesKHR = extern "system" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlags,
    fd: c_int,
    pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetMemoryFdKHR(device: VkDevice, pGetFdInfo: *const VkMemoryGetFdInfoKHR, pFd: *mut c_int) -> VkResult;
    pub fn vkGetMemoryFdPropertiesKHR(
        device: VkDevice,
        handleType: VkExternalMemoryHandleTypeFlags,
        fd: c_int,
        pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR,
    ) -> VkResult;
}
