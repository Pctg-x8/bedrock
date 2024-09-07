//! VK_KHR_xlib_surface extensions

use super::*;
use derives::{implements, vk_ext_command};
use x11::xlib::*;

pub const VK_KHR_XLIB_SURFACE_SPEC_VERSION: usize = 6;
pub static VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_xlib_surface";

pub const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(5, 0) as _;

pub type VkXlibSurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR)]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXlibSurfaceCreateFlagsKHR,
    pub dpy: *mut Display,
    pub window: Window,
}

vk_ext_command!(
    pub fn vkCreateXlibSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    suffix = "KHR";
    static_callable;
);
vk_ext_command!(
    pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut Display, visualID: VisualID) -> VkBool32;
    suffix = "KHR";
    static_callable;
);
