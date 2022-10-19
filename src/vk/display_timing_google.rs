//! VK_GOOGLE_display_timing extensions

pub const VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION: usize = 1;
pub static VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &'static str = "VK_GOOGLE_display_timing";

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkRefreshCycleDurationGOOGLE {
    pub refreshDuration: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPastPresentationTimingGOOGLE {
    pub presentID: u32,
    pub desiredPresentTime: u64,
    pub actualPresentTime: u64,
    pub earliestPresentTime: u64,
    pub presentMargin: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPresentTimeGOOGLE {
    pub presentID: u32,
    pub desiredPresentTime: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE"]
pub struct VkPresentTimesInfoGOOGLE {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pTimes: *const VkPresentTimeGOOGLE,
}

pub type PFN_vkGetRefreshCycleDurationGOOGLE = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE,
) -> VkResult;
pub type PFN_vkGetPastPresentationTimingGOOGLE = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pPresentationTimingCount: *mut u32,
    pPresentationTimings: *mut VkPastPresentationTimingGOOGLE,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetRefreshCycleDurationGOOGLE(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE,
    ) -> VkResult;
    pub fn vkGetPastPresentationTimingGOOGLE(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        pPresentationTimingCount: *mut u32,
        pPresentationTimings: *mut VkPastPresentationTimingGOOGLE,
    ) -> VkResult;
}
