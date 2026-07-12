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

ExportExtensions!("VK_AMD_shader_info": shader_info_amd);
ExportExtensions!("VK_EXT_full_screen_exclusive": full_screen_exclusive_ext);
ExportExtensions!("VK_KHR_maintenance7": maintenance7_khr);
ExportExtensions!("VK_KHR_maintenance8": maintenance8_khr);

// Promoted Extensions (1.2)
ExportExtensions!("VK_KHR_create_renderpass2": create_renderpass2_khr);
ExportExtensions!("VK_KHR_depth_stencil_resolve": depth_stencil_resolve_khr);
ExportExtensions!("VK_EXT_descriptor_indexing": descriptor_indexing_ext);

// Promoted Extensions (1.3)
ExportExtensions!("VK_KHR_maintenance4": maintenance4_khr);
ExportExtensions!("VK_KHR_dynamic_rendering": dynamic_rendering_khr);
ExportExtensions!("VK_KHR_copy_commands2": copy_commands2_khr);

// Promoted Extensions (1.4)
ExportExtensions!("VK_KHR_dynamic_rendering_local_read": dynamic_rendering_local_read_khr);
ExportExtensions!("VK_KHR_index_type_uint8": index_type_uint8_khr);
ExportExtensions!("VK_KHR_line_rasterization": line_rasterization_khr);
ExportExtensions!("VK_KHR_maintenance5": maintenance5_khr);
ExportExtensions!("VK_KHR_maintenance6": maintenance6_khr);
ExportExtensions!("VK_KHR_map_memory2": map_memory2_khr);
ExportExtensions!("VK_KHR_push_descriptor": push_descriptor_khr);
ExportExtensions!("VK_KHR_shader_subgroup_rotate": shader_subgroup_rotate_khr);
ExportExtensions!("VK_EXT_host_image_copy": host_image_copy_ext);
ExportExtensions!("VK_EXT_pipeline_protected_access": pipeline_protected_access_ext);
ExportExtensions!("VK_EXT_pipeline_robustness": pipeline_robustness_ext);
