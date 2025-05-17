//! VK_KHR_shared_presentable_image extension

pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: usize = 1;
pub static VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &'static str = "VK_KHR_shared_presentable_image";

use super::*;
use crate::PFN;

pub const VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR: VkImageLayout = ext_enum_value(112, 0) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR)]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub sharedPresentSupportedUsageFlags: VkImageUsageFlags,
}

vk_ext_command! {
    pub fn vkGetSwapchainStatusKHR(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}
