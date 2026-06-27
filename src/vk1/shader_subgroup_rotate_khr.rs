//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_shader_subgroup_rotate.html

pub const VK_KHR_SHADER_SUBGROUP_ROTATE_SPEC_VERSION: usize = 2;
pub const VK_KHR_SHADER_SUBGROUP_ROTATE_EXTENSION_NAME: &str = "VK_KHR_shader_subgroup_rotate";

use super::*;
use crate::vk2::*;
use derives::promote_1_4;

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR: VkStructureType =
    ext_enum_value(417, 0) as _;

vk_bitmask! {
    extending enum VkSubgroupFeatureFlagBits {
        #[promote_1_4]
        pub VK_SUBGROUP_FEATURE_ROTATE_BIT_KHR: 9,
        #[promote_1_4]
        pub VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT_KHR: 10,
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure, TypedVulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderSubgroupRotate: VkBool32,
    pub shaderSubgroupRotateClustered: VkBool32,
}
