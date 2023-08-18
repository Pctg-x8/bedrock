//! VK_MVK_ios_surface extensions

pub const VK_MVK_IOS_SURFACE_SPEC_VERSION: usize = 2;
pub static VK_MVK_IOS_SURFACE_EXTENSION_NAME: &'static str = "VK_MVK_ios_surface";

use super::*;

pub type VkIOSSurfaceCreateFlagsMVK = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK)]
pub struct VkIOSSurfaceCreateInfoMVK {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkIOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkCreateIOSSurfaceMVK"]
pub struct PFN_vkCreateIOSSurfaceMVK(
    pub  extern "system" fn(
        instance: VkInstance,
        pCreateInfo: *const VkIOSSurfaceCreateInfoMVK,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCreateIOSSurfaceMVK(
        instance: VkInstance,
        pCreateInfo: *const VkIOSSurfaceCreateInfoMVK,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
}
