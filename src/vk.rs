#![allow(clippy::inconsistent_digit_grouping)]
//! Vulkan API Definitions 1.1.70.1

/*
** Copyright (c) 2015-2018 The Khronos Group Inc.
**
** Licensed under the Apache License, Version 2.0 (the "License");
** you may not use this file except in compliance with the License.
** You may obtain a copy of the License at
**
**     http://www.apache.org/licenses/LICENSE-2.0
**
** Unless required by applicable law or agreed to in writing, software
** distributed under the License is distributed on an "AS IS" BASIS,
** WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
** See the License for the specific language governing permissions and
** limitations under the License.
*/

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, dead_code)]

use crate::{ffi_helper::FixedCStrBuffer, StaticCallable, VulkanStructure, PFN};
use cfg_if::cfg_if;
use derives::vk_raw_handle;
use libc::*;
use std;

#[inline]
pub const fn VK_MAKE_VERSION(major: u16, minor: u16, patch: u16) -> u32 {
    ((major as u32) << 22) | ((minor as u32) << 16) | patch as u32
}

/// Vulkan 1.0 version number
pub const VK_API_VERSION_1_0: u32 = VK_MAKE_VERSION(1, 0, 0);

#[inline]
pub const fn VK_MAJOR_VERSION(v: u32) -> u16 {
    (v >> 22) as _
}

#[inline]
pub const fn VK_MINOR_VERSION(v: u32) -> u16 {
    ((v >> 22) & 0x3ff) as _
}

#[inline]
pub const fn VK_PATCH_VERSION(v: u32) -> u16 {
    (v & 0xfff) as _
}

/// (major, minor, patch)
#[inline]
pub const fn vk_deserialize_version(v: u32) -> (u16, u16, u16) {
    (VK_MAJOR_VERSION(v), VK_MINOR_VERSION(v), VK_PATCH_VERSION(v))
}

// define macros

#[inline]
const fn ext_enum_value(ext_number: u16, index: u16) -> u64 {
    1000_000_000 + ((ext_number - 1) as u64 * 1_000) + index as u64
}

macro_rules! vk_bitmask {
    ($(#[$ty_attr: meta])* $ty_vis: vis enum $ty_name: ident { $($(#[$val_attr: meta])* $val_vis: vis $val_name: ident : $bitpos: expr),* }) => {
        $(#[$ty_attr])* $ty_vis type $ty_name = VkFlags;
        $(
            $(#[$val_attr])* $val_vis const $val_name: $ty_name = 1 << $bitpos;
        )*
    };
    (extending enum $ty_name: ident { $($(#[$val_attr: meta])* $val_vis: vis $val_name: ident : $bitpos: expr),* }) => {
        $(
            $(#[$val_attr])* $val_vis const $val_name: $ty_name = 1 << $bitpos;
        )*
    }
}

// define macros end

/// Version of this file
pub const VK_HEADER_VERSION: u32 = 70;

pub type VkFlags = u32;
pub type VkBool32 = u32;
pub type VkDeviceSize = u64;
pub type VkSampleMask = u32;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_INSTANCE)]
pub struct VkInstance(pub *mut c_void);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_PHYSICAL_DEVICE)]
pub struct VkPhysicalDevice(pub *mut c_void);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_DEVICE)]
pub struct VkDevice(pub *mut c_void);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_QUEUE)]
pub struct VkQueue(pub *mut c_void);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_SEMAPHORE)]
pub struct VkSemaphore(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_COMMAND_BUFFER)]
pub struct VkCommandBuffer(pub *mut c_void);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_FENCE)]
pub struct VkFence(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_DEVICE_MEMORY)]
pub struct VkDeviceMemory(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_BUFFER)]
pub struct VkBuffer(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_IMAGE)]
pub struct VkImage(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_EVENT)]
pub struct VkEvent(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_QUERY_POOL)]
pub struct VkQueryPool(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_BUFFER_VIEW)]
pub struct VkBufferView(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_IMAGE_VIEW)]
pub struct VkImageView(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_SHADER_MODULE)]
pub struct VkShaderModule(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_PIPELINE_CACHE)]
pub struct VkPipelineCache(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_PIPELINE_LAYOUT)]
pub struct VkPipelineLayout(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_RENDER_PASS)]
pub struct VkRenderPass(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_PIPELINE)]
pub struct VkPipeline(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT)]
pub struct VkDescriptorSetLayout(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_SAMPLER)]
pub struct VkSampler(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_DESCRIPTOR_POOL)]
pub struct VkDescriptorPool(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_DESCRIPTOR_SET)]
pub struct VkDescriptorSet(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_FRAMEBUFFER)]
pub struct VkFramebuffer(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_COMMAND_POOL)]
pub struct VkCommandPool(pub u64);

pub const VK_LOD_CLAMP_NONE: f32 = 1000.0;
pub const VK_REMAINING_MIP_LEVELS: u32 = !0;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0;
pub const VK_WHOLE_SIZE: u64 = !0;
pub const VK_ATTACHMENT_UNUSED: u32 = !0;
pub const VK_TRUE: VkBool32 = 1;
pub const VK_FALSE: VkBool32 = 0;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0;
pub const VK_SUBPASS_EXTERNAL: u32 = !0;
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;

pub type VkPipelineCacheHeaderVersion = i32;
pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE: VkPipelineCacheHeaderVersion = 1;

pub type VkResult = i32;
pub const VK_SUCCESS: VkResult = 0;
pub const VK_NOT_READY: VkResult = 1;
pub const VK_TIMEOUT: VkResult = 2;
pub const VK_EVENT_SET: VkResult = 3;
pub const VK_EVENT_RESET: VkResult = 4;
pub const VK_INCOMPLETE: VkResult = 5;
pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = -1;
pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = -2;
pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = -3;
pub const VK_ERROR_DEVICE_LOST: VkResult = -4;
pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = -5;
pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = -6;
pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = -7;
pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = -8;
pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = -9;
pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = -10;
pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = -11;
pub const VK_ERROR_FRAGMENTED_POOL: VkResult = -12;
pub const VK_ERROR_UNKNOWN: VkResult = -13;
pub const VK_ERROR_SURFACE_LOST_KHR: VkResult = -100_0000_000;
pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult = -100_0000_001;
pub const VK_SUBOPTIMAL_KHR: VkResult = 100_0001_003;
pub const VK_ERROR_OUT_OF_DATE_KHR: VkResult = -100_0001_004;
pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: VkResult = -100_0003_001;
pub const VK_ERROR_VALIDATION_FAILED_EXT: VkResult = -100_0011_001;
pub const VK_ERROR_INVALID_SHADER_NV: VkResult = -100_0012_000;
pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: VkResult = -100_0255_000;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
pub const VK_ERROR_INVALID_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: VkResult = -1000158000;

pub type VkStructureType = i32;
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = 0;
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = 1;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = 2;
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = 3;
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = 4;
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = 5;
pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: VkStructureType = 6;
pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: VkStructureType = 7;
pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = 8;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: VkStructureType = 9;
pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: VkStructureType = 10;
pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: VkStructureType = 11;
pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = 12;
pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: VkStructureType = 13;
pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: VkStructureType = 14;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: VkStructureType = 15;
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType = 16;
pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: VkStructureType = 17;
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType = 18;
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: VkStructureType = 19;
pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: VkStructureType = 20;
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: VkStructureType = 21;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: VkStructureType = 22;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: VkStructureType = 23;
pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: VkStructureType = 24;
pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: VkStructureType = 25;
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: VkStructureType = 26;
pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: VkStructureType = 27;
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: VkStructureType = 28;
pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = 29;
pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: VkStructureType = 30;
pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: VkStructureType = 31;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType = 32;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: VkStructureType = 33;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType = 34;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: VkStructureType = 35;
pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: VkStructureType = 36;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: VkStructureType = 37;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: VkStructureType = 38;
pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType = 39;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType = 40;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: VkStructureType = 41;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = 42;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: VkStructureType = 43;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: VkStructureType = 44;
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: VkStructureType = 45;
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: VkStructureType = 46;
pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: VkStructureType = 47;
pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: VkStructureType = 48;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: VkStructureType = 100_0094_000;
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO: VkStructureType = 100_0157_000;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO: VkStructureType = 100_0157_001;
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS: VkStructureType = 100_0127_000;
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO: VkStructureType = 100_0127_001;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 100_0146_000;
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 100_0146_001;
pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 100_0146_002;
pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2: VkStructureType = 100_0146_003;
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: VkStructureType = 100_0146_004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: VkStructureType = 100_0117_000;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: VkStructureType = 100_0117_001;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO: VkStructureType = 100_0117_002;
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: VkStructureType = 100_0117_003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: VkStructureType = 100_0120_000;
pub const VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO: VkStructureType = 100_0145_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: VkStructureType = 100_0145_001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: VkStructureType = 100_0145_002;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2: VkStructureType = 100_0145_003;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO: VkStructureType = 100_0156_000;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO: VkStructureType = 100_0156_001;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO: VkStructureType = 100_0156_002;
pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: VkStructureType = 100_0156_003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: VkStructureType = 100_0156_004;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: VkStructureType = 100_0156_005;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: VkStructureType = 100_0071_000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES: VkStructureType = 100_0071_001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: VkStructureType = 100_0071_002;
pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES: VkStructureType = 100_0071_003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES: VkStructureType = 100_0071_004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: VkStructureType = 100_0112_000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES: VkStructureType = 100_0112_001;
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO: VkStructureType = 100_0113_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: VkStructureType = 100_0168_000;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT: VkStructureType = 100_0168_001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: VkStructureType = 100_0063_000;
cfg_if! {
    if #[cfg(feature = "VK_EXT_image_drm_format_modifier")] {
        pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: VkStructureType = 1000158000;
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: VkStructureType = 1000158002;
        pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: VkStructureType = 1000158003;
        pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: VkStructureType = 1000158004;
        pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: VkStructureType = 1000158005;
        #[cfg(feature = "VK_KHR_format_feature_flags2")]
        pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: VkStructureType = 1000158006;
    }
}

pub const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = 100_0001_000;
pub const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR: VkStructureType = 100_0001_001;
pub const VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR: VkStructureType = 100_0002_000;
pub const VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR: VkStructureType = 100_0002_001;
pub const VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR: VkStructureType = 100_0003_000;
pub const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR: VkStructureType = 100_0004_000;
pub const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR: VkStructureType = 100_0005_000;
pub const VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR: VkStructureType = 100_0006_000;
pub const VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR: VkStructureType = 100_0007_000;
pub const VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR: VkStructureType = 100_0008_000;
pub const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR: VkStructureType = 100_0009_000;
pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: VkStructureType = 100_0011_000;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: VkStructureType = 100_0018_000;
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT: VkStructureType = 100_0022_000;
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT: VkStructureType = 100_0022_001;
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT: VkStructureType = 100_0022_002;
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: VkStructureType = 100_0026_000;
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: VkStructureType = 100_0026_001;
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: VkStructureType = 100_0026_002;
pub const VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: VkStructureType = 100_0041_000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: VkStructureType = 100_0056_000;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV: VkStructureType = 100_0056_001;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: VkStructureType = 100_0057_000;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: VkStructureType = 100_0057_001;
pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: VkStructureType = 100_0058_000;
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHX: VkStructureType = 100_0060_001;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHX: VkStructureType = 100_0060_002;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX: VkStructureType = 100_0060_007;
pub const VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHX: VkStructureType = 100_0060_008;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX: VkStructureType = 100_0060_009;
pub const VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHX: VkStructureType = 100_0060_010;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHX: VkStructureType = 100_0060_011;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX: VkStructureType = 100_0060_012;
pub const VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT: VkStructureType = 100_0061_000;
pub const VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN: VkStructureType = 100_0062_000;
pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: VkStructureType = 100_0075_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: VkStructureType = 100_0080_000;
pub const VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR: VkStructureType = 100_0084_000;
pub const VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX: VkStructureType = 100_0086_000;
pub const VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX: VkStructureType = 100_0086_001;
pub const VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX: VkStructureType = 100_0086_002;
pub const VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX: VkStructureType = 100_0086_003;
pub const VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX: VkStructureType = 100_0086_004;
pub const VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX: VkStructureType = 100_0086_005;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: VkStructureType = 100_0087_000;
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT: VkStructureType = 100_0090_000;
pub const VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT: VkStructureType = 100_0091_000;
pub const VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT: VkStructureType = 100_0091_001;
pub const VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT: VkStructureType = 100_0091_002;
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT: VkStructureType = 100_0091_003;
pub const VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE: VkStructureType = 100_0092_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: VkStructureType =
    100_0097_000;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: VkStructureType = 100_0098_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: VkStructureType = 100_0099_000;
pub const VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: VkStructureType = 100_0099_001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: VkStructureType = 100_0101_000;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: VkStructureType = 100_0101_001;
pub const VK_STRUCTURE_TYPE_HDR_METADATA_EXT: VkStructureType = 100_0105_000;
pub const VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: VkStructureType = 100_0111_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: VkStructureType = 100_0119_000;
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR: VkStructureType = 100_0119_001;
pub const VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR: VkStructureType = 100_0119_002;
pub const VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK: VkStructureType = 100_0122_000;
pub const VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK: VkStructureType = 100_0123_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: VkStructureType = 100_0130_000;
pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: VkStructureType = 100_0130_001;
pub const VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT: VkStructureType = 100_0143_000;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: VkStructureType = 100_0143_001;
pub const VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: VkStructureType = 100_0143_002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: VkStructureType = 100_0143_003;
pub const VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT: VkStructureType = 100_0143_004;
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR: VkStructureType = 100_0147_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: VkStructureType = 100_0148_000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: VkStructureType = 100_0148_001;
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: VkStructureType = 100_0148_002;
pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: VkStructureType = 100_0149_000;
pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: VkStructureType = 100_0152_000;
pub const VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT: VkStructureType = 100_0160_000;
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: VkStructureType = 100_0160_001;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: VkStructureType = 100_0174_000;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT: VkStructureType = 100_0178_000;
pub const VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT: VkStructureType = 100_0178_001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: VkStructureType = 100_0178_002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: VkStructureType = 100_0190_000;
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: VkStructureType = 100_0190_001;

pub type VkSystemAllocationScope = i32;
pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: VkSystemAllocationScope = 0;
pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: VkSystemAllocationScope = 1;
pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: VkSystemAllocationScope = 2;
pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: VkSystemAllocationScope = 3;
pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: VkSystemAllocationScope = 4;

pub type VkInternalAllocationType = i32;
pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: VkInternalAllocationType = 0;

