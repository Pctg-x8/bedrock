//! VK_KHR_line_rasterization

use crate::*;
use derives::{TypedVulkanSinkStructure, TypedVulkanStructure, promote_1_4, vk_ext_command};

pub const VK_KHR_LINE_RASTERIZATION_SPEC_VERSION: usize = 1;
pub const VK_KHR_LINE_RASTERIZATION_EXTENSION_NAME: &str = "VK_KHR_line_rasterization";

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR: VkStructureType =
    ext_enum_value(260, 0) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR: VkStructureType =
    ext_enum_value(260, 1) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR: VkStructureType =
    ext_enum_value(260, 2) as _;

#[promote_1_4(suffix = "KHR")]
pub type VkLineRasterizationModeKHR = core::ffi::c_int;
#[promote_1_4]
pub const VK_LINE_RASTERIZATION_MODE_DEFAULT_KHR: VkLineRasterizationModeKHR = 0;
#[promote_1_4]
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_KHR: VkLineRasterizationModeKHR = 1;
#[promote_1_4]
pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM_KHR: VkLineRasterizationModeKHR = 2;
#[promote_1_4]
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_KHR: VkLineRasterizationModeKHR = 3;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceLineRasterizationFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub rectangularLines: VkBool32,
    pub bresenhamLines: VkBool32,
    pub smoothLines: VkBool32,
    pub stippledRectangularLines: VkBool32,
    pub stippledBresenhamLines: VkBool32,
    pub stippledSmoothLines: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceLineRasterizationPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub lineSubPixelPrecisionBits: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPipelineRasterizationLineStateCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub lineRasterizationMode: VkLineRasterizationModeKHR,
    pub stippledLineEnable: VkBool32,
    pub lineStippleFactor: u32,
    pub lineStipplePattern: u16,
}

vk_ext_command! {
    pub fn vkCmdSetLineStippleKHR(commandBuffer: VkCommandBuffer, lineStippleFactor: u32, lineStipplePattern: u16);
    suffix = "KHR";
    promote = "1.4";
}
