//! VK_KHR_maintenance1 extensions

pub const VK_KHR_MAINTENANCE1_SPEC_VERSION: usize = 2;
pub static VK_KHR_MAINTENANCE1_EXTENSION_NAME: &'static str = "VK_KHR_maintenance1";

use super::*;

pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR: VkFormatFeatureFlagBits = 1 << 14;
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR: VkFormatFeatureFlagBits = 1 << 15;

pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits = 1 << 5;

pub type VkCommandPoolTrimFlagsKHR = VkFlags;
pub type PFN_vkTrimCommandPoolKHR =
    extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkTrimCommandPoolKHR(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);
}

cfg_if! {
    if #[cfg(feature = "Allow1_1APIs")] {
        pub const VK_ERROR_OUT_OF_POOL_MEMORY: VkResult = VK_ERROR_OUT_OF_POOL_MEMORY_KHR;
        pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR;
        pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT: VkFormatFeatureFlagBits = VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR;
        pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT: VkImageCreateFlagBits = VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR;
    }
}
