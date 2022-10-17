//! VK_EXT_discard_rectangles extensions

pub const VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION: usize = 1;
pub static VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &'static str = "VK_EXT_discard_rectangles";

use super::*;
use libc::*;

pub type VkDiscardRectangleModeEXT = i32;
pub const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: VkDiscardRectangleModeEXT = 0;
pub const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: VkDiscardRectangleModeEXT = 1;

pub type VkPipelineDiscardRectangleStateCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT"]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxDiscardRectangles: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT"]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
    pub discardRectangleMode: VkDiscardRectangleModeEXT,
    pub discardRectangleCount: u32,
    pub pDiscardRectangles: *const VkRect2D,
}

pub type PFN_vkCmdSetDiscardRectangleEXT = extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstDiscardRectangle: u32,
    discardRectangleCount: u32,
    pDiscardRectangles: *const VkRect2D,
);
