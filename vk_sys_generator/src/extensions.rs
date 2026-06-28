use crate::{parts::*, vk_ext_enum};

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
            Enum::member("ZERO", vk_ext_enum(149, 0) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SRC", vk_ext_enum(149, 1) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DST", vk_ext_enum(149, 2) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SRC_OVER", vk_ext_enum(149, 3) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DST_OVER", vk_ext_enum(149, 4) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SRC_IN", vk_ext_enum(149, 5) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DST_IN", vk_ext_enum(149, 6) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SRC_OUT", vk_ext_enum(149, 7) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DST_OUT", vk_ext_enum(149, 8) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SRC_ATOP", vk_ext_enum(149, 9) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DST_ATOP", vk_ext_enum(149, 10) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("XOR", vk_ext_enum(149, 11) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("MULTIPLY", vk_ext_enum(149, 12) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SCREEN", vk_ext_enum(149, 13) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("OVERLAY", vk_ext_enum(149, 14) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DARKEN", vk_ext_enum(149, 15) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("LIGHTEN", vk_ext_enum(149, 16) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("COLORDODGE", vk_ext_enum(149, 17) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("COLORBURN", vk_ext_enum(149, 18) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HARDLIGHT", vk_ext_enum(149, 19) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("SOFTLIGHT", vk_ext_enum(149, 20) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("DIFFERENCE", vk_ext_enum(149, 21) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("EXCLUSION", vk_ext_enum(149, 22) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("INVERT", vk_ext_enum(149, 23) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("INVERT_RGB", vk_ext_enum(149, 24) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("LINEARDODGE", vk_ext_enum(149, 25) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("LINEARBURN", vk_ext_enum(149, 26) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("VIVIDLIGHT", vk_ext_enum(149, 27) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("LINEARLIGHT", vk_ext_enum(149, 28) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("PINLIGHT", vk_ext_enum(149, 29) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HARDMIX", vk_ext_enum(149, 30) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HSL_HUE", vk_ext_enum(149, 31) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HSL_SATURATION", vk_ext_enum(149, 32) as _)
                .extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HSL_COLOR", vk_ext_enum(149, 33) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("HSL_LUMINOSITY", vk_ext_enum(149, 34) as _)
                .extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("PLUS", vk_ext_enum(149, 35) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("PLUS_CLAMPED", vk_ext_enum(149, 36) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("PLUS_CLAMPED_ALHPA", vk_ext_enum(149, 37) as _)
                .extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("PLUS_DARKER", vk_ext_enum(149, 38) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("MINUS", vk_ext_enum(149, 39) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("MINUS_CLAMPED", vk_ext_enum(149, 40) as _)
                .extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("CONTRAST", vk_ext_enum(149, 41) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("INVERT_OVG", vk_ext_enum(149, 42) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("RED", vk_ext_enum(149, 43) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("GREEN", vk_ext_enum(149, 44) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
            Enum::member("BLUE", vk_ext_enum(149, 45) as _).extension("VK_EXT_blend_operation_advanced", "EXT"),
        ],
    )),
    Element::Enum(
        Enum::new(
            "BlendOverlap",
            "BLEND_OVERLAP",
            &[
                Enum::member("UNCORRELATED", 0).extension("VK_EXT_blend_operation_advanced", "EXT"),
                Enum::member("DISJOINT", 1).extension("VK_EXT_blend_operation_advanced", "EXT"),
                Enum::member("CONJOINT", 2).extension("VK_EXT_blend_operation_advanced", "EXT"),
            ],
        )
        .extension("VK_EXT_blend_operation_advanced", "EXT"),
    ),
    Element::Enum(
        Enum::new(
            "ValidationCacheHeaderVersion",
            "VALIDATION_CACHE_HEADER_VERSION",
            &[Enum::member("ONE", 1).extension("VK_EXT_validation_cache", "EXT")],
        )
        .extension("VK_EXT_validation_cache", "EXT"),
    ),
    Element::Enum(
        Enum::new(
            "ValidationCheck",
            "VALIDATION_CHECK",
            &[
                Enum::member("ALL", 0).extension("VK_EXT_validation_flags", "EXT"),
                Enum::member("SHADERS", 1).extension("VK_EXT_validation_flags", "EXT"),
            ],
        )
        .extension("VK_EXT_validation_flags", "EXT"),
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
                Enum::member("BOOL32", 0).extension("VK_EXT_layer_settings", "EXT"),
                Enum::member("INT32", 1).extension("VK_EXT_layer_settings", "EXT"),
                Enum::member("INT64", 2).extension("VK_EXT_layer_settings", "EXT"),
                Enum::member("UINT32", 3).extension("VK_EXT_layer_settings", "EXT"),
                Enum::member("UINT64", 4).extension("VK_EXT_layer_settings", "EXT"),
                Enum::member("FLOAT32", 5).extension("VK_EXT_layer_settings", "EXT"),
                Enum::member("FLOAT64", 6).extension("VK_EXT_layer_settings", "EXT"),
                Enum::member("STRING", 7).extension("VK_EXT_layer_settings", "EXT"),
            ],
        )
        .extension("VK_EXT_layer_settings", "EXT"),
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
                Bitmask::entry("HOST_ALLOCATION_BIT_EXT", 7).extension("EXT", "external_memory_host"),
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
];
