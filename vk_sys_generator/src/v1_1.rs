//! v1.2 promoted elements

use crate::{parts::*, vk_ext_enum};

const VERSION: &str = "1_1";
const VK_KHR_SAMPLER_YCBCR_CONVERSION: &Extension = &Extension::khr("sampler_ycbcr_conversion", 14, 157);
const VK_EXT_SHADER_SUBGROUP_VOTE: &Extension = &Extension::ext("shader_subgroup_vote", 1, 66);
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE: &Extension = &Extension::khr("descriptor_update_template", 1, 86);
pub const VK_KHR_DEVICE_GROUP: &Extension = &Extension::khr("device_group", 4, 61);

pub const ELEMENTS: &[Element] = &[
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_EXT_SHADER_SUBGROUP_VOTE)),
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(VK_KHR_SAMPLER_YCBCR_CONVERSION)),
    Object::new(
        "VkSamplerYcbcrConversionKHR",
        "SAMPLER_YCBCR_CONVERSION_KHR",
        vk_ext_enum(157, 0),
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .promoted(VERSION, "VkSamplerYcbcrConversion", "SAMPLER_YCBCR_CONVERSION")
    .into_element(),
    Bitmask::extending(
        "FormatFeatureFlagBits",
        "FORMAT_FEATURE",
        &[
            Bitmask::entry("MIDPOINT_CHROMA_SAMPLES", 17)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER", 18)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER", 19)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Bitmask::entry("SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT", 20)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Bitmask::entry(
                "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE",
                21,
            )
            .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
            .promoted(VERSION),
            Bitmask::entry("DISJOINT", 22)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Bitmask::entry("COSITED_CHROMA_SAMPLES", 23)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
        ],
    )
    .into_element(),
    Bitmask::extending(
        "ImageAspectFlagBits",
        "IMAGE_ASPECT",
        &[
            Bitmask::entry("PLANE_0", 4)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Bitmask::entry("PLANE_1", 5)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Bitmask::entry("PLANE_2", 6)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
        ],
    )
    .into_element(),
    Bitmask::extending(
        "ImageCreateFlagBits",
        "IMAGE_CREATE",
        &[Bitmask::entry("DISJOINT", 9)
            .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
            .promoted(VERSION)],
    )
    .into_element(),
    Enum::new(
        "SamplerYcbcrModelConversion",
        "SAMPLER_YCBCR_MODEL_CONVERSION",
        &[
            Enum::member("RGB_IDENTITY", 0)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Enum::member("YCBCR_IDENTITY", 1)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Enum::member("YCBCR_709", 2)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Enum::member("YCBCR_601", 3)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Enum::member("YCBCR_2020", 4)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
        ],
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .promoted(VERSION)
    .into_element(),
    Enum::new(
        "ChromaLocation",
        "CHROMA_LOCATION",
        &[
            Enum::member("COSITED_EVEN", 0)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Enum::member("MIDPOINT", 1)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
        ],
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .promoted(VERSION)
    .into_element(),
    Enum::new(
        "SamplerYcbcrRange",
        "SAMPLER_YCBCR_RANGE",
        &[
            Enum::member("ITU_FULL", 0)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
            Enum::member("ITU_NARROW", 1)
                .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
                .promoted(VERSION),
        ],
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .promoted(VERSION)
    .into_element(),
    Enum::extending(
        "DebugReportObjectType",
        "DEBUG_REPORT_OBJECT_TYPE",
        &[Enum::member("SAMPLER_YCBCR_CONVERSION", vk_ext_enum(157, 0) as _)
            .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
            .promoted(VERSION)],
    )
    .extension(crate::extensions::VK_EXT_DEBUG_REPORT)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceSamplerYcbcrConversionFeatures",
        "PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES",
        vk_ext_enum(157, 4) as _,
        StructUsage::Both,
        &[Struct::member("samplerYcbcrConversion", "VkBool32")],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BindImagePlaneMemoryInfo",
        "BIND_IMAGE_PLANE_MEMORY_INFO",
        vk_ext_enum(157, 2) as _,
        StructUsage::Source,
        &[Struct::member("planeAspect", "VkImageAspectFlags")],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImagePlaneMemoryRequirementsInfo",
        "IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO",
        vk_ext_enum(157, 2) as _,
        StructUsage::Source,
        &[Struct::member("planeAspect", "VkImageAspectFlagBits")],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SamplerYcbcrConversionCreateInfo",
        "SAMPLER_YCBCR_CONVERSION_CREATE_INFO",
        vk_ext_enum(157, 0) as _,
        StructUsage::Source,
        &[
            Struct::member("format", "VkFormat"),
            Struct::member("ycbcrModel", "VkSamplerYcbcrModelConversionKHR"),
            Struct::member("ycbcrRange", "VkSamplerYcbcrRangeKHR"),
            Struct::member("components", "VkComponentMapping"),
            Struct::member("xChromaOffset", "VkChromaLocationKHR"),
            Struct::member("yChromaOffset", "VkChromaLocationKHR"),
            Struct::member("chromaFilter", "VkFilter"),
            Struct::member("forceExplicitReconstruction", "VkBool32"),
        ],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SamplerYcbcrConversionImageFormatProperties",
        "SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES",
        vk_ext_enum(157, 5) as _,
        StructUsage::Sink,
        &[Struct::member("combinedImageSamplerDescriptorCount", "u32")],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "SamplerYcbcrConversionInfo",
        "SAMPLER_YCBCR_CONVERSION_INFO",
        vk_ext_enum(157, 1) as _,
        StructUsage::Source,
        &[Struct::member("conversion", "VkSamplerYcbcrConversion")],
    )
    .extensions(&[VK_KHR_SAMPLER_YCBCR_CONVERSION])
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "CreateSamplerYcbcrConversion",
        &[
            ("device", "VkDevice"),
            ("pCreateInfo", "VkSamplerYcbcrConversionCreateInfoKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
            ("pYcbcrConversion", "*mut VkSamplerYcbcrConversionKHR"),
        ],
    )
    .failable()
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .promoted(VERSION)
    .into_element(),
    Command::new(
        "DestroySamplerYcbcrConversion",
        &[
            ("device", "VkDevice"),
            ("ycbcrConversion", "VkSamplerYcbcrConversionKHR"),
            ("pAllocator", "*const VkAllocationCallbacks"),
        ],
    )
    .extension(VK_KHR_SAMPLER_YCBCR_CONVERSION)
    .promoted(VERSION)
    .into_element(),
];
