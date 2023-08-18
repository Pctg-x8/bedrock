//! VK_NN_vi_surface extensions

pub const VK_NN_VI_SURFACE_SPEC_VERSION: usize = 1;
pub static VK_NN_VI_SURFACE_EXTENSION_NAME: &'static str = "VK_NN_vi_surface";

use super::*;
use crate::PFN;

pub type VkViSurfaceCreateFlagsNN = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN)]
pub struct VkViSurfaceCreateInfoNN {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkViSurfaceCreateFlagsNN,
    pub window: *mut c_void,
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateViSurfaceNN)]
pub struct PFN_vkCreateViSurfaceNN(
    pub  unsafe extern "system" fn(
        instance: VkInstance,
        pCreateInfo: *const VkViSurfaceCreateInfoNN,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCreateViSurfaceNN(
        instance: VkInstance,
        pCreateInfo: *const VkViSurfaceCreateInfoNN,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
}
