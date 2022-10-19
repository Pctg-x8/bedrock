//! VK_NV_external_memory_win32 extensions

pub const VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: usize = 1;
pub static VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static str = "VK_NV_external_memory_win32";

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV"]
pub struct VkImportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsNV,
    pub handle: winapi::shared::ntdef::HANDLE,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV"]
pub struct VkExportMemoryWin32HandleInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const winapi::um::minwinbase::SECURITY_ATTRIBUTES,
    pub dwAccess: winapi::shared::minwindef::DWORD,
}

pub type PFN_vkGetMemoryWin32HandleNV = extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    handleType: VkExternalMemoryHandleTypeFlagsNV,
    pHandle: *mut winapi::shared::ntdef::HANDLE,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetMemoryWin32HandleNV(
        device: VkDevice,
        memory: VkDeviceMemory,
        handleType: VkExternalMemoryHandleTypeFlagsNV,
        pHandle: *mut winapi::shared::ntdef::HANDLE,
    ) -> VkResult;
}
