//! VK_KHR_win32_keyed_mutex extensions

pub const VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: usize = 1;
pub static VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: &'static str = "VK_KHR_win32_keyed_mutex";

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub acquireCount: u32,
    pub pAcquireSyncs: *const VkDeviceMemory,
    pub pAcquireKeys: *const u64,
    pub pAcquireTimeouts: *const u32,
    pub releaseCount: u32,
    pub pReleaseSyncs: *const VkDeviceMemory,
    pub pReleaseKeys: *const u64,
}
