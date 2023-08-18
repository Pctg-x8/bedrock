//! VK_EXT_acquire_xlib_display extensions

pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: usize = 1;
pub static VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &'static str = "VK_EXT_acquire_xlib_display";

use super::*;
use x11::xlib::Display;
use x11::xrandr::RROutput;

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkAcquireXlibDisplayEXT)]
pub struct PFN_vkAcquireXlibDisplayEXT(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        dpy: *mut Display,
        display: VkDisplayKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetRandROutputDisplayEXT)]
pub struct PFN_vkGetRandROutputDisplayEXT(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        dpy: *mut Display,
        rrOutput: RROutput,
        pDisplay: *mut VkDisplayKHR,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkAcquireXlibDisplayEXT(
        physicalDevice: VkPhysicalDevice,
        dpy: *mut Display,
        display: VkDisplayKHR,
    ) -> VkResult;
    pub fn vkGetRandROutputDisplayEXT(
        physicalDevice: VkPhysicalDevice,
        dpy: *mut Display,
        rrOutput: RROutput,
        pDisplay: *mut VkDisplayKHR,
    ) -> VkResult;
}
