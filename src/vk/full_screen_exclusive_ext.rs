//! VK_EXT_full_screen_exclusive

pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: usize = 4;
pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &str = "VK_EXT_full_screen_exclusive";

use super::*;
use crate::PFN;

pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: VkStructureType = ext_enum_value(256, 0) as _;
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: VkStructureType =
    ext_enum_value(256, 2) as _;

pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: VkResult =
    unsafe { -std::mem::transmute::<_, VkResult>(ext_enum_value(256, 0) as u32) };

pub type VkFullScreenExclusiveEXT = i32;
pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: VkFullScreenExclusiveEXT = 0;
pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: VkFullScreenExclusiveEXT = 1;
pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: VkFullScreenExclusiveEXT = 2;
pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: VkFullScreenExclusiveEXT = 3;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fullScreenExclusive: VkFullScreenExclusiveEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub fullScreenExclusiveSupported: VkBool32,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceSurfacePresentModes2EXT)]
pub struct PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkAcquireFullScreenExclusiveModeEXT)]
pub struct PFN_vkAcquireFullScreenExclusiveModeEXT(
    pub unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkReleaseFullScreenExclusiveModeEXT)]
pub struct PFN_vkReleaseFullScreenExclusiveModeEXT(
    pub unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetPhysicalDeviceSurfacePresentModes2EXT(
        physicalDevice: VkPhysicalDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR,
    ) -> VkResult;
    pub fn vkAcquireFullScreenExclusiveModeEXT(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
    pub fn vkReleaseFullScreenExclusiveModeEXT(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_win32_surface")] {
        pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: VkStructureType = ext_enum_value(256, 1) as _;

        #[repr(C)]
        #[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
        #[VulkanStructure(type = VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT)]
        pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
            pub sType: VkStructureType,
            pub pNext: *const c_void,
            pub hmonitor: windows::Win32::Graphics::Gdi::HMONITOR,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_device_group")] {
        #[repr(transparent)] #[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)] #[pfn_of(vkGetDeviceGroupSurfacePresentModes2EXT)] pub struct PFN_vkGetDeviceGroupSurfacePresentModes2EXT(pub unsafe extern "system" fn(
            physicalDevice: VkPhysicalDevice,
            pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
            pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
        ) -> VkResult);

        #[cfg(feature = "Implements")]
        #[cfg(not(feature = "DynamicLoaded"))]
        extern "system" {
            pub fn vkGetDeviceGroupSurfacePresentModes2EXT(
                physicalDevice: VkPhysicalDevice,
                pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
                pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
            ) -> VkResult;
        }
    }
}
