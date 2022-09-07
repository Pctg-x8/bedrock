//! Physical Device Extended Queries

use crate::vk::*;

#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
unsafe impl crate::ext::VulkanStructure for VkPhysicalDeviceSurfaceInfo2KHR {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR;
}
