//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_format_feature_flags2.html

pub const VK_KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION: usize = 2;
pub const VK_KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME: &'static str = "VK_KHR_format_feature_flags2";

use super::*;
use derives::promote_1_3;

#[promote_1_3]
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3_KHR: VkStructureType = ext_enum_value(361, 0) as _;

#[promote_1_3(suffix = "KHR")]
pub type VkFormatFeatureFlags2KHR = VkFlags64;
vk_bitmask! {
    #[promote_1_3(suffix = "KHR")]
    pub enum64 VkFormatFeatureFlagBits2KHR {
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR: 0,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR: 1,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR: 2,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR: 3,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR: 4,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR: 5,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR: 6,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR: 7,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR: 8,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR: 9,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR: 10,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR: 11,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR: 12,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR: 14,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR: 15,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR: 16,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR: 17,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: 18,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: 19,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: 20,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: 21,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR: 22,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR: 23,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR: 35,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR: 32,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR: 33,
        #[promote_1_3]
        pub VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_KHR: 13,
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkFormatProperties3KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub linearTilingFeatures: VkFormatFeatureFlags2KHR,
    pub optimalTilingFeatures: VkFormatFeatureFlags2KHR,
    pub bufferFeatures: VkFormatFeatureFlags2KHR,
}