pub type VkFormat = i32;
pub const VK_FORMAT_UNDEFINED: VkFormat = 0;
pub const VK_FORMAT_R4G4_UNORM_PACK8: VkFormat = 1;
pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: VkFormat = 2;
pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: VkFormat = 3;
pub const VK_FORMAT_R5G6B5_UNORM_PACK16: VkFormat = 4;
pub const VK_FORMAT_B5G6R5_UNORM_PACK16: VkFormat = 5;
pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: VkFormat = 6;
pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: VkFormat = 7;
pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: VkFormat = 8;
pub const VK_FORMAT_R8_UNORM: VkFormat = 9;
pub const VK_FORMAT_R8_SNORM: VkFormat = 10;
pub const VK_FORMAT_R8_USCALED: VkFormat = 11;
pub const VK_FORMAT_R8_SSCALED: VkFormat = 12;
pub const VK_FORMAT_R8_UINT: VkFormat = 13;
pub const VK_FORMAT_R8_SINT: VkFormat = 14;
pub const VK_FORMAT_R8_SRGB: VkFormat = 15;
pub const VK_FORMAT_R8G8_UNORM: VkFormat = 16;
pub const VK_FORMAT_R8G8_SNORM: VkFormat = 17;
pub const VK_FORMAT_R8G8_USCALED: VkFormat = 18;
pub const VK_FORMAT_R8G8_SSCALED: VkFormat = 19;
pub const VK_FORMAT_R8G8_UINT: VkFormat = 20;
pub const VK_FORMAT_R8G8_SINT: VkFormat = 21;
pub const VK_FORMAT_R8G8_SRGB: VkFormat = 22;
pub const VK_FORMAT_R8G8B8_UNORM: VkFormat = 23;
pub const VK_FORMAT_R8G8B8_SNORM: VkFormat = 24;
pub const VK_FORMAT_R8G8B8_USCALED: VkFormat = 25;
pub const VK_FORMAT_R8G8B8_SSCALED: VkFormat = 26;
pub const VK_FORMAT_R8G8B8_UINT: VkFormat = 27;
pub const VK_FORMAT_R8G8B8_SINT: VkFormat = 28;
pub const VK_FORMAT_R8G8B8_SRGB: VkFormat = 29;
pub const VK_FORMAT_B8G8R8_UNORM: VkFormat = 30;
pub const VK_FORMAT_B8G8R8_SNORM: VkFormat = 31;
pub const VK_FORMAT_B8G8R8_USCALED: VkFormat = 32;
pub const VK_FORMAT_B8G8R8_SSCALED: VkFormat = 33;
pub const VK_FORMAT_B8G8R8_UINT: VkFormat = 34;
pub const VK_FORMAT_B8G8R8_SINT: VkFormat = 35;
pub const VK_FORMAT_B8G8R8_SRGB: VkFormat = 36;
pub const VK_FORMAT_R8G8B8A8_UNORM: VkFormat = 37;
pub const VK_FORMAT_R8G8B8A8_SNORM: VkFormat = 38;
pub const VK_FORMAT_R8G8B8A8_USCALED: VkFormat = 39;
pub const VK_FORMAT_R8G8B8A8_SSCALED: VkFormat = 40;
pub const VK_FORMAT_R8G8B8A8_UINT: VkFormat = 41;
pub const VK_FORMAT_R8G8B8A8_SINT: VkFormat = 42;
pub const VK_FORMAT_R8G8B8A8_SRGB: VkFormat = 43;
pub const VK_FORMAT_B8G8R8A8_UNORM: VkFormat = 44;
pub const VK_FORMAT_B8G8R8A8_SNORM: VkFormat = 45;
pub const VK_FORMAT_B8G8R8A8_USCALED: VkFormat = 46;
pub const VK_FORMAT_B8G8R8A8_SSCALED: VkFormat = 47;
pub const VK_FORMAT_B8G8R8A8_UINT: VkFormat = 48;
pub const VK_FORMAT_B8G8R8A8_SINT: VkFormat = 49;
pub const VK_FORMAT_B8G8R8A8_SRGB: VkFormat = 50;
pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: VkFormat = 51;
pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: VkFormat = 52;
pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: VkFormat = 53;
pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: VkFormat = 54;
pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: VkFormat = 55;
pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: VkFormat = 56;
pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: VkFormat = 57;
pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: VkFormat = 58;
pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: VkFormat = 59;
pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: VkFormat = 60;
pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: VkFormat = 61;
pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: VkFormat = 62;
pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: VkFormat = 63;
pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: VkFormat = 64;
pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: VkFormat = 65;
pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: VkFormat = 66;
pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: VkFormat = 67;
pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: VkFormat = 68;
pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: VkFormat = 69;
pub const VK_FORMAT_R16_UNORM: VkFormat = 70;
pub const VK_FORMAT_R16_SNORM: VkFormat = 71;
pub const VK_FORMAT_R16_USCALED: VkFormat = 72;
pub const VK_FORMAT_R16_SSCALED: VkFormat = 73;
pub const VK_FORMAT_R16_UINT: VkFormat = 74;
pub const VK_FORMAT_R16_SINT: VkFormat = 75;
pub const VK_FORMAT_R16_SFLOAT: VkFormat = 76;
pub const VK_FORMAT_R16G16_UNORM: VkFormat = 77;
pub const VK_FORMAT_R16G16_SNORM: VkFormat = 78;
pub const VK_FORMAT_R16G16_USCALED: VkFormat = 79;
pub const VK_FORMAT_R16G16_SSCALED: VkFormat = 80;
pub const VK_FORMAT_R16G16_UINT: VkFormat = 81;
pub const VK_FORMAT_R16G16_SINT: VkFormat = 82;
pub const VK_FORMAT_R16G16_SFLOAT: VkFormat = 83;
pub const VK_FORMAT_R16G16B16_UNORM: VkFormat = 84;
pub const VK_FORMAT_R16G16B16_SNORM: VkFormat = 85;
pub const VK_FORMAT_R16G16B16_USCALED: VkFormat = 86;
pub const VK_FORMAT_R16G16B16_SSCALED: VkFormat = 87;
pub const VK_FORMAT_R16G16B16_UINT: VkFormat = 88;
pub const VK_FORMAT_R16G16B16_SINT: VkFormat = 89;
pub const VK_FORMAT_R16G16B16_SFLOAT: VkFormat = 90;
pub const VK_FORMAT_R16G16B16A16_UNORM: VkFormat = 91;
pub const VK_FORMAT_R16G16B16A16_SNORM: VkFormat = 92;
pub const VK_FORMAT_R16G16B16A16_USCALED: VkFormat = 93;
pub const VK_FORMAT_R16G16B16A16_SSCALED: VkFormat = 94;
pub const VK_FORMAT_R16G16B16A16_UINT: VkFormat = 95;
pub const VK_FORMAT_R16G16B16A16_SINT: VkFormat = 96;
pub const VK_FORMAT_R16G16B16A16_SFLOAT: VkFormat = 97;
pub const VK_FORMAT_R32_UINT: VkFormat = 98;
pub const VK_FORMAT_R32_SINT: VkFormat = 99;
pub const VK_FORMAT_R32_SFLOAT: VkFormat = 100;
pub const VK_FORMAT_R32G32_UINT: VkFormat = 101;
pub const VK_FORMAT_R32G32_SINT: VkFormat = 102;
pub const VK_FORMAT_R32G32_SFLOAT: VkFormat = 103;
pub const VK_FORMAT_R32G32B32_UINT: VkFormat = 104;
pub const VK_FORMAT_R32G32B32_SINT: VkFormat = 105;
pub const VK_FORMAT_R32G32B32_SFLOAT: VkFormat = 106;
pub const VK_FORMAT_R32G32B32A32_UINT: VkFormat = 107;
pub const VK_FORMAT_R32G32B32A32_SINT: VkFormat = 108;
pub const VK_FORMAT_R32G32B32A32_SFLOAT: VkFormat = 109;
pub const VK_FORMAT_R64_UINT: VkFormat = 110;
pub const VK_FORMAT_R64_SINT: VkFormat = 111;
pub const VK_FORMAT_R64_SFLOAT: VkFormat = 112;
pub const VK_FORMAT_R64G64_UINT: VkFormat = 113;
pub const VK_FORMAT_R64G64_SINT: VkFormat = 114;
pub const VK_FORMAT_R64G64_SFLOAT: VkFormat = 115;
pub const VK_FORMAT_R64G64B64_UINT: VkFormat = 116;
pub const VK_FORMAT_R64G64B64_SINT: VkFormat = 117;
pub const VK_FORMAT_R64G64B64_SFLOAT: VkFormat = 118;
pub const VK_FORMAT_R64G64B64A64_UINT: VkFormat = 119;
pub const VK_FORMAT_R64G64B64A64_SINT: VkFormat = 120;
pub const VK_FORMAT_R64G64B64A64_SFLOAT: VkFormat = 121;
pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: VkFormat = 122;
pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: VkFormat = 123;
pub const VK_FORMAT_D16_UNORM: VkFormat = 124;
pub const VK_FORMAT_X8_D24_UNORM_PACK32: VkFormat = 125;
pub const VK_FORMAT_D32_SFLOAT: VkFormat = 126;
pub const VK_FORMAT_S8_UINT: VkFormat = 127;
pub const VK_FORMAT_D16_UNORM_S8_UINT: VkFormat = 128;
pub const VK_FORMAT_D24_UNORM_S8_UINT: VkFormat = 129;
pub const VK_FORMAT_D32_SFLOAT_S8_UINT: VkFormat = 130;
pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: VkFormat = 131;
pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: VkFormat = 132;
pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: VkFormat = 133;
pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: VkFormat = 134;
pub const VK_FORMAT_BC2_UNORM_BLOCK: VkFormat = 135;
pub const VK_FORMAT_BC2_SRGB_BLOCK: VkFormat = 136;
pub const VK_FORMAT_BC3_UNORM_BLOCK: VkFormat = 137;
pub const VK_FORMAT_BC3_SRGB_BLOCK: VkFormat = 138;
pub const VK_FORMAT_BC4_UNORM_BLOCK: VkFormat = 139;
pub const VK_FORMAT_BC4_SNORM_BLOCK: VkFormat = 140;
pub const VK_FORMAT_BC5_UNORM_BLOCK: VkFormat = 141;
pub const VK_FORMAT_BC5_SNORM_BLOCK: VkFormat = 142;
pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: VkFormat = 143;
pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: VkFormat = 144;
pub const VK_FORMAT_BC7_UNORM_BLOCK: VkFormat = 145;
pub const VK_FORMAT_BC7_SRGB_BLOCK: VkFormat = 146;
pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: VkFormat = 147;
pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: VkFormat = 148;
pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: VkFormat = 149;
pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: VkFormat = 150;
pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: VkFormat = 151;
pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: VkFormat = 152;
pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: VkFormat = 153;
pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: VkFormat = 154;
pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: VkFormat = 155;
pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: VkFormat = 156;
pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: VkFormat = 157;
pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: VkFormat = 158;
pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: VkFormat = 159;
pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: VkFormat = 160;
pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: VkFormat = 161;
pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: VkFormat = 162;
pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: VkFormat = 163;
pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: VkFormat = 164;
pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: VkFormat = 165;
pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: VkFormat = 166;
pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: VkFormat = 167;
pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: VkFormat = 168;
pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: VkFormat = 169;
pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: VkFormat = 170;
pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: VkFormat = 171;
pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: VkFormat = 172;
pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: VkFormat = 173;
pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: VkFormat = 174;
pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: VkFormat = 175;
pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: VkFormat = 176;
pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: VkFormat = 177;
pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: VkFormat = 178;
pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: VkFormat = 179;
pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: VkFormat = 180;
pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: VkFormat = 181;
pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: VkFormat = 182;
pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: VkFormat = 183;
pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: VkFormat = 184;
pub const VK_FORMAT_G8B8G8R8_422_UNORM: VkFormat = 100_0156_000;
pub const VK_FORMAT_B8G8R8G8_422_UNORM: VkFormat = 100_0156_001;
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM: VkFormat = 100_0156_002;
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM: VkFormat = 100_0156_003;
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM: VkFormat = 100_0156_004;
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM: VkFormat = 100_0156_005;
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM: VkFormat = 100_0156_006;
pub const VK_FORMAT_R10X6_UNORM_PACK16: VkFormat = 100_0156_007;
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: VkFormat = 100_0156_008;
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: VkFormat = 100_0156_009;
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: VkFormat = 100_0156_010;
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: VkFormat = 100_0156_011;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: VkFormat = 100_0156_012;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: VkFormat = 100_0156_013;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: VkFormat = 100_0156_014;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: VkFormat = 100_0156_015;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: VkFormat = 100_0156_016;
pub const VK_FORMAT_R12X4_UNORM_PACK16: VkFormat = 100_0156_017;
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: VkFormat = 100_0156_018;
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: VkFormat = 100_0156_019;
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: VkFormat = 100_0156_020;
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: VkFormat = 100_0156_021;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: VkFormat = 100_0156_022;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: VkFormat = 100_0156_023;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: VkFormat = 100_0156_024;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: VkFormat = 100_0156_025;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: VkFormat = 100_0156_026;
pub const VK_FORMAT_G16B16G16R16_422_UNORM: VkFormat = 100_0156_027;
pub const VK_FORMAT_B16G16R16G16_422_UNORM: VkFormat = 100_0156_028;
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM: VkFormat = 100_0156_029;
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM: VkFormat = 100_0156_030;
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM: VkFormat = 100_0156_031;
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM: VkFormat = 100_0156_032;
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM: VkFormat = 100_0156_033;
pub const VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG: VkFormat = 100_0054_000;
pub const VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG: VkFormat = 100_0054_001;
pub const VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG: VkFormat = 100_0054_002;
pub const VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG: VkFormat = 100_0054_003;
pub const VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG: VkFormat = 100_0054_004;
pub const VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG: VkFormat = 100_0054_005;
pub const VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG: VkFormat = 100_0054_006;
pub const VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG: VkFormat = 100_0054_007;

pub type VkImageType = i32;
pub const VK_IMAGE_TYPE_1D: VkImageType = 0;
pub const VK_IMAGE_TYPE_2D: VkImageType = 1;
pub const VK_IMAGE_TYPE_3D: VkImageType = 2;

pub type VkImageTiling = i32;
pub const VK_IMAGE_TILING_OPTIMAL: VkImageTiling = 0;
pub const VK_IMAGE_TILING_LINEAR: VkImageTiling = 1;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
pub const VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT: VkImageTiling = 1000158000;

pub type VkPhysicalDeviceType = i32;
pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: VkPhysicalDeviceType = 0;
pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: VkPhysicalDeviceType = 1;
pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: VkPhysicalDeviceType = 2;
pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: VkPhysicalDeviceType = 3;
pub const VK_PHYSICAL_DEVICE_TYPE_CPU: VkPhysicalDeviceType = 4;

pub type VkQueryType = i32;
pub const VK_QUERY_TYPE_OCCLUSION: VkQueryType = 0;
pub const VK_QUERY_TYPE_PIPELINE_STATISTICS: VkQueryType = 1;
pub const VK_QUERY_TYPE_TIMESTAMP: VkQueryType = 2;

pub type VkSharingMode = i32;
pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = 0;
pub const VK_SHARING_MODE_CONCURRENT: VkSharingMode = 1;

pub type VkImageLayout = i32;
pub const VK_IMAGE_LAYOUT_UNDEFINED: VkImageLayout = 0;
pub const VK_IMAGE_LAYOUT_GENERAL: VkImageLayout = 1;
pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: VkImageLayout = 2;
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 3;
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 4;
pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: VkImageLayout = 5;
pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: VkImageLayout = 6;
pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: VkImageLayout = 7;
pub const VK_IMAGE_LAYOUT_PREINITIALIZED: VkImageLayout = 8;
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 100_0117_000;
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 100_0117_001;
pub const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR: VkImageLayout = 100_0001_002;
pub const VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR: VkImageLayout = 100_0111_000;

pub type VkImageViewType = i32;
pub const VK_IMAGE_VIEW_TYPE_1D: VkImageViewType = 0;
pub const VK_IMAGE_VIEW_TYPE_2D: VkImageViewType = 1;
pub const VK_IMAGE_VIEW_TYPE_3D: VkImageViewType = 2;
pub const VK_IMAGE_VIEW_TYPE_CUBE: VkImageViewType = 3;
pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: VkImageViewType = 4;
pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: VkImageViewType = 5;
pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: VkImageViewType = 6;

pub type VkComponentSwizzle = i32;
pub const VK_COMPONENT_SWIZZLE_IDENTITY: VkComponentSwizzle = 0;
pub const VK_COMPONENT_SWIZZLE_ZERO: VkComponentSwizzle = 1;
pub const VK_COMPONENT_SWIZZLE_ONE: VkComponentSwizzle = 2;
pub const VK_COMPONENT_SWIZZLE_R: VkComponentSwizzle = 3;
pub const VK_COMPONENT_SWIZZLE_G: VkComponentSwizzle = 4;
pub const VK_COMPONENT_SWIZZLE_B: VkComponentSwizzle = 5;
pub const VK_COMPONENT_SWIZZLE_A: VkComponentSwizzle = 6;

pub type VkVertexInputRate = i32;
pub const VK_VERTEX_INPUT_RATE_VERTEX: VkVertexInputRate = 0;
pub const VK_VERTEX_INPUT_RATE_INSTANCE: VkVertexInputRate = 1;

pub type VkPrimitiveTopology = i32;
pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: VkPrimitiveTopology = 0;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: VkPrimitiveTopology = 1;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: VkPrimitiveTopology = 2;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: VkPrimitiveTopology = 3;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: VkPrimitiveTopology = 4;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: VkPrimitiveTopology = 5;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = 6;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = 7;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = 8;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = 9;
pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: VkPrimitiveTopology = 10;

pub type VkPolygonMode = i32;
pub const VK_POLYGON_MODE_FILL: VkPolygonMode = 0;
pub const VK_POLYGON_MODE_LINE: VkPolygonMode = 1;
pub const VK_POLYGON_MODE_POINT: VkPolygonMode = 2;
pub const VK_POLYGON_MODE_FILL_RECTANGLE_NV: VkPolygonMode = 100_0153_000;

pub type VkFrontFace = i32;
pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: VkFrontFace = 0;
pub const VK_FRONT_FACE_CLOCKWISE: VkFrontFace = 1;

pub type VkCompareOp = i32;
pub const VK_COMPARE_OP_NEVER: VkCompareOp = 0;
pub const VK_COMPARE_OP_LESS: VkCompareOp = 1;
pub const VK_COMPARE_OP_EQUAL: VkCompareOp = 2;
pub const VK_COMPARE_OP_LESS_OR_EQUAL: VkCompareOp = 3;
pub const VK_COMPARE_OP_GREATER: VkCompareOp = 4;
pub const VK_COMPARE_OP_NOT_EQUAL: VkCompareOp = 5;
pub const VK_COMPARE_OP_GREATER_OR_EQUAL: VkCompareOp = 6;
pub const VK_COMPARE_OP_ALWAYS: VkCompareOp = 7;

