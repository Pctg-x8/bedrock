//! VK_AMD_buffer_marker extension

pub const VK_AMD_BUFFER_MARKER_SPEC_VERSION: usize = 1;
pub const VK_AMD_BUFFER_MARKER_EXTENSION_NAME: &str = "VK_AMD_buffer_marker";

use super::*;
use crate::PFN;

vk_ext_command! {
    pub fn vkCmdWriteBufferMarkerAMD(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlags, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, marker: u32);
}
