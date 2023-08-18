//! VK_KHR_xlib_surface extensions

use super::*;
use crate::PFN;
use x11::xlib::*;

pub const VK_KHR_XLIB_SURFACE_SPEC_VERSION: usize = 6;
pub static VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_xlib_surface";

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

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateXlibSurfaceKHR)]
pub struct PFN_vkCreateXlibSurfaceKHR(
    pub  unsafe extern "system" fn(
        instance: VkInstance,
        pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceXlibPresentationSupportKHR)]
pub struct PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        dpy: *mut Display,
        visualID: VisualID,
    ) -> VkBool32,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCreateXlibSurfaceKHR(
        instance: VkInstance,
        pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        dpy: *mut Display,
        visualID: VisualID,
    ) -> VkBool32;
}