pub type VkStencilOp = i32;
pub const VK_STENCIL_OP_KEEP: VkStencilOp = 0;
pub const VK_STENCIL_OP_ZERO: VkStencilOp = 1;
pub const VK_STENCIL_OP_REPLACE: VkStencilOp = 2;
pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP: VkStencilOp = 3;
pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP: VkStencilOp = 4;
pub const VK_STENCIL_OP_INVERT: VkStencilOp = 5;
pub const VK_STENCIL_OP_INCREMENT_AND_WRAP: VkStencilOp = 6;
pub const VK_STENCIL_OP_DECREMENT_AND_WRAP: VkStencilOp = 7;

pub type VkLogicOp = i32;
pub const VK_LOGIC_OP_CLEAR: VkLogicOp = 0;
pub const VK_LOGIC_OP_AND: VkLogicOp = 1;
pub const VK_LOGIC_OP_AND_REVERSE: VkLogicOp = 2;
pub const VK_LOGIC_OP_COPY: VkLogicOp = 3;
pub const VK_LOGIC_OP_AND_INVERTED: VkLogicOp = 4;
pub const VK_LOGIC_OP_NO_OP: VkLogicOp = 5;
pub const VK_LOGIC_OP_XOR: VkLogicOp = 6;
pub const VK_LOGIC_OP_OR: VkLogicOp = 7;
pub const VK_LOGIC_OP_NOR: VkLogicOp = 8;
pub const VK_LOGIC_OP_EQUIVALENT: VkLogicOp = 9;
pub const VK_LOGIC_OP_INVERT: VkLogicOp = 10;
pub const VK_LOGIC_OP_OR_REVERSE: VkLogicOp = 11;
pub const VK_LOGIC_OP_COPY_INVERTED: VkLogicOp = 12;
pub const VK_LOGIC_OP_OR_INVERTED: VkLogicOp = 13;
pub const VK_LOGIC_OP_NAND: VkLogicOp = 14;
pub const VK_LOGIC_OP_SET: VkLogicOp = 15;

pub type VkBlendFactor = i32;
pub const VK_BLEND_FACTOR_ZERO: VkBlendFactor = 0;
pub const VK_BLEND_FACTOR_ONE: VkBlendFactor = 1;
pub const VK_BLEND_FACTOR_SRC_COLOR: VkBlendFactor = 2;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: VkBlendFactor = 3;
pub const VK_BLEND_FACTOR_DST_COLOR: VkBlendFactor = 4;
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: VkBlendFactor = 5;
pub const VK_BLEND_FACTOR_SRC_ALPHA: VkBlendFactor = 6;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: VkBlendFactor = 7;
pub const VK_BLEND_FACTOR_DST_ALPHA: VkBlendFactor = 8;
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: VkBlendFactor = 9;
pub const VK_BLEND_FACTOR_CONSTANT_COLOR: VkBlendFactor = 10;
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: VkBlendFactor = 11;
pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: VkBlendFactor = 12;
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: VkBlendFactor = 13;
pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: VkBlendFactor = 14;
pub const VK_BLEND_FACTOR_SRC1_COLOR: VkBlendFactor = 15;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: VkBlendFactor = 16;
pub const VK_BLEND_FACTOR_SRC1_ALPHA: VkBlendFactor = 17;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: VkBlendFactor = 18;

pub type VkBlendOp = i32;
pub const VK_BLEND_OP_ADD: VkBlendOp = 0;
pub const VK_BLEND_OP_SUBTRACT: VkBlendOp = 1;
pub const VK_BLEND_OP_REVERSE_SUBTRACT: VkBlendOp = 2;
pub const VK_BLEND_OP_MIN: VkBlendOp = 3;
pub const VK_BLEND_OP_MAX: VkBlendOp = 4;
pub const VK_BlEND_OP_ZERO_EXT: VkBlendOp = 100_0148_000;
pub const VK_BLEND_OP_SRC_EXT: VkBlendOp = 100_0148_001;
pub const VK_BLEND_OP_DST_EXT: VkBlendOp = 100_0148_002;
pub const VK_BLEND_OP_SRC_OVER_EXT: VkBlendOp = 100_0148_003;
pub const VK_BLEND_OP_DST_OVER_EXT: VkBlendOp = 100_0148_004;
pub const VK_BLEND_OP_SRC_IN_EXT: VkBlendOp = 100_0148_005;
pub const VK_BLEND_OP_DST_IN_EXT: VkBlendOp = 100_0148_006;
pub const VK_BLEND_OP_SRC_OUT_EXT: VkBlendOp = 100_0148_007;
pub const VK_BLEND_OP_DST_OUT_EXT: VkBlendOp = 100_0148_008;
pub const VK_BLEND_OP_SRC_ATOP_EXT: VkBlendOp = 100_0148_009;
pub const VK_BLEND_OP_DST_ATOP_EXT: VkBlendOp = 100_0148_010;
pub const VK_BLEND_OP_XOR_EXT: VkBlendOp = 100_0148_011;
pub const VK_BLEND_OP_MULTIPLY_EXT: VkBlendOp = 100_0148_012;
pub const VK_BLEND_OP_SCREEN_EXT: VkBlendOp = 100_0148_013;
pub const VK_BLEND_OP_OVERLAY_EXT: VkBlendOp = 100_0148_014;
pub const VK_BLEND_OP_DARKEN_EXT: VkBlendOp = 100_0148_015;
pub const VK_BLEND_OP_LIGHTEN_EXT: VkBlendOp = 100_0148_016;
pub const VK_BLEND_OP_COLORDODGE_EXT: VkBlendOp = 100_0148_017;
pub const VK_BLEND_OP_COLORBURN_EXT: VkBlendOp = 100_0148_018;
pub const VK_BLEND_OP_HARDLIGHT_EXT: VkBlendOp = 100_0148_019;
pub const VK_BLEND_OP_SOFTLIGHT_EXT: VkBlendOp = 100_0148_020;
pub const VK_BLEND_OP_DIFFERENCE_EXT: VkBlendOp = 100_0148_021;
pub const VK_BLEND_OP_EXCLUSION_EXT: VkBlendOp = 100_0148_022;
pub const VK_BLEND_OP_INVERT_EXT: VkBlendOp = 100_0148_023;
pub const VK_BLEND_OP_INVERT_RGB_EXT: VkBlendOp = 100_0148_024;
pub const VK_BLEND_OP_LINEARDODGE_EXT: VkBlendOp = 100_0148_025;
pub const VK_BLEND_OP_LINEARBURN_EXT: VkBlendOp = 100_0148_026;
pub const VK_BLEND_OP_VIVIDLIGHT_EXT: VkBlendOp = 100_0148_027;
pub const VK_BLEND_OP_LINEARLIGHT_EXT: VkBlendOp = 100_0148_028;
pub const VK_BLEND_OP_PINLIGHT_EXT: VkBlendOp = 100_0148_029;
pub const VK_BLEND_OP_HARDMIX_EXT: VkBlendOp = 100_0148_030;
pub const VK_BLEND_OP_HSL_HUE_EXT: VkBlendOp = 100_0148_031;
pub const VK_BLEND_OP_HSL_SATURATION_EXT: VkBlendOp = 100_0148_032;
pub const VK_BLEND_OP_HSL_COLOR_EXT: VkBlendOp = 100_0148_033;
pub const VK_BLEND_OP_HSL_LUMINOSITY_EXT: VkBlendOp = 100_0148_034;
pub const VK_BLEND_OP_PLUS_EXT: VkBlendOp = 100_0148_035;
pub const VK_BLEND_OP_PLUS_CLAMPED_EXT: VkBlendOp = 100_0148_036;
pub const VK_BLEND_OP_PLUS_CLAMPED_ALHPA_EXT: VkBlendOp = 100_0148_037;
pub const VK_BLEND_OP_PLUS_DARKER_EXT: VkBlendOp = 100_0148_038;
pub const VK_BLEND_OP_MINUS_EXT: VkBlendOp = 100_0148_039;
pub const VK_BLEND_OP_MINUS_CLAMPED_EXT: VkBlendOp = 100_0148_040;
pub const VK_BLEND_OP_CONTRAST_EXT: VkBlendOp = 100_0148_041;
pub const VK_BLEND_OP_INVERT_OVG_EXT: VkBlendOp = 100_0148_042;
pub const VK_BLEND_OP_RED_EXT: VkBlendOp = 100_0148_043;
pub const VK_BLEND_OP_GREEN_EXT: VkBlendOp = 100_0148_044;
pub const VK_BLEND_OP_BLUE_EXT: VkBlendOp = 100_0148_045;

pub type VkDynamicState = i32;
pub const VK_DYNAMIC_STATE_VIEWPORT: VkDynamicState = 0;
pub const VK_DYNAMIC_STATE_SCISSOR: VkDynamicState = 1;
pub const VK_DYNAMIC_STATE_LINE_WIDTH: VkDynamicState = 2;
pub const VK_DYNAMIC_STATE_DEPTH_BIAS: VkDynamicState = 3;
pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: VkDynamicState = 4;
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: VkDynamicState = 5;
pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: VkDynamicState = 6;
pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: VkDynamicState = 7;
pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: VkDynamicState = 8;
pub const VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV: VkDynamicState = 100_0087_000;
pub const VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT: VkDynamicState = 100_0099_000;
pub const VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT: VkDynamicState = 100_0143_000;

pub type VkFilter = i32;
pub const VK_FILTER_NEAREST: VkFilter = 0;
pub const VK_FILTER_LINEAR: VkFilter = 1;
pub const VK_FILTER_CUBIC_IMG: VkFilter = 100_0015_000;

pub type VkSamplerMipmapMode = i32;
pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: VkSamplerMipmapMode = 0;
pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: VkSamplerMipmapMode = 1;

pub type VkSamplerAddressMode = i32;
pub const VK_SAMPLER_ADDRESS_MODE_REPEAT: VkSamplerAddressMode = 0;
pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: VkSamplerAddressMode = 1;
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: VkSamplerAddressMode = 2;
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: VkSamplerAddressMode = 3;
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE: VkSamplerAddressMode = 4;

pub type VkBorderColor = i32;
pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: VkBorderColor = 0;
pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: VkBorderColor = 1;
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: VkBorderColor = 2;
pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK: VkBorderColor = 3;
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: VkBorderColor = 4;
pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE: VkBorderColor = 5;

pub type VkDescriptorType = i32;
pub const VK_DESCRIPTOR_TYPE_SAMPLER: VkDescriptorType = 0;
pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: VkDescriptorType = 1;
pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: VkDescriptorType = 2;
pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: VkDescriptorType = 3;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: VkDescriptorType = 4;
pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: VkDescriptorType = 5;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: VkDescriptorType = 6;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: VkDescriptorType = 7;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: VkDescriptorType = 8;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: VkDescriptorType = 9;
pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: VkDescriptorType = 10;

pub type VkAttachmentLoadOp = i32;
pub const VK_ATTACHMENT_LOAD_OP_LOAD: VkAttachmentLoadOp = 0;
pub const VK_ATTACHMENT_LOAD_OP_CLEAR: VkAttachmentLoadOp = 1;
pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: VkAttachmentLoadOp = 2;

pub type VkAttachmentStoreOp = i32;
pub const VK_ATTACHMENT_STORE_OP_STORE: VkAttachmentStoreOp = 0;
pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: VkAttachmentStoreOp = 1;

pub type VkPipelineBindPoint = i32;
pub const VK_PIPELINE_BIND_POINT_GRAPHICS: VkPipelineBindPoint = 0;
pub const VK_PIPELINE_BIND_POINT_COMPUTE: VkPipelineBindPoint = 1;

pub type VkCommandBufferLevel = i32;
pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: VkCommandBufferLevel = 0;
pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: VkCommandBufferLevel = 1;

pub type VkIndexType = i32;
pub const VK_INDEX_TYPE_UINT16: VkIndexType = 0;
pub const VK_INDEX_TYPE_UINT32: VkIndexType = 1;

pub type VkSubpassContents = i32;
pub const VK_SUBPASS_CONTENTS_INLINE: VkSubpassContents = 0;
pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: VkSubpassContents = 1;

pub type VkObjectType = i32;
pub const VK_OBJECT_TYPE_UNKNOWN: VkObjectType = 0;
pub const VK_OBJECT_TYPE_INSTANCE: VkObjectType = 1;
pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: VkObjectType = 2;
pub const VK_OBJECT_TYPE_DEVICE: VkObjectType = 3;
pub const VK_OBJECT_TYPE_QUEUE: VkObjectType = 4;
pub const VK_OBJECT_TYPE_SEMAPHORE: VkObjectType = 5;
pub const VK_OBJECT_TYPE_COMMAND_BUFFER: VkObjectType = 6;
pub const VK_OBJECT_TYPE_FENCE: VkObjectType = 7;
pub const VK_OBJECT_TYPE_DEVICE_MEMORY: VkObjectType = 8;
pub const VK_OBJECT_TYPE_BUFFER: VkObjectType = 9;
pub const VK_OBJECT_TYPE_IMAGE: VkObjectType = 10;
pub const VK_OBJECT_TYPE_EVENT: VkObjectType = 11;
pub const VK_OBJECT_TYPE_QUERY_POOL: VkObjectType = 12;
pub const VK_OBJECT_TYPE_BUFFER_VIEW: VkObjectType = 13;
pub const VK_OBJECT_TYPE_IMAGE_VIEW: VkObjectType = 14;
pub const VK_OBJECT_TYPE_SHADER_MODULE: VkObjectType = 15;
pub const VK_OBJECT_TYPE_PIPELINE_CACHE: VkObjectType = 16;
pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: VkObjectType = 17;
pub const VK_OBJECT_TYPE_RENDER_PASS: VkObjectType = 18;
pub const VK_OBJECT_TYPE_PIPELINE: VkObjectType = 19;
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: VkObjectType = 20;
pub const VK_OBJECT_TYPE_SAMPLER: VkObjectType = 21;
pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL: VkObjectType = 22;
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: VkObjectType = 23;
pub const VK_OBJECT_TYPE_FRAMEBUFFER: VkObjectType = 24;
pub const VK_OBJECT_TYPE_COMMAND_POOL: VkObjectType = 25;

pub type VkInstanceCreateFlags = VkFlags;

pub type VkFormatFeatureFlags = VkFlags;
pub type VkFormatFeatureFlagBits = VkFlags;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits = 0x0000_0001;
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits = 0x0000_0002;
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits = 0x0000_0004;
pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = 0x0000_0008;
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = 0x0000_0010;
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits = 0x0000_0020;
pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits = 0x0000_0040;
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits = 0x0000_0080;
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits = 0x0000_0100;
pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits = 0x0000_0200;
pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT: VkFormatFeatureFlagBits = 0x0000_0400;
pub const VK_FORMAT_FEATURE_BLIT_DST_BIT: VkFormatFeatureFlagBits = 0x0000_0800;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits = 0x0000_1000;
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits = 0x0002_0000;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlagBits = 0x0004_0000;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits =
    0x0008_0000;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits =
    0x0010_0000;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT:
    VkFormatFeatureFlagBits = 0x0020_0000;
pub const VK_FORMAT_FEATURE_DISJOINT_BIT: VkFormatFeatureFlagBits = 0x0040_0000;
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits = 0x0080_0000;
pub const VK_FORMAT_FeATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: VkFormatFeatureFlagBits = 0x0000_2000;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: VkFormatFeatureFlagBits = 0x0001_0000;

pub type VkImageUsageFlags = VkFlags;
pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkImageUsageFlags = 0x01;
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlags = 0x02;
pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkImageUsageFlags = 0x04;
pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlags = 0x08;
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlags = 0x10;
pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlags = 0x20;
pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlags = 0x40;
pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkImageUsageFlags = 0x80;

pub type VkImageCreateFlags = VkFlags;
pub type VkImageCreateFlagBits = VkFlags;
pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkImageCreateFlagBits = 0x0001;
pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkImageCreateFlagBits = 0x0002;
pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkImageCreateFlagBits = 0x0004;
pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkImageCreateFlagBits = 0x0008;
pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkImageCreateFlagBits = 0x0010;
pub const VK_IMAGE_CREATE_ALIAS_BIT: VkImageCreateFlagBits = 0x0400;
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT: VkImageCreateFlagBits = 0x0040;
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT: VkImageCreateFlagBits = 0x0080;
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT: VkImageCreateFlagBits = 0x0100;
pub const VK_IMAGE_CREATE_PROTECTED_BIT: VkImageCreateFlagBits = 0x0800;
pub const VK_IMAGE_CREATE_DISJOINT_BIT: VkImageCreateFlagBits = 0x0200;
pub const VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT: VkImageCreateFlagBits = 0x1000;

pub type VkSampleCountFlags = VkFlags;
pub type VkSampleCountFlagBits = VkFlags;
pub const VK_SAMPLE_COUNT_1_BIT: VkSampleCountFlagBits = 0x01;
pub const VK_SAMPLE_COUNT_2_BIT: VkSampleCountFlagBits = 0x02;
pub const VK_SAMPLE_COUNT_4_BIT: VkSampleCountFlagBits = 0x04;
pub const VK_SAMPLE_COUNT_8_BIT: VkSampleCountFlagBits = 0x08;
pub const VK_SAMPLE_COUNT_16_BIT: VkSampleCountFlagBits = 0x10;
pub const VK_SAMPLE_COUNT_32_BIT: VkSampleCountFlagBits = 0x20;
pub const VK_SAMPLE_COUNT_64_BIT: VkSampleCountFlagBits = 0x40;

