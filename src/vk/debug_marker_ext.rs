//! VK_EXT_debug_marker extensions

pub const VK_EXT_DEBUG_MARKER_SPEC_VERSION: usize = 4;
pub static VK_EXT_DEBUG_MARKER_EXTENSION_NAME: &'static str = "VK_EXT_debug_marker";

use super::*;
use derives::vk_ext_command;

pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT: VkStructureType = ext_enum_value(23, 0) as _;
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT: VkStructureType = ext_enum_value(23, 1) as _;
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT: VkStructureType = ext_enum_value(23, 2) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT)]
pub struct VkDebugMarkerObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkDebugReportObjectTypeEXT,
    pub object: u64,
    pub pObjectName: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT)]
pub struct VkDebugMarkerObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkDebugReportObjectTypeEXT,
    pub object: u64,
    pub tagName: u64,
    pub tagSize: usize,
    pub pTag: *const c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT)]
pub struct VkDebugMarkerMarkerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pMarkerName: *const c_char,
    pub color: [c_float; 4],
}

vk_ext_command! {
    pub fn vkDebugMarkerSetObjectTagEXT(device: VkDevice, pTagInfo: *const VkDebugMarkerObjectTagInfoEXT) -> VkResult;
}

vk_ext_command! {
    pub fn vkDebugMarkerSetObjectNameEXT(device: VkDevice, pNameInfo: *const VkDebugMarkerObjectNameInfoEXT) -> VkResult;
}

vk_ext_command! {
    pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);
}

vk_ext_command! {
    pub fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer);
}

vk_ext_command! {
    pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);
}
