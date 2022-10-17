//! VK_KHR_external_memory_win32 extensions

pub const VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static str = "VK_KHR_external_memory_win32";

use super::*;
use winapi::shared::{
    minwindef::DWORD,
    ntdef::{HANDLE, LPCWSTR},
};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR"]
pub struct VkImportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlags,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR"]
pub struct VkExportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const winapi::um::minwinbase::SECURITY_ATTRIBUTES,
    pub dwAccess: DWORD,
    pub name: LPCWSTR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR"]
pub struct VkMemoryWin32HandlePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR"]
pub struct VkMemoryGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlags,
}

pub type PFN_vkGetMemoryWin32HandleKHR = extern "system" fn(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult;
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = extern "system" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlags,
    handle: HANDLE,
    pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetMemoryWin32HandleKHR(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
        pHandle: *mut HANDLE,
    ) -> VkResult;
    pub fn vkGetMemoryWin32HandlePropertiesKHR(
        device: VkDevice,
        handleType: VkExternalMemoryHandleTypeFlags,
        handle: HANDLE,
        pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
    ) -> VkResult;
}
