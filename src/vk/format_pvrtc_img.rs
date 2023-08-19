//! VK_IMG_format_pvrtc extensions

pub const VK_IMG_FORMAT_PVRTC_SPEC_VERSION: usize = 1;
pub static VK_IMG_FORMAT_PVRTC_EXTENSION_NAME: &'static str = "VK_IMG_format_pvrtc";

use super::*;

pub const VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG: VkFormat = ext_enum_value(55, 0) as _;
pub const VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG: VkFormat = ext_enum_value(55, 1) as _;
pub const VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG: VkFormat = ext_enum_value(55, 2) as _;
pub const VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG: VkFormat = ext_enum_value(55, 3) as _;
pub const VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG: VkFormat = ext_enum_value(55, 4) as _;
pub const VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG: VkFormat = ext_enum_value(55, 5) as _;
pub const VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG: VkFormat = ext_enum_value(55, 6) as _;
pub const VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG: VkFormat = ext_enum_value(55, 7) as _;
