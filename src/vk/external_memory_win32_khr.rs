//! VK_KHR_external_memory_win32 extensions

pub const VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static str = "VK_KHR_external_memory_win32";

use derives::vk_ext_command;

use super::*;

pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = ext_enum_value(74, 0) as _;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = ext_enum_value(74, 1) as _;
pub const VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR: VkStructureType = ext_enum_value(74, 2) as _;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = ext_enum_value(74, 3) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR)]
pub struct VkImportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
    pub handle: windows::Win32::Foundation::HANDLE,
    pub name: windows::core::PCWSTR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR)]
pub struct VkExportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const windows::Win32::Security::SECURITY_ATTRIBUTES,
    pub dwAccess: u32,
    pub name: windows::core::PCWSTR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR)]
pub struct VkMemoryWin32HandlePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR)]
pub struct VkMemoryGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
}

vk_ext_command! {
    pub fn vkGetMemoryWin32HandleKHR(device: VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut windows::Win32::Foundation::HANDLE) -> VkResult;
}

vk_ext_command! {
    pub fn vkGetMemoryWin32HandlePropertiesKHR(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagsKHR, handle: windows::Win32::Foundation::HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR) -> VkResult;
}
