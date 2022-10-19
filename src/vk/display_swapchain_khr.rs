//! VK_KHR_display_swapchain extensions

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR"]
pub struct VkDisplayPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcRect: VkRect2D,
    pub dstRect: VkRect2D,
    pub persistent: VkBool32,
}

pub type PFN_vkCreateSharedSwapchainsKHR = extern "system" fn(
    device: VkDevice,
    swapchainCount: u32,
    pCreateInfos: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchains: *mut VkSwapchainKHR,
) -> VkResult;

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
