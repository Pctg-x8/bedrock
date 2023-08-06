//! VK_EXT_external_memory_dma_buf extension

pub const VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION: usize = 1;
pub const VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME: &str = "VK_EXT_external_memory_dma_buf";

use super::*;

vk_bitmask! {
    extending enum VkExternalMemoryHandleTypeFlagBitsKHR {
        pub VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT: 9
    }
}
