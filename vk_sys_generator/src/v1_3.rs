//! v1.3 promoted elements

use crate::{parts::*, vk_ext_enum};

const VERSION: &str = "1_3";

const VK_KHR_FORMAT_FEATURE_FLAGS2: Extension = Extension::new("KHR", "format_feature_flags2", 2);

pub const ELEMENTS: &[Element] = &[
    // VK_KHR_format_feature_flags2
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(&VK_KHR_FORMAT_FEATURE_FLAGS2)),
    Element::Bitmask(
        Bitmask::new(
            "FormatFeatureFlags2",
            "FormatFeatureFlagBits2",
            "FORMAT_FEATURE_2",
            &[
                Bitmask::entry("SAMPLED_IMAGE", 0),
                Bitmask::entry("STORAGE_IMAGE", 1),
                Bitmask::entry("STORAGE_IMAGE_ATOMIC", 2),
                Bitmask::entry("UNIFORM_TEXEL_BUFFER", 3),
                Bitmask::entry("STORAGE_TEXEL_BUFFER", 4),
                Bitmask::entry("STORAGE_TEXEL_BUFFER_ATOMIC", 5),
                Bitmask::entry("VERTEX_BUFFER", 6),
                Bitmask::entry("COLOR_ATTACHMENT", 7),
                Bitmask::entry("COLOR_ATTACHMENT_BLEND", 8),
                Bitmask::entry("DEPTH_STENCIL_ATTACHMENT", 9),
                Bitmask::entry("BLIT_SRC", 10),
                Bitmask::entry("BLIT_DST", 11),
                Bitmask::entry("SAMPLED_IMAGE_FILTER_LINEAR", 12),
                Bitmask::entry("TRANSFER_SRC", 14),
                Bitmask::entry("TRANSFER_DST", 15),
                Bitmask::entry("SAMPLED_IMAGE_FILTER_MINMAX", 16),
                Bitmask::entry("MIDPOINT_CHROMA_SAMPLES", 17),
                Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER", 18),
                Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER", 19),
                Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT", 20),
                Bitmask::entry(
                    "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE",
                    21,
                ),
                Bitmask::entry("DISJOINT", 22),
                Bitmask::entry("COSITED_CHROMA_SAMPLES", 23),
                Bitmask::entry("STORAGE_READ_WITHOUT_FORMAT", 35),
                Bitmask::entry("STORAGE_WRITE_WITHOUT_FORMAT", 32),
                Bitmask::entry("SAMPLED_IMAGE_DEPTH_COMPARISON", 33),
                Bitmask::entry("SAMPLED_IMAGE_FILTER_CUBIC", 13),
            ],
        )
        .long()
        .extension2(&VK_KHR_FORMAT_FEATURE_FLAGS2)
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "FormatProperties3",
            "FORMAT_PROPERTIES_3",
            vk_ext_enum(361, 0) as _,
            StructUsage::Sink,
            &[
                Struct::member("linearTilingFeatures", "VkFormatFeatureFlags2KHR"),
                Struct::member("optimalTilingFeatures", "VkFormatFeatureFlags2KHR"),
                Struct::member("bufferFeatures", "VkFormatFeatureFlags2KHR"),
            ],
        )
        .extensions2(&[&VK_KHR_FORMAT_FEATURE_FLAGS2])
        .promoted(VERSION),
    ),
];
