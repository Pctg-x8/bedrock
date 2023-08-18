//! VK_KHR_shared_presentable_image extension

pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: usize = 1;
pub static VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &'static str = "VK_KHR_shared_presentable_image";

use super::*;
use crate::PFN;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR)]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub sharedPresentSupportedUsageFlags: VkImageUsageFlags,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetSwapchainStatusKHR)]
pub struct PFN_vkGetSwapchainStatusKHR(
    pub unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetSwapchainStatusKHR(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}
