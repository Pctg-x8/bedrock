//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_depth_stencil_resolve.html

pub const VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: usize = 1;
pub const VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME: &'static str = "VK_KHR_depth_stencil_resolve";

use crate::vk2::*;
use derives::promote_1_2;

use super::*;

#[promote_1_2]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: VkStructureType =
    ext_enum_value(200, 0) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: VkStructureType =
    ext_enum_value(200, 1) as _;

vk_bitmask! {
    #[promote_1_2(suffix = "KHR")]
    pub enum VkResolveModeFlagBitsKHR {
        #[promote_1_2(suffix = "KHR")]
        pub VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR: 0,
        #[promote_1_2(suffix = "KHR")]
        pub VK_RESOLVE_MODE_AVERAGE_BIT_KHR: 1,
        #[promote_1_2(suffix = "KHR")]
        pub VK_RESOLVE_MODE_MIN_BIT_KHR: 2,
        #[promote_1_2(suffix = "KHR")]
        pub VK_RESOLVE_MODE_MAX_BIT_KHR: 3,
    }
}
#[promote_1_2(suffix = "KHR")]
pub type VkResolveModeFlagsKHR = VkFlags;
#[promote_1_2(suffix = "KHR")]
pub const VK_RESOLVE_MODE_NONE_KHR: VkResolveModeFlagBitsKHR = 0;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR)]
#[promote_1_2(suffix = "KHR")]
pub struct VkPhysicalDeviceDepthStencilResolvePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub supportedDepthResolveModes: VkResolveModeFlagsKHR,
    pub supportedStencilResolveModes: VkResolveModeFlagsKHR,
    pub independentResolveNone: VkBool32,
    pub independentResolve: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR)]
#[promote_1_2(suffix = "KHR")]
pub struct VkSubpassDescriptionDepthStencilResolveKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub depthResolveMode: VkResolveModeFlagBitsKHR,
    pub stencilResolveMode: VkResolveModeFlagBitsKHR,
    pub pDepthStencilResolveAttachment: *const VkAttachmentReference2KHR,
}
