//! device extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_acquire_winrt_display.html

use derives::vk_ext_command;

use super::*;

pub const VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: usize = 1;
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &'static str = "VK_NV_acquire_winrt_display";

vk_ext_command! {
    pub fn vkAcquireWinrtDisplayNV(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;
    suffix = "NV";
}

vk_ext_command! {
    pub fn vkGetWinrtDisplayNV(physicalDevice: VkPhysicalDevice, deviceRelativeId: u32, pDisplay: *mut VkDisplayKHR) -> VkResult;
    suffix = "NV";
}
