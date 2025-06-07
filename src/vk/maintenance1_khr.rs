//! VK_KHR_maintenance1 extensions

pub const VK_KHR_MAINTENANCE1_SPEC_VERSION: usize = 2;
pub static VK_KHR_MAINTENANCE1_EXTENSION_NAME: &'static str = "VK_KHR_maintenance1";

use derives::{promote_1_1, vk_ext_command};

use super::*;

#[promote_1_1]
pub const VK_ERROR_OUT_OF_POOL_MEMORY_KHR: VkResult = VkResult::ext_err_value(70, 0);

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

vk_ext_command! {
    pub fn vkTrimCommandPoolKHR(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);
    suffix = "KHR";
    promote = "1.1";
}
