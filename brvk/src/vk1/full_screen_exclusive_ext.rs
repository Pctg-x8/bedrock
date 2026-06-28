//! VK_EXT_full_screen_exclusive

pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: usize = 4;
pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &str = "VK_EXT_full_screen_exclusive";

use derives::{TypedVulkanSinkStructure, TypedVulkanStructure, vk_ext_command};

use crate::*;

pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: VkStructureType = ext_enum_value(256, 0) as _;
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: VkStructureType =
    ext_enum_value(256, 2) as _;

pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: VkResult = VkResult::ext_err_value(256, 0);

pub type VkFullScreenExclusiveEXT = i32;
pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: VkFullScreenExclusiveEXT = 0;
pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: VkFullScreenExclusiveEXT = 1;
pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: VkFullScreenExclusiveEXT = 2;
pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: VkFullScreenExclusiveEXT = 3;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanSinkStructure, TypedVulkanStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub fullScreenExclusive: VkFullScreenExclusiveEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub fullScreenExclusiveSupported: VkBool32,
}

vk_ext_command! {
    pub fn vkGetPhysicalDeviceSurfacePresentModes2EXT(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
}

vk_ext_command! {
    pub fn vkAcquireFullScreenExclusiveModeEXT(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}

vk_ext_command! {
    pub fn vkReleaseFullScreenExclusiveModeEXT(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}

#[cfg(feature = "VK_KHR_win32_surface")]
pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: VkStructureType = ext_enum_value(256, 1) as _;

#[cfg(feature = "VK_KHR_win32_surface")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub hmonitor: windows::Win32::Graphics::Gdi::HMONITOR,
}

#[cfg(feature = "VK_KHR_device_group")]
vk_ext_command! {
    pub fn vkGetDeviceGroupSurfacePresentModes2EXT(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult;
}
