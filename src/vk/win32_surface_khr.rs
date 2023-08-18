//! VK_KHR_win32_surface extensions

use super::*;

pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: usize = 6;
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &str = "VK_KHR_win32_surface";

pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: windows::Win32::Foundation::HINSTANCE,
    pub hwnd: windows::Win32::Foundation::HWND,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkCreateWin32SurfaceKHR"]
pub struct PFN_vkCreateWin32SurfaceKHR(
    pub  extern "system" fn(
        instance: VkInstance,
        pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkGetPhysicalDeviceWin32PresentationSupportKHR"]
pub struct PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR(
    pub extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32,
);

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
