//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_shader_float_controls.html

pub const VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION: usize = 4;
pub const VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME: &'static str = "VK_KHR_shader_float_controls";

use super::*;
use crate::vk2::*;
use derives::promote_1_2;

#[promote_1_2]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: VkStructureType =
    ext_enum_value(198, 0) as _;

#[promote_1_2(suffix = "KHR")]
pub type VkShaderFloatControlsIndependenceKHR = i32;
#[promote_1_2]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR: VkShaderFloatControlsIndependenceKHR = 0;
#[promote_1_2]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR: VkShaderFloatControlsIndependenceKHR = 1;
#[promote_1_2]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR: VkShaderFloatControlsIndependenceKHR = 2;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR)]
#[promote_1_2(suffix = "KHR")]
pub struct VkPhysicalDeviceFloatControlsPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub denormBehaviorIndependence: VkShaderFloatControlsIndependenceKHR,
    pub roundingModeIndependence: VkShaderFloatControlsIndependenceKHR,
    pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
    pub shaderDenormPreserveFloat16: VkBool32,
    pub shaderDenormPreserveFloat32: VkBool32,
    pub shaderDenormPreserveFloat64: VkBool32,
    pub shaderDenormFlushToZeroFloat16: VkBool32,
    pub shaderDenormFlushToZeroFloat32: VkBool32,
    pub shaderDenormFlushToZeroFloat64: VkBool32,
    pub shaderRoundingModeRTEFloat16: VkBool32,
    pub shaderRoundingModeRTEFloat32: VkBool32,
    pub shaderRoundingModeRTEFloat64: VkBool32,
    pub shaderRoundingModeRTZFloat16: VkBool32,
    pub shaderRoundingModeRTZFloat32: VkBool32,
    pub shaderRoundingModeRTZFloat64: VkBool32,
}
