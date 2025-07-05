//! VK_KHR_display_swapchain extensions

use super::*;
use derives::vk_ext_command;

pub const VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR: VkStructureType = ext_enum_value(4, 0) as _;

pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: VkResult = VkResult::ext_err_value(4, 1);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR)]
pub struct VkDisplayPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcRect: VkRect2D,
    pub dstRect: VkRect2D,
    pub persistent: VkBool32,
}

vk_ext_command! {
    pub fn vkCreateSharedSwapchainKHR(device: VkDevice, swapchainCount: u32, pCreateInfos: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchains: *mut VkSwapchainKHR) -> VkResult;
}
