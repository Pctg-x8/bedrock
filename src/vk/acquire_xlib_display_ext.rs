//! VK_EXT_acquire_xlib_display extensions

pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: usize = 1;
pub static VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &'static str = "VK_EXT_acquire_xlib_display";

use super::*;
use derives::vk_ext_command;
use x11::xlib::Display;
use x11::xrandr::RROutput;

vk_ext_command! {
    pub fn vkAcquireXlibDisplayEXT(physicalDevice: VkPhysicalDevice, dpy: *mut Display, display: VkDisplayKHR) -> VkResult;
}

vk_ext_command! {
    pub fn vkGetRandROutputDisplayEXT(physicalDevice: VkPhysicalDevice, dpy: *mut Display, rrOutput: RROutput, pDisplay: *mut VkDisplayKHR) -> VkResult;
}
