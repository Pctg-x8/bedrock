//! VK_AMD_draw_indirect_count extensions

pub const VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: usize = 1;
pub static VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &'static str = "VK_AMD_draw_indirect_count";

use super::*;
use crate::PFN;

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDrawIndirectCountAMD)]
pub struct PFN_vkCmdDrawIndirectCountAMD(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        countBuffer: VkBuffer,
        countBufferOffset: VkDeviceSize,
        maxDrawCount: u32,
        stride: u32,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDrawIndexedIndirectCountAMD)]
pub struct PFN_vkCmdDrawIndexedIndirectCountAMD(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        countBuffer: VkBuffer,
        countBufferOffset: VkDeviceSize,
        maxDrawCount: u32,
        stride: u32,
    ),
);
