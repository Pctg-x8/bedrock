//! VK_KHR_external_memory_win32 extensions

pub const VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static str = "VK_KHR_external_memory_win32";

use super::*;
use crate::PFN;

pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = ext_enum_value(74, 0) as _;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = ext_enum_value(74, 1) as _;
pub const VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR: VkStructureType = ext_enum_value(74, 2) as _;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = ext_enum_value(74, 3) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR)]
pub struct VkImportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
    pub handle: windows::Win32::Foundation::HANDLE,
    pub name: windows::core::PCWSTR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR)]
pub struct VkExportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const windows::Win32::Security::SECURITY_ATTRIBUTES,
    pub dwAccess: u32,
    pub name: windows::core::PCWSTR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR)]
pub struct VkMemoryWin32HandlePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryTypeBits: u32,
}
impl VkMemoryWin32HandlePropertiesKHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = &mut *p.as_mut_ptr();
            x.sType = Self::TYPE;
            x.pNext = core::ptr::null_mut();
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR)]
pub struct VkMemoryGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetMemoryWin32HandleKHR)]
pub struct PFN_vkGetMemoryWin32HandleKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
        pHandle: *mut windows::Win32::Foundation::HANDLE,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetMemoryWin32HandlePropertiesKHR)]
pub struct PFN_vkGetMemoryWin32HandlePropertiesKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        handleType: VkExternalMemoryHandleTypeFlagsKHR,
        handle: windows::Win32::Foundation::HANDLE,
        pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
    ) -> VkResult,
);
