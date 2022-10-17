//! VK_KHR_win32_surface extensions

use super::*;
use libc::c_void;
use winapi::shared::minwindef::HINSTANCE;
use winapi::shared::windef::HWND;

pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: usize = 6;
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &str = "VK_KHR_win32_surface";

pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR"]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}

pub type PFN_vkCreateWin32SurfaceKHR = extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
    extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCreateWin32SurfaceKHR(
        instance: VkInstance,
        pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
    ) -> VkBool32;
}
