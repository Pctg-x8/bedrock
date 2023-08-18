//! VK_NV_clip_space_w_scaling extensions

pub const VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: usize = 1;
pub static VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &'static str = "VK_NV_clip_space_w_scaling";

use super::*;
use crate::PFN;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkViewportWScalingNV {
    pub xcoeff: c_float,
    pub ycoeff: c_float,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub viewportWScalingEnable: VkBool32,
    pub viewportCount: u32,
    pub pViewportWScalings: *const VkViewportWScalingNV,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetViewportWScalingNV)]
pub struct PFN_vkCmdSetViewportWScalingNV(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        firstViewprt: u32,
        viewportCount: u32,
        pViewportWScalings: *const VkViewportWScalingNV,
    ),
);
