//! VK_KHR_shared_presentable_image extension

pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: usize = 1;
pub static VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &'static str = "VK_KHR_shared_presentable_image";

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR"]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub sharedPresentSupportedUsageFlags: VkImageUsageFlags,
}

pub type PFN_vkGetSwapchainStatusKHR = extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetSwapchainStatusKHR(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}
