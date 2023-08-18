//! VK_EXT_direct_mode_display extensions

pub const VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: usize = 1;
pub static VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &'static str = "VK_EXT_direct_mode_display";

use super::*;

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkReleaseDisplayEXT)]
pub struct PFN_vkReleaseDisplayEXT(
    pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkReleaseDisplayEXT(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;
}