pub type VkQueueFlags = VkFlags;
pub const VK_QUEUE_GRAPHICS_BIT: VkQueueFlags = 0x01;
pub const VK_QUEUE_COMPUTE_BIT: VkQueueFlags = 0x02;
pub const VK_QUEUE_TRANSFER_BIT: VkQueueFlags = 0x04;
pub const VK_QUEUE_SPARSE_BINDING_BIT: VkQueueFlags = 0x08;
pub const VK_QUEUE_PROTECTED_BIT: VkQueueFlags = 0x10;

pub type VkMemoryPropertyFlags = VkFlags;
pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlags = 0x01;
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlags = 0x02;
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlags = 0x04;
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlags = 0x08;
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlags = 0x10;
pub const VK_MEMORY_PROPERTY_PROTECTED_BIT: VkMemoryPropertyFlags = 0x20;

pub type VkMemoryHeapFlags = VkFlags;
pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT: VkMemoryHeapFlags = 0x01;
pub type VkDeviceCreateFlags = VkFlags;

pub type VkDeviceQueueCreateFlags = VkFlags;
pub const VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT: VkDeviceQueueCreateFlags = 0x01;

pub type VkPipelineStageFlags = VkFlags;
pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkPipelineStageFlags = 0x0000_0001;
pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkPipelineStageFlags = 0x0000_0002;
pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkPipelineStageFlags = 0x0000_0004;
pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkPipelineStageFlags = 0x0000_0008;
pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlags = 0x0000_0010;
pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlags = 0x0000_0020;
pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkPipelineStageFlags = 0x0000_0040;
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkPipelineStageFlags = 0x0000_0080;
pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlags = 0x0000_0100;
pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlags = 0x0000_0200;
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlags = 0x0000_0400;
pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkPipelineStageFlags = 0x0000_0800;
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkPipelineStageFlags = 0x0000_1000;
pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlags = 0x0000_2000;
pub const VK_PIPELINE_STAGE_HOST_BIT: VkPipelineStageFlags = 0x0000_4000;
pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkPipelineStageFlags = 0x0000_8000;
pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkPipelineStageFlags = 0x0001_0000;
pub const VK_PIPELINE_STAGE_COMMAND_PROCESS_BIT_NVX: VkPipelineStageFlags = 0x0002_0000;
pub type VkMemoryMapFlags = VkFlags;

pub type VkImageAspectFlags = VkFlags;
vk_bitmask! {
    pub enum VkImageAspectFlagBits {
        pub VK_IMAGE_ASPECT_COLOR_BIT: 0,
        pub VK_IMAGE_ASPECT_DEPTH_BIT: 1,
        pub VK_IMAGE_ASPECT_STENCIL_BIT: 2,
        pub VK_IMAGE_ASPECT_METADATA_BIT: 3,
        pub VK_IMAGE_ASPECT_PLANE_0_BIT: 4,
        pub VK_IMAGE_ASPECT_PLANE_1_BIT: 5,
        pub VK_IMAGE_ASPECT_PLANE_2_BIT: 6
    }
}
cfg_if! {
    if #[cfg(feature = "VK_EXT_image_drm_format_modifier")] {
        pub const VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT: VkImageAspectFlagBits = 0x00000080;
        pub const VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT: VkImageAspectFlagBits = 0x00000100;
        pub const VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT: VkImageAspectFlagBits = 0x00000200;
        pub const VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT: VkImageAspectFlagBits = 0x00000400;
    }
}

pub type VkSparseImageFormatFlags = VkFlags;
pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: VkSparseImageFormatFlags = 0x01;
pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: VkSparseImageFormatFlags = 0x02;
pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: VkSparseImageFormatFlags = 0x04;

pub type VkSparseMemoryBindFlags = VkFlags;
pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT: VkSparseMemoryBindFlags = 0x01;

pub type VkFenceCreateFlags = VkFlags;
pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlags = 0x01;
pub type VkSemaphoreCreateFlags = VkFlags;
pub type VkEventCreateFlags = VkFlags;
pub type VkQueryPoolCreateFlags = VkFlags;

pub type VkQueryPipelineStatisticFlags = VkFlags;
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: VkQueryPipelineStatisticFlags = 0x0001;
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: VkQueryPipelineStatisticFlags = 0x0002;
pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = 0x0004;
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = 0x0008;
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: VkQueryPipelineStatisticFlags = 0x0010;
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = 0x0020;
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: VkQueryPipelineStatisticFlags = 0x0040;
pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = 0x0080;
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: VkQueryPipelineStatisticFlags = 0x0100;
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags =
    0x0200;
pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = 0x0400;

pub type VkQueryResultFlags = VkFlags;
pub const VK_QUERY_RESULT_64_BIT: VkQueryResultFlags = 0x01;
pub const VK_QUERY_RESULT_WAIT_BIT: VkQueryResultFlags = 0x02;
pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT: VkQueryResultFlags = 0x04;
pub const VK_QUERY_RESULT_PARTIAL_BIT: VkQueryResultFlags = 0x08;

pub type VkBufferCreateFlags = VkFlags;
pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT: VkBufferCreateFlags = 0x01;
pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT: VkBufferCreateFlags = 0x02;
pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT: VkBufferCreateFlags = 0x04;
pub const VK_BUFFER_CREATE_PROTECTED_BIT: VkBufferCreateFlags = 0x08;

pub type VkBufferUsageFlags = VkFlags;
pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: VkBufferUsageFlags = 0x0001;
pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: VkBufferUsageFlags = 0x0002;
pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlags = 0x0004;
pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlags = 0x0008;
pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkBufferUsageFlags = 0x0010;
pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: VkBufferUsageFlags = 0x0020;
pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: VkBufferUsageFlags = 0x0040;
pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: VkBufferUsageFlags = 0x0080;
pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: VkBufferUsageFlags = 0x0100;
pub type VkBufferViewCreateFlags = VkFlags;
pub type VkImageViewCreateFlags = VkFlags;
pub type VkShaderModuleCreateFlags = VkFlags;
pub type VkPipelineCacheCreateFlags = VkFlags;

pub type VkPipelineCreateFlags = VkFlags;
pub type VkPipelineCreateFlagBits = VkFlags;
pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlagBits = 0x01;
pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlagBits = 0x02;
pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT: VkPipelineCreateFlagBits = 0x04;
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT: VkPipelineCreateFlagBits = 0x08;
pub const VK_PIPELINE_CREATE_DISPATCH_BASE: VkPipelineCreateFlagBits = 0x10;
pub type VkPipelineShaderStageCreateFlags = VkFlags;

pub type VkShaderStageFlags = VkFlags;
pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlags = 0x01;
pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlags = 0x02;
pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlags = 0x04;
pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlags = 0x08;
pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlags = 0x10;
pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlags = 0x20;
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlags = VK_SHADER_STAGE_COMPUTE_BIT - 1;
pub const VK_SHADER_STAGE_ALL: VkShaderStageFlags = 0x7fff_ffff;
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
pub type VkPipelineViewportStateCreateFlags = VkFlags;
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;

pub type VkCullModeFlags = VkFlags;
pub const VK_CULL_MODE_NONE: VkCullModeFlags = 0;
pub const VK_CULL_MODE_FRONT_BIT: VkCullModeFlags = 0x01;
pub const VK_CULL_MODE_BACK_BIT: VkCullModeFlags = 0x02;
pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlags = 0x03;
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;
pub type VkPipelineColorBlendStateCreateFlags = VkFlags;

pub type VkColorComponentFlags = VkFlags;
pub const VK_COLOR_COMPONENT_R_BIT: VkColorComponentFlags = 0x01;
pub const VK_COLOR_COMPONENT_G_BIT: VkColorComponentFlags = 0x02;
pub const VK_COLOR_COMPONENT_B_BIT: VkColorComponentFlags = 0x04;
pub const VK_COLOR_COMPONENT_A_BIT: VkColorComponentFlags = 0x08;
pub type VkPipelineDynamicStateCreateFlags = VkFlags;
pub type VkPipelineLayoutCreateFlags = VkFlags;
pub type VkSamplerCreateFlags = VkFlags;

pub type VkDescriptorSetLayoutCreateFlags = VkFlags;
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR: VkDescriptorSetLayoutCreateFlags = 0x01;

pub type VkDescriptorPoolCreateFlags = VkFlags;
pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: VkDescriptorPoolCreateFlags = 0x01;
pub type VkDescriptorPoolResetFlags = VkFlags;
pub type VkFramebufferCreateFlags = VkFlags;
pub type VkRenderPassCreateFlags = VkFlags;

pub type VkAttachmentDescriptionFlags = VkFlags;
pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkAttachmentDescriptionFlags = 0x01;

pub type VkSubpassDescriptionFlags = VkFlags;
pub const VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX: VkSubpassDescriptionFlags = 0x01;
pub const VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX: VkSubpassDescriptionFlags = 0x02;

pub type VkAccessFlags = VkFlags;
pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkAccessFlags = 0x0000_0001;
pub const VK_ACCESS_INDEX_READ_BIT: VkAccessFlags = 0x0000_0002;
pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlags = 0x0000_0004;
pub const VK_ACCESS_UNIFORM_READ_BIT: VkAccessFlags = 0x0000_0008;
pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkAccessFlags = 0x0000_0010;
pub const VK_ACCESS_SHADER_READ_BIT: VkAccessFlags = 0x0000_0020;
pub const VK_ACCESS_SHADER_WRITE_BIT: VkAccessFlags = 0x0000_0040;
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkAccessFlags = 0x0000_0080;
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlags = 0x0000_0100;
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlags = 0x0000_0200;
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlags = 0x0000_0400;
pub const VK_ACCESS_TRANSFER_READ_BIT: VkAccessFlags = 0x0000_0800;
pub const VK_ACCESS_TRANSFER_WRITE_BIT: VkAccessFlags = 0x0000_1000;
pub const VK_ACCESS_HOST_READ_BIT: VkAccessFlags = 0x0000_2000;
pub const VK_ACCESS_HOST_WRITE_BIT: VkAccessFlags = 0x0000_4000;
pub const VK_ACCESS_MEMORY_READ_BIT: VkAccessFlags = 0x0000_8000;
pub const VK_ACCESS_MEMORY_WRITE_BIT: VkAccessFlags = 0x0001_0000;
pub const VK_ACCESS_COMMAND_PROCESS_READ_BIT_NVX: VkAccessFlags = 0x0002_0000;
pub const VK_ACCESS_COMMAND_PROCESS_WRITE_BIT_NVX: VkAccessFlags = 0x0004_0000;
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: VkAccessFlags = 0x0008_0000;

pub type VkDependencyFlags = VkFlags;
pub type VkDependencyFlagBits = VkFlags;
pub const VK_DEPENDENCY_BY_REGION_BIT: VkDependencyFlagBits = 0x01;
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT: VkDependencyFlagBits = 0x02;
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT: VkDependencyFlagBits = 0x04;

pub type VkCommandPoolCreateFlags = VkFlags;
pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: VkCommandPoolCreateFlags = 0x01;
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlags = 0x02;
pub const VK_COMMAND_POOL_PROTECTED_BIT: VkCommandPoolCreateFlags = 0x04;

pub type VkCommandPoolResetFlags = VkFlags;
pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: VkCommandPoolCreateFlags = 0x01;

pub type VkCommandBufferUsageFlags = VkFlags;
pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: VkCommandBufferUsageFlags = 0x01;
pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: VkCommandBufferUsageFlags = 0x02;
pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: VkCommandBufferUsageFlags = 0x04;

pub type VkQueryControlFlags = VkFlags;
pub const VK_QUERY_CONTROL_PRECISE_BIT: VkQueryControlFlags = 0x01;

pub type VkCommandBufferResetFlags = VkFlags;
pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: VkCommandBufferResetFlags = 0x01;

pub type VkStencilFaceFlags = VkFlags;
pub const VK_STENCIL_FACE_FRONT_BIT: VkStencilFaceFlags = 0x01;
pub const VK_STENCIL_FACE_BACK_BIT: VkStencilFaceFlags = 0x02;
pub const VK_STENCIL_FRONT_AND_BACK: VkStencilFaceFlags = 0x03;

pub type PFN_vkAllocationFunction = extern "system" fn(
    pUserData: *mut c_void,
    size: size_t,
    alignment: size_t,
    allocationScope: VkSystemAllocationScope,
) -> *mut c_void;
pub type PFN_vkReallocationFunction = extern "system" fn(
    pUserData: *mut c_void,
    pOriginal: *mut c_void,
    size: size_t,
    alignment: size_t,
    allocationScope: VkSystemAllocationScope,
) -> *mut c_void;
pub type PFN_vkFreeFunction = extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);
pub type PFN_vkInternalAllocationNotification = extern "system" fn(
    pUserData: *mut c_void,
    size: size_t,
    allocationType: VkInternalAllocationType,
    allocationScope: VkSystemAllocationScope,
);
pub type PFN_vkInternalFreeNotification = extern "system" fn(
    pUserData: *mut c_void,
    size: size_t,
    allocationType: VkInternalAllocationType,
    allocationScope: VkSystemAllocationScope,
);
pub type PFN_vkVoidFunction = extern "system" fn();

#[repr(C)]
#[derive(Debug, Clone, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_APPLICATION_INFO)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

#[repr(C)]
#[derive(Debug, Clone, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkInstanceCreateFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
}

#[repr(C)]
#[derive(Clone)]
pub struct VkAllocationCallbacks {
    pub pUserData: *mut c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: Option<PFN_vkInternalAllocationNotification>,
    pub pfnInternalFree: Option<PFN_vkInternalFreeNotification>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceFeatures {
    pub robustBufferAccess: VkBool32,
    pub fullDrawIndexUint32: VkBool32,
    pub imageCubeArray: VkBool32,
    pub independentBlend: VkBool32,
    pub geometryShader: VkBool32,
    pub tessellationShader: VkBool32,
    pub sampleRateShading: VkBool32,
    pub dualSrcBlend: VkBool32,
    pub logicOp: VkBool32,
    pub multiDrawIndirect: VkBool32,
    pub drawIndirectFirstInstance: VkBool32,
    pub depthClamp: VkBool32,
    pub depthBiasClamp: VkBool32,
    pub fillModeNonSolid: VkBool32,
    pub depthBounds: VkBool32,
    pub wideLines: VkBool32,
    pub largePoints: VkBool32,
    pub alphaToOne: VkBool32,
    pub multiViewport: VkBool32,
    pub samplerAnisotropy: VkBool32,
    pub textureCompressionETC2: VkBool32,
    pub textureCompressionASTC_LDR: VkBool32,
    pub textureCompressionBC: VkBool32,
    pub occlusionQueryPrecise: VkBool32,
    pub pipelineStatisticsQuery: VkBool32,
    pub vertexPipelineStoresAndAtomics: VkBool32,
    pub fragmentStoresAndAtomics: VkBool32,
    pub shaderTessellationAndGeometryPointSize: VkBool32,
    pub shaderImageGatherExtended: VkBool32,
    pub shaderStorageImageExtendedFormats: VkBool32,
    pub shaderStorageImageMultisample: VkBool32,
    pub shaderStorageImageReadWithoutFormat: VkBool32,
    pub shaderStorageImageWriteWithoutFormat: VkBool32,
    pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
    pub shaderSampledImageArrayDynamicIndexing: VkBool32,
    pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageImageArrayDynamicIndexing: VkBool32,
    pub shaderClipDistance: VkBool32,
    pub shaderCullDistance: VkBool32,
    pub shaderFloat64: VkBool32,
    pub shaderInt64: VkBool32,
    pub shaderInt16: VkBool32,
    pub shaderResourceResidency: VkBool32,
    pub shaderResourceMinLod: VkBool32,
    pub sparseBinding: VkBool32,
    pub sparseResidencyBuffer: VkBool32,
    pub sparseResidencyImage2D: VkBool32,
    pub sparseResidencyImage3D: VkBool32,
    pub sparseResidency2Samples: VkBool32,
    pub sparseResidency4Samples: VkBool32,
    pub sparseResidency8Samples: VkBool32,
    pub sparseResidency16Samples: VkBool32,
    pub sparseResidencyAliased: VkBool32,
    pub variableMultisampleRate: VkBool32,
    pub inheritedQueries: VkBool32,
}
impl Default for VkPhysicalDeviceFeatures {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkFormatProperties {
    pub linearTilingFeatures: VkFormatFeatureFlags,
    pub optimalTilingFeatures: VkFormatFeatureFlags,
    pub bufferFeatures: VkFormatFeatureFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
impl VkExtent3D {
    #[inline]
    pub const fn new1(width: u32) -> Self {
        Self {
            width,
            height: 1,
            depth: 1,
        }
    }

