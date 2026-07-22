//! v1.3 promoted elements

use crate::{parts::*, vk_ext_enum};

const VERSION: &str = "1_3";

const VK_KHR_MAINTENANCE_4: &Extension = &Extension::khr("maintenance4", 2, 414);
pub const VK_KHR_FORMAT_FEATURE_FLAGS_2: &Extension = &Extension::khr("format_feature_flags2", 2, 361);
pub const VK_KHR_COPY_COMMANDS_2: &Extension = &Extension::khr("copy_commands2", 1, 338);
pub const VK_KHR_DYNAMIC_RENDERING: &Extension = &Extension::khr("dynamic_rendering", 1, 45);

pub const ELEMENTS: &[Element] = &[
    Bitmask::extending(
        "PipelineCacheCreateFlagBits",
        "PIPELINE_CACHE_CREATE",
        &[Bitmask::entry("EXTERNALLY_SYNCHRONIZED", 0).version_since(VERSION)],
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
    VK_KHR_FORMAT_FEATURE_FLAGS_2.header_constants().into_element(),
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
        .extension(VK_KHR_FORMAT_FEATURE_FLAGS_2)
        .promoted(VERSION),
    ),
    Element::Struct(
        Struct::typed(
            "FormatProperties3",
            "FORMAT_PROPERTIES_3",
            VK_KHR_FORMAT_FEATURE_FLAGS_2.ext_enum(0) as _,
            StructUsage::Sink,
            &[
                Struct::member("linearTilingFeatures", "VkFormatFeatureFlags2KHR"),
                Struct::member("optimalTilingFeatures", "VkFormatFeatureFlags2KHR"),
                Struct::member("bufferFeatures", "VkFormatFeatureFlags2KHR"),
            ],
        )
        .extensions(&[VK_KHR_FORMAT_FEATURE_FLAGS_2])
        .promoted(VERSION),
    ),
    // VK_KHR_copy_commands2
    VK_KHR_COPY_COMMANDS_2.header_constants().into_element(),
    Struct::typed(
        "CopyBufferInfo2",
        "COPY_BUFFER_INFO_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(0) as _,
        StructUsage::Source,
        &[
            Struct::member("srcBuffer", "VkBuffer"),
            Struct::member("dstBuffer", "VkBuffer"),
            Struct::member("regionCount", "u32"),
            Struct::member("pRegions", "*const VkBufferCopy2KHR"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "CopyImageInfo2",
        "COPY_IMAGE_INFO_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(1) as _,
        StructUsage::Source,
        &[
            Struct::member("srcImage", "VkImage"),
            Struct::member("srcImageLayout", "VkImageLayout"),
            Struct::member("dstImage", "VkImage"),
            Struct::member("dstImageLayout", "VkImageLayout"),
            Struct::member("regionCount", "u32"),
            Struct::member("pRegions", "*const VkImageCopy2KHR"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "CopyBufferToImageInfo2",
        "COPY_BUFFER_TO_IMAGE_INFO_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(2) as _,
        StructUsage::Source,
        &[
            Struct::member("srcBuffer", "VkBuffer"),
            Struct::member("dstImage", "VkImage"),
            Struct::member("dstImageLayout", "VkImageLayout"),
            Struct::member("regionCount", "u32"),
            Struct::member("pRegions", "*const VkBufferImageCopy2KHR"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "CopyImageToBufferInfo2",
        "COPY_IMAGE_TO_BUFFER_INFO_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(3) as _,
        StructUsage::Source,
        &[
            Struct::member("srcImage", "VkImage"),
            Struct::member("srcImageLayout", "VkImageLayout"),
            Struct::member("dstBuffer", "VkBuffer"),
            Struct::member("regionCount", "u32"),
            Struct::member("pRegions", "*const VkBufferImageCopy2KHR"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BlitImageInfo2",
        "BLIT_IMAGE_INFO_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(4) as _,
        StructUsage::Source,
        &[
            Struct::member("srcImage", "VkImage"),
            Struct::member("srcImageLayout", "VkImageLayout"),
            Struct::member("dstImage", "VkImage"),
            Struct::member("dstImageLayout", "VkImageLayout"),
            Struct::member("regionCount", "u32"),
            Struct::member("pRegions", "*const VkImageBlit2KHR"),
            Struct::member("filter", "VkFilter"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ResolveImageInfo2",
        "RESOLVE_IMAGE_INFO_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(5) as _,
        StructUsage::Source,
        &[
            Struct::member("srcImage", "VkImage"),
            Struct::member("srcImageLayout", "VkImageLayout"),
            Struct::member("dstImage", "VkImage"),
            Struct::member("dstImageLayout", "VkImageLayout"),
            Struct::member("regionCount", "u32"),
            Struct::member("pRegions", "*const VkImageResolve2KHR"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BufferCopy2",
        "BUFFER_COPY_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(6) as _,
        StructUsage::Source,
        &[
            Struct::member("srcOffset", "VkDeviceSize"),
            Struct::member("dstOffset", "VkDeviceSize"),
            Struct::member("size", "VkDeviceSize"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImageCopy2",
        "IMAGE_COPY_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(7) as _,
        StructUsage::Source,
        &[
            Struct::member("srcSubresource", "VkImageSubresourceLayers"),
            Struct::member("srcOffset", "VkOffset3D"),
            Struct::member("dstSubresource", "VkImageSubresourceLayers"),
            Struct::member("dstOffset", "VkOffset3D"),
            Struct::member("extent", "VkExtent3D"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImageBlit2",
        "IMAGE_BLIT_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(8) as _,
        StructUsage::Source,
        &[
            Struct::member("srcSubresources", "VkImageSubresourceLayers"),
            Struct::member("srcOffset", "[VkOffset3D; 2]"),
            Struct::member("dstSubresources", "VkImageSubresourceLayers"),
            Struct::member("dstOffset", "[VkOffset3D; 2]"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "BufferImageCopy2",
        "BUFFER_IMAGE_COPY_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(9) as _,
        StructUsage::Source,
        &[
            Struct::member("bufferOffset", "VkDeviceSize"),
            Struct::member("bufferRowLength", "u32"),
            Struct::member("bufferImageHeight", "u32"),
            Struct::member("imageSubresource", "VkImageSubresourceLayers"),
            Struct::member("imageOffset", "VkOffset3D"),
            Struct::member("imageExtent", "VkExtent3D"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "ImageResolve2",
        "IMAGE_RESOLVE_2",
        VK_KHR_COPY_COMMANDS_2.ext_enum(10) as _,
        StructUsage::Source,
        &[
            Struct::member("srcSubresource", "VkImageSubresourceLayers"),
            Struct::member("srcOffset", "VkOffset3D"),
            Struct::member("dstSubresource", "VkImageSubresourceLayers"),
            Struct::member("dstOffset", "VkOffset3D"),
            Struct::member("extent", "VkExtent3D"),
        ],
    )
    .extensions(&[VK_KHR_COPY_COMMANDS_2])
    .promoted(VERSION)
    .into_element(),
    Command::inst("CopyBuffer2", &[("pCopyBufferInfo", "*const VkCopyBufferInfo2KHR")])
        .extension(VK_KHR_COPY_COMMANDS_2)
        .promoted(VERSION)
        .into_element(),
    Command::inst("CopyImage2", &[("pCopyImageInfo", "*const VkCopyImageInfo2KHR")])
        .extension(VK_KHR_COPY_COMMANDS_2)
        .promoted(VERSION)
        .into_element(),
    Command::inst(
        "CopyBufferToImage2",
        &[("pCopyBufferToImageInfo", "*const VkCopyBufferToImageInfo2KHR")],
    )
    .extension(VK_KHR_COPY_COMMANDS_2)
    .promoted(VERSION)
    .into_element(),
    Command::inst(
        "CopyImageToBuffer2",
        &[("pCopyImageToBufferInfo", "*const VkCopyImageToBufferInfo2KHR")],
    )
    .extension(VK_KHR_COPY_COMMANDS_2)
    .promoted(VERSION)
    .into_element(),
    Command::inst("BlitImage2", &[("pBlitImageInfo", "*const VkBlitImageInfo2KHR")])
        .extension(VK_KHR_COPY_COMMANDS_2)
        .promoted(VERSION)
        .into_element(),
    Command::inst(
        "ResolveImage2",
        &[("pResolveImageInfo", "*const VkResolveImageInfo2KHR")],
    )
    .extension(VK_KHR_COPY_COMMANDS_2)
    .promoted(VERSION)
    .into_element(),
    // VK_KHR_dynamic_rendering
    VK_KHR_DYNAMIC_RENDERING.header_constants().into_element(),
    Bitmask::new(
        "RenderingFlags",
        "RenderingFlagBits",
        "RENDERING",
        &[
            Bitmask::entry("CONTENTS_SECONDARY_COMMAND_BUFFERS", 0)
                .extension(VK_KHR_DYNAMIC_RENDERING)
                .promoted(VERSION),
            Bitmask::entry("SUSPENDING", 1)
                .extension(VK_KHR_DYNAMIC_RENDERING)
                .promoted(VERSION),
            Bitmask::entry("RESUMING", 2)
                .extension(VK_KHR_DYNAMIC_RENDERING)
                .promoted(VERSION),
        ],
    )
    .extension(VK_KHR_DYNAMIC_RENDERING)
    .promoted(VERSION)
    .into_element(),
    Enum::extending(
        "AttachmentStoreOp",
        "ATTACHMENT_STORE_OP",
        &[Enum::member("NONE", vk_ext_enum(302, 0) as _)
            .extension(VK_KHR_DYNAMIC_RENDERING)
            .promoted(VERSION)],
    )
    .into_element(),
    Struct::typed(
        "RenderingInfo",
        "RENDERING_INFO",
        VK_KHR_DYNAMIC_RENDERING.ext_enum(0) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkRenderingFlagsKHR"),
            Struct::member("renderArea", "VkRect2D"),
            Struct::member("layoutCount", "u32"),
            Struct::member("viewMask", "u32"),
            Struct::member("colorAttachmentCount", "u32"),
            Struct::member("pColorAttachments", "*const VkRenderingAttachmentInfoKHR"),
            Struct::member("pDepthAttachment", "*const VkRenderingAttachmentInfoKHR"),
            Struct::member("pStencilAttachment", "*const VkRenderingAttachmentInfoKHR"),
        ],
    )
    .extensions(&[VK_KHR_DYNAMIC_RENDERING])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "RenderingAttachmentInfo",
        "RENDERING_ATTACHMENT_INFO",
        VK_KHR_DYNAMIC_RENDERING.ext_enum(1) as _,
        StructUsage::Source,
        &[
            Struct::member("imageView", "VkImageView"),
            Struct::member("imageLayout", "VkImageLayout"),
            Struct::member("resolveMode", "VkResolveModeFlagBitsKHR"),
            Struct::member("resolveImageView", "VkImageView"),
            Struct::member("resolveImageLayout", "VkImageLayout"),
            Struct::member("loadOp", "VkAttachmentLoadOp"),
            Struct::member("storeOp", "VkAttachmentStoreOp"),
            Struct::member("clearValue", "VkClearValue"),
        ],
    )
    .non_debuggable()
    .extensions(&[VK_KHR_DYNAMIC_RENDERING])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PipelineRenderingCreateInfo",
        "PIPELINE_RENDERING_CREATE_INFO",
        VK_KHR_DYNAMIC_RENDERING.ext_enum(2) as _,
        StructUsage::Source,
        &[
            Struct::member("viewMask", "u32"),
            Struct::member("colorAttachmentCount", "u32"),
            Struct::member("pColorAttachmentFormats", "*const VkFormat"),
            Struct::member("depthAttachmentFormat", "VkFormat"),
            Struct::member("stencilAttachmentFormat", "VkFormat"),
        ],
    )
    .extensions(&[VK_KHR_DYNAMIC_RENDERING])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "PhysicalDeviceDynamicRenderingFeatures",
        "PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES",
        VK_KHR_DYNAMIC_RENDERING.ext_enum(3) as _,
        StructUsage::Both,
        &[Struct::member("dynamicRendering", TY_VK_BOOL)],
    )
    .extensions(&[VK_KHR_DYNAMIC_RENDERING])
    .promoted(VERSION)
    .into_element(),
    Struct::typed(
        "CommandBufferInheritanceRenderingInfo",
        "COMMAND_BUFFER_INHERITANCE_RENDERING_INFO",
        VK_KHR_DYNAMIC_RENDERING.ext_enum(4) as _,
        StructUsage::Source,
        &[
            Struct::member("flags", "VkRenderingFlagsKHR"),
            Struct::member("viewMask", "u32"),
            Struct::member("colorAttachmentCount", "u32"),
            Struct::member("pColorAttachmentFormats", "*const VkFormat"),
            Struct::member("denpthAttachmentFormat", "VkFormat"),
            Struct::member("stencilAttachmentFormat", "VkFormat"),
            Struct::member("rasterizationSamples", "VkSampleCountFlagBits"),
        ],
    )
    .extensions(&[VK_KHR_DYNAMIC_RENDERING])
    .promoted(VERSION)
    .into_element(),
    Command::inst("BeginRendering", &[("pRenderingInfo", "*const VkRenderingInfoKHR")])
        .extension(VK_KHR_DYNAMIC_RENDERING)
        .promoted(VERSION)
        .into_element(),
    Command::inst("EndRendering", &[])
        .extension(VK_KHR_DYNAMIC_RENDERING)
        .promoted(VERSION)
        .into_element(),
];
