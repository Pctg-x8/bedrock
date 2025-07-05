//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_dynamic_rendering_local_read.html

pub const VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION: usize = 1;
pub const VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_EXTENSION_NAME: &'static str = "VK_KHR_dynamic_rendering_local_read";

use crate::vk2::*;
use derives::{promote_1_4, vk_ext_command};

use super::*;

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR: VkStructureType =
    ext_enum_value(233, 0) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_LOCATION_INFO_KHR: VkStructureType = ext_enum_value(233, 1) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR: VkStructureType = ext_enum_value(233, 2) as _;

#[promote_1_4]
pub const VK_IMAGE_LAYOUT_RENDERING_LOCAL_READ_KHR: VkImageLayout = ext_enum_value(233, 0) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure, TypedVulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub dynamicRenderingLocalRead: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_LOCATION_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkRenderingAttachmentLocationInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentLocations: *const u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkRenderingInputAttachmentIndexInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentInputIndices: *const u32,
    pub pDepthInputAttachmentIndex: *const u32,
    pub pStencilInputAttachmentIndex: *const u32,
}

vk_ext_command! {
    pub fn vkCmdSetRenderingAttachmentLocationsKHR(commandBuffer: VkCommandBuffer, pLocationInfo: *const VkRenderingAttachmentLocationInfoKHR);
    suffix = "KHR";
    promote = "1.4";
}

vk_ext_command! {
    pub fn vkCmdSetRenderingInputAttachmentIndicesKHR(commandBuffer: VkCommandBuffer, pInputAttachmentIndexInfo: *const VkRenderingInputAttachmentIndexInfoKHR);
    suffix = "KHR";
    promote = "1.4";
}
