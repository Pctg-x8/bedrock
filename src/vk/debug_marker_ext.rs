//! VK_EXT_debug_marker extensions

pub const VK_EXT_DEBUG_MARKER_SPEC_VERSION: usize = 4;
pub static VK_EXT_DEBUG_MARKER_EXTENSION_NAME: &'static str = "VK_EXT_debug_marker";

use super::*;
use crate::PFN;

pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT: VkStructureType = ext_enum_value(23, 0) as _;
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT: VkStructureType = ext_enum_value(23, 1) as _;
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT: VkStructureType = ext_enum_value(23, 2) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT)]
pub struct VkDebugMarkerObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkDebugReportObjectTypeEXT,
    pub object: u64,
    pub pObjectName: *const c_char,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT)]
pub struct VkDebugMarkerObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkDebugReportObjectTypeEXT,
    pub object: u64,
    pub tagName: u64,
    pub tagSize: size_t,
    pub pTag: *const c_void,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT)]
pub struct VkDebugMarkerMarkerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pMarkerName: *const c_char,
    pub color: [c_float; 4],
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDebugMarkerSetObjectTagEXT)]
pub struct PFN_vkDebugMarkerSetObjectTagEXT(
    pub unsafe extern "system" fn(device: VkDevice, pTagInfo: *const VkDebugMarkerObjectTagInfoEXT) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDebugMarkerSetObjectNameEXT)]
pub struct PFN_vkDebugMarkerSetObjectNameEXT(
    pub unsafe extern "system" fn(device: VkDevice, pNameInfo: *const VkDebugMarkerObjectNameInfoEXT) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDebugMarkerBeginEXT)]
pub struct PFN_vkCmdDebugMarkerBeginEXT(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDebugMarkerEndEXT)]
pub struct PFN_vkCmdDebugMarkerEndEXT(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer));
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDebugMarkerInsertEXT)]
pub struct PFN_vkCmdDebugMarkerInsertEXT(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT),
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkDebugMarkerSetObjectTagEXT(device: VkDevice, pTagInfo: *const VkDebugMarkerObjectTagInfoEXT) -> VkResult;
    pub fn vkDebugMarkerSetObjectNameEXT(
        device: VkDevice,
        pNameInfo: *const VkDebugMarkerObjectNameInfoEXT,
    ) -> VkResult;
}
