//! Device Extention - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_pipeline_protected_access.html

pub const VK_EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION: usize = 1;
pub const VK_EXT_PIPELINE_PROTECTED_ACCESS_EXTENSION_NAME: &'static str = "VK_EXT_pipeline_protected_access";

use super::*;
use derives::promote_1_4;

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT: VkStructureType =
    ext_enum_value(467, 0) as _;

vk_bitmask! {
    extending enum VkPipelineCreateFlagBits {
        #[promote_1_4]
        pub VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT_EXT: 27,
        #[promote_1_4]
        pub VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT_EXT: 30
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure, VulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkPhysicalDevicePipelineProtectedAccessFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub pipelineProtectedAccess: VkBool32,
}
