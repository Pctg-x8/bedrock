//! VK_KHR_external_semaphore_win32 extensions

pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &'static str = "VK_KHR_external_semaphore_win32";

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR"]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub flags: VkSemaphoreImportFlags,
    pub handleType: VkExternalSemaphoreHandleTypeFlags,
    pub handle: windows::Win32::Foundation::HANDLE,
    pub name: windows::core::PCWSTR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR"]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const windows::Win32::Security::SECURITY_ATTRIBUTES,
    pub dwAccess: u32,
    pub name: windows::core::PCWSTR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR"]
pub struct VkD3D12FenceSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreValuesCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValuesCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR"]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlags,
}

pub type PFN_vkImportSemaphoreWin32HandleKHR = extern "system" fn(
    device: VkDevice,
    pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR,
) -> VkResult;
pub type PFN_vkGetSemaphoreWin32HandleKHR = extern "system" fn(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR,
    pHandle: *mut windows::Win32::Foundation::HANDLE,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkImportSemaphoreWin32HandleKHR(
        device: VkDevice,
        pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR,
    ) -> VkResult;
    pub fn vkGetSemaphoreWin32HandleKHR(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR,
        pHandle: *mut windows::Win32::Foundation::HANDLE,
    ) -> VkResult;
}
