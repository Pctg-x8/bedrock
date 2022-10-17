//! VK_KHR_get_surface_capabilities2 extensions

pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: usize = 1;
pub static VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &'static str = "VK_KHR_get_surface_capabilities2";

use crate::VulkanStructure;

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub surface: VkSurfaceKHR,
}
impl Default for VkPhysicalDeviceSurfaceInfo2KHR {
    fn default() -> Self {
        VkPhysicalDeviceSurfaceInfo2KHR {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
            ..unsafe { std::mem::zeroed() }
        }
    }
}
unsafe impl VulkanStructure for VkPhysicalDeviceSurfaceInfo2KHR {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR;
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceCapabilities2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub surfaceCapabilities: VkSurfaceCapabilitiesKHR,
}
unsafe impl VulkanStructure for VkSurfaceCapabilities2KHR {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR;
}
impl VkSurfaceCapabilities2KHR {
    pub fn uninit() -> std::mem::MaybeUninit<Self> {
        let mut p = std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            (*p.as_mut_ptr()).sType = Self::TYPE;
            (*p.as_mut_ptr()).pNext = std::ptr::null_mut();
        }
        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceFormat2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub surfaceFormat: VkSurfaceFormatKHR,
}
impl Default for VkSurfaceFormat2KHR {
    fn default() -> Self {
        VkSurfaceFormat2KHR {
            sType: VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR,
            surfaceFormat: Default::default(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}
unsafe impl crate::VulkanStructure for VkSurfaceFormat2KHR {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR;
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
