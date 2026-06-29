#![allow(clippy::inconsistent_digit_grouping)]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, dead_code)]

//! Vulkan API Definitions 1.4.305 with some extensions

/*
** Copyright 2025 S.Percentage.
** Original C Header: Copyright 2015-2025 The Khronos Group Inc.
**
** SPDX-License-Identifier: Apache-2.0
*/

use core::ffi::*;

// define macros

macro_rules! vk_bitmask {
    ($(#[$ty_attr: meta])* $ty_vis: vis enum $ty_name: ident { $($(#[$val_attr: meta])* $val_vis: vis $val_name: ident : $bitpos: expr),* $(,)? }) => {
        $(#[$ty_attr])* $ty_vis type $ty_name = crate::VkFlags;
        $(
            $(#[$val_attr])* $val_vis const $val_name: $ty_name = 1 << $bitpos;
        )*
    };
    (extending enum $ty_name: ident { $($(#[$val_attr: meta])* $val_vis: vis $val_name: ident : $bitpos: expr),* $(,)? }) => {
        $(
            $(#[$val_attr])* $val_vis const $val_name: $ty_name = 1 << $bitpos;
        )*
    };
    ($(#[$ty_attr: meta])* $ty_vis: vis enum64 $ty_name: ident { $($(#[$val_attr: meta])* $val_vis: vis $val_name: ident : $bitpos: expr),* $(,)? }) => {
        $(#[$ty_attr])* $ty_vis type $ty_name = crate::VkFlags64;
        $(
            $(#[$val_attr])* $val_vis const $val_name: $ty_name = 1 << $bitpos;
        )*
    };
}

// define macros end

pub type VkPipelineCacheHeaderVersion = i32;
pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE: VkPipelineCacheHeaderVersion = 1;

impl crate::vk2::VkResult {
    const fn ext_value(ext_number: u16, offset: u16) -> Self {
        Self(1000_000_000 + ((ext_number - 1) as i32 * 1_000) + offset as i32)
    }

    const fn ext_err_value(ext_number: u16, offset: u16) -> Self {
        Self(-Self::ext_value(ext_number, offset).0)
    }
}
pub const VK_ERROR_VALIDATION_FAILED_EXT: crate::vk2::VkResult = crate::vk2::VkResult(-100_0011_001);
pub const VK_ERROR_INVALID_SHADER_NV: crate::vk2::VkResult = crate::vk2::VkResult(-100_0012_000);
pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: crate::vk2::VkResult = crate::vk2::VkResult(-100_0255_000);
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
pub const VK_ERROR_INVALID_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: crate::vk2::VkResult = crate::vk2::VkResult(-100_0158_000);

pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: crate::vk2::VkStructureType = 47;
pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: crate::vk2::VkStructureType = 48;

pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: crate::vk2::VkStructureType = 100_0094_000;
pub const VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO: crate::vk2::VkStructureType = 100_0145_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: crate::vk2::VkStructureType = 100_0145_001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: crate::vk2::VkStructureType = 100_0145_002;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2: crate::vk2::VkStructureType = 100_0145_003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: crate::vk2::VkStructureType = 100_0063_000;

pub const VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: crate::vk2::VkStructureType = 100_0041_000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: crate::vk2::VkStructureType = 100_0056_000;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV: crate::vk2::VkStructureType = 100_0056_001;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: crate::vk2::VkStructureType = 100_0057_000;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: crate::vk2::VkStructureType = 100_0057_001;
pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: crate::vk2::VkStructureType = 100_0058_000;
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHX: crate::vk2::VkStructureType = 100_0060_001;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHX: crate::vk2::VkStructureType = 100_0060_002;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX: crate::vk2::VkStructureType = 100_0060_007;
pub const VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHX: crate::vk2::VkStructureType = 100_0060_008;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX: crate::vk2::VkStructureType = 100_0060_009;
pub const VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHX: crate::vk2::VkStructureType = 100_0060_010;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHX: crate::vk2::VkStructureType = 100_0060_011;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX: crate::vk2::VkStructureType = 100_0060_012;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: crate::vk2::VkStructureType = 100_0080_000;
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT: crate::vk2::VkStructureType = 100_0090_000;
pub const VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK: crate::vk2::VkStructureType = 100_0122_000;
pub const VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK: crate::vk2::VkStructureType = 100_0123_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: crate::vk2::VkStructureType =
    100_0148_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: crate::vk2::VkStructureType =
    100_0148_001;
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: crate::vk2::VkStructureType =
    100_0148_002;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: crate::vk2::VkStructureType = 100_0174_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: crate::vk2::VkStructureType =
    100_0190_000;
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: crate::vk2::VkStructureType =
    100_0190_001;

/*impl Clone for VkPhysicalDeviceMemoryProperties {
    fn clone(&self) -> Self {
        VkPhysicalDeviceMemoryProperties {
            memoryTypeCount: self.memoryTypeCount,
            memoryHeapCount: self.memoryHeapCount,
            memoryTypes: {
                let mut s = std::mem::MaybeUninit::<[_; VK_MAX_MEMORY_TYPES]>::uninit();
                for (i, e) in self.memoryTypes.iter().enumerate() {
                    unsafe {
                        (*s.as_mut_ptr())[i] = e.clone();
                    }
                }
                unsafe { s.assume_init() }
            },
            memoryHeaps: {
                let mut s = std::mem::MaybeUninit::<[_; VK_MAX_MEMORY_HEAPS]>::uninit();
                for (i, e) in self.memoryHeaps.iter().enumerate() {
                    unsafe {
                        (*s.as_mut_ptr())[i] = e.clone();
                    }
                }
                unsafe { s.assume_init() }
            },
        }
    }
}

impl Clone for VkImageBlit {
    fn clone(&self) -> Self {
        VkImageBlit {
            srcSubresource: self.srcSubresource.clone(),
            dstSubresource: self.dstSubresource.clone(),
            srcOffsets: [self.srcOffsets[0].clone(), self.srcOffsets[1].clone()],
            dstOffsets: [self.dstOffsets[0].clone(), self.dstOffsets[1].clone()],
        }
    }
}*/

// --- Extension Definitions --- //
macro_rules! ExportExtensions {
    ($fname: tt: $mname: ident) => {
        #[cfg(feature = $fname)]
        mod $mname;
        #[cfg(feature = $fname)]
        pub use self::$mname::*;
    };
}

ExportExtensions!("VK_NV_glsl_shader": glsl_shader_nv);
ExportExtensions!("VK_EXT_depth_range_unrestricted": depth_range_unrestricted_ext);
ExportExtensions!("VK_AMD_gcn_shader": gcn_shader_amd);
ExportExtensions!("VK_AMD_draw_indirect_count": draw_indirect_count_amd);
ExportExtensions!("VK_AMD_negative_viewport_height": negative_viewport_height_amd);
ExportExtensions!("VK_AMD_gpu_shader_half_float": gpu_shader_half_float_amd);
ExportExtensions!("VK_AMD_shader_ballot": shader_ballot_amd);
ExportExtensions!("VK_AMD_shader_info": shader_info_amd);
ExportExtensions!("VK_AMD_shader_image_load_store_lod": shader_image_load_store_lod_amd);
ExportExtensions!("VK_EXT_shader_group_ballot": shader_group_ballot_ext);
ExportExtensions!("VK_EXT_shader_group_vote": shader_group_vote_ext);
ExportExtensions!("VK_EXT_direct_mode_display": direct_mode_display_ext);
ExportExtensions!("VK_NV_sample_mask_override_coverage": sample_mask_override_coverage_nv);
ExportExtensions!("VK_NV_geometry_shader_passthrough": geometry_shader_passthrough_nv);
ExportExtensions!("VK_NV_viewport_array2": viewport_array2_nv);
ExportExtensions!("VK_EXT_swapchain_colorspace": swapchain_colorspace_ext);
ExportExtensions!("VK_MVK_ios_surface": ios_surface_mvk);
ExportExtensions!("VK_MVK_macos_surface": macos_surface_mvk);
ExportExtensions!("VK_EXT_external_memory_dma_buf": external_memory_dma_buf_ext);
// ExportExtensions!("VK_EXT_queue_family_foreign": queue_family_foreign);
ExportExtensions!("VK_AMD_gpu_shader_int16": gpu_shader_int16_amd);
ExportExtensions!("VK_AMD_mixed_attachment_samples": mixed_attachment_samples_amd);
ExportExtensions!("VK_AMD_shader_fragment_mask": shader_fragment_mask_amd);
ExportExtensions!("VK_EXT_post_depth_coverage": post_depth_coverage_ext);
ExportExtensions!("VK_EXT_shader_viewport_index_layer": shader_viewport_index_layer_ext);
ExportExtensions!("VK_EXT_full_screen_exclusive": full_screen_exclusive_ext);
ExportExtensions!("VK_EXT_metal_objects": metal_objects_ext);
ExportExtensions!("VK_KHR_maintenance7": maintenance7_khr);
ExportExtensions!("VK_KHR_maintenance8": maintenance8_khr);

// Promoted Extensions (1.2)
ExportExtensions!("VK_KHR_create_renderpass2": create_renderpass2_khr);
ExportExtensions!("VK_KHR_depth_stencil_resolve": depth_stencil_resolve_khr);
ExportExtensions!("VK_EXT_descriptor_indexing": descriptor_indexing_ext);
ExportExtensions!("VK_KHR_shader_float_controls": shader_float_controls_khr);

// Promoted Extensions (1.3)
ExportExtensions!("VK_KHR_maintenance4": maintenance4_khr);
ExportExtensions!("VK_KHR_dynamic_rendering": dynamic_rendering_khr);
ExportExtensions!("VK_KHR_copy_commands2": copy_commands2_khr);
ExportExtensions!("VK_KHR_format_feature_flags2": format_feature_flags2_khr);

// Promoted Extensions (1.4)
ExportExtensions!("VK_KHR_dynamic_rendering_local_read": dynamic_rendering_local_read_khr);
ExportExtensions!("VK_KHR_index_type_uint8": index_type_uint8_khr);
ExportExtensions!("VK_KHR_line_rasterization": line_rasterization_khr);
ExportExtensions!("VK_KHR_load_store_op_none": load_store_op_none_khr);
ExportExtensions!("VK_KHR_maintenance5": maintenance5_khr);
ExportExtensions!("VK_KHR_maintenance6": maintenance6_khr);
ExportExtensions!("VK_KHR_map_memory2": map_memory2_khr);
ExportExtensions!("VK_KHR_push_descriptor": push_descriptor_khr);
ExportExtensions!("VK_KHR_shader_expect_assume": shader_expect_assume_khr);
ExportExtensions!("VK_KHR_shader_float_controls2": shader_float_controls2_khr);
ExportExtensions!("VK_KHR_shader_subgroup_rotate": shader_subgroup_rotate_khr);
ExportExtensions!("VK_EXT_host_image_copy": host_image_copy_ext);
ExportExtensions!("VK_EXT_pipeline_protected_access": pipeline_protected_access_ext);
ExportExtensions!("VK_EXT_pipeline_robustness": pipeline_robustness_ext);
