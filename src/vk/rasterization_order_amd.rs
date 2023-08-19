//! VK_AMD_rasterization_order extensions

pub const VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION: usize = 1;
pub static VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME: &'static str = "VK_AMD_rasterization_order";

use super::*;

pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: VkStructureType =
    ext_enum_value(19, 0) as _;

pub type VkRasterizationOrderAMD = i32;
pub const VK_RASTERIZATION_ORDER_STRICT_AMD: VkRasterizationOrderAMD = 0;
pub const VK_RASTERIZATION_ORDER_RELAXED_AMD: VkRasterizationOrderAMD = 1;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub rasterizationOrder: VkRasterizationOrderAMD,
}
