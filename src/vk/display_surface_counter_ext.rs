//! VK_EXT_display_surface_counter extensions

pub const VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: usize = 1;
pub static VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &'static str = "VK_EXT_display_surface_counter";

use super::*;
use crate::PFN;

pub type VkSurfaceCounterFlagsEXT = VkFlags;
pub const VK_SURFACE_COUNTER_VBLANK_EXT: VkSurfaceCounterFlagsEXT = 0x01;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT)]
pub struct VkSurfaceCapabilities2EXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
    pub supportedSurfaceCounters: VkSurfaceCounterFlagsEXT,
}
impl VkSurfaceCapabilities2EXT {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = &mut *p.as_mut_ptr();
            x.sType = Self::TYPE;
            x.pNext = core::ptr::null_mut();
        }

        p
    }
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceSurfaceCapabilities2EXT)]
pub struct PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT,
    ) -> VkResult;
}
