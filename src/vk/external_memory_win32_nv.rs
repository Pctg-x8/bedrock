//! VK_NV_external_memory_win32 extensions

pub const VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: usize = 1;
pub static VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static str = "VK_NV_external_memory_win32";

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV)]
pub struct VkImportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsNV,
    pub handle: windows::Win32::Foundation::HANDLE,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV)]
pub struct VkExportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const windows::Win32::Security::SECURITY_ATTRIBUTES,
    pub dwAccess: u32,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetMemoryWin32HandleNV)]
pub struct PFN_vkGetMemoryWin32HandleNV(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        memory: VkDeviceMemory,
        handleType: VkExternalMemoryHandleTypeFlagsNV,
        pHandle: *mut windows::Win32::Foundation::HANDLE,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetMemoryWin32HandleNV(
        device: VkDevice,
        memory: VkDeviceMemory,
        handleType: VkExternalMemoryHandleTypeFlagsNV,
        pHandle: *mut windows::Win32::Foundation::HANDLE,
    ) -> VkResult;
}
