//! VK_KHR_wayland_surface extensions

use super::*;
use wayland_client::sys::*;

pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: usize = 6;
pub static VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_wayland_surface";

pub type VkWaylandSurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR)]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWaylandSurfaceCreateFlagsKHR,
    pub display: *mut wl_display,
    pub surface: *mut wl_proxy, /*wl_surface*/
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkCreateWaylandSurfaceKHR"]
pub struct PFN_vkCreateWaylandSurfaceKHR(
    pub  extern "system" fn(
        instance: VkInstance,
        pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkGetPhysicalDeviceWaylandPresentationSupportKHR"]
pub struct PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR(
    pub  extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        display: *mut wl_display,
    ) -> VkBool32,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCreateWaylandSurfaceKHR(
        instance: VkInstance,
        pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        display: *mut wayland_client::sys::wl_display,
    ) -> VkBool32;
}
