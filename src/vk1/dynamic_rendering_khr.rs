//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_dynamic_rendering.html

pub const VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION: usize = 1;
pub const VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME: &'static str = "VK_KHR_dynamic_rendering";

use super::*;
use crate::vk2::*;
use derives::{promote_1_3, vk_ext_command};

#[promote_1_3]
pub const VK_STRUCTURE_TYPE_RENDERING_INFO_KHR: VkStructureType = ext_enum_value(45, 0) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO_KHR: VkStructureType = ext_enum_value(45, 1) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO_KHR: VkStructureType = ext_enum_value(45, 2) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: VkStructureType =
    ext_enum_value(45, 3) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: VkStructureType = ext_enum_value(45, 4) as _;

vk_bitmask! {
    #[promote_1_3(suffix = "KHR")]
    pub enum VkRenderingFlagBitsKHR {
        #[promote_1_3]
        pub VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR: 0,
        #[promote_1_3]
        pub VK_RENDERING_SUSPENDING_BIT_KHR: 1,
        #[promote_1_3]
        pub VK_RENDERING_RESUMING_BIT_KHR: 2,
    }
}
#[promote_1_3(suffix = "KHR")]
pub type VkRenderingFlagsKHR = VkFlags;

// 302??(ただしこの数値で正解らしい)
#[promote_1_3]
pub const VK_ATTACHMENT_STORE_OP_NONE_KHR: VkAttachmentStoreOp = ext_enum_value(302, 0) as _;

#[repr(C)]
#[derive(Clone, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDERING_INFO_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkRenderingInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkRenderingFlagsKHR,
    pub renderArea: VkRect2D,
    pub layerCount: u32,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkRenderingAttachmentInfoKHR,
    pub pDepthAttachment: *const VkRenderingAttachmentInfoKHR,
    pub pStencilAttachment: *const VkRenderingAttachmentInfoKHR,
}

#[repr(C)]
#[derive(Clone, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkRenderingAttachmentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
    pub resolveMode: VkResolveModeFlagBitsKHR,
    pub resolveImageView: VkImageView,
    pub resolveImageLayout: VkImageLayout,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub clearValue: VkClearValue,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkPipelineRenderingCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentFormats: *const VkFormat,
    pub depthAttachmentFormat: VkFormat,
    pub stencilAttachmnetFormat: VkFormat,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure, TypedVulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkPhysicalDeviceDynamicRenderingFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub dynamicRendering: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkCommandBufferInheritanceRenderingInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkRenderingFlagsKHR,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentFormats: *const VkFormat,
    pub depthAttachmentFormat: VkFormat,
    pub stencilAttachmentFormat: VkFormat,
    pub rasterizationSamples: VkSampleCountFlagBits,
}

vk_ext_command! {
    pub fn vkCmdBeginRenderingKHR(commandBuffer: VkCommandBuffer, pRenderingInfo: *const VkRenderingInfoKHR);
    suffix = "KHR";
    promote = "1.3";
}

vk_ext_command! {
    pub fn vkCmdEndRenderingKHR(commandBuffer: VkCommandBuffer);
    suffix = "KHR";
    promote = "1.3";
}
