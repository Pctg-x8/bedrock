use crate::{parts::*, vk_ext_enum};

const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER: Extension = Extension::new("EXT", "image_drm_format_modifier", 1);
const VK_EXT_METAL_OBJECTS: Extension = Extension::new("EXT", "metal_objects", 2);
const VK_MVK_MACOS_SURFACE: Extension = Extension::new("MVK", "macos_surface", 2);
const VK_MVK_IOS_SURFACE: Extension = Extension::new("MVK", "ios_surface", 2);

pub const ELEMENTS: &[Element] = &[
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_acquire_drm_display", 1)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_acquire_xlib_display", 1)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_blend_operation_advanced", 2)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_validation_cache", 1)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_validation_flags", 1)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_AMD_buffer_marker", 1)),
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_NV_acquire_winrt_display", 1)),
    Element::Bitmask(
        Bitmask::new(
            "ValidationCacheCreateFlags",
            "ValidationCacheCreateFlagBits",
            "VALIDATION_CACHE_CREATE_FLAGS",
            &[],
        )
        .extension("EXT", "validation_cache"),
    ),
    Element::Object(
        Object::new("VkValidationCacheEXT", "VALIDATION_CACHE_EXT", vk_ext_enum(161, 0) as _)
            .extension("VK_EXT_validation_cache"),
    ),
    Element::Enum(Enum::extending(
        "BlendOp",
        "BLEND_OP",
        &[
            Enum::member("ZERO", vk_ext_enum(149, 0) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("SRC", vk_ext_enum(149, 1) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("DST", vk_ext_enum(149, 2) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("SRC_OVER", vk_ext_enum(149, 3) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("DST_OVER", vk_ext_enum(149, 4) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("SRC_IN", vk_ext_enum(149, 5) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("DST_IN", vk_ext_enum(149, 6) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("SRC_OUT", vk_ext_enum(149, 7) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("DST_OUT", vk_ext_enum(149, 8) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("SRC_ATOP", vk_ext_enum(149, 9) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("DST_ATOP", vk_ext_enum(149, 10) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("XOR", vk_ext_enum(149, 11) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("MULTIPLY", vk_ext_enum(149, 12) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("SCREEN", vk_ext_enum(149, 13) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("OVERLAY", vk_ext_enum(149, 14) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("DARKEN", vk_ext_enum(149, 15) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("LIGHTEN", vk_ext_enum(149, 16) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("COLORDODGE", vk_ext_enum(149, 17) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("COLORBURN", vk_ext_enum(149, 18) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("HARDLIGHT", vk_ext_enum(149, 19) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("SOFTLIGHT", vk_ext_enum(149, 20) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("DIFFERENCE", vk_ext_enum(149, 21) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("EXCLUSION", vk_ext_enum(149, 22) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("INVERT", vk_ext_enum(149, 23) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("INVERT_RGB", vk_ext_enum(149, 24) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("LINEARDODGE", vk_ext_enum(149, 25) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("LINEARBURN", vk_ext_enum(149, 26) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("VIVIDLIGHT", vk_ext_enum(149, 27) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("LINEARLIGHT", vk_ext_enum(149, 28) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("PINLIGHT", vk_ext_enum(149, 29) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("HARDMIX", vk_ext_enum(149, 30) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("HSL_HUE", vk_ext_enum(149, 31) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("HSL_SATURATION", vk_ext_enum(149, 32) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("HSL_COLOR", vk_ext_enum(149, 33) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("HSL_LUMINOSITY", vk_ext_enum(149, 34) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("PLUS", vk_ext_enum(149, 35) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("PLUS_CLAMPED", vk_ext_enum(149, 36) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("PLUS_CLAMPED_ALHPA", vk_ext_enum(149, 37) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("PLUS_DARKER", vk_ext_enum(149, 38) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("MINUS", vk_ext_enum(149, 39) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("MINUS_CLAMPED", vk_ext_enum(149, 40) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("CONTRAST", vk_ext_enum(149, 41) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("INVERT_OVG", vk_ext_enum(149, 42) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("RED", vk_ext_enum(149, 43) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("GREEN", vk_ext_enum(149, 44) as _).extension("EXT", "blend_operation_advanced"),
            Enum::member("BLUE", vk_ext_enum(149, 45) as _).extension("EXT", "blend_operation_advanced"),
        ],
    )),
    Element::Enum(
        Enum::new(
            "BlendOverlap",
            "BLEND_OVERLAP",
            &[
                Enum::member("UNCORRELATED", 0).extension("EXT", "blend_operation_advanced"),
                Enum::member("DISJOINT", 1).extension("EXT", "blend_operation_advanced"),
                Enum::member("CONJOINT", 2).extension("EXT", "blend_operation_advanced"),
            ],
        )
        .extension("EXT", "blend_operation_advanced"),
    ),
    Element::Enum(
        Enum::new(
            "ValidationCacheHeaderVersion",
            "VALIDATION_CACHE_HEADER_VERSION",
            &[Enum::member("ONE", 1).extension("EXT", "validation_cache")],
        )
        .extension("EXT", "validation_cache"),
    ),
    Element::Enum(
        Enum::new(
            "ValidationCheck",
            "VALIDATION_CHECK",
            &[
                Enum::member("ALL", 0).extension("EXT", "validation_flags"),
                Enum::member("SHADERS", 1).extension("EXT", "validation_flags"),
            ],
        )
        .extension("EXT", "validation_flags"),
    ),
    Element::Struct(
        Struct::new(
            "PhysicalDeviceBlendOperationAdvanccedFeatures",
            &[Struct::member("advancedBlendCoherentOperations", "VkBool32")],
        )
        .stype(
            "PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES",
            vk_ext_enum(149, 0) as _,
            StructUsage::Both,
        )
        .extensions(&[("EXT", "blend_operation_advanced")]),
    ),
    Element::Struct(
        Struct::new(
            "PhysicalDeviceBlendOperationAdvancedProperties",
            &[
                Struct::member("advancedBlendMaxColorAttachments", "u32"),
                Struct::member("advancedBlendIndependentBlend", "VkBool32"),
                Struct::member("advancedBlendNonPremultipliedSrcColor", "VkBool32"),
                Struct::member("advancedBlendNonPremultipliedDstColor", "VkBool32"),
                Struct::member("advancedBlendCorrelatedOverlap", "VkBool32"),
                Struct::member("advancedBlendAllOperations", "VkBool32"),
            ],
        )
        .stype(
            "PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES",
            vk_ext_enum(149, 1) as _,
            StructUsage::Sink,
        )
        .extensions(&[("EXT", "blend_operation_advanced")]),
    ),
    Element::Struct(
        Struct::new(
            "PipelineColorBlendAdvancedStateCreateInfo",
            &[
                Struct::member("srcPremultiplied", "VkBool32"),
                Struct::member("dstPremultiplied", "VkBool32"),
                Struct::member("blendOverlap", "VkBlendOverlapEXT"),
            ],
        )
        .stype(
            "PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO",
            vk_ext_enum(149, 2) as _,
            StructUsage::Source,
        )
        .extensions(&[("EXT", "blend_operation_advanced")]),
    ),
    Element::Struct(
        Struct::typed(
            "ShaderModuleValidationcacheCreateInfo",
            "SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO",
            vk_ext_enum(161, 1) as _,
            StructUsage::Source,
            &[Struct::member("validationCache", "VkValidationCacheEXT")],
        )
        .extensions(&[("EXT", "validation_cache")]),
    ),
    Element::Struct(
        Struct::typed(
            "ValidationCacheCreateInfo",
            "VALIDATION_CACHE_CREATE_INFO",
            vk_ext_enum(161, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("flags", "VkValidationCacheCreateFlagsEXT"),
                Struct::member("initialDataSize", "usize"),
                Struct::member("pInitialData", "*const core::ffi::c_void"),
            ],
        )
        .extensions(&[("EXT", "validation_cache")]),
    ),
    Element::Struct(
        Struct::typed(
            "ValidationFlags",
            "VALIDATION_FLAGS",
            vk_ext_enum(62, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("disabledValidationCheckCount", "u32"),
                Struct::member("pDisabledValidationChecks", "*mut VkValidationCheckEXT"),
            ],
        )
        .extensions(&[("EXT", "validation_flags")]),
    ),
    Element::Command(
        Command::new(
            "AcquireDrmDisplay",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("drmFd", "i32"),
                ("display", "VkDisplayKHR"),
            ],
        )
        .failable()
        .extension("EXT", "acquire_drm_display"),
    ),
    Element::Command(
        Command::new(
            "AcquireWinrtDisplay",
            &[("physicalDevice", "VkPhysicalDevice"), ("display", "VkDisplayKHR")],
        )
        .failable()
        .extension("NV", "acquire_winrt_display"),
    ),
    Element::Command(
        Command::new(
            "AcquireXlibDisplay",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("dpy", "*mut x11::xlib::Display"),
                ("display", "VkDisplayKHR"),
            ],
        )
        .failable()
        .extension("EXT", "acquire_xlib_display"),
    ),
    Element::Command(
        Command::new(
            "CreateValidationCache",
            &[
                ("device", "VkDevice"),
                ("pCreateInfo", "*const VkValidationCacheCreateInfoEXT"),
                ("pAllocator", "*const VkAllocationCallbacks"),
                ("pValidationCache", "*mut VkValidationCacheEXT"),
            ],
        )
        .failable()
        .extension("EXT", "validation_cache"),
    ),
    Element::Command(
        Command::new(
            "DestroyValidationCache",
            &[
                ("device", "VkDevice"),
                ("validationCache", "VkValidationCacheEXT"),
                ("pAllocator", "*const VkAllocationCallbacks"),
            ],
        )
        .extension("EXT", "validation_cache"),
    ),
    Element::Command(
        Command::new(
            "GetDrmDisplay",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("drmFd", "i32"),
                ("connectorId", "u32"),
                ("pDisplay", "*mut VkDisplayKHR"),
            ],
        )
        .failable()
        .extension("EXT", "acquire_drm_display"),
    ),
    Element::Command(
        Command::new(
            "GetRandROutputDisplay",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("dpy", "*mut x11::xlib::Display"),
                ("rrOutput", "x11::xrandr::RROutput"),
                ("pDisplay", "*mut VkDisplayKHR"),
            ],
        )
        .failable()
        .extension("EXT", "acquire_xlib_display"),
    ),
    Element::Command(
        Command::new(
            "GetValidationCacheData",
            &[
                ("device", "VkDevice"),
                ("validationCache", "VkValidationCacheEXT"),
                ("pDataSize", "*mut usize"),
                ("pData", "*mut core::ffi::c_void"),
            ],
        )
        .failable()
        .extension("EXT", "validation_cache"),
    ),
    Element::Command(
        Command::new(
            "GetWinrtDisplay",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("deviceRelativeId", "u32"),
                ("pDisplay", "*mut VkDisplayKHR"),
            ],
        )
        .failable()
        .extension("NV", "acquire_winrt_display"),
    ),
    Element::Command(
        Command::new(
            "MergeValidationCaches",
            &[
                ("device", "VkDevice"),
                ("dstCache", "VkValidationCacheEXT"),
                ("srcCacheCount", "u32"),
                ("pSrcCaches", "*const VkValidationCacheEXT"),
            ],
        )
        .failable()
        .extension("EXT", "validation_cache"),
    ),
    Element::Command(
        Command::inst(
            "WriteBufferMarker",
            &[
                ("pipelineStage", "VkPipelineStageFlags"),
                ("dstBuffer", "VkBuffer"),
                ("dstOffset", "VkDeviceSize"),
                ("marker", "u32"),
            ],
        )
        .extension("AMD", "buffer_marker"),
    ),
    // VK_EXT_layer_settings: 497
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_layer_settings", 2)),
    Element::Enum(
        Enum::new(
            "LayerSettingType",
            "LAYER_SETTING_TYPE",
            &[
                Enum::member("BOOL32", 0).extension("EXT", "layer_settings"),
                Enum::member("INT32", 1).extension("EXT", "layer_settings"),
                Enum::member("INT64", 2).extension("EXT", "layer_settings"),
                Enum::member("UINT32", 3).extension("EXT", "layer_settings"),
                Enum::member("UINT64", 4).extension("EXT", "layer_settings"),
                Enum::member("FLOAT32", 5).extension("EXT", "layer_settings"),
                Enum::member("FLOAT64", 6).extension("EXT", "layer_settings"),
                Enum::member("STRING", 7).extension("EXT", "layer_settings"),
            ],
        )
        .extension("EXT", "layer_settings"),
    ),
    Element::Struct(
        Struct::new(
            "LayerSetting",
            &[
                Struct::member("pLayerName", "*const core::ffi::c_char"),
                Struct::member("pSettingName", "*const core::ffi::c_char"),
                Struct::member("r#type", "VkLayerSettingTypeEXT"),
                Struct::member("valueCount", "u32"),
                Struct::member("pValues", "*const core::ffi::c_void"),
            ],
        )
        .extensions(&[("EXT", "layer_settings")]),
    ),
    Element::Struct(
        Struct::typed(
            "LayerSettingsCreateInfo",
            "LAYER_SETTINGS_CREATE_INFO",
            vk_ext_enum(497, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("settingCount", "u32"),
                Struct::member("pSettings", "*const VkLayerSettingEXT"),
            ],
        )
        .extensions(&[("EXT", "layer_settings")]),
    ),
    // VK_EXT_descriptor_buffer
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_descriptor_buffer", 1)),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceDescriptorBufferProperties",
            "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES",
            vk_ext_enum(317, 0) as _,
            StructUsage::Sink,
            &[
                Struct::member("combinedImageSamplerDescriptorSingleArray", "VkBool32"),
                Struct::member("bufferlessPushDescriptors", "VkBool32"),
                Struct::member("allowSamplerImageViewPostSubmitCreation", "VkBool32"),
                Struct::member("descriptorBufferOffsetAlignment", "VkDeviceSize"),
                Struct::member("maxDescriptorBufferBindings", "u32"),
                Struct::member("maxResourceDescriptorBufferBindings", "u32"),
                Struct::member("maxSamplerDescriptorBufferBindings", "u32"),
                Struct::member("maxEmbeddedImmutableSamplerBindings", "u32"),
                Struct::member("maxEmbeddedImmutableSamplers", "u32"),
                Struct::member("bufferCaptureReplayDescriptorDataSize", "usize"),
                Struct::member("imageCaptureReplayDescriptorDataSize", "usize"),
                Struct::member("imageViewCaptureReplayDescriptorDataSize", "usize"),
                Struct::member("samplerCaptureReplayDescriptorDataSize", "usize"),
                Struct::member("accelerationStructureCaptureReplayDescriptorDataSize", "usize"),
                Struct::member("samplerDescriptorSize", "usize"),
                Struct::member("combinedImageSamplerDescriptorSize", "usize"),
                Struct::member("sampledImageDescriptorSize", "usize"),
                Struct::member("storageImageDescriptorSize", "usize"),
                Struct::member("uniformTexelBufferDescriptorSize", "usize"),
                Struct::member("robustUniformTexelBufferDescriptorSize", "usize"),
                Struct::member("storageTexelBufferDescriptorSize", "usize"),
                Struct::member("robustStorageTexelBufferDescriptorSize", "usize"),
                Struct::member("uniformBufferDescriptorSize", "usize"),
                Struct::member("robustUniformBufferDescriptorSize", "usize"),
                Struct::member("storageBufferDescriptorSize", "usize"),
                Struct::member("robustStorageBufferDescriptorSize", "usize"),
                Struct::member("inputAttachmentDescriptorSize", "usize"),
                Struct::member("accelerationStructureDescriptorSize", "usize"),
                Struct::member("maxSamplerDescriptorBufferRange", "VkDeviceSize"),
                Struct::member("maxResourceDescriptorBufferRange", "VkDeviceSize"),
                Struct::member("samplerDescriptorBufferAddressSpaceSize", "VkDeviceSize"),
                Struct::member("resourceDescriptorBufferAddressSpaceSize", "VkDeviceSize"),
                Struct::member("descriptorBufferAddressSpaceSize", "VkDeviceSize"),
            ],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceDescriptorBuferDensityMapProperties",
            "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES",
            vk_ext_enum(317, 1) as _,
            StructUsage::Sink,
            &[Struct::member("combinedImageSamplerDensityMapDescriptorSize", "usize")],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceDescriptorBufferFeatures",
            "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES",
            vk_ext_enum(317, 2) as _,
            StructUsage::Both,
            &[
                Struct::member("descriptorBuffer", "VkBool32"),
                Struct::member("descriptorBufferCaptureReplay", "VkBool32"),
                Struct::member("descriptorBufferImageLayoutIgnored", "VkBool32"),
                Struct::member("descriptorBufferPushDescriptors", "VkBool32"),
            ],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Struct(
        Struct::typed(
            "DescriptorAddressInfo",
            "DESCRIPTOR_ADDRESS_INFO",
            vk_ext_enum(317, 3) as _,
            StructUsage::Sink,
            &[
                Struct::member("address", "VkDeviceAddress"),
                Struct::member("range", "VkDeviceSize"),
                Struct::member("format", "VkFormat"),
            ],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Struct(
        Struct::typed(
            "DescriptorBufferBindingInfo",
            "DESCRIPTOR_BUFFER_BINDING_INFO",
            vk_ext_enum(317, 11) as _,
            StructUsage::Sink,
            &[
                Struct::member("address", "VkDeviceAddress"),
                Struct::member("usage", "VkBufferUsageFlags"),
            ],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Struct(
        Struct::typed(
            "DescriptorBufferBindingPushDescriptorBufferHandle",
            "DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE",
            vk_ext_enum(317, 12) as _,
            StructUsage::Sink,
            &[Struct::member("buffer", "VkBuffer")],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Union(
        Union::new(
            "DescriptorData",
            &[
                Union::member("pSampler", "*const VkSampler"),
                Union::member("pCombinedImageSampler", "*const VkDescriptorImageInfo"),
                Union::member("pInputAttachmentImage", "*const VkDescriptorImageInfo"),
                Union::member("pSampledImage", "*const VkDescriptorImageInfo"),
                Union::member("pStorageImage", "*const VkDescriptorImageInfo"),
                Union::member("pUniformTexelBuffer", "*const VkDescriptorAddressInfoEXT"),
                Union::member("pStorageTexelBuffer", "*const VkDescriptorAddressInfoEXT"),
                Union::member("pUniformBuffer", "*const VkDescriptorAddressInfoEXT"),
                Union::member("pStorageBuffer", "*const VkDescriptorAddressInfoEXT"),
                Union::member("accelerationStructure", "VkDeviceAddress"),
            ],
        )
        .extension("EXT", "descriptor_buffer"),
    ),
    Element::Struct(
        Struct::typed(
            "DescriptorGetInfo",
            "DESCRIPTOR_GET_INFO",
            vk_ext_enum(317, 4) as _,
            StructUsage::Source,
            &[
                Struct::member("r#type", "VkDescriptorType"),
                Struct::member("data", "VkDescriptorDataEXT"),
            ],
        )
        .extensions(&[("EXT", "descriptor_buffer")])
        .non_debuggable(),
    ),
    Element::Struct(
        Struct::typed(
            "BufferCaptureDescriptorDataInfo",
            "BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO",
            vk_ext_enum(317, 5) as _,
            StructUsage::Source,
            &[Struct::member("buffer", "VkBuffer")],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Struct(
        Struct::typed(
            "ImageCaptureDescriptorDataInfo",
            "IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO",
            vk_ext_enum(317, 6) as _,
            StructUsage::Source,
            &[Struct::member("image", "VkImage")],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Struct(
        Struct::typed(
            "ImageViewCaptureDescriptorDataInfo",
            "IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO",
            vk_ext_enum(317, 7) as _,
            StructUsage::Source,
            &[Struct::member("imageView", "VkImageView")],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Struct(
        Struct::typed(
            "SamplerCaptureDescriptorDataInfo",
            "SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO",
            vk_ext_enum(317, 8) as _,
            StructUsage::Source,
            &[Struct::member("sampler", "VkSampler")],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Struct(
        Struct::typed(
            "OpaqueCaptureDescriptorDataCreateInfo",
            "OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO",
            vk_ext_enum(317, 10) as _,
            StructUsage::Source,
            &[Struct::member(
                "opaqueCaptureDescriptorData",
                "*const core::ffi::c_void",
            )],
        )
        .extensions(&[("EXT", "descriptor_buffer")]),
    ),
    Element::Command(
        Command::new(
            "GetDescriptorSetLayoutSize",
            &[
                ("device", "VkDevice"),
                ("layout", "VkDescriptorSetLayout"),
                ("pLayoutSizeInBytes", "*mut VkDeviceSize"),
            ],
        )
        .extension("EXT", "descriptor_buffer"),
    ),
    Element::Command(
        Command::new(
            "GetDescriptorSetLayoutBindingOffset",
            &[
                ("device", "VkDevice"),
                ("layout", "VkDescriptorSetLayout"),
                ("binding", "u32"),
                ("pOffset", "*mut VkDeviceSize"),
            ],
        )
        .extension("EXT", "descriptor_buffer"),
    ),
    Element::Command(
        Command::new(
            "GetDescriptor",
            &[
                ("device", "VkDevice"),
                ("pDescriptorInfo", "*const VkDescriptorGetInfoEXT"),
                ("dataSize", "usize"),
                ("pDescriptor", "*mut core::ffi::c_void"),
            ],
        )
        .extension("EXT", "descriptor_buffer"),
    ),
    Element::Command(
        Command::inst(
            "BindDescriptorBuffers",
            &[
                ("bufferCount", "u32"),
                ("pBindingInfos", "*const VkDescriptorBufferBindingInfoEXT"),
            ],
        )
        .extension("EXT", "descriptor_buffer"),
    ),
    Element::Command(
        Command::inst(
            "SetDescriptorBufferOffsets",
            &[
                ("pipelineBindPoint", "VkPipelineBindPoint"),
                ("layout", "VkPipelineLayout"),
                ("firstSet", "u32"),
                ("setCount", "u32"),
                ("pBufferIndices", "*const u32"),
                ("pOffsets", "*const VkDeviceSize"),
            ],
        )
        .extension("EXT", "descriptor_buffer"),
    ),
    Element::Command(
        Command::inst(
            "BindDescriptorBufferEmbeddedSamplers",
            &[
                ("pipelineBindPoint", "VkPipelineBindPoint"),
                ("layout", "VkPipelineLayout"),
                ("set", "u32"),
            ],
        )
        .extension("EXT", "descriptor_buffer"),
    ),
    Element::Command(
        Command::new(
            "GetBufferOpaqueCaptureDescriptorData",
            &[
                ("device", "VkDevice"),
                ("pInfo", "*const VkBufferCaptureDescriptorDataInfoEXT"),
                ("pData", "*mut core::ffi::c_void"),
            ],
        )
        .failable()
        .extension("EXT", "descriptor_buffer"),
    ),
    Element::Command(
        Command::new(
            "GetImageOpaqueCaptureDescriptorData",
            &[
                ("device", "VkDevice"),
                ("pInfo", "*const VkImageCaptureDescriptorDataInfoEXT"),
                ("pData", "*mut core::ffi::c_void"),
            ],
        )
        .failable()
        .extension("EXT", "descriptor_buffer"),
    ),
    Element::Command(
        Command::new(
            "GetImageViewOpaqueCaptureDescriptorData",
            &[
                ("device", "VkDevice"),
                ("pInfo", "*const VkImageViewCaptureDescriptorDataInfoEXT"),
                ("pData", "*mut core::ffi::c_void"),
            ],
        )
        .failable()
        .extension("EXT", "descriptor_buffer"),
    ),
    Element::Command(
        Command::new(
            "GetSamplerOpaqueCaptureDescriptorData",
            &[
                ("device", "VkDevice"),
                ("pInfo", "*const VkSamplerCaptureDescriptorDataInfoEXT"),
                ("pData", "*mut core::ffi::c_void"),
            ],
        )
        .failable()
        .extension("EXT", "descriptor_buffer"),
    ),
    // VK_EXT_external_memory_host
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_external_memory_host", 1)),
    Element::Bitmask(
        Bitmask::extending(
            "ExternalMemoryHandleTypeFlagBits",
            "EXTERNAL_MEMORY_HANDLE_TYPE",
            &[
                Bitmask::entry("HOST_ALLOCATION", 7).extension("EXT", "external_memory_host"),
                Bitmask::entry("HOST_MAPPED_FOREIGN_MEMORY", 8).extension("EXT", "external_memory_host"),
            ],
        )
        .extension("KHR", "external_memory"),
    ),
    Element::Struct(
        Struct::typed(
            "ImportMemoryHostPointerInfo",
            "IMPORT_MEMORY_HOST_POINTER_INFO",
            vk_ext_enum(179, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("handleType", "VkExternalMemoryHandleTypeFlagsKHR"),
                Struct::member("pHostPointer", "*mut core::ffi::c_void"),
            ],
        )
        .extensions(&[("EXT", "external_memory_host")]),
    ),
    Element::Struct(
        Struct::typed(
            "MemoryHostPointerProperties",
            "MEMORY_HOST_POINTER_PROPERTIES",
            vk_ext_enum(179, 1) as _,
            StructUsage::Sink,
            &[Struct::member("memoryTypeBits", "u32")],
        )
        .extensions(&[("EXT", "external_memory_host")]),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceExternalMemoryHostProperties",
            "PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES",
            vk_ext_enum(179, 2) as _,
            StructUsage::Sink,
            &[Struct::member("minImportedHostPointerAlignment", "VkDeviceSize")],
        )
        .extensions(&[("EXT", "external_memory_host")]),
    ),
    Element::Command(
        Command::new(
            "GetMemoryHostPointerProperties",
            &[
                ("device", "VkDevice"),
                ("handleType", "VkExternalMemoryHandleTypeFlagsKHR"),
                ("pHostPointer", "*const core::ffi::c_void"),
                ("pMemoryHostPointerProperties", "*mut VkMemoryHostPointerPropertiesEXT"),
            ],
        )
        .failable()
        .extension("EXT", "external_memory_host"),
    ),
    // VK_EXT_vertex_attribute_divisor
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_vertex_attribute_divisor", 1)),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceVertexAttributeDivisorProperties",
            "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES",
            vk_ext_enum(191, 0) as _,
            StructUsage::Sink,
            &[Struct::member("maxVertexAttribDivisor", "u32")],
        )
        .extensions(&[("EXT", "vertex_attribute_divisor")]),
    ),
    Element::Struct(
        Struct::new(
            "VertexInputBindingDivisorDescription",
            &[Struct::member("binding", "u32"), Struct::member("divisor", "u32")],
        )
        .extensions(&[("EXT", "vertex_attribute_divisor")]),
    ),
    Element::Struct(
        Struct::typed(
            "PipelineVertexInputDivisorStateCreateInfo",
            "PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO",
            vk_ext_enum(191, 1) as _,
            StructUsage::Source,
            &[
                Struct::member("vertexBindingDivisorCount", "u32"),
                Struct::member(
                    "pVertexBindingDivisors",
                    "*const VkVertexInputBindingDivisorDescriptionEXT",
                ),
            ],
        )
        .extensions(&[("EXT", "vertex_attribute_divisor")]),
    ),
    // VK_EXT_sample_locations
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_sample_locations", 1)),
    Element::Bitmask(Bitmask::extending(
        "ImageCreateFlagBits",
        "IMAGE_CREATE",
        &[Bitmask::entry("SAMPLE_LOCATIONS_COMPATIBLE_DEPTH", 12).extension("EXT", "sample_locations")],
    )),
    Element::Struct(
        Struct::new(
            "SampleLocation",
            &[
                Struct::member("x", "core::ffi::c_float"),
                Struct::member("y", "core::ffi::c_float"),
            ],
        )
        .extensions(&[ex_ext("sample_locations")]),
    ),
    Element::Struct(
        Struct::typed(
            "SampleLocationsInfo",
            "SAMPLE_LOCATIONS_INFO",
            vk_ext_enum(144, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("sampleLocationsPerPixel", "VkSampleCountFlags"),
                Struct::member("sampleLocationGridSize", "VkExtent2D"),
                Struct::member("sampleLocationsCount", "u32"),
                Struct::member("pSampleLocations", "*const VkSampleLocationEXT"),
            ],
        )
        .extensions(&[ex_ext("sample_locations")]),
    ),
    Element::Struct(
        Struct::new(
            "AttachmentSampleLocations",
            &[
                Struct::member("attachmentIndex", "u32"),
                Struct::member("sampleLocationsInfo", "VkSampleLocationsInfoEXT"),
            ],
        )
        .extensions(&[ex_ext("sample_locations")]),
    ),
    Element::Struct(
        Struct::new(
            "SubpassSampleLocations",
            &[
                Struct::member("subpassIndex", "u32"),
                Struct::member("sampleLocationsInfo", "VkSampleLocationsInfoEXT"),
            ],
        )
        .extensions(&[ex_ext("sample_locations")]),
    ),
    Element::Struct(
        Struct::typed(
            "RenderPassSampleLocationsBeginInfo",
            "RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO",
            vk_ext_enum(144, 1) as _,
            StructUsage::Source,
            &[
                Struct::member("attachmentInitialSampleLocationsCount", "u32"),
                Struct::member(
                    "pAttachmentInitialSampleLocations",
                    "*const VkAttachmentSampleLocationsEXT",
                ),
                Struct::member("postSubpassSampleLocationsCount", "u32"),
                Struct::member("pPostSubpassSampleLocations", "*const VkSubpassSampleLocationsEXT"),
            ],
        )
        .extensions(&[ex_ext("sample_locations")]),
    ),
    Element::Struct(
        Struct::typed(
            "PipelineSampleLocationsStateCreateInfo",
            "PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO",
            vk_ext_enum(144, 2) as _,
            StructUsage::Source,
            &[
                Struct::member("sampleLocationsEnable", TY_VK_BOOL),
                Struct::member("sampleLocationsInfo", "VkSampleLocationsInfoEXT"),
            ],
        )
        .extensions(&[ex_ext("sample_locations")]),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceSampleLocationsProperties",
            "PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES",
            vk_ext_enum(144, 3) as _,
            StructUsage::Sink,
            &[
                Struct::member("sampleLocationSampleCounts", "VkSampleCountFlags"),
                Struct::member("maxSampleLocationGridSize", "VkExtent2D"),
                Struct::member("sampleLocationCoordinateRange", "[core::ffi::c_float; 2]"),
                Struct::member("sampleLocationSubpixelBits", "u32"),
                Struct::member("variableSampleLocations", TY_VK_BOOL),
            ],
        )
        .extensions(&[ex_ext("sample_locations")]),
    ),
    Element::Struct(
        Struct::typed(
            "MultisampleProperties",
            "MULTISAMPLE_PROPERTIES",
            vk_ext_enum(144, 4) as _,
            StructUsage::Sink,
            &[Struct::member("maxSampleLocationGridSize", "VkExtent2D")],
        )
        .extensions(&[ex_ext("sample_locations")]),
    ),
    Element::Command(
        Command::inst(
            "SampleLocations",
            &[("pSampleLocationsInfo", "*const VkSampleLocationsInfoEXT")],
        )
        .extension("EXT", "sample_locations"),
    ),
    Element::Command(
        Command::new(
            "GetPhysicalDeviceMultisampleProperties",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("samples", "VkSampleCountFlags"),
                ("pMultisampleProperties", "*mut VkMultisamplePropertiesEXT"),
            ],
        )
        .extension("EXT", "sample_locations"),
    ),
    // VK_NV_fragment_coverage_to_color
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_NV_fragment_coverage_to_color", 1)),
    Element::Bitmask(
        Bitmask::new(
            "PipelineCoverageToColorStateCreateFlags",
            "PipelineCoverageToColorStateCreateFlagBits",
            "PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE",
            &[],
        )
        .extension("NV", "fragment_coverage_to_color"),
    ),
    Element::Struct(
        Struct::typed(
            "PipelineCoverageToColorStateCreateInfo",
            "PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO",
            vk_ext_enum(150, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("flags", "VkPipelineCoverageToColorStateCreateFlagsNV"),
                Struct::member("coverageToColorEnable", TY_VK_BOOL),
                Struct::member("coverageToColorLocation", "u32"),
            ],
        )
        .extensions(&[("NV", "fragment_coverage_to_color")]),
    ),
    // VK_NV_framebuffer_mixed_samples
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_NV_framebuffer_mixed_samples", 1)),
    Element::Enum(
        Enum::new(
            "CoverageModulationMode",
            "COVERAGE_MODULATION_MODE",
            &[
                Enum::member("NONE", 0).extension("NV", "framebuffer_mixed_samples"),
                Enum::member("RGB", 1).extension("NV", "framebuffer_mixed_samples"),
                Enum::member("ALPHA", 2).extension("NV", "framebuffer_mixed_samples"),
                Enum::member("RGBA", 3).extension("NV", "framebuffer_mixed_samples"),
            ],
        )
        .extension("NV", "framebuffer_mixed_samples"),
    ),
    Element::Bitmask(
        Bitmask::new(
            "PipelineCoverageModulationStateCreateFlags",
            "PipelineCoverageModulationStateCreateFlagBits",
            "PIPELINE_COVERAGE_MODULATION_STATE_CREATE",
            &[],
        )
        .extension("NV", "framebuffer_mixed_samples"),
    ),
    Element::Struct(
        Struct::typed(
            "PipelineCoverageModulationStateCreateInfo",
            "PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO",
            vk_ext_enum(153, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("flags", "VkPipelineCoverageModulationStateCreateFlagsNV"),
                Struct::member("coverageModulationMode", "VkCoverageModulationModeNV"),
                Struct::member("coverageModulationTableEnable", TY_VK_BOOL),
                Struct::member("coverageModulationTableCount", "u32"),
                Struct::member("pCoverageModulationTable", "*const core::ffi::c_float"),
            ],
        )
        .extensions(&[("NV", "framebuffer_mixed_samples")]),
    ),
    // VK_EXT_global_priority
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_global_priority", 2)),
    Element::Enum(
        Enum::new(
            "QueueGlobalPriority",
            "QUEUE_GLOBAL_PRIORITY",
            &[
                Enum::member("LOW", 128).extension("EXT", "global_priority"),
                Enum::member("MEDIUM", 256).extension("EXT", "global_priority"),
                Enum::member("HIGH", 512).extension("EXT", "global_priority"),
                Enum::member("REALTIME", 1024).extension("EXT", "global_priority"),
            ],
        )
        .extension("EXT", "global_priority"),
    ),
    Element::Struct(
        Struct::typed(
            "DeviceQueueGlobalPriorityCreateInfo",
            "DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO",
            vk_ext_enum(175, 0) as _,
            StructUsage::Source,
            &[Struct::member("globalPriority", "VkQueueGlobalPriorityEXT")],
        )
        .extensions(&[ex_ext("global_priority")]),
    ),
    // VK_NVX_multiview_per_view_attributes
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_NVX_multiview_per_view_attributes", 1)),
    Element::Bitmask(Bitmask::extending(
        "SubpassDescriptionFlagBits",
        "SUBPASS_DESCRIPTION",
        &[
            Bitmask::entry("PER_VIEW_ATTRIBUTES", 0).extension("NVX", "multiview_per_view_attributes"),
            Bitmask::entry("PER_VIEW_POSITION_X_ONLY", 1).extension("NVX", "multiview_per_view_attributes"),
        ],
    )),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceMultiviewPerViewAttributesProperties",
            "PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES",
            vk_ext_enum(98, 0) as _,
            StructUsage::Sink,
            &[Struct::member("perViewPositionAllComponents", TY_VK_BOOL)],
        )
        .extensions(&[("NVX", "multiview_per_view_attributes")]),
    ),
    // VK_NV_viewport_swizzle
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_NV_viewport_swizzle", 1)),
    Element::Enum(
        Enum::new(
            "ViewportCoordinateSwizzle",
            "VIEWPORT_COORDINATE_SWIZZLE",
            &[
                Enum::member("POSITIVE_X", 0).extension("NV", "viewport_swizzle"),
                Enum::member("NEGATIVE_X", 1).extension("NV", "viewport_swizzle"),
                Enum::member("POSITIVE_Y", 2).extension("NV", "viewport_swizzle"),
                Enum::member("NEGATIVE_Y", 3).extension("NV", "viewport_swizzle"),
                Enum::member("POSITIVE_Z", 4).extension("NV", "viewport_swizzle"),
                Enum::member("NEGATIVE_Z", 5).extension("NV", "viewport_swizzle"),
                Enum::member("POSITIVE_W", 6).extension("NV", "viewport_swizzle"),
                Enum::member("NEGATIVE_W", 7).extension("NV", "viewport_swizzle"),
            ],
        )
        .extension("NV", "viewport_swizzle"),
    ),
    Element::Bitmask(
        Bitmask::new(
            "PipelineViewportSwizzleStateCreateFlags",
            "PipelineViewportSwizzleStateCreateFlagBits",
            "PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE",
            &[],
        )
        .extension("NV", "viewport_swizzle"),
    ),
    Element::Struct(
        Struct::new(
            "ViewportSwizzle",
            &[
                Struct::member("x", "VkViewportCoordinateSwizzleNV"),
                Struct::member("y", "VkViewportCoordinateSwizzleNV"),
                Struct::member("z", "VkViewportCoordinateSwizzleNV"),
                Struct::member("w", "VkViewportCoordinateSwizzleNV"),
            ],
        )
        .extensions(&[("NV", "viewport_swizzle")]),
    ),
    Element::Struct(
        Struct::typed(
            "PipelineViewportSwizzleStateCreateInfo",
            "PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO",
            vk_ext_enum(99, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("viewportCount", "u32"),
                Struct::member("pViewportSwizzles", "*const VkViewportSwizzleNV"),
            ],
        )
        .extensions(&[("NV", "viewport_swizzle")]),
    ),
    // VK_EXT_hdr_metadata
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_hdr_metadata", 1)),
    Element::Struct(
        Struct::new(
            "XYColor",
            &[
                Struct::member("x", "core::ffi::c_float"),
                Struct::member("y", "core::ffi::c_float"),
            ],
        )
        .extensions(&[ex_ext("hdr_metadata")]),
    ),
    Element::Struct(
        Struct::typed(
            "HdrMetadata",
            "HDR_METADATA",
            vk_ext_enum(106, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("displayPrimaryRed", "VkXYColorEXT"),
                Struct::member("displayPrimaryGreen", "VkXYColorEXT"),
                Struct::member("displayPrimaryBlue", "VkXYColorEXT"),
                Struct::member("whitePoint", "VkXYColorEXT"),
                Struct::member("maxLuminance", "core::ffi::c_float"),
                Struct::member("minLuminance", "core::ffi::c_float"),
                Struct::member("maxContentLightLevel", "core::ffi::c_float"),
                Struct::member("maxFrameAverageLightLevel", "core::ffi::c_float"),
            ],
        )
        .extensions(&[ex_ext("hdr_metadata")]),
    ),
    // VK_EXT_display_control
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_display_control", 1)),
    Element::Enum(
        Enum::new(
            "DisplayPowerState",
            "DISPLAY_POWER_STATE",
            &[
                Enum::member("OFF", 0).extension("EXT", "display_control"),
                Enum::member("SUSPEND", 1).extension("EXT", "display_control"),
                Enum::member("ON", 2).extension("EXT", "display_control"),
            ],
        )
        .extension("EXT", "display_control"),
    ),
    Element::Enum(
        Enum::new(
            "DeviceEventType",
            "DEVICE_EVENT_TYPE",
            &[Enum::member("DISPLAY_HOTPLUG", 0).extension("EXT", "display_control")],
        )
        .extension("EXT", "display_control"),
    ),
    Element::Enum(
        Enum::new(
            "DisplayEventType",
            "DISPLAY_EVENT_TYPE",
            &[Enum::member("FIRST_PIXEL_OUT", 0).extension("EXT", "display_control")],
        )
        .extension("EXT", "display_control"),
    ),
    Element::Struct(
        Struct::typed(
            "DisplayPowerInfo",
            "DISPLAY_POWER_INFO",
            vk_ext_enum(92, 0) as _,
            StructUsage::Source,
            &[Struct::member("powerState", "VkDisplayPowerStateEXT")],
        )
        .extensions(&[ex_ext("display_control")]),
    ),
    Element::Struct(
        Struct::typed(
            "DeviceEventInfo",
            "DEVICE_EVENT_INFO",
            vk_ext_enum(92, 1) as _,
            StructUsage::Source,
            &[Struct::member("deviceEvent", "VkDeviceEventTypeEXT")],
        )
        .extensions(&[ex_ext("display_control")]),
    ),
    Element::Struct(
        Struct::typed(
            "DisplayEventInfo",
            "DISPLAY_EVENT_INFO",
            vk_ext_enum(92, 2) as _,
            StructUsage::Source,
            &[Struct::member("displayEVent", "VkDisplayEventTypeEXT")],
        )
        .extensions(&[ex_ext("display_control")]),
    ),
    Element::Struct(
        Struct::typed(
            "SwapchainCounterCreateInfo",
            "SWAPCHAIN_COUNTER_CREATE_INFO",
            vk_ext_enum(92, 3) as _,
            StructUsage::Source,
            &[Struct::member("surfaceCounters", "VkSurfaceCounterFlagsEXT")],
        )
        .extensions(&[ex_ext("display_control")]),
    ),
    Element::Command(
        Command::new(
            "DisplayPowerControl",
            &[
                ("device", "VkDevice"),
                ("display", "VkDisplayKHR"),
                ("pDisplayPowerInfo", "*const VkDisplayPowerInfoEXT"),
            ],
        )
        .failable()
        .extension("EXT", "display_control"),
    ),
    Element::Command(
        Command::new(
            "RegisterDeviceEvent",
            &[
                ("device", "VkDevice"),
                ("pDeviceEventInfo", "*const VkDeviceEventInfoEXT"),
                ("pAllocator", "*const VkAllocationCallbacks"),
                ("pFence", "*mut VkFence"),
            ],
        )
        .failable()
        .extension("EXT", "display_control"),
    ),
    Element::Command(
        Command::new(
            "RegisterDisplayEvent",
            &[
                ("device", "VkDevice"),
                ("pDisplayEventInfo", "*const VkDisplayEventInfoEXT"),
                ("pAllocator", "*const VkAllocationCallbacks"),
                ("pFence", "*mut VkFence"),
            ],
        )
        .failable()
        .extension("EXT", "display_control"),
    ),
    Element::Command(
        Command::new(
            "GetSwapchainCounter",
            &[
                ("device", "VkDevice"),
                ("swapchain", "VkSwapchainKHR"),
                ("counter", "VkSurfaceCounterFlagsEXT"),
                ("pCounterValue", "*mut u64"),
            ],
        )
        .failable()
        .extension("EXT", "display_control"),
    ),
    // VK_GOOGLE_display_timing
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_GOOGLE_display_timing", 1)),
    Element::Struct(
        Struct::new("RefreshCycleDuration", &[Struct::member("refreshDuration", "u64")])
            .extensions(&[("GOOGLE", "display_timing")]),
    ),
    Element::Struct(
        Struct::new(
            "PastPresentationTiming",
            &[
                Struct::member("presentID", "u32"),
                Struct::member("desiredPresentTime", "u64"),
                Struct::member("actualPresentTime", "u64"),
                Struct::member("earliestPresentTime", "u64"),
                Struct::member("presentMargin", "u64"),
            ],
        )
        .extensions(&[("GOOGLE", "display_timing")]),
    ),
    Element::Struct(
        Struct::new(
            "PresentTime",
            &[
                Struct::member("presentID", "u32"),
                Struct::member("desiredPresentTime", "u64"),
            ],
        )
        .extensions(&[("GOOGLE", "display_timing")]),
    ),
    Element::Struct(
        Struct::typed(
            "PresentTimesInfo",
            "PRESENT_TIMES_INFO",
            vk_ext_enum(93, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("swapchainCount", "u32"),
                Struct::member("pTimes", "*const VkPresentTimeGOOGLE"),
            ],
        )
        .extensions(&[("GOOGLE", "display_timing")]),
    ),
    Element::Command(
        Command::new(
            "GetRefreshCycleDuration",
            &[
                ("device", "VkDevice"),
                ("swapchain", "VkSwapchainKHR"),
                ("pDisplayTimingProperties", "*mut VkRefreshCycleDurationGOOGLE"),
            ],
        )
        .failable()
        .extension("GOOGLE", "display_timing"),
    ),
    Element::Command(
        Command::new(
            "GetPastPresentationTiming",
            &[
                ("device", "VkDevice"),
                ("swapchain", "VkSwapchainKHR"),
                ("pPresentationTimingCount", "*mut u32"),
                ("pPresentationTimings", "*mut VkPastPresentationTimingGOOGLE"),
            ],
        )
        .failable()
        .extension("GOOGLE", "display_timing"),
    ),
    // VK_KHR_shared_presentable_image
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_shared_presentable_image", 1)),
    Element::Enum(Enum::extending(
        "ImageLayout",
        "IMAGE_LAYOUT",
        &[Enum::member("SHARED_PRESENT", vk_ext_enum(112, 0) as _).extension("KHR", "shared_presentable_image")],
    )),
    Element::Struct(
        Struct::typed(
            "SharedPresentSurfaceCapabilities",
            "SHARED_PRESENT_SURFACE_CAPABILITIES",
            vk_ext_enum(112, 0) as _,
            StructUsage::Sink,
            &[Struct::member("sharedPresentSupportedUsageFlags", "VkImageUsageFlags")],
        )
        .extensions(&[ex_khr("shared_presentable_image")]),
    ),
    Element::Command(
        Command::new(
            "GetSwapchainStatus",
            &[("device", "VkDevice"), ("swapchain", "VkSwapchainKHR")],
        )
        .failable()
        .extension("KHR", "shared_presentable_image"),
    ),
    // VK_AMD_rasterization_order
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_AMD_rasterization_order", 1)),
    Element::Enum(
        Enum::new(
            "RasterizationOrder",
            "RASTERIZATION_ORDER",
            &[
                Enum::member("STRICT", 0).extension("AMD", "rasterization_order"),
                Enum::member("RELAXED", 1).extension("AMD", "rasterization_order"),
            ],
        )
        .extension("AMD", "rasterization_order"),
    ),
    Element::Struct(
        Struct::typed(
            "PipelineRasterizationStateRasterizationOrder",
            "PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER",
            vk_ext_enum(19, 0) as _,
            StructUsage::Source,
            &[Struct::member("rasterizationOrder", "VkRasterizationOrderAMD")],
        )
        .extensions(&[("AMD", "rasterization_order")]),
    ),
    // VK_AMD_texture_gather_bias_lod
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_AMD_texture_gather_bias_lod", 1)),
    Element::Struct(
        Struct::typed(
            "TextureLODGatherFormatProperties",
            "TEXTURE_LOD_GATHER_FORMAT_PROPERTIES",
            vk_ext_enum(42, 0) as _,
            StructUsage::Sink,
            &[Struct::member("supportsTextureGatherLODBiasAMD", TY_VK_BOOL)],
        )
        .extensions(&[("AMD", "texture_gather_bias_lod")]),
    ),
    // VK_NN_vi_surface
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_NN_vi_surface", 1)),
    Element::Bitmask(
        Bitmask::new(
            "ViSurfaceCreateFlags",
            "ViSurfaceCreateFlagBits",
            "VI_SURFACE_CREATE",
            &[],
        )
        .extension("NN", "vi_surface"),
    ),
    Element::Struct(
        Struct::typed(
            "ViSurfaceCreateInfo",
            "VI_SURFACE_CREATE_INFO",
            vk_ext_enum(63, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("flags", "VkViSurfaceCreateFlagsNN"),
                Struct::member("window", "*mut core::ffi::c_void"),
            ],
        )
        .extensions(&[("NN", "vi_surface")]),
    ),
    Element::Command(
        Command::new(
            "CreateViSurface",
            &[
                ("instance", "VkInstance"),
                ("pCreateInfo", "*const VkViSurfaceCreateInfoNN"),
                ("pAllocator", "*const VkAllocationCallbacks"),
                ("pSurface", "*mut VkSurfaceKHR"),
            ],
        )
        .failable()
        .static_callable()
        .extension("NN", "vi_surface"),
    ),
    // VK_EXT_display_surface_counter
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_display_surface_counter", 1)),
    Element::Bitmask(
        Bitmask::new(
            "SurfaceCounterFlags",
            "SurfaceCounterFlagBits",
            "SURFACE_COUNTER",
            &[Bitmask::entry("VBLANK", 0).extension("EXT", "display_surface_counter")],
        )
        .extension("EXT", "display_surface_counter"),
    ),
    Element::Struct(
        Struct::typed(
            "SurfaceCapabilities2",
            "SURFACE_CAPABILITIES2",
            vk_ext_enum(91, 0) as _,
            StructUsage::Sink,
            &[
                Struct::member("minImageCount", "u32"),
                Struct::member("maxImageCount", "u32"),
                Struct::member("currentExtent", "VkExtent2D"),
                Struct::member("minImageExtent", "VkExtent2D"),
                Struct::member("maxImageExtent", "VkExtent2D"),
                Struct::member("maxImageArrayLayers", "u32"),
                Struct::member("supportedTransforms", "VkSurfaceTransformFlagsKHR"),
                Struct::member("currentTransform", "VkSurfaceTransformFlagsKHR"),
                Struct::member("supportedCompositeAlpha", "VkCompositeAlphaFlagsKHR"),
                Struct::member("supportedUsageFlags", "VkImageUsageFlags"),
                Struct::member("supportedSurfaceCounters", "VkSurfaceCounterFlagsEXT"),
            ],
        )
        .extensions(&[ex_ext("display_surface_counter")]),
    ),
    Element::Command(
        Command::new(
            "GetPhysicalDeviceSurfaceCapabilities2",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("surface", "VkSurfaceKHR"),
                ("pSurfaceCapabilities", "*mut VkSurfaceCapabilities2EXT"),
            ],
        )
        .failable()
        .extension("EXT", "display_surface_counter"),
    ),
    // VK_EXT_debug_marker
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_debug_marker", 4)),
    Element::Struct(
        Struct::typed(
            "DebugMarkerObjectNameInfo",
            "DEBUG_MARKER_OBJECT_NAME_INFO",
            vk_ext_enum(23, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("objectType", "VkDebugReportObjectTypeEXT"),
                Struct::member("object", "u64"),
                Struct::member("pObjectName", "*const core::ffi::c_char"),
            ],
        )
        .extensions(&[ex_ext("debug_marker")]),
    ),
    Element::Struct(
        Struct::typed(
            "DebugMarkerObjectTagInfo",
            "DEBUG_MARKER_OBJECT_TAG_INFO",
            vk_ext_enum(23, 1) as _,
            StructUsage::Source,
            &[
                Struct::member("objectType", "VkDebugReportObjectTypeEXT"),
                Struct::member("object", "u64"),
                Struct::member("tagName", "u64"),
                Struct::member("tagSize", "usize"),
                Struct::member("pTag", "*const core::ffi::c_void"),
            ],
        )
        .extensions(&[ex_ext("debug_marker")]),
    ),
    Element::Struct(
        Struct::typed(
            "DebugMarkerMarkerInfo",
            "DEBUG_MARKER_MARKER_INFO",
            vk_ext_enum(23, 2) as _,
            StructUsage::Source,
            &[
                Struct::member("pMarkerName", "*const core::ffi::c_char"),
                Struct::member("color", "[core::ffi::c_float; 4]"),
            ],
        )
        .extensions(&[ex_ext("debug_marker")]),
    ),
    Element::Command(
        Command::new(
            "DebugMarkerSetObjectTag",
            &[
                ("device", "VkDevice"),
                ("pTagInfo", "*const VkDebugMarkerObjectTagInfoEXT"),
            ],
        )
        .failable()
        .extension("EXT", "debug_marker"),
    ),
    Element::Command(
        Command::new(
            "DebugMarkerSetObjectName",
            &[
                ("device", "VkDevice"),
                ("pNameInfo", "*const VkDebugMarkerObjectNameInfoEXT"),
            ],
        )
        .failable()
        .extension("EXT", "debug_marker"),
    ),
    Element::Command(
        Command::inst(
            "DebugMarkerBegin",
            &[("pMarkerInfo", "*const VkDebugMarkerMarkerInfoEXT")],
        )
        .extension("EXT", "debug_marker"),
    ),
    Element::Command(Command::inst("DebugMarkerEnd", &[]).extension("EXT", "debug_marker")),
    Element::Command(
        Command::inst(
            "DebugMarkerInsert",
            &[("pMarkerInfo", "*const VkDebugMarkerMarkerInfoEXT")],
        )
        .extension("EXT", "debug_marker"),
    ),
    // VK_IMG_filter_cubic
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_IMG_filter_cubic", 1)),
    Element::Enum(Enum::extending(
        "Filter",
        "FILTER",
        &[Enum::member("CUBIC", vk_ext_enum(16, 0) as _).extension("IMG", "filter_cubic")],
    )),
    Element::Bitmask(Bitmask::extending(
        "FormatFeatureFlagBits",
        "FORMAT_FEATURE",
        &[Bitmask::entry("SAMPLED_IMAGE_FILTER_CUBIC", 13).extension("IMG", "filter_cubic")],
    )),
    // VK_NVX_device_generated_commands
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_NVX_device_generated_commands", 1)),
    Element::Object(
        Object::new("VkObjectTableNVX", "OBJECT_TABLE_NVX", vk_ext_enum(87, 0) as _)
            .extension("VK_NVX_device_generated_commands"),
    ),
    Element::Object(
        Object::new(
            "VkIndirectCommandsLayoutNVX",
            "INDIRECT_COMMANDS_LAYOUT_NVX",
            vk_ext_enum(87, 1) as _,
        )
        .extension("VK_NVX_device_generated_commands"),
    ),
    Element::Enum(
        Enum::new(
            "IndirectCommandsTokenType",
            "INDIRECT_COMMANDS_TOKEN_TYPE",
            &[
                Enum::member("PIPELINE", 0).extension("NVX", "device_generated_commands"),
                Enum::member("DESCRIPTOR_SET", 1).extension("NVX", "device_generated_commands"),
                Enum::member("INDEX_BUFFER", 2).extension("NVX", "device_generated_commands"),
                Enum::member("VERTEX_BUFFER", 3).extension("NVX", "device_generated_commands"),
                Enum::member("PUSH_CONSTANT", 4).extension("NVX", "device_generated_commands"),
                Enum::member("DRAW_INDEXED", 5).extension("NVX", "device_generated_commands"),
                Enum::member("DRAW", 6).extension("NVX", "device_generated_commands"),
                Enum::member("DISPATCH", 7).extension("NVX", "device_generated_commands"),
            ],
        )
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Enum(
        Enum::new(
            "ObjectEntryType",
            "OBJECT_ENTRY_TYPE",
            &[
                Enum::member("DESCRIPTOR_SET", 0).extension("NVX", "device_generated_commands"),
                Enum::member("PIPELINE", 1).extension("NVX", "device_generated_commands"),
                Enum::member("INDEX_BUFFER", 2).extension("NVX", "device_generated_commands"),
                Enum::member("VERTEX_BUFFER", 3).extension("NVX", "device_generated_commands"),
                Enum::member("PUSH_CONSTANT", 4).extension("NVX", "device_generated_commands"),
            ],
        )
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Bitmask(
        Bitmask::new(
            "IndirectCommandsLayoutUsageFlags",
            "IndirectCommandsLayoutUsageFlagBits",
            "INDIRECT_COMMANDS_LAYOUT_USAGE",
            &[
                Bitmask::entry("UNORDERED_SEQUENCES", 0).extension("NVX", "device_generated_commands"),
                Bitmask::entry("SPARSE_SEQUENCES", 1).extension("NVX", "device_generated_commands"),
                Bitmask::entry("EMPTY_EXECUTIONS", 2).extension("NVX", "device_generated_commands"),
                Bitmask::entry("INDEXED_SEQUENCES", 3).extension("NVX", "device_generated_commands"),
            ],
        )
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Bitmask(
        Bitmask::new(
            "ObjectEntryUsageFlags",
            "ObjectEntryUsageFlagBits",
            "OBJECT_ENTRY_USAGE",
            &[
                Bitmask::entry("GRAPHICS", 0).extension("NVX", "device_generated_commands"),
                Bitmask::entry("COMPUTE", 1).extension("NVX", "device_generated_commands"),
            ],
        )
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Bitmask(Bitmask::extending(
        "PipelineStageFlagBits",
        "PIPELINE_STAGE",
        &[Bitmask::entry("COMMAND_PROCESS", 17).extension("NVX", "device_generated_commands")],
    )),
    Element::Bitmask(Bitmask::extending(
        "AccessFlagBits",
        "ACCESS",
        &[
            Bitmask::entry("COMMAND_PROCESS_READ", 17).extension("NVX", "device_generated_commands"),
            Bitmask::entry("COMMAND_PROCESS_WRITE", 18).extension("NVX", "device_generated_commands"),
        ],
    )),
    Element::Struct(
        Struct::typed(
            "DeviceGeneratedCommandsFeatures",
            "DEVICE_GENERATED_COMMANDS_FEATURES",
            vk_ext_enum(87, 5) as _,
            StructUsage::Source,
            &[Struct::member("computeBindingPointSupport", TY_VK_BOOL)],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::typed(
            "DeviceGeneratedCommandsLimits",
            "DEVICE_GENERATED_COMMANDS_LIMITS",
            vk_ext_enum(87, 4) as _,
            StructUsage::Source,
            &[
                Struct::member("maxIndirectCommandsLayoutTokenCount", "u32"),
                Struct::member("maxObjectEntryCounts", "u32"),
                Struct::member("minSequenceCountBufferOffsetAlignment", "u32"),
                Struct::member("minSequenceIndexBufferOffsetAlignment", "u32"),
                Struct::member("minCommandsTokenBufferOffsetAlignment", "u32"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::new(
            "IndirectCommandsToken",
            &[
                Struct::member("tokenType", "VkIndirectCommandsTokenTypeNVX"),
                Struct::member("buffer", "VkBuffer"),
                Struct::member("offset", "VkDeviceSize"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::new(
            "IndirectCommandsLayoutToken",
            &[
                Struct::member("tokenType", "VkIndirectCommandsTokenTypeNVX"),
                Struct::member("bindingUnit", "u32"),
                Struct::member("dynamicCount", "u32"),
                Struct::member("divisor", "u32"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::typed(
            "IndirectCommandsLayoutCreateInfo",
            "INDIRECT_COMMANDS_LAYOUT_CREATE_INFO",
            vk_ext_enum(87, 1) as _,
            StructUsage::Source,
            &[
                Struct::member("pipelineBindPoint", "VkPipelineBindPoint"),
                Struct::member("flags", "VkIndirectCommandsLayoutUsageFlagsNVX"),
                Struct::member("tokenCount", "u32"),
                Struct::member("pTokens", "*const VkIndirectCommandsLayoutTokenNVX"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::typed(
            "CmdProcessCommandsInfo",
            "CMD_PROCESS_COMMANDS_INFO",
            vk_ext_enum(87, 2) as _,
            StructUsage::Source,
            &[
                Struct::member("objectTable", "VkObjectTableNVX"),
                Struct::member("indirectCommandsLayout", "VkIndirectCommandsLayoutNVX"),
                Struct::member("indirectCommandsTokenCount", "u32"),
                Struct::member("pIndirectCommandsTokens", "*const VkIndirectCommandsTokenNVX"),
                Struct::member("maxSequencesCount", "u32"),
                Struct::member("targetCommandBuffer", "VkCommandBuffer"),
                Struct::member("sequencesCountBuffer", "VkBuffer"),
                Struct::member("sequencesCountOffset", "VkDeviceSize"),
                Struct::member("sequencesIndexBuffer", "VkBuffer"),
                Struct::member("sequencesIndexOffset", "VkDeviceSize"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::typed(
            "CmdReserveSpaceForCommandsInfo",
            "CMD_RESERVE_SPACE_FOR_COMMANDS_INFO",
            vk_ext_enum(87, 3) as _,
            StructUsage::Source,
            &[
                Struct::member("objectTable", "VkObjectTableNVX"),
                Struct::member("indirectCommandsLayout", "VkIndirectCommandsLayoutNVX"),
                Struct::member("maxSequencesCount", "u32"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::typed(
            "ObjectTableCreateInfo",
            "OBJECT_TABLE_CREATE_INFO",
            vk_ext_enum(87, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("objectCount", "u32"),
                Struct::member("pObjectEntryTypes", "*const VkObjectEntryTypeNVX"),
                Struct::member("pObjectEntryCounts", "*const u32"),
                Struct::member("pObjectEntryUsageFlags", "*const VkObjectEntryUsageFlagsNVX"),
                Struct::member("maxUniformBuffersPerDescriptor", "u32"),
                Struct::member("maxStorageBuffersPerDescriptor", "u32"),
                Struct::member("maxStorageImagesPerDescriptor", "u32"),
                Struct::member("maxSampledImagesPerDescriptor", "u32"),
                Struct::member("maxPipelineLayouts", "u32"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::new(
            "ObjectTableEntry",
            &[
                Struct::member("r#type", "VkObjectEntryTypeNVX"),
                Struct::member("flags", "VkObjectEntryUsageFlagsNVX"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::new(
            "ObjectTablePipelineEntry",
            &[
                Struct::member("r#type", "VkObjectEntryTypeNVX"),
                Struct::member("flags", "VkObjectEntryUsageFlagsNVX"),
                Struct::member("pipeline", "VkPipeline"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::new(
            "ObjectTableDescriptorSEtEntry",
            &[
                Struct::member("r#type", "VkObjectEntryTypeNVX"),
                Struct::member("flags", "VkObjectEntryUsageFlagsNVX"),
                Struct::member("pipelineLayout", "VkPipelineLayout"),
                Struct::member("descriptorSet", "VkDescriptorSet"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::new(
            "ObjectTableVertexBufferEntry",
            &[
                Struct::member("r#type", "VkObjectEntryTypeNVX"),
                Struct::member("flags", "VkObjectEntryUsageFlagsNVX"),
                Struct::member("buffer", "VkBuffer"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::new(
            "ObjectTableIndexBufferEntry",
            &[
                Struct::member("r#type", "VkObjectEntryTypeNVX"),
                Struct::member("flags", "VkObjectEntryUsageFlagsNVX"),
                Struct::member("buffer", "VkBuffer"),
                Struct::member("indexType", "VkIndexType"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Struct(
        Struct::new(
            "ObjectTablePushConstantEntry",
            &[
                Struct::member("r#type", "VkObjectEntryTypeNVX"),
                Struct::member("flags", "VkObjectEntryUsageFlagsNVX"),
                Struct::member("pipelineLayout", "VkPipelineLayout"),
                Struct::member("stageFlags", "VkShaderStageFlags"),
            ],
        )
        .extensions(&[("NVX", "device_generated_commands")]),
    ),
    Element::Command(
        Command::inst(
            "ProcessCommands",
            &[("pProcessCommandsInfo", "*const VkCmdProcessCommandsInfoNVX")],
        )
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Command(
        Command::inst(
            "ReserveSpaceForCommands",
            &[("pReserveSpaceInfo", "*const VkCmdReserveSpaceForCommandsInfoNVX")],
        )
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Command(
        Command::new(
            "CreateIndirectCommandsLayout",
            &[
                ("device", "VkDevice"),
                ("pCreateInfo", "*const VkIndirectCommandsLayoutCreateInfoNVX"),
                ("pAllocator", "*const VkAllocationCallbacks"),
                ("pIndirectCommandsLayout", "*mut VkIndirectCommandsLayoutNVX"),
            ],
        )
        .failable()
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Command(
        Command::new(
            "DestroyIndirectCommandsLayout",
            &[
                ("device", "VkDevice"),
                ("indirectCommandsLayout", "VkIndirectCommandsLayoutNVX"),
                ("pAllocator", "*const VkAllocationCallbacks"),
            ],
        )
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Command(
        Command::new(
            "CreateObjectTable",
            &[
                ("device", "VkDevice"),
                ("pCreateInfo", "*const VkObjectTableCreateInfoNVX"),
                ("pAllocator", "*const VkAllocationCallbacks"),
                ("pObjectTable", "*mut VkObjectTableNVX"),
            ],
        )
        .failable()
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Command(
        Command::new(
            "DestroyObjectTable",
            &[
                ("device", "VkDevice"),
                ("objectTable", "VkObjectTableNVX"),
                ("pAllocator", "*const VkAllocationCallbacks"),
            ],
        )
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Command(
        Command::new(
            "RegisterObjects",
            &[
                ("device", "VkDevice"),
                ("objectTable", "VkObjectTableNVX"),
                ("objectCount", "u32"),
                ("ppObjectTableEntries", "*const *const VkObjectTableEntryNVX"),
                ("pObjectIndices", "*const u32"),
            ],
        )
        .failable()
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Command(
        Command::new(
            "UnregisterObjects",
            &[
                ("device", "VkDevice"),
                ("objectTable", "VkObjectTableNVX"),
                ("objectCount", "u32"),
                ("pObjectEntryTypes", "*const VkObjectEntryTypeNVX"),
                ("pObjectIndices", "*const u32"),
            ],
        )
        .failable()
        .extension("NVX", "device_generated_commands"),
    ),
    Element::Command(
        Command::new(
            "GetPhysicalDeviceGeneratedCommandsProperties",
            &[
                ("physicalDevice", "VkPhysicalDevice"),
                ("pFeatures", "*mut VkDeviceGeneratedCommandsFeaturesNVX"),
                ("pLimits", "*mut VkDeviceGeneratedCommandsLimitsNVX"),
            ],
        )
        .extension("NVX", "device_generated_commands"),
    ),
    // VK_KHR_incremental_present
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_KHR_incremental_present", 1)),
    Element::Struct(
        Struct::new(
            "RectLayer",
            &[
                Struct::member("offset", "VkOffset2D"),
                Struct::member("extent", "VkExtent2D"),
                Struct::member("layer", "u32"),
            ],
        )
        .extensions(&[ex_khr("incremental_present")]),
    ),
    Element::Struct(
        Struct::new(
            "PresentRegion",
            &[
                Struct::member("rectangleCount", "u32"),
                Struct::member("pRectangles", "*const VkRectLayerKHR"),
            ],
        )
        .extensions(&[ex_khr("incremental_present")]),
    ),
    Element::Struct(
        Struct::typed(
            "PresentRegions",
            "PRESENT_REGIONS",
            vk_ext_enum(85, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("swapchainCount", "u32"),
                Struct::member("pRegions", "*const VkPresentRegionKHR"),
            ],
        )
        .extensions(&[ex_khr("incremental_present")]),
    ),
    // VK_NV_clip_space_w_scaling
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_NV_clip_space_w_scaling", 1)),
    Element::Enum(Enum::extending(
        "DynamicState",
        "DYNAMIC_STATE",
        &[Enum::member("VIEWPORT_W_SCALING", vk_ext_enum(88, 0) as _).extension("NV", "clip_space_w_scaling")],
    )),
    Element::Struct(
        Struct::new(
            "ViewportWScaling",
            &[
                Struct::member("xcoeff", "core::ffi::c_float"),
                Struct::member("ycoeff", "core::ffi::c_float"),
            ],
        )
        .extensions(&[("NV", "clip_space_w_scaling")]),
    ),
    Element::Struct(
        Struct::typed(
            "PipelineViewportWScalingStateCreateInfo",
            "PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO",
            vk_ext_enum(88, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("viewportWScalingEnable", TY_VK_BOOL),
                Struct::member("viewportCount", "u32"),
                Struct::member("pViewportWScalings", "*const VkViewportWScalingNV"),
            ],
        )
        .extensions(&[("NV", "clip_space_w_scaling")]),
    ),
    Element::Command(
        Command::inst(
            "SetViewportWScaling",
            &[
                ("firstViewport", "u32"),
                ("viewportCount", "u32"),
                ("pViewportWScalings", "*const VkViewportWScalingNV"),
            ],
        )
        .extension("NV", "clip_space_w_scaling"),
    ),
    // VK_IMG_format_pvrtc
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_IMG_format_pvrtc", 1)),
    Element::Enum(Enum::extending(
        "Format",
        "FORMAT",
        &[
            Enum::member("PVRTC1_2BPP_UNORM_BLOCK", vk_ext_enum(55, 0) as _).extension("IMG", "format_pvrtc"),
            Enum::member("PVRTC1_4BPP_UNORM_BLOCK", vk_ext_enum(55, 1) as _).extension("IMG", "format_pvrtc"),
            Enum::member("PVRTC2_2BPP_UNORM_BLOCK", vk_ext_enum(55, 2) as _).extension("IMG", "format_pvrtc"),
            Enum::member("PVRTC2_4BPP_UNORM_BLOCK", vk_ext_enum(55, 3) as _).extension("IMG", "format_pvrtc"),
            Enum::member("PVRTC1_2BPP_SRGB_BLOCK", vk_ext_enum(55, 4) as _).extension("IMG", "format_pvrtc"),
            Enum::member("PVRTC1_4BPP_SRGB_BLOCK", vk_ext_enum(55, 5) as _).extension("IMG", "format_pvrtc"),
            Enum::member("PVRTC2_2BPP_SRGB_BLOCK", vk_ext_enum(55, 6) as _).extension("IMG", "format_pvrtc"),
            Enum::member("PVRTC2_4BPP_SRGB_BLOCK", vk_ext_enum(55, 7) as _).extension("IMG", "format_pvrtc"),
        ],
    )),
    // VK_NV_fill_rectangle
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_NV_fill_rectangle", 1)),
    Element::Enum(Enum::extending(
        "PolygonMode",
        "POLYGON_MODE",
        &[Enum::member("FILL_RECTANGLE", vk_ext_enum(154, 0) as _).extension("NV", "fill_rectangle")],
    )),
    // VK_EXT_conservative_rasterization
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_conservative_rasterization", 1)),
    Element::Enum(
        Enum::new(
            "ConservativeRasterizationMode",
            "CONSERVATIVE_RASTERIZATION_MODE",
            &[
                Enum::member("DISABLED", 0).extension("EXT", "conservative_rasterization"),
                Enum::member("OVERESTIMATE", 1).extension("EXT", "conservative_rasterization"),
                Enum::member("UNDERESTIMATE", 2).extension("EXT", "conservative_rasterization"),
            ],
        )
        .extension("EXT", "conservative_rasterization"),
    ),
    Element::Bitmask(
        Bitmask::new(
            "PipelineRasterizationConservativeStateCreateFlags",
            "PipelineRasterizationConservativeStateCreateFlagBits",
            "PIPELINE_RASTERIZATION_CONSERVATIVE_STATE",
            &[],
        )
        .extension("EXT", "conservative_rasterization"),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceConservativeRasterizationProperties",
            "PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES",
            vk_ext_enum(102, 0) as _,
            StructUsage::Sink,
            &[
                Struct::member("primitiveOverestimationSize", "core::ffi::c_float"),
                Struct::member("maxExtraPrimitiveOverestimationSize", "core::ffi::c_float"),
                Struct::member("extraPrimitiveOverestimationSizeGranularity", "core::ffi::c_float"),
                Struct::member("primitiveUnderestimation", TY_VK_BOOL),
                Struct::member("conservativePointAndLineRasterization", TY_VK_BOOL),
                Struct::member("degenerateTrianglesRasterized", TY_VK_BOOL),
                Struct::member("degenerateLinesRasterized", TY_VK_BOOL),
                Struct::member("fullyCoveredFragmentShaderInputVariable", TY_VK_BOOL),
                Struct::member("conservativeRasterizationPostDepthCoverage", TY_VK_BOOL),
            ],
        )
        .extensions(&[ex_ext("conservative_rasterization")]),
    ),
    Element::Struct(
        Struct::typed(
            "PipelineRasterizationConservativeStateCreateInfo",
            "PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO",
            vk_ext_enum(102, 1) as _,
            StructUsage::Source,
            &[
                Struct::member("flags", "VkPipelineRasterizationConservativeStateCreateFlagsEXT"),
                Struct::member("conservativeRasterizationMode", "VkConservativeRasterizationModeEXT"),
                Struct::member("extraPrimitiveOverestimationSize", "core::ffi::c_float"),
            ],
        )
        .extensions(&[ex_ext("conservative_rasterization")]),
    ),
    // VK_EXT_discard_rectangles
    Element::ExtensionHeaderConstants(ExtensionHeaderConstants::new("VK_EXT_discard_rectangles", 1)),
    Element::Enum(Enum::extending(
        "DynamicState",
        "DYNAMIC_STATE",
        &[Enum::member("DISCARD_RECTANGLE", vk_ext_enum(100, 0) as _).extension("EXT", "discard_rectangles")],
    )),
    Element::Enum(
        Enum::new(
            "DiscardRectangleMode",
            "DISCARD_RECTANGLE_MODE",
            &[
                Enum::member("INCLUSIVE", 0).extension("EXT", "discard_rectangles"),
                Enum::member("EXCLUSIVE", 1).extension("EXT", "discard_rectangles"),
            ],
        )
        .extension("EXT", "discard_rectangles"),
    ),
    Element::Bitmask(
        Bitmask::new(
            "PipelineDiscardRectangleStateCreateFlags",
            "PipelineDiscardRectangleStateCreateFlagBits",
            "PIPELINE_DISCARD_RECTANGLE_STATE",
            &[],
        )
        .extension("EXT", "discard_rectangles"),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceDiscardRectangleProperties",
            "PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES",
            vk_ext_enum(100, 0) as _,
            StructUsage::Sink,
            &[Struct::member("maxDiscardRectangles", "u32")],
        )
        .extensions(&[ex_ext("discard_rectangles")]),
    ),
    Element::Struct(
        Struct::typed(
            "PipelineDiscardRectangleStateCreateInfo",
            "PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO",
            vk_ext_enum(100, 1) as _,
            StructUsage::Source,
            &[
                Struct::member("flags", "VkPipelineDiscardRectangleStateCreateFlagsEXT"),
                Struct::member("discardRectangleMode", "VkDiscardRectangleModeEXT"),
                Struct::member("discardRectangleCount", "u32"),
                Struct::member("pDiscardRectangles", "*const VkRect2D"),
            ],
        )
        .extensions(&[ex_ext("discard_rectangles")]),
    ),
    Element::Command(
        Command::inst(
            "SetDiscardRectangle",
            &[
                ("firstDiscardRectangle", "u32"),
                ("discardRectangleCount", "u32"),
                ("pDiscardRectangles", "*const VkRect2D"),
            ],
        )
        .extension("EXT", "discard_rectangles"),
    ),
    // VK_EXT_image_drm_format_modifier
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER)),
    Element::Enum(Enum::extending(
        "ImageTiling",
        "IMAGE_TILING",
        &[Enum::member("DRM_FORMAT_MODIFIER", vk_ext_enum(159, 0) as _).extension2(&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER)],
    )),
    Element::Bitmask(Bitmask::extending(
        "ImageAspectFlagBits",
        "IMAGE_ASPECT",
        &[
            Bitmask::entry("MEMORY_PLANE_0", 7).extension2(&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER),
            Bitmask::entry("MEMORY_PLANE_1", 8).extension2(&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER),
            Bitmask::entry("MEMORY_PLANE_2", 9).extension2(&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER),
            Bitmask::entry("MEMORY_PLANE_3", 10).extension2(&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER),
        ],
    )),
    Element::Struct(
        Struct::new(
            "DrmFormatModifierProperties",
            &[
                Struct::member("drmFormatModifier", "u64"),
                Struct::member("drmFormatModifierPlaneCount", "u32"),
                Struct::member("drmFormatModifierTilingFeatures", "VkFormatFeatureFlags"),
            ],
        )
        .extensions2(&[&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER]),
    ),
    Element::Struct(
        Struct::typed(
            "DrmFormatModifierPropertiesList",
            "DRM_FORMAT_MODIFIER_PROPERTIES_LIST",
            vk_ext_enum(159, 0) as _,
            StructUsage::Sink,
            &[
                Struct::member("drmFormatModifierCount", "u32"),
                Struct::member("pDrmFormatModifierProperties", "*mut VkDrmFormatModifierPropertiesEXT"),
            ],
        )
        .extensions2(&[&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER]),
    ),
    Element::Struct(
        Struct::typed(
            "PhysicalDeviceImageDrmFormatModifierInfo",
            "PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO",
            vk_ext_enum(159, 2) as _,
            StructUsage::Source,
            &[
                Struct::member("drmFormatModifer", "u64"),
                Struct::member("sharingMode", "VkSharingMode"),
                Struct::member("queueFamilyIndexCount", "u32"),
                Struct::member("pQueueFamilyIndices", "*const u32"),
            ],
        )
        .extensions2(&[&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER]),
    ),
    Element::Struct(
        Struct::typed(
            "ImageDrmFormatModifierListCreateInfo",
            "IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO",
            vk_ext_enum(159, 3) as _,
            StructUsage::Source,
            &[
                Struct::member("drmFormatModifierCount", "u32"),
                Struct::member("pDrmFormatModifiers", "*const u64"),
            ],
        )
        .extensions2(&[&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER]),
    ),
    Element::Struct(
        Struct::typed(
            "ImageDrmFormatModifierExplicitCreateInfo",
            "IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO",
            vk_ext_enum(159, 4) as _,
            StructUsage::Source,
            &[
                Struct::member("drmFormatModifier", "u64"),
                Struct::member("drmFormatModifierPlaneCount", "u32"),
                Struct::member("pPlaneLayouts", "*const VkSubresourceLayout"),
            ],
        )
        .extensions2(&[&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER]),
    ),
    Element::Struct(
        Struct::typed(
            "ImageDrmFormatModifierProperties",
            "IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES",
            vk_ext_enum(159, 5) as _,
            StructUsage::Sink,
            &[Struct::member("drmFormatModifier", "u64")],
        )
        .extensions2(&[&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER]),
    ),
    Element::Struct(
        Struct::new(
            "DrmFormatModifierProperties2",
            &[
                Struct::member("drmFormatModifier", "u64"),
                Struct::member("drmFormatModifierPlaneCount", "u32"),
                Struct::member("drmFormatModifierTilingFeatures", "VkFormatFeatureFlags2KHR"),
            ],
        )
        .extensions2(&[&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER])
        .available_condition("feature = \"VK_KHR_format_feature_flags2\""),
    ),
    Element::Struct(
        Struct::typed(
            "DrmFormatModifierPropertiesList2",
            "DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2",
            vk_ext_enum(159, 6) as _,
            StructUsage::Sink,
            &[
                Struct::member("drmFormatModifierCount", "u32"),
                Struct::member("pDrmFormatModifierProperties", "*mut VkDrmFormatModifierProperties2EXT"),
            ],
        )
        .extensions2(&[&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER])
        .available_condition("feature = \"VK_KHR_format_feature_flags2\""),
    ),
    Element::Command(
        Command::new(
            "GetImageDrmFormatModifierProperties",
            &[
                ("device", "VkDevice"),
                ("image", "VkImage"),
                ("pProperites", "*mut VkImageDrmFormatModifierPropertiesEXT"),
            ],
        )
        .failable()
        .extension2(&VK_EXT_IMAGE_DRM_FORMAT_MODIFIER),
    ),
    // VK_EXT_metal_objects
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(&VK_EXT_METAL_OBJECTS)),
    Element::Bitmask(
        Bitmask::new(
            "ExportMetalObjectTypeFlags",
            "ExportMetalObjectTypeFlagBits",
            "EXPORT_METAL_OBJECT_TYPE",
            &[
                Bitmask::entry("METAL_DEVICE", 0),
                Bitmask::entry("METAL_COMMAND_QUEUE", 1),
                Bitmask::entry("METAL_BUFFER", 2),
                Bitmask::entry("METAL_TEXTURE", 3),
                Bitmask::entry("METAL_IOSURFACE", 4),
                Bitmask::entry("METAL_SHARED_EVENT", 5),
            ],
        )
        .extension2(&VK_EXT_METAL_OBJECTS),
    ),
    Element::Struct(
        Struct::typed(
            "ExportMetalObjectCreateInfo",
            "EXPORT_METAL_OBJECT_CREATE_INFO",
            vk_ext_enum(312, 0) as _,
            StructUsage::Source,
            &[Struct::member("exportObjectType", "VkExportMetalObjectTypeFlagBitsEXT")],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ExportMetalObjectsInfo",
            "EXPORT_METAL_OBJECTS_INFO",
            vk_ext_enum(312, 1) as _,
            StructUsage::Source,
            &[],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ExportMetalDeviceInfo",
            "EXPORT_METAL_DEVICE_INFO",
            vk_ext_enum(312, 2) as _,
            StructUsage::Source,
            &[Struct::member(
                "mtlDevice",
                "*mut core::ffi::c_void", /* id<MTLDevice> */
            )],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ExportMetalCommandQueueInfo",
            "EXPORT_METAL_COMMAND_QUEUE_INFO",
            vk_ext_enum(312, 3) as _,
            StructUsage::Source,
            &[
                Struct::member("queue", "VkQueue"),
                Struct::member(
                    "mtlCommandQueue",
                    "*mut core::ffi::c_void", /* id<MTLCommandQueue> */
                ),
            ],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ExportMetalBufferInfo",
            "EXPORT_METAL_BUFFER_INFO",
            vk_ext_enum(312, 4) as _,
            StructUsage::Source,
            &[
                Struct::member("buffer", "VkBuffer"),
                Struct::member("mtlBuffer", "*mut core::ffi::c_void" /* id<MTLBuffer> */),
            ],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ImportMetalBufferInfo",
            "IMPORT_METAL_BUFFER_INFO",
            vk_ext_enum(312, 5) as _,
            StructUsage::Sink,
            &[Struct::member(
                "mtlBuffer",
                "*mut core::ffi::c_void", /* id<MTLBuffer> */
            )],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ExportMetalTextureInfo",
            "EXPORT_METAL_TEXTURE_INFO",
            vk_ext_enum(312, 6) as _,
            StructUsage::Source,
            &[
                Struct::member("image", "VkImage"),
                Struct::member("imageView", "VkImageView"),
                Struct::member("bufferView", "VkBufferView"),
                Struct::member("plane", "VkImageAspectFlagBits"),
                Struct::member("mtlTexture", "*mut core::ffi::c_void" /* id<MTLTexture> */),
            ],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ImportMetalTextureInfo",
            "IMPORT_METAL_TEXTURE_INFO",
            vk_ext_enum(312, 7) as _,
            StructUsage::Sink,
            &[
                Struct::member("plane", "VkImageAspectFlagBits"),
                Struct::member("mtlTexture", "*mut core::ffi::c_void" /* id<MTLTexture> */),
            ],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ExportMetalIOSurfaceInfo",
            "EXPORT_METAL_IO_SURFACE_INFO",
            vk_ext_enum(312, 8) as _,
            StructUsage::Source,
            &[
                Struct::member("image", "VkImage"),
                Struct::member("ioSurface", "*mut core::ffi::c_void" /* IOSurfaceRef */),
            ],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ImportMetalIOSurfaceInfo",
            "IMPORT_METAL_IO_SURFACE_INFO",
            vk_ext_enum(312, 9) as _,
            StructUsage::Sink,
            &[Struct::member(
                "ioSurface",
                "*mut core::ffi::c_void", /* IOSurfaceRef */
            )],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ExportMetalSharedEVentInfo",
            "EXPORT_METAL_SHARED_EVENT_INFO",
            vk_ext_enum(312, 10) as _,
            StructUsage::Source,
            &[
                Struct::member("semaphore", "VkSemaphore"),
                Struct::member("event", "VkEvent"),
                Struct::member("mtlSharedEvent", "*mut core::ffi::c_void" /* id<MTLSharedEvent> */),
            ],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Struct(
        Struct::typed(
            "ImportMetalSharedEventInfo",
            "IMPORT_METAL_SHARED_EVENT_INFO",
            vk_ext_enum(312, 11) as _,
            StructUsage::Sink,
            &[Struct::member(
                "mtlSharedEvent",
                "*mut core::ffi::c_void", /* id<MTLSharedEvent> */
            )],
        )
        .extensions2(&[&VK_EXT_METAL_OBJECTS]),
    ),
    Element::Command(
        Command::new(
            "ExportMetalObjects",
            &[
                ("device", "VkDevice"),
                ("pMetalObjectsInfo", "*mut VkExportMetalObjectsInfoEXT"),
            ],
        )
        .extension2(&VK_EXT_METAL_OBJECTS),
    ),
    // VK_MVK_macos_surface
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(&VK_MVK_MACOS_SURFACE)),
    Element::Bitmask(
        Bitmask::new(
            "MacOSSurfaceCreateFlags",
            "MacOSSurfaceCreateFlagBits",
            "MACOS_SURFACE_CREATE",
            &[],
        )
        .extension2(&VK_MVK_MACOS_SURFACE),
    ),
    Element::Struct(
        Struct::typed(
            "MacOSSurfaceCreateInfo",
            "MACOS_SURFACE_CREATE_INFO",
            vk_ext_enum(124, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("flags", "VkMacOSSurfaceCreateFlagsMVK"),
                Struct::member("pView", "*const core::ffi::c_void"),
            ],
        )
        .extensions2(&[&VK_MVK_MACOS_SURFACE]),
    ),
    Element::Command(
        Command::new(
            "CreateMacOSSurface",
            &[
                ("instance", "VkInstance"),
                ("pCreateInfo", "*const VkMacOSSurfaceCreateInfoMVK"),
                ("pAllocator", "*const VkAllocationCallbacks"),
                ("pSurface", "*mut VkSurfaceKHR"),
            ],
        )
        .failable()
        .static_callable()
        .extension2(&VK_MVK_MACOS_SURFACE),
    ),
    // VK_MVK_ios_surface
    Element::ExtensionHeaderConstants2(ExtensionHeaderConstants2(&VK_MVK_IOS_SURFACE)),
    Element::Bitmask(
        Bitmask::new(
            "IOSSurfaceCreateFlags",
            "IOSSurfaceCreateFlagBits",
            "IOS_SURFACE_CREATE",
            &[],
        )
        .extension2(&VK_MVK_IOS_SURFACE),
    ),
    Element::Struct(
        Struct::typed(
            "IOSSurfaceCreateInfo",
            "IOS_SURFACE_CREATE_INFO",
            vk_ext_enum(123, 0) as _,
            StructUsage::Source,
            &[
                Struct::member("flags", "VkIOSSurfaceCreateFlagsMVK"),
                Struct::member("pView", "*const core::ffi::c_void"),
            ],
        )
        .extensions2(&[&VK_MVK_IOS_SURFACE]),
    ),
    Element::Command(
        Command::new(
            "CreateIOSSurface",
            &[
                ("instance", "VkInstance"),
                ("pCreateInfo", "*const VkIOSSurfaceCreateInfoMVK"),
                ("pAllocator", "*const VkAllocationCallbacks"),
                ("pSurface", "*mut VkSurfaceKHR"),
            ],
        )
        .failable()
        .static_callable()
        .extension2(&VK_MVK_IOS_SURFACE),
    ),
];
