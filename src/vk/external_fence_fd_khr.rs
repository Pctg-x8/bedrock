//! VK_KHR_external_fence_fd extensions

pub const VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: &'static str = "VK_KHR_external_fence_fd";

use super::*;
use libc::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImportFenceFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlags,
    pub handleType: VkExternalFenceHandleTypeFlags,
    pub fd: c_int,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkFenceGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlags,
}
impl Default for VkImportFenceFdInfoKHR {
    fn default() -> Self {
        VkImportFenceFdInfoKHR {
            sType: VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR,
            ..unsafe { std::mem::zeroed() }
        }
    }
}
impl Default for VkFenceGetFdInfoKHR {
    fn default() -> Self {
        VkFenceGetFdInfoKHR {
            sType: VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR,
            ..unsafe { std::mem::zeroed() }
        }
    }
}

pub type PFN_vkImportFenceFdKHR =
    extern "system" fn(device: VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR) -> VkResult;
pub type PFN_vkGetFenceFdKHR =
    extern "system" fn(device: VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut c_int) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkImportFenceFdKHR(device: VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR) -> VkResult;
    pub fn vkGetFenceFdKHR(device: VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut c_int) -> VkResult;
}
