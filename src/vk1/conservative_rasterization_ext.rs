//! VK_EXT_conservative_rasterization extension

pub const VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: usize = 1;
pub const VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME: &str = "VK_EXT_conservative_rasterization";

use super::*;
use crate::vk2::*;

pub type VkConservativeRasterizationModeEXT = i32;
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT: VkConservativeRasterizationModeEXT = 0;
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT: VkConservativeRasterizationModeEXT = 1;
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT: VkConservativeRasterizationModeEXT = 2;

pub type VkPipelineRasterizationConservativeStateCreateFlagsEXT = crate::vk2::VkFlags;

#[repr(C)]
#[derive(Clone, Debug, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT)]
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
    pub sType: crate::vk2::VkStructureType,
    pub pNext: *mut c_void,
    pub primitiveOverestimationSize: c_float,
    pub maxExtraPrimitiveOverestimationSize: c_float,
    pub extraPrimitiveOverestimationSizeGranularity: c_float,
    pub primitiveUnderestimation: crate::vk2::VkBool32,
    pub conservativePointAndLineRasterization: crate::vk2::VkBool32,
    pub degenerateTrianglesRasterized: crate::vk2::VkBool32,
    pub degenerateLinesRasterized: crate::vk2::VkBool32,
    pub fullyCoveredFragmentShaderInputVariable: crate::vk2::VkBool32,
    pub conservativeRasterizationPostDepthCoverage: crate::vk2::VkBool32,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT)]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
    pub sType: crate::vk2::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
    pub conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
    pub extraPrimitiveOverestimationSize: c_float,
}
