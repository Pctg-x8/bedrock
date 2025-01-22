//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_shader_float_controls2.html

pub const VK_KHR_SHADER_FLOAT_CONTROLS_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_SHADER_FLOAT_CONTROLS_2_EXTENSION_NAME: &'static str = "VK_KHR_shader_float_controls2";

use super::*;
use derives::promote_1_4;

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR: VkStructureType =
    ext_enum_value(529, 0) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure, VulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceShaderFloatControls2FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderFloatControls2: VkBool32,
}
