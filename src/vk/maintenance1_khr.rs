//! VK_KHR_maintenance1 extensions

pub const VK_KHR_MAINTENANCE1_SPEC_VERSION: usize = 2;
pub static VK_KHR_MAINTENANCE1_EXTENSION_NAME: &'static str = "VK_KHR_maintenance1";

use derives::promote_1_1;

use super::*;

vk_bitmask! {
    extending enum VkFormatFeatureFlagBits {
        #[promote_1_1]
        pub VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR: 14,
        #[promote_1_1]
        pub VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR: 15
    }
}

vk_bitmask! {
    extending enum VkImageCreateFlagBits {
        #[promote_1_1]
        pub VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR: 5
    }
}

#[promote_1_1(suffix = "KHR")]
pub type VkCommandPoolTrimFlagsKHR = VkFlags;
#[promote_1_1(suffix = "KHR")]
pub type PFN_vkTrimCommandPoolKHR =
    extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    #[promote_1_1(suffix = "KHR")]
    pub fn vkTrimCommandPoolKHR(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);
}
