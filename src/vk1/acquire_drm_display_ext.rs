//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_acquire_drm_display.html

pub const VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION: usize = 1;
pub const VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME: &'static str = "VK_EXT_acquire_drm_display";

use derives::vk_ext_command;

vk_ext_command! {
    pub fn vkAcquireDrmDisplayEXT(physicalDevice: crate::vk2::VkPhysicalDevice, drmFd: i32, display: crate::vk2::VkDisplayKHR) -> crate::vk2::VkResult;
}

vk_ext_command! {
    pub fn vkGetDrmDisplayEXT(physicalDevice: crate::vk2::VkPhysicalDevice, drmFd: i32, connectorId: u32, display: *mut crate::vk2::VkDisplayKHR) -> crate::vk2::VkResult;
}
