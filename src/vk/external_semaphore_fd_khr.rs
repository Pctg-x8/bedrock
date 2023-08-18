//! VK_KHR_external_semaphore_fd extensions

pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &'static str = "VK_KHR_external_semaphore_fd";

use super::*;
use crate::PFN;

pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR: VkStructureType = ext_enum_value(80, 0) as _;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR: VkStructureType = ext_enum_value(80, 1) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR)]
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
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR)]
pub struct VkSemaphoreGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlagsKHR,
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkImportSemaphoreFdKHR)]
pub struct PFN_vkImportSemaphoreFdKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetSemaphoreFdKHR)]
pub struct PFN_vkGetSemaphoreFdKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pGetFdInfo: *const VkSemaphoreGetFdInfoKHR,
        pFd: *mut c_int,
    ) -> VkResult,
);
