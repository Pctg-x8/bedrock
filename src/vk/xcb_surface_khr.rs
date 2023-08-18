//! VK_KHR_xcb_surface extensions

use super::*;
use xcb::ffi::*;

pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: usize = 6;
pub static VK_KHR_XCB_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_xcb_surface";

pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb::x::Window,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkCreateXcbSurfaceKHR"]
pub struct PFN_vkCreateXcbSurfaceKHR(
    pub  extern "system" fn(
        instance: VkInstance,
        pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkGetPhysicalDeviceXcbPresentationSupportKHR"]
pub struct PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR(
    pub  extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb::x::Visualid,
    ) -> VkBool32,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCreateXcbSurfaceKHR(
        instance: VkInstance,
        pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb::x::Visualid,
    ) -> VkBool32;
}
