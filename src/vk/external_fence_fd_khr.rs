//! VK_KHR_external_fence_fd extensions

pub const VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: &'static str = "VK_KHR_external_fence_fd";

use super::*;

pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR: VkStructureType = ext_enum_value(116, 0) as _;
pub const VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR: VkStructureType = ext_enum_value(116, 1) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR)]
pub struct VkImportFenceFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlagsKHR,
    pub handleType: VkExternalFenceHandleTypeFlagsKHR,
    pub fd: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR)]
pub struct VkFenceGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagsKHR,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkImportFenceFdKHR)]
pub struct PFN_vkImportFenceFdKHR(
    pub unsafe extern "system" fn(device: VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetFenceFdKHR)]
pub struct PFN_vkGetFenceFdKHR(
    pub unsafe extern "system" fn(device: VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut c_int) -> VkResult,
);
