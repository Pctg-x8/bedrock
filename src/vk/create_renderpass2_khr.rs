//! VK_KHR_create_renderpass2

use super::*;
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR)]
pub struct VkRenderPassCreateInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR)]
pub struct VkAttachmentDescription2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR)]
pub struct VkAttachmentReference2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachment: u32,
    pub layout: VkImageLayout,
    pub aspectMask: VkImageAspectFlags,
}

#[promote_1_2(suffix = "KHR")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR)]
pub struct VkSubpassDescription2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR)]
pub struct VkSubpassDependency2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR)]
pub struct VkSubpassBeginInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub contents: VkSubpassContents,
}

#[promote_1_2(suffix = "KHR")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR)]
pub struct VkSubpassEndInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
}

#[implements]
#[promote_1_2(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateRenderPass2KHR)]
pub struct PFN_vkCreateRenderPass2KHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkRenderPassCreateInfo2KHR,
        pAllocator: *const VkAllocationCallbacks,
        pRenderPass: *mut VkRenderPass,
    ) -> VkResult,
);

#[implements]
#[promote_1_2(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdBeginRenderPass2KHR)]
pub struct PFN_vkCmdBeginRenderPass2KHR(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        pRenderPassBegin: *const VkRenderPassBeginInfo,
        pSubpassBeginInfo: *const VkSubpassBeginInfoKHR,
    ),
);

#[implements]
#[promote_1_2(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdNextSubpass2KHR)]
pub struct PFN_vkCmdNextSubpass2KHR(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        pSubpassBeginInfo: *const VkSubpassBeginInfoKHR,
        pSubpassEndInfo: *const VkSubpassEndInfo,
    ),
);

#[implements]
#[promote_1_2(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdEndRenderPass2KHR)]
pub struct PFN_vkCmdEndRenderPass2KHR(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo),
);

#[implements]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    #[promote_1_2(suffix = "KHR")]
    pub fn vkCreateRenderPass2KHR(
        device: VkDevice,
        pCreateInfo: *const VkRenderPassCreateInfo2KHR,
        pAllocator: *const VkAllocationCallbacks,
        pRenderPass: *mut VkRenderPass,
    ) -> VkResult;
    #[promote_1_2(suffix = "KHR")]
    pub fn vkCmdBeginRenderPass2KHR(
        commandBuffer: VkCommandBuffer,
        pRenderPassBegin: *const VkRenderPassBeginInfo,
        pSubpassBeginInfo: *const VkSubpassBeginInfoKHR,
    );
    #[promote_1_2(suffix = "KHR")]
    pub fn vkCmdNextSubpass2KHR(
        commandBuffer: VkCommandBuffer,
        pSubpassBeginInfo: *const VkSubpassBeginInfoKHR,
        pSubpassEndInfo: *const VkSubpassEndInfo,
    );
    #[promote_1_2(suffix = "KHR")]
    pub fn vkCmdEndRenderPass2KHR(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);
}
