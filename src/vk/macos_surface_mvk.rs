//! VK_MVK_macos_surface extensions

pub const VK_MVK_MACOS_SURFACE_SPEC_VERSION: usize = 2;
pub static VK_MVK_MACOS_SURFACE_EXTENSION_NAME: &'static str = "VK_MVK_macos_surface";

use super::*;
use libc::*;

pub type VkMacOSSurfaceCreateFlagsMVK = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK"]
pub struct VkMacOSSurfaceCreateInfoMVK {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMacOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

pub type PFN_vkCreateMacOSSurfaceMVK = extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCreateMacOSSurfaceMVK(
        instance: VkInstance,
        pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
}
