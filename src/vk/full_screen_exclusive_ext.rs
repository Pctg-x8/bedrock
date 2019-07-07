//! VK_EXT_full_screen_exclusive

pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: usize = 3;
pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &str = "VK_EXT_full_screen_exclusive";

use super::*;

pub type VkFullScreenExclusiveEXT = i32;
pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: VkFullScreenExclusiveEXT = 0;
pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: VkFullScreenExclusiveEXT = 1;
pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: VkFullScreenExclusiveEXT = 2;
pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: VkFullScreenExclusiveEXT = 3;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub fullScreenExclusive: VkFullScreenExclusiveEXT;
}
impl Default for VkSurfaceFullScreenExclusiveInfoEXT
{
    fn default() -> Self
    {
        VkSurfaceFullScreenExclusiveInfoEXT
        {
            sType: VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT,
            .. unsafe { std::mem::zeroed() }
        }
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive_win32")]
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub hmonitor: winapi::shared::windef::HMONITOR
}
#[cfg(feature = "VK_EXT_full_screen_exclusive_win32")]
impl Default for VkSurfaceFullScreenExclusiveWin32InfoEXT
{
    fn default() -> Self
    {
        VkSurfaceFullScreenExclusiveWin32InfoEXT
        {
            sType: VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT,
            .. unsafe { std::mem::zeroed() }
        }
    }
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub fullScreenExclusiveSupported: VkBool32
}

pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = extern "system" fn(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
#[cfg(feature = "VK_KHR_device_group")]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = extern "system" fn(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult;
pub type PFN_vkAcquireFullScreenExclusiveModeEXT = extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
pub type PFN_vkReleaseFullScreenExclusiveModeEXT = extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;

#[cfg(feature = "Implements")]
extern "system"
{
    fn vkGetPhysicalDeviceSurfacePresentModes2EXT(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_device_group")]
    fn vkGetDeviceGroupSurfacePresentModes2EXT(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult;
    fn vkAcquireFullScreenExclusiveModeEXT(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
    fn vkReleaseFullScreenExclusiveModeEXT(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}
