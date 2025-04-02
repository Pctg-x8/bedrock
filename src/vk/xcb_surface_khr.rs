//! VK_KHR_xcb_surface extensions

use super::*;
use derives::vk_ext_command;
use xcb::ffi::*;

pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: usize = 6;
pub static VK_KHR_XCB_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_xcb_surface";

pub const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(6, 0) as _;

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

vk_ext_command!(
    pub fn vkCreateXcbSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    suffix = "KHR";
    static_callable;
);
vk_ext_command!(
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_connection_t, visual_id: xcb::x::Visualid) -> VkBool32;
    suffix = "KHR";
    static_callable;
);
