//! VK_KHR_external_semaphore_win32 extensions

pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &'static str = "VK_KHR_external_semaphore_win32";

use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub semaphore: VkSemaphore, pub flags: VkSemaphoreImportFlags,
    pub handleType: VkExternalSemaphoreHandleTypeFlags,
    pub handle: winapi::shared::ntdef::HANDLE, pub name: winapi::shared::ntdef::LPCWSTR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub pAttributes: *const winapi::um::minwinbase::SECURITY_ATTRIBUTES,
    pub dwAccess: winapi::shared::minwindef::DWORD, pub name: winapi::shared::ntdef::LPCWSTR
}
impl Default for VkImportSemaphoreWin32HandleInfoKHR {
    fn default() -> Self {
        VkImportSemaphoreWin32HandleInfoKHR {
            sType: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            .. unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
        }
    }
}
impl Default for VkExportSemaphoreWin32HandleInfoKHR {
    fn default() -> Self {
        VkExportSemaphoreWin32HandleInfoKHR {
            sType: VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            .. unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkD3D12FenceSubmitInfoKHR {
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub waitSemaphoreValuesCount: u32, pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValuesCount: u32, pub pSignalSemaphoreValues: *const u64
}
impl Default for VkD3D12FenceSubmitInfoKHR {
    fn default() -> Self {
        VkD3D12FenceSubmitInfoKHR {
            sType: VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR,
            .. unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub semaphore: VkSemaphore, pub handleType: VkExternalSemaphoreHandleTypeFlags
}
impl Default for VkSemaphoreGetWin32HandleInfoKHR {
    fn default() -> Self {
        VkSemaphoreGetWin32HandleInfoKHR {
            sType: VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
            .. unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
        }
    }
}

pub type PFN_vkImportSemaphoreWin32HandleKHR = extern "system" fn(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> VkResult;
pub type PFN_vkGetSemaphoreWin32HandleKHR = extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut winapi::shared::ntdef::HANDLE) -> VkResult;

#[cfg(feature = "Implements")]
extern "system" {
    pub fn vkImportSemaphoreWin32HandleKHR(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> VkResult;
    pub fn vkGetSemaphoreWin32HandleKHR(device: VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut winapi::shared::ntdef::HANDLE) -> VkResult;
}