    #[inline]
    pub const fn new2(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            depth: 1,
        }
    }

    #[inline]
    pub const fn new(width: u32, height: u32, depth: u32) -> Self {
        Self { width, height, depth }
    }

    #[inline]
    pub const fn as_2d_ref(&self) -> &VkExtent2D {
        unsafe { std::mem::transmute(self) }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageFormatProperties {
    pub maxExtent: VkExtent3D,
    pub maxMipLevels: u32,
    pub maxArrayLayers: u32,
    pub sampleCounts: VkSampleCountFlags,
    pub maxResourceSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAlocationCount: u32,
    pub bufferImageGranularity: VkDeviceSize,
    pub sparseAddressSparseSize: VkDeviceSize,
    pub maxBoudnDescriptorSets: u32,
    pub maxPerStageDescriptorSample: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamples: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: c_float,
    pub maxSamplerAnisotropy: c_float,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [c_float; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: size_t,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: c_float,
    pub maxInterpolationOffset: c_float,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: VkSampleCountFlags,
    pub framebufferDepthSampleCounts: VkSampleCountFlags,
    pub framebufferStencilSampleCounts: VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
    pub sampledImageDepthSampleCounts: VkSampleCountFlags,
    pub sampledImageStencilSampleCounts: VkSampleCountFlags,
    pub storageImageSampleCounts: VkSampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: VkBool32,
    pub timestampPeriod: c_float,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [c_float; 2],
    pub lineWidthRange: [c_float; 2],
    pub pointSizeGranularity: c_float,
    pub lineWidthGranularity: c_float,
    pub strictLines: VkBool32,
    pub standardSampleLocations: VkBool32,
    pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    pub nonCoherentAtomSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: FixedCStrBuffer<VK_MAX_PHYSICAL_DEVICE_NAME_SIZE>,
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}
impl Clone for VkPhysicalDeviceMemoryProperties {
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

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const c_float,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO)]
pub struct VkDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkExtensionProperties {
    pub extensionName: FixedCStrBuffer<VK_MAX_EXTENSION_NAME_SIZE>,
    pub specVersion: u32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkLayerProperties {
    pub layerName: FixedCStrBuffer<VK_MAX_EXTENSION_NAME_SIZE>,
    pub specVersion: u32,
    pub implementationVersion: u32,
    pub description: FixedCStrBuffer<VK_MAX_DESCRIPTION_SIZE>,
}

#[repr(C)]
#[derive(Debug, Clone, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBMIT_INFO)]
pub struct VkSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub pWaitDstStageMask: *const VkPipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const VkCommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

#[repr(C)]
#[derive(Debug, Clone, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO)]
pub struct VkMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeIndex: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE)]
pub struct VkMappedMemoryRange {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSparseImageFormatProperties {
    pub aspectMask: VkImageAspectFlags,
    pub imageGranularity: VkExtent3D,
    pub flags: VkSparseImageFormatFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSparseImageMemoryRequirements {
    pub formatProperties: VkSparseImageFormatProperties,
    pub imageMiptailFirstLod: u32,
    pub imageMipTailSize: VkDeviceSize,
    pub imageMipTailOffset: VkDeviceSize,
    pub imageMipTailStride: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSparseMemoryBind {
    pub resourceOffset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSparseBufferMemoryBindInfo {
    pub buffer: VkBuffer,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageSubresource {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub arrayLayer: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkOffset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl VkOffset3D {
    #[inline]
    pub const fn new1(x: i32) -> Self {
        Self { x, y: 0, z: 0 }
    }

    #[inline]
    pub const fn new2(x: i32, y: i32) -> Self {
        Self { x, y, z: 0 }
    }

    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn as_2d_ref(&self) -> &VkOffset2D {
        unsafe { std::mem::transmute(self) }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSparseImageMemoryBind {
    pub subresource: VkImageSubresource,
    pub offset: VkOffset3D,
    pub extent: VkExtent3D,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSparseImageMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseImageMemoryBind,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_SPARSE_INFO)]
pub struct VkBindSparseInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub bufferBindCount: u32,
    pub pBufferBinds: *const VkSparseBufferMemoryBindInfo,
    pub imageOpaqueBindCount: u32,
    pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
    pub imageBindCount: u32,
    pub pImageBinds: *const VkSparseImageMemoryBindInfo,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO)]
pub struct VkFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFenceCreateFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO)]
pub struct VkSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSemaphoreCreateFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EVENT_CREATE_INFO)]
pub struct VkEventCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkEventCreateFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO)]
pub struct VkQueryPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkQueryPoolCreateFlags,
    pub queryType: VkQueryType,
    pub queryCount: u32,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO)]
pub struct VkBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO)]
pub struct VkBufferViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub buffer: VkBuffer,
    pub format: VkFormat,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO)]
pub struct VkImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageCreateFlags,
    pub imageType: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: VkSampleCountFlags,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSubresourceLayout {
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub rowPitch: VkDeviceSize,
    pub arrayPitch: VkDeviceSize,
    pub depthPitch: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}
impl Default for VkComponentMapping {
    fn default() -> Self {
        VkComponentMapping {
            r: VK_COMPONENT_SWIZZLE_R,
            g: VK_COMPONENT_SWIZZLE_G,
            b: VK_COMPONENT_SWIZZLE_B,
            a: VK_COMPONENT_SWIZZLE_A,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageSubresourceRange {
    pub aspectMask: VkImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO)]
pub struct VkImageViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageViewCreateFlags,
    pub image: VkImage,
    pub viewType: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO)]
pub struct VkShaderModuleCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub codeSize: size_t,
    pub pCode: *const u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO)]
pub struct VkPipelineCacheCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initialDataSize: size_t,
    pub pInitialData: *const c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: size_t,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: size_t,
    pub pData: *const c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlags,
    pub module: VkShaderModule,
    pub pName: *const c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VkVertexInputRate,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlags,
    pub topology: VkPrimitiveTopology,
    pub primitiveRestartEnable: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkViewport {
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
    pub minDepth: c_float,
    pub maxDepth: c_float,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32,
}
impl From<VkOffset3D> for VkOffset2D {
    fn from(value: VkOffset3D) -> Self {
        Self { x: value.x, y: value.y }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}
impl From<VkExtent3D> for VkExtent2D {
    fn from(value: VkExtent3D) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO)]
pub struct VkPipelineViewportStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineViewportStateCreateFlags,
    pub viewportCount: u32,
    pub pViewports: *const VkViewport,
    pub scissorCount: u32,
    pub pScissors: *const VkRect2D,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineRasterizationStateCreateFlags,
    pub depthClampEnable: VkBool32,
    pub rasterizerDiscardEnable: VkBool32,
    pub polygonMode: VkPolygonMode,
    pub cullMode: VkCullModeFlags,
    pub frontFace: VkFrontFace,
    pub depthBiasEnable: VkBool32,
    pub depthBiasConstantFactor: c_float,
    pub depthBiasClamp: c_float,
    pub depthBiasSlopeFactor: c_float,
    pub lineWidth: c_float,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: VkSampleCountFlags,
    pub sampleShadingEnable: VkBool32,
    pub minSampleShading: c_float,
    pub pSampleMask: *const VkSampleMask,
    pub alphaToCoverageEnable: VkBool32,
    pub alphaToOneEnable: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkStencilOpState {
    pub failOp: VkStencilOp,
    pub passOp: VkStencilOp,
    pub depthFailOp: VkStencilOp,
    pub compareOp: VkCompareOp,
    pub compareMask: u32,
    pub writeMask: u32,
    pub reference: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDepthStencilStateCreateFlags,
    pub depthTestEnable: VkBool32,
    pub depthWriteEnable: VkBool32,
    pub depthCompareOp: VkCompareOp,
    pub depthBoundsTestEnable: VkBool32,
    pub stencilTestEnable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub minDepthBounds: c_float,
    pub maxDepthBounds: c_float,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blendEnable: VkBool32,
    pub srcColorBlendFactor: VkBlendFactor,
    pub dstColorBlendFactor: VkBlendFactor,
    pub colorBlendOp: VkBlendOp,
    pub srcAlphaBlendFactor: VkBlendFactor,
    pub dstAlphaBlendFactor: VkBlendFactor,
    pub alphaBlendOp: VkBlendOp,
    pub colorWriteMask: VkColorComponentFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO)]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineColorBlendStateCreateFlags,
    pub logicOpEnable: VkBool32,
    pub logicOp: VkLogicOp,
    pub attachmentCount: u32,
    pub pAttachments: *const VkPipelineColorBlendAttachmentState,
    pub blendConstants: [c_float; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const VkDynamicState,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO)]
pub struct VkGraphicsPipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
    pub pViewportState: *const VkPipelineViewportStateCreateInfo,
    pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO)]
pub struct VkComputePipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPushConstantRange {
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO)]
pub struct VkPipelineLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO)]
pub struct VkSamplerCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSamplerCreateFlags,
    pub magFilter: VkFilter,
    pub minFilter: VkFilter,
    pub mipmapMode: VkSamplerMipmapMode,
    pub addressModeU: VkSamplerAddressMode,
    pub addressModeV: VkSamplerAddressMode,
    pub addressModeW: VkSamplerAddressMode,
    pub mipLodBias: c_float,
    pub anisotropyEnable: VkBool32,
    pub maxAnisotropy: c_float,
    pub compareEnable: VkBool32,
    pub compareOp: VkCompareOp,
    pub minLod: c_float,
    pub maxLod: c_float,
    pub borderColor: VkBorderColor,
    pub unnormalizedCoordinates: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: VkShaderStageFlags,
    pub pImmutableSamplers: *const VkSampler,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorPoolSize {
    pub _type: VkDescriptorType,
    pub descriptorCount: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO)]
pub struct VkDescriptorPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const VkDescriptorPoolSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO)]
pub struct VkDescriptorSetAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET)]
pub struct VkWriteDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub pImageInfo: *const VkDescriptorImageInfo,
    pub pBufferInfo: *const VkDescriptorBufferInfo,
    pub pTexelBufferView: *const VkBufferView,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET)]
pub struct VkCopyDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSet: VkDescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO)]
pub struct VkFramebufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFramebufferCreateFlags,
    pub renderPass: VkRenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkAttachmentDescription {
    /// A bitmask of `VkAttachmentDescriptionFlagBits` specifying additional properties of the attachment.
    ///
    /// ## Possible Bitmasks
    ///
    /// * 0: No flags
    /// * `VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT`: The attachment aliases the same device memory as other attachments
    ///
    pub flags: VkAttachmentDescriptionFlags,
    /// A `VkFormat` value specifying the format of the image that will be used for the attachment
    pub format: VkFormat,
    /// The number of samples of the image as defined in `VkSampleCountFlagBits`
    ///
    /// ## Possible Bitmasks
    ///
    /// 1, 2, 4, 8, 16, 32, 64
    ///
    pub samples: VkSampleCountFlags,
    /// A `VkAttachmentLoadOp` value specifying how the contents of color and depth components of the attachment are
    /// treated at the beginning of the subpass where it is first used
    ///
    /// ## Possible Values
    ///
    /// * `VK_ATTACHMENT_LOAD_OP_LOAD`: The previous contents of the image within the render area will be preserved.
    /// * `VK_ATTACHMENT_LOAD_OP_CLEAR`: The contents within the render area will be cleared to a uniform value,
    ///   which is specified when a render pass instance is begun.
    /// * `VK_ATTACHMENT_LOAD_OP_DONT_CARE`: The previous contents within the area need not be preserved;
    ///   the contents of the attachment will be undefined inside the render area.
    ///
    /// ## Access Type Requirements
    ///
    /// |                                   | For attachments with a depth format            | For attachments with a color format     |
    /// |-----------------------------------|------------------------------------------------|-----------------------------------------|
    /// | `VK_ATTACHMENT_LOAD_OP_LOAD`      | `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT`  | `VK_ACCESS_COLOR_ATTTACHMENT_READ_BIT`  |
    /// | `VK_ATTACHMENT_LOAD_OP_CLEAR`     | `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT` | `VK_ACCESS_COLOR_ATTTACHMENT_WRITE_BIT` |
    /// | `VK_ATTACHMENT_LOAD_OP_DONT_CARE` | `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT` | `VK_ACCESS_COLOR_ATTTACHMENT_WRITE_BIT` |
    ///
    pub loadOp: VkAttachmentLoadOp,
    /// A `VkAttachmentStoreOp` value specifying how the contents of color and depth components of the attachment are
    /// treated at the end of the subpass where it is last used.
    ///
    /// ## Possible Values
    ///
    /// * `VK_ATTACHMENT_STORE_OP_STORE`: The contents generated during the render pass and within the render area are
    ///   written to memory.
    /// * `VK_ATTACHMENT_STORE_OP_DONT_CARE`: The contents within the render area are not needed after rendering,
    ///   and *may* be discarded; the contents of the attachment will be undefined inside the render area.
    ///
    /// ## Access Type Requirements
    ///
    /// * For attachments with a depth format: Both values require `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`
    /// * For attachments with a color format: Both values require `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`
    ///
    pub storeOp: VkAttachmentStoreOp,
    /// A `VkAttachmentLoadOp` value specifying how the contents of stencil components of the attachment are
    /// treated at the beginning of the subpass where it is first used
    ///
    /// ## Possible Values
    ///
    /// * `VK_ATTACHMENT_LOAD_OP_LOAD`: The previous contents of the image within the render area will be preserved.
    /// * `VK_ATTACHMENT_LOAD_OP_CLEAR`: The contents within the render area will be cleared to a uniform value,
    ///   which is specified when a render pass instance is begun.
    /// * `VK_ATTACHMENT_LOAD_OP_DONT_CARE`: The previous contents within the area need not be preserved;
    ///   the contents of the attachment will be undefined inside the render area.
    ///
    /// ## Access Type Requirements
    ///
    /// * `VK_ATTACHMENT_LOAD_OP_LOAD` requires `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT`
    /// * `VK_ATTACHMENT_LOAD_OP_CLEAR` requires `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`
    /// * `VK_ATTACHMENT_LOAD_OP_DONT_CARE` requires `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`
    ///
    pub stencilLoadOp: VkAttachmentLoadOp,
    /// A `VkAttachmentStoreOp` value specifying how the contents of stencil components of the attachment are
    /// treated at the end of the subpass where it is last used.
    ///
    /// ## Possible Values
    ///
    /// * `VK_ATTACHMENT_STORE_OP_STORE`: The contents generated during the render pass and within the render area are
    ///   written to memory.
    /// * `VK_ATTACHMENT_STORE_OP_DONT_CARE`: The contents within the render area are not needed after rendering,
    ///   and *may* be discarded; the contents of the attachment will be undefined inside the render area.
    ///
    /// ## Access Type Requirements
    ///
    /// * Both values require `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`
    ///
    pub stencilStoreOp: VkAttachmentStoreOp,
    /// The layout the attachment image subresource will be in when a render pass instance begins.
    pub initialLayout: VkImageLayout,
    /// The layout the attachment image subresource will be transitioned to when a render pass instance ends.
    /// During a render pass instance, an attachment *can* use a different layout in each subpass, if desired.
    pub finalLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSubpassDescription {
    /// A bitmask of `VkSubpassDescriptionFlagBits` specifying usage of the subpass.
    ///
    /// ## Possible Bitmasks
    ///
    /// * 0: No flags
    pub flags: VkSubpassDescriptionFlags,
    /// A `VkPipelineBindPoint` value specifying whether this is a compute or graphics subpass.
    ///
    /// ## Possible Values
    ///
    /// * `VK_PIPELINE_BIND_POINT_GRAPHICS`: Binding as a graphics pipeline.
    /// * ~~`VK_PIPELINE_BIND_POINT_COMPUTE`: Binding as a compute pipeline.~~ (Currently unsupported)
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference,
    pub pResolveAttachments: *const VkAttachmentReference,
    pub pDepthStencilAttachment: *const VkAttachmentReference,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO)]
pub struct VkRenderPassCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO)]
pub struct VkCommandPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO)]
pub struct VkCommandBufferInheritanceInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusionQueryEnable: VkBool32,
    pub queryFlags: VkQueryControlFlags,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO)]
pub struct VkCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBufferCopy {
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageSubresourceLayers {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageCopy {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct VkImageBlit {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2],
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
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBufferImageCopy {
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VkClearColorValue {
    pub float32: [c_float; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VkClearDepthStencilValue {
    pub depth: c_float,
    pub stencil: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depthStencil: VkClearDepthStencilValue,
}

#[repr(C)]
#[derive(Clone)]
pub struct VkClearAttachment {
    pub aspectMask: VkImageAspectFlags,
    pub colorAttachment: u32,
    pub clearValue: VkClearValue,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkClearRect {
    pub rect: VkRect2D,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageResolve {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_BARRIER)]
pub struct VkMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER)]
pub struct VkBufferMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER)]
pub struct VkImageMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO)]
pub struct VkRenderPassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub framebuffer: VkFramebuffer,
    pub renderArea: VkRect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const VkClearValue,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDrawIndexedIndirectCommand {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub firstIndex: u32,
    pub vertexOffset: i32,
    pub firstInstance: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDrawIndirectCommand {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub firstVertex: u32,
    pub firstInstance: u32,
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateInstance)]
pub struct PFN_vkCreateInstance(
    pub  unsafe extern "system" fn(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyInstance)]
pub struct PFN_vkDestroyInstance(
    pub unsafe extern "system" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkEnumeratePhysicalDevices)]
