//! VK_EXT_vertex_attribute_divisor extension

pub const VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: usize = 1;
pub const VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &str = "VK_EXT_vertex_attribute_divisor";

use super::*;

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT"]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxVertexAttribDivisor: u32,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VkVertexInputBindingDivisorDescriptionEXT {
    pub binding: u32,
    pub divisor: u32,
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT"]
pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub vertexBindingDivisorCount: u32,
    pub pVertexBindingDivisors: *const VkVertexInputBindingDivisorDescriptionEXT,
}
