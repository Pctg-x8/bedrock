//! VK_KHR_get_surface_capabilities2 extensions

pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: usize = 1;
pub static VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &'static str = "VK_KHR_get_surface_capabilities2";

use crate::VulkanStructure;

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub surface: VkSurfaceKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR)]
pub struct VkSurfaceCapabilities2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub surfaceCapabilities: VkSurfaceCapabilitiesKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR)]
pub struct VkSurfaceFormat2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub surfaceFormat: VkSurfaceFormatKHR,
}

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormat2KHR,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(
        physicalDevice: VkPhysicalDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfaceFormats2KHR(
        physicalDevice: VkPhysicalDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pSurfaceFormatCount: *mut u32,
        pSurfaceFormats: *mut VkSurfaceFormat2KHR,
    ) -> VkResult;
}
