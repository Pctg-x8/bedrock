//! VK_IMG_filter_cubic extensions

pub const VK_IMG_FILTER_CUBIC_SPEC_VERSION: usize = 1;
pub static VK_IMG_FILTER_CUBIC_EXTENSION_NAME: &'static str = "VK_IMG_filter_cubic";

use super::*;

pub const VK_FILTER_CUBIC_IMG: VkFilter = ext_enum_value(16, 0) as _;

vk_bitmask! {
    extending enum VkFormatFeatureFlagBits {
        pub VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: 13
    }
}
