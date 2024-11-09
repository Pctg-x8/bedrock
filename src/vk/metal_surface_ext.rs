//! VK_EXT_metal_surface extensions

use derives::vk_ext_command;

use super::*;

pub const VK_EXT_METAL_SURFACE_SPEC_VERSION: usize = 1;
pub const VK_EXT_METAL_SURFACE_EXTENSION_NAME: &'static str = "VK_EXT_metal_surface";

pub const VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT: VkStructureType = ext_enum_value(218, 0) as _;

pub type VkMetalSurfaceCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT)]
pub struct VkMetalSurfaceCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkMetalSurfaceCreateFlagsEXT,
    /// *const CAMetalLayer
    pub pLayer: *const core::ffi::c_void,
}

vk_ext_command!(
    pub fn vkCreateMetalSurfaceEXT(instance: VkInstance, pCreateInfo: *const VkMetalSurfaceCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    suffix = "EXT";
    static_callable;
);
