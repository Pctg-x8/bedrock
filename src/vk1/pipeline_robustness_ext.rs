//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_pipeline_robustness.html

pub const VK_EXT_PIPELINE_ROBUSTNESS_SPEC_VERISON: usize = 1;
pub const VK_EXT_PIPELINE_ROBUSTNESS_EXTENSION_NAME: &'static str = "VK_EXT_pipeline_robustness";

use super::*;
use crate::vk2::*;
use derives::promote_1_4;

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT: VkStructureType = ext_enum_value(69, 0) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT: VkStructureType =
    ext_enum_value(69, 1) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT: VkStructureType =
    ext_enum_value(69, 2) as _;

#[promote_1_4(suffix = "EXT")]
pub type VkPipelineRobustnessBufferBehaviorEXT = i32;
#[promote_1_4]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT_EXT: VkPipelineRobustnessBufferBehaviorEXT = 0;
#[promote_1_4]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED_EXT: VkPipelineRobustnessBufferBehaviorEXT = 1;
#[promote_1_4]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_EXT: VkPipelineRobustnessBufferBehaviorEXT = 2;
#[promote_1_4]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2_EXT: VkPipelineRobustnessBufferBehaviorEXT = 3;

#[promote_1_4(suffix = "EXT")]
pub type VkPipelineRobustnessImageBehaviorEXT = i32;
#[promote_1_4]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT_EXT: VkPipelineRobustnessImageBehaviorEXT = 0;
#[promote_1_4]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED_EXT: VkPipelineRobustnessImageBehaviorEXT = 1;
#[promote_1_4]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_EXT: VkPipelineRobustnessImageBehaviorEXT = 2;
#[promote_1_4]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2_EXT: VkPipelineRobustnessImageBehaviorEXT = 3;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure, TypedVulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkPhysicalDevicePipelineRobustnessFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub pipelineRobustness: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkPhysicalDevicePipelineRobustnessPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub defaultRobustnessStorageBuffers: VkPipelineRobustnessBufferBehaviorEXT,
    pub defaultRobustnessUniformBuffers: VkPipelineRobustnessBufferBehaviorEXT,
    pub defaultRobustnessVertexInputs: VkPipelineRobustnessBufferBehaviorEXT,
    pub defaultRobustnessImage: VkPipelineRobustnessImageBehaviorEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT)]
#[promote_1_4(suffix = "EXT")]
pub struct VkPipelineRobustnessCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub storageBuffers: VkPipelineRobustnessBufferBehaviorEXT,
    pub uniformBuffers: VkPipelineRobustnessBufferBehaviorEXT,
    pub vertexInputs: VkPipelineRobustnessBufferBehaviorEXT,
    pub images: VkPipelineRobustnessImageBehaviorEXT,
}
