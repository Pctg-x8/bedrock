//! VK_KHR_win32_surface extensions

use super::*;
use derives::vk_ext_command;

pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: usize = 6;
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &str = "VK_KHR_win32_surface";

pub const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(10, 0) as _;

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

vk_ext_command!(
    pub fn vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    suffix = "KHR";
    static_callable;
);
vk_ext_command!(
    pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;
    suffix = "KHR";
    static_callable;
);
