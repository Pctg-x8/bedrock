//! External Memory Import/Export Operations

use crate::*;

#[cfg(feature = "VK_KHR_external_memory")]
pub type ExternalMemoryImageCreateInfo = VkExternalMemoryImageCreateInfoKHR;
#[cfg(feature = "VK_KHR_external_memory")]
impl ExternalMemoryImageCreateInfo {
    #[inline(always)]
    pub const fn new(handle_types: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
        Self {
            sType: VkExternalMemoryImageCreateInfoKHR::TYPE,
            pNext: core::ptr::null(),
            handleTypes: handle_types,
        }
    }
}

#[cfg(feature = "VK_KHR_external_memory_win32")]
mod memory_win32;
#[cfg(feature = "VK_KHR_external_memory_win32")]
pub use self::memory_win32::*;

#[cfg(feature = "VK_KHR_external_memory_fd")]
mod memory_fd;
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub use self::memory_fd::*;

#[cfg(feature = "VK_EXT_external_memory_host")]
mod memory_host;
#[cfg(feature = "VK_EXT_external_memory_host")]
pub use self::memory_host::*;

#[cfg(feature = "VK_KHR_external_fence_fd")]
mod fence_fd;
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub use self::fence_fd::*;

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
mod semaphore_win32;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub use self::semaphore_win32::*;
