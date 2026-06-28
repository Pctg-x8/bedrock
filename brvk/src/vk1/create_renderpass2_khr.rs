//! VK_KHR_create_renderpass2

use crate::vk2::*;
use crate::*;
use derives::*;

pub const VK_KHR_CREATE_RENDERPASS_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME: &str = "VK_KHR_create_renderpass2";

#[promote_1_2]
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR: VkStructureType = ext_enum_value(110, 0) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR: VkStructureType = ext_enum_value(110, 1) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR: VkStructureType = ext_enum_value(110, 2) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR: VkStructureType = ext_enum_value(110, 3) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR: VkStructureType = ext_enum_value(110, 4) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR: VkStructureType = ext_enum_value(110, 5) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR: VkStructureType = ext_enum_value(110, 6) as _;

#[promote_1_2(suffix = "KHR")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR)]
pub struct VkRenderPassCreateInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription2KHR,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription2KHR,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency2KHR,
    pub correlatedViewMaskCount: u32,
    pub pCorrelatedViewMasks: *const u32,
}

#[promote_1_2(suffix = "KHR")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR)]
pub struct VkAttachmentDescription2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub stencilLoadOp: VkAttachmentLoadOp,
    pub stencilStoreOp: VkAttachmentStoreOp,
    pub initialLayout: VkImageLayout,
    pub finalLayout: VkImageLayout,
}

#[promote_1_2(suffix = "KHR")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR)]
pub struct VkAttachmentReference2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub attachment: u32,
    pub layout: VkImageLayout,
    pub aspectMask: VkImageAspectFlags,
}

#[promote_1_2(suffix = "KHR")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR)]
pub struct VkSubpassDescription2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSubpassDescriptionFlags,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub viewMask: u32,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference2KHR,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference2KHR,
    pub pResolveAttachments: *const VkAttachmentReference2KHR,
    pub pDepthStencilAttachment: *const VkAttachmentReference2KHR,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

#[promote_1_2(suffix = "KHR")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR)]
pub struct VkSubpassDependency2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
    pub viewOffset: i32,
}

#[promote_1_2(suffix = "KHR")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR)]
pub struct VkSubpassBeginInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub contents: VkSubpassContents,
}

#[promote_1_2(suffix = "KHR")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR)]
pub struct VkSubpassEndInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
}

vk_ext_command! {
    pub fn vkCreateRenderPass2KHR(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo2KHR, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
    suffix = "KHR";
    promote = "1.2";
}

vk_ext_command! {
    pub fn vkCmdBeginRenderPass2KHR(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, pSubpassBeginInfo: *const VkSubpassBeginInfoKHR);
    suffix = "KHR";
    promote = "1.2";
}

vk_ext_command! {
    pub fn vkCmdNextSubpass2KHR(commandBuffer: VkCommandBuffer, pSubpassBeginInfo: *const VkSubpassBeginInfoKHR, pSubpassEndInfo: *const VkSubpassEndInfoKHR);
    suffix = "KHR";
    promote = "1.2";
}

vk_ext_command! {
    pub fn vkCmdEndRenderPass2KHR(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfoKHR);
    suffix = "KHR";
    promote = "1.2";
}
