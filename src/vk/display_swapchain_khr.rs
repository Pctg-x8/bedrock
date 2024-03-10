//! VK_KHR_display_swapchain extensions

use super::*;
use crate::PFN;

pub const VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR: VkStructureType = ext_enum_value(4, 0) as _;

pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: VkResult = VkResult::ext_err_value(4, 1);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR)]
pub struct VkDisplayPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcRect: VkRect2D,
    pub dstRect: VkRect2D,
    pub persistent: VkBool32,
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateSharedSwapchainsKHR)]
pub struct PFN_vkCreateSharedSwapchainsKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        swapchainCount: u32,
        pCreateInfos: *const VkSwapchainCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSwapchains: *mut VkSwapchainKHR,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCreateSharedSwapchainsKHR(
        device: VkDevice,
        swapchainCount: u32,
        pCreateInfos: *const VkSwapchainCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSwapchains: *mut VkSwapchainKHR,
    ) -> VkResult;
}
