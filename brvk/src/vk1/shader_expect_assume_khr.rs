//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_shader_expect_assume.html

pub const VK_KHR_SHADER_EXPECT_ASSUME_SPEC_VERSION: usize = 1;
pub const VK_KHR_SHADER_EXPECT_ASSUME_EXTENSION_NAME: &str = "VK_KHR_shader_expect_assume";

use crate::*;
use derives::{TypedVulkanSinkStructure, TypedVulkanStructure, promote_1_4};

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR: VkStructureType =
    ext_enum_value(545, 0) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure, TypedVulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceShaderExpectAssumeFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderExpectAssume: VkBool32,
}
