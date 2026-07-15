//! v1.3 promoted elements

use crate::{parts::*, vk_ext_enum};

const VERSION: &str = "1_3";

const VK_KHR_MAINTENANCE_4: &Extension = &Extension::khr("maintenance4", 2, 414);
const VK_KHR_FORMAT_FEATURE_FLAGS2: &Extension = &Extension::khr("format_feature_flags2", 2, 361);

pub const ELEMENTS: &[Element] = &[
    Bitmask::extending(
        "PipelineCacheCreateFlagBits",
        "PIPELINE_CACHE_CREATE",
        &[Bitmask::entry("EXTERNALLY_SYNCHRONIZED", 0).promoted(VERSION)],
    )
    .into_element(),
    // VK_KHR_maintenace4
    VK_KHR_MAINTENANCE_4.header_constants().into_element(),
    Struct::typed(
        "PhysicalDeviceMaintenance4Features",
        "PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES",
        VK_KHR_MAINTENANCE_4.ext_enum(0) as _,
        StructUsage::Both,
        &[Struct::member("maintenance4", TY_VK_BOOL)],
    )
    .extensions(&[VK_KHR_MAINTENANCE_4])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceMaintenance4Properties",
        "PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES",
        VK_KHR_MAINTENANCE_4.ext_enum(1) as _,
        StructUsage::Sink,
        &[Struct::member("maxBufferSize", "VkDeviceSize")],
    )
    .extensions(&[VK_KHR_MAINTENANCE_4])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DeviceBufferMemoryRequirements",
        "DEVICE_BUFFER_MEMORY_REQUIREMENTS",
        VK_KHR_MAINTENANCE_4.ext_enum(2) as _,
        StructUsage::Source,
        &[Struct::member("pCreateInfo", "*const VkBufferCreateInfo")],
    )
    .extensions(&[VK_KHR_MAINTENANCE_4])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "DeviceImageMemoryRequirements",
        "DEVICE_IMAGE_MEMORY_REQUIREMENTS",
        VK_KHR_MAINTENANCE_4.ext_enum(3) as _,
        StructUsage::Source,
        &[
            Struct::member("pCreateInfo", "*const VkImageCreateInfo"),
            Struct::member("planeAspect", "VkImageAspectFlagBits"),
        ],
    )
    .extensions(&[VK_KHR_MAINTENANCE_4])
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "GetDeviceBufferMemoryRequirements",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkDeviceBufferMemoryRequirementsKHR"),
            ("pMemoryRequirements", "*mut VkMemoryRequirements2KHR"),
        ],
    )
    .extension(VK_KHR_MAINTENANCE_4)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "GetDeviceImageMemoryREquirements",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkDeviceImageMemoryRequirementsKHR"),
            ("pMemoryRequirements", "*mut VkMemoryRequirements2KHR"),
        ],
    )
    .extension(VK_KHR_MAINTENANCE_4)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "GetDeviceImageSparseMemoryRequirements",
        &[
            ("device", "VkDevice"),
            ("pInfo", "*const VkDeviceImageMemoryRequirementsKHR"),
            ("pSparseMemoryRequirementsCount", "*mut u32"),
            ("pSparseMemoryRequirements", "*mut VkSparseImageMemoryRequirements2KHR"),
        ],
    )
    .extension(VK_KHR_MAINTENANCE_4)
    .promoted(VERSION)
    .into_element(),
    // VK_KHR_format_feature_flags2
    VK_KHR_FORMAT_FEATURE_FLAGS2.header_constants().into_element(),
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
        .extension(VK_KHR_FORMAT_FEATURE_FLAGS2)
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "FormatProperties3",
            "FORMAT_PROPERTIES_3",
            VK_KHR_FORMAT_FEATURE_FLAGS2.ext_enum(0) as _,
            StructUsage::Sink,
            &[
                Struct::member("linearTilingFeatures", "VkFormatFeatureFlags2KHR"),
                Struct::member("optimalTilingFeatures", "VkFormatFeatureFlags2KHR"),
                Struct::member("bufferFeatures", "VkFormatFeatureFlags2KHR"),
            ],
        )
        .extensions(&[VK_KHR_FORMAT_FEATURE_FLAGS2])
        .promoted(VERSION),
    ),
];