pub struct PFN_vkEnumeratePhysicalDevices(
    pub  unsafe extern "system" fn(
        instance: VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut VkPhysicalDevice,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceFeatures)]
pub struct PFN_vkGetPhysicalDeviceFeatures(
    pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceFormatProperties)]
pub struct PFN_vkGetPhysicalDeviceFormatProperties(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        format: VkFormat,
        pFormatProperties: *mut VkFormatProperties,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceImageFormatProperties)]
pub struct PFN_vkGetPhysicalDeviceImageFormatProperties(
    pub  unsafe extern "system" fn(
        physicalDeivce: VkPhysicalDevice,
        format: VkFormat,
        itype: VkImageType,
        tiling: VkImageTiling,
        usage: VkImageUsageFlags,
        flags: VkImageCreateFlags,
        pImageFormatProperties: *mut VkImageFormatProperties,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceProperties)]
pub struct PFN_vkGetPhysicalDeviceProperties(
    pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceQueueFamilyProperties)]
pub struct PFN_vkGetPhysicalDeviceQueueFamilyProperties(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32,
        pQuueFamilProperties: *mut VkQueueFamilyProperties,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceMemoryProperties)]
pub struct PFN_vkGetPhysicalDeviceMemoryProperties(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetInstanceProcAddr)]
pub struct PFN_vkGetInstanceProcAddr(
    pub unsafe extern "system" fn(instance: VkInstance, pName: *const c_char) -> Option<PFN_vkVoidFunction>,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetDeviceProcAddr)]
pub struct PFN_vkGetDeviceProcAddr(
    pub unsafe extern "system" fn(device: VkDevice, pName: *const c_char) -> Option<PFN_vkVoidFunction>,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateDevice)]
pub struct PFN_vkCreateDevice(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pCreateInfo: *const VkDeviceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDevice: *mut VkDevice,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyDevice)]
pub struct PFN_vkDestroyDevice(
    pub unsafe extern "system" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkEnumerateInstanceExtensionProperties)]
pub struct PFN_vkEnumerateInstanceExtensionProperties(
    pub  unsafe extern "system" fn(
        pLayerName: *const c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkEnumerateDeviceExtensionProperties)]
pub struct PFN_vkEnumerateDeviceExtensionProperties(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pLayerName: *const c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkEnumerateInstanceLayerProperties)]
pub struct PFN_vkEnumerateInstanceLayerProperties(
    pub unsafe extern "system" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkEnumerateDeviceLayerProperties)]
pub struct PFN_vkEnumerateDeviceLayerProperties(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkLayerProperties,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetDeviceQueue)]
pub struct PFN_vkGetDeviceQueue(
    pub unsafe extern "system" fn(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkQueueSubmit)]
pub struct PFN_vkQueueSubmit(
    pub  unsafe extern "system" fn(
        queue: VkQueue,
        submitCount: u32,
        pSubmits: *const VkSubmitInfo,
        fence: VkFence,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkQueueWaitIdle)]
pub struct PFN_vkQueueWaitIdle(pub unsafe extern "system" fn(queue: VkQueue) -> VkResult);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDeviceWaitIdle)]
pub struct PFN_vkDeviceWaitIdle(pub unsafe extern "system" fn(device: VkDevice) -> VkResult);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkAllocateMemory)]
pub struct PFN_vkAllocateMemory(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pAllocateInfo: *const VkMemoryAllocateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pMemory: *mut VkDeviceMemory,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkFreeMemory)]
pub struct PFN_vkFreeMemory(
    pub unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkMapMemory)]
pub struct PFN_vkMapMemory(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
        size: VkDeviceSize,
        flags: VkMemoryMapFlags,
        ppData: *mut *mut c_void,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkUnmapMemory)]
pub struct PFN_vkUnmapMemory(pub unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory));
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkFlushMappedMemoryRanges)]
pub struct PFN_vkFlushMappedMemoryRanges(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        memoryRangeCount: u32,
        pMemoryRanges: *const VkMappedMemoryRange,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkInvalidateMappedMemoryRanges)]
pub struct PFN_vkInvalidateMappedMemoryRanges(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        memoryRangeCount: u32,
        pMemoryRanges: *const VkMappedMemoryRange,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetDeviceMemoryCommitment)]
pub struct PFN_vkGetDeviceMemoryCommitment(
    pub unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkBindBufferMemory)]
pub struct PFN_vkBindBufferMemory(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkBindImageMemory)]
pub struct PFN_vkBindImageMemory(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        image: VkImage,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetBufferMemoryRequirements)]
pub struct PFN_vkGetBufferMemoryRequirements(
    pub unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetImageMemoryRequirements)]
pub struct PFN_vkGetImageMemoryRequirements(
    pub unsafe extern "system" fn(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetImageSparseMemoryRequirements)]
pub struct PFN_vkGetImageSparseMemoryRequirements(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        image: VkImage,
        pSparseMemoryRequirementCount: *mut u32,
        pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceSparseImageFormatProperties)]
pub struct PFN_vkGetPhysicalDeviceSparseImageFormatProperties(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        format: VkFormat,
        itype: VkImageType,
        samples: VkSampleCountFlags,
        usage: VkImageUsageFlags,
        tiling: VkImageTiling,
        pPropertyCount: *mut u32,
        pProperties: *mut VkSparseImageFormatProperties,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkQueueBindSparse)]
pub struct PFN_vkQueueBindSparse(
    pub  unsafe extern "system" fn(
        queue: VkQueue,
        bindInfoCount: u32,
        pBindInfo: *const VkBindSparseInfo,
        fence: VkFence,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateFence)]
pub struct PFN_vkCreateFence(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkFenceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFence: *mut VkFence,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyFence)]
pub struct PFN_vkDestroyFence(
    pub unsafe extern "system" fn(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkResetFences)]
pub struct PFN_vkResetFences(
    pub unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetFenceStatus)]
pub struct PFN_vkGetFenceStatus(pub unsafe extern "system" fn(device: VkDevice, fence: VkFence) -> VkResult);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkWaitForFences)]
pub struct PFN_vkWaitForFences(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        fenceCount: u32,
        pFences: *const VkFence,
        waitAll: VkBool32,
        timeout: u64,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateSemaphore)]
pub struct PFN_vkCreateSemaphore(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkSemaphoreCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSemaphore: *mut VkSemaphore,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroySemaphore)]
pub struct PFN_vkDestroySemaphore(
    pub unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateEvent)]
pub struct PFN_vkCreateEvent(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkEventCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pEvent: *mut VkEvent,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyEvent)]
pub struct PFN_vkDestroyEvent(
    pub unsafe extern "system" fn(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetEventStatus)]
pub struct PFN_vkGetEventStatus(pub unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkSetEvent)]
pub struct PFN_vkSetEvent(pub unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkResetEvent)]
pub struct PFN_vkResetEvent(pub unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateQueryPool)]
pub struct PFN_vkCreateQueryPool(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkQueryPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pQueryPool: *mut VkQueryPool,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyQueryPool)]
pub struct PFN_vkDestroyQueryPool(
    pub unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetQueryPoolResults)]
pub struct PFN_vkGetQueryPoolResults(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
        dataSize: size_t,
        pData: *mut c_void,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateBuffer)]
pub struct PFN_vkCreateBuffer(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkBufferCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pBuffer: *mut VkBuffer,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyBuffer)]
pub struct PFN_vkDestroyBuffer(
    pub unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateBufferView)]
pub struct PFN_vkCreateBufferView(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkBufferViewCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pView: *mut VkBufferView,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyBufferView)]
pub struct PFN_vkDestroyBufferView(
    pub unsafe extern "system" fn(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateImage)]
pub struct PFN_vkCreateImage(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkImageCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pImage: *mut VkImage,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyImage)]
pub struct PFN_vkDestroyImage(
    pub unsafe extern "system" fn(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetImageSubresourceLayout)]
pub struct PFN_vkGetImageSubresourceLayout(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        image: VkImage,
        pSubresource: *const VkImageSubresource,
        pLayout: *mut VkSubresourceLayout,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateImageView)]
pub struct PFN_vkCreateImageView(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkImageViewCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pView: *mut VkImageView,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyImageView)]
pub struct PFN_vkDestroyImageView(
    pub unsafe extern "system" fn(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateShaderModule)]
pub struct PFN_vkCreateShaderModule(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkShaderModuleCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pShaderModule: *mut VkShaderModule,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyShaderModule)]
pub struct PFN_vkDestroyShaderModule(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        shaderModule: VkShaderModule,
        pAllocator: *const VkAllocationCallbacks,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreatePipelineCache)]
pub struct PFN_vkCreatePipelineCache(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkPipelineCacheCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineCache: *mut VkPipelineCache,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyPipelineCache)]
pub struct PFN_vkDestroyPipelineCache(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        pAllocator: *const VkAllocationCallbacks,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPipelineCacheData)]
pub struct PFN_vkGetPipelineCacheData(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        pDataSize: *mut size_t,
        pData: *mut c_void,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkMergePipelineCaches)]
pub struct PFN_vkMergePipelineCaches(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        dstCache: VkPipelineCache,
        srcCacheCount: u32,
        pSrcCaches: *const VkPipelineCache,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateGraphicsPipelines)]
pub struct PFN_vkCreateGraphicsPipelines(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkGraphicsPipelineCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateComputePipelines)]
pub struct PFN_vkCreateComputePipelines(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkComputePipelineCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyPipeline)]
pub struct PFN_vkDestroyPipeline(
    pub unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreatePipelineLayout)]
pub struct PFN_vkCreatePipelineLayout(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkPipelineLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineLayout: *mut VkPipelineLayout,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyPipelineLayout)]
pub struct PFN_vkDestroyPipelineLayout(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pipelineLayout: VkPipelineLayout,
        pAllocator: *const VkAllocationCallbacks,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateSampler)]
pub struct PFN_vkCreateSampler(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkSamplerCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSampler: *mut VkSampler,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroySampler)]
pub struct PFN_vkDestroySampler(
    pub unsafe extern "system" fn(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks),
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateDescriptorSetLayout)]
pub struct PFN_vkCreateDescriptorSetLayout(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSetLayout: *mut VkDescriptorSetLayout,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyDescriptorSetLayout)]
pub struct PFN_vkDestroyDescriptorSetLayout(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        descriptorSetLayout: VkDescriptorSetLayout,
        pAllocator: *const VkAllocationCallbacks,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateDescriptorPool)]
pub struct PFN_vkCreateDescriptorPool(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDescriptorPool: *mut VkDescriptorPool,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyDescriptorPool)]
pub struct PFN_vkDestroyDescriptorPool(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        descriptorPool: VkDescriptorPool,
        pAllocator: *const VkAllocationCallbacks,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkResetDescriptorPool)]
pub struct PFN_vkResetDescriptorPool(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        descriptorPool: VkDescriptorPool,
        flags: VkDescriptorPoolResetFlags,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkAllocateDescriptorSets)]
pub struct PFN_vkAllocateDescriptorSets(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pAllocateInfo: *const VkDescriptorSetAllocateInfo,
        pDescriotorSet: *mut VkDescriptorSet,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkFreeDescriptorSets)]
pub struct PFN_vkFreeDescriptorSets(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        descriptorPool: VkDescriptorPool,
        descriptorSetCount: u32,
        pDescriptorSets: *const VkDescriptorSet,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkUpdateDescriptorSets)]
pub struct PFN_vkUpdateDescriptorSets(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        descriptorWriteCount: u32,
        pDescriptorWrites: *const VkWriteDescriptorSet,
        descriptorCopyCount: u32,
        pDescriptorCopies: *const VkCopyDescriptorSet,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateFramebuffer)]
pub struct PFN_vkCreateFramebuffer(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkFramebufferCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFramebuffer: *mut VkFramebuffer,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyFramebuffer)]
pub struct PFN_vkDestroyFramebuffer(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        framebuffer: VkFramebuffer,
        pAllocator: *const VkAllocationCallbacks,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateRenderPass)]
pub struct PFN_vkCreateRenderPass(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkRenderPassCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pRenderPass: *mut VkRenderPass,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyRenderPass)]
pub struct PFN_vkDestroyRenderPass(
    pub unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetRenderAreaGranularity)]
pub struct PFN_vkGetRenderAreaGranularity(
    pub unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateCommandPool)]
pub struct PFN_vkCreateCommandPool(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkCommandPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pCommandPool: *mut VkCommandPool,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyCommandPool)]
pub struct PFN_vkDestroyCommandPool(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        commandPool: VkCommandPool,
        pAllocator: *const VkAllocationCallbacks,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkResetCommandPool)]
pub struct PFN_vkResetCommandPool(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        commandPool: VkCommandPool,
        flags: VkCommandPoolResetFlags,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkAllocateCommandBuffers)]
pub struct PFN_vkAllocateCommandBuffers(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pAllocateInfo: *const VkCommandBufferAllocateInfo,
        pCommandBuffers: *mut VkCommandBuffer,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkFreeCommandBuffers)]
pub struct PFN_vkFreeCommandBuffers(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        commandPool: VkCommandPool,
        commandBufferCount: u32,
        pCommandBuffers: *const VkCommandBuffer,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkBeginCommandBuffer)]
pub struct PFN_vkBeginCommandBuffer(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        pBeginInfo: *const VkCommandBufferBeginInfo,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkEndCommandBuffer)]
pub struct PFN_vkEndCommandBuffer(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer) -> VkResult);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkResetCommandBuffer)]
pub struct PFN_vkResetCommandBuffer(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdBindPipeline)]
pub struct PFN_vkCmdBindPipeline(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        pipeline: VkPipeline,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetViewport)]
pub struct PFN_vkCmdSetViewport(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        firstViewport: u32,
        viewportCount: u32,
        pViewports: *const VkViewport,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetScissor)]
pub struct PFN_vkCmdSetScissor(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        firstScissor: u32,
        scissorCount: u32,
        pScissors: *const VkRect2D,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetLineWidth)]
pub struct PFN_vkCmdSetLineWidth(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineWidth: c_float));
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetDepthBias)]
pub struct PFN_vkCmdSetDepthBias(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        depthBiasConstantFactor: c_float,
        depthBiasClamp: c_float,
        depthBiasSlopeFactor: c_float,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetBlendConstants)]
pub struct PFN_vkCmdSetBlendConstants(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, blendConstants: *const c_float),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetDepthBounds)]
pub struct PFN_vkCmdSetDepthBounds(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, minDepthBounds: c_float, maxDepthBounds: c_float),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetStencilCompareMask)]
pub struct PFN_vkCmdSetStencilCompareMask(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetStencilWriteMask)]
pub struct PFN_vkCmdSetStencilWriteMask(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetStencilReference)]
pub struct PFN_vkCmdSetStencilReference(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdBindDescriptorSets)]
pub struct PFN_vkCmdBindDescriptorSets(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        firstSet: u32,
        descriptorSetCount: u32,
        pDescriptorSets: *const VkDescriptorSet,
        dynamicOffsetCount: u32,
        pDynamicOffsets: *const u32,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdBindIndexBuffer)]
pub struct PFN_vkCmdBindIndexBuffer(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        indexType: VkIndexType,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdBindVertexBuffers)]
pub struct PFN_vkCmdBindVertexBuffers(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        firstBinding: u32,
        bindingCount: u32,
        pBuffers: *const VkBuffer,
        pOffsets: *const VkDeviceSize,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDraw)]
pub struct PFN_vkCmdDraw(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        vertexCount: u32,
        instanceCount: u32,
        firstVertex: u32,
        firstInstance: u32,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDrawIndexed)]
pub struct PFN_vkCmdDrawIndexed(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        indexCount: u32,
        instanceCount: u32,
        firstIndex: u32,
        vertexOffset: i32,
        firstInstance: u32,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDrawIndirect)]
pub struct PFN_vkCmdDrawIndirect(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        drawCount: u32,
        stride: u32,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDrawIndexedIndirect)]
pub struct PFN_vkCmdDrawIndexedIndirect(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        drawCount: u32,
        stride: u32,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDispatch)]
pub struct PFN_vkCmdDispatch(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDispatchIndirect)]
pub struct PFN_vkCmdDispatchIndirect(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdCopyBuffer)]
pub struct PFN_vkCmdCopyBuffer(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstBuffer: VkBuffer,
        regionCount: u32,
        pRegions: *const VkBufferCopy,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdCopyImage)]
