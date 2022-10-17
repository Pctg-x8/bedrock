//! VK_KHR_external_semaphore_fd extensions

pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &'static str = "VK_KHR_external_semaphore_fd";

use super::*;
use libc::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR"]
pub struct VkImportSemaphoreFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub flags: VkSemaphoreImportFlagsKHR,
    pub handleType: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub fd: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR"]
pub struct VkSemaphoreGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlagsKHR,
}

pub type PFN_vkImportSemaphoreFdKHR =
    extern "system" fn(device: VkDevice, pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR) -> VkResult;
pub type PFN_vkGetSemaphoreFdKHR =
    extern "system" fn(device: VkDevice, pGetFdInfo: *const VkSemaphoreGetFdInfoKHR, pFd: *mut c_int) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkImportSemaphoreFdKHR(
        device: VkDevice,
        pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR,
    ) -> VkResult;
    pub fn vkGetSemaphoreFdKHR(
        device: VkDevice,
        pGetFdInfo: *const VkSemaphoreGetFdInfoKHR,
        pFd: *mut c_int,
    ) -> VkResult;
}
