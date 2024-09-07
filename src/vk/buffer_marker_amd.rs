//! VK_AMD_buffer_marker extension

pub const VK_AMD_BUFFER_MARKER_SPEC_VERSION: usize = 1;
pub const VK_AMD_BUFFER_MARKER_EXTENSION_NAME: &str = "VK_AMD_buffer_marker";

use super::*;
use crate::PFN;

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdWriteBufferMarkerAMD)]
pub struct PFN_vkCmdWriteBufferMarkerAMD(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        pipelineStage: VkPipelineStageFlags,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        marker: u32,
    ),
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCmdWriteBufferMarkerAMD(
        commandBuffer: VkCommandBuffer,
        pipelineStage: VkPipelineStageFlags,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        marker: u32,
    );
}