pub struct PFN_vkCmdCopyImage(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageCopy,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdBlitImage)]
pub struct PFN_vkCmdBlitImage(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageBlit,
        filter: VkFilter,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdCopyBufferToImage)]
pub struct PFN_vkCmdCopyBufferToImage(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkBufferImageCopy,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdCopyImageToBuffer)]
pub struct PFN_vkCmdCopyImageToBuffer(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstBuffer: VkBuffer,
        regionCount: u32,
        pRegions: *const VkBufferImageCopy,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdUpdateBuffer)]
pub struct PFN_vkCmdUpdateBuffer(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        dataSize: VkDeviceSize,
        pData: *const c_void,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdFillBuffer)]
pub struct PFN_vkCmdFillBuffer(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        size: VkDeviceSize,
        data: u32,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdClearColorImage)]
pub struct PFN_vkCmdClearColorImage(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        image: VkImage,
        imageLayout: VkImageLayout,
        pColor: *const VkClearColorValue,
        rangeCount: u32,
        pRanges: *const VkImageSubresourceRange,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdClearDepthStencilImage)]
pub struct PFN_vkCmdClearDepthStencilImage(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        image: VkImage,
        imageLayout: VkImageLayout,
        pDepthStencil: *const VkClearDepthStencilValue,
        rangeCount: u32,
        pRanges: *const VkImageSubresourceRange,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdClearAttachments)]
pub struct PFN_vkCmdClearAttachments(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        attachmentCount: u32,
        pAttachments: *const VkClearAttachment,
        rectCount: u32,
        pRects: *const VkClearRect,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdResolveImage)]
pub struct PFN_vkCmdResolveImage(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageResolve,
    ),
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetEvent)]
pub struct PFN_vkCmdSetEvent(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdResetEvent)]
pub struct PFN_vkCmdResetEvent(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdWaitEvents)]
pub struct PFN_vkCmdWaitEvents(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        eventCount: u32,
        pEvents: *const VkEvent,
        srcStageMask: VkPipelineStageFlags,
        dstStageMask: VkPipelineStageFlags,
        memoryBarrierCount: u32,
        pMemoryBarriers: *const VkMemoryBarrier,
        bufferMemoryBarrierCount: u32,
        pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
        imageMemoryBarrierCount: u32,
        pImageMemoryBarriers: *const VkImageMemoryBarrier,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdPipelineBarrier)]
pub struct PFN_vkCmdPipelineBarrier(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        srcStageMask: VkPipelineStageFlags,
        dstStageMask: VkPipelineStageFlags,
        dependencyFlags: VkDependencyFlags,
        memoryBarrierCount: u32,
        pMemoryBarriers: *const VkMemoryBarrier,
        bufferMemoryBarrierCount: u32,
        pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
        imageMemoryBarrierCount: u32,
        pImageMemoryBarriers: *const VkImageMemoryBarrier,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdBeginQuery)]
pub struct PFN_vkCmdBeginQuery(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        query: u32,
        flags: VkQueryControlFlags,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdEndQuery)]
pub struct PFN_vkCmdEndQuery(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdResetQueryPool)]
pub struct PFN_vkCmdResetQueryPool(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdWriteTimestamp)]
pub struct PFN_vkCmdWriteTimestamp(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        pipelineStage: VkPipelineStageFlags,
        queryPool: VkQueryPool,
        query: u32,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdCopyQueryPoolResults)]
pub struct PFN_vkCmdCopyQueryPoolResults(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdPushConstants)]
pub struct PFN_vkCmdPushConstants(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        layout: VkPipelineLayout,
        stageFlags: VkShaderStageFlags,
        offset: u32,
        size: u32,
        pValues: *const c_void,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdBeginRenderPass)]
pub struct PFN_vkCmdBeginRenderPass(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        pRenderPassBegin: *const VkRenderPassBeginInfo,
        contents: VkSubpassContents,
    ),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdNextSubpass)]
pub struct PFN_vkCmdNextSubpass(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdEndRenderPass)]
pub struct PFN_vkCmdEndRenderPass(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer));
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdExecuteCommands)]
pub struct PFN_vkCmdExecuteCommands(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        commandBufferCount: u32,
        pCommandBuffers: *const VkCommandBuffer,
    ),
);

#[cfg(all(feature = "Implements", not(feature = "DynamicLoaded")))]
#[cfg_attr(
    all(not(windows), not(target_os = "macos"), not(feature = "DynamicLoaded")),
    link(name = "vulkan")
)]
#[cfg_attr(all(windows, not(feature = "DynamicLoaded")), link(name = "vulkan-1"))]
extern "system" {
    pub fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    ) -> VkResult;
    pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
    pub fn vkEnumeratePhysicalDevices(
        instance: VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut VkPhysicalDevice,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
    pub fn vkGetPhysicalDeviceFormatProperties(
        physicalDevice: VkPhysicalDevice,
        format: VkFormat,
        pFormatProperties: *mut VkFormatProperties,
    );
    pub fn vkGetPhysicalDeviceImageFormatProperties(
        physicalDevice: VkPhysicalDevice,
        format: VkFormat,
        itype: VkImageType,
        tiling: VkImageTiling,
        usage: VkImageUsageFlags,
        flags: VkImageCreateFlags,
        pImageFormatProperties: *mut VkImageFormatProperties,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceProperties(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties,
    );
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(
        physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32,
        pQueueFamilyProperties: *mut VkQueueFamilyProperties,
    );
    pub fn vkGetPhysicalDeviceMemoryProperties(
        physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
    );
    pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const c_char) -> Option<PFN_vkVoidFunction>;
    pub fn vkGetDeviceProcAddr(device: VkDevice, pName: *const c_char) -> Option<PFN_vkVoidFunction>;
    pub fn vkCreateDevice(
        physicalDevice: VkPhysicalDevice,
        pCreateInfo: *const VkDeviceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDevice: *mut VkDevice,
    ) -> VkResult;
    pub fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
    pub fn vkEnumerateInstanceExtensionProperties(
        pLayerName: *const c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties,
    ) -> VkResult;
    pub fn vkEnumerateDeviceExtensionProperties(
        physicalDevice: VkPhysicalDevice,
        pLayerName: *const c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties,
    ) -> VkResult;
    pub fn vkEnumerateInstanceLayerProperties(
        pPropertyCount: *mut u32,
        pProperties: *mut VkLayerProperties,
    ) -> VkResult;
    pub fn vkEnumerateDeviceLayerProperties(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkLayerProperties,
    ) -> VkResult;
    pub fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
    pub fn vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
    pub fn vkQueueWaitIdle(queue: VkQueue) -> VkResult;
    pub fn vkDeviceWaitIdle(device: VkDevice) -> VkResult;
    pub fn vkAllocateMemory(
        device: VkDevice,
        pAllocateInfo: *const VkMemoryAllocateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pMemory: *mut VkDeviceMemory,
    ) -> VkResult;
    pub fn vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
    pub fn vkMapMemory(
        device: VkDevice,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
        size: VkDeviceSize,
        flags: VkMemoryMapFlags,
        ppData: *mut *mut c_void,
    ) -> VkResult;
    pub fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory);
    pub fn vkFlushMappedMemoryRanges(
        device: VkDevice,
        memoryRangeCount: u32,
        pMemoryRanges: *const VkMappedMemoryRange,
    ) -> VkResult;
    pub fn vkInvalidateMappedMemoryRanges(
        device: VkDevice,
        memoryRangeCount: u32,
        pMemoryRanges: *const VkMappedMemoryRange,
    ) -> VkResult;
    pub fn vkGetDeviceMemoryCommitment(
        device: VkDevice,
        memory: VkDeviceMemory,
        pCommittedMemoryInBytes: *mut VkDeviceSize,
    );
    pub fn vkBindBufferMemory(
        device: VkDevice,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize,
    ) -> VkResult;
    pub fn vkBindImageMemory(
        device: VkDevice,
        image: VkImage,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize,
    ) -> VkResult;
    pub fn vkGetBufferMemoryRequirements(
        device: VkDevice,
        buffer: VkBuffer,
        pMemoryRequirements: *mut VkMemoryRequirements,
    );
    pub fn vkGetImageMemoryRequirements(
        device: VkDevice,
        image: VkImage,
        pMemoryRequirements: *mut VkMemoryRequirements,
    );
    pub fn vkGetImageSparseMemoryRequirements(
        device: VkDevice,
        image: VkImage,
        pSparseMemoryRequirementCount: *mut u32,
        pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements,
    );
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties(
        physicalDevice: VkPhysicalDevice,
        format: VkFormat,
        _type: VkImageType,
        samples: VkSampleCountFlags,
        usage: VkImageUsageFlags,
        tiling: VkImageTiling,
        pPropertyCount: *mut u32,
        pProperties: *mut VkSparseImageFormatProperties,
    );
    pub fn vkQueueBindSparse(
        queue: VkQueue,
        bindInfoCount: u32,
        pBindInfo: *const VkBindSparseInfo,
        fence: VkFence,
    ) -> VkResult;
    pub fn vkCreateFence(
        device: VkDevice,
        pCreateInfo: *const VkFenceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFence: *mut VkFence,
    ) -> VkResult;
    pub fn vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
    pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult;
    pub fn vkWaitForFences(
        device: VkDevice,
        fenceCount: u32,
        pFences: *const VkFence,
        waitAll: VkBool32,
        timeout: u64,
    ) -> VkResult;
    pub fn vkCreateSemaphore(
        device: VkDevice,
        pCreateInfo: *const VkSemaphoreCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSemaphore: *mut VkSemaphore,
    ) -> VkResult;
    pub fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateEvent(
        device: VkDevice,
        pCreateInfo: *const VkEventCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pEvent: *mut VkEvent,
    ) -> VkResult;
    pub fn vkDestroyEvent(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkCreateQueryPool(
        device: VkDevice,
        pCreateInfo: *const VkQueryPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pQueryPool: *mut VkQueryPool,
    ) -> VkResult;
    pub fn vkDestroyQueryPool(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetQueryPoolResults(
        device: VkDevice,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
        dataSize: size_t,
        pData: *mut c_void,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags,
    ) -> VkResult;
    pub fn vkCreateBuffer(
        device: VkDevice,
        pCreateInfo: *const VkBufferCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pBuffer: *mut VkBuffer,
    ) -> VkResult;
    pub fn vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateBufferView(
        device: VkDevice,
        pCreateInfo: *const VkBufferViewCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pView: *mut VkBufferView,
    ) -> VkResult;
    pub fn vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateImage(
        device: VkDevice,
        pCreateInfo: *const VkImageCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pImage: *mut VkImage,
    ) -> VkResult;
    pub fn vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetImageSubresourceLayout(
        device: VkDevice,
        image: VkImage,
        pSubresource: *const VkImageSubresource,
        pLayout: *mut VkSubresourceLayout,
    );
    pub fn vkCreateImageView(
        device: VkDevice,
        pCreateInfo: *const VkImageViewCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pView: *mut VkImageView,
    ) -> VkResult;
    pub fn vkDestroyImageView(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateShaderModule(
        device: VkDevice,
        pCreateInfo: *const VkShaderModuleCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pShaderModule: *mut VkShaderModule,
    ) -> VkResult;
    pub fn vkDestroyShaderModule(
        device: VkDevice,
        shaderModule: VkShaderModule,
        pAllocator: *const VkAllocationCallbacks,
    );
    pub fn vkCreatePipelineCache(
        device: VkDevice,
        pCreateInfo: *const VkPipelineCacheCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineCache: *mut VkPipelineCache,
    ) -> VkResult;
    pub fn vkDestroyPipelineCache(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        pAllocator: *const VkAllocationCallbacks,
    );
    pub fn vkGetPipelineCacheData(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        pDataSize: *mut size_t,
        pData: *mut c_void,
    ) -> VkResult;
    pub fn vkMergePipelineCaches(
        device: VkDevice,
        dstCache: VkPipelineCache,
        srcCacheCount: u32,
        pSrcCaches: *const VkPipelineCache,
    ) -> VkResult;
    pub fn vkCreateGraphicsPipelines(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkGraphicsPipelineCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> VkResult;
    pub fn vkCreateComputePipelines(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkComputePipelineCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> VkResult;
    pub fn vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreatePipelineLayout(
        device: VkDevice,
        pCreateInfo: *const VkPipelineLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineLayout: *mut VkPipelineLayout,
    ) -> VkResult;
    pub fn vkDestroyPipelineLayout(
        device: VkDevice,
        pipelineLayout: VkPipelineLayout,
        pAllocator: *const VkAllocationCallbacks,
    );
    pub fn vkCreateSampler(
        device: VkDevice,
        pCreateInfo: *const VkSamplerCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSampler: *mut VkSampler,
    ) -> VkResult;
    pub fn vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateDescriptorSetLayout(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSetLayout: *mut VkDescriptorSetLayout,
    ) -> VkResult;
    pub fn vkDestroyDescriptorSetLayout(
        device: VkDevice,
        descriptorSetLayout: VkDescriptorSetLayout,
        pAllocator: *const VkAllocationCallbacks,
    );
    pub fn vkCreateDescriptorPool(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDescriptorPool: *mut VkDescriptorPool,
    ) -> VkResult;
    pub fn vkDestroyDescriptorPool(
        device: VkDevice,
        descriptorPool: VkDescriptorPool,
        pAllocator: *const VkAllocationCallbacks,
    );
    pub fn vkResetDescriptorPool(
        device: VkDevice,
        descriptorPool: VkDescriptorPool,
        flags: VkDescriptorPoolResetFlags,
    ) -> VkResult;
    pub fn vkAllocateDescriptorSets(
        device: VkDevice,
        pAllocateInfo: *const VkDescriptorSetAllocateInfo,
        pDescriptorSets: *mut VkDescriptorSet,
    ) -> VkResult;
    pub fn vkFreeDescriptorSets(
        device: VkDevice,
        descriptorPool: VkDescriptorPool,
        descriptorSetCount: u32,
        pDescriptorSets: *const VkDescriptorSet,
    ) -> VkResult;
    pub fn vkUpdateDescriptorSets(
        device: VkDevice,
        descriptorWriteCount: u32,
        pDescriptorWrites: *const VkWriteDescriptorSet,
        descriptorCopyCount: u32,
        pDescriptorCopies: *const VkCopyDescriptorSet,
    );
    pub fn vkCreateFramebuffer(
        device: VkDevice,
        pCreateInfo: *const VkFramebufferCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFramebuffer: *mut VkFramebuffer,
    ) -> VkResult;
    pub fn vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateRenderPass(
        device: VkDevice,
        pCreateInfo: *const VkRenderPassCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pRenderPass: *mut VkRenderPass,
    ) -> VkResult;
    pub fn vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
    pub fn vkCreateCommandPool(
        device: VkDevice,
        pCreateInfo: *const VkCommandPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pCommandPool: *mut VkCommandPool,
    ) -> VkResult;
    pub fn vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags)
        -> VkResult;
    pub fn vkAllocateCommandBuffers(
        device: VkDevice,
        pAllocateInfo: *const VkCommandBufferAllocateInfo,
        pCommandBuffers: *mut VkCommandBuffer,
    ) -> VkResult;
    pub fn vkFreeCommandBuffers(
        device: VkDevice,
        commandPool: VkCommandPool,
        commandBufferCount: u32,
        pCommandBuffers: *const VkCommandBuffer,
    );
    pub fn vkBeginCommandBuffer(
        commandBuffer: VkCommandBuffer,
        pBeginInfo: *const VkCommandBufferBeginInfo,
    ) -> VkResult;
    pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult;
    pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;

    pub fn vkCmdBindPipeline(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        pipeline: VkPipeline,
    );
    pub fn vkCmdSetViewport(
        commandBuffer: VkCommandBuffer,
        firstViewport: u32,
        viewportCount: u32,
        pViewports: *const VkViewport,
    );
    pub fn vkCmdSetScissor(
        commandBuffer: VkCommandBuffer,
        firstScissor: u32,
        scissorCount: u32,
        pScissors: *const VkRect2D,
    );
    pub fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: c_float);
    pub fn vkCmdSetDepthBias(
        commandBuffer: VkCommandBuffer,
        depthBiasConstantFactor: c_float,
        depthBiasClamp: c_float,
        depthBiasSlopeFactor: c_float,
    );
    pub fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: *const c_float);
    pub fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: c_float, maxDepthBounds: c_float);
    pub fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
    pub fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
    pub fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
    pub fn vkCmdBindDescriptorSets(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        firstSet: u32,
        descriptorSetCount: u32,
        pDescriptorSets: *const VkDescriptorSet,
        dynamicOffsetCount: u32,
        pDynamicOffsets: *const u32,
    );
    pub fn vkCmdBindIndexBuffer(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        indexType: VkIndexType,
    );
    pub fn vkCmdBindVertexBuffers(
        commandBuffer: VkCommandBuffer,
        firstBinding: u32,
        bindingCount: u32,
        pBuffers: *const VkBuffer,
        pOffsets: *const VkDeviceSize,
    );
    pub fn vkCmdDraw(
        commandBuffer: VkCommandBuffer,
        vertexCount: u32,
        instanceCount: u32,
        firstVertex: u32,
        firstInstance: u32,
    );
    pub fn vkCmdDrawIndexed(
        commandBuffer: VkCommandBuffer,
        indexCount: u32,
        instanceCount: u32,
        firstIndex: u32,
        vertexOffset: i32,
        firstInstance: u32,
    );
    pub fn vkCmdDrawIndirect(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        drawCount: u32,
        stride: u32,
    );
    pub fn vkCmdDrawIndexedIndirect(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        drawCount: u32,
        stride: u32,
    );
    pub fn vkCmdDispatch(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    pub fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
    pub fn vkCmdCopyBuffer(
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstBuffer: VkBuffer,
        regionCount: u32,
        pRegions: *const VkBufferCopy,
    );
    pub fn vkCmdCopyImage(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageCopy,
    );
    pub fn vkCmdBlitImage(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageBlit,
        filter: VkFilter,
    );
    pub fn vkCmdCopyBufferToImage(
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkBufferImageCopy,
    );
    pub fn vkCmdCopyImageToBuffer(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstBuffer: VkBuffer,
        regionCount: u32,
        pRegions: *const VkBufferImageCopy,
    );
    pub fn vkCmdUpdateBuffer(
        commandBuffer: VkCommandBuffer,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        dataSize: VkDeviceSize,
        pData: *const c_void,
    );
    pub fn vkCmdFillBuffer(
        commandBuffer: VkCommandBuffer,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        size: VkDeviceSize,
        data: u32,
    );
    pub fn vkCmdClearColorImage(
        commandBuffer: VkCommandBuffer,
        image: VkImage,
        imageLayout: VkImageLayout,
        pColor: *const VkClearColorValue,
        rangeCount: u32,
        pRanges: *const VkImageSubresourceRange,
    );
    pub fn vkCmdClearDepthStencilImage(
        commandBuffer: VkCommandBuffer,
        image: VkImage,
        imageLayout: VkImageLayout,
        pDepthStencil: *const VkClearDepthStencilValue,
        rangeCount: u32,
        pRanges: *const VkImageSubresourceRange,
    );
    pub fn vkCmdClearAttachments(
        commandBuffer: VkCommandBuffer,
        attachmentCount: u32,
        pAttachments: *const VkClearAttachment,
        rectCount: u32,
        pRects: *const VkClearRect,
    );
    pub fn vkCmdResolveImage(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageResolve,
    );
    pub fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    pub fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    pub fn vkCmdWaitEvents(
        commandBuffer: VkCommandBuffer,
        eventCount: u32,
        pEvents: *const VkEvent,
        srcStageMask: VkPipelineStageFlags,
        dstStageMask: VkPipelineStageFlags,
        memoryBarrierCount: u32,
        pMemoryBarriers: *const VkMemoryBarrier,
        bufferMemoryBarrierCount: u32,
        pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
        imageMemoryBarrierCount: u32,
        pImageMemoryBariers: *const VkImageMemoryBarrier,
    );
    pub fn vkCmdPipelineBarrier(
        commandBuffer: VkCommandBuffer,
        srcStageMask: VkPipelineStageFlags,
        dstStageMask: VkPipelineStageFlags,
        dependencyFlags: VkDependencyFlags,
        memoryBarrierCount: u32,
        pMemoryBarriers: *const VkMemoryBarrier,
        bufferMemoryBarrierCount: u32,
        pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
        imageMemoryBarrierCount: u32,
        pImageMemoryBarriers: *const VkImageMemoryBarrier,
    );
    pub fn vkCmdBeginQuery(
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        query: u32,
        flags: VkQueryControlFlags,
    );
    pub fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
    pub fn vkCmdResetQueryPool(
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
    );
    pub fn vkCmdWriteTimestamp(
        commandBuffer: VkCommandBuffer,
        pipelineStage: VkPipelineStageFlags,
        queryPool: VkQueryPool,
        query: u32,
    );
    pub fn vkCmdCopyQueryPoolResults(
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags,
    );
    pub fn vkCmdPushConstants(
        commandBuffer: VkCommandBuffer,
        layout: VkPipelineLayout,
        stageFlags: VkShaderStageFlags,
        offset: u32,
        size: u32,
        pValues: *const c_void,
    );
    pub fn vkCmdBeginRenderPass(
        commandBuffer: VkCommandBuffer,
        pRenderPassBegin: *const VkRenderPassBeginInfo,
        contents: VkSubpassContents,
    );
    pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
    pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer);
    pub fn vkCmdExecuteCommands(
        commandBuffer: VkCommandBuffer,
        commandBufferCount: u32,
        pCommandBuffers: *const VkCommandBuffer,
    );
    #[cfg(feature = "VK_KHR_push_descriptor")]
    pub fn vkCmdPushDescriptorSetKHR(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        set: u32,
        descriptorWriteCount: u32,
        pDescriptorWrites: *const VkWriteDescriptorSet,
    );
    #[cfg(feature = "VK_EXT_debug_marker")]
    pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);
    #[cfg(feature = "VK_EXT_debug_marker")]
    pub fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer);
    #[cfg(feature = "VK_EXT_debug_marker")]
    pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    pub fn vkCmdDrawIndirectCountAMD(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        countBuffer: VkBuffer,
        countBufferOffset: VkDeviceSize,
        maxDrawCount: u32,
        stride: u32,
    );
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    pub fn vkCmdDrawIndexedIndirectCountAMD(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        countBuffer: VkBuffer,
        countBufferOffset: VkDeviceSize,
        maxDrawCount: u32,
        stride: u32,
    );
    #[cfg(feature = "VK_KHX_device_group")]
    pub fn vkCmdSetDeviceMaskKHX(commandBuffer: VkCommandBuffer, deviceMask: u32);
    #[cfg(feature = "VK_KHX_device_group")]
    pub fn vkCmdDispatchBaseKHX(
        commandBuffer: VkCommandBuffer,
        baseGroupX: u32,
        baseGroupY: u32,
        baseGroupZ: u32,
        groupCountX: u32,
        groupCountY: u32,
        groupCountZ: u32,
    );
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    pub fn vkCmdProcessCommandsNVX(
        commandBuffer: VkCommandBuffer,
        pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX,
    );
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    pub fn vkCmdReserveSpaceForCommandsNVX(
        commandBuffer: VkCommandBuffer,
        pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX,
    );
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    pub fn vkCmdSetViewportWScalingNV(
        commandBuffer: VkCommandBuffer,
        firstViewport: u32,
        viewportCount: u32,
        pViewportWScalings: *const VkViewportWScalingNV,
    );
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    pub fn vkCmdDiscardRectangleEXT(
        commandBuffer: VkCommandBuffer,
        firstDiscardRectangle: u32,
        discardRectangleCount: u32,
        pDiscardRectangles: *const VkRect2D,
    );
}

