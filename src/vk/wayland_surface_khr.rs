//! VK_KHR_wayland_surface extensions

use derives::vk_ext_command;

use super::*;

pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: usize = 6;
pub static VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_wayland_surface";

pub const VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(7, 0) as _;

pub type VkWaylandSurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR)]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWaylandSurfaceCreateFlagsKHR,
    /// `*mut wl_display`
    pub display: *mut c_void,
    /// `*mut wl_surface`
    pub surface: *mut c_void,
}

vk_ext_command!(
    pub fn vkCreateWaylandSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    suffix = "KHR";
    static_callable;
);
vk_ext_command!(
    pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut c_void) -> VkBool32;
    suffix = "KHR";
    static_callable;
);
