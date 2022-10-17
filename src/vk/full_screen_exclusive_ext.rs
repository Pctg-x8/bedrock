//! VK_EXT_full_screen_exclusive

pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: usize = 3;
pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &str = "VK_EXT_full_screen_exclusive";

use crate::VulkanStructure;

use super::*;

pub type VkFullScreenExclusiveEXT = i32;
pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: VkFullScreenExclusiveEXT = 0;
pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: VkFullScreenExclusiveEXT = 1;
pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: VkFullScreenExclusiveEXT = 2;
pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: VkFullScreenExclusiveEXT = 3;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fullScreenExclusive: VkFullScreenExclusiveEXT,
}
unsafe impl VulkanStructure for VkSurfaceFullScreenExclusiveInfoEXT {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT;
}
impl Default for VkSurfaceFullScreenExclusiveInfoEXT {
    fn default() -> Self {
        VkSurfaceFullScreenExclusiveInfoEXT {
            sType: VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT,
            ..unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
        }
    }
}

#[cfg(windows)]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub hmonitor: winapi::shared::windef::HMONITOR,
}
unsafe impl VulkanStructure for VkSurfaceFullScreenExclusiveWin32InfoEXT {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT;
}
#[cfg(windows)]
impl Default for VkSurfaceFullScreenExclusiveWin32InfoEXT {
    fn default() -> Self {
        VkSurfaceFullScreenExclusiveWin32InfoEXT {
            sType: Self::TYPE,
            ..unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fullScreenExclusiveSupported: VkBool32,
}
unsafe impl VulkanStructure for VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT;
}
impl VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    pub fn zeroed() -> std::mem::MaybeUninit<Self> {
        let mut p = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            (*p.as_mut_ptr()).sType = Self::TYPE;
        }
        p
    }
}

pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut VkPresentModeKHR,
) -> VkResult;
#[cfg(feature = "VK_KHR_device_group")]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult;
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetPhysicalDeviceSurfacePresentModes2EXT(
        physicalDevice: VkPhysicalDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR,
    ) -> VkResult;
    #[cfg(feature = "VK_KHR_device_group")]
    pub fn vkGetDeviceGroupSurfacePresentModes2EXT(
        physicalDevice: VkPhysicalDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
    ) -> VkResult;
    pub fn vkAcquireFullScreenExclusiveModeEXT(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
    pub fn vkReleaseFullScreenExclusiveModeEXT(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}