// --- Extension Definitions --- //
macro_rules! ExportExtensions {
    ($fname: tt: $mname: ident) => {
        #[cfg(feature = $fname)]
        mod $mname;
        #[cfg(feature = $fname)]
        pub use self::$mname::*;
    };
}

ExportExtensions!("VK_KHR_surface": surface_khr);
ExportExtensions!("VK_KHR_swapchain": swapchain_khr);
ExportExtensions!("VK_KHR_display": display_khr);
ExportExtensions!("VK_KHR_display_swapchain": display_swapchain_khr);
ExportExtensions!("VK_KHR_xlib_surface": xlib_surface_khr);
ExportExtensions!("VK_KHR_xcb_surface": xcb_surface_khr);
ExportExtensions!("VK_KHR_wayland_surface": wayland_surface_khr);
// TODO: Mir support
ExportExtensions!("VK_KHR_android_surface": android_surface_khr);
ExportExtensions!("VK_KHR_win32_surface": win32_surface_khr);
ExportExtensions!("VK_KHR_sampler_mirror_clamp_to_edge": sampler_mirror_clamp_to_edge_khr);
ExportExtensions!("VK_KHR_external_memory_win32": external_memory_win32_khr);
ExportExtensions!("VK_KHR_external_memory_fd": external_memory_fd_khr);
ExportExtensions!("VK_KHR_win32_keyed_mutex": win32_keyed_mutex_khr);
ExportExtensions!("VK_KHR_external_semaphore_win32": external_semaphore_win32_khr);
ExportExtensions!("VK_KHR_external_semaphore_fd": external_semaphore_fd_khr);
ExportExtensions!("VK_KHR_push_descriptor": push_descriptor_khr);
ExportExtensions!("VK_KHR_incremental_present": incremental_present_khr);
ExportExtensions!("VK_KHR_shared_presentable_image": shared_presentable_image_khr);
ExportExtensions!("VK_KHR_external_fence_win32": external_fence_win32_khr);
ExportExtensions!("VK_KHR_external_fence_fd": external_fence_fd_khr);
ExportExtensions!("VK_KHR_get_surface_capabilities2": get_surface_capabilities2_khr);
ExportExtensions!("VK_EXT_debug_report": debug_report_ext);
ExportExtensions!("VK_EXT_debug_utils": debug_utils_ext);
ExportExtensions!("VK_NV_glsl_shader": glsl_shader_nv);
ExportExtensions!("VK_EXT_depth_range_unrestricted": depth_range_unrestricted_ext);
ExportExtensions!("VK_IMG_filter_cubic": filter_cubic_img);
ExportExtensions!("VK_AMD_rasterization_order": rasterization_order_amd);
ExportExtensions!("VK_EXT_debug_marker": debug_marker_ext);
ExportExtensions!("VK_AMD_gcn_shader": gcn_shader_amd);
ExportExtensions!("VK_NV_dedicated_allocation": dedicated_allocation_nv);
ExportExtensions!("VK_AMD_draw_indirect_count": draw_indirect_count_amd);
ExportExtensions!("VK_AMD_negative_viewport_height": negative_viewport_height_amd);
ExportExtensions!("VK_AMD_gpu_shader_half_float": gpu_shader_half_float_amd);
ExportExtensions!("VK_AMD_shader_ballot": shader_ballot_amd);
ExportExtensions!("VK_AMD_texture_gather_bias_lod": texture_gather_bias_lod_amd);
ExportExtensions!("VK_AMD_shader_info": shader_info_amd);
ExportExtensions!("VK_AMD_shader_image_load_store_lod": shader_image_load_store_lod_amd);
ExportExtensions!("VK_IMG_format_pvrtc": format_pvrtc_img);
ExportExtensions!("VK_NV_external_memory_capabilities": external_memory_capabilities_nv);
ExportExtensions!("VK_NV_external_memory": external_memory_nv);
ExportExtensions!("VK_NV_external_memory_win32": external_memory_win32_nv);
ExportExtensions!("VK_NV_win32_keyed_mutex": win32_keyed_mutex_nv);
ExportExtensions!("VK_EXT_validation_flags": validation_flags_ext);
ExportExtensions!("VK_NN_vi_surface": vi_surface_nn);
ExportExtensions!("VK_EXT_shader_group_ballot": shader_group_ballot_ext);
ExportExtensions!("VK_EXT_shader_group_vote": shader_group_vote_ext);
ExportExtensions!("VK_NVX_device_generated_commands": device_generated_commands_nvx);
ExportExtensions!("VK_NV_clip_space_w_scaling": clip_space_w_scaling_nv);
ExportExtensions!("VK_EXT_direct_mode_display": direct_mode_display_ext);
ExportExtensions!("VK_EXT_acquire_xlib_display": acquire_xlib_display_ext);
ExportExtensions!("VK_EXT_display_surface_counter": display_surface_counter_ext);
ExportExtensions!("VK_EXT_display_control": display_control_ext);
ExportExtensions!("VK_GOOGLE_display_timing": display_timing_google);
ExportExtensions!("VK_NV_sample_mask_override_coverage": sample_mask_override_coverage_nv);
ExportExtensions!("VK_NV_geometry_shader_passthrough": geometry_shader_passthrough_nv);
ExportExtensions!("VK_NV_viewport_array2": viewport_array2_nv);
ExportExtensions!("VK_NVX_multiview_per_view_attributes": multiview_per_view_attributes_nvx);
ExportExtensions!("VK_NV_viewport_swizzle": viewport_swizzle_nv);
ExportExtensions!("VK_EXT_discard_rectangles": discard_rectangles_ext);
ExportExtensions!("VK_EXT_conservative_rasterization": conservative_rasterization_ext);
ExportExtensions!("VK_EXT_swapchain_colorspace": swapchain_colorspace_ext);
ExportExtensions!("VK_EXT_hdr_metadata": hdr_metadata_ext);
ExportExtensions!("VK_MVK_ios_surface": ios_surface_mvk);
ExportExtensions!("VK_MVK_macos_surface": macos_surface_mvk);
ExportExtensions!("VK_EXT_external_memory_dma_buf": external_memory_dma_buf_ext);
// ExportExtensions!("VK_EXT_queue_family_foreign": queue_family_foreign);
ExportExtensions!("VK_EXT_sampler_filter_minmax": sampler_filter_minmax_ext);
ExportExtensions!("VK_AMD_gpu_shader_int16": gpu_shader_int16_amd);
ExportExtensions!("VK_AMD_mixed_attachment_samples": mixed_attachment_samples_amd);
ExportExtensions!("VK_AMD_shader_fragment_mask": shader_fragment_mask_amd);
ExportExtensions!("VK_EXT_sample_locations": sample_locations_ext);
ExportExtensions!("VK_EXT_blend_operation_advanced": blend_operation_advanced_ext);
ExportExtensions!("VK_NV_fragment_coverage_to_color": fragment_coverage_to_color_nv);
ExportExtensions!("VK_NV_framebuffer_mixed_samples": framebuffer_mixed_samples_nv);
ExportExtensions!("VK_NV_fill_rectangle": fill_rectangle_nv);
ExportExtensions!("VK_EXT_post_depth_coverage": post_depth_coverage_ext);
ExportExtensions!("VK_EXT_validation_cache": validation_cache_ext);
ExportExtensions!("VK_EXT_shader_viewport_index_layer": shader_viewport_index_layer_ext);
ExportExtensions!("VK_EXT_global_priority": global_priority_ext);
ExportExtensions!("VK_EXT_external_memory_host": external_memory_host_ext);
ExportExtensions!("VK_AMD_buffer_marker": buffer_marker_amd);
ExportExtensions!("VK_EXT_vertex_attribute_divisor": vertex_attribute_divisor_ext);
ExportExtensions!("VK_EXT_full_screen_exclusive": full_screen_exclusive_ext);
ExportExtensions!("VK_KHR_image_format_list": image_format_list_khr);
ExportExtensions!("VK_EXT_image_drm_format_modifier": image_drm_format_modifier);

// Promoted Extensions
ExportExtensions!("VK_KHR_multiview": multiview_khr);
ExportExtensions!("VK_KHR_get_physical_device_properties2": get_physical_device_properties2);
ExportExtensions!("VK_KHR_device_group": device_group_khr);
ExportExtensions!("VK_KHR_shader_draw_parameters": shader_draw_parameters_khr);
ExportExtensions!("VK_KHR_maintenance1": maintenance1_khr);
ExportExtensions!("VK_KHR_device_group_creation": device_group_creation_khr);
ExportExtensions!("VK_KHR_external_memory_capabilities": external_memory_capabilities_khr);
ExportExtensions!("VK_KHR_external_memory": external_memory_khr);
ExportExtensions!("VK_KHR_external_semaphore_capabilities": external_semaphore_capabilities_khr);
ExportExtensions!("VK_KHR_external_semaphore": external_semaphore_khr);
ExportExtensions!("VK_KHR_16bit_storage": n16bit_storage_khr);
ExportExtensions!("VK_KHR_descriptor_update_template": descriptor_update_template_khr);
ExportExtensions!("VK_KHR_external_fence_capabilities": external_fence_capabilities_khr);
ExportExtensions!("VK_KHR_external_fence": external_fence_khr);
ExportExtensions!("VK_KHR_maintenance2": maintenance2_khr);
ExportExtensions!("VK_KHR_variable_pointers": variable_pointers_khr);
ExportExtensions!("VK_KHR_dedicated_allocation": dedicated_allocation_khr);
ExportExtensions!("VK_KHR_storage_buffer_storage_class": storage_buffer_storage_class_khr);
ExportExtensions!("VK_KHR_relaxed_block_layout": relaxed_block_layout_khr);
ExportExtensions!("VK_KHR_get_memory_requirements2": get_memory_requirements2_khr);
ExportExtensions!("VK_KHR_sampler_ycbcr_conversion": sampler_ycbcr_conversion_khr);
ExportExtensions!("VK_KHR_bind_memory2": bind_memory2_khr);
ExportExtensions!("VK_KHR_maintenance3": maintenance3_khr);
