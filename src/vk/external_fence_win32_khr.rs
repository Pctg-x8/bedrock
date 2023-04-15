//! VK_KHR_external_fence_win32 extensions

pub const VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &'static str = "VK_KHR_external_fence_win32";

use super::*;

#[cfg(windows)]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR)]
pub struct VkImportFenceWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlagsKHR,
    pub handleType: VkExternalFenceHandleTypeFlagsKHR,
    pub handle: windows::Win32::Foundation::HANDLE,
    pub name: windows::core::PCWSTR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR)]
pub struct VkExportFenceWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const windows::Win32::Security::SECURITY_ATTRIBUTES,
    pub dwAccess: u32,
    pub name: windows::core::PCWSTR,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR)]
pub struct VkFenceGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagsKHR,
}

pub type PFN_vkImportFenceWin32HandleKHR = extern "system" fn(
    device: VkDevice,
    pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR,
) -> VkResult;
pub type PFN_vkGetFenceWin32HandleKHR = extern "system" fn(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR,
    pHandle: *mut windows::Win32::Foundation::HANDLE,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkImportFenceWin32HandleKHR(
        device: VkDevice,
        pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR,
    ) -> VkResult;
    pub fn vkGetFenceWin32HandleKHR(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR,
        pHandle: *mut windows::Win32::Foundation::HANDLE,
    ) -> VkResult;
}
