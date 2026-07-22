#![allow(clippy::inconsistent_digit_grouping)]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, dead_code)]

//! Vulkan API Definitions 1.4.305 with some extensions

/*
** Copyright 2025 S.Percentage.
** Original C Header: Copyright 2015-2025 The Khronos Group Inc.
**
** SPDX-License-Identifier: Apache-2.0
*/

#[inline]
pub const fn VK_MAKE_VERSION(variant: u8, major: u16, minor: u16, patch: u16) -> u32 {
    ((variant as u32) << 29) | ((major as u32) << 22) | ((minor as u32) << 12) | patch as u32
}

pub const fn VK_VARIANT_VERSION(v: u32) -> u8 {
    (v >> 29) as _
}

#[inline]
pub const fn VK_MAJOR_VERSION(v: u32) -> u16 {
    ((v >> 22) & 0x7f) as _
}

#[inline]
pub const fn VK_MINOR_VERSION(v: u32) -> u16 {
    ((v >> 12) & 0x3ff) as _
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

/// Vulkan 1.0 version number
pub const VK_API_VERSION_1_0: u32 = VK_MAKE_VERSION(0, 1, 0, 0);

/// Version of this file
pub const VK_HEADER_VERSION: u16 = 305;
pub const VK_HEADER_VERSION_COMPLETE: u32 = VK_MAKE_VERSION(0, 1, 4, VK_HEADER_VERSION);

#[rustfmt::skip]
pub type VkSampleMask = u32;
#[rustfmt::skip]
pub type VkBool32 = u32;
#[rustfmt::skip]
pub type VkFlags = u32;
#[rustfmt::skip]
pub type VkFlags64 = u64;
#[rustfmt::skip]
pub type VkDeviceSize = u64;
#[rustfmt::skip]
pub type VkDeviceAddress = u64;

#[rustfmt::skip]
pub type VkAccessFlags = VkFlags;
#[rustfmt::skip]
pub type VkAccessFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_ACCESS_INDEX_READ_BIT: VkAccessFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_ACCESS_UNIFORM_READ_BIT: VkAccessFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits = 0x00000010;
#[rustfmt::skip]
pub const VK_ACCESS_SHADER_READ_BIT: VkAccessFlagBits = 0x00000020;
#[rustfmt::skip]
pub const VK_ACCESS_SHADER_WRITE_BIT: VkAccessFlagBits = 0x00000040;
#[rustfmt::skip]
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits = 0x00000080;
#[rustfmt::skip]
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = 0x00000100;
#[rustfmt::skip]
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits = 0x00000200;
#[rustfmt::skip]
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = 0x00000400;
#[rustfmt::skip]
pub const VK_ACCESS_TRANSFER_READ_BIT: VkAccessFlagBits = 0x00000800;
#[rustfmt::skip]
pub const VK_ACCESS_TRANSFER_WRITE_BIT: VkAccessFlagBits = 0x00001000;
#[rustfmt::skip]
pub const VK_ACCESS_HOST_READ_BIT: VkAccessFlagBits = 0x00002000;
#[rustfmt::skip]
pub const VK_ACCESS_HOST_WRITE_BIT: VkAccessFlagBits = 0x00004000;
#[rustfmt::skip]
pub const VK_ACCESS_MEMORY_READ_BIT: VkAccessFlagBits = 0x00008000;
#[rustfmt::skip]
pub const VK_ACCESS_MEMORY_WRITE_BIT: VkAccessFlagBits = 0x00010000;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: VkAccessFlagBits = 0x00080000;

#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub type VkAccessFlags2KHR = VkFlags64;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkAccessFlags2 = VkAccessFlags2KHR;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub type VkAccessFlagBits2KHR = VkFlags64;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkAccessFlagBits2 = VkAccessFlagBits2KHR;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000001;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000000001;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_INDEX_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000002;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_INDEX_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000000002;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000004;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000000004;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_UNIFORM_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000008;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_UNIFORM_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000000008;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000010;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000000010;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_SHADER_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000020;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_SHADER_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000000020;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_SHADER_WRITE_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000040;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_SHADER_WRITE_BIT: VkAccessFlagBits2KHR = 0x0000000000000040;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000080;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000000080;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000100;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits2KHR = 0x0000000000000100;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000200;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000000200;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000400;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits2KHR = 0x0000000000000400;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_TRANSFER_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000000800;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_TRANSFER_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000000800;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000001000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_TRANSFER_WRITE_BIT: VkAccessFlagBits2KHR = 0x0000000000001000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_HOST_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000002000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_HOST_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000002000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_HOST_WRITE_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000004000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_HOST_WRITE_BIT: VkAccessFlagBits2KHR = 0x0000000000004000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_MEMORY_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000008000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_MEMORY_READ_BIT: VkAccessFlagBits2KHR = 0x0000000000008000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_MEMORY_WRITE_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000000010000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_MEMORY_WRITE_BIT: VkAccessFlagBits2KHR = 0x0000000000010000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000100000000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT: VkAccessFlagBits2KHR = 0x0000000100000000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000200000000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT: VkAccessFlagBits2KHR = 0x0000000200000000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR: VkAccessFlagBits2KHR = 0x0000000400000000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT: VkAccessFlagBits2KHR = 0x0000000400000000;

#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip]
pub type VkAndroidSurfaceCreateFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip]
pub type VkAndroidSurfaceCreateFlagBitsKHR = VkFlags;

#[rustfmt::skip]
pub type VkAttachmentDescriptionFlags = VkFlags;
#[rustfmt::skip]
pub type VkAttachmentDescriptionFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkAttachmentDescriptionFlagBits = 0x00000001;

#[rustfmt::skip]
pub type VkBufferCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkBufferCreateFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT: VkBufferCreateFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT: VkBufferCreateFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT: VkBufferCreateFlagBits = 0x00000004;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_CREATE_PROTECTED_BIT: VkBufferCreateFlagBits = 0x00000008;

#[rustfmt::skip]
pub type VkBufferUsageFlags = VkFlags;
#[rustfmt::skip]
pub type VkBufferUsageFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: VkBufferUsageFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: VkBufferUsageFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkBufferUsageFlagBits = 0x00000010;
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: VkBufferUsageFlagBits = 0x00000020;
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: VkBufferUsageFlagBits = 0x00000040;
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: VkBufferUsageFlagBits = 0x00000080;
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: VkBufferUsageFlagBits = 0x00000100;

#[rustfmt::skip]
pub type VkBufferViewCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkBufferViewCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkColorComponentFlags = VkFlags;
#[rustfmt::skip]
pub type VkColorComponentFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_COLOR_COMPONENT_R_BIT: VkColorComponentFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_COLOR_COMPONENT_G_BIT: VkColorComponentFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_COLOR_COMPONENT_B_BIT: VkColorComponentFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_COLOR_COMPONENT_A_BIT: VkColorComponentFlagBits = 0x00000008;

#[rustfmt::skip]
pub type VkCommandBufferResetFlags = VkFlags;
#[rustfmt::skip]
pub type VkCommandBufferResetFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: VkCommandBufferResetFlagBits = 0x00000001;

#[rustfmt::skip]
pub type VkCommandBufferUsageFlags = VkFlags;
#[rustfmt::skip]
pub type VkCommandBufferUsageFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: VkCommandBufferUsageFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: VkCommandBufferUsageFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: VkCommandBufferUsageFlagBits = 0x00000004;

#[rustfmt::skip]
pub type VkCommandPoolCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkCommandPoolCreateFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: VkCommandPoolCreateFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlagBits = 0x00000002;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_COMMAND_POOL_CREATE_PROTECTED_BIT: VkCommandPoolCreateFlagBits = 0x00000004;

#[rustfmt::skip]
pub type VkCommandPoolResetFlags = VkFlags;
#[rustfmt::skip]
pub type VkCommandPoolResetFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: VkCommandPoolResetFlagBits = 0x00000001;

#[cfg(feature = "VK_KHR_maintenance1")]
#[rustfmt::skip]
pub type VkCommandPoolTrimFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkCommandPoolTrimFlags = VkCommandPoolTrimFlagsKHR;
#[cfg(feature = "VK_KHR_maintenance1")]
#[rustfmt::skip]
pub type VkCommandPoolTrimFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkCommandPoolTrimFlagBits = VkCommandPoolTrimFlagBitsKHR;

#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub type VkCompositeAlphaFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub type VkCompositeAlphaFlagBitsKHR = VkFlags;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: VkCompositeAlphaFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR = 0x00000002;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR = 0x00000004;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: VkCompositeAlphaFlagBitsKHR = 0x00000008;

#[rustfmt::skip]
pub type VkCullModeFlags = VkFlags;
#[rustfmt::skip]
pub type VkCullModeFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_CULL_MODE_FRONT_BIT: VkCullModeFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_CULL_MODE_BACK_BIT: VkCullModeFlagBits = 0x00000002;

#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub type VkDebugReportFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub type VkDebugReportFlagBitsEXT = VkFlags;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkDebugReportFlagBitsEXT = 0x00000001;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_WARNING_BIT_EXT: VkDebugReportFlagBitsEXT = 0x00000002;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: VkDebugReportFlagBitsEXT = 0x00000004;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_ERROR_BIT_EXT: VkDebugReportFlagBitsEXT = 0x00000008;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT: VkDebugReportFlagBitsEXT = 0x00000010;

#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub type VkDebugUtilsMessageSeverityFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub type VkDebugUtilsMessageSeverityFlagBitsEXT = VkFlags;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00000001;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00000010;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00000100;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00001000;

#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub type VkDebugUtilsMessageTypeFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub type VkDebugUtilsMessageTypeFlagBitsEXT = VkFlags;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 0x00000001;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 0x00000002;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 0x00000004;

#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub type VkDebugUtilsMessengerCallbackDataFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub type VkDebugUtilsMessengerCallbackDataFlagBitsEXT = VkFlags;

#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub type VkDebugUtilsMessengerCreateFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub type VkDebugUtilsMessengerCreateFlagBitsEXT = VkFlags;

#[rustfmt::skip]
pub type VkDependencyFlags = VkFlags;
#[rustfmt::skip]
pub type VkDependencyFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_DEPENDENCY_BY_REGION_BIT: VkDependencyFlagBits = 0x00000001;
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR: VkDependencyFlagBits = 0x00000002;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT: VkDependencyFlagBits = 0x00000002;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR: VkDependencyFlagBits = 0x00000004;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT: VkDependencyFlagBits = 0x00000004;

#[rustfmt::skip]
pub type VkDescriptorPoolCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkDescriptorPoolCreateFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: VkDescriptorPoolCreateFlagBits = 0x00000001;

#[rustfmt::skip]
pub type VkDescriptorPoolResetFlags = VkFlags;
#[rustfmt::skip]
pub type VkDescriptorPoolResetFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkDescriptorSetLayoutCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkDescriptorSetLayoutCreateFlagBits = VkFlags;

#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDescriptorUpdateTemplateCreateFlags = VkDescriptorUpdateTemplateCreateFlagsKHR;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
pub type VkDescriptorUpdateTemplateCreateFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDescriptorUpdateTemplateCreateFlagBits = VkDescriptorUpdateTemplateCreateFlagBitsKHR;

#[rustfmt::skip]
pub type VkDeviceCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkDeviceCreateFlagBits = VkFlags;

#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub type VkDeviceGroupPresentModeFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub type VkDeviceGroupPresentModeFlagBitsKHR = VkFlags;

#[rustfmt::skip]
pub type VkDeviceQueueCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkDeviceQueueCreateFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT: VkDeviceQueueCreateFlagBits = 0x00000001;

#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub type VkDisplayModeCreateFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub type VkDisplayModeCreateFlagBitsKHR = VkFlags;

#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub type VkDisplayPlaneAlphaFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub type VkDisplayPlaneAlphaFlagBitsKHR = VkFlags;
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = 0x00000002;
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = 0x00000004;
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = 0x00000008;

#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub type VkDisplaySurfaceCreateFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub type VkDisplaySurfaceCreateFlagBitsKHR = VkFlags;

#[rustfmt::skip]
pub type VkEventCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkEventCreateFlagBits = VkFlags;

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub type VkExternalFenceFeatureFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalFenceFeatureFlags = VkExternalFenceFeatureFlagsKHR;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub type VkExternalFenceFeatureFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalFenceFeatureFlagBits = VkExternalFenceFeatureFlagBitsKHR;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR: VkExternalFenceFeatureFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT: VkExternalFenceFeatureFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR: VkExternalFenceFeatureFlagBitsKHR = 0x00000002;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT: VkExternalFenceFeatureFlagBitsKHR = 0x00000002;

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub type VkExternalFenceHandleTypeFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlagsKHR;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub type VkExternalFenceHandleTypeFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlagBitsKHR;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalFenceHandleTypeFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalFenceHandleTypeFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalFenceHandleTypeFlagBitsKHR = 0x00000002;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalFenceHandleTypeFlagBitsKHR = 0x00000002;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalFenceHandleTypeFlagBitsKHR = 0x00000004;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalFenceHandleTypeFlagBitsKHR = 0x00000004;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR: VkExternalFenceHandleTypeFlagBitsKHR = 0x00000008;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalFenceHandleTypeFlagBitsKHR = 0x00000008;

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub type VkExternalMemoryFeatureFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalMemoryFeatureFlags = VkExternalMemoryFeatureFlagsKHR;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub type VkExternalMemoryFeatureFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalMemoryFeatureFlagBits = VkExternalMemoryFeatureFlagBitsKHR;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR: VkExternalMemoryFeatureFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT: VkExternalMemoryFeatureFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR: VkExternalMemoryFeatureFlagBitsKHR = 0x00000002;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT: VkExternalMemoryFeatureFlagBitsKHR = 0x00000002;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR: VkExternalMemoryFeatureFlagBitsKHR = 0x00000004;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT: VkExternalMemoryFeatureFlagBitsKHR = 0x00000004;

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub type VkExternalMemoryHandleTypeFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlagsKHR;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub type VkExternalMemoryHandleTypeFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlagBitsKHR;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000002;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000002;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000004;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000004;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000008;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000008;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000010;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000010;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000020;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000020;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000040;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000040;

#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub type VkExternalSemaphoreFeatureFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalSemaphoreFeatureFlags = VkExternalSemaphoreFeatureFlagsKHR;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub type VkExternalSemaphoreFeatureFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalSemaphoreFeatureFlagBits = VkExternalSemaphoreFeatureFlagBitsKHR;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR: VkExternalSemaphoreFeatureFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT: VkExternalSemaphoreFeatureFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR: VkExternalSemaphoreFeatureFlagBitsKHR = 0x00000002;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT: VkExternalSemaphoreFeatureFlagBitsKHR = 0x00000002;

#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub type VkExternalSemaphoreHandleTypeFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlagsKHR;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub type VkExternalSemaphoreHandleTypeFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlagBitsKHR;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000002;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000002;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000004;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000004;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000008;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000008;
#[cfg(feature = "Allow1_1APIs")]
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000008;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000010;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalSemaphoreHandleTypeFlagBitsKHR = 0x00000010;

#[rustfmt::skip]
pub type VkFenceCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkFenceCreateFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlagBits = 0x00000001;

#[cfg(feature = "VK_KHR_external_fence")]
#[rustfmt::skip]
pub type VkFenceImportFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkFenceImportFlags = VkFenceImportFlagsKHR;
#[cfg(feature = "VK_KHR_external_fence")]
#[rustfmt::skip]
pub type VkFenceImportFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkFenceImportFlagBits = VkFenceImportFlagBitsKHR;
#[cfg(feature = "VK_KHR_external_fence")]
#[rustfmt::skip]
pub const VK_FENCE_IMPORT_TEMPORARY_BIT_KHR: VkFenceImportFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_FENCE_IMPORT_TEMPORARY_BIT: VkFenceImportFlagBitsKHR = 0x00000001;

#[rustfmt::skip]
pub type VkFormatFeatureFlags = VkFlags;
#[rustfmt::skip]
pub type VkFormatFeatureFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = 0x00000010;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits = 0x00000020;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits = 0x00000040;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits = 0x00000080;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits = 0x00000100;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits = 0x00000200;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT: VkFormatFeatureFlagBits = 0x00000400;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_BLIT_DST_BIT: VkFormatFeatureFlagBits = 0x00000800;
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits = 0x00001000;
#[cfg(feature = "VK_KHR_maintenance1")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR: VkFormatFeatureFlagBits = 0x00004000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits = 0x00004000;
#[cfg(feature = "VK_KHR_maintenance1")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR: VkFormatFeatureFlagBits = 0x00008000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT: VkFormatFeatureFlagBits = 0x00008000;

#[rustfmt::skip]
pub type VkFramebufferCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkFramebufferCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkInstanceCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkInstanceCreateFlagBits = VkFlags;
#[cfg(feature = "VK_KHR_portability_enumeration")]
#[rustfmt::skip]
pub const VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR: VkInstanceCreateFlagBits = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT: VkInstanceCreateFlagBits = 0x00000001;

#[rustfmt::skip]
pub type VkImageAspectFlags = VkFlags;
#[rustfmt::skip]
pub type VkImageAspectFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkImageAspectFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkImageAspectFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_METADATA_BIT: VkImageAspectFlagBits = 0x00000008;

#[rustfmt::skip]
pub type VkImageUsageFlags = VkFlags;
#[rustfmt::skip]
pub type VkImageUsageFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkImageUsageFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkImageUsageFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlagBits = 0x00000010;
#[rustfmt::skip]
pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlagBits = 0x00000020;
#[rustfmt::skip]
pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlagBits = 0x00000040;
#[rustfmt::skip]
pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkImageUsageFlagBits = 0x00000080;

#[rustfmt::skip]
pub type VkImageCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkImageCreateFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkImageCreateFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkImageCreateFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkImageCreateFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkImageCreateFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkImageCreateFlagBits = 0x00000010;
#[cfg(feature = "VK_KHR_maintenance1")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits = 0x00000020;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT: VkImageCreateFlagBits = 0x00000020;
#[cfg(feature = "VK_KHR_bind_memory2")]
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: VkImageCreateFlagBits = 0x00000040;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT: VkImageCreateFlagBits = 0x00000040;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits = 0x00000080;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT: VkImageCreateFlagBits = 0x00000080;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR: VkImageCreateFlagBits = 0x00000100;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT: VkImageCreateFlagBits = 0x00000100;
#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_ALIAS_BIT_KHR: VkImageCreateFlagBits = 0x00000400;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_ALIAS_BIT: VkImageCreateFlagBits = 0x00000400;
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_PROTECTED_BIT: VkImageCreateFlagBits = 0x00000800;

#[rustfmt::skip]
pub type VkImageViewCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkImageViewCreateFlagBits = VkFlags;

#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub type VkMemoryAllocateFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkMemoryAllocateFlags = VkMemoryAllocateFlagsKHR;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub type VkMemoryAllocateFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkMemoryAllocateFlagBits = VkMemoryAllocateFlagBitsKHR;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR: VkMemoryAllocateFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT: VkMemoryAllocateFlagBitsKHR = 0x00000001;

#[rustfmt::skip]
pub type VkMemoryHeapFlags = VkFlags;
#[rustfmt::skip]
pub type VkMemoryHeapFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT: VkMemoryHeapFlagBits = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_MEMORY_HEAP_MULTI_INSTANCE_BIT: VkMemoryHeapFlagBits = 0x00000002;

#[rustfmt::skip]
pub type VkMemoryPropertyFlags = VkFlags;
#[rustfmt::skip]
pub type VkMemoryPropertyFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlagBits = 0x00000010;
#[rustfmt::skip]
pub const VK_MEMORY_PROPERTY_PROTECTED_BIT: VkMemoryPropertyFlagBits = 0x00000020;

#[rustfmt::skip]
pub type VkMemoryMapFlags = VkFlags;
#[rustfmt::skip]
pub type VkMemoryMapFlagBits = VkFlags;

#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip]
pub type VkMetalSurfaceCreateFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip]
pub type VkMetalSurfaceCreateFlagBitsEXT = VkFlags;

#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub type VkPeerMemoryFeatureFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlagsKHR;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub type VkPeerMemoryFeatureFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPeerMemoryFeatureFlagBits = VkPeerMemoryFeatureFlagBitsKHR;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR: VkPeerMemoryFeatureFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT: VkPeerMemoryFeatureFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR: VkPeerMemoryFeatureFlagBitsKHR = 0x00000002;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT: VkPeerMemoryFeatureFlagBitsKHR = 0x00000002;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR: VkPeerMemoryFeatureFlagBitsKHR = 0x00000004;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT: VkPeerMemoryFeatureFlagBitsKHR = 0x00000004;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR: VkPeerMemoryFeatureFlagBitsKHR = 0x00000008;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT: VkPeerMemoryFeatureFlagBitsKHR = 0x00000008;

#[rustfmt::skip]
pub type VkPipelineCacheCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineCacheCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineCreateFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT: VkPipelineCreateFlagBits = 0x00000004;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: VkPipelineCreateFlagBits = 0x00000008;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT: VkPipelineCreateFlagBits = 0x00000008;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_DISPATCH_BASE_BIT_KHR: VkPipelineCreateFlagBits = 0x00000010;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_DISPATCH_BASE_BIT: VkPipelineCreateFlagBits = 0x00000010;

#[rustfmt::skip]
pub type VkPipelineLayoutCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineLayoutCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineDepthStencilStateCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineDynamicStateCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineDynamicStateCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineColorBlendStateCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineColorBlendStateCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineMultisampleStateCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineRasterizationStateCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineViewportStateCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineViewportStateCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineTessellationStateCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineInputAssemblyStateCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineVertexInputStateCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineShaderStageCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineShaderStageCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkPipelineStageFlags = VkFlags;
#[rustfmt::skip]
pub type VkPipelineStageFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkPipelineStageFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkPipelineStageFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits = 0x00000010;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits = 0x00000020;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits = 0x00000040;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits = 0x00000080;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = 0x00000100;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = 0x00000200;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits = 0x00000400;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits = 0x00000800;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkPipelineStageFlagBits = 0x00001000;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits = 0x00002000;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_HOST_BIT: VkPipelineStageFlagBits = 0x00004000;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits = 0x00008000;
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkPipelineStageFlagBits = 0x00010000;

#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub type VkPipelineStageFlags2KHR = VkFlags64;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkPipelineStageFlags2 = VkPipelineStageFlags2KHR;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub type VkPipelineStageFlagBits2KHR = VkFlags64;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkPipelineStageFlagBits2 = VkPipelineStageFlagBits2KHR;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000001;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000001;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000002;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000002;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000004;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000004;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000008;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000008;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000010;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000010;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000020;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000020;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000040;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000040;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000080;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000080;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000100;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000100;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000200;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000200;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000400;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000400;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000000800;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000000800;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000001000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000001000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000002000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000002000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_HOST_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000004000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_HOST_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000004000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000008000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000008000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000000010000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT: VkPipelineStageFlagBits2KHR = 0x0000000000010000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_COPY_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000100000000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_COPY_BIT: VkPipelineStageFlagBits2KHR = 0x0000000100000000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000200000000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT: VkPipelineStageFlagBits2KHR = 0x0000000200000000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_BLIT_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000400000000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_BLIT_BIT: VkPipelineStageFlagBits2KHR = 0x0000000400000000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000000800000000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_CLEAR_BIT: VkPipelineStageFlagBits2KHR = 0x0000000800000000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000001000000000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT: VkPipelineStageFlagBits2KHR = 0x0000001000000000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000002000000000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT: VkPipelineStageFlagBits2KHR = 0x0000002000000000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR: VkPipelineStageFlagBits2KHR = 0x0000004000000000;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT: VkPipelineStageFlagBits2KHR = 0x0000004000000000;

#[rustfmt::skip]
pub type VkQueryControlFlags = VkFlags;
#[rustfmt::skip]
pub type VkQueryControlFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_QUERY_CONTROL_PRECISE_BIT: VkQueryControlFlagBits = 0x00000001;

#[rustfmt::skip]
pub type VkQueryPipelineStatisticFlags = VkFlags;
#[rustfmt::skip]
pub type VkQueryPipelineStatisticFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: VkQueryPipelineStatisticFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = 0x00000010;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 0x00000020;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = 0x00000040;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 0x00000080;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: VkQueryPipelineStatisticFlagBits = 0x00000100;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 0x00000200;
#[rustfmt::skip]
pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 0x00000400;

#[rustfmt::skip]
pub type VkQueryPoolCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkQueryPoolCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkQueryResultFlags = VkFlags;
#[rustfmt::skip]
pub type VkQueryResultFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_QUERY_RESULT_64_BIT: VkQueryResultFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_QUERY_RESULT_WAIT_BIT: VkQueryResultFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT: VkQueryResultFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_QUERY_RESULT_PARTIAL_BIT: VkQueryResultFlagBits = 0x00000008;

#[rustfmt::skip]
pub type VkQueueFlags = VkFlags;
#[rustfmt::skip]
pub type VkQueueFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_QUEUE_GRAPHICS_BIT: VkQueueFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_QUEUE_COMPUTE_BIT: VkQueueFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_QUEUE_TRANSFER_BIT: VkQueueFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_QUEUE_SPARSE_BINDING_BIT: VkQueueFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_QUEUE_PROTECTED_BIT: VkQueueFlagBits = 0x00000010;

#[rustfmt::skip]
pub type VkRenderPassCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkRenderPassCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkSampleCountFlags = VkFlags;
#[rustfmt::skip]
pub type VkSampleCountFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_SAMPLE_COUNT_1_BIT: VkSampleCountFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_SAMPLE_COUNT_2_BIT: VkSampleCountFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_SAMPLE_COUNT_4_BIT: VkSampleCountFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_SAMPLE_COUNT_8_BIT: VkSampleCountFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_SAMPLE_COUNT_16_BIT: VkSampleCountFlagBits = 0x00000010;
#[rustfmt::skip]
pub const VK_SAMPLE_COUNT_32_BIT: VkSampleCountFlagBits = 0x00000020;
#[rustfmt::skip]
pub const VK_SAMPLE_COUNT_64_BIT: VkSampleCountFlagBits = 0x00000040;

#[rustfmt::skip]
pub type VkSamplerCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkSamplerCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkSemaphoreCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkSemaphoreCreateFlagBits = VkFlags;

#[cfg(feature = "VK_KHR_external_semaphore")]
#[rustfmt::skip]
pub type VkSemaphoreImportFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSemaphoreImportFlags = VkSemaphoreImportFlagsKHR;
#[cfg(feature = "VK_KHR_external_semaphore")]
#[rustfmt::skip]
pub type VkSemaphoreImportFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSemaphoreImportFlagBits = VkSemaphoreImportFlagBitsKHR;
#[cfg(feature = "VK_KHR_external_semaphore")]
#[rustfmt::skip]
pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR: VkSemaphoreImportFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT: VkSemaphoreImportFlagBitsKHR = 0x00000001;

#[rustfmt::skip]
pub type VkShaderModuleCreateFlags = VkFlags;
#[rustfmt::skip]
pub type VkShaderModuleCreateFlagBits = VkFlags;

#[rustfmt::skip]
pub type VkShaderStageFlags = VkFlags;
#[rustfmt::skip]
pub type VkShaderStageFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlagBits = 0x00000004;
#[rustfmt::skip]
pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlagBits = 0x00000008;
#[rustfmt::skip]
pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlagBits = 0x00000010;
#[rustfmt::skip]
pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlagBits = 0x00000020;

#[rustfmt::skip]
pub type VkSparseMemoryBindFlags = VkFlags;
#[rustfmt::skip]
pub type VkSparseMemoryBindFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT: VkSparseMemoryBindFlagBits = 0x00000001;

#[rustfmt::skip]
pub type VkSparseImageFormatFlags = VkFlags;
#[rustfmt::skip]
pub type VkSparseImageFormatFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: VkSparseImageFormatFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: VkSparseImageFormatFlagBits = 0x00000002;
#[rustfmt::skip]
pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: VkSparseImageFormatFlagBits = 0x00000004;

#[rustfmt::skip]
pub type VkStencilFaceFlags = VkFlags;
#[rustfmt::skip]
pub type VkStencilFaceFlagBits = VkFlags;
#[rustfmt::skip]
pub const VK_STENCIL_FACE_FRONT_BIT: VkStencilFaceFlagBits = 0x00000001;
#[rustfmt::skip]
pub const VK_STENCIL_FACE_BACK_BIT: VkStencilFaceFlagBits = 0x00000002;

#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSubgroupFeatureFlags = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSubgroupFeatureFlagBits = VkFlags;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_BASIC_BIT: VkSubgroupFeatureFlagBits = 0x00000001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_VOTE_BIT: VkSubgroupFeatureFlagBits = 0x00000002;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_ARITHMETIC_BIT: VkSubgroupFeatureFlagBits = 0x00000004;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_BALLOT_BIT: VkSubgroupFeatureFlagBits = 0x00000008;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_SHUFFLE_BIT: VkSubgroupFeatureFlagBits = 0x00000010;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT: VkSubgroupFeatureFlagBits = 0x00000020;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_CLUSTERED_BIT: VkSubgroupFeatureFlagBits = 0x00000040;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_QUAD_BIT: VkSubgroupFeatureFlagBits = 0x00000080;

#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub type VkSubmitFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkSubmitFlags = VkSubmitFlagsKHR;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub type VkSubmitFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkSubmitFlagBits = VkSubmitFlagBitsKHR;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_SUBMIT_PROTECTED_BIT_KHR: VkSubmitFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_SUBMIT_PROTECTED_BIT: VkSubmitFlagBitsKHR = 0x00000001;

#[rustfmt::skip]
pub type VkSubpassDescriptionFlags = VkFlags;
#[rustfmt::skip]
pub type VkSubpassDescriptionFlagBits = VkFlags;

#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub type VkSurfaceTransformFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub type VkSurfaceTransformFlagBitsKHR = VkFlags;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 0x00000002;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 0x00000004;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 0x00000008;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 0x00000010;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 0x00000020;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 0x00000040;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 0x00000080;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 0x00000100;

#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub type VkSwapchainCreateFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub type VkSwapchainCreateFlagBitsKHR = VkFlags;
#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub const VK_SWAPCHAIN_CREATE_SPLIT_INSTNACE_BIND_REGIONS_BIT_KHR: VkSwapchainCreateFlagBitsKHR = 0x00000001;

#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
pub type VkWaylandSurfaceCreateFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
pub type VkWaylandSurfaceCreateFlagBitsKHR = VkFlags;

#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
pub type VkWin32SurfaceCreateFlagBitsKHR = VkFlags;

#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
pub type VkXcbSurfaceCreateFlagBitsKHR = VkFlags;

#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
pub type VkXlibSurfaceCreateFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
pub type VkXlibSurfaceCreateFlagBitsKHR = VkFlags;
#[rustfmt::skip]
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlags = 0x0000001f;
#[rustfmt::skip]
pub const VK_SHADER_STAGE_ALL: VkShaderStageFlags = 0x7fffffff;
#[rustfmt::skip]
pub const VK_CULL_MODE_NONE: VkCullModeFlags = 0;
#[rustfmt::skip]
pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlags = 3;
#[rustfmt::skip]
pub const VK_STENCIL_FACE_FRONT_AND_BACK: VkStencilFaceFlags = 3;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_NONE_KHR: VkPipelineStageFlagBits2KHR = 0;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_2_NONE: VkPipelineStageFlagBits2 = 0;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_ACCESS_2_NONE_KHR: VkAccessFlagBits2KHR = 0;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_ACCESS_2_NONE: VkAccessFlagBits2 = 0;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkInstance(pub core::ptr::NonNull<core::ffi::c_void>, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkInstance {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_INSTANCE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.as_ptr().addr() as _
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_INSTANCE: VkObjectType = 1;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkPhysicalDevice(pub core::ptr::NonNull<core::ffi::c_void>, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkPhysicalDevice {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_PHYSICAL_DEVICE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.as_ptr().addr() as _
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: VkObjectType = 2;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkDevice(pub core::ptr::NonNull<core::ffi::c_void>, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkDevice {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_DEVICE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.as_ptr().addr() as _
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DEVICE: VkObjectType = 3;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkQueue(pub core::ptr::NonNull<core::ffi::c_void>, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkQueue {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_QUEUE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.as_ptr().addr() as _
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_QUEUE: VkObjectType = 4;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkCommandBuffer(pub core::ptr::NonNull<core::ffi::c_void>, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkCommandBuffer {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_COMMAND_BUFFER;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.as_ptr().addr() as _
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_COMMAND_BUFFER: VkObjectType = 6;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkDeviceMemory(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkDeviceMemory {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_DEVICE_MEMORY;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DEVICE_MEMORY: VkObjectType = 8;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkCommandPool(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkCommandPool {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_COMMAND_POOL;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_COMMAND_POOL: VkObjectType = 25;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkBuffer(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkBuffer {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_BUFFER;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_BUFFER: VkObjectType = 9;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkBufferView(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkBufferView {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_BUFFER_VIEW;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_BUFFER_VIEW: VkObjectType = 13;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkImage(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkImage {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_IMAGE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_IMAGE: VkObjectType = 10;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkImageView(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkImageView {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_IMAGE_VIEW;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_IMAGE_VIEW: VkObjectType = 14;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkShaderModule(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkShaderModule {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_SHADER_MODULE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_SHADER_MODULE: VkObjectType = 15;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkPipeline(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkPipeline {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_PIPELINE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_PIPELINE: VkObjectType = 19;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkPipelineLayout(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkPipelineLayout {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_PIPELINE_LAYOUT;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: VkObjectType = 17;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkSampler(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkSampler {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_SAMPLER;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_SAMPLER: VkObjectType = 21;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkDescriptorSet(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkDescriptorSet {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_DESCRIPTOR_SET;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: VkObjectType = 23;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkDescriptorSetLayout(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkDescriptorSetLayout {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: VkObjectType = 20;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkDescriptorPool(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkDescriptorPool {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_DESCRIPTOR_POOL;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL: VkObjectType = 22;

#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkDescriptorUpdateTemplateKHR(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkDescriptorUpdateTemplateKHR {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR: VkObjectType = 1000085000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDescriptorUpdateTemplate = VkDescriptorUpdateTemplateKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE: VkObjectType = 1000085000;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkFence(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkFence {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_FENCE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_FENCE: VkObjectType = 7;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkSemaphore(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkSemaphore {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_SEMAPHORE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_SEMAPHORE: VkObjectType = 5;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkEvent(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkEvent {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_EVENT;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_EVENT: VkObjectType = 11;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkQueryPool(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkQueryPool {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_QUERY_POOL;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_QUERY_POOL: VkObjectType = 12;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkFramebuffer(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkFramebuffer {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_FRAMEBUFFER;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_FRAMEBUFFER: VkObjectType = 24;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkRenderPass(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkRenderPass {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_RENDER_PASS;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_RENDER_PASS: VkObjectType = 18;

#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkPipelineCache(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[rustfmt::skip]
impl crate::VkRawHandle for VkPipelineCache {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_PIPELINE_CACHE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_PIPELINE_CACHE: VkObjectType = 16;

#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkDisplayKHR(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkDisplayKHR {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_DISPLAY_KHR;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DISPLAY_KHR: VkObjectType = 1000002000;

#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkDisplayModeKHR(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkDisplayModeKHR {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_DISPLAY_MODE_KHR;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DISPLAY_MODE_KHR: VkObjectType = 1000002001;

#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkSurfaceKHR(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkSurfaceKHR {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_SURFACE_KHR;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_SURFACE_KHR: VkObjectType = 1000000000;

#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkSwapchainKHR(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkSwapchainKHR {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_SWAPCHAIN_KHR;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_SWAPCHAIN_KHR: VkObjectType = 1000001000;

#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkDebugReportCallbackEXT(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkDebugReportCallbackEXT {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT: VkObjectType = 1000011000;

#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkDebugUtilsMessengerEXT(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkDebugUtilsMessengerEXT {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT: VkObjectType = 1000128000;
#[rustfmt::skip]
pub const VK_LOD_CLAMP_NONE: f32 = 1000.0;
#[rustfmt::skip]
pub const VK_REMAINING_MIP_LEVELS: u32 = 0xffff_ffff;
#[rustfmt::skip]
pub const VK_REMAINING_ARRAY_LAYERS: u32 = 0xffff_ffff;
#[rustfmt::skip]
pub const VK_WHOLE_SIZE: u64 = !0;
#[rustfmt::skip]
pub const VK_ATTACHMENT_UNUSED: u32 = 0xffff_ffff;
#[rustfmt::skip]
pub const VK_TRUE: VkBool32 = 1;
#[rustfmt::skip]
pub const VK_FALSE: VkBool32 = 0;
#[rustfmt::skip]
pub const VK_QUEUE_FAMILY_IGNORED: u32 = 0xffff_ffff;
#[rustfmt::skip]
pub const VK_SUBPASS_EXTERNAL: u32 = 0xffff_ffff;
#[rustfmt::skip]
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
#[rustfmt::skip]
pub const VK_UUID_SIZE: usize = 16;
#[rustfmt::skip]
pub const VK_MAX_MEMORY_TYPES: usize = 32;
#[rustfmt::skip]
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
#[rustfmt::skip]
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
#[rustfmt::skip]
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
#[cfg(any(feature = "VK_KHR_external_fence_capabilities", feature = "VK_KHR_external_memory_capabilities", feature = "VK_KHR_external_semaphore_capabilities"))]
#[rustfmt::skip]
pub const VK_LUID_SIZE_KHR: usize = 8;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_LUID_SIZE: usize = 8;
#[cfg(feature = "VK_KHR_external_memory")]#[rustfmt::skip]
pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: u32 = !1;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = !1;
#[cfg(feature = "VK_EXT_queue_family_foreign")]#[rustfmt::skip]
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = !0u32 - 2;
#[cfg(feature = "VK_KHR_device_group_creation")]#[rustfmt::skip]
pub const VK_MAX_DEVICE_GROUP_SIZE_KHR: usize = 32;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]
pub const VK_MAX_GLOBAL_PRIORITY_SIZE_KHR: usize = 16;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]
pub const VK_MAX_GLOBAL_PRIORITY_SIZE: usize = 16;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")] #[rustfmt::skip] pub const VK_RESOLVE_MODE_NONE_KHR: VkResolveModeFlagBitsKHR = 0;
#[cfg(feature = "Allow1_2APIs")] #[rustfmt::skip] pub const VK_RESOLVE_MODE_NONE: VkResolveModeFlagBitsKHR = 0;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkResult(pub i32);
#[rustfmt::skip]
pub const VK_SUCCESS: VkResult = VkResult(0);
#[rustfmt::skip]
pub const VK_NOT_READY: VkResult = VkResult(1);
#[rustfmt::skip]
pub const VK_TIMEOUT: VkResult = VkResult(2);
#[rustfmt::skip]
pub const VK_EVENT_SET: VkResult = VkResult(3);
#[rustfmt::skip]
pub const VK_EVENT_RESET: VkResult = VkResult(4);
#[rustfmt::skip]
pub const VK_INCOMPLETE: VkResult = VkResult(5);
#[rustfmt::skip]
pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = VkResult(-1);
#[rustfmt::skip]
pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = VkResult(-2);
#[rustfmt::skip]
pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = VkResult(-3);
#[rustfmt::skip]
pub const VK_ERROR_DEVICE_LOST: VkResult = VkResult(-4);
#[rustfmt::skip]
pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = VkResult(-5);
#[rustfmt::skip]
pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = VkResult(-6);
#[rustfmt::skip]
pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = VkResult(-7);
#[rustfmt::skip]
pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = VkResult(-8);
#[rustfmt::skip]
pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = VkResult(-9);
#[rustfmt::skip]
pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = VkResult(-10);
#[rustfmt::skip]
pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = VkResult(-11);
#[rustfmt::skip]
pub const VK_ERROR_FRAGMENTED_POOL: VkResult = VkResult(-12);
#[rustfmt::skip]
pub const VK_ERROR_UNKNOWN: VkResult = VkResult(-13);
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_ERROR_SURFACE_LOST_KHR: VkResult = VkResult(-1000000000);
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult = VkResult(-1000000001);
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub const VK_SUBOPTIMAL_KHR: VkResult = VkResult(1000001003);
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub const VK_ERROR_OUT_OF_DATE_KHR: VkResult = VkResult(-1000001004);
#[cfg(feature = "VK_KHR_display_swapchain")]
#[rustfmt::skip]
pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: VkResult = VkResult(-1000003001);
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR: VkResult = VkResult(-1000072003);
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE: VkResult = VkResult(-1000072003);
#[cfg(feature = "VK_KHR_maintenance1")]
#[rustfmt::skip]
pub const VK_ERROR_OUT_OF_POOL_MEMORY_KHR: VkResult = VkResult(-1000069000);
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_ERROR_OUT_OF_POOL_MEMORY: VkResult = VkResult(-1000069000);

#[rustfmt::skip]
pub type VkStructureType = i32;
#[rustfmt::skip]
pub type VkObjectType = i32;

#[rustfmt::skip]
pub type VkAttachmentLoadOp = i32;
#[rustfmt::skip]
pub const VK_ATTACHMENT_LOAD_OP_LOAD: VkAttachmentLoadOp = 0;
#[rustfmt::skip]
pub const VK_ATTACHMENT_LOAD_OP_CLEAR: VkAttachmentLoadOp = 1;
#[rustfmt::skip]
pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: VkAttachmentLoadOp = 2;

#[rustfmt::skip]
pub type VkAttachmentStoreOp = i32;
#[rustfmt::skip]
pub const VK_ATTACHMENT_STORE_OP_STORE: VkAttachmentStoreOp = 0;
#[rustfmt::skip]
pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: VkAttachmentStoreOp = 1;

#[rustfmt::skip]
pub type VkBlendFactor = i32;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_ZERO: VkBlendFactor = 0;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_ONE: VkBlendFactor = 1;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_SRC_COLOR: VkBlendFactor = 2;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: VkBlendFactor = 3;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_DST_COLOR: VkBlendFactor = 4;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: VkBlendFactor = 5;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_SRC_ALPHA: VkBlendFactor = 6;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: VkBlendFactor = 7;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_DST_ALPHA: VkBlendFactor = 8;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: VkBlendFactor = 9;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_CONSTANT_COLOR: VkBlendFactor = 10;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: VkBlendFactor = 11;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: VkBlendFactor = 12;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: VkBlendFactor = 13;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_SRC_ALPHA_STAURATE: VkBlendFactor = 14;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_SRC1_COLOR: VkBlendFactor = 15;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: VkBlendFactor = 16;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_SRC1_ALPHA: VkBlendFactor = 17;
#[rustfmt::skip]
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: VkBlendFactor = 18;

#[rustfmt::skip]
pub type VkBlendOp = i32;
#[rustfmt::skip]
pub const VK_BLEND_OP_ADD: VkBlendOp = 0;
#[rustfmt::skip]
pub const VK_BLEND_OP_SUBTRACT: VkBlendOp = 1;
#[rustfmt::skip]
pub const VK_BLEND_OP_REVERSE_SUBTRACT: VkBlendOp = 2;
#[rustfmt::skip]
pub const VK_BLEND_OP_MIN: VkBlendOp = 3;
#[rustfmt::skip]
pub const VK_BLEND_OP_MAX: VkBlendOp = 4;

#[rustfmt::skip]
pub type VkBorderColor = i32;
#[rustfmt::skip]
pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: VkBorderColor = 0;
#[rustfmt::skip]
pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: VkBorderColor = 1;
#[rustfmt::skip]
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: VkBorderColor = 2;
#[rustfmt::skip]
pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK: VkBorderColor = 3;
#[rustfmt::skip]
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: VkBorderColor = 4;
#[rustfmt::skip]
pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE: VkBorderColor = 5;

#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub type VkColorSpaceKHR = i32;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = 0;

#[rustfmt::skip]
pub type VkCommandBufferLevel = i32;
#[rustfmt::skip]
pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: VkCommandBufferLevel = 0;
#[rustfmt::skip]
pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: VkCommandBufferLevel = 1;

#[rustfmt::skip]
pub type VkCompareOp = i32;
#[rustfmt::skip]
pub const VK_COMPARE_OP_NEVER: VkCompareOp = 0;
#[rustfmt::skip]
pub const VK_COMPARE_OP_LESS: VkCompareOp = 1;
#[rustfmt::skip]
pub const VK_COMPARE_OP_EQUAL: VkCompareOp = 2;
#[rustfmt::skip]
pub const VK_COMPARE_OP_LESS_OR_EQUAL: VkCompareOp = 3;
#[rustfmt::skip]
pub const VK_COMPARE_OP_GREATER: VkCompareOp = 4;
#[rustfmt::skip]
pub const VK_COMPARE_OP_NOT_EQUAL: VkCompareOp = 5;
#[rustfmt::skip]
pub const VK_COMPARE_OP_GREATER_OR_EQUAL: VkCompareOp = 6;
#[rustfmt::skip]
pub const VK_COMPARE_OP_ALWAYS: VkCompareOp = 7;

#[rustfmt::skip]
pub type VkComponentSwizzle = i32;
#[rustfmt::skip]
pub const VK_COMPONENT_SWIZZLE_IDENTITY: VkComponentSwizzle = 0;
#[rustfmt::skip]
pub const VK_COMPONENT_SWIZZLE_ZERO: VkComponentSwizzle = 1;
#[rustfmt::skip]
pub const VK_COMPONENT_SWIZZLE_ONE: VkComponentSwizzle = 2;
#[rustfmt::skip]
pub const VK_COMPONENT_SWIZZLE_R: VkComponentSwizzle = 3;
#[rustfmt::skip]
pub const VK_COMPONENT_SWIZZLE_G: VkComponentSwizzle = 4;
#[rustfmt::skip]
pub const VK_COMPONENT_SWIZZLE_B: VkComponentSwizzle = 5;
#[rustfmt::skip]
pub const VK_COMPONENT_SWIZZLE_A: VkComponentSwizzle = 6;

#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub type VkDebugReportObjectTypeEXT = i32;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: VkDebugReportObjectTypeEXT = 0;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: VkDebugReportObjectTypeEXT = 1;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: VkDebugReportObjectTypeEXT = 2;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: VkDebugReportObjectTypeEXT = 3;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: VkDebugReportObjectTypeEXT = 4;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: VkDebugReportObjectTypeEXT = 5;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: VkDebugReportObjectTypeEXT = 6;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: VkDebugReportObjectTypeEXT = 7;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: VkDebugReportObjectTypeEXT = 8;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: VkDebugReportObjectTypeEXT = 9;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: VkDebugReportObjectTypeEXT = 10;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: VkDebugReportObjectTypeEXT = 11;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: VkDebugReportObjectTypeEXT = 12;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: VkDebugReportObjectTypeEXT = 13;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: VkDebugReportObjectTypeEXT = 14;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: VkDebugReportObjectTypeEXT = 15;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: VkDebugReportObjectTypeEXT = 16;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: VkDebugReportObjectTypeEXT = 17;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: VkDebugReportObjectTypeEXT = 18;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: VkDebugReportObjectTypeEXT = 19;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT: VkDebugReportObjectTypeEXT = 20;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: VkDebugReportObjectTypeEXT = 21;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: VkDebugReportObjectTypeEXT = 22;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: VkDebugReportObjectTypeEXT = 23;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: VkDebugReportObjectTypeEXT = 24;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: VkDebugReportObjectTypeEXT = 25;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: VkDebugReportObjectTypeEXT = 26;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: VkDebugReportObjectTypeEXT = 27;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: VkDebugReportObjectTypeEXT = 28;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: VkDebugReportObjectTypeEXT = 29;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: VkDebugReportObjectTypeEXT = 30;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_OBJECT_TABLE_NVX_EXT: VkDebugReportObjectTypeEXT = 31;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX_EXT: VkDebugReportObjectTypeEXT = 32;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT: VkDebugReportObjectTypeEXT = 33;
#[cfg(feature = "VK_EXT_debug_report")]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR: VkDebugReportObjectTypeEXT = 1000085000;
#[cfg(feature = "VK_EXT_debug_report")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE: VkDebugReportObjectTypeEXT = 1000085000;

#[rustfmt::skip]
pub type VkDescriptorType = i32;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_SAMPLER: VkDescriptorType = 0;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: VkDescriptorType = 1;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: VkDescriptorType = 2;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: VkDescriptorType = 3;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: VkDescriptorType = 4;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: VkDescriptorType = 5;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: VkDescriptorType = 6;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: VkDescriptorType = 7;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: VkDescriptorType = 8;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: VkDescriptorType = 9;
#[rustfmt::skip]
pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: VkDescriptorType = 10;

#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
pub type VkDescriptorUpdateTemplateTypeKHR = i32;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDescriptorUpdateTemplateType = i32;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR: VkDescriptorUpdateTemplateTypeKHR = 0;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET: VkDescriptorUpdateTemplateTypeKHR = 0;

#[rustfmt::skip]
pub type VkDynamicState = i32;
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_VIEWPORT: VkDynamicState = 0;
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_SCISSOR: VkDynamicState = 1;
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_LINE_WIDTH: VkDynamicState = 2;
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_DEPTH_BIAS: VkDynamicState = 3;
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: VkDynamicState = 4;
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: VkDynamicState = 5;
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: VkDynamicState = 6;
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: VkDynamicState = 7;
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: VkDynamicState = 8;

#[rustfmt::skip]
pub type VkFilter = i32;
#[rustfmt::skip]
pub const VK_FILTER_NEAREST: VkFilter = 0;
#[rustfmt::skip]
pub const VK_FILTER_LINEAR: VkFilter = 1;

#[rustfmt::skip]
pub type VkFrontFace = i32;
#[rustfmt::skip]
pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: VkFrontFace = 0;
#[rustfmt::skip]
pub const VK_FRONT_FACE_CLOCKWISE: VkFrontFace = 1;

#[rustfmt::skip]
pub type VkImageLayout = i32;
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_UNDEFINED: VkImageLayout = 0;
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_GENERAL: VkImageLayout = 1;
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: VkImageLayout = 2;
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 3;
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 4;
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: VkImageLayout = 5;
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: VkImageLayout = 6;
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: VkImageLayout = 7;
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_PREINITIALIZED: VkImageLayout = 8;
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR: VkImageLayout = 1000001002;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = 1000117000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 1000117000;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: VkImageLayout = 1000117001;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 1000117001;

#[rustfmt::skip]
pub type VkImageTiling = i32;
#[rustfmt::skip]
pub const VK_IMAGE_TILING_OPTIMAL: VkImageTiling = 0;
#[rustfmt::skip]
pub const VK_IMAGE_TILING_LINEAR: VkImageTiling = 1;

#[rustfmt::skip]
pub type VkImageType = i32;
#[rustfmt::skip]
pub const VK_IMAGE_TYPE_1D: VkImageType = 0;
#[rustfmt::skip]
pub const VK_IMAGE_TYPE_2D: VkImageType = 1;
#[rustfmt::skip]
pub const VK_IMAGE_TYPE_3D: VkImageType = 2;

#[rustfmt::skip]
pub type VkImageViewType = i32;
#[rustfmt::skip]
pub const VK_IMAGE_VIEW_TYPE_1D: VkImageViewType = 0;
#[rustfmt::skip]
pub const VK_IMAGE_VIEW_TYPE_2D: VkImageViewType = 1;
#[rustfmt::skip]
pub const VK_IMAGE_VIEW_TYPE_3D: VkImageViewType = 2;
#[rustfmt::skip]
pub const VK_IMAGE_VIEW_TYPE_CUBE: VkImageViewType = 3;
#[rustfmt::skip]
pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: VkImageViewType = 4;
#[rustfmt::skip]
pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: VkImageViewType = 5;
#[rustfmt::skip]
pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: VkImageViewType = 6;

#[rustfmt::skip]
pub type VkIndexType = i32;
#[rustfmt::skip]
pub const VK_INDEX_TYPE_UINT16: VkIndexType = 0;
#[rustfmt::skip]
pub const VK_INDEX_TYPE_UINT32: VkIndexType = 1;

#[rustfmt::skip]
pub type VkLogicOp = i32;
#[rustfmt::skip]
pub const VK_LOGIC_OP_CLEAR: VkLogicOp = 0;
#[rustfmt::skip]
pub const VK_LOGIC_OP_AND: VkLogicOp = 1;
#[rustfmt::skip]
pub const VK_LOGIC_OP_AND_REVERSE: VkLogicOp = 2;
#[rustfmt::skip]
pub const VK_LOGIC_OP_COPY: VkLogicOp = 3;
#[rustfmt::skip]
pub const VK_LOGIC_OP_AND_INVERTED: VkLogicOp = 4;
#[rustfmt::skip]
pub const VK_LOGIC_OP_NO_OP: VkLogicOp = 5;
#[rustfmt::skip]
pub const VK_LOGIC_OP_XOR: VkLogicOp = 6;
#[rustfmt::skip]
pub const VK_LOGIC_OP_OR: VkLogicOp = 7;
#[rustfmt::skip]
pub const VK_LOGIC_OP_NOR: VkLogicOp = 8;
#[rustfmt::skip]
pub const VK_LOGIC_OP_EQUIVALENT: VkLogicOp = 9;
#[rustfmt::skip]
pub const VK_LOGIC_OP_INVERT: VkLogicOp = 10;
#[rustfmt::skip]
pub const VK_LOGIC_OP_OR_REVERSE: VkLogicOp = 11;
#[rustfmt::skip]
pub const VK_LOGIC_OP_COPY_INVERTED: VkLogicOp = 12;
#[rustfmt::skip]
pub const VK_LOGIC_OP_OR_INVERTED: VkLogicOp = 13;
#[rustfmt::skip]
pub const VK_LOGIC_OP_NAND: VkLogicOp = 14;
#[rustfmt::skip]
pub const VK_LOGIC_OP_SET: VkLogicOp = 15;

#[rustfmt::skip]
pub type VkInternalAllocationType = i32;
#[rustfmt::skip]
pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: VkInternalAllocationType = 0;

#[rustfmt::skip]
pub type VkPhysicalDeviceType = i32;
#[rustfmt::skip]
pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: VkPhysicalDeviceType = 0;
#[rustfmt::skip]
pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: VkPhysicalDeviceType = 1;
#[rustfmt::skip]
pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: VkPhysicalDeviceType = 2;
#[rustfmt::skip]
pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: VkPhysicalDeviceType = 3;
#[rustfmt::skip]
pub const VK_PHYSICAL_DEVICE_TYPE_CPU: VkPhysicalDeviceType = 4;

#[rustfmt::skip]
pub type VkPipelineBindPoint = i32;
#[rustfmt::skip]
pub const VK_PIPELINE_BIND_POINT_GRAPHICS: VkPipelineBindPoint = 0;
#[rustfmt::skip]
pub const VK_PIPELINE_BIND_POINT_COMPUTE: VkPipelineBindPoint = 1;

#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub type VkPointClippingBehaviorKHR = i32;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPointClippingBehavior = i32;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR: VkPointClippingBehaviorKHR = 0;
#[cfg(feature = "VK_KHR_maintenance2")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: VkPointClippingBehaviorKHR = 0;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_KHR: VkPointClippingBehaviorKHR = 1;
#[cfg(feature = "VK_KHR_maintenance2")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES: VkPointClippingBehaviorKHR = 1;

#[rustfmt::skip]
pub type VkPolygonMode = i32;
#[rustfmt::skip]
pub const VK_POLYGON_MODE_FILL: VkPolygonMode = 0;
#[rustfmt::skip]
pub const VK_POLYGON_MODE_LINE: VkPolygonMode = 1;
#[rustfmt::skip]
pub const VK_POLYGON_MODE_POINT: VkPolygonMode = 2;

#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub type VkPresentModeKHR = i32;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_PRESENT_MODE_IMMEDIATE_KHR: VkPresentModeKHR = 0;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_PRESENT_MODE_MAILBOX_KHR: VkPresentModeKHR = 1;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_PRESENT_MODE_FIFO_KHR: VkPresentModeKHR = 2;
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: VkPresentModeKHR = 3;

#[rustfmt::skip]
pub type VkPrimitiveTopology = i32;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: VkPrimitiveTopology = 0;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: VkPrimitiveTopology = 1;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: VkPrimitiveTopology = 2;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: VkPrimitiveTopology = 3;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: VkPrimitiveTopology = 4;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: VkPrimitiveTopology = 5;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = 6;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = 7;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = 8;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = 9;
#[rustfmt::skip]
pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: VkPrimitiveTopology = 10;

#[rustfmt::skip]
pub type VkQueryType = i32;
#[rustfmt::skip]
pub const VK_QUERY_TYPE_OCCLUSION: VkQueryType = 0;
#[rustfmt::skip]
pub const VK_QUERY_TYPE_PIPELINE_STATISTICS: VkQueryType = 1;
#[rustfmt::skip]
pub const VK_QUERY_TYPE_TIMESTAMP: VkQueryType = 2;

#[rustfmt::skip]
pub type VkSamplerAddressMode = i32;
#[rustfmt::skip]
pub const VK_SAMPLER_ADDRESS_MODE_REPEAT: VkSamplerAddressMode = 0;
#[rustfmt::skip]
pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: VkSamplerAddressMode = 1;
#[rustfmt::skip]
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: VkSamplerAddressMode = 2;
#[rustfmt::skip]
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: VkSamplerAddressMode = 3;

#[rustfmt::skip]
pub type VkSamplerMipmapMode = i32;
#[rustfmt::skip]
pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: VkSamplerMipmapMode = 0;
#[rustfmt::skip]
pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: VkSamplerMipmapMode = 1;

#[rustfmt::skip]
pub type VkSharingMode = i32;
#[rustfmt::skip]
pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = 0;
#[rustfmt::skip]
pub const VK_SHARING_MODE_CONCURRENT: VkSharingMode = 1;

#[rustfmt::skip]
pub type VkStencilOp = i32;
#[rustfmt::skip]
pub const VK_STENCIL_OP_KEEP: VkStencilOp = 0;
#[rustfmt::skip]
pub const VK_STENCIL_OP_ZERO: VkStencilOp = 1;
#[rustfmt::skip]
pub const VK_STENCIL_OP_REPLACE: VkStencilOp = 2;
#[rustfmt::skip]
pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP: VkStencilOp = 3;
#[rustfmt::skip]
pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP: VkStencilOp = 4;
#[rustfmt::skip]
pub const VK_STENCIL_OP_INVERT: VkStencilOp = 5;
#[rustfmt::skip]
pub const VK_STENCIL_OP_INCREMENT_AND_WRAP: VkStencilOp = 6;
#[rustfmt::skip]
pub const VK_STENCIL_OP_DECREMENT_AND_WRAP: VkStencilOp = 7;

#[rustfmt::skip]
pub type VkSubpassContents = i32;
#[rustfmt::skip]
pub const VK_SUBPASS_CONTENTS_INLINE: VkSubpassContents = 0;
#[rustfmt::skip]
pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: VkSubpassContents = 1;

#[rustfmt::skip]
pub type VkSystemAllocationScope = i32;
#[rustfmt::skip]
pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: VkSystemAllocationScope = 0;
#[rustfmt::skip]
pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: VkSystemAllocationScope = 1;
#[rustfmt::skip]
pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: VkSystemAllocationScope = 2;
#[rustfmt::skip]
pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: VkSystemAllocationScope = 3;
#[rustfmt::skip]
pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: VkSystemAllocationScope = 4;

#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub type VkTessellationDomainOriginKHR = i32;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkTessellationDomainOrigin = i32;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR: VkTessellationDomainOriginKHR = 0;
#[cfg(feature = "VK_KHR_maintenance2")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT: VkTessellationDomainOriginKHR = 0;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR: VkTessellationDomainOriginKHR = 1;
#[cfg(feature = "VK_KHR_maintenance2")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT: VkTessellationDomainOriginKHR = 1;

#[rustfmt::skip]
pub type VkVertexInputRate = i32;
#[rustfmt::skip]
pub const VK_VERTEX_INPUT_RATE_VERTEX: VkVertexInputRate = 0;
#[rustfmt::skip]
pub const VK_VERTEX_INPUT_RATE_INSTANCE: VkVertexInputRate = 1;

#[rustfmt::skip]
pub type VkFormat = i32;
#[rustfmt::skip]
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
#[cfg(feature = "VK_KHR_maintenance5")]
pub const VK_FORMAT_A1B5G5R4_UNORM_PACK16_KHR: VkFormat = 1000470000;
#[cfg(feature = "Allow1_4APIs")]
pub const VK_FORMAT_A1B5G5R4_UNORM_PACK16: VkFormat = 1000470000;
#[cfg(feature = "VK_KHR_maintenance5")]
pub const VK_FORMAT_A8_UNORM_KHR: VkFormat = 1000470001;
#[cfg(feature = "Allow1_4APIs")]
pub const VK_FORMAT_A8_UNORM: VkFormat = 1000470001;
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
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G8B8G8R8_422_UNORM_KHR: VkFormat = 1000156000;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G8B8G8R8_422_UNORM: VkFormat = 1000156000;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_B8G8R8G8_422_UNORM_KHR: VkFormat = 1000156001;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_B8G8R8G8_422_UNORM: VkFormat = 1000156001;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR: VkFormat = 1000156002;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM: VkFormat = 1000156002;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR: VkFormat = 1000156003;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM: VkFormat = 1000156003;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR: VkFormat = 1000156004;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM: VkFormat = 1000156004;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR: VkFormat = 1000156005;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM: VkFormat = 1000156005;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR: VkFormat = 1000156006;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM: VkFormat = 1000156006;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_R10X6_UNORM_PACK16_KHR: VkFormat = 1000156007;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_R10X6_UNORM_PACK16: VkFormat = 1000156007;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR: VkFormat = 1000156008;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: VkFormat = 1000156008;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: VkFormat = 1000156009;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: VkFormat = 1000156009;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: VkFormat = 1000156010;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: VkFormat = 1000156010;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: VkFormat = 1000156011;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: VkFormat = 1000156011;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: VkFormat = 1000156012;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: VkFormat = 1000156012;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: VkFormat = 1000156013;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: VkFormat = 1000156013;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: VkFormat = 1000156014;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: VkFormat = 1000156014;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: VkFormat = 1000156015;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: VkFormat = 1000156015;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: VkFormat = 1000156016;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: VkFormat = 1000156016;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_R12X4_UNORM_PACK16_KHR: VkFormat = 1000156017;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_R12X4_UNORM_PACK16: VkFormat = 1000156017;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR: VkFormat = 1000156018;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: VkFormat = 1000156018;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: VkFormat = 1000156019;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: VkFormat = 1000156019;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: VkFormat = 1000156020;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: VkFormat = 1000156020;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: VkFormat = 1000156021;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: VkFormat = 1000156021;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: VkFormat = 1000156022;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: VkFormat = 1000156022;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: VkFormat = 1000156023;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: VkFormat = 1000156023;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: VkFormat = 1000156024;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: VkFormat = 1000156024;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: VkFormat = 1000156025;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: VkFormat = 1000156025;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: VkFormat = 1000156026;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: VkFormat = 1000156026;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G16B16G16R16_422_UNORM_KHR: VkFormat = 1000156027;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G16B16G16R16_422_UNORM: VkFormat = 1000156027;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_B16G16R16G16_422_UNORM_KHR: VkFormat = 1000156028;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_B16G16R16G16_422_UNORM: VkFormat = 1000156028;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR: VkFormat = 1000156029;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM: VkFormat = 1000156029;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR: VkFormat = 1000156030;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM: VkFormat = 1000156030;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR: VkFormat = 1000156031;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM: VkFormat = 1000156031;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR: VkFormat = 1000156032;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM: VkFormat = 1000156032;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR: VkFormat = 1000156033;
#[cfg(feature = "Allow1_1APIs")]
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM: VkFormat = 1000156033;

#[rustfmt::skip]
pub type PFN_vkInternalAllocationNotification = extern "system" fn(pUserData: *mut core::ffi::c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

#[rustfmt::skip]
pub type PFN_vkInternalFreeNotification = extern "system" fn(pUserData: *mut core::ffi::c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

#[rustfmt::skip]
pub type PFN_vkReallocationFunction = extern "system" fn(pUserData: *mut core::ffi::c_void, pOriginal: *mut core::ffi::c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut core::ffi::c_void;

#[rustfmt::skip]
pub type PFN_vkAllocationFunction = extern "system" fn(pUserData: *mut core::ffi::c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut core::ffi::c_void;

#[rustfmt::skip]
pub type PFN_vkFreeFunction = extern "system" fn(pUserData: *mut core::ffi::c_void, pMemory: *mut core::ffi::c_void);

#[rustfmt::skip]
pub type PFN_vkVoidFunction = extern "system" fn();

#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub type PFN_vkDebugReportCallbackEXT = extern "system" fn(flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: usize, messageCode: i32, pLayerPrefix: *const core::ffi::c_char, pMessage: *const core::ffi::c_char, pUserData: *mut core::ffi::c_void) -> VkBool32;

#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub type PFN_vkDebugUtilsMessengerCallbackEXT = extern "system" fn(messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT, messageTypes: VkDebugUtilsMessageTypeFlagsEXT, pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT, pUserData: *mut core::ffi::c_void) -> VkBool32;
#[cfg(any(feature = "VK_KHR_external_fence_capabilities", feature = "VK_KHR_external_memory_capabilities", feature = "VK_KHR_external_semaphore_capabilities"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR:VkStructureType=1000071004;
#[cfg(any(feature = "VK_KHR_external_fence_capabilities", feature = "VK_KHR_external_memory_capabilities", feature = "VK_KHR_external_semaphore_capabilities"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceIDPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub deviceUUID:[u8; VK_UUID_SIZE],pub driverUUID:[u8; VK_UUID_SIZE],pub deviceLUID:[u8; VK_LUID_SIZE_KHR],pub deviceNodeMask:u32,pub deviceLUIDValid:VkBool32,}
#[cfg(any(feature = "VK_KHR_external_fence_capabilities", feature = "VK_KHR_external_memory_capabilities", feature = "VK_KHR_external_semaphore_capabilities"))]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceIDPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(any(feature = "VK_KHR_external_fence_capabilities", feature = "VK_KHR_external_memory_capabilities", feature = "VK_KHR_external_semaphore_capabilities"))]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceIDPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceIDProperties = VkPhysicalDeviceIDPropertiesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR;
#[cfg(feature = "VK_KHR_variable_pointers")]
#[rustfmt::skip]
pub type VkPhysicalDeviceVariablePointerFeaturesKHR = VkPhysicalDeviceVariablePointersFeaturesKHR;
#[cfg(feature = "VK_KHR_variable_pointers")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;

#[derive(Clone, Copy)]
#[repr(C)]
#[rustfmt::skip]
pub union VkClearColorValue {
    pub float32: [core::ffi::c_float; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

#[derive(Clone, Copy)]
#[repr(C)]
#[rustfmt::skip]
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depthStencil: VkClearDepthStencilValue,
}

#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_KHR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_surface";
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
pub const VK_KHR_SURFACE_SPEC_VERSION: usize = 25;

#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_swapchain";
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: usize = 68;

#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub const VK_KHR_DISPLAY_EXTENSION_NAME: &str = "VK_KHR_display";
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub const VK_KHR_DISPLAY_SPEC_VERSION: usize = 21;

#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &str = "VK_KHR_xlib_surface";
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
pub const VK_KHR_XLIB_SURFACE_SPEC_VERSION: usize = 6;

#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &str = "VK_KHR_xcb_surface";
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: usize = 6;

#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &str = "VK_KHR_wayland_surface";
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: usize = 6;

#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip]
pub const VK_KHR_ANDROID_SURFACE_EXTENSION_NAME: &str = "VK_KHR_android_surface";
#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip]
pub const VK_KHR_ANDROID_SURFACE_SPEC_VERSION: usize = 6;

#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &str = "VK_KHR_win32_surface";
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: usize = 6;

#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip]
pub const VK_EXT_METAL_SURFACE_EXTENSION_NAME: &str = "VK_EXT_metal_surface";
#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip]
pub const VK_EXT_METAL_SURFACE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[rustfmt::skip]
pub const VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: &str = "VK_KHR_win32_keyed_mutex";
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[rustfmt::skip]
pub const VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &str = "VK_KHR_external_memory";
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &str = "VK_KHR_external_memory_capabilities";
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &str = "VK_KHR_external_memory_win32";
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &str = "VK_KHR_external_memory_fd";
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_external_semaphore")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &str = "VK_KHR_external_semaphore";
#[cfg(feature = "VK_KHR_external_semaphore")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &str = "VK_KHR_external_semaphore_win32";
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &str = "VK_KHR_external_semaphore_fd";
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &str = "VK_KHR_external_fence_win32";
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_external_fence_fd")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: &str = "VK_KHR_external_fence_fd";
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[rustfmt::skip]
pub const VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
pub const VK_KHR_GET_SURFACE_CAPABILITIES2_EXTENSION_NAME: &str = "VK_KHR_get_surface_capabilities2";
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
pub const VK_KHR_GET_SURFACE_CAPABILITIES2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME: &str = "VK_EXT_debug_report";
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: usize = 8;

#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_EXT_DEBUG_UTILS_EXTENSION_NAME: &str = "VK_EXT_debug_utils";
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_EXT_DEBUG_UTILS_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_KHR_DEVICE_GROUP_EXTENSION_NAME: &str = "VK_KHR_device_group";
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_KHR_DEVICE_GROUP_SPEC_VERSION: usize = 4;

#[cfg(feature = "VK_KHR_device_group_creation")]
#[rustfmt::skip]
pub const VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: &str = "VK_KHR_device_group_creation";
#[cfg(feature = "VK_KHR_device_group_creation")]
#[rustfmt::skip]
pub const VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
pub const VK_KHR_MULTIVIEW_EXTENSION_NAME: &str = "VK_KHR_multiview";
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
pub const VK_KHR_MULTIVIEW_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION_NAME: &str = "VK_KHR_get_physical_device_properties2";
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_shader_draw_parameters")]
#[rustfmt::skip]
pub const VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME: &str = "VK_KHR_shader_draw_parameters";
#[cfg(feature = "VK_KHR_shader_draw_parameters")]
#[rustfmt::skip]
pub const VK_KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_portability_enumeration")]
#[rustfmt::skip]
pub const VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME: &str = "VK_KHR_portability_enumeration";
#[cfg(feature = "VK_KHR_portability_enumeration")]
#[rustfmt::skip]
pub const VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
pub const VK_KHR_GET_MEMORY_REQUIREMENTS2_EXTENSION_NAME: &str = "VK_KHR_get_memory_requirements2";
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
pub const VK_KHR_GET_MEMORY_REQUIREMENTS2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_relaxed_block_layout")]
#[rustfmt::skip]
pub const VK_KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME: &str = "VK_KHR_relaxed_block_layout";
#[cfg(feature = "VK_KHR_relaxed_block_layout")]
#[rustfmt::skip]
pub const VK_KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
#[rustfmt::skip]
pub const VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME: &str = "VK_KHR_storage_buffer_storage_class";
#[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
#[rustfmt::skip]
pub const VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
pub const VK_KHR_BIND_MEMORY2_EXTENSION_NAME: &str = "VK_KHR_bind_memory2";
#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
pub const VK_KHR_BIND_MEMORY2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_variable_pointers")]
#[rustfmt::skip]
pub const VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME: &str = "VK_KHR_variable_pointers";
#[cfg(feature = "VK_KHR_variable_pointers")]
#[rustfmt::skip]
pub const VK_KHR_VARIABLE_POINTERS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[rustfmt::skip]
pub const VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME: &str = "VK_KHR_dedicated_allocation";
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[rustfmt::skip]
pub const VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION: usize = 3;

#[cfg(feature = "VK_KHR_16bit_storage")]
#[rustfmt::skip]
pub const VK_KHR_16BIT_STORAGE_EXTENSION_NAME: &str = "VK_KHR_16bit_storage";
#[cfg(feature = "VK_KHR_16bit_storage")]
#[rustfmt::skip]
pub const VK_KHR_16BIT_STORAGE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_maintenance1")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE1_EXTENSION_NAME: &str = "VK_KHR_maintenance1";
#[cfg(feature = "VK_KHR_maintenance1")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE1_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE2_EXTENSION_NAME: &str = "VK_KHR_maintenance2";
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_maintenance3")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE3_EXTENSION_NAME: &str = "VK_KHR_maintenance3";
#[cfg(feature = "VK_KHR_maintenance3")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE3_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_KHR_SYNCHRONIZATION2_EXTENSION_NAME: &str = "VK_KHR_synchronization2";
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_KHR_SYNCHRONIZATION2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_shader_subgroup_vote")]
#[rustfmt::skip]
pub const VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME: &str = "VK_EXT_shader_subgroup_vote";
#[cfg(feature = "VK_EXT_shader_subgroup_vote")]
#[rustfmt::skip]
pub const VK_EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: &str = "VK_KHR_sampler_ycbcr_conversion";
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: usize = 14;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkSamplerYcbcrConversionKHR(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkSamplerYcbcrConversionKHR {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR: VkObjectType = 1000156000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSamplerYcbcrConversion = VkSamplerYcbcrConversionKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION: VkObjectType = 1000156000;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits = 0x00020000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits = 0x00020000;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: VkFormatFeatureFlagBits = 0x00040000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlagBits = 0x00040000;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: VkFormatFeatureFlagBits = 0x00080000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits = 0x00080000;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: VkFormatFeatureFlagBits = 0x00100000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits = 0x00100000;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: VkFormatFeatureFlagBits = 0x00200000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlagBits = 0x00200000;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_DISJOINT_BIT_KHR: VkFormatFeatureFlagBits = 0x00400000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_DISJOINT_BIT: VkFormatFeatureFlagBits = 0x00400000;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits = 0x00800000;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits = 0x00800000;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_PLANE_0_BIT_KHR: VkImageAspectFlagBits = 0x00000010;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_PLANE_0_BIT: VkImageAspectFlagBits = 0x00000010;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_PLANE_1_BIT_KHR: VkImageAspectFlagBits = 0x00000020;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_PLANE_1_BIT: VkImageAspectFlagBits = 0x00000020;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_PLANE_2_BIT_KHR: VkImageAspectFlagBits = 0x00000040;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_PLANE_2_BIT: VkImageAspectFlagBits = 0x00000040;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_DISJOINT_BIT_KHR: VkImageCreateFlagBits = 0x00000200;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_DISJOINT_BIT: VkImageCreateFlagBits = 0x00000200;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub type VkSamplerYcbcrModelConversionKHR = i32;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSamplerYcbcrModelConversion = i32;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR: VkSamplerYcbcrModelConversionKHR = 0;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: VkSamplerYcbcrModelConversionKHR = 0;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR: VkSamplerYcbcrModelConversionKHR = 1;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY: VkSamplerYcbcrModelConversionKHR = 1;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR: VkSamplerYcbcrModelConversionKHR = 2;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709: VkSamplerYcbcrModelConversionKHR = 2;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR: VkSamplerYcbcrModelConversionKHR = 3;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601: VkSamplerYcbcrModelConversionKHR = 3;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR: VkSamplerYcbcrModelConversionKHR = 4;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020: VkSamplerYcbcrModelConversionKHR = 4;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub type VkChromaLocationKHR = i32;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkChromaLocation = i32;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_CHROMA_LOCATION_COSITED_EVEN_KHR: VkChromaLocationKHR = 0;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_CHROMA_LOCATION_COSITED_EVEN: VkChromaLocationKHR = 0;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_CHROMA_LOCATION_MIDPOINT_KHR: VkChromaLocationKHR = 1;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_CHROMA_LOCATION_MIDPOINT: VkChromaLocationKHR = 1;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub type VkSamplerYcbcrRangeKHR = i32;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSamplerYcbcrRange = i32;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR: VkSamplerYcbcrRangeKHR = 0;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL: VkSamplerYcbcrRangeKHR = 0;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR: VkSamplerYcbcrRangeKHR = 1;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW: VkSamplerYcbcrRangeKHR = 1;

#[cfg(feature = "VK_EXT_debug_report")]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR: VkDebugReportObjectTypeEXT = 1000156000;
#[cfg(feature = "VK_EXT_debug_report")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION: VkDebugReportObjectTypeEXT = 1000156000;

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &str = "VK_KHR_buffer_device_address";
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: VkResult = VkResult(-1000257000);
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: VkResult = VkResult(-1000257000);

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkBufferCreateFlagBits = 0x00000010;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: VkBufferCreateFlagBits = 0x00000010;

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR: VkBufferUsageFlagBits = 0x00020000;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT: VkBufferUsageFlagBits = 0x00020000;

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR: VkMemoryAllocateFlagBits = 0x00000002;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT: VkMemoryAllocateFlagBits = 0x00000002;
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkMemoryAllocateFlagBits = 0x00000004;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: VkMemoryAllocateFlagBits = 0x00000004;

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: &str = "VK_KHR_timeline_semaphore";
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub type VkSemaphoreTypeKHR = i32;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkSemaphoreType = i32;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_SEMAPHORE_TYPE_BINARY_KHR: VkSemaphoreTypeKHR = 0;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_SEMAPHORE_TYPE_BINARY: VkSemaphoreTypeKHR = 0;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_SEMAPHORE_TYPE_TIMELINE_KHR: VkSemaphoreTypeKHR = 1;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_SEMAPHORE_TYPE_TIMELINE: VkSemaphoreTypeKHR = 1;

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub type VkSemaphoreWaitFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkSemaphoreWaitFlags = VkSemaphoreWaitFlagsKHR;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub type VkSemaphoreWaitFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkSemaphoreWaitFlagBits = VkSemaphoreWaitFlagBitsKHR;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_SEMAPHORE_WAIT_ANY_BIT_KHR: VkSemaphoreWaitFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_SEMAPHORE_WAIT_ANY_BIT: VkSemaphoreWaitFlagBitsKHR = 0x00000001;

#[cfg(feature = "VK_KHR_image_format_list")]
#[rustfmt::skip]
pub const VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME: &str = "VK_KHR_image_format_list";
#[cfg(feature = "VK_KHR_image_format_list")]
#[rustfmt::skip]
pub const VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[rustfmt::skip]
pub const VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: &str = "VK_EXT_sampler_filter_minmax";
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[rustfmt::skip]
pub const VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[rustfmt::skip]
pub type VkSamplerReductionModeEXT = i32;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkSamplerReductionMode = i32;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[rustfmt::skip]
pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT: VkSamplerReductionModeEXT = 0;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE: VkSamplerReductionModeEXT = 0;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[rustfmt::skip]
pub const VK_SAMPLER_REDUCTION_MODE_MIN_EXT: VkSamplerReductionModeEXT = 1;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_REDUCTION_MODE_MIN: VkSamplerReductionModeEXT = 1;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[rustfmt::skip]
pub const VK_SAMPLER_REDUCTION_MODE_MAX_EXT: VkSamplerReductionModeEXT = 2;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_REDUCTION_MODE_MAX: VkSamplerReductionModeEXT = 2;

#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: VkFormatFeatureFlagBits = 0x00010000;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlagBits = 0x00010000;

#[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
#[rustfmt::skip]
pub const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME: &str = "VK_KHR_sampler_mirror_clamp_to_edge";
#[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
#[rustfmt::skip]
pub const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION: usize = 3;

#[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
#[rustfmt::skip]
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE_KHR: VkSamplerAddressMode = 4;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE: VkSamplerAddressMode = 4;

#[cfg(feature = "VK_KHR_shader_float_controls")]
#[rustfmt::skip]
pub const VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME: &str = "VK_KHR_shader_float_controls";
#[cfg(feature = "VK_KHR_shader_float_controls")]
#[rustfmt::skip]
pub const VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION: usize = 4;

#[cfg(feature = "VK_KHR_shader_float_controls")]
#[rustfmt::skip]
pub type VkShaderFloatControlsIndependenceKHR = i32;
#[cfg(feature = "VK_KHR_shader_float_controls")]
#[rustfmt::skip]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR: VkShaderFloatControlsIndependenceKHR = 0;
#[cfg(feature = "VK_KHR_shader_float_controls")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY: VkShaderFloatControlsIndependenceKHR = 0;
#[cfg(feature = "VK_KHR_shader_float_controls")]
#[rustfmt::skip]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR: VkShaderFloatControlsIndependenceKHR = 1;
#[cfg(feature = "VK_KHR_shader_float_controls")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL: VkShaderFloatControlsIndependenceKHR = 1;
#[cfg(feature = "VK_KHR_shader_float_controls")]
#[rustfmt::skip]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR: VkShaderFloatControlsIndependenceKHR = 2;
#[cfg(feature = "VK_KHR_shader_float_controls")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE: VkShaderFloatControlsIndependenceKHR = 2;

#[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
#[rustfmt::skip]
pub const VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME: &str = "VK_EXT_shader_viewport_index_layer";
#[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
#[rustfmt::skip]
pub const VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_create_renderpass2")]
#[rustfmt::skip]
pub const VK_KHR_CREATE_RENDERPASS2_EXTENSION_NAME: &str = "VK_KHR_create_renderpass2";
#[cfg(feature = "VK_KHR_create_renderpass2")]
#[rustfmt::skip]
pub const VK_KHR_CREATE_RENDERPASS2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
#[rustfmt::skip]
pub const VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME: &str = "VK_KHR_depth_stencil_resolve";
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
#[rustfmt::skip]
pub const VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
#[rustfmt::skip]
pub type VkResolveModeFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkResolveModeFlags = VkResolveModeFlagsKHR;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
#[rustfmt::skip]
pub type VkResolveModeFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkResolveModeFlagBits = VkResolveModeFlagBitsKHR;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
#[rustfmt::skip]
pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR: VkResolveModeFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT: VkResolveModeFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
#[rustfmt::skip]
pub const VK_RESOLVE_MODE_AVERAGE_BIT_KHR: VkResolveModeFlagBitsKHR = 0x00000002;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_RESOLVE_MODE_AVERAGE_BIT: VkResolveModeFlagBitsKHR = 0x00000002;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
#[rustfmt::skip]
pub const VK_RESOLVE_MODE_MIN_BIT_KHR: VkResolveModeFlagBitsKHR = 0x00000004;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_RESOLVE_MODE_MIN_BIT: VkResolveModeFlagBitsKHR = 0x00000004;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
#[rustfmt::skip]
pub const VK_RESOLVE_MODE_MAX_BIT_KHR: VkResolveModeFlagBitsKHR = 0x00000008;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_RESOLVE_MODE_MAX_BIT: VkResolveModeFlagBitsKHR = 0x00000008;

#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub const VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME: &str = "VK_EXT_descriptor_indexing";
#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub const VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub const VK_ERROR_FRAGMENTATION_EXT: VkResult = VkResult(-1000161000);
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_ERROR_FRAGMENTATION: VkResult = VkResult(-1000161000);

#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub type VkDescriptorBindingFlagsEXT = VkFlags;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkDescriptorBindingFlags = VkDescriptorBindingFlagsEXT;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub type VkDescriptorBindingFlagBitsEXT = VkFlags;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkDescriptorBindingFlagBits = VkDescriptorBindingFlagBitsEXT;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT: VkDescriptorBindingFlagBitsEXT = 0x00000001;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT: VkDescriptorBindingFlagBitsEXT = 0x00000001;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT: VkDescriptorBindingFlagBitsEXT = 0x00000002;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT: VkDescriptorBindingFlagBitsEXT = 0x00000002;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT: VkDescriptorBindingFlagBitsEXT = 0x00000004;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT: VkDescriptorBindingFlagBitsEXT = 0x00000004;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT: VkDescriptorBindingFlagBitsEXT = 0x00000008;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT: VkDescriptorBindingFlagBitsEXT = 0x00000008;

#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT: VkDescriptorPoolCreateFlagBits = 0x00000004;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT: VkDescriptorPoolCreateFlagBits = 0x00000004;

#[cfg(feature = "VK_EXT_descriptor_indexing")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT: VkDescriptorSetLayoutCreateFlagBits = 0x00000002;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT: VkDescriptorSetLayoutCreateFlagBits = 0x00000002;

#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT: VkPipelineCacheCreateFlagBits = 0x00000001;

#[cfg(feature = "VK_KHR_maintenance4")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE4_EXTENSION_NAME: &str = "VK_KHR_maintenance4";
#[cfg(feature = "VK_KHR_maintenance4")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE4_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_KHR_FORMAT_FEATURE_FLAGS2_EXTENSION_NAME: &str = "VK_KHR_format_feature_flags2";
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_KHR_FORMAT_FEATURE_FLAGS2_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub type VkFormatFeatureFlags2KHR = VkFlags64;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkFormatFeatureFlags2 = VkFormatFeatureFlags2KHR;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub type VkFormatFeatureFlagBits2KHR = VkFlags64;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkFormatFeatureFlagBits2 = VkFormatFeatureFlagBits2KHR;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000001;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000002;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000004;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000008;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000010;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000020;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000040;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000080;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000100;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000200;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000400;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000000800;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000001000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000004000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000008000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000010000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000020000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000040000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000080000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000100000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000200000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000400000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000800000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000800000000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000100000000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000200000000;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT: VkFormatFeatureFlagBits2KHR = 0x0000000000002000;

#[cfg(feature = "VK_KHR_copy_commands2")]
#[rustfmt::skip]
pub const VK_KHR_COPY_COMMANDS2_EXTENSION_NAME: &str = "VK_KHR_copy_commands2";
#[cfg(feature = "VK_KHR_copy_commands2")]
#[rustfmt::skip]
pub const VK_KHR_COPY_COMMANDS2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_dynamic_rendering")]
#[rustfmt::skip]
pub const VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME: &str = "VK_KHR_dynamic_rendering";
#[cfg(feature = "VK_KHR_dynamic_rendering")]
#[rustfmt::skip]
pub const VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_dynamic_rendering")]
#[rustfmt::skip]
pub type VkRenderingFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkRenderingFlags = VkRenderingFlagsKHR;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
#[rustfmt::skip]
pub type VkRenderingFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkRenderingFlagBits = VkRenderingFlagBitsKHR;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
#[rustfmt::skip]
pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR: VkRenderingFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT: VkRenderingFlagBitsKHR = 0x00000001;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
#[rustfmt::skip]
pub const VK_RENDERING_SUSPENDING_BIT_KHR: VkRenderingFlagBitsKHR = 0x00000002;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_RENDERING_SUSPENDING_BIT: VkRenderingFlagBitsKHR = 0x00000002;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
#[rustfmt::skip]
pub const VK_RENDERING_RESUMING_BIT_KHR: VkRenderingFlagBitsKHR = 0x00000004;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_RENDERING_RESUMING_BIT: VkRenderingFlagBitsKHR = 0x00000004;

#[rustfmt::skip]
pub const VK_ATTACHMENT_STORE_OP_NONE: VkAttachmentStoreOp = 1000301000;

#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE5_EXTENSION_NAME: &str = "VK_KHR_maintenance5";
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE5_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub type VkBufferUsageFlags2KHR = VkFlags64;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkBufferUsageFlags2 = VkBufferUsageFlags2KHR;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub type VkBufferUsageFlagBits2KHR = VkFlags64;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkBufferUsageFlagBits2 = VkBufferUsageFlagBits2KHR;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT_KHR: VkBufferUsageFlagBits2KHR = 0x0000000000000001;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT: VkBufferUsageFlagBits2KHR = 0x0000000000000001;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_TRANSFER_DST_BIT_KHR: VkBufferUsageFlagBits2KHR = 0x0000000000000002;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_TRANSFER_DST_BIT: VkBufferUsageFlagBits2KHR = 0x0000000000000002;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR: VkBufferUsageFlagBits2KHR = 0x0000000000000004;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits2KHR = 0x0000000000000004;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT_KHR: VkBufferUsageFlagBits2KHR = 0x0000000000000008;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits2KHR = 0x0000000000000008;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT_KHR: VkBufferUsageFlagBits2KHR = 0x0000000000000010;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT: VkBufferUsageFlagBits2KHR = 0x0000000000000010;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT_KHR: VkBufferUsageFlagBits2KHR = 0x0000000000000020;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT: VkBufferUsageFlagBits2KHR = 0x0000000000000020;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT_KHR: VkBufferUsageFlagBits2KHR = 0x0000000000000040;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT: VkBufferUsageFlagBits2KHR = 0x0000000000000040;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT_KHR: VkBufferUsageFlagBits2KHR = 0x0000000000000080;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT: VkBufferUsageFlagBits2KHR = 0x0000000000000080;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT_KHR: VkBufferUsageFlagBits2KHR = 0x0000000000000100;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT: VkBufferUsageFlagBits2KHR = 0x0000000000000100;

#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub type VkPipelineCreateFlags2KHR = VkFlags64;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkPipelineCreateFlags2 = VkPipelineCreateFlags2KHR;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub type VkPipelineCreateFlagBits2KHR = VkFlags64;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkPipelineCreateFlagBits2 = VkPipelineCreateFlagBits2KHR;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT_KHR: VkPipelineCreateFlagBits2KHR = 0x0000000000000001;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlagBits2KHR = 0x0000000000000001;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT_KHR: VkPipelineCreateFlagBits2KHR = 0x0000000000000002;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlagBits2KHR = 0x0000000000000002;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_2_DERIVATIVE_BIT_KHR: VkPipelineCreateFlagBits2KHR = 0x0000000000000004;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_2_DERIVATIVE_BIT: VkPipelineCreateFlagBits2KHR = 0x0000000000000004;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: VkPipelineCreateFlagBits2KHR = 0x0000000000000008;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT: VkPipelineCreateFlagBits2KHR = 0x0000000000000008;
#[cfg(feature = "VK_KHR_maintenance5")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT_KHR: VkPipelineCreateFlagBits2KHR = 0x0000000000000010;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT: VkPipelineCreateFlagBits2KHR = 0x0000000000000010;

#[cfg(feature = "VK_KHR_maintenance6")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE6_EXTENSION_NAME: &str = "VK_KHR_maintenance6";
#[cfg(feature = "VK_KHR_maintenance6")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE6_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &str = "VK_KHR_vertex_attribute_divisor";
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME: &str = "VK_KHR_global_priority";
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_ERROR_NOT_PERMITTED_KHR_KHR: VkResult = VkResult(-1000174001);
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_ERROR_NOT_PERMITTED_KHR: VkResult = VkResult(-1000174001);

#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub type VkQueueGlobalPriorityKHR = i32;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkQueueGlobalPriority = i32;
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR: VkQueueGlobalPriorityKHR = 128;
#[cfg(feature = "VK_KHR_global_priority")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_LOW: VkQueueGlobalPriorityKHR = 128;
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR: VkQueueGlobalPriorityKHR = 256;
#[cfg(feature = "VK_KHR_global_priority")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM: VkQueueGlobalPriorityKHR = 256;
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR: VkQueueGlobalPriorityKHR = 512;
#[cfg(feature = "VK_KHR_global_priority")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH: VkQueueGlobalPriorityKHR = 512;
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR: VkQueueGlobalPriorityKHR = 1024;
#[cfg(feature = "VK_KHR_global_priority")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME: VkQueueGlobalPriorityKHR = 1024;

#[cfg(feature = "VK_KHR_load_store_op_none")]
#[rustfmt::skip]
pub const VK_KHR_LOAD_STORE_OP_NONE_EXTENSION_NAME: &str = "VK_KHR_load_store_op_none";
#[cfg(feature = "VK_KHR_load_store_op_none")]
#[rustfmt::skip]
pub const VK_KHR_LOAD_STORE_OP_NONE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_load_store_op_none")]
#[rustfmt::skip]
pub const VK_ATTACHMENT_LOAD_OP_NONE_KHR: VkAttachmentLoadOp = 1000400000;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_ATTACHMENT_LOAD_OP_NONE: VkAttachmentLoadOp = 1000400000;

#[cfg(not(feature = "VK_KHR_dynamic_rendering"))]
#[cfg(feature = "VK_KHR_load_store_op_none")]
#[rustfmt::skip]
pub const VK_ATTACHMENT_STORE_OP_NONE_KHR: VkAttachmentStoreOp = 1000301000;
#[cfg(not(feature = "VK_KHR_dynamic_rendering"))]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_ATTACHMENT_STORE_OP_NONE: VkAttachmentStoreOp = 1000301000;

#[cfg(feature = "VK_KHR_shader_expect_assume")]
#[rustfmt::skip]
pub const VK_KHR_SHADER_EXPECT_ASSUME_EXTENSION_NAME: &str = "VK_KHR_shader_expect_assume";
#[cfg(feature = "VK_KHR_shader_expect_assume")]
#[rustfmt::skip]
pub const VK_KHR_SHADER_EXPECT_ASSUME_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_shader_float_controls2")]
#[rustfmt::skip]
pub const VK_KHR_SHADER_FLOAT_CONTROLS2_EXTENSION_NAME: &str = "VK_KHR_shader_float_controls2";
#[cfg(feature = "VK_KHR_shader_float_controls2")]
#[rustfmt::skip]
pub const VK_KHR_SHADER_FLOAT_CONTROLS2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_push_descriptor")]
#[rustfmt::skip]
pub const VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &str = "VK_KHR_push_descriptor";
#[cfg(feature = "VK_KHR_push_descriptor")]
#[rustfmt::skip]
pub const VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_push_descriptor")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR: VkDescriptorSetLayoutCreateFlagBits = 0x00000001;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT: VkDescriptorSetLayoutCreateFlagBits = 0x00000001;

#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[cfg(feature = "VK_KHR_push_descriptor")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR: VkDescriptorUpdateTemplateTypeKHR = 1;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS: VkDescriptorUpdateTemplateTypeKHR = 1;

#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
#[rustfmt::skip]
pub const VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_EXTENSION_NAME: &str = "VK_KHR_dynamic_rendering_local_read";
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
#[rustfmt::skip]
pub const VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_RENDERING_LOCAL_READ_KHR: VkImageLayout = 1000232000;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_RENDERING_LOCAL_READ: VkImageLayout = 1000232000;

#[cfg(feature = "VK_KHR_index_type_uint8")]
#[rustfmt::skip]
pub const VK_KHR_INDEX_TYPE_UINT8_EXTENSION_NAME: &str = "VK_KHR_index_type_uint8";
#[cfg(feature = "VK_KHR_index_type_uint8")]
#[rustfmt::skip]
pub const VK_KHR_INDEX_TYPE_UINT8_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_index_type_uint8")]
#[rustfmt::skip]
pub const VK_INDEX_TYPE_UINT8_KHR: VkIndexType = 1000265000;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_INDEX_TYPE_UINT8: VkIndexType = 1000265000;

#[cfg(feature = "VK_KHR_line_rasterization")]
#[rustfmt::skip]
pub const VK_KHR_LINE_RASTERIZATION_EXTENSION_NAME: &str = "VK_KHR_line_rasterization";
#[cfg(feature = "VK_KHR_line_rasterization")]
#[rustfmt::skip]
pub const VK_KHR_LINE_RASTERIZATION_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_line_rasterization")]
#[rustfmt::skip]
pub type VkLineRasterizationModeKHR = i32;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkLineRasterizationMode = i32;
#[cfg(feature = "VK_KHR_line_rasterization")]
#[rustfmt::skip]
pub const VK_LINE_RASTERIZATION_MODE_DEFAULT_KHR: VkLineRasterizationModeKHR = 0;
#[cfg(feature = "VK_KHR_line_rasterization")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_LINE_RASTERIZATION_MODE_DEFAULT: VkLineRasterizationModeKHR = 0;
#[cfg(feature = "VK_KHR_line_rasterization")]
#[rustfmt::skip]
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_KHR: VkLineRasterizationModeKHR = 1;
#[cfg(feature = "VK_KHR_line_rasterization")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR: VkLineRasterizationModeKHR = 1;
#[cfg(feature = "VK_KHR_line_rasterization")]
#[rustfmt::skip]
pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM_KHR: VkLineRasterizationModeKHR = 2;
#[cfg(feature = "VK_KHR_line_rasterization")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM: VkLineRasterizationModeKHR = 2;
#[cfg(feature = "VK_KHR_line_rasterization")]
#[rustfmt::skip]
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_KHR: VkLineRasterizationModeKHR = 3;
#[cfg(feature = "VK_KHR_line_rasterization")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH: VkLineRasterizationModeKHR = 3;

#[cfg(feature = "VK_KHR_map_memory2")]
#[rustfmt::skip]
pub const VK_KHR_MAP_MEMORY2_EXTENSION_NAME: &str = "VK_KHR_map_memory2";
#[cfg(feature = "VK_KHR_map_memory2")]
#[rustfmt::skip]
pub const VK_KHR_MAP_MEMORY2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_map_memory2")]
#[rustfmt::skip]
pub type VkMemoryUnmapFlagsKHR = VkFlags;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkMemoryUnmapFlags = VkMemoryUnmapFlagsKHR;
#[cfg(feature = "VK_KHR_map_memory2")]
#[rustfmt::skip]
pub type VkMemoryUnmapFlagBitsKHR = VkFlags;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkMemoryUnmapFlagBits = VkMemoryUnmapFlagBitsKHR;
#[cfg(feature = "VK_KHR_map_memory2")]
#[rustfmt::skip]
pub const VK_MEMORY_UNMAP_RESERVE_BIT_KHR: VkMemoryUnmapFlagBitsKHR = 0x00000001;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_MEMORY_UNMAP_RESERVE_BIT: VkMemoryUnmapFlagBitsKHR = 0x00000001;

#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]
#[rustfmt::skip]
pub const VK_KHR_SHADER_SUBGROUP_ROTATE_EXTENSION_NAME: &str = "VK_KHR_shader_subgroup_rotate";
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]
#[rustfmt::skip]
pub const VK_KHR_SHADER_SUBGROUP_ROTATE_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_ROTATE_BIT_KHR: VkSubgroupFeatureFlagBits = 0x00000200;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_ROTATE_BIT: VkSubgroupFeatureFlagBits = 0x00000200;
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT_KHR: VkSubgroupFeatureFlagBits = 0x00000400;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_SUBGROUP_FEATURE_ROTATE_CLUSTERED_BIT: VkSubgroupFeatureFlagBits = 0x00000400;

#[cfg(feature = "VK_EXT_host_image_copy")]
#[rustfmt::skip]
pub const VK_EXT_HOST_IMAGE_COPY_EXTENSION_NAME: &str = "VK_EXT_host_image_copy";
#[cfg(feature = "VK_EXT_host_image_copy")]
#[rustfmt::skip]
pub const VK_EXT_HOST_IMAGE_COPY_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_host_image_copy")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT_EXT: VkFormatFeatureFlagBits2 = 0x0000000400000000;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER_BIT: VkFormatFeatureFlagBits2 = 0x0000000400000000;

#[cfg(feature = "VK_EXT_host_image_copy")]
#[rustfmt::skip]
pub const VK_IMAGE_USAGE_HOST_TRANSFER_BIT_EXT: VkImageUsageFlagBits = 0x00400000;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_USAGE_HOST_TRANSFER_BIT: VkImageUsageFlagBits = 0x00400000;

#[cfg(feature = "VK_EXT_host_image_copy")]
#[rustfmt::skip]
pub type VkHostImageCopyFlagsEXT = VkFlags;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkHostImageCopyFlags = VkHostImageCopyFlagsEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]
#[rustfmt::skip]
pub type VkHostImageCopyFlagBitsEXT = VkFlags;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkHostImageCopyFlagBits = VkHostImageCopyFlagBitsEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]
#[rustfmt::skip]
pub const VK_HOST_IMAGE_COPY_MEMCPY_BIT_EXT: VkHostImageCopyFlagBitsEXT = 0x00000001;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_HOST_IMAGE_COPY_MEMCPY_BIT: VkHostImageCopyFlagBitsEXT = 0x00000001;

#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[rustfmt::skip]
pub const VK_EXT_PIPELINE_PROTECTED_ACCESS_EXTENSION_NAME: &str = "VK_EXT_pipeline_protected_access";
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[rustfmt::skip]
pub const VK_EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT_EXT: VkPipelineCreateFlagBits = 0x08000000;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT: VkPipelineCreateFlagBits = 0x08000000;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT_EXT: VkPipelineCreateFlagBits = 0x40000000;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT: VkPipelineCreateFlagBits = 0x40000000;

#[cfg(feature = "VK_EXT_pipeline_robustness")]
#[rustfmt::skip]
pub const VK_EXT_PIPELINE_ROBUSTNESS_EXTENSION_NAME: &str = "VK_EXT_pipeline_robustness";
#[cfg(feature = "VK_EXT_pipeline_robustness")]
#[rustfmt::skip]
pub const VK_EXT_PIPELINE_ROBUSTNESS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[rustfmt::skip]
pub type VkPipelineRobustnessBufferBehaviorEXT = i32;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkPipelineRobustnessBufferBehavior = i32;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "VK_EXT_pipeline_robustness")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT_EXT: VkPipelineRobustnessBufferBehaviorEXT = 0;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT: VkPipelineRobustnessBufferBehaviorEXT = 0;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "VK_EXT_pipeline_robustness")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED_EXT: VkPipelineRobustnessBufferBehaviorEXT = 1;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED: VkPipelineRobustnessBufferBehaviorEXT = 1;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "VK_EXT_pipeline_robustness")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_EXT: VkPipelineRobustnessBufferBehaviorEXT = 2;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS: VkPipelineRobustnessBufferBehaviorEXT = 2;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "VK_EXT_pipeline_robustness")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2_EXT: VkPipelineRobustnessBufferBehaviorEXT = 3;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2: VkPipelineRobustnessBufferBehaviorEXT = 3;

#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[rustfmt::skip]
pub type VkPipelineRobustnessImageBehaviorEXT = i32;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkPipelineRobustnessImageBehavior = i32;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "VK_EXT_pipeline_robustness")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT_EXT: VkPipelineRobustnessImageBehaviorEXT = 0;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT: VkPipelineRobustnessImageBehaviorEXT = 0;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "VK_EXT_pipeline_robustness")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED_EXT: VkPipelineRobustnessImageBehaviorEXT = 1;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED: VkPipelineRobustnessImageBehaviorEXT = 1;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "VK_EXT_pipeline_robustness")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_EXT: VkPipelineRobustnessImageBehaviorEXT = 2;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS: VkPipelineRobustnessImageBehaviorEXT = 2;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "VK_EXT_pipeline_robustness")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2_EXT: VkPipelineRobustnessImageBehaviorEXT = 3;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2: VkPipelineRobustnessImageBehaviorEXT = 3;

#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &str = "VK_EXT_blend_operation_advanced";
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_ZERO_EXT: VkBlendOp = 1000148000;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_SRC_EXT: VkBlendOp = 1000148001;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_DST_EXT: VkBlendOp = 1000148002;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_SRC_OVER_EXT: VkBlendOp = 1000148003;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_DST_OVER_EXT: VkBlendOp = 1000148004;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_SRC_IN_EXT: VkBlendOp = 1000148005;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_DST_IN_EXT: VkBlendOp = 1000148006;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_SRC_OUT_EXT: VkBlendOp = 1000148007;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_DST_OUT_EXT: VkBlendOp = 1000148008;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_SRC_ATOP_EXT: VkBlendOp = 1000148009;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_DST_ATOP_EXT: VkBlendOp = 1000148010;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_XOR_EXT: VkBlendOp = 1000148011;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_MULTIPLY_EXT: VkBlendOp = 1000148012;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_SCREEN_EXT: VkBlendOp = 1000148013;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_OVERLAY_EXT: VkBlendOp = 1000148014;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_DARKEN_EXT: VkBlendOp = 1000148015;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_LIGHTEN_EXT: VkBlendOp = 1000148016;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_COLORDODGE_EXT: VkBlendOp = 1000148017;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_COLORBURN_EXT: VkBlendOp = 1000148018;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_HARDLIGHT_EXT: VkBlendOp = 1000148019;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_SOFTLIGHT_EXT: VkBlendOp = 1000148020;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_DIFFERENCE_EXT: VkBlendOp = 1000148021;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_EXCLUSION_EXT: VkBlendOp = 1000148022;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_INVERT_EXT: VkBlendOp = 1000148023;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_INVERT_RGB_EXT: VkBlendOp = 1000148024;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_LINEARDODGE_EXT: VkBlendOp = 1000148025;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_LINEARBURN_EXT: VkBlendOp = 1000148026;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_VIVIDLIGHT_EXT: VkBlendOp = 1000148027;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_LINEARLIGHT_EXT: VkBlendOp = 1000148028;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_PINLIGHT_EXT: VkBlendOp = 1000148029;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_HARDMIX_EXT: VkBlendOp = 1000148030;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_HSL_HUE_EXT: VkBlendOp = 1000148031;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_HSL_SATURATION_EXT: VkBlendOp = 1000148032;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_HSL_COLOR_EXT: VkBlendOp = 1000148033;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_HSL_LUMINOSITY_EXT: VkBlendOp = 1000148034;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_PLUS_EXT: VkBlendOp = 1000148035;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_PLUS_CLAMPED_EXT: VkBlendOp = 1000148036;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_PLUS_CLAMPED_ALHPA_EXT: VkBlendOp = 1000148037;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_PLUS_DARKER_EXT: VkBlendOp = 1000148038;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_MINUS_EXT: VkBlendOp = 1000148039;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_MINUS_CLAMPED_EXT: VkBlendOp = 1000148040;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_CONTRAST_EXT: VkBlendOp = 1000148041;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_INVERT_OVG_EXT: VkBlendOp = 1000148042;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_RED_EXT: VkBlendOp = 1000148043;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_GREEN_EXT: VkBlendOp = 1000148044;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OP_BLUE_EXT: VkBlendOp = 1000148045;

#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub type VkBlendOverlapEXT = i32;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OVERLAP_UNCORRELATED_EXT: VkBlendOverlapEXT = 0;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OVERLAP_DISJOINT_EXT: VkBlendOverlapEXT = 1;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_BLEND_OVERLAP_CONJOINT_EXT: VkBlendOverlapEXT = 2;

#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub const VK_EXT_VALIDATION_CACHE_EXTENSION_NAME: &str = "VK_EXT_validation_cache";
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub const VK_EXT_VALIDATION_CACHE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub type VkValidationCacheCreateFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub type VkValidationCacheCreateFlagBitsEXT = VkFlags;

#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkValidationCacheEXT(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkValidationCacheEXT {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_VALIDATION_CACHE_EXT;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkObjectType = 1000160000;

#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub type VkValidationCacheHeaderVersionEXT = i32;
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: VkValidationCacheHeaderVersionEXT = 1;

#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub const VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME: &str = "VK_EXT_validation_flags";
#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub const VK_EXT_VALIDATION_FLAGS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub type VkValidationCheckEXT = i32;
#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub const VK_VALIDATION_CHECK_ALL_EXT: VkValidationCheckEXT = 0;
#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub const VK_VALIDATION_CHECK_SHADERS_EXT: VkValidationCheckEXT = 1;

#[cfg(feature = "VK_EXT_acquire_drm_display")]
#[rustfmt::skip]
pub const VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME: &str = "VK_EXT_acquire_drm_display";
#[cfg(feature = "VK_EXT_acquire_drm_display")]
#[rustfmt::skip]
pub const VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[rustfmt::skip]
pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &str = "VK_EXT_acquire_xlib_display";
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[rustfmt::skip]
pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_acquire_winrt_display")]
#[rustfmt::skip]
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &str = "VK_NV_acquire_winrt_display";
#[cfg(feature = "VK_NV_acquire_winrt_display")]
#[rustfmt::skip]
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_buffer_marker")]
#[rustfmt::skip]
pub const VK_AMD_BUFFER_MARKER_EXTENSION_NAME: &str = "VK_AMD_buffer_marker";
#[cfg(feature = "VK_AMD_buffer_marker")]
#[rustfmt::skip]
pub const VK_AMD_BUFFER_MARKER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_EXT_LAYER_SETTINGS_EXTENSION_NAME: &str = "VK_EXT_layer_settings";
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_EXT_LAYER_SETTINGS_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub type VkLayerSettingTypeEXT = i32;
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_LAYER_SETTING_TYPE_BOOL32_EXT: VkLayerSettingTypeEXT = 0;
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_LAYER_SETTING_TYPE_INT32_EXT: VkLayerSettingTypeEXT = 1;
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_LAYER_SETTING_TYPE_INT64_EXT: VkLayerSettingTypeEXT = 2;
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_LAYER_SETTING_TYPE_UINT32_EXT: VkLayerSettingTypeEXT = 3;
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_LAYER_SETTING_TYPE_UINT64_EXT: VkLayerSettingTypeEXT = 4;
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_LAYER_SETTING_TYPE_FLOAT32_EXT: VkLayerSettingTypeEXT = 5;
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_LAYER_SETTING_TYPE_FLOAT64_EXT: VkLayerSettingTypeEXT = 6;
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_LAYER_SETTING_TYPE_STRING_EXT: VkLayerSettingTypeEXT = 7;

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_EXT_DESCRIPTOR_BUFFER_EXTENSION_NAME: &str = "VK_EXT_descriptor_buffer";
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_EXT_DESCRIPTOR_BUFFER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Clone, Copy)]
#[repr(C)]
#[rustfmt::skip]
pub union VkDescriptorDataEXT {
    pub pSampler: *const VkSampler,
    pub pCombinedImageSampler: *const VkDescriptorImageInfo,
    pub pInputAttachmentImage: *const VkDescriptorImageInfo,
    pub pSampledImage: *const VkDescriptorImageInfo,
    pub pStorageImage: *const VkDescriptorImageInfo,
    pub pUniformTexelBuffer: *const VkDescriptorAddressInfoEXT,
    pub pStorageTexelBuffer: *const VkDescriptorAddressInfoEXT,
    pub pUniformBuffer: *const VkDescriptorAddressInfoEXT,
    pub pStorageBuffer: *const VkDescriptorAddressInfoEXT,
    pub accelerationStructure: VkDeviceAddress,
}

#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
pub const VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: &str = "VK_EXT_external_memory_host";
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
pub const VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_external_memory_host")]
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000080;
#[cfg(feature = "VK_EXT_external_memory_host")]
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000100;

#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &str = "VK_EXT_vertex_attribute_divisor";
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &str = "VK_EXT_sample_locations";
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT: VkImageCreateFlagBits = 0x00001000;

#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
#[rustfmt::skip]
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &str = "VK_NV_fragment_coverage_to_color";
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
#[rustfmt::skip]
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
#[rustfmt::skip]
pub type VkPipelineCoverageToColorStateCreateFlagsNV = VkFlags;
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
#[rustfmt::skip]
pub type VkPipelineCoverageToColorStateCreateFlagBitsNV = VkFlags;

#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &str = "VK_NV_framebuffer_mixed_samples";
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
pub type VkCoverageModulationModeNV = i32;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
pub const VK_COVERAGE_MODULATION_MODE_NONE_NV: VkCoverageModulationModeNV = 0;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
pub const VK_COVERAGE_MODULATION_MODE_RGB_NV: VkCoverageModulationModeNV = 1;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
pub const VK_COVERAGE_MODULATION_MODE_ALPHA_NV: VkCoverageModulationModeNV = 2;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
pub const VK_COVERAGE_MODULATION_MODE_RGBA_NV: VkCoverageModulationModeNV = 3;

#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
pub type VkPipelineCoverageModulationStateCreateFlagsNV = VkFlags;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
pub type VkPipelineCoverageModulationStateCreateFlagBitsNV = VkFlags;

#[cfg(feature = "VK_EXT_global_priority")]
#[rustfmt::skip]
pub const VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME: &str = "VK_EXT_global_priority";
#[cfg(feature = "VK_EXT_global_priority")]
#[rustfmt::skip]
pub const VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_EXT_global_priority")]
#[rustfmt::skip]
pub type VkQueueGlobalPriorityEXT = i32;
#[cfg(feature = "VK_EXT_global_priority")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT: VkQueueGlobalPriorityEXT = 128;
#[cfg(feature = "VK_EXT_global_priority")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT: VkQueueGlobalPriorityEXT = 256;
#[cfg(feature = "VK_EXT_global_priority")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT: VkQueueGlobalPriorityEXT = 512;
#[cfg(feature = "VK_EXT_global_priority")]
#[rustfmt::skip]
pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT: VkQueueGlobalPriorityEXT = 1024;

#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &str = "VK_NV_viewport_swizzle";
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub type VkViewportCoordinateSwizzleNV = i32;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: VkViewportCoordinateSwizzleNV = 0;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: VkViewportCoordinateSwizzleNV = 1;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: VkViewportCoordinateSwizzleNV = 2;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: VkViewportCoordinateSwizzleNV = 3;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: VkViewportCoordinateSwizzleNV = 4;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: VkViewportCoordinateSwizzleNV = 5;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: VkViewportCoordinateSwizzleNV = 6;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: VkViewportCoordinateSwizzleNV = 7;

#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub type VkPipelineViewportSwizzleStateCreateFlagsNV = VkFlags;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub type VkPipelineViewportSwizzleStateCreateFlagBitsNV = VkFlags;

#[cfg(feature = "VK_EXT_hdr_metadata")]
#[rustfmt::skip]
pub const VK_EXT_HDR_METADATA_EXTENSION_NAME: &str = "VK_EXT_hdr_metadata";
#[cfg(feature = "VK_EXT_hdr_metadata")]
#[rustfmt::skip]
pub const VK_EXT_HDR_METADATA_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_display_control")]
#[rustfmt::skip]
pub const VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME: &str = "VK_EXT_display_control";
#[cfg(feature = "VK_EXT_display_control")]
#[rustfmt::skip]
pub const VK_EXT_DISPLAY_CONTROL_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_display_control")]
#[rustfmt::skip]
pub type VkDisplayPowerStateEXT = i32;
#[cfg(feature = "VK_EXT_display_control")]
#[rustfmt::skip]
pub const VK_DISPLAY_POWER_STATE_OFF_EXT: VkDisplayPowerStateEXT = 0;
#[cfg(feature = "VK_EXT_display_control")]
#[rustfmt::skip]
pub const VK_DISPLAY_POWER_STATE_SUSPEND_EXT: VkDisplayPowerStateEXT = 1;
#[cfg(feature = "VK_EXT_display_control")]
#[rustfmt::skip]
pub const VK_DISPLAY_POWER_STATE_ON_EXT: VkDisplayPowerStateEXT = 2;

#[cfg(feature = "VK_EXT_display_control")]
#[rustfmt::skip]
pub type VkDeviceEventTypeEXT = i32;
#[cfg(feature = "VK_EXT_display_control")]
#[rustfmt::skip]
pub const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: VkDeviceEventTypeEXT = 0;

#[cfg(feature = "VK_EXT_display_control")]
#[rustfmt::skip]
pub type VkDisplayEventTypeEXT = i32;
#[cfg(feature = "VK_EXT_display_control")]
#[rustfmt::skip]
pub const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: VkDisplayEventTypeEXT = 0;

#[cfg(feature = "VK_GOOGLE_display_timing")]
#[rustfmt::skip]
pub const VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &str = "VK_GOOGLE_display_timing";
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[rustfmt::skip]
pub const VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_shared_presentable_image")]
#[rustfmt::skip]
pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &str = "VK_KHR_shared_presentable_image";
#[cfg(feature = "VK_KHR_shared_presentable_image")]
#[rustfmt::skip]
pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_shared_presentable_image")]
#[rustfmt::skip]
pub const VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR: VkImageLayout = 1000111000;

#[cfg(feature = "VK_AMD_rasterization_order")]
#[rustfmt::skip]
pub const VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME: &str = "VK_AMD_rasterization_order";
#[cfg(feature = "VK_AMD_rasterization_order")]
#[rustfmt::skip]
pub const VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_rasterization_order")]
#[rustfmt::skip]
pub type VkRasterizationOrderAMD = i32;
#[cfg(feature = "VK_AMD_rasterization_order")]
#[rustfmt::skip]
pub const VK_RASTERIZATION_ORDER_STRICT_AMD: VkRasterizationOrderAMD = 0;
#[cfg(feature = "VK_AMD_rasterization_order")]
#[rustfmt::skip]
pub const VK_RASTERIZATION_ORDER_RELAXED_AMD: VkRasterizationOrderAMD = 1;

#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
#[rustfmt::skip]
pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &str = "VK_AMD_texture_gather_bias_lod";
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
#[rustfmt::skip]
pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NN_vi_surface")]
#[rustfmt::skip]
pub const VK_NN_VI_SURFACE_EXTENSION_NAME: &str = "VK_NN_vi_surface";
#[cfg(feature = "VK_NN_vi_surface")]
#[rustfmt::skip]
pub const VK_NN_VI_SURFACE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NN_vi_surface")]
#[rustfmt::skip]
pub type VkViSurfaceCreateFlagsNN = VkFlags;
#[cfg(feature = "VK_NN_vi_surface")]
#[rustfmt::skip]
pub type VkViSurfaceCreateFlagBitsNN = VkFlags;

#[cfg(feature = "VK_EXT_display_surface_counter")]
#[rustfmt::skip]
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &str = "VK_EXT_display_surface_counter";
#[cfg(feature = "VK_EXT_display_surface_counter")]
#[rustfmt::skip]
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_display_surface_counter")]
#[rustfmt::skip]
pub type VkSurfaceCounterFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_display_surface_counter")]
#[rustfmt::skip]
pub type VkSurfaceCounterFlagBitsEXT = VkFlags;
#[cfg(feature = "VK_EXT_display_surface_counter")]
#[rustfmt::skip]
pub const VK_SURFACE_COUNTER_VBLANK_BIT_EXT: VkSurfaceCounterFlagBitsEXT = 0x00000001;

#[cfg(feature = "VK_EXT_debug_marker")]
#[rustfmt::skip]
pub const VK_EXT_DEBUG_MARKER_EXTENSION_NAME: &str = "VK_EXT_debug_marker";
#[cfg(feature = "VK_EXT_debug_marker")]
#[rustfmt::skip]
pub const VK_EXT_DEBUG_MARKER_SPEC_VERSION: usize = 4;

#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_NVX_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &str = "VK_NVX_device_generated_commands";
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_NVX_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkObjectTableNVX(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkObjectTableNVX {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_OBJECT_TABLE_NVX;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_OBJECT_TABLE_NVX: VkObjectType = 1000086000;

#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VkIndirectCommandsLayoutNVX(pub core::num::NonZeroU64, pub core::marker::PhantomData<*mut u8>);
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
impl crate::VkRawHandle for VkIndirectCommandsLayoutNVX {
    const OBJECT_TYPE: VkObjectType = VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        self.0.get()
    }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX: VkObjectType = 1000086001;

#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub type VkIndirectCommandsTokenTypeNVX = i32;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PIPELINE_NVX: VkIndirectCommandsTokenTypeNVX = 0;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DESCRIPTOR_SET_NVX: VkIndirectCommandsTokenTypeNVX = 1;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NVX: VkIndirectCommandsTokenTypeNVX = 2;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NVX: VkIndirectCommandsTokenTypeNVX = 3;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NVX: VkIndirectCommandsTokenTypeNVX = 4;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NVX: VkIndirectCommandsTokenTypeNVX = 5;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NVX: VkIndirectCommandsTokenTypeNVX = 6;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_NVX: VkIndirectCommandsTokenTypeNVX = 7;

#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub type VkObjectEntryTypeNVX = i32;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_OBJECT_ENTRY_TYPE_DESCRIPTOR_SET_NVX: VkObjectEntryTypeNVX = 0;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_OBJECT_ENTRY_TYPE_PIPELINE_NVX: VkObjectEntryTypeNVX = 1;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_OBJECT_ENTRY_TYPE_INDEX_BUFFER_NVX: VkObjectEntryTypeNVX = 2;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_OBJECT_ENTRY_TYPE_VERTEX_BUFFER_NVX: VkObjectEntryTypeNVX = 3;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_OBJECT_ENTRY_TYPE_PUSH_CONSTANT_NVX: VkObjectEntryTypeNVX = 4;

#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub type VkIndirectCommandsLayoutUsageFlagsNVX = VkFlags;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub type VkIndirectCommandsLayoutUsageFlagBitsNVX = VkFlags;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NVX: VkIndirectCommandsLayoutUsageFlagBitsNVX = 0x00000001;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_SPARSE_SEQUENCES_BIT_NVX: VkIndirectCommandsLayoutUsageFlagBitsNVX = 0x00000002;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EMPTY_EXECUTIONS_BIT_NVX: VkIndirectCommandsLayoutUsageFlagBitsNVX = 0x00000004;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NVX: VkIndirectCommandsLayoutUsageFlagBitsNVX = 0x00000008;

#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub type VkObjectEntryUsageFlagsNVX = VkFlags;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub type VkObjectEntryUsageFlagBitsNVX = VkFlags;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_OBJECT_ENTRY_USAGE_GRAPHICS_BIT_NVX: VkObjectEntryUsageFlagBitsNVX = 0x00000001;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_OBJECT_ENTRY_USAGE_COMPUTE_BIT_NVX: VkObjectEntryUsageFlagBitsNVX = 0x00000002;

#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_PIPELINE_STAGE_COMMAND_PROCESS_BIT_NVX: VkPipelineStageFlagBits = 0x00020000;

#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_ACCESS_COMMAND_PROCESS_READ_BIT_NVX: VkAccessFlagBits = 0x00020000;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[rustfmt::skip]
pub const VK_ACCESS_COMMAND_PROCESS_WRITE_BIT_NVX: VkAccessFlagBits = 0x00040000;

#[cfg(feature = "VK_KHR_incremental_present")]
#[rustfmt::skip]
pub const VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &str = "VK_KHR_incremental_present";
#[cfg(feature = "VK_KHR_incremental_present")]
#[rustfmt::skip]
pub const VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_clip_space_w_scaling")]
#[rustfmt::skip]
pub const VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &str = "VK_NV_clip_space_w_scaling";
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
#[rustfmt::skip]
pub const VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_clip_space_w_scaling")]
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV: VkDynamicState = 1000087000;

#[cfg(feature = "VK_NV_fill_rectangle")]
#[rustfmt::skip]
pub const VK_NV_FILL_RECTANGLE_EXTENSION_NAME: &str = "VK_NV_fill_rectangle";
#[cfg(feature = "VK_NV_fill_rectangle")]
#[rustfmt::skip]
pub const VK_NV_FILL_RECTANGLE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_fill_rectangle")]
#[rustfmt::skip]
pub const VK_POLYGON_MODE_FILL_RECTANGLE_NV: VkPolygonMode = 1000153000;

#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[rustfmt::skip]
pub const VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME: &str = "VK_EXT_conservative_rasterization";
#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[rustfmt::skip]
pub const VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[rustfmt::skip]
pub type VkConservativeRasterizationModeEXT = i32;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[rustfmt::skip]
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT: VkConservativeRasterizationModeEXT = 0;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[rustfmt::skip]
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT: VkConservativeRasterizationModeEXT = 1;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[rustfmt::skip]
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT: VkConservativeRasterizationModeEXT = 2;

#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[rustfmt::skip]
pub type VkPipelineRasterizationConservativeStateCreateFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[rustfmt::skip]
pub type VkPipelineRasterizationConservativeStateCreateFlagBitsEXT = VkFlags;

#[cfg(feature = "VK_EXT_discard_rectangles")]
#[rustfmt::skip]
pub const VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &str = "VK_EXT_discard_rectangles";
#[cfg(feature = "VK_EXT_discard_rectangles")]
#[rustfmt::skip]
pub const VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_discard_rectangles")]
#[rustfmt::skip]
pub const VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT: VkDynamicState = 1000099000;

#[cfg(feature = "VK_EXT_discard_rectangles")]
#[rustfmt::skip]
pub type VkDiscardRectangleModeEXT = i32;
#[cfg(feature = "VK_EXT_discard_rectangles")]
#[rustfmt::skip]
pub const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: VkDiscardRectangleModeEXT = 0;
#[cfg(feature = "VK_EXT_discard_rectangles")]
#[rustfmt::skip]
pub const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: VkDiscardRectangleModeEXT = 1;

#[cfg(feature = "VK_EXT_discard_rectangles")]
#[rustfmt::skip]
pub type VkPipelineDiscardRectangleStateCreateFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_discard_rectangles")]
#[rustfmt::skip]
pub type VkPipelineDiscardRectangleStateCreateFlagBitsEXT = VkFlags;

#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
#[rustfmt::skip]
pub const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: &str = "VK_EXT_image_drm_format_modifier";
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
#[rustfmt::skip]
pub const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
#[rustfmt::skip]
pub const VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT: VkImageTiling = 1000158000;

#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT: VkImageAspectFlagBits = 0x00000080;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT: VkImageAspectFlagBits = 0x00000100;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT: VkImageAspectFlagBits = 0x00000200;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
#[rustfmt::skip]
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT: VkImageAspectFlagBits = 0x00000400;

#[cfg(feature = "VK_EXT_metal_objects")]
#[rustfmt::skip]
pub const VK_EXT_METAL_OBJECTS_EXTENSION_NAME: &str = "VK_EXT_metal_objects";
#[cfg(feature = "VK_EXT_metal_objects")]
#[rustfmt::skip]
pub const VK_EXT_METAL_OBJECTS_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_EXT_metal_objects")]
#[rustfmt::skip]
pub type VkExportMetalObjectTypeFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_metal_objects")]
#[rustfmt::skip]
pub type VkExportMetalObjectTypeFlagBitsEXT = VkFlags;
#[cfg(feature = "VK_EXT_metal_objects")]
#[rustfmt::skip]
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT: VkExportMetalObjectTypeFlagBitsEXT = 0x00000001;
#[cfg(feature = "VK_EXT_metal_objects")]
#[rustfmt::skip]
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT: VkExportMetalObjectTypeFlagBitsEXT = 0x00000002;
#[cfg(feature = "VK_EXT_metal_objects")]
#[rustfmt::skip]
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT: VkExportMetalObjectTypeFlagBitsEXT = 0x00000004;
#[cfg(feature = "VK_EXT_metal_objects")]
#[rustfmt::skip]
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT: VkExportMetalObjectTypeFlagBitsEXT = 0x00000008;
#[cfg(feature = "VK_EXT_metal_objects")]
#[rustfmt::skip]
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT: VkExportMetalObjectTypeFlagBitsEXT = 0x00000010;
#[cfg(feature = "VK_EXT_metal_objects")]
#[rustfmt::skip]
pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT: VkExportMetalObjectTypeFlagBitsEXT = 0x00000020;

#[cfg(feature = "VK_MVK_macos_surface")]
#[rustfmt::skip]
pub const VK_MVK_MACOS_SURFACE_EXTENSION_NAME: &str = "VK_MVK_macos_surface";
#[cfg(feature = "VK_MVK_macos_surface")]
#[rustfmt::skip]
pub const VK_MVK_MACOS_SURFACE_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_MVK_macos_surface")]
#[rustfmt::skip]
pub type VkMacOSSurfaceCreateFlagsMVK = VkFlags;
#[cfg(feature = "VK_MVK_macos_surface")]
#[rustfmt::skip]
pub type VkMacOSSurfaceCreateFlagBitsMVK = VkFlags;

#[cfg(feature = "VK_MVK_ios_surface")]
#[rustfmt::skip]
pub const VK_MVK_IOS_SURFACE_EXTENSION_NAME: &str = "VK_MVK_ios_surface";
#[cfg(feature = "VK_MVK_ios_surface")]
#[rustfmt::skip]
pub const VK_MVK_IOS_SURFACE_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_MVK_ios_surface")]
#[rustfmt::skip]
pub type VkIOSSurfaceCreateFlagsMVK = VkFlags;
#[cfg(feature = "VK_MVK_ios_surface")]
#[rustfmt::skip]
pub type VkIOSSurfaceCreateFlagBitsMVK = VkFlags;

#[cfg(feature = "VK_EXT_depth_range_unrestricted")]
#[rustfmt::skip]
pub const VK_EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME: &str = "VK_EXT_depth_range_unrestricted";
#[cfg(feature = "VK_EXT_depth_range_unrestricted")]
#[rustfmt::skip]
pub const VK_EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_direct_mode_display")]
#[rustfmt::skip]
pub const VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &str = "VK_EXT_direct_mode_display";
#[cfg(feature = "VK_EXT_direct_mode_display")]
#[rustfmt::skip]
pub const VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_display_swapchain")]
#[rustfmt::skip]
pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_display_swapchain";
#[cfg(feature = "VK_KHR_display_swapchain")]
#[rustfmt::skip]
pub const VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: usize = 10;

#[cfg(feature = "VK_AMD_draw_indirect_count")]
#[rustfmt::skip]
pub const VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &str = "VK_AMD_draw_indirect_count";
#[cfg(feature = "VK_AMD_draw_indirect_count")]
#[rustfmt::skip]
pub const VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_external_memory_dma_buf")]
#[rustfmt::skip]
pub const VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME: &str = "VK_EXT_external_memory_dma_buf";
#[cfg(feature = "VK_EXT_external_memory_dma_buf")]
#[rustfmt::skip]
pub const VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_external_memory_dma_buf")]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000200;

#[cfg(feature = "VK_IMG_filter_cubic")]
#[rustfmt::skip]
pub const VK_IMG_FILTER_CUBIC_EXTENSION_NAME: &str = "VK_IMG_filter_cubic";
#[cfg(feature = "VK_IMG_filter_cubic")]
#[rustfmt::skip]
pub const VK_IMG_FILTER_CUBIC_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_IMG_filter_cubic")]
#[rustfmt::skip]
pub const VK_FILTER_CUBIC_IMG: VkFilter = 1000015000;

#[cfg(feature = "VK_IMG_filter_cubic")]
#[rustfmt::skip]
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: VkFormatFeatureFlagBits = 0x00002000;

#[cfg(feature = "VK_IMG_format_pvrtc")]
#[rustfmt::skip]
pub const VK_IMG_FORMAT_PVRTC_EXTENSION_NAME: &str = "VK_IMG_format_pvrtc";
#[cfg(feature = "VK_IMG_format_pvrtc")]
#[rustfmt::skip]
pub const VK_IMG_FORMAT_PVRTC_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_IMG_format_pvrtc")]
#[rustfmt::skip]
pub const VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG: VkFormat = 1000054000;
#[cfg(feature = "VK_IMG_format_pvrtc")]
#[rustfmt::skip]
pub const VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG: VkFormat = 1000054001;
#[cfg(feature = "VK_IMG_format_pvrtc")]
#[rustfmt::skip]
pub const VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG: VkFormat = 1000054002;
#[cfg(feature = "VK_IMG_format_pvrtc")]
#[rustfmt::skip]
pub const VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG: VkFormat = 1000054003;
#[cfg(feature = "VK_IMG_format_pvrtc")]
#[rustfmt::skip]
pub const VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG: VkFormat = 1000054004;
#[cfg(feature = "VK_IMG_format_pvrtc")]
#[rustfmt::skip]
pub const VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG: VkFormat = 1000054005;
#[cfg(feature = "VK_IMG_format_pvrtc")]
#[rustfmt::skip]
pub const VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG: VkFormat = 1000054006;
#[cfg(feature = "VK_IMG_format_pvrtc")]
#[rustfmt::skip]
pub const VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG: VkFormat = 1000054007;

#[cfg(feature = "VK_AMD_gcn_shader")]
#[rustfmt::skip]
pub const VK_AMD_GCN_SHADER_EXTENSION_NAME: &str = "VK_AMD_gcn_shader";
#[cfg(feature = "VK_AMD_gcn_shader")]
#[rustfmt::skip]
pub const VK_AMD_GCN_SHADER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_geometry_shader_passthrough")]
#[rustfmt::skip]
pub const VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME: &str = "VK_NV_geometry_shader_passthrough";
#[cfg(feature = "VK_NV_geometry_shader_passthrough")]
#[rustfmt::skip]
pub const VK_NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_glsl_shader")]
#[rustfmt::skip]
pub const VK_NV_GLSL_SHADER_EXTENSION_NAME: &str = "VK_NV_glsl_shader";
#[cfg(feature = "VK_NV_glsl_shader")]
#[rustfmt::skip]
pub const VK_NV_GLSL_SHADER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_gpu_shader_half_float")]
#[rustfmt::skip]
pub const VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME: &str = "VK_AMD_gpu_shader_half_float";
#[cfg(feature = "VK_AMD_gpu_shader_half_float")]
#[rustfmt::skip]
pub const VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_gpu_shader_int16")]
#[rustfmt::skip]
pub const VK_AMD_GPU_SHADER_INT16_EXTENSION_NAME: &str = "VK_AMD_gpu_shader_int16";
#[cfg(feature = "VK_AMD_gpu_shader_int16")]
#[rustfmt::skip]
pub const VK_AMD_GPU_SHADER_INT16_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_mixed_attachment_samples")]
#[rustfmt::skip]
pub const VK_AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME: &str = "VK_AMD_mixed_attachment_samples";
#[cfg(feature = "VK_AMD_mixed_attachment_samples")]
#[rustfmt::skip]
pub const VK_AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
#[rustfmt::skip]
pub const VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &str = "VK_NVX_multiview_per_view_attributes";
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
#[rustfmt::skip]
pub const VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
#[rustfmt::skip]
pub const VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX: VkSubpassDescriptionFlagBits = 0x00000001;
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
#[rustfmt::skip]
pub const VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX: VkSubpassDescriptionFlagBits = 0x00000002;

#[cfg(feature = "VK_AMD_negative_viewport_height")]
#[rustfmt::skip]
pub const VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_EXTENSION_NAME: &str = "VK_AMD_negative_viewport_height";
#[cfg(feature = "VK_AMD_negative_viewport_height")]
#[rustfmt::skip]
pub const VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_post_depth_coverage")]
#[rustfmt::skip]
pub const VK_EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME: &str = "VK_EXT_post_depth_coverage";
#[cfg(feature = "VK_EXT_post_depth_coverage")]
#[rustfmt::skip]
pub const VK_EXT_POST_DEPTH_COVERAGE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_queue_family_foreign")]
#[rustfmt::skip]
pub const VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME: &str = "VK_EXT_queue_family_foreign";
#[cfg(feature = "VK_EXT_queue_family_foreign")]
#[rustfmt::skip]
pub const VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_sample_mask_override_coverage")]
#[rustfmt::skip]
pub const VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME: &str = "VK_NV_sample_mask_override_coverage";
#[cfg(feature = "VK_NV_sample_mask_override_coverage")]
#[rustfmt::skip]
pub const VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_shader_ballot")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_BALLOT_EXTENSION_NAME: &str = "VK_AMD_shader_ballot";
#[cfg(feature = "VK_AMD_shader_ballot")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_BALLOT_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME: &str = "VK_AMD_shader_explicit_vertex_parameter";
#[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_shader_fragment_mask")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME: &str = "VK_AMD_shader_fragment_mask";
#[cfg(feature = "VK_AMD_shader_fragment_mask")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_EXTENSION_NAME: &str = "VK_AMD_shader_image_load_store_lod";
#[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_shader_trinary_minmax")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME: &str = "VK_AMD_shader_trinary_minmax";
#[cfg(feature = "VK_AMD_shader_trinary_minmax")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_swapchain_colorspace")]
#[rustfmt::skip]
pub const VK_EXT_SWAPCHAIN_COLORSPACE_EXTENSION_NAME: &str = "VK_EXT_swapchain_colorspace";
#[cfg(feature = "VK_EXT_swapchain_colorspace")]
#[rustfmt::skip]
pub const VK_EXT_SWAPCHAIN_COLORSPACE_SPEC_VERSION: usize = 3;

#[cfg(feature = "VK_NV_viewport_array2")]
#[rustfmt::skip]
pub const VK_NV_VIEWPORT_ARRAY2_EXTENSION_NAME: &str = "VK_NV_viewport_array2";
#[cfg(feature = "VK_NV_viewport_array2")]
#[rustfmt::skip]
pub const VK_NV_VIEWPORT_ARRAY2_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_maintenance7")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE7_EXTENSION_NAME: &str = "VK_KHR_maintenance7";
#[cfg(feature = "VK_KHR_maintenance7")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE7_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_maintenance7")]
#[rustfmt::skip]
pub type VkPhysicalDeviceLayeredApiKHR = i32;
#[cfg(feature = "VK_KHR_maintenance7")]
#[rustfmt::skip]
pub const VK_PHYSICAL_DEVICE_LAYERED_API_VULKAN_KHR: VkPhysicalDeviceLayeredApiKHR = 0;
#[cfg(feature = "VK_KHR_maintenance7")]
#[rustfmt::skip]
pub const VK_PHYSICAL_DEVICE_LAYERED_API_D3D12_KHR: VkPhysicalDeviceLayeredApiKHR = 1;
#[cfg(feature = "VK_KHR_maintenance7")]
#[rustfmt::skip]
pub const VK_PHYSICAL_DEVICE_LAYERED_API_METAL_KHR: VkPhysicalDeviceLayeredApiKHR = 2;
#[cfg(feature = "VK_KHR_maintenance7")]
#[rustfmt::skip]
pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGL_KHR: VkPhysicalDeviceLayeredApiKHR = 3;
#[cfg(feature = "VK_KHR_maintenance7")]
#[rustfmt::skip]
pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGLES_KHR: VkPhysicalDeviceLayeredApiKHR = 4;

#[cfg(feature = "VK_KHR_maintenance8")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE8_EXTENSION_NAME: &str = "VK_KHR_maintenance8";
#[cfg(feature = "VK_KHR_maintenance8")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE8_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_maintenance8")]
#[rustfmt::skip]
pub type VkAccessFlags3KHR = VkFlags;
#[cfg(feature = "VK_KHR_maintenance8")]
#[rustfmt::skip]
pub type VkAccessFlagBits3KHR = VkFlags;
#[cfg(feature = "VK_KHR_maintenance8")]
#[rustfmt::skip]
pub const VK_ACCESS_3_NONE_BIT_KHR: VkAccessFlagBits3KHR = 0x00000001;

#[cfg(feature = "VK_KHR_maintenance8")]
#[rustfmt::skip]
pub const VK_PIPELINE_CACHE_CREATE_INTERNALLY_SYNCHRONIZED_MERGE_BIT_KHR: VkPipelineCacheCreateFlagBits = 0x00000008;

#[cfg(feature = "VK_KHR_maintenance9")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE9_EXTENSION_NAME: &str = "VK_KHR_maintenance9";
#[cfg(feature = "VK_KHR_maintenance9")]
#[rustfmt::skip]
pub const VK_KHR_MAINTENANCE9_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_maintenance9")]
#[rustfmt::skip]
pub type VkDefaultVertexAttributeValueKHR = i32;
#[cfg(feature = "VK_KHR_maintenance9")]
#[rustfmt::skip]
pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ZERO_KHR: VkDefaultVertexAttributeValueKHR = 0;
#[cfg(feature = "VK_KHR_maintenance9")]
#[rustfmt::skip]
pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ONE_KHR: VkDefaultVertexAttributeValueKHR = 1;

#[cfg(feature = "VK_KHR_maintenance9")]
#[rustfmt::skip]
pub const VK_DEPENDENCY_ASYMMETRIC_EVENT_BIT_KHR: VkDependencyFlagBits = 0x00000040;

#[cfg(feature = "VK_KHR_maintenance9")]
#[rustfmt::skip]
pub const VK_QUERY_POOL_CREATE_RESET_BIT_KHR: VkQueryPoolCreateFlagBits = 0x00000001;

#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[rustfmt::skip]
pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &str = "VK_EXT_full_screen_exclusive";
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[rustfmt::skip]
pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: usize = 4;

#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[rustfmt::skip]
pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: VkResult = VkResult(-1000255000);

#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[rustfmt::skip]
pub type VkFullScreenExclusiveEXT = i32;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[rustfmt::skip]
pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: VkFullScreenExclusiveEXT = 0;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[rustfmt::skip]
pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: VkFullScreenExclusiveEXT = 1;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[rustfmt::skip]
pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: VkFullScreenExclusiveEXT = 2;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[rustfmt::skip]
pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: VkFullScreenExclusiveEXT = 3;

#[cfg(feature = "VK_AMD_shader_info")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_INFO_EXTENSION_NAME: &str = "VK_AMD_shader_info";
#[cfg(feature = "VK_AMD_shader_info")]
#[rustfmt::skip]
pub const VK_AMD_SHADER_INFO_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_shader_info")]
#[rustfmt::skip]
pub type VkShaderInfoTypeAMD = i32;
#[cfg(feature = "VK_AMD_shader_info")]
#[rustfmt::skip]
pub const VK_SHADER_INFO_TYPE_STATISTICS_AMD: VkShaderInfoTypeAMD = 0;
#[cfg(feature = "VK_AMD_shader_info")]
#[rustfmt::skip]
pub const VK_SHADER_INFO_TYPE_BINARY_AMD: VkShaderInfoTypeAMD = 1;
#[cfg(feature = "VK_AMD_shader_info")]
#[rustfmt::skip]
pub const VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD: VkShaderInfoTypeAMD = 2;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_APPLICATION_INFO:VkStructureType=0;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2:VkStructureType=1000109000;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2:VkStructureType=1000109001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO:VkStructureType=1000060013;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO:VkStructureType=1000157000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_DESCRIPTOR_SETS_INFO:VkStructureType=1000545003;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO:VkStructureType=1000060014;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO:VkStructureType=1000157001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO:VkStructureType=1000156002;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_MEMORY_STATUS:VkStructureType=1000545002;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO:VkStructureType=7;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2:VkStructureType=1000337004;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_COPY_2:VkStructureType=1000337006;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO:VkStructureType=12;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO:VkStructureType=1000244001;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2:VkStructureType=1000337009;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER:VkStructureType=44;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2:VkStructureType=1000314001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2:VkStructureType=1000146000;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO:VkStructureType=1000257002;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_USAGE_FLAGS_2_CREATE_INFO:VkStructureType=1000470006;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO:VkStructureType=13;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO:VkStructureType=40;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO:VkStructureType=42;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO:VkStructureType=41;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO:VkStructureType=1000044004;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO:VkStructureType=1000314006;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO:VkStructureType=39;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO:VkStructureType=29;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2:VkStructureType=1000337000;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2:VkStructureType=1000337002;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET:VkStructureType=36;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2:VkStructureType=1000337001;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2:VkStructureType=1000337003;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_IMAGE_INFO:VkStructureType=1000270007;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_MEMORY_INFO:VkStructureType=1000270004;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_MEMORY_TO_IMAGE_INFO:VkStructureType=1000270005;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO:VkStructureType=1000314003;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO:VkStructureType=33;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO:VkStructureType=34;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO:VkStructureType=1000161000;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO:VkStructureType=32;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT:VkStructureType=1000168001;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO:VkStructureType=1000161003;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT:VkStructureType=1000161004;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO:VkStructureType=1000085000;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS:VkStructureType=1000413002;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO:VkStructureType=3;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO:VkStructureType=1000060006;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO:VkStructureType=1000060004;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_CREATE_INFO:VkStructureType=1000070001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO:VkStructureType=1000060003;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO:VkStructureType=1000060005;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS:VkStructureType=1000413003;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_SUBRESOURCE_INFO:VkStructureType=1000470004;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO:VkStructureType=1000257004;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO:VkStructureType=2;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO:VkStructureType=1000174000;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO:VkStructureType=10;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO:VkStructureType=1000072002;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO:VkStructureType=1000077000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERANL_IMAGE_FORMAT_PROPERTIES:VkStructureType=1000071001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES:VkStructureType=1000071003;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES:VkStructureType=1000112001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO:VkStructureType=1000072000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO:VkStructureType=1000072001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES:VkStructureType=1000076001;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO:VkStructureType=8;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2:VkStructureType=1000059002;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3:VkStructureType=1000360000;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO:VkStructureType=37;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO:VkStructureType=28;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY:VkStructureType=1000270009;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_HOST_IMAGE_LAYOUT_TRANSITION:VkStructureType=1000270006;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_BLIT_2:VkStructureType=1000337008;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_COPY_2:VkStructureType=1000337007;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO:VkStructureType=14;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO:VkStructureType=1000147000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2:VkStructureType=1000059003;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER:VkStructureType=45;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2:VkStructureType=1000314002;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2:VkStructureType=1000146001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO:VkStructureType=1000156002;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2:VkStructureType=1000337010;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2:VkStructureType=1000146002;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2:VkStructureType=1000338003;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_TO_MEMORY_COPY:VkStructureType=1000270003;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO:VkStructureType=15;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO:VkStructureType=1000117002;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO:VkStructureType=1;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE:VkStructureType=6;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO:VkStructureType=1000060000;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO:VkStructureType=5;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER:VkStructureType=46;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2:VkStructureType=1000314000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO:VkStructureType=1000127001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS:VkStructureType=1000127000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_MAP_INFO:VkStructureType=1000271000;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO:VkStructureType=1000257003;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2:VkStructureType=1000146003;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_TO_IMAGE_COPY:VkStructureType=1000270002;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO:VkStructureType=1000271001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES:VkStructureType=1000083000;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES:VkStructureType=1000257000;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES:VkStructureType=1000199000;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES:VkStructureType=1000161001;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES:VkStructureType=1000161002;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES:VkStructureType=1000044003;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES:VkStructureType=1000232000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO:VkStructureType=1000071002;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO:VkStructureType=1000112000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO:VkStructureType=1000071000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO:VkStructureType=1000076000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2:VkStructureType=1000059000;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES:VkStructureType=1000197000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES:VkStructureType=1000388000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES:VkStructureType=1000070000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES:VkStructureType=1000270000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES:VkStructureType=1000270001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2:VkStructureType=1000059004;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES:VkStructureType=1000265000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES:VkStructureType=1000534000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES:VkStructureType=1000534002;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES:VkStructureType=1000168000;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES:VkStructureType=1000413000;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES:VkStructureType=1000413001;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES:VkStructureType=1000470000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES:VkStructureType=1000470001;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES:VkStructureType=1000545000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES:VkStructureType=1000545001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2:VkStructureType=1000059006;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES:VkStructureType=1000053001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES:VkStructureType=1000053002;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES:VkStructureType=1000466000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES:VkStructureType=1000068001;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES:VkStructureType=1000068002;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES:VkStructureType=1000117000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2:VkStructureType=1000059001;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES:VkStructureType=1000080000;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES:VkStructureType=1000130000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES:VkStructureType=1000156004;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES:VkStructureType=1000544000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES:VkStructureType=1000528000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES:VkStructureType=1000416000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2:VkStructureType=1000059008;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES:VkStructureType=1000314007;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES:VkStructureType=1000207000;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES:VkStructureType=1000207001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES:VkStructureType=1000120000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES:VkStructureType=1000190002;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES:VkStructureType=1000190000;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES:VkStructureType=49;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES:VkStructureType=50;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO:VkStructureType=17;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO:VkStructureType=26;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_CREATE_FLAGS_2_CREATE_INFO:VkStructureType=1000470005;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO:VkStructureType=25;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO:VkStructureType=27;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO:VkStructureType=20;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO:VkStructureType=30;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO:VkStructureType=24;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO:VkStructureType=1000534001;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO:VkStructureType=23;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO:VkStructureType=1000044002;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO:VkStructureType=1000068000;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO:VkStructureType=18;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO:VkStructureType=1000117003;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO:VkStructureType=21;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO:VkStructureType=1000190001;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO:VkStructureType=19;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO:VkStructureType=22;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PUSH_CONSTANTS_INFO:VkStructureType=1000545004;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_INFO:VkStructureType=1000545005;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO:VkStructureType=1000545006;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO:VkStructureType=11;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES:VkStructureType=1000388001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2:VkStructureType=1000059005;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDERING_AREA_INFO:VkStructureType=1000470003;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO:VkStructureType=1000044001;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_LOCATION_INFO:VkStructureType=1000232001;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDERING_INFO:VkStructureType=1000044000;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDERING_INPUT_ATTACHMENT_INDEX_INFO:VkStructureType=1000232002;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO:VkStructureType=43;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO:VkStructureType=38;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2:VkStructureType=1000109004;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO:VkStructureType=1000117001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO:VkStructureType=1000053000;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2:VkStructureType=1000337005;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO:VkStructureType=31;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO:VkStructureType=1000130001;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO:VkStructureType=1000156000;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES:VkStructureType=1000156005;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO:VkStructureType=1000156001;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO:VkStructureType=9;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO:VkStructureType=1000207005;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO:VkStructureType=1000314005;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO:VkStructureType=1000207002;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO:VkStructureType=1000207004;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO:VkStructureType=16;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2:VkStructureType=1000059007;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2:VkStructureType=1000146004;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBMIT_INFO:VkStructureType=4;
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2:VkStructureType=1000314004;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO:VkStructureType=1000109005;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2:VkStructureType=1000109003;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2:VkStructureType=1000109002;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE:VkStructureType=1000199001;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO:VkStructureType=1000109006;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBRESOURCE_HOST_MEMCPY_SIZE:VkStructureType=1000270008;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2:VkStructureType=1000338002;
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SURESOURCE_LAYOUT_2:VkStructureType=1000338002;
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO:VkStructureType=1000207003;
#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET:VkStructureType=35;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR:VkStructureType=1000060010;
#[cfg(feature = "VK_KHR_android_surface")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR:VkStructureType=1000008000;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR:VkStructureType=1000109000;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR:VkStructureType=1000109001;
#[cfg(all(feature = "VK_KHR_bind_memory2", feature = "VK_KHR_device_group"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR:VkStructureType=1000060013;
#[cfg(feature = "VK_KHR_bind_memory2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR:VkStructureType=1000157000;
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_DESCRIPTOR_SETS_INFO_KHR:VkStructureType=1000545003;
#[cfg(all(feature = "VK_KHR_bind_memory2", feature = "VK_KHR_device_group"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR:VkStructureType=1000060014;
#[cfg(feature = "VK_KHR_bind_memory2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR:VkStructureType=1000157001;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR:VkStructureType=1000060009;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR:VkStructureType=1000156002;
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BIND_MEMORY_STATUS_KHR:VkStructureType=1000545002;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2_KHR:VkStructureType=1000337004;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT:VkStructureType=1000316005;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_COPY_2_KHR:VkStructureType=1000337006;
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR:VkStructureType=1000244001;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2_KHR:VkStructureType=1000337009;
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR:VkStructureType=1000314001;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR:VkStructureType=1000146000;
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR:VkStructureType=1000257002;
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR:VkStructureType=1000470006;
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX:VkStructureType=1000086002;
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX:VkStructureType=1000086003;
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR:VkStructureType=1000044004;
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR:VkStructureType=1000314006;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2_KHR:VkStructureType=1000337000;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2_KHR:VkStructureType=1000337002;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2_KHR:VkStructureType=1000337001;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2_KHR:VkStructureType=1000337003;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_IMAGE_INFO_EXT:VkStructureType=1000270007;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_MEMORY_INFO_EXT:VkStructureType=1000270004;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_COPY_MEMORY_TO_IMAGE_INFO_EXT:VkStructureType=1000270005;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR:VkStructureType=1000078002;
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT:VkStructureType=1000022002;
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT:VkStructureType=1000022000;
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT:VkStructureType=1000022001;
#[cfg(feature = "VK_EXT_debug_report")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT:VkStructureType=1000011000;
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT:VkStructureType=1000128002;
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT:VkStructureType=1000128003;
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT:VkStructureType=1000128004;
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT:VkStructureType=1000128000;
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT:VkStructureType=1000128001;
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR:VkStructureType=1000314003;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT:VkStructureType=1000316003;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT:VkStructureType=1000316011;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT:VkStructureType=1000316012;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT:VkStructureType=1000316004;
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT:VkStructureType=1000161000;
#[cfg(feature = "VK_KHR_maintenance3")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR:VkStructureType=1000168001;
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT:VkStructureType=1000161003;
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT:VkStructureType=1000161004;
#[cfg(feature = "VK_KHR_descriptor_update_template")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR:VkStructureType=1000085000;
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR:VkStructureType=1000413002;
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT:VkStructureType=1000091001;
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX:VkStructureType=1000086005;
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX:VkStructureType=1000086004;
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR:VkStructureType=1000060006;
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR:VkStructureType=1000060004;
#[cfg(feature = "VK_KHR_device_group_creation")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_CREATE_INFO_KHR:VkStructureType=1000070001;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR:VkStructureType=1000060007;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR:VkStructureType=1000060011;
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR:VkStructureType=1000060003;
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR:VkStructureType=1000060005;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR:VkStructureType=1000060012;
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR:VkStructureType=1000413003;
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_SUBRESOURCE_INFO_KHR:VkStructureType=1000470004;
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR:VkStructureType=1000257004;
#[cfg(feature = "VK_EXT_global_priority")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT:VkStructureType=1000174000;
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR:VkStructureType=1000174000;
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT:VkStructureType=1000091002;
#[cfg(feature = "VK_KHR_display")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR:VkStructureType=1000002000;
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT:VkStructureType=1000091000;
#[cfg(feature = "VK_KHR_display_swapchain")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR:VkStructureType=1000003000;
#[cfg(feature = "VK_KHR_display")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR:VkStructureType=1000002001;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT:VkStructureType=1000158000;
#[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "VK_KHR_format_feature_flags2"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT:VkStructureType=1000158006;
#[cfg(feature = "VK_KHR_external_fence_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR:VkStructureType=1000114001;
#[cfg(feature = "VK_KHR_external_memory")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR:VkStructureType=1000072002;
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR:VkStructureType=1000073001;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT:VkStructureType=1000311004;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT:VkStructureType=1000311003;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT:VkStructureType=1000311002;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT:VkStructureType=1000311008;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT:VkStructureType=1000311001;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT:VkStructureType=1000311000;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT:VkStructureType=1000311010;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT:VkStructureType=1000311006;
#[cfg(feature = "VK_KHR_external_semaphore")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR:VkStructureType=1000077000;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR:VkStructureType=1000078001;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERANL_IMAGE_FORMAT_PROPERTIES_KHR:VkStructureType=1000071001;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR:VkStructureType=1000071003;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR:VkStructureType=1000112001;
#[cfg(feature = "VK_KHR_external_memory")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR:VkStructureType=1000072000;
#[cfg(feature = "VK_KHR_external_memory")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR:VkStructureType=1000072001;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR:VkStructureType=1000076001;
#[cfg(feature = "VK_KHR_external_fence_fd")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR:VkStructureType=1000115001;
#[cfg(feature = "VK_KHR_external_fence_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR:VkStructureType=1000114002;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR:VkStructureType=1000059002;
#[cfg(feature = "VK_KHR_format_feature_flags2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3_KHR:VkStructureType=1000360000;
#[cfg(feature = "VK_EXT_hdr_metadata")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_HDR_METADATA_EXT:VkStructureType=1000105000;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT:VkStructureType=1000270009;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_HOST_IMAGE_LAYOUT_TRANSITION_EXT:VkStructureType=1000270006;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_BLIT_2_KHR:VkStructureType=1000337008;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT:VkStructureType=1000316006;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_COPY_2_KHR:VkStructureType=1000337007;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT:VkStructureType=1000158004;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT:VkStructureType=1000158003;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT:VkStructureType=1000158005;
#[cfg(feature = "VK_KHR_image_format_list")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR:VkStructureType=1000147000;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR:VkStructureType=1000059003;
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR:VkStructureType=1000314002;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR:VkStructureType=1000146001;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR:VkStructureType=1000156002;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2_KHR:VkStructureType=1000337010;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR:VkStructureType=1000146002;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT:VkStructureType=1000338003;
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_KHR:VkStructureType=1000338003;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR:VkStructureType=1000060008;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_TO_MEMORY_COPY_EXT:VkStructureType=1000270003;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT:VkStructureType=1000316007;
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR:VkStructureType=1000117002;
#[cfg(feature = "VK_KHR_external_fence_fd")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR:VkStructureType=1000115000;
#[cfg(feature = "VK_KHR_external_fence_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR:VkStructureType=1000114000;
#[cfg(feature = "VK_KHR_external_memory_fd")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR:VkStructureType=1000074000;
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT:VkStructureType=1000178000;
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR:VkStructureType=1000073000;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT:VkStructureType=1000311005;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT:VkStructureType=1000311009;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT:VkStructureType=1000311011;
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT:VkStructureType=1000311007;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR:VkStructureType=1000079000;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR:VkStructureType=1000078000;
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX:VkStructureType=1000086001;
#[cfg(feature = "VK_MVK_ios_surface")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK:VkStructureType=1000122000;
#[cfg(feature = "VK_EXT_layer_settings")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_LAYER_SETTINGS_CREATE_INFO_EXT:VkStructureType=1000496000;
#[cfg(feature = "VK_MVK_macos_surface")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK:VkStructureType=1000123000;
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR:VkStructureType=1000060000;
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR:VkStructureType=1000314000;
#[cfg(feature = "VK_KHR_maintenance8")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_ACCESS_FLAGS_3_KHR:VkStructureType=1000574002;
#[cfg(feature = "VK_KHR_dedicated_allocation")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR:VkStructureType=1000127001;
#[cfg(feature = "VK_KHR_dedicated_allocation")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR:VkStructureType=1000127000;
#[cfg(feature = "VK_KHR_external_memory_fd")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR:VkStructureType=1000074001;
#[cfg(feature = "VK_KHR_external_memory_fd")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR:VkStructureType=1000074002;
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR:VkStructureType=1000073003;
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT:VkStructureType=1000178001;
#[cfg(feature = "VK_KHR_map_memory2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_MAP_INFO_KHR:VkStructureType=1000271000;
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR:VkStructureType=1000257003;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR:VkStructureType=1000146003;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_TO_IMAGE_COPY_EXT:VkStructureType=1000270002;
#[cfg(feature = "VK_KHR_map_memory2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO_KHR:VkStructureType=1000271001;
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR:VkStructureType=1000073002;
#[cfg(feature = "VK_EXT_metal_surface")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT:VkStructureType=1000217000;
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT:VkStructureType=1000143004;
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX:VkStructureType=1000086000;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT:VkStructureType=1000316010;
#[cfg(feature = "VK_KHR_16bit_storage")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR:VkStructureType=1000083000;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT:VkStructureType=1000148000;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT:VkStructureType=1000148001;
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR:VkStructureType=1000257000;
#[cfg(feature = "VK_EXT_conservative_rasterization")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT:VkStructureType=1000101000;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR:VkStructureType=1000199000;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT:VkStructureType=1000316001;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT:VkStructureType=1000316002;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT:VkStructureType=1000316000;
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT:VkStructureType=1000161001;
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT:VkStructureType=1000161002;
#[cfg(feature = "VK_EXT_discard_rectangles")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT:VkStructureType=1000099000;
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR:VkStructureType=1000044003;
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR:VkStructureType=1000232000;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR:VkStructureType=1000071002;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR:VkStructureType=1000112000;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR:VkStructureType=1000071000;
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT:VkStructureType=1000178002;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR:VkStructureType=1000076000;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR:VkStructureType=1000059000;
#[cfg(feature = "VK_KHR_shader_float_controls")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR:VkStructureType=1000197000;
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR:VkStructureType=1000388000;
#[cfg(feature = "VK_KHR_device_group_creation")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR:VkStructureType=1000070000;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT:VkStructureType=1000270000;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT:VkStructureType=1000270001;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT:VkStructureType=1000158002;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR:VkStructureType=1000059004;
#[cfg(feature = "VK_KHR_index_type_uint8")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR:VkStructureType=1000265000;
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR:VkStructureType=1000562003;
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR:VkStructureType=1000562002;
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR:VkStructureType=1000562004;
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR:VkStructureType=1000534000;
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR:VkStructureType=1000534002;
#[cfg(feature = "VK_KHR_maintenance3")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR:VkStructureType=1000168000;
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR:VkStructureType=1000413000;
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR:VkStructureType=1000413001;
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR:VkStructureType=1000470000;
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR:VkStructureType=1000470001;
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR:VkStructureType=1000545000;
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR:VkStructureType=1000545001;
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR:VkStructureType=1000562000;
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR:VkStructureType=1000562001;
#[cfg(feature = "VK_KHR_maintenance8")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR:VkStructureType=1000574000;
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR:VkStructureType=1000584000;
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR:VkStructureType=1000584001;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR:VkStructureType=1000059006;
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR:VkStructureType=1000053001;
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX:VkStructureType=1000097000;
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR:VkStructureType=1000053002;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT:VkStructureType=1000466000;
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT:VkStructureType=1000068001;
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT:VkStructureType=1000068002;
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR:VkStructureType=1000117000;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR:VkStructureType=1000059001;
#[cfg(feature = "VK_KHR_push_descriptor")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR:VkStructureType=1000080000;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT:VkStructureType=1000130000;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR:VkStructureType=1000156004;
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT:VkStructureType=1000143003;
#[cfg(feature = "VK_KHR_shader_expect_assume")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR:VkStructureType=1000544000;
#[cfg(feature = "VK_KHR_shader_float_controls2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR:VkStructureType=1000528000;
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR:VkStructureType=1000416000;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR:VkStructureType=1000059008;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR:VkStructureType=1000119000;
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR:VkStructureType=1000314007;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR:VkStructureType=1000207000;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR:VkStructureType=1000207001;
#[cfg(feature = "VK_KHR_variable_pointers")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR:VkStructureType=1000120000;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR:VkStructureType=1000190002;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT:VkStructureType=1000190000;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR:VkStructureType=1000190000;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT:VkStructureType=1000148002;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV:VkStructureType=1000152000;
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV:VkStructureType=1000149000;
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR:VkStructureType=1000470005;
#[cfg(feature = "VK_EXT_discard_rectangles")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT:VkStructureType=1000099001;
#[cfg(feature = "VK_EXT_conservative_rasterization")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT:VkStructureType=1000101001;
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR:VkStructureType=1000534001;
#[cfg(feature = "VK_AMD_rasterization_order")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD:VkStructureType=1000018000;
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO_KHR:VkStructureType=1000044002;
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT:VkStructureType=1000068000;
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT:VkStructureType=1000143002;
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR:VkStructureType=1000117003;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT:VkStructureType=1000190001;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR:VkStructureType=1000190001;
#[cfg(feature = "VK_NV_viewport_swizzle")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV:VkStructureType=1000098000;
#[cfg(feature = "VK_NV_clip_space_w_scaling")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV:VkStructureType=1000087000;
#[cfg(feature = "VK_KHR_swapchain")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR:VkStructureType=1000001001;
#[cfg(feature = "VK_KHR_incremental_present")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR:VkStructureType=1000084000;
#[cfg(feature = "VK_GOOGLE_display_timing")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE:VkStructureType=1000092000;
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PUSH_CONSTANTS_INFO_KHR:VkStructureType=1000545004;
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_INFO_KHR:VkStructureType=1000545005;
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR:VkStructureType=1000545006;
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR:VkStructureType=1000388001;
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR:VkStructureType=1000584002;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR:VkStructureType=1000059005;
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDERING_AREA_INFO_KHR:VkStructureType=1000470003;
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO_KHR:VkStructureType=1000044001;
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_LOCATION_INFO_KHR:VkStructureType=1000232001;
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDERING_INFO_KHR:VkStructureType=1000044000;
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR:VkStructureType=1000232002;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR:VkStructureType=1000109004;
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR:VkStructureType=1000117001;
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR:VkStructureType=1000053000;
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT:VkStructureType=1000143001;
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2_KHR:VkStructureType=1000337005;
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT:VkStructureType=1000316008;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT:VkStructureType=1000130001;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR:VkStructureType=1000156000;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR:VkStructureType=1000156005;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR:VkStructureType=1000156001;
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT:VkStructureType=1000143000;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR:VkStructureType=1000079001;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR:VkStructureType=1000078003;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO_KHR:VkStructureType=1000207005;
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR:VkStructureType=1000314005;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO_KHR:VkStructureType=1000207002;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO_KHR:VkStructureType=1000207004;
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT:VkStructureType=1000160001;
#[cfg(feature = "VK_KHR_shared_presentable_image")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR:VkStructureType=1000111000;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR:VkStructureType=1000059007;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR:VkStructureType=1000146004;
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR:VkStructureType=1000314004;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR:VkStructureType=1000109005;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR:VkStructureType=1000109003;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR:VkStructureType=1000109002;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR:VkStructureType=1000199001;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR:VkStructureType=1000109006;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBRESOURCE_HOST_MEMCPY_SIZE_EXT:VkStructureType=1000270008;
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_KHR:VkStructureType=1000338002;
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SURESOURCE_LAYOUT_2_EXT:VkStructureType=1000338002;
#[cfg(feature = "VK_EXT_display_surface_counter")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT:VkStructureType=1000090000;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR:VkStructureType=1000119001;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT:VkStructureType=1000255002;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR:VkStructureType=1000119002;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT:VkStructureType=1000255000;
#[cfg(all(feature = "VK_EXT_full_screen_exclusive", feature = "VK_KHR_win32_surface"))]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT:VkStructureType=1000255001;
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT:VkStructureType=1000091003;
#[cfg(feature = "VK_KHR_swapchain")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR:VkStructureType=1000001000;
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD:VkStructureType=1000041000;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR:VkStructureType=1000207003;
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT:VkStructureType=1000160000;
#[cfg(feature = "VK_EXT_validation_flags")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT:VkStructureType=1000061000;
#[cfg(feature = "VK_NN_vi_surface")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN:VkStructureType=1000062000;
#[cfg(feature = "VK_KHR_wayland_surface")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR:VkStructureType=1000006000;
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR:VkStructureType=1000075000;
#[cfg(feature = "VK_KHR_win32_surface")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR:VkStructureType=1000009000;
#[cfg(feature = "VK_KHR_xcb_surface")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR:VkStructureType=1000005000;
#[cfg(feature = "VK_KHR_xlib_surface")]#[rustfmt::skip]pub const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR:VkStructureType=1000004000;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkAcquireNextImageInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub swapchain:VkSwapchainKHR,pub timeout:u64,pub semaphore:VkSemaphore,pub fence:VkFence,pub deviceMask:u32,}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkAcquireNextImageInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkAcquireNextImageInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkAllocationCallbacks{pub pUserData:*mut core::ffi::c_void,pub pfnAllocation:PFN_vkAllocationFunction,pub pfnReallocation:PFN_vkReallocationFunction,pub pfnFree:PFN_vkFreeFunction,pub pfnInternalAllocation:Option<PFN_vkInternalAllocationNotification>,pub pfnInternalFree:Option<PFN_vkInternalFreeNotification>,}
#[cfg(feature = "VK_KHR_android_surface")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkAndroidSurfaceCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkAndroidSurfaceCreateFlagsKHR,pub window:*mut android::ANativeWindow,}
#[cfg(feature = "VK_KHR_android_surface")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkAndroidSurfaceCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_android_surface")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkAndroidSurfaceCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkApplicationInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pApplicationName:*const core::ffi::c_char,pub applicationVersion:u32,pub pEngineName:*const core::ffi::c_char,pub engineVersion:u32,pub apiVersion:u32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkApplicationInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkApplicationInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_APPLICATION_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkAttachmentDescription{pub flags:VkAttachmentDescriptionFlags,pub format:VkFormat,pub samples:VkSampleCountFlagBits,pub loadOp:VkAttachmentLoadOp,pub storeOp:VkAttachmentStoreOp,pub stencilLoadOp:VkAttachmentLoadOp,pub stencilStoreOp:VkAttachmentStoreOp,pub initialLayout:VkImageLayout,pub finalLayout:VkImageLayout,}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkAttachmentDescription2=VkAttachmentDescription2KHR;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkAttachmentDescription2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkAttachmentDescriptionFlags,pub format:VkFormat,pub samples:VkSampleCountFlagBits,pub loadOp:VkAttachmentLoadOp,pub storeOp:VkAttachmentStoreOp,pub stencilLoadOp:VkAttachmentLoadOp,pub stencilStoreOp:VkAttachmentStoreOp,pub initialLayout:VkImageLayout,pub finalLayout:VkImageLayout,}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkAttachmentDescription2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkAttachmentDescription2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR;}
#[derive(Debug, Clone, Copy)] #[rustfmt::skip]#[repr(C)]pub struct VkAttachmentReference{pub attachment:u32,pub layout:VkImageLayout,}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkAttachmentReference2=VkAttachmentReference2KHR;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkAttachmentReference2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub attachment:u32,pub layout:VkImageLayout,pub aspectMask:VkImageAspectFlags,}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkAttachmentReference2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkAttachmentReference2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR;}
#[cfg(feature = "VK_EXT_sample_locations")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkAttachmentSampleLocationsEXT{pub attachmentIndex:u32,pub sampleLocationsInfo:VkSampleLocationsInfoEXT,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkBindBufferMemoryDeviceGroupInfo=VkBindBufferMemoryDeviceGroupInfoKHR;
#[cfg(all(feature = "VK_KHR_bind_memory2", feature = "VK_KHR_device_group"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBindBufferMemoryDeviceGroupInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub deviceIndexCount:u32,pub pDeviceIndices:*const u32,}
#[cfg(all(feature = "VK_KHR_bind_memory2", feature = "VK_KHR_device_group"))]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBindBufferMemoryDeviceGroupInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(all(feature = "VK_KHR_bind_memory2", feature = "VK_KHR_device_group"))]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBindBufferMemoryDeviceGroupInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkBindBufferMemoryInfo=VkBindBufferMemoryInfoKHR;
#[cfg(feature = "VK_KHR_bind_memory2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBindBufferMemoryInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub buffer:VkBuffer,pub memory:VkDeviceMemory,pub memoryOffset:VkDeviceSize,}
#[cfg(feature = "VK_KHR_bind_memory2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBindBufferMemoryInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_bind_memory2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBindBufferMemoryInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkBindDescriptorSetsInfo=VkBindDescriptorSetsInfoKHR;
#[cfg(feature = "VK_KHR_maintenance6")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBindDescriptorSetsInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub stageFlags:VkShaderStageFlags,pub layout:VkPipelineLayout,pub firstSet:u32,pub descriptorSetCount:u32,pub pDescriptorSets:*const VkDescriptorSet,pub dynamicOffsetCount:u32,pub pDynamicOffsets:*const u32,}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBindDescriptorSetsInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBindDescriptorSetsInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BIND_DESCRIPTOR_SETS_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkBindImageMemoryDeviceGroupInfo=VkBindImageMemoryDeviceGroupInfoKHR;
#[cfg(all(feature = "VK_KHR_bind_memory2", feature = "VK_KHR_device_group"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBindImageMemoryDeviceGroupInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub deviceIndexCount:u32,pub pDeviceIndices:*const u32,pub splitInstanceBindRegionCount:u32,pub pSplitInstanceBindRegions:*const VkRect2D,}
#[cfg(all(feature = "VK_KHR_bind_memory2", feature = "VK_KHR_device_group"))]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBindImageMemoryDeviceGroupInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(all(feature = "VK_KHR_bind_memory2", feature = "VK_KHR_device_group"))]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBindImageMemoryDeviceGroupInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkBindImageMemoryInfo=VkBindImageMemoryInfoKHR;
#[cfg(feature = "VK_KHR_bind_memory2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBindImageMemoryInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub image:VkImage,pub memory:VkDeviceMemory,pub memoryOffset:VkDeviceSize,}
#[cfg(feature = "VK_KHR_bind_memory2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBindImageMemoryInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_bind_memory2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBindImageMemoryInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR;}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBindImageMemorySwapchainInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub swapchain:VkSwapchainKHR,pub imageIndex:u32,}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBindImageMemorySwapchainInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBindImageMemorySwapchainInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkBindImagePlaneMemoryInfo=VkBindImagePlaneMemoryInfoKHR;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBindImagePlaneMemoryInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub planeAspect:VkImageAspectFlags,}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBindImagePlaneMemoryInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBindImagePlaneMemoryInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkBindMemoryStatus=VkBindMemoryStatusKHR;
#[cfg(feature = "VK_KHR_maintenance6")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBindMemoryStatusKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pResult:*mut VkResult,}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBindMemoryStatusKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBindMemoryStatusKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BIND_MEMORY_STATUS_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBindSparseInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub waitSemaphoreCount:u32,pub pWaitSemaphores:*const VkSemaphore,pub bufferBindCount:u32,pub pBufferBinds:*const VkSparseBufferMemoryBindInfo,pub imageOpaqueBindCount:u32,pub pImageOpaqueBinds:*const VkSparseImageOpaqueMemoryBindInfo,pub imageBindCount:u32,pub pImageBinds:*const VkSparseImageMemoryBindInfo,pub signalSemaphoreCount:u32,pub pSignalSemaphores:*const VkSemaphore,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBindSparseInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBindSparseInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BIND_SPARSE_INFO;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkBlitImageInfo2=VkBlitImageInfo2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBlitImageInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcImage:VkImage,pub srcImageLayout:VkImageLayout,pub dstImage:VkImage,pub dstImageLayout:VkImageLayout,pub regionCount:u32,pub pRegions:*const VkImageBlit2KHR,pub filter:VkFilter,}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBlitImageInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBlitImageInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2_KHR;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferCaptureDescriptorDataInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub buffer:VkBuffer,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferCaptureDescriptorDataInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferCaptureDescriptorDataInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferCopy{pub srcOffset:VkDeviceSize,pub dstOffset:VkDeviceSize,pub size:VkDeviceSize,}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkBufferCopy2=VkBufferCopy2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferCopy2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcOffset:VkDeviceSize,pub dstOffset:VkDeviceSize,pub size:VkDeviceSize,}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferCopy2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferCopy2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_COPY_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkBufferCreateFlags,pub size:VkDeviceSize,pub usage:VkBufferUsageFlags,pub sharingMode:VkSharingMode,pub queueFamilyIndexCount:u32,pub pQueueFamilyIndices:*const u32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkBufferDeviceAddressInfo=VkBufferDeviceAddressInfoKHR;
#[cfg(feature = "VK_KHR_buffer_device_address")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferDeviceAddressInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub buffer:VkBuffer,}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferDeviceAddressInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferDeviceAddressInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferImageCopy{pub bufferOffset:VkDeviceSize,pub bufferRowLength:u32,pub bufferImageHeight:u32,pub imageSubresource:VkImageSubresourceLayers,pub imageOffset:VkOffset3D,pub imageExtent:VkExtent3D,}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkBufferImageCopy2=VkBufferImageCopy2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferImageCopy2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub bufferOffset:VkDeviceSize,pub bufferRowLength:u32,pub bufferImageHeight:u32,pub imageSubresource:VkImageSubresourceLayers,pub imageOffset:VkOffset3D,pub imageExtent:VkExtent3D,}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferImageCopy2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferImageCopy2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferMemoryBarrier{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcAccessMask:VkAccessFlags,pub dstAccessMask:VkAccessFlags,pub srcQueueFamilyIndex:u32,pub dstQueueFamilyIndex:u32,pub buffer:VkBuffer,pub offset:VkDeviceSize,pub size:VkDeviceSize,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferMemoryBarrier{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferMemoryBarrier{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkBufferMemoryBarrier2=VkBufferMemoryBarrier2KHR;
#[cfg(feature = "VK_KHR_synchronization2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferMemoryBarrier2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcStageMask:VkPipelineStageFlags2KHR,pub srcAccessMask:VkAccessFlags2KHR,pub dstStageMask:VkPipelineStageFlags2KHR,pub dstAccessMask:VkAccessFlags2KHR,pub srcQueueFamilyIndex:u32,pub dstQueueFamilyIndex:u32,pub buffer:VkBuffer,pub offset:VkDeviceSize,pub size:VkDeviceSize,}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferMemoryBarrier2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferMemoryBarrier2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkBufferMemoryRequirementsInfo2=VkBufferMemoryRequirementsInfo2KHR;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferMemoryRequirementsInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub buffer:VkBuffer,}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferMemoryRequirementsInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferMemoryRequirementsInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkBufferOpaqueCaptureAddressCreateInfo=VkBufferOpaqueCaptureAddressCreateInfoKHR;
#[cfg(feature = "VK_KHR_buffer_device_address")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferOpaqueCaptureAddressCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub opaqueCaptureAddress:u64,}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferOpaqueCaptureAddressCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferOpaqueCaptureAddressCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkBufferUsageFlags2CreateInfo=VkBufferUsageFlags2CreateInfoKHR;
#[cfg(feature = "VK_KHR_maintenance5")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferUsageFlags2CreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub usage:VkBufferUsageFlags2KHR,}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferUsageFlags2CreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferUsageFlags2CreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkBufferViewCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkBufferViewCreateFlags,pub buffer:VkBuffer,pub format:VkFormat,pub offset:VkDeviceSize,pub range:VkDeviceSize,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkBufferViewCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkBufferViewCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO;}
#[derive(Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkClearAttachment{pub aspectMask:VkImageAspectFlags,pub colorAttachment:u32,pub clearValue:VkClearValue,}
#[derive(Debug, Clone, Copy)] #[rustfmt::skip]#[repr(C)]pub struct VkClearDepthStencilValue{pub depth:core::ffi::c_float,pub stencil:u32,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkClearRect{pub rect:VkRect2D,pub baseArrayLayer:u32,pub layerCount:u32,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCmdProcessCommandsInfoNVX{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub objectTable:VkObjectTableNVX,pub indirectCommandsLayout:VkIndirectCommandsLayoutNVX,pub indirectCommandsTokenCount:u32,pub pIndirectCommandsTokens:*const VkIndirectCommandsTokenNVX,pub maxSequencesCount:u32,pub targetCommandBuffer:VkCommandBuffer,pub sequencesCountBuffer:VkBuffer,pub sequencesCountOffset:VkDeviceSize,pub sequencesIndexBuffer:VkBuffer,pub sequencesIndexOffset:VkDeviceSize,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCmdProcessCommandsInfoNVX{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCmdProcessCommandsInfoNVX{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX;}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCmdReserveSpaceForCommandsInfoNVX{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub objectTable:VkObjectTableNVX,pub indirectCommandsLayout:VkIndirectCommandsLayoutNVX,pub maxSequencesCount:u32,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCmdReserveSpaceForCommandsInfoNVX{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCmdReserveSpaceForCommandsInfoNVX{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCommandBufferAllocateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub commandPool:VkCommandPool,pub level:VkCommandBufferLevel,pub commandBufferCount:u32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCommandBufferAllocateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCommandBufferAllocateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCommandBufferBeginInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkCommandBufferUsageFlags,pub pInheritanceInfo:*const VkCommandBufferInheritanceInfo,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCommandBufferBeginInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCommandBufferBeginInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCommandBufferInheritanceInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub renderPass:Option<VkRenderPass>,pub subpass:u32,pub framebuffer:Option<VkFramebuffer>,pub occlusionQueryEnable:VkBool32,pub queryFlags:VkQueryControlFlags,pub pipelineStatistics:VkQueryPipelineStatisticFlags,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCommandBufferInheritanceInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCommandBufferInheritanceInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkCommandBufferInheritanceRenderingInfo=VkCommandBufferInheritanceRenderingInfoKHR;
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCommandBufferInheritanceRenderingInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkRenderingFlagsKHR,pub viewMask:u32,pub colorAttachmentCount:u32,pub pColorAttachmentFormats:*const VkFormat,pub denpthAttachmentFormat:VkFormat,pub stencilAttachmentFormat:VkFormat,pub rasterizationSamples:VkSampleCountFlagBits,}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCommandBufferInheritanceRenderingInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCommandBufferInheritanceRenderingInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkCommandBufferSubmitInfo=VkCommandBufferSubmitInfoKHR;
#[cfg(feature = "VK_KHR_synchronization2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCommandBufferSubmitInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub commandBuffer:VkCommandBuffer,pub deviceMask:u32,}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCommandBufferSubmitInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCommandBufferSubmitInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCommandPoolCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkCommandPoolCreateFlags,pub queueFamilyIndex:u32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCommandPoolCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCommandPoolCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;}
#[derive(Debug, Clone, Copy)] #[rustfmt::skip]#[repr(C)]pub struct VkComponentMapping{pub r:VkComponentSwizzle,pub g:VkComponentSwizzle,pub b:VkComponentSwizzle,pub a:VkComponentSwizzle,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkComputePipelineCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineCreateFlags,pub stage:VkPipelineShaderStageCreateInfo,pub layout:Option<VkPipelineLayout>,pub basePipelineHandle:Option<VkPipeline>,pub basePipelineIndex:i32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkComputePipelineCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkComputePipelineCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkCopyBufferInfo2=VkCopyBufferInfo2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCopyBufferInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcBuffer:VkBuffer,pub dstBuffer:VkBuffer,pub regionCount:u32,pub pRegions:*const VkBufferCopy2KHR,}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCopyBufferInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCopyBufferInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkCopyBufferToImageInfo2=VkCopyBufferToImageInfo2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCopyBufferToImageInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcBuffer:VkBuffer,pub dstImage:VkImage,pub dstImageLayout:VkImageLayout,pub regionCount:u32,pub pRegions:*const VkBufferImageCopy2KHR,}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCopyBufferToImageInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCopyBufferToImageInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCopyDescriptorSet{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcSet:VkDescriptorSet,pub srcBinding:u32,pub srcArrayElement:u32,pub dstSet:VkDescriptorSet,pub dstBinding:u32,pub dstArrayElement:u32,pub descriptorCount:u32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCopyDescriptorSet{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCopyDescriptorSet{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkCopyImageInfo2=VkCopyImageInfo2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCopyImageInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcImage:VkImage,pub srcImageLayout:VkImageLayout,pub dstImage:VkImage,pub dstImageLayout:VkImageLayout,pub regionCount:u32,pub pRegions:*const VkImageCopy2KHR,}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCopyImageInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCopyImageInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkCopyImageToBufferInfo2=VkCopyImageToBufferInfo2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCopyImageToBufferInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcImage:VkImage,pub srcImageLayout:VkImageLayout,pub dstBuffer:VkBuffer,pub regionCount:u32,pub pRegions:*const VkBufferImageCopy2KHR,}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCopyImageToBufferInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCopyImageToBufferInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkCopyImageToImageInfo=VkCopyImageToImageInfoEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCopyImageToImageInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkHostImageCopyFlagsEXT,pub srcImage:VkImage,pub srcImageLayout:VkImageLayout,pub dstImage:VkImage,pub dstImageLayout:VkImageLayout,pub regionCount:u32,pub pRegions:*const VkImageCopy2KHR,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCopyImageToImageInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCopyImageToImageInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COPY_IMAGE_TO_IMAGE_INFO_EXT;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkCopyImageToMemoryInfo=VkCopyImageToMemoryInfoEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCopyImageToMemoryInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkHostImageCopyFlagsEXT,pub srcImage:VkImage,pub srcImageLayout:VkImageLayout,pub regionCount:u32,pub pRegions:*const VkImageToMemoryCopyEXT,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCopyImageToMemoryInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCopyImageToMemoryInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COPY_IMAGE_TO_MEMORY_INFO_EXT;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkCopyMemoryToImageInfo=VkCopyMemoryToImageInfoEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkCopyMemoryToImageInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkHostImageCopyFlagsEXT,pub dstImage:VkImage,pub dstImageLayout:VkImageLayout,pub regionCount:u32,pub pRegions:*const VkMemoryToImageCopyEXT,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkCopyMemoryToImageInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkCopyMemoryToImageInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_COPY_MEMORY_TO_IMAGE_INFO_EXT;}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkD3D12FenceSubmitInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub waitSemaphoreValuesCount:u32,pub pWaitSemaphoreValues:*const u64,pub signalSemaphoreValuesCount:u32,pub pSignalSemaphoreValues:*const u64,}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkD3D12FenceSubmitInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkD3D12FenceSubmitInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR;}
#[cfg(feature = "VK_EXT_debug_marker")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDebugMarkerMarkerInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pMarkerName:*const core::ffi::c_char,pub color:[core::ffi::c_float; 4],}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDebugMarkerMarkerInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDebugMarkerMarkerInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT;}
#[cfg(feature = "VK_EXT_debug_marker")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDebugMarkerObjectNameInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub objectType:VkDebugReportObjectTypeEXT,pub object:u64,pub pObjectName:*const core::ffi::c_char,}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDebugMarkerObjectNameInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDebugMarkerObjectNameInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT;}
#[cfg(feature = "VK_EXT_debug_marker")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDebugMarkerObjectTagInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub objectType:VkDebugReportObjectTypeEXT,pub object:u64,pub tagName:u64,pub tagSize:usize,pub pTag:*const core::ffi::c_void,}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDebugMarkerObjectTagInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDebugMarkerObjectTagInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT;}
#[cfg(feature = "VK_EXT_debug_report")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDebugReportCallbackCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkDebugReportFlagsEXT,pub pfnCallback:PFN_vkDebugReportCallbackEXT,pub pUserData:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_debug_report")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDebugReportCallbackCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_debug_report")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDebugReportCallbackCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;}
#[cfg(feature = "VK_EXT_debug_utils")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDebugUtilsLabelEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pLabelName:*const core::ffi::c_char,pub pColor:[core::ffi::c_float; 4],}
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDebugUtilsLabelEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDebugUtilsLabelEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT;}
#[cfg(feature = "VK_EXT_debug_utils")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDebugUtilsMessengerCallbackDataEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkDebugUtilsMessengerCallbackDataFlagsEXT,pub pMessageIdName:*const core::ffi::c_char,pub messageIdNumber:i32,pub pMessage:*const core::ffi::c_char,pub queueLabelCount:u32,pub pQueueLabels:*const VkDebugUtilsLabelEXT,pub cmdBufLabelCount:u32,pub pCmdBufLabels:*const VkDebugUtilsLabelEXT,pub objectCount:u32,pub pObjects:*const VkDebugUtilsObjectNameInfoEXT,}
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDebugUtilsMessengerCallbackDataEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDebugUtilsMessengerCallbackDataEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT;}
#[cfg(feature = "VK_EXT_debug_utils")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDebugUtilsMessengerCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkDebugUtilsMessengerCreateFlagsEXT,pub messageSeverity:VkDebugUtilsMessageSeverityFlagsEXT,pub messageType:VkDebugUtilsMessageTypeFlagsEXT,pub pfnUserCallback:PFN_vkDebugUtilsMessengerCallbackEXT,pub pUserData:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDebugUtilsMessengerCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDebugUtilsMessengerCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;}
#[cfg(feature = "VK_EXT_debug_utils")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDebugUtilsObjectNameInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub objectType:VkObjectType,pub objectHandle:u64,pub pObjectName:*const core::ffi::c_char,}
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDebugUtilsObjectNameInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDebugUtilsObjectNameInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT;}
#[cfg(feature = "VK_EXT_debug_utils")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDebugUtilsObjectTagInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub objectType:VkObjectType,pub objectHandle:u64,pub tagName:u64,pub tagSize:u64,pub pTag:*const core::ffi::c_void,}
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDebugUtilsObjectTagInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_debug_utils")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDebugUtilsObjectTagInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkDependencyInfo=VkDependencyInfoKHR;
#[cfg(feature = "VK_KHR_synchronization2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDependencyInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub dependencyFlags:VkDependencyFlags,pub memoryBarrierCount:u32,pub pMemoryBarriers:*const VkMemoryBarrier2KHR,pub bufferMemoryBarrierCount:u32,pub pBufferMemoryBarriers:*const VkBufferMemoryBarrier2KHR,pub imageMemoryBarrierCount:u32,pub pImageMemoryBarriers:*const VkImageMemoryBarrier2KHR,}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDependencyInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDependencyInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorAddressInfoEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub address:VkDeviceAddress,pub range:VkDeviceSize,pub format:VkFormat,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkDescriptorAddressInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkDescriptorAddressInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorBufferBindingInfoEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub address:VkDeviceAddress,pub usage:VkBufferUsageFlags,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkDescriptorBufferBindingInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkDescriptorBufferBindingInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorBufferBindingPushDescriptorBufferHandleEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub buffer:VkBuffer,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkDescriptorBufferBindingPushDescriptorBufferHandleEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkDescriptorBufferBindingPushDescriptorBufferHandleEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorBufferInfo{pub buffer:VkBuffer,pub offset:VkDeviceSize,pub range:VkDeviceSize,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorGetInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub r#type:VkDescriptorType,pub data:VkDescriptorDataEXT,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDescriptorGetInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDescriptorGetInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorImageInfo{pub sampler:Option<VkSampler>,pub imageView:VkImageView,pub imageLayout:VkImageLayout,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorPoolCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkDescriptorPoolCreateFlags,pub maxSets:u32,pub poolSizeCount:u32,pub pPoolSizes:*const VkDescriptorPoolSize,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDescriptorPoolCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDescriptorPoolCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorPoolSize{pub r#type:VkDescriptorType,pub descriptorCount:u32,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorSetAllocateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub descriptorPool:VkDescriptorPool,pub descriptorSetCount:u32,pub pSetLayouts:*const VkDescriptorSetLayout,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDescriptorSetAllocateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDescriptorSetAllocateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorSetLayoutBinding{pub binding:u32,pub descriptorType:VkDescriptorType,pub descriptorCount:u32,pub stageFlags:VkShaderStageFlags,pub pImmutableSamplers:*const VkSampler,}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkDescriptorSetLayoutBindingFlagsCreateInfo=VkDescriptorSetLayoutBindingFlagsCreateInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorSetLayoutBindingFlagsCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub bindingCount:u32,pub pBindingFlags:*const VkDescriptorBindingFlagsEXT,}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDescriptorSetLayoutBindingFlagsCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDescriptorSetLayoutBindingFlagsCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorSetLayoutCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkDescriptorSetLayoutCreateFlags,pub bindingCount:u32,pub pBindings:*const VkDescriptorSetLayoutBinding,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDescriptorSetLayoutCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDescriptorSetLayoutCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkDescriptorSetLayoutSupport=VkDescriptorSetLayoutSupportKHR;
#[cfg(feature = "VK_KHR_maintenance3")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorSetLayoutSupportKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub supported:VkBool32,}
#[cfg(feature = "VK_KHR_maintenance3")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkDescriptorSetLayoutSupportKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance3")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkDescriptorSetLayoutSupportKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkDescriptorSetVariableDescriptorCountAllocateInfo=VkDescriptorSetVariableDescriptorCountAllocateInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorSetVariableDescriptorCountAllocateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub descriptorSetCount:u32,pub pDescriptorCounts:*const u32,}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDescriptorSetVariableDescriptorCountAllocateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDescriptorSetVariableDescriptorCountAllocateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkDescriptorSetVariableDescriptorCountLayoutSupport=VkDescriptorSetVariableDescriptorCountLayoutSupportEXT;
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorSetVariableDescriptorCountLayoutSupportEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxVariableDescriptorCount:u32,}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkDescriptorSetVariableDescriptorCountLayoutSupportEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkDescriptorSetVariableDescriptorCountLayoutSupportEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkDescriptorUpdateTemplateCreateInfo=VkDescriptorUpdateTemplateCreateInfoKHR;
#[cfg(feature = "VK_KHR_descriptor_update_template")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorUpdateTemplateCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkDescriptorUpdateTemplateCreateFlagsKHR,pub descriptorUpdateEntryCount:u32,pub pDescriptorUpdateEntries:*const VkDescriptorUpdateTemplateEntryKHR,pub templateType:VkDescriptorUpdateTemplateTypeKHR,pub descriptorSetLayout:VkDescriptorSetLayout,pub pipelineBindPoint:VkPipelineBindPoint,pub pipelineLayout:VkPipelineLayout,pub set:u32,}
#[cfg(feature = "VK_KHR_descriptor_update_template")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDescriptorUpdateTemplateCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_descriptor_update_template")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDescriptorUpdateTemplateCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkDescriptorUpdateTemplateEntry=VkDescriptorUpdateTemplateEntryKHR;
#[cfg(feature = "VK_KHR_descriptor_update_template")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDescriptorUpdateTemplateEntryKHR{pub dstBinding:u32,pub dstArrayElement:u32,pub descriptorCount:u32,pub descriptorType:VkDescriptorType,pub offset:usize,pub stride:usize,}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkDeviceBufferMemoryRequirements=VkDeviceBufferMemoryRequirementsKHR;
#[cfg(feature = "VK_KHR_maintenance4")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceBufferMemoryRequirementsKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pCreateInfo:*const VkBufferCreateInfo,}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceBufferMemoryRequirementsKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceBufferMemoryRequirementsKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkDeviceCreateFlags,pub queueCreateInfoCount:u32,pub pQueueCreateInfos:*const VkDeviceQueueCreateInfo,pub enabledLayerCount:u32,pub ppEnabledLayerNames:*const *const core::ffi::c_char,pub enabledExtensionCount:u32,pub ppEnabledExtensionNames:*const *const core::ffi::c_char,pub pEnabledFeatures:*const VkPhysicalDeviceFeatures,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;}
#[cfg(feature = "VK_EXT_display_control")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceEventInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub deviceEvent:VkDeviceEventTypeEXT,}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceEventInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceEventInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT;}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceGeneratedCommandsFeaturesNVX{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub computeBindingPointSupport:VkBool32,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceGeneratedCommandsFeaturesNVX{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceGeneratedCommandsFeaturesNVX{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX;}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceGeneratedCommandsLimitsNVX{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub maxIndirectCommandsLayoutTokenCount:u32,pub maxObjectEntryCounts:u32,pub minSequenceCountBufferOffsetAlignment:u32,pub minSequenceIndexBufferOffsetAlignment:u32,pub minCommandsTokenBufferOffsetAlignment:u32,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceGeneratedCommandsLimitsNVX{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceGeneratedCommandsLimitsNVX{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkDeviceGroupBindSparseInfo=VkDeviceGroupBindSparseInfoKHR;
#[cfg(feature = "VK_KHR_device_group")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceGroupBindSparseInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub resourceDeviceIndex:u32,pub memoryDeviceIndex:u32,}
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceGroupBindSparseInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceGroupBindSparseInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkDeviceGroupCommandBufferBeginInfo=VkDeviceGroupCommandBufferBeginInfoKHR;
#[cfg(feature = "VK_KHR_device_group")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceGroupCommandBufferBeginInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub deviceMask:u32,}
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceGroupCommandBufferBeginInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceGroupCommandBufferBeginInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkDeviceGroupCreateInfo=VkDeviceGroupCreateInfoKHR;
#[cfg(feature = "VK_KHR_device_group_creation")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceGroupCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub physicalDeviceCount:u32,pub pPhysicalDevices:*const VkPhysicalDevice,}
#[cfg(feature = "VK_KHR_device_group_creation")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceGroupCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_device_group_creation")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceGroupCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_GROUP_CREATE_INFO_KHR;}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceGroupPresentCapabilitiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub presentMask:[u32; VK_MAX_DEVICE_GROUP_SIZE_KHR],pub modes:VkDeviceGroupPresentModeFlagsKHR,}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface"))]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkDeviceGroupPresentCapabilitiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface"))]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkDeviceGroupPresentCapabilitiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR;}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceGroupPresentInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub swapchainCount:u32,pub pDeviceMasks:*const u32,pub mode:VkDeviceGroupPresentModeFlagBitsKHR,}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceGroupPresentInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceGroupPresentInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkDeviceGroupRenderPassBeginInfo=VkDeviceGroupRenderPassBeginInfoKHR;
#[cfg(feature = "VK_KHR_device_group")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceGroupRenderPassBeginInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub deviceMask:u32,pub deviceRenderAreaCount:u32,pub pDeviceRenderAreas:*const VkRect2D,}
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceGroupRenderPassBeginInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceGroupRenderPassBeginInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkDeviceGroupSubmitInfo=VkDeviceGroupSubmitInfoKHR;
#[cfg(feature = "VK_KHR_device_group")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceGroupSubmitInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub waitSemaphoreCount:u32,pub pWaitSemaphoreDeviceIndices:*const u32,pub commandBufferCount:u32,pub pCommandBufferDeviceMasks:*const u32,pub signalSemaphoreCount:u32,pub pSignalSemaphoreDeviceIndices:*const u32,}
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceGroupSubmitInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceGroupSubmitInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR;}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceGroupSwapchainCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub modes:VkDeviceGroupPresentModeFlagsKHR,}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceGroupSwapchainCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceGroupSwapchainCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkDeviceImageMemoryRequirements=VkDeviceImageMemoryRequirementsKHR;
#[cfg(feature = "VK_KHR_maintenance4")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceImageMemoryRequirementsKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pCreateInfo:*const VkImageCreateInfo,pub planeAspect:VkImageAspectFlagBits,}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceImageMemoryRequirementsKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceImageMemoryRequirementsKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkDeviceImageSubresourceInfo=VkDeviceImageSubresourceInfoKHR;
#[cfg(feature = "VK_KHR_maintenance5")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceImageSubresourceInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pCreateInfo:*const VkImageCreateInfo,pub pSubresource:*const VkImageSubresource2KHR,}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceImageSubresourceInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceImageSubresourceInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_IMAGE_SUBRESOURCE_INFO_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkDeviceMemoryOpaqueCaptureAddressInfo=VkDeviceMemoryOpaqueCaptureAddressInfoKHR;
#[cfg(feature = "VK_KHR_buffer_device_address")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceMemoryOpaqueCaptureAddressInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub memory:VkDeviceMemory,}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceMemoryOpaqueCaptureAddressInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceMemoryOpaqueCaptureAddressInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceQueueCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkDeviceQueueCreateFlags,pub queueFamilyIndex:u32,pub queueCount:u32,pub pQueuePriorities:*const core::ffi::c_float,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceQueueCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceQueueCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkDeviceQueueGlobalPriorityCreateInfo=VkDeviceQueueGlobalPriorityCreateInfoKHR;
#[cfg(feature = "VK_EXT_global_priority")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceQueueGlobalPriorityCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub globalPriority:VkQueueGlobalPriorityEXT,}
#[cfg(feature = "VK_EXT_global_priority")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceQueueGlobalPriorityCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_global_priority")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceQueueGlobalPriorityCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT;}
#[cfg(feature = "VK_KHR_global_priority")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub globalPriority:VkQueueGlobalPriorityKHR,}
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDeviceQueueGlobalPriorityCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDeviceQueueGlobalPriorityCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDispatchIndirectCommand{pub x:u32,pub y:u32,pub z:u32,}
#[cfg(feature = "VK_EXT_display_control")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDisplayEventInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub displayEVent:VkDisplayEventTypeEXT,}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDisplayEventInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDisplayEventInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT;}
#[cfg(feature = "VK_KHR_display")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDisplayModeCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkDisplayModeCreateFlagsKHR,pub parameters:VkDisplayModeParametersKHR,}
#[cfg(feature = "VK_KHR_display")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDisplayModeCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_display")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDisplayModeCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR;}
#[cfg(feature = "VK_KHR_display")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDisplayModeParametersKHR{pub visibleRegion:VkExtent2D,pub refreshRate:u32,}
#[cfg(feature = "VK_KHR_display")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDisplayModePropertiesKHR{pub displayMode:VkDisplayModeKHR,pub parameters:VkDisplayModeParametersKHR,}
#[cfg(feature = "VK_KHR_display")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDisplayPlaneCapabilitiesKHR{pub supportedAlpha:VkDisplayPlaneAlphaFlagsKHR,pub minSrcPosition:VkOffset2D,pub maxSrcPosition:VkOffset2D,pub minSrcExtent:VkExtent2D,pub maxSrcExtent:VkExtent2D,pub minDstPosition:VkOffset2D,pub maxDstPosition:VkOffset2D,pub minDstExtent:VkExtent2D,pub maxDstExtent:VkExtent2D,}
#[cfg(feature = "VK_KHR_display")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDisplayPlanePropertiesKHR{pub currentDisplay:VkDisplayKHR,pub currentStackIndex:u32,}
#[cfg(feature = "VK_EXT_display_control")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDisplayPowerInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub powerState:VkDisplayPowerStateEXT,}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDisplayPowerInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDisplayPowerInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT;}
#[cfg(feature = "VK_KHR_display_swapchain")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDisplayPresentInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcRect:VkRect2D,pub dstRect:VkRect2D,pub persistent:VkBool32,}
#[cfg(feature = "VK_KHR_display_swapchain")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDisplayPresentInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_display_swapchain")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDisplayPresentInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR;}
#[cfg(feature = "VK_KHR_display")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDisplayPropertiesKHR{pub display:VkDisplayKHR,pub displayName:*const core::ffi::c_char,pub physicalDimensions:VkExtent2D,pub physicalResolution:VkExtent2D,pub supportedTransforms:VkSurfaceTransformFlagsKHR,pub planeReorderPossible:VkBool32,pub persistentContent:VkBool32,}
#[cfg(feature = "VK_KHR_display")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDisplaySurfaceCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkDisplaySurfaceCreateFlagsKHR,pub displayMode:VkDisplayModeKHR,pub planeIndex:u32,pub planeStackIndex:u32,pub transform:VkSurfaceTransformFlagBitsKHR,pub globalAlpha:core::ffi::c_float,pub alphaMode:VkDisplayPlaneAlphaFlagBitsKHR,pub imageExtent:VkExtent2D,}
#[cfg(feature = "VK_KHR_display")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkDisplaySurfaceCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_display")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkDisplaySurfaceCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDrawIndexedIndirectCommand{pub indexCount:u32,pub instanceCount:u32,pub firstIndex:u32,pub vertexOffset:i32,pub firstInstance:u32,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDrawIndirectCommand{pub vertexCount:u32,pub instanceCount:u32,pub firstVertex:u32,pub firstInstance:u32,}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDrmFormatModifierPropertiesEXT{pub drmFormatModifier:u64,pub drmFormatModifierPlaneCount:u32,pub drmFormatModifierTilingFeatures:VkFormatFeatureFlags,}
#[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "VK_KHR_format_feature_flags2"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDrmFormatModifierProperties2EXT{pub drmFormatModifier:u64,pub drmFormatModifierPlaneCount:u32,pub drmFormatModifierTilingFeatures:VkFormatFeatureFlags2KHR,}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDrmFormatModifierPropertiesListEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub drmFormatModifierCount:u32,pub pDrmFormatModifierProperties:*mut VkDrmFormatModifierPropertiesEXT,}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkDrmFormatModifierPropertiesListEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkDrmFormatModifierPropertiesListEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT;}
#[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "VK_KHR_format_feature_flags2"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkDrmFormatModifierPropertiesList2EXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub drmFormatModifierCount:u32,pub pDrmFormatModifierProperties:*mut VkDrmFormatModifierProperties2EXT,}
#[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "VK_KHR_format_feature_flags2"))]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkDrmFormatModifierPropertiesList2EXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "VK_KHR_format_feature_flags2"))]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkDrmFormatModifierPropertiesList2EXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkEventCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkEventCreateFlags,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkEventCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkEventCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EVENT_CREATE_INFO;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkExportFenceCreateInfo=VkExportFenceCreateInfoKHR;
#[cfg(feature = "VK_KHR_external_fence")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportFenceCreateInfoKHR{pub handleTypes:VkExternalFenceHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_fence_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportFenceWin32HandleInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pAttributes:*const windows::Win32::Security::SECURITY_ATTRIBUTES,pub dwAccess:u32,pub name:windows::core::PCWSTR,}
#[cfg(feature = "VK_KHR_external_fence_win32")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportFenceWin32HandleInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_fence_win32")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportFenceWin32HandleInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR;}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportMemoryWin32HandleInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pAttributes:*const windows::Win32::Security::SECURITY_ATTRIBUTES,pub dwAccess:u32,pub name:windows::core::PCWSTR,}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportMemoryWin32HandleInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportMemoryWin32HandleInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportMetalBufferInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub buffer:VkBuffer,pub mtlBuffer:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportMetalBufferInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportMetalBufferInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportMetalCommandQueueInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub queue:VkQueue,pub mtlCommandQueue:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportMetalCommandQueueInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportMetalCommandQueueInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportMetalDeviceInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub mtlDevice:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportMetalDeviceInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportMetalDeviceInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportMetalIOSurfaceInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub image:VkImage,pub ioSurface:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportMetalIOSurfaceInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportMetalIOSurfaceInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportMetalObjectCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub exportObjectType:VkExportMetalObjectTypeFlagBitsEXT,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportMetalObjectCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportMetalObjectCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportMetalObjectsInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportMetalObjectsInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportMetalObjectsInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportMetalSharedEVentInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub semaphore:VkSemaphore,pub event:VkEvent,pub mtlSharedEvent:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportMetalSharedEVentInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportMetalSharedEVentInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportMetalTextureInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub image:VkImage,pub imageView:VkImageView,pub bufferView:VkBufferView,pub plane:VkImageAspectFlagBits,pub mtlTexture:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportMetalTextureInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportMetalTextureInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkExportSemaphoreCreateInfo=VkExportSemaphoreCreateInfoKHR;
#[cfg(feature = "VK_KHR_external_semaphore")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportSemaphoreCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub handleType:VkExternalSemaphoreHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_semaphore")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportSemaphoreCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_semaphore")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportSemaphoreCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR;}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExportSemaphoreWin32HandleInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pAttributes:*const windows::Win32::Security::SECURITY_ATTRIBUTES,pub dwAccess:u32,pub name:windows::core::PCWSTR,}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExportSemaphoreWin32HandleInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExportSemaphoreWin32HandleInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExtensionProperties{pub extensionName:crate::FixedCStrBuffer<VK_MAX_EXTENSION_NAME_SIZE>,pub specVersion:u32,}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[rustfmt::skip]#[repr(C)]pub struct VkExtent2D{pub width:u32,pub height:u32,}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[rustfmt::skip]#[repr(C)]pub struct VkExtent3D{pub width:u32,pub height:u32,pub depth:u32,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkExternalBufferProperties=VkExternalBufferPropertiesKHR;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExternalBufferPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub externalMemoryProperties:VkExternalMemoryPropertiesKHR,}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkExternalBufferPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkExternalBufferPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkExternalFenceProperties=VkExternalFencePropertiesKHR;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExternalFencePropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub exportFromImportedHandleTypes:VkExternalFenceHandleTypeFlagsKHR,pub compatibleHandleTypes:VkExternalFenceHandleTypeFlagsKHR,pub externalFenceFeatures:VkExternalFenceFeatureFlagsKHR,}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkExternalFencePropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkExternalFencePropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkExternalImageFormatProperties=VkExternalImageFormatPropertiesKHR;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExternalImageFormatPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub externalMemoryProperties:VkExternalMemoryPropertiesKHR,}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkExternalImageFormatPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkExternalImageFormatPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXTERANL_IMAGE_FORMAT_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkExternalMemoryBufferCreateInfo=VkExternalMemoryBufferCreateInfoKHR;
#[cfg(feature = "VK_KHR_external_memory")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExternalMemoryBufferCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub handleTypes:VkExternalMemoryHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_memory")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExternalMemoryBufferCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExternalMemoryBufferCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkExternalMemoryImageCreateInfo=VkExternalMemoryImageCreateInfoKHR;
#[cfg(feature = "VK_KHR_external_memory")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExternalMemoryImageCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub handleTypes:VkExternalMemoryHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_memory")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkExternalMemoryImageCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkExternalMemoryImageCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkExternalMemoryProperties=VkExternalMemoryPropertiesKHR;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExternalMemoryPropertiesKHR{pub externalMemoryFeatures:VkExternalMemoryFeatureFlagsKHR,pub exportFromImportedHandleTypes:VkExternalMemoryHandleTypeFlagsKHR,pub compatibleHandleTypes:VkExternalMemoryHandleTypeFlagsKHR,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkExternalSemaphoreProperties=VkExternalSemaphorePropertiesKHR;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkExternalSemaphorePropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub exportFromImportedHandleTypes:VkExternalSemaphoreHandleTypeFlagsKHR,pub compatibleHandleTypes:VkExternalSemaphoreHandleTypeFlagsKHR,pub externalSemaphoreFeatures:VkExternalSemaphoreFeatureFlagsKHR,}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkExternalSemaphorePropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkExternalSemaphorePropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkFenceCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkFenceCreateFlags,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkFenceCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkFenceCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;}
#[cfg(feature = "VK_KHR_external_fence_fd")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkFenceGetFdInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub fence:VkFence,pub handleType:VkExternalFenceHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_fence_fd")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkFenceGetFdInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_fence_fd")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkFenceGetFdInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR;}
#[cfg(feature = "VK_KHR_external_fence_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkFenceGetWin32HandleInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub fence:VkFence,pub handleType:VkExternalFenceHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_fence_win32")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkFenceGetWin32HandleInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_fence_win32")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkFenceGetWin32HandleInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkFormatProperties{pub linearTilingFeatures:VkFormatFeatureFlags,pub optimalTilingFeatures:VkFormatFeatureFlags,pub bufferFeatures:VkFormatFeatureFlags,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkFormatProperties2=VkFormatProperties2KHR;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkFormatProperties2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub formatProperties:VkFormatProperties,}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkFormatProperties2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkFormatProperties2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkFormatProperties3=VkFormatProperties3KHR;
#[cfg(feature = "VK_KHR_format_feature_flags2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkFormatProperties3KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub linearTilingFeatures:VkFormatFeatureFlags2KHR,pub optimalTilingFeatures:VkFormatFeatureFlags2KHR,pub bufferFeatures:VkFormatFeatureFlags2KHR,}
#[cfg(feature = "VK_KHR_format_feature_flags2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkFormatProperties3KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_format_feature_flags2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkFormatProperties3KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkFramebufferCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkFramebufferCreateFlags,pub renderPass:VkRenderPass,pub attachmentCount:u32,pub pAttachments:*const VkImageView,pub width:u32,pub height:u32,pub layers:u32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkFramebufferCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkFramebufferCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkGraphicsPipelineCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineCreateFlags,pub stageCount:u32,pub pStages:*const VkPipelineShaderStageCreateInfo,pub pVertexInputState:*const VkPipelineVertexInputStateCreateInfo,pub pInputAssemblyState:*const VkPipelineInputAssemblyStateCreateInfo,pub pTessellationState:*const VkPipelineTessellationStateCreateInfo,pub pViewportState:*const VkPipelineViewportStateCreateInfo,pub pRasterizationState:*const VkPipelineRasterizationStateCreateInfo,pub pMultisampleState:*const VkPipelineMultisampleStateCreateInfo,pub pDepthStencilState:*const VkPipelineDepthStencilStateCreateInfo,pub pColorBlendState:*const VkPipelineColorBlendStateCreateInfo,pub pDynamicState:*const VkPipelineDynamicStateCreateInfo,pub layout:Option<VkPipelineLayout>,pub renderPass:Option<VkRenderPass>,pub subpass:u32,pub basePipelineHandle:Option<VkPipeline>,pub basePipelineIndex:i32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkGraphicsPipelineCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkGraphicsPipelineCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;}
#[cfg(feature = "VK_EXT_hdr_metadata")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkHdrMetadataEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub displayPrimaryRed:VkXYColorEXT,pub displayPrimaryGreen:VkXYColorEXT,pub displayPrimaryBlue:VkXYColorEXT,pub whitePoint:VkXYColorEXT,pub maxLuminance:core::ffi::c_float,pub minLuminance:core::ffi::c_float,pub maxContentLightLevel:core::ffi::c_float,pub maxFrameAverageLightLevel:core::ffi::c_float,}
#[cfg(feature = "VK_EXT_hdr_metadata")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkHdrMetadataEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_hdr_metadata")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkHdrMetadataEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_HDR_METADATA_EXT;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkHostImageCopyDevicePerformanceQuery=VkHostImageCopyDevicePerformanceQueryEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkHostImageCopyDevicePerformanceQueryEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub optimalDeviceAccess:VkBool32,pub identicalMemoryLayout:VkBool32,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkHostImageCopyDevicePerformanceQueryEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkHostImageCopyDevicePerformanceQueryEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkHostImageLayoutTransitionInfo=VkHostImageLayoutTransitionInfoEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkHostImageLayoutTransitionInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub image:VkImage,pub oldLayout:VkImageLayout,pub newLayout:VkImageLayout,pub subresourceRange:VkImageSubresourceRange,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkHostImageLayoutTransitionInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkHostImageLayoutTransitionInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_HOST_IMAGE_LAYOUT_TRANSITION_EXT;}
#[cfg(feature = "VK_MVK_ios_surface")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkIOSSurfaceCreateInfoMVK{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkIOSSurfaceCreateFlagsMVK,pub pView:*const core::ffi::c_void,}
#[cfg(feature = "VK_MVK_ios_surface")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkIOSSurfaceCreateInfoMVK{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_MVK_ios_surface")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkIOSSurfaceCreateInfoMVK{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageBlit{pub srcSubresource:VkImageSubresourceLayers,pub srcOffsets:[VkOffset3D; 2],pub dstSubresource:VkImageSubresourceLayers,pub dstOffsets:[VkOffset3D; 2],}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkImageBlit2=VkImageBlit2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageBlit2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcSubresources:VkImageSubresourceLayers,pub srcOffset:[VkOffset3D; 2],pub dstSubresources:VkImageSubresourceLayers,pub dstOffset:[VkOffset3D; 2],}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageBlit2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageBlit2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_BLIT_2_KHR;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageCaptureDescriptorDataInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub image:VkImage,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageCaptureDescriptorDataInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageCaptureDescriptorDataInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageCopy{pub srcSubresource:VkImageSubresourceLayers,pub srcOffset:VkOffset3D,pub dstSubresource:VkImageSubresourceLayers,pub dstOffset:VkOffset3D,pub extent:VkExtent3D,}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkImageCopy2=VkImageCopy2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageCopy2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcSubresource:VkImageSubresourceLayers,pub srcOffset:VkOffset3D,pub dstSubresource:VkImageSubresourceLayers,pub dstOffset:VkOffset3D,pub extent:VkExtent3D,}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageCopy2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageCopy2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_COPY_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkImageCreateFlags,pub imageType:VkImageType,pub format:VkFormat,pub extent:VkExtent3D,pub mipLevels:u32,pub arrayLayers:u32,pub samples:VkSampleCountFlagBits,pub tiling:VkImageTiling,pub usage:VkImageUsageFlags,pub sharingMode:VkSharingMode,pub queueFamilyIndexCount:u32,pub pQueueFamilyIndices:*const u32,pub initialLayout:VkImageLayout,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub drmFormatModifier:u64,pub drmFormatModifierPlaneCount:u32,pub pPlaneLayouts:*const VkSubresourceLayout,}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageDrmFormatModifierExplicitCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageDrmFormatModifierExplicitCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT;}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageDrmFormatModifierListCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub drmFormatModifierCount:u32,pub pDrmFormatModifiers:*const u64,}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageDrmFormatModifierListCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageDrmFormatModifierListCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT;}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageDrmFormatModifierPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub drmFormatModifier:u64,}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkImageDrmFormatModifierPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkImageDrmFormatModifierPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkImageFormatListCreateInfo=VkImageFormatListCreateInfoKHR;
#[cfg(feature = "VK_KHR_image_format_list")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageFormatListCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub viewFormatCount:u32,pub pViewFormats:*const VkFormat,}
#[cfg(feature = "VK_KHR_image_format_list")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageFormatListCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_image_format_list")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageFormatListCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageFormatProperties{pub maxExtent:VkExtent3D,pub maxMipLevels:u32,pub maxArrayLayers:u32,pub sampleCounts:VkSampleCountFlags,pub maxResourceSize:VkDeviceSize,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkImageFormatProperties2=VkImageFormatProperties2KHR;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageFormatProperties2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub imageFormatProperties:VkImageFormatProperties,}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkImageFormatProperties2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkImageFormatProperties2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageMemoryBarrier{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcAccessMask:VkAccessFlags,pub dstAccessMask:VkAccessFlags,pub oldLayout:VkImageLayout,pub newLayout:VkImageLayout,pub srcQueueFamilyIndex:u32,pub dstQueueFamilyIndex:u32,pub image:VkImage,pub subresourceRange:VkImageSubresourceRange,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageMemoryBarrier{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageMemoryBarrier{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkImageMemoryBarrier2=VkImageMemoryBarrier2KHR;
#[cfg(feature = "VK_KHR_synchronization2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageMemoryBarrier2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcStageMask:VkPipelineStageFlags2KHR,pub srcAccessMask:VkAccessFlags2KHR,pub dstStageMask:VkPipelineStageFlags2KHR,pub dstAccessMask:VkAccessFlags2KHR,pub oldLayout:VkImageLayout,pub newLayout:VkImageLayout,pub srcQueueFamilyIndex:u32,pub dstQueueFamilyIndex:u32,pub image:VkImage,pub subresourceRange:VkImageSubresourceRange,}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageMemoryBarrier2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageMemoryBarrier2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkImageMemoryRequirementsInfo2=VkImageMemoryRequirementsInfo2KHR;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageMemoryRequirementsInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub image:VkImage,}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageMemoryRequirementsInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageMemoryRequirementsInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkImagePlaneMemoryRequirementsInfo=VkImagePlaneMemoryRequirementsInfoKHR;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImagePlaneMemoryRequirementsInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub planeAspect:VkImageAspectFlagBits,}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImagePlaneMemoryRequirementsInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImagePlaneMemoryRequirementsInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageResolve{pub srcSubresource:VkImageSubresourceLayers,pub srcOffset:VkOffset3D,pub dstSubresource:VkImageSubresourceLayers,pub dstOffset:VkOffset3D,pub extent:VkExtent3D,}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkImageResolve2=VkImageResolve2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageResolve2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcSubresource:VkImageSubresourceLayers,pub srcOffset:VkOffset3D,pub dstSubresource:VkImageSubresourceLayers,pub dstOffset:VkOffset3D,pub extent:VkExtent3D,}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageResolve2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageResolve2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkImageSparseMemoryRequirementsInfo2=VkImageSparseMemoryRequirementsInfo2KHR;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageSparseMemoryRequirementsInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub image:VkImage,}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageSparseMemoryRequirementsInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageSparseMemoryRequirementsInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageSubresource{pub aspectMask:VkImageAspectFlags,pub mipLevel:u32,pub arrayLayer:u32,}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkImageSubresource2=VkImageSubresource2KHR;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageSubresource2EXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub imageSubresource:VkImageSubresource,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageSubresource2EXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkImageSubresource2EXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageSubresource2EXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT;}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkImageSubresource2EXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_EXT;}
#[cfg(feature = "VK_KHR_maintenance5")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageSubresource2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub imageSubresource:VkImageSubresource,}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageSubresource2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageSubresource2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageSubresourceLayers{pub aspectMask:VkImageAspectFlags,pub mipLevel:u32,pub baseArrayLayer:u32,pub layerCount:u32,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageSubresourceRange{pub aspectMask:VkImageAspectFlags,pub baseMipLevel:u32,pub levelCount:u32,pub baseArrayLayer:u32,pub layerCount:u32,}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageSwapchainCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub swapchain:VkSwapchainKHR,}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageSwapchainCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageSwapchainCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkImageToMemoryCopy=VkImageToMemoryCopyEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageToMemoryCopyEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pHostPointer:*mut core::ffi::c_void,pub memoryRowLength:u32,pub memoryImageHeight:u32,pub imageSubresource:VkImageSubresourceLayers,pub imageOffset:VkOffset3D,pub imageExtent:VkExtent3D,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageToMemoryCopyEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageToMemoryCopyEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_TO_MEMORY_COPY_EXT;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageViewCaptureDescriptorDataInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub imageView:VkImageView,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageViewCaptureDescriptorDataInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageViewCaptureDescriptorDataInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageViewCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkImageViewCreateFlags,pub image:VkImage,pub viewType:VkImageViewType,pub format:VkFormat,pub components:VkComponentMapping,pub subresourceRange:VkImageSubresourceRange,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageViewCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageViewCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkImageViewUsageCreateInfo=VkImageViewUsageCreateInfoKHR;
#[cfg(feature = "VK_KHR_maintenance2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImageViewUsageCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub sliceOffset:u32,pub sliceCount:u32,}
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImageViewUsageCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImageViewUsageCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR;}
#[cfg(feature = "VK_KHR_external_fence_fd")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportFenceFdInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub fence:VkFence,pub flags:VkFenceImportFlagsKHR,pub handleType:VkExternalFenceHandleTypeFlagsKHR,pub fd:core::ffi::c_int,}
#[cfg(feature = "VK_KHR_external_fence_fd")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImportFenceFdInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_fence_fd")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImportFenceFdInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR;}
#[cfg(feature = "VK_KHR_external_fence_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportFenceWin32HandleInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub fence:VkFence,pub flags:VkFenceImportFlagsKHR,pub handleType:VkExternalFenceHandleTypeFlagsKHR,pub handle:windows::Win32::Foundation::HANDLE,pub name:windows::core::PCWSTR,}
#[cfg(feature = "VK_KHR_external_fence_win32")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImportFenceWin32HandleInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_fence_win32")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImportFenceWin32HandleInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR;}
#[cfg(feature = "VK_KHR_external_memory_fd")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportMemoryFdInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub handleType:VkExternalMemoryHandleTypeFlagsKHR,pub fd:core::ffi::c_int,}
#[cfg(feature = "VK_KHR_external_memory_fd")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImportMemoryFdInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_fd")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImportMemoryFdInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR;}
#[cfg(feature = "VK_EXT_external_memory_host")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportMemoryHostPointerInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub handleType:VkExternalMemoryHandleTypeFlagsKHR,pub pHostPointer:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImportMemoryHostPointerInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImportMemoryHostPointerInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT;}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportMemoryWin32HandleInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub handleType:VkExternalMemoryHandleTypeFlagsKHR,pub handle:windows::Win32::Foundation::HANDLE,pub name:windows::core::PCWSTR,}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImportMemoryWin32HandleInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImportMemoryWin32HandleInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportMetalBufferInfoEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub mtlBuffer:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkImportMetalBufferInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkImportMetalBufferInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportMetalIOSurfaceInfoEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub ioSurface:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkImportMetalIOSurfaceInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkImportMetalIOSurfaceInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportMetalSharedEventInfoEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub mtlSharedEvent:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkImportMetalSharedEventInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkImportMetalSharedEventInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT;}
#[cfg(feature = "VK_EXT_metal_objects")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportMetalTextureInfoEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub plane:VkImageAspectFlagBits,pub mtlTexture:*mut core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkImportMetalTextureInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkImportMetalTextureInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT;}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportSemaphoreFdInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub semaphore:VkSemaphore,pub flags:VkSemaphoreImportFlagsKHR,pub handleType:VkExternalSemaphoreHandleTypeFlagsKHR,pub fd:core::ffi::c_int,}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImportSemaphoreFdInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImportSemaphoreFdInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR;}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkImportSemaphoreWin32HandleInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub semaphore:VkSemaphore,pub flags:VkSemaphoreImportFlagsKHR,pub handleType:VkExternalSemaphoreHandleTypeFlagsKHR,pub handle:windows::Win32::Foundation::HANDLE,pub name:windows::core::PCWSTR,}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkImportSemaphoreWin32HandleInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkImportSemaphoreWin32HandleInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR;}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkIndirectCommandsLayoutCreateInfoNVX{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pipelineBindPoint:VkPipelineBindPoint,pub flags:VkIndirectCommandsLayoutUsageFlagsNVX,pub tokenCount:u32,pub pTokens:*const VkIndirectCommandsLayoutTokenNVX,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkIndirectCommandsLayoutCreateInfoNVX{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkIndirectCommandsLayoutCreateInfoNVX{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX;}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkIndirectCommandsLayoutTokenNVX{pub tokenType:VkIndirectCommandsTokenTypeNVX,pub bindingUnit:u32,pub dynamicCount:u32,pub divisor:u32,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkIndirectCommandsTokenNVX{pub tokenType:VkIndirectCommandsTokenTypeNVX,pub buffer:VkBuffer,pub offset:VkDeviceSize,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkInputAttachmentAspectReference=VkInputAttachmentAspectReferenceKHR;
#[cfg(feature = "VK_KHR_maintenance2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkInputAttachmentAspectReferenceKHR{pub subpass:u32,pub inputAttachmentIndex:u32,pub aspectMask:VkImageAspectFlags,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkInstanceCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkInstanceCreateFlags,pub pApplicationInfo:*const VkApplicationInfo,pub enabledLayerCount:u32,pub ppEnabledLayerNames:*const *const core::ffi::c_char,pub enabledExtensionCount:u32,pub ppEnabledExtensionNames:*const *const core::ffi::c_char,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkInstanceCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkInstanceCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkLayerProperties{pub layerName:crate::FixedCStrBuffer<VK_MAX_EXTENSION_NAME_SIZE>,pub specVersion:u32,pub implementationVersion:u32,pub description:crate::FixedCStrBuffer<VK_MAX_DESCRIPTION_SIZE>,}
#[cfg(feature = "VK_EXT_layer_settings")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkLayerSettingEXT{pub pLayerName:*const core::ffi::c_char,pub pSettingName:*const core::ffi::c_char,pub r#type:VkLayerSettingTypeEXT,pub valueCount:u32,pub pValues:*const core::ffi::c_void,}
#[cfg(feature = "VK_EXT_layer_settings")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkLayerSettingsCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub settingCount:u32,pub pSettings:*const VkLayerSettingEXT,}
#[cfg(feature = "VK_EXT_layer_settings")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkLayerSettingsCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_layer_settings")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkLayerSettingsCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_LAYER_SETTINGS_CREATE_INFO_EXT;}
#[cfg(feature = "VK_MVK_macos_surface")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMacOSSurfaceCreateInfoMVK{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkMacOSSurfaceCreateFlagsMVK,pub pView:*const core::ffi::c_void,}
#[cfg(feature = "VK_MVK_macos_surface")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMacOSSurfaceCreateInfoMVK{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_MVK_macos_surface")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMacOSSurfaceCreateInfoMVK{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMappedMemoryRange{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub memory:VkDeviceMemory,pub offset:VkDeviceSize,pub size:VkDeviceSize,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMappedMemoryRange{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMappedMemoryRange{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkMemoryAllocateFlagsInfo=VkMemoryAllocateFlagsInfoKHR;
#[cfg(feature = "VK_KHR_device_group")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryAllocateFlagsInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkMemoryAllocateFlags,pub deviceMask:u32,}
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryAllocateFlagsInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_device_group")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryAllocateFlagsInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryAllocateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub allocationSize:VkDeviceSize,pub memoryTypeIndex:u32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryAllocateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryAllocateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryBarrier{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcAccessMask:VkAccessFlags,pub dstAccessMask:VkAccessFlags,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryBarrier{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryBarrier{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_BARRIER;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkMemoryBarrier2=VkMemoryBarrier2KHR;
#[cfg(feature = "VK_KHR_synchronization2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryBarrier2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcStageMask:VkPipelineStageFlags2KHR,pub srcAccessMask:VkAccessFlags2KHR,pub dstStageMask:VkPipelineStageFlags2KHR,pub dstAccessMask:VkAccessFlags2KHR,}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryBarrier2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryBarrier2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR;}
#[cfg(feature = "VK_KHR_maintenance8")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryBarrierAccessFlags3KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcAccessMask3:VkAccessFlags3KHR,pub dstAccessMask3:VkAccessFlags3KHR,}
#[cfg(feature = "VK_KHR_maintenance8")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryBarrierAccessFlags3KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance8")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryBarrierAccessFlags3KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_BARRIER_ACCESS_FLAGS_3_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkMemoryDedicatedAllocateInfo=VkMemoryDedicatedAllocateInfoKHR;
#[cfg(feature = "VK_KHR_dedicated_allocation")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryDedicatedAllocateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub image:Option<VkImage>,pub buffer:Option<VkBuffer>,}
#[cfg(feature = "VK_KHR_dedicated_allocation")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryDedicatedAllocateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_dedicated_allocation")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryDedicatedAllocateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkMemoryDedicatedRequirements=VkMemoryDedicatedRequirementsKHR;
#[cfg(feature = "VK_KHR_dedicated_allocation")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryDedicatedRequirementsKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub prefersDedicatedAllocation:VkBool32,pub requiresDedicatedAllocation:VkBool32,}
#[cfg(feature = "VK_KHR_dedicated_allocation")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkMemoryDedicatedRequirementsKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_dedicated_allocation")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkMemoryDedicatedRequirementsKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR;}
#[cfg(feature = "VK_KHR_external_memory_fd")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryFdPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub memoryTypeBits:u32,}
#[cfg(feature = "VK_KHR_external_memory_fd")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkMemoryFdPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_fd")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkMemoryFdPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR;}
#[cfg(feature = "VK_KHR_external_memory_fd")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryGetFdInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub memory:VkDeviceMemory,pub handleType:VkExternalMemoryHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_memory_fd")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryGetFdInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_fd")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryGetFdInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR;}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryGetWin32HandleInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub memory:VkDeviceMemory,pub handleType:VkExternalMemoryHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryGetWin32HandleInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryGetWin32HandleInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryHeap{pub size:VkDeviceSize,pub flags:VkMemoryHeapFlags,}
#[cfg(feature = "VK_EXT_external_memory_host")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryHostPointerPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub memoryTypeBits:u32,}
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkMemoryHostPointerPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkMemoryHostPointerPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkMemoryMapInfo=VkMemoryMapInfoKHR;
#[cfg(feature = "VK_KHR_map_memory2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryMapInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkMemoryUnmapFlagsKHR,pub memory:VkDeviceMemory,}
#[cfg(feature = "VK_KHR_map_memory2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryMapInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_map_memory2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryMapInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_MAP_INFO_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkMemoryOpaqueCaptureAddressAllocateInfo=VkMemoryOpaqueCaptureAddressAllocateInfoKHR;
#[cfg(feature = "VK_KHR_buffer_device_address")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryOpaqueCaptureAddressAllocateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub opaqueCaptureAddress:u64,}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryOpaqueCaptureAddressAllocateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryOpaqueCaptureAddressAllocateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryRequirements{pub size:VkDeviceSize,pub alignment:VkDeviceSize,pub memoryTypeBits:u32,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkMemoryRequirements2=VkMemoryRequirements2KHR;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryRequirements2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub memoryRequirements:VkMemoryRequirements,}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkMemoryRequirements2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkMemoryRequirements2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkMemoryToImageCopy=VkMemoryToImageCopyEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryToImageCopyEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub pHostPointer:*const core::ffi::c_void,pub memoryRowLength:u32,pub memoryImageHeight:u32,pub imageSubresource:VkImageSubresourceLayers,pub imageOffset:VkOffset3D,pub imageExtent:VkExtent3D,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryToImageCopyEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryToImageCopyEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_TO_IMAGE_COPY_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryType{pub propertyFlags:VkMemoryPropertyFlags,pub heapIndex:u32,}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkMemoryUnmapInfo=VkMemoryUnmapInfoKHR;
#[cfg(feature = "VK_KHR_map_memory2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryUnmapInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkMemoryUnmapFlagsKHR,pub memory:VkDeviceMemory,}
#[cfg(feature = "VK_KHR_map_memory2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMemoryUnmapInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_map_memory2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMemoryUnmapInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO_KHR;}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMemoryWin32HandlePropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub memoryTypeBits:u32,}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkMemoryWin32HandlePropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_win32")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkMemoryWin32HandlePropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR;}
#[cfg(feature = "VK_EXT_metal_surface")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMetalSurfaceCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkMetalSurfaceCreateFlagsEXT,pub pLayer:*const core::ffi::c_void,}
#[cfg(feature = "VK_EXT_metal_surface")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkMetalSurfaceCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_metal_surface")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkMetalSurfaceCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT;}
#[cfg(feature = "VK_EXT_sample_locations")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkMultisamplePropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxSampleLocationGridSize:VkExtent2D,}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkMultisamplePropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkMultisamplePropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT;}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkObjectTableCreateInfoNVX{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub objectCount:u32,pub pObjectEntryTypes:*const VkObjectEntryTypeNVX,pub pObjectEntryCounts:*const u32,pub pObjectEntryUsageFlags:*const VkObjectEntryUsageFlagsNVX,pub maxUniformBuffersPerDescriptor:u32,pub maxStorageBuffersPerDescriptor:u32,pub maxStorageImagesPerDescriptor:u32,pub maxSampledImagesPerDescriptor:u32,pub maxPipelineLayouts:u32,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkObjectTableCreateInfoNVX{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkObjectTableCreateInfoNVX{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX;}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkObjectTableDescriptorSEtEntryNVX{pub r#type:VkObjectEntryTypeNVX,pub flags:VkObjectEntryUsageFlagsNVX,pub pipelineLayout:VkPipelineLayout,pub descriptorSet:VkDescriptorSet,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkObjectTableEntryNVX{pub r#type:VkObjectEntryTypeNVX,pub flags:VkObjectEntryUsageFlagsNVX,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkObjectTableIndexBufferEntryNVX{pub r#type:VkObjectEntryTypeNVX,pub flags:VkObjectEntryUsageFlagsNVX,pub buffer:VkBuffer,pub indexType:VkIndexType,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkObjectTablePipelineEntryNVX{pub r#type:VkObjectEntryTypeNVX,pub flags:VkObjectEntryUsageFlagsNVX,pub pipeline:VkPipeline,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkObjectTablePushConstantEntryNVX{pub r#type:VkObjectEntryTypeNVX,pub flags:VkObjectEntryUsageFlagsNVX,pub pipelineLayout:VkPipelineLayout,pub stageFlags:VkShaderStageFlags,}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkObjectTableVertexBufferEntryNVX{pub r#type:VkObjectEntryTypeNVX,pub flags:VkObjectEntryUsageFlagsNVX,pub buffer:VkBuffer,}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[rustfmt::skip]#[repr(C)]pub struct VkOffset2D{pub x:i32,pub y:i32,}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[rustfmt::skip]#[repr(C)]pub struct VkOffset3D{pub x:i32,pub y:i32,pub z:i32,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkOpaqueCaptureDescriptorDataCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub opaqueCaptureDescriptorData:*const core::ffi::c_void,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkOpaqueCaptureDescriptorDataCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkOpaqueCaptureDescriptorDataCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT;}
#[cfg(feature = "VK_GOOGLE_display_timing")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPastPresentationTimingGOOGLE{pub presentID:u32,pub desiredPresentTime:u64,pub actualPresentTime:u64,pub earliestPresentTime:u64,pub presentMargin:u64,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDevice16BitStorageFeatures=VkPhysicalDevice16BitStorageFeaturesKHR;
#[cfg(feature = "VK_KHR_16bit_storage")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDevice16BitStorageFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub storageBuffer16BitAccess:VkBool32,pub uniformAndStorageBuffer16BitAccess:VkBool32,pub storagePushConstant16:VkBool32,pub storageInputOutput16:VkBool32,}
#[cfg(feature = "VK_KHR_16bit_storage")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDevice16BitStorageFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_16bit_storage")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDevice16BitStorageFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_16bit_storage")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDevice16BitStorageFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_16bit_storage")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDevice16BitStorageFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR;}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceBlendOperationAdvanccedFeaturesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub advancedBlendCoherentOperations:VkBool32,}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceBlendOperationAdvanccedFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceBlendOperationAdvanccedFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceBlendOperationAdvanccedFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT;}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceBlendOperationAdvanccedFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT;}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub advancedBlendMaxColorAttachments:u32,pub advancedBlendIndependentBlend:VkBool32,pub advancedBlendNonPremultipliedSrcColor:VkBool32,pub advancedBlendNonPremultipliedDstColor:VkBool32,pub advancedBlendCorrelatedOverlap:VkBool32,pub advancedBlendAllOperations:VkBool32,}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceBufferDeviceAddressFeatures=VkPhysicalDeviceBufferDeviceAddressFeaturesKHR;
#[cfg(feature = "VK_KHR_buffer_device_address")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub bufferDeviceAddress:VkBool32,pub bufferDeviceAddressCaptureReplay:VkBool32,pub bufferDeviceAddressMultiDevice:VkBool32,}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceBufferDeviceAddressFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceBufferDeviceAddressFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceBufferDeviceAddressFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceBufferDeviceAddressFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR;}
#[cfg(feature = "VK_EXT_conservative_rasterization")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub primitiveOverestimationSize:core::ffi::c_float,pub maxExtraPrimitiveOverestimationSize:core::ffi::c_float,pub extraPrimitiveOverestimationSizeGranularity:core::ffi::c_float,pub primitiveUnderestimation:VkBool32,pub conservativePointAndLineRasterization:VkBool32,pub degenerateTrianglesRasterized:VkBool32,pub degenerateLinesRasterized:VkBool32,pub fullyCoveredFragmentShaderInputVariable:VkBool32,pub conservativeRasterizationPostDepthCoverage:VkBool32,}
#[cfg(feature = "VK_EXT_conservative_rasterization")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceConservativeRasterizationPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_conservative_rasterization")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceConservativeRasterizationPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceDepthStencilResolveProperties=VkPhysicalDeviceDepthStencilResolvePropertiesKHR;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceDepthStencilResolvePropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub supportedDepthREsolveModes:VkResolveModeFlagsKHR,pub supportedStencilResolveModes:VkResolveModeFlagsKHR,pub independentResolveNone:VkBool32,pub independentREsolve:VkBool32,}
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDepthStencilResolvePropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDepthStencilResolvePropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceDescriptorBuferDensityMapPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub combinedImageSamplerDensityMapDescriptorSize:usize,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDescriptorBuferDensityMapPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDescriptorBuferDensityMapPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceDescriptorBufferFeaturesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub descriptorBuffer:VkBool32,pub descriptorBufferCaptureReplay:VkBool32,pub descriptorBufferImageLayoutIgnored:VkBool32,pub descriptorBufferPushDescriptors:VkBool32,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceDescriptorBufferFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDescriptorBufferFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceDescriptorBufferFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDescriptorBufferFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceDescriptorBufferPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub combinedImageSamplerDescriptorSingleArray:VkBool32,pub bufferlessPushDescriptors:VkBool32,pub allowSamplerImageViewPostSubmitCreation:VkBool32,pub descriptorBufferOffsetAlignment:VkDeviceSize,pub maxDescriptorBufferBindings:u32,pub maxResourceDescriptorBufferBindings:u32,pub maxSamplerDescriptorBufferBindings:u32,pub maxEmbeddedImmutableSamplerBindings:u32,pub maxEmbeddedImmutableSamplers:u32,pub bufferCaptureReplayDescriptorDataSize:usize,pub imageCaptureReplayDescriptorDataSize:usize,pub imageViewCaptureReplayDescriptorDataSize:usize,pub samplerCaptureReplayDescriptorDataSize:usize,pub accelerationStructureCaptureReplayDescriptorDataSize:usize,pub samplerDescriptorSize:usize,pub combinedImageSamplerDescriptorSize:usize,pub sampledImageDescriptorSize:usize,pub storageImageDescriptorSize:usize,pub uniformTexelBufferDescriptorSize:usize,pub robustUniformTexelBufferDescriptorSize:usize,pub storageTexelBufferDescriptorSize:usize,pub robustStorageTexelBufferDescriptorSize:usize,pub uniformBufferDescriptorSize:usize,pub robustUniformBufferDescriptorSize:usize,pub storageBufferDescriptorSize:usize,pub robustStorageBufferDescriptorSize:usize,pub inputAttachmentDescriptorSize:usize,pub accelerationStructureDescriptorSize:usize,pub maxSamplerDescriptorBufferRange:VkDeviceSize,pub maxResourceDescriptorBufferRange:VkDeviceSize,pub samplerDescriptorBufferAddressSpaceSize:VkDeviceSize,pub resourceDescriptorBufferAddressSpaceSize:VkDeviceSize,pub descriptorBufferAddressSpaceSize:VkDeviceSize,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDescriptorBufferPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDescriptorBufferPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceDescriptorIndexingFeatures=VkPhysicalDeviceDescriptorIndexingFeaturesEXT;
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceDescriptorIndexingFeaturesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub shaderInputAttachmentArrayDynamicIndexing:VkBool32,pub shaderUniformTexelBufferArrayDynamicIndexing:VkBool32,pub shaderStorageTexelBufferArrayDynamicIndexing:VkBool32,pub shaderUniformBufferArrayNonUniformIndexing:VkBool32,pub shaderSampledImageArrayNonUniformIndexing:VkBool32,pub shaderStorageBufferArrayNonUniformIndexing:VkBool32,pub shaderStorageImageArrayNonUniformIndexing:VkBool32,pub shaderInputAttachmentArrayNonUniformIndexing:VkBool32,pub shaderUniformTexelBufferArrayNonUniformIndexing:VkBool32,pub shaderStorageTexelBufferArrayNonUniformIndexing:VkBool32,pub descriptorBindingUniformBufferUpdateAfterBind:VkBool32,pub descriptorBindingSampledImageUpdateAfterBind:VkBool32,pub descriptorBindingStorageImageUpdateAfterBind:VkBool32,pub descriptorBindingStorageBufferUpdateAfterBind:VkBool32,pub descriptorBindingUniformTexelBufferUpdateAfterBind:VkBool32,pub descriptorBindingStorageTexelBufferUpdateAfterBind:VkBool32,pub descriptorBindingUpdateUnusedWhilePending:VkBool32,pub descriptorBindingPartiallyBound:VkBool32,pub descriptorBindingVariableDescriptorCount:VkBool32,pub runtimeDescriptorArray:VkBool32,}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceDescriptorIndexingFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDescriptorIndexingFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceDescriptorIndexingFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT;}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDescriptorIndexingFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceDescriptorIndexingProperties=VkPhysicalDeviceDescriptorIndexingPropertiesEXT;
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceDescriptorIndexingPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxUpdateAfterBindDescriptorsInAllPools:u32,pub shaderUniformBufferArrayNonUniformIndexingNative:VkBool32,pub shaderSampledImageArrayNonUniformIndexingNative:VkBool32,pub shaderStorageBufferArrayNonUniformIndexingNative:VkBool32,pub shaderStorageImageArrayNonUniformIndexingNative:VkBool32,pub shaderInputAttachmentArrayNonUniformIndexingNative:VkBool32,pub robustBufferAccessUpdateAfterBind:VkBool32,pub quadDivergentImplicitLod:VkBool32,pub maxPerStageDescriptorUpdateAfterBindSamplers:u32,pub maxPerStageDescriptorUpdateAfterBindUniformBuffers:u32,pub maxPerStageDescriptorUpdateAfterBindStorageBuffers:u32,pub maxPerStageDescriptorUpdateAfterBindSampledImages:u32,pub maxPerStageDescriptorUpdateAfterBindStorageImages:u32,pub maxPerStageDescriptorUpdateAfterBindInputAttachments:u32,pub maxPerStageUpdateAfterBindResources:u32,pub maxDescriptorSetUpdateAfterBindSamplers:u32,pub maxDescriptorSetUpdateAfterBindUniformBuffers:u32,pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic:u32,pub maxDescriptorSetUpdateAfterBindStorageBuffers:u32,pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic:u32,pub maxDescriptorSetUpdateAfterBindSampledImages:u32,pub maxDescriptorSetUpdateAfterBindStorageImages:u32,pub maxDescriptorSetUpdateAfterBindInputAttachments:u32,}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDescriptorIndexingPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_indexing")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDescriptorIndexingPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT;}
#[cfg(feature = "VK_EXT_discard_rectangles")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxDiscardRectangles:u32,}
#[cfg(feature = "VK_EXT_discard_rectangles")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDiscardRectanglePropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_discard_rectangles")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDiscardRectanglePropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceDynamicRenderingFeatures=VkPhysicalDeviceDynamicRenderingFeaturesKHR;
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceDynamicRenderingFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub dynamicRendering:VkBool32,}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceDynamicRenderingFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDynamicRenderingFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceDynamicRenderingFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDynamicRenderingFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceDynamicRenderingLocalReadFeatures=VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR;
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub dynamicRenderingLocalRead:VkBool32,}
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceExternalBufferInfo=VkPhysicalDeviceExternalBufferInfoKHR;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceExternalBufferInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkBufferCreateFlags,pub usage:VkBufferUsageFlags,pub handleType:VkExternalMemoryHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceExternalBufferInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceExternalBufferInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceExternalFenceInfo=VkPhysicalDeviceExternalFenceInfoKHR;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceExternalFenceInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub handleType:VkExternalFenceHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceExternalFenceInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceExternalFenceInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceExternalImageFormatInfo=VkPhysicalDeviceExternalImageFormatInfoKHR;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceExternalImageFormatInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub handleType:VkExternalMemoryHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceExternalImageFormatInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceExternalImageFormatInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR;}
#[cfg(feature = "VK_EXT_external_memory_host")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub minImportedHostPointerAlignment:VkDeviceSize,}
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceExternalMemoryHostPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceExternalMemoryHostPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceExternalSemaphoreInfo=VkPhysicalDeviceExternalSemaphoreInfoKHR;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub handleType:VkExternalSemaphoreHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceExternalSemaphoreInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceExternalSemaphoreInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceFeatures{pub robustBufferAccess:VkBool32,pub fullDrawIndexUint32:VkBool32,pub imageCubeArray:VkBool32,pub independentBlend:VkBool32,pub geometryShader:VkBool32,pub tessellationShader:VkBool32,pub sampleRateShading:VkBool32,pub dualSrcBlend:VkBool32,pub logicOp:VkBool32,pub multiDrawIndirect:VkBool32,pub drawIndirectFirstInstance:VkBool32,pub depthClamp:VkBool32,pub depthBiasClamp:VkBool32,pub fillModeNonSolid:VkBool32,pub depthBounds:VkBool32,pub wideLines:VkBool32,pub largePoints:VkBool32,pub alphaToOne:VkBool32,pub multiViewport:VkBool32,pub samplerAnisotropy:VkBool32,pub textureCompressionETC2:VkBool32,pub textureCompressionASTC_LDR:VkBool32,pub textureCompressionBC:VkBool32,pub occlusionQueryPrecise:VkBool32,pub pipelineStatisticsQuery:VkBool32,pub vertexPipelineStoresAndAtomics:VkBool32,pub fragmentStoresAndAtomics:VkBool32,pub shaderTessellationAndGeometryPointSize:VkBool32,pub shaderImageGatherExtended:VkBool32,pub shaderStorageImageExtendedFormats:VkBool32,pub shaderStorageImageMultisample:VkBool32,pub shaderStorageImageReadWithoutFormat:VkBool32,pub shaderStorageImageWriteWithoutFormat:VkBool32,pub shaderUniformBufferArrayDynamicIndexing:VkBool32,pub shaderSampledImageArrayDynamicIndexing:VkBool32,pub shaderStorageBufferArrayDynamicIndexing:VkBool32,pub shaderStorageImageArrayDynamicIndexing:VkBool32,pub shaderClipDistance:VkBool32,pub shaderCullDistance:VkBool32,pub shaderFloat64:VkBool32,pub shaderInt64:VkBool32,pub shaderInt16:VkBool32,pub shaderResourceResidency:VkBool32,pub shaderResourceMinLod:VkBool32,pub sparseBinding:VkBool32,pub sparseResidencyBuffer:VkBool32,pub sparseResidencyImage2D:VkBool32,pub sparseResidencyImage3D:VkBool32,pub sparseResidency2Samples:VkBool32,pub sparseResidency4Samples:VkBool32,pub sparseResidency8Samples:VkBool32,pub sparseResidency16Samples:VkBool32,pub sparseResidencyAliased:VkBool32,pub variableMultisampleRate:VkBool32,pub inheritedQueries:VkBool32,}
#[rustfmt::skip]impl Default for VkPhysicalDeviceFeatures{#[inline(always)]fn default()->Self{unsafe{core::mem::MaybeUninit::zeroed().assume_init()}}}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceFeatures2=VkPhysicalDeviceFeatures2KHR;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceFeatures2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub features:VkPhysicalDeviceFeatures,}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceFeatures2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceFeatures2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceFeatures2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR;}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceFeatures2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceFloatControlsProperties=VkPhysicalDeviceFloatControlsPropertiesKHR;
#[cfg(feature = "VK_KHR_shader_float_controls")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceFloatControlsPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub denormBehaviorIndependence:VkShaderFloatControlsIndependenceKHR,pub roundingModeIndependence:VkShaderFloatControlsIndependenceKHR,pub shaderSignedZeroInfNanPreserveFloat16:VkBool32,pub shaderSignedZeroInfNanPreserveFloat32:VkBool32,pub shaderSignedZeroInfNanPreserveFloat64:VkBool32,pub shaderDenormPreserveFloat16:VkBool32,pub shaderDenormPreserveFloat32:VkBool32,pub shaderDenormPreserveFloat64:VkBool32,pub shaderDenormFlushToZeroFloat16:VkBool32,pub shaderDenormFlushToZeroFloat32:VkBool32,pub shaderDenormFlushToZeroFloat64:VkBool32,pub shaderRoundingModeRTEFloat16:VkBool32,pub shaderRoundingModeRTEFloat32:VkBool32,pub shaderRoundingModeRTEFloat64:VkBool32,pub shaderRoundingModeRTZFloat16:VkBool32,pub shaderRoundingModeRTZFloat32:VkBool32,pub shaderRoundingModeRTZFloat64:VkBool32,}
#[cfg(feature = "VK_KHR_shader_float_controls")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceFloatControlsPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_shader_float_controls")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceFloatControlsPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceGlobalPriorityQueryFeatures=VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR;
#[cfg(feature = "VK_KHR_global_priority")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub globalPriorityQuery:VkBool32,}
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceGroupProperties=VkPhysicalDeviceGroupPropertiesKHR;
#[cfg(feature = "VK_KHR_device_group_creation")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceGroupPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub physicalDeviceCount:u32,pub physicalDevices:[VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE_KHR],pub subsetAllocation:VkBool32,}
#[cfg(feature = "VK_KHR_device_group_creation")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceGroupPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_device_group_creation")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceGroupPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceHostImageCopyFeatures=VkPhysicalDeviceHostImageCopyFeaturesEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceHostImageCopyFeaturesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub hostImageCopy:VkBool32,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceHostImageCopyFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceHostImageCopyFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceHostImageCopyFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT;}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceHostImageCopyFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceHostImageCopyProperites=VkPhysicalDeviceHostImageCopyProperitesEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceHostImageCopyProperitesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub copySrcLayoutCount:u32,pub pCopySrcLayouts:*mut VkImageLayout,pub copyDstLayoutCount:u32,pub pCopyDstLayouts:*mut VkImageLayout,pub optimalTilingLayoutUUID:[u8; VK_UUID_SIZE],pub identicalMemoryTypeRequirements:VkBool32,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceHostImageCopyProperitesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceHostImageCopyProperitesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT;}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub drmFormatModifer:u64,pub sharingMode:VkSharingMode,pub queueFamilyIndexCount:u32,pub pQueueFamilyIndices:*const u32,}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceImageDrmFormatModifierInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceImageDrmFormatModifierInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceImageFormatInfo2=VkPhysicalDeviceImageFormatInfo2KHR;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceImageFormatInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub format:VkFormat,pub r#type:VkImageType,pub tiling:VkImageTiling,pub usage:VkImageUsageFlags,pub flags:VkImageCreateFlags,}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceImageFormatInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceImageFormatInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceIndexTypeUint8Features=VkPhysicalDeviceIndexTypeUint8FeaturesKHR;
#[cfg(feature = "VK_KHR_index_type_uint8")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceIndexTypeUint8FeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub indexTypeUint8:VkBool32,}
#[cfg(feature = "VK_KHR_index_type_uint8")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceIndexTypeUint8FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_index_type_uint8")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceIndexTypeUint8FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_index_type_uint8")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceIndexTypeUint8FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_index_type_uint8")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceIndexTypeUint8FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_maintenance7")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceLayeredApiPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub vendorID:u32,pub deviceID:u32,pub layeredAPI:VkPhysicalDeviceLayeredApiKHR,pub deviceName:crate::FixedCStrBuffer<VK_MAX_PHYSICAL_DEVICE_NAME_SIZE>,}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceLayeredApiPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceLayeredApiPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR;}
#[cfg(feature = "VK_KHR_maintenance7")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceLayeredApiPropertiesListKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub layeredApiCount:u32,pub pLayeredApis:*mut VkPhysicalDeviceLayeredApiPropertiesKHR,}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceLayeredApiPropertiesListKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceLayeredApiPropertiesListKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR;}
#[cfg(feature = "VK_KHR_maintenance7")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceLayeredApiVulkanPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub properties:VkPhysicalDeviceProperties2,}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceLayeredApiVulkanPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceLayeredApiVulkanPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceLimits{pub maxImageDimension1D:u32,pub maxImageDimension2D:u32,pub maxImageDimension3D:u32,pub maxImageDimensionCube:u32,pub maxImageArrayLayers:u32,pub maxTexelBufferElements:u32,pub maxUniformBufferRange:u32,pub maxStorageBufferRange:u32,pub maxPushConstantsSize:u32,pub maxMemoryAllocationCount:u32,pub maxSamplerAllocationCount:u32,pub bufferImageGranularity:VkDeviceSize,pub sparseAddressSpaceSize:VkDeviceSize,pub maxBoundDescriptorSets:u32,pub maxPerStageDescriptorSamplers:u32,pub maxPerStageDescriptorUniformBuffers:u32,pub maxPerStageDescriptorStorageBuffers:u32,pub maxPerStageDescriptorSampledImages:u32,pub maxPerStageDescriptorStorageImages:u32,pub maxPerStageDescriptorInputAttachments:u32,pub maxPerStageResources:u32,pub maxDescriptorSetSamplers:u32,pub maxDescriptorSetUniformBuffers:u32,pub maxDescriptorSetUniformBuffersDynamic:u32,pub maxDescriptorSetStorageBuffers:u32,pub maxDescriptorSetStorageBuffersDynamic:u32,pub maxDescriptorSetSampledImages:u32,pub maxDescriptorSetStorageImages:u32,pub maxDescriptorSetInputAttachments:u32,pub maxVertexInputAttributes:u32,pub maxVertexInputBindings:u32,pub maxVertexInputAttributeOffset:u32,pub maxVertexInputBindingStride:u32,pub maxVertexOutputComponents:u32,pub maxTessellationGenerationLevel:u32,pub maxTessellationPatchSize:u32,pub maxTessellationControlPerVertexInputComponents:u32,pub maxTessellationControlPerVertexOutputComponents:u32,pub maxTessellationControlPerPatchOutputComponents:u32,pub maxTessellationControlTotalOutputComponents:u32,pub maxTessellationEvaluationInputComponents:u32,pub maxTessellationEvaluationOutputComponents:u32,pub maxGeometryShaderInvocations:u32,pub maxGeometryInputComponents:u32,pub maxGeometryOutputComponents:u32,pub maxGeometryOutputVertices:u32,pub maxGeometryTotalOutputComponents:u32,pub maxFragmentInputComponents:u32,pub maxFragmentOutputAttachments:u32,pub maxFragmentDualSrcAttachments:u32,pub maxFragmentCombinedOutputResources:u32,pub maxComputeSharedMemorySize:u32,pub maxComputeWorkGroupCount:[u32; 3],pub maxComputeWorkGroupInvocations:u32,pub maxComputeWorkGroupSize:[u32; 3],pub subPixelPrecisionBits:u32,pub subTexelPrecisionBits:u32,pub mipmapPrecisionBits:u32,pub maxDrawIndexedIndexValue:u32,pub maxDrawIndirectCount:u32,pub maxSamplerLodBias:core::ffi::c_float,pub maxSamplerAnisotropy:core::ffi::c_float,pub maxViewports:u32,pub maxViewportDimensions:[u32; 2],pub viewportBoundsRange:[core::ffi::c_float; 2],pub viewportSubPixelBits:u32,pub minMemoryMapAlignment:usize,pub minTexelBufferOffsetAlignment:VkDeviceSize,pub minUniformBufferOffsetAlignment:VkDeviceSize,pub minStorageBufferOffsetAlignment:VkDeviceSize,pub minTexelOffset:i32,pub maxTexelOffset:u32,pub minTexelGatherOffset:i32,pub maxTexelGatherOffset:u32,pub minInterpolationOffset:core::ffi::c_float,pub maxInterpolationOffset:core::ffi::c_float,pub subPixelInterpolationOffsetBits:u32,pub maxFramebufferWidth:u32,pub maxFramebufferHeight:u32,pub maxFramebufferLayers:u32,pub framebufferColorSampleCounts:VkSampleCountFlags,pub framebufferDepthSampleCounts:VkSampleCountFlags,pub framebufferStencilSampleCounts:VkSampleCountFlags,pub framebufferNoAttachmentsSampleCounts:VkSampleCountFlags,pub maxColorAttachments:u32,pub sampledImageColorSampleCounts:VkSampleCountFlags,pub sampledImageIntegerSampleCounts:VkSampleCountFlags,pub sampledImageDepthSampleCounts:VkSampleCountFlags,pub sampledImageStencilSampleCounts:VkSampleCountFlags,pub storageImageSampleCounts:VkSampleCountFlags,pub maxSampleMaskWords:u32,pub timestampComputeAndGraphics:VkBool32,pub timestampPeriod:core::ffi::c_float,pub maxClipDistances:u32,pub maxCullDistances:u32,pub maxCombinedClipAndCullDistances:u32,pub discreteQueuePriorities:u32,pub pointSizeRange:[core::ffi::c_float; 2],pub lineWidthRange:[core::ffi::c_float; 2],pub pointSizeGranularity:core::ffi::c_float,pub lineWidthGranularity:core::ffi::c_float,pub strictLines:VkBool32,pub standardSampleLocations:VkBool32,pub optimalBufferCopyOffsetAlignment:VkDeviceSize,pub optimalBufferCopyRowPitchAlignment:VkDeviceSize,pub nonCoherentAtomSize:VkDeviceSize,}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceLineRasterizationFeatures=VkPhysicalDeviceLineRasterizationFeaturesKHR;
#[cfg(feature = "VK_KHR_line_rasterization")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceLineRasterizationFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub rectangularLines:VkBool32,pub bresenhamLines:VkBool32,pub smoothLines:VkBool32,pub stippledRectangularLines:VkBool32,pub stippledBresenhamLines:VkBool32,pub stippledSmoothLines:VkBool32,}
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceLineRasterizationFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceLineRasterizationFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceLineRasterizationFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceLineRasterizationFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceLineRasterizationProperties=VkPhysicalDeviceLineRasterizationPropertiesKHR;
#[cfg(feature = "VK_KHR_line_rasterization")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceLineRasterizationPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub lineSubPixelPrecisionBits:u32,}
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceLineRasterizationPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceLineRasterizationPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceMaintenance3Properties=VkPhysicalDeviceMaintenance3PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance3")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance3PropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxPerSetDescriptors:u32,pub maxMemoryAllocationSize:VkDeviceSize,}
#[cfg(feature = "VK_KHR_maintenance3")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance3PropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance3")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance3PropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceMaintenance4Features=VkPhysicalDeviceMaintenance4FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance4")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance4FeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maintenance4:VkBool32,}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceMaintenance4FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance4FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceMaintenance4FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance4FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceMaintenance4Properties=VkPhysicalDeviceMaintenance4PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance4")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance4PropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxBufferSize:VkDeviceSize,}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance4PropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance4PropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceMaintenance5Features=VkPhysicalDeviceMaintenance5FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance5")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance5FeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maintenance5:VkBool32,}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceMaintenance5FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance5FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceMaintenance5FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance5FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceMaintenance5Properties=VkPhysicalDeviceMaintenance5PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance5")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance5PropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub earlyFragmentMultisampleCoverageAfterSampleCounting:VkBool32,pub earlyFragmentSampleMaskTestBeforeSampleCounting:VkBool32,pub depthStencilSwizzleOneSupport:VkBool32,pub polygonModePointSize:VkBool32,pub nonStrictSinglePixelWideLinesUseParallelogram:VkBool32,pub nonStrictWideLinesUseParallelogram:VkBool32,}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance5PropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance5PropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceMaintenance6Features=VkPhysicalDeviceMaintenance6FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance6")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance6FeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maintenance6:VkBool32,}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceMaintenance6FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance6FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceMaintenance6FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance6FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceMaintenance6Properties=VkPhysicalDeviceMaintenance6PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance6")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance6PropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub blockTexelViewCompatibleMultipleLayers:VkBool32,pub maxCombinedImageSamplerDescriptorCount:u32,pub fragmentShadingRateClampCombinerInputs:VkBool32,}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance6PropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance6PropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR;}
#[cfg(feature = "VK_KHR_maintenance7")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance7FeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maintenance7:VkBool32,}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceMaintenance7FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance7FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceMaintenance7FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance7FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_maintenance7")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance7PropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub robustFragmentShadingRateAttachmentAccess:VkBool32,pub separateDepthStencilAttachmentAccess:VkBool32,pub maxDescriptorSetTotalUniformBuffersDynamic:u32,pub maxDescriptorSetTotalStorageBuffersDynamic:u32,pub maxDescriptorSetTotalBuffersDynamic:u32,pub maxDescriptorSetUpdateAfterBindTotalUniformBuffersDynamic:u32,pub maxDescriptorSetUpdateAfterBindTotalStorageBuffersDynamic:u32,pub maxDescriptorSetUpdateAfterBindTotalBuffersDynamic:u32,}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance7PropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance7")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance7PropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR;}
#[cfg(feature = "VK_KHR_maintenance8")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance8FeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maintenance8:VkBool32,}
#[cfg(feature = "VK_KHR_maintenance8")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceMaintenance8FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance8")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance8FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance8")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceMaintenance8FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_maintenance8")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance8FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_maintenance9")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance9FeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maintenance9:VkBool32,}
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceMaintenance9FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance9FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceMaintenance9FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance9FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_maintenance9")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMaintenance9PropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub image2DViewOf3DSparse:VkBool32,pub defaultVertexAttributeValue:VkDefaultVertexAttributeValueKHR,}
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance9PropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance9PropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMemoryProperties{pub memoryTypeCount:u32,pub memoryTypes:[VkMemoryType; VK_MAX_MEMORY_TYPES],pub memoryHeapCount:u32,pub memoryHeaps:[VkMemoryHeap; VK_MAX_MEMORY_HEAPS],}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceMemoryProperties2=VkPhysicalDeviceMemoryProperties2KHR;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMemoryProperties2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub memoryProperties:VkPhysicalDeviceMemoryProperties,}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMemoryProperties2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMemoryProperties2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceMultiviewFeatures=VkPhysicalDeviceMultiviewFeaturesKHR;
#[cfg(feature = "VK_KHR_multiview")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMultiviewFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub multiview:VkBool32,pub multiviewGeometryShader:VkBool32,pub multiviewTessellationShader:VkBool32,}
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceMultiviewFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMultiviewFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceMultiviewFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMultiviewFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR;}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub perViewPositionAllComponents:VkBool32,}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceMultiviewProperties=VkPhysicalDeviceMultiviewPropertiesKHR;
#[cfg(feature = "VK_KHR_multiview")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceMultiviewPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxMultiviewViewCount:u32,pub maxMultiviewInstanceIndex:u32,}
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMultiviewPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMultiviewPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDevicePipelineProtectedAccessFeatures=VkPhysicalDevicePipelineProtectedAccessFeaturesEXT;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDevicePipelineProtectedAccessFeaturesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub pipelineProtectedAccess:VkBool32,}
#[cfg(feature = "VK_EXT_pipeline_protected_access")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDevicePipelineProtectedAccessFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_pipeline_protected_access")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDevicePipelineProtectedAccessFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_pipeline_protected_access")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDevicePipelineProtectedAccessFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT;}
#[cfg(feature = "VK_EXT_pipeline_protected_access")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDevicePipelineProtectedAccessFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDevicePipelineRobustnessFeatures=VkPhysicalDevicePipelineRobustnessFeaturesEXT;
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDevicePipelineRobustnessFeaturesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub pipelineRobustness:VkBool32,}
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDevicePipelineRobustnessFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDevicePipelineRobustnessFeaturesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDevicePipelineRobustnessFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT;}
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDevicePipelineRobustnessFeaturesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDevicePipelineRobustnessProperties=VkPhysicalDevicePipelineRobustnessPropertiesEXT;
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDevicePipelineRobustnessPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub defaultRobustnessStorageBuffers:VkPipelineRobustnessBufferBehaviorEXT,pub defaultRobustnessUniformBuffers:VkPipelineRobustnessBufferBehaviorEXT,pub defaultRobustnessVertexInputs:VkPipelineRobustnessBufferBehaviorEXT,pub defaultRobustnessImage:VkPipelineRobustnessImageBehaviorEXT,}
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDevicePipelineRobustnessPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDevicePipelineRobustnessPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDevicePointClippingProperties=VkPhysicalDevicePointClippingPropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDevicePointClippingPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub pointClippingBehavior:VkPointClippingBehaviorKHR,}
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDevicePointClippingPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDevicePointClippingPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceProperties{pub apiVersion:u32,pub driverVersion:u32,pub vendorID:u32,pub deviceID:u32,pub deviceType:VkPhysicalDeviceType,pub deviceName:crate::FixedCStrBuffer<VK_MAX_PHYSICAL_DEVICE_NAME_SIZE>,pub pipelineCacheUUID:[u8; VK_UUID_SIZE],pub limits:VkPhysicalDeviceLimits,pub sparseProperties:VkPhysicalDeviceSparseProperties,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceProperties2=VkPhysicalDeviceProperties2KHR;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceProperties2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub properties:VkPhysicalDeviceProperties,}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceProperties2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceProperties2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDevicePushDescriptorProperties=VkPhysicalDevicePushDescriptorPropertiesKHR;
#[cfg(feature = "VK_KHR_push_descriptor")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDevicePushDescriptorPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxPushDescriptors:u32,}
#[cfg(feature = "VK_KHR_push_descriptor")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDevicePushDescriptorPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_push_descriptor")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDevicePushDescriptorPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR;}
#[cfg(feature = "VK_EXT_sample_locations")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub sampleLocationSampleCounts:VkSampleCountFlags,pub maxSampleLocationGridSize:VkExtent2D,pub sampleLocationCoordinateRange:[core::ffi::c_float; 2],pub sampleLocationSubpixelBits:u32,pub variableSampleLocations:VkBool32,}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceSampleLocationsPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceSampleLocationsPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceSamplerFilterMinmaxProperties=VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub filterMinmaxSingleComponentFormats:VkBool32,pub filterMinmaxImageComponentMapping:VkBool32,}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceSamplerYcbcrConversionFeatures=VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub samplerYcbcrConversion:VkBool32,}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceShaderExpectAssumeFeatures=VkPhysicalDeviceShaderExpectAssumeFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_expect_assume")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceShaderExpectAssumeFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub shaderExpectAssume:VkBool32,}
#[cfg(feature = "VK_KHR_shader_expect_assume")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceShaderExpectAssumeFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_shader_expect_assume")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceShaderExpectAssumeFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_shader_expect_assume")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceShaderExpectAssumeFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_shader_expect_assume")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceShaderExpectAssumeFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceShaderFloatControls2Features=VkPhysicalDeviceShaderFloatControls2FeaturesKHR;
#[cfg(feature = "VK_KHR_shader_float_controls2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceShaderFloatControls2FeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub shaderFloatControls2:VkBool32,}
#[cfg(feature = "VK_KHR_shader_float_controls2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceShaderFloatControls2FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_shader_float_controls2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceShaderFloatControls2FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_shader_float_controls2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceShaderFloatControls2FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_shader_float_controls2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceShaderFloatControls2FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceShaderSubgroupRotateFeatures=VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub shaderSubgroupRotate:VkBool32,pub shaderSubgroupRotateClustered:VkBool32,}
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceSparseImageFormatInfo2=VkPhysicalDeviceSparseImageFormatInfo2KHR;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub format:VkFormat,pub r#type:VkImageType,pub samples:VkSampleCountFlagBits,pub usage:VkImageUsageFlags,pub tiling:VkImageTiling,}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceSparseImageFormatInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceSparseImageFormatInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceSparseProperties{pub residencyStandard2DBlockShape:VkBool32,pub residencyStandard2DMultisampleBlockShape:VkBool32,pub residencyStandard3DBlockShape:VkBool32,pub residencyAlignedMipSize:VkBool32,pub residencyNonResidentStrict:VkBool32,}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceSurfaceInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub surface:VkSurfaceKHR,}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceSurfaceInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceSurfaceInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceSynchronization2Features=VkPhysicalDeviceSynchronization2FeaturesKHR;
#[cfg(feature = "VK_KHR_synchronization2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceSynchronization2FeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub synchronization2:VkBool32,}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceSynchronization2FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceSynchronization2FeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceSynchronization2FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceSynchronization2FeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceTimelineSemaphoreFeatures=VkPhysicalDeviceTimelineSemaphoreFeaturesKHR;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceTimelineSemaphoreFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub timelineSemaphore:VkBool32,}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceTimelineSemaphoreFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceTimelineSemaphoreFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceTimelineSemaphoreFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceTimelineSemaphoreFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceTimelineSemaphoreProperties=VkPhysicalDeviceTimelineSemaphorePropertiesKHR;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceTimelineSemaphorePropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxTimelineSemaphoreValueDifference:u64,}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceTimelineSemaphorePropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceTimelineSemaphorePropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceVariablePointersFeatures=VkPhysicalDeviceVariablePointersFeaturesKHR;
#[cfg(feature = "VK_KHR_variable_pointers")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceVariablePointersFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub variablePointersStorageBuffer:VkBool32,pub variablePointers:VkBool32,}
#[cfg(feature = "VK_KHR_variable_pointers")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceVariablePointersFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_variable_pointers")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVariablePointersFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_variable_pointers")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceVariablePointersFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_variable_pointers")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVariablePointersFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceVertexAttributeDivisorFeatures=VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub vertexAttributeInstanceRateDivisor:VkBool32,pub vertexAttributeInstanceRateZeroDivisor:VkBool32,}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR;}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPhysicalDeviceVertexAttributeDivisorProperties=VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxVertexAttribDivisor:u32,}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT;}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub maxVertexAttribDivisor:u32,pub supportsNonZeroFirstInstance:VkBool32,}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceVulkan11Features{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub storageBuffer16BitAccess:VkBool32,pub uniformAndStorageBuffer16BitAccess:VkBool32,pub storagePushConstant16:VkBool32,pub storageInputOutput16:VkBool32,pub multiview:VkBool32,pub multiviewGeometryShader:VkBool32,pub multiviewTessellationShader:VkBool32,pub variablePointersStorageBuffer:VkBool32,pub variablePointers:VkBool32,pub protectedMemory:VkBool32,pub samplerYcbcrConversion:VkBool32,pub shaderDrawParameters:VkBool32,}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPhysicalDeviceVulkan11Features{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVulkan11Features{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPhysicalDeviceVulkan11Features{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVulkan11Features{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]impl Default for VkPhysicalDeviceVulkan11Features{#[inline(always)]fn default()->Self{let mut p=core::mem::MaybeUninit::<Self>::zeroed();unsafe{core::ptr::addr_of_mut!((*p.as_mut_ptr()).sType).write(VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES);p.assume_init()}}}
#[cfg(feature = "Allow1_2APIs")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPhysicalDeviceVulkan11Properties{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub deviceUUID:[u8; VK_UUID_SIZE],pub driverUUID:[u8; VK_UUID_SIZE],pub deviceLUID:[u8; VK_LUID_SIZE],pub deviceNodeMask:u32,pub deviceLUIDValid:VkBool32,pub subgroupSize:u32,pub subgroupSupportedStages:VkShaderStageFlags,pub subgroupSupportedOperations:VkSubgroupFeatureFlags,pub subgroupQuadOperationsInAllStages:VkBool32,pub pointClippingBehavior:VkPointClippingBehavior,pub maxMultiviewViewCount:u32,pub maxMultiviewInstanceIndex:u32,pub protectedNoFault:VkBool32,pub maxPerSetDescriptors:u32,pub maxMemoryAllocationSize:VkDeviceSize,}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVulkan11Properties{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVulkan11Properties{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineCacheCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineCacheCreateFlags,pub initialDataSize:usize,pub pInitialData:*const core::ffi::c_void,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineCacheCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineCacheCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO;}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcPremultiplied:VkBool32,pub dstPremultiplied:VkBool32,pub blendOverlap:VkBlendOverlapEXT,}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineColorBlendAdvancedStateCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineColorBlendAdvancedStateCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineColorBlendAttachmentState{pub blendEnable:VkBool32,pub srcColorBlendFactor:VkBlendFactor,pub dstColorBlendFactor:VkBlendFactor,pub colorBlendOp:VkBlendOp,pub srcAlphaBlendFactor:VkBlendFactor,pub dstAlphaBlendFactor:VkBlendFactor,pub alphaBlendOp:VkBlendOp,pub colorWriteMask:VkColorComponentFlags,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineColorBlendStateCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineColorBlendStateCreateFlags,pub logicOpEnable:VkBool32,pub logicOp:VkLogicOp,pub attachmentCount:u32,pub pAttachments:*const VkPipelineColorBlendAttachmentState,pub blendConstants:[core::ffi::c_float; 4],}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineColorBlendStateCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineColorBlendStateCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineCoverageModulationStateCreateInfoNV{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineCoverageModulationStateCreateFlagsNV,pub coverageModulationMode:VkCoverageModulationModeNV,pub coverageModulationTableEnable:VkBool32,pub coverageModulationTableCount:u32,pub pCoverageModulationTable:*const core::ffi::c_float,}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineCoverageModulationStateCreateInfoNV{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineCoverageModulationStateCreateInfoNV{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV;}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineCoverageToColorStateCreateInfoNV{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineCoverageToColorStateCreateFlagsNV,pub coverageToColorEnable:VkBool32,pub coverageToColorLocation:u32,}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineCoverageToColorStateCreateInfoNV{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineCoverageToColorStateCreateInfoNV{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPipelineCreateFlags2CreateInfo=VkPipelineCreateFlags2CreateInfoKHR;
#[cfg(feature = "VK_KHR_maintenance5")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineCreateFlags2CreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineCreateFlags2KHR,}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineCreateFlags2CreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineCreateFlags2CreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineDepthStencilStateCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineDepthStencilStateCreateFlags,pub depthTestEnable:VkBool32,pub depthWriteEnable:VkBool32,pub depthCompareOp:VkCompareOp,pub depthBoundsTestEnable:VkBool32,pub stencilTestEnable:VkBool32,pub front:VkStencilOpState,pub back:VkStencilOpState,pub minDepthBounds:core::ffi::c_float,pub maxDepthBounds:core::ffi::c_float,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineDepthStencilStateCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineDepthStencilStateCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;}
#[cfg(feature = "VK_EXT_discard_rectangles")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineDiscardRectangleStateCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineDiscardRectangleStateCreateFlagsEXT,pub discardRectangleMode:VkDiscardRectangleModeEXT,pub discardRectangleCount:u32,pub pDiscardRectangles:*const VkRect2D,}
#[cfg(feature = "VK_EXT_discard_rectangles")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineDiscardRectangleStateCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_discard_rectangles")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineDiscardRectangleStateCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineDynamicStateCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineDynamicStateCreateFlags,pub dynamicStateCount:u32,pub pDynamicStates:*const VkDynamicState,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineDynamicStateCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineDynamicStateCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineInputAssemblyStateCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineInputAssemblyStateCreateFlags,pub topology:VkPrimitiveTopology,pub primitiveRestartEnable:VkBool32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineInputAssemblyStateCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineInputAssemblyStateCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineLayoutCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineLayoutCreateFlags,pub setLayoutCount:u32,pub pSetLayouts:*const VkDescriptorSetLayout,pub pushConstantRangeCount:u32,pub pPushConstantRanges:*const VkPushConstantRange,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineLayoutCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineLayoutCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineMultisampleStateCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineMultisampleStateCreateFlags,pub rasterizationSamples:VkSampleCountFlagBits,pub sampleShadingEnable:VkBool32,pub minSampleShading:core::ffi::c_float,pub pSampleMask:*const VkSampleMask,pub alphaToCoverageEnable:VkBool32,pub alphaToOneEnable:VkBool32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineMultisampleStateCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineMultisampleStateCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;}
#[cfg(feature = "VK_EXT_conservative_rasterization")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineRasterizationConservativeStateCreateFlagsEXT,pub conservativeRasterizationMode:VkConservativeRasterizationModeEXT,pub extraPrimitiveOverestimationSize:core::ffi::c_float,}
#[cfg(feature = "VK_EXT_conservative_rasterization")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineRasterizationConservativeStateCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_conservative_rasterization")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineRasterizationConservativeStateCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPipelineRasterizationLineStateCreateInfo=VkPipelineRasterizationLineStateCreateInfoKHR;
#[cfg(feature = "VK_KHR_line_rasterization")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineRasterizationLineStateCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub lineRasterizationMode:VkLineRasterizationModeKHR,pub stippledLineEnable:VkBool32,pub lineStippleFactor:u32,pub lineStipplePattern:u16,}
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineRasterizationLineStateCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineRasterizationLineStateCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineRasterizationStateCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineRasterizationStateCreateFlags,pub depthClampEnable:VkBool32,pub rasterizerDiscardEnable:VkBool32,pub polygonMode:VkPolygonMode,pub cullMode:VkCullModeFlags,pub frontFace:VkFrontFace,pub depthBiasEnable:VkBool32,pub depthBiasConstantFactor:core::ffi::c_float,pub depthBiasClamp:core::ffi::c_float,pub depthBiasSlopeFactor:core::ffi::c_float,pub lineWidth:core::ffi::c_float,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineRasterizationStateCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineRasterizationStateCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;}
#[cfg(feature = "VK_AMD_rasterization_order")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineRasterizationStateRasterizationOrderAMD{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub rasterizationOrder:VkRasterizationOrderAMD,}
#[cfg(feature = "VK_AMD_rasterization_order")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineRasterizationStateRasterizationOrderAMD{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_AMD_rasterization_order")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineRasterizationStateRasterizationOrderAMD{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkPipelineRenderingCreateInfo=VkPipelineRenderingCreateInfoKHR;
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineRenderingCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub viewMask:u32,pub colorAttachmentCount:u32,pub pColorAttachmentFormats:*const VkFormat,pub depthAttachmentFormat:VkFormat,pub stencilAttachmentFormat:VkFormat,}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineRenderingCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineRenderingCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPipelineRobustnessCreateInfo=VkPipelineRobustnessCreateInfoEXT;
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineRobustnessCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub storageBuffers:VkPipelineRobustnessBufferBehaviorEXT,pub uniformBuffers:VkPipelineRobustnessBufferBehaviorEXT,pub vertexInputs:VkPipelineRobustnessBufferBehaviorEXT,pub images:VkPipelineRobustnessImageBehaviorEXT,}
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineRobustnessCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_pipeline_robustness")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineRobustnessCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO_EXT;}
#[cfg(feature = "VK_EXT_sample_locations")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineSampleLocationsStateCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub sampleLocationsEnable:VkBool32,pub sampleLocationsInfo:VkSampleLocationsInfoEXT,}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineSampleLocationsStateCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineSampleLocationsStateCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineShaderStageCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineShaderStageCreateFlags,pub stage:VkShaderStageFlagBits,pub module:Option<VkShaderModule>,pub pName:*const core::ffi::c_char,pub pSpecializationInfo:*const VkSpecializationInfo,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineShaderStageCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineShaderStageCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkPipelineTessellationDomainOriginStateCreateInfo=VkPipelineTessellationDomainOriginStateCreateInfoKHR;
#[cfg(feature = "VK_KHR_maintenance2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub domainOrigin:VkTessellationDomainOriginKHR,}
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineTessellationDomainOriginStateCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineTessellationDomainOriginStateCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineTessellationStateCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineTessellationStateCreateFlags,pub patchControlPoints:u32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineTessellationStateCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineTessellationStateCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPipelineVertexInputDivisorStateCreateInfo=VkPipelineVertexInputDivisorStateCreateInfoKHR;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub vertexBindingDivisorCount:u32,pub pVertexBindingDivisors:*const VkVertexInputBindingDivisorDescriptionEXT,}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineVertexInputDivisorStateCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineVertexInputDivisorStateCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT;}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineVertexInputDivisorStateCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub vertexBindingDivisorCount:u32,pub pVertexBindingDivisors:*const VkVertexInputBindingDivisorDescriptionKHR,}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineVertexInputDivisorStateCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineVertexInputDivisorStateCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineVertexInputStateCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineVertexInputStateCreateFlags,pub vertexBindingDescriptionCount:u32,pub pVertexBindingDescriptions:*const VkVertexInputBindingDescription,pub vertexAttributeDescriptionCount:u32,pub pVertexAttributeDescriptions:*const VkVertexInputAttributeDescription,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineVertexInputStateCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineVertexInputStateCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineViewportStateCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkPipelineViewportStateCreateFlags,pub viewportCount:u32,pub pViewports:*const VkViewport,pub scissorCount:u32,pub pScissors:*const VkRect2D,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineViewportStateCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineViewportStateCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;}
#[cfg(feature = "VK_NV_viewport_swizzle")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineViewportSwizzleStateCreateInfoNV{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub viewportCount:u32,pub pViewportSwizzles:*const VkViewportSwizzleNV,}
#[cfg(feature = "VK_NV_viewport_swizzle")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineViewportSwizzleStateCreateInfoNV{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NV_viewport_swizzle")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineViewportSwizzleStateCreateInfoNV{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV;}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPipelineViewportWScalingStateCreateInfoNV{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub viewportWScalingEnable:VkBool32,pub viewportCount:u32,pub pViewportWScalings:*const VkViewportWScalingNV,}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPipelineViewportWScalingStateCreateInfoNV{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPipelineViewportWScalingStateCreateInfoNV{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV;}
#[cfg(feature = "VK_KHR_swapchain")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPresentInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub waitSemaphoreCount:u32,pub pWaitSemaphores:*const VkSemaphore,pub swapchainCount:u32,pub pSwapchains:*const VkSwapchainKHR,pub pImageIndices:*const u32,pub pResults:*mut VkResult,}
#[cfg(feature = "VK_KHR_swapchain")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPresentInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_swapchain")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPresentInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;}
#[cfg(feature = "VK_KHR_incremental_present")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPresentRegionKHR{pub rectangleCount:u32,pub pRectangles:*const VkRectLayerKHR,}
#[cfg(feature = "VK_KHR_incremental_present")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPresentRegionsKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub swapchainCount:u32,pub pRegions:*const VkPresentRegionKHR,}
#[cfg(feature = "VK_KHR_incremental_present")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPresentRegionsKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_incremental_present")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPresentRegionsKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR;}
#[cfg(feature = "VK_GOOGLE_display_timing")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPresentTimeGOOGLE{pub presentID:u32,pub desiredPresentTime:u64,}
#[cfg(feature = "VK_GOOGLE_display_timing")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPresentTimesInfoGOOGLE{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub swapchainCount:u32,pub pTimes:*const VkPresentTimeGOOGLE,}
#[cfg(feature = "VK_GOOGLE_display_timing")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPresentTimesInfoGOOGLE{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_GOOGLE_display_timing")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPresentTimesInfoGOOGLE{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPushConstantRange{pub stageFlags:VkShaderStageFlags,pub offset:u32,pub size:u32,}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPushConstantsInfo=VkPushConstantsInfoKHR;
#[cfg(feature = "VK_KHR_maintenance6")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPushConstantsInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub layout:VkPipelineLayout,pub stageFlags:VkShaderStageFlags,pub offset:u32,pub size:u32,pub pValues:*const core::ffi::c_void,}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPushConstantsInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPushConstantsInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PUSH_CONSTANTS_INFO_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPushDescriptorSetInfo=VkPushDescriptorSetInfoKHR;
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPushDescriptorSetInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub stageFlags:VkShaderStageFlags,pub layout:VkPipelineLayout,pub set:u32,pub descriptorWriteCount:u32,pub pDescriptorWrites:*const VkWriteDescriptorSet,}
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPushDescriptorSetInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPushDescriptorSetInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_INFO_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkPushDescriptorSetWithTemplateInfo=VkPushDescriptorSetWithTemplateInfoKHR;
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkPushDescriptorSetWithTemplateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub descriptorUpdateTemplate:VkDescriptorUpdateTemplateKHR,pub layout:VkPipelineLayout,pub set:u32,pub pData:*const core::ffi::c_void,}
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkPushDescriptorSetWithTemplateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkPushDescriptorSetWithTemplateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkQueryPoolCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkQueryPoolCreateFlags,pub queryType:VkQueryType,pub queryCount:u32,pub pipelineStatistics:VkQueryPipelineStatisticFlags,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkQueryPoolCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkQueryPoolCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkQueueFamilyGlobalPriorityProperties=VkQueueFamilyGlobalPriorityPropertiesKHR;
#[cfg(feature = "VK_KHR_global_priority")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkQueueFamilyGlobalPriorityPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub priorityCount:u32,pub priorities:[VkQueueGlobalPriorityKHR; VK_MAX_GLOBAL_PRIORITY_SIZE_KHR],}
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkQueueFamilyGlobalPriorityPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_global_priority")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkQueueFamilyGlobalPriorityPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR;}
#[cfg(feature = "VK_KHR_maintenance9")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkQueueFamilyOwnershipTransferPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub optimalImageTransferToQueueFamilies:u32,}
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkQueueFamilyOwnershipTransferPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance9")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkQueueFamilyOwnershipTransferPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkQueueFamilyProperties{pub queueFlags:VkQueueFlags,pub queueCount:u32,pub timestampValidBits:u32,pub minImageTransferGranularity:VkExtent3D,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkQueueFamilyProperties2=VkQueueFamilyProperties2KHR;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkQueueFamilyProperties2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub queueFamilyProperties:VkQueueFamilyProperties,}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkQueueFamilyProperties2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkQueueFamilyProperties2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR;}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[rustfmt::skip]#[repr(C)]pub struct VkRect2D{pub offset:VkOffset2D,pub extent:VkExtent2D,}
#[cfg(feature = "VK_KHR_incremental_present")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRectLayerKHR{pub offset:VkOffset2D,pub extent:VkExtent2D,pub layer:u32,}
#[cfg(feature = "VK_GOOGLE_display_timing")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRefreshCycleDurationGOOGLE{pub refreshDuration:u64,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderPassBeginInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub renderPass:VkRenderPass,pub framebuffer:VkFramebuffer,pub renderArea:VkRect2D,pub clearValueCount:u32,pub pClearValues:*const VkClearValue,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderPassBeginInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderPassBeginInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderPassCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkRenderPassCreateFlags,pub attachmentCount:u32,pub pAttachments:*const VkAttachmentDescription,pub subpassCount:u32,pub pSubpasses:*const VkSubpassDescription,pub dependencyCount:u32,pub pDependencies:*const VkSubpassDependency,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderPassCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderPassCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkRenderPassCreateInfo2=VkRenderPassCreateInfo2KHR;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderPassCreateInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkRenderPassCreateFlags,pub attachmentCount:u32,pub pAttachments:*const VkAttachmentDescription2KHR,pub subpassCount:u32,pub pSubpasses:*const VkSubpassDescription2KHR,pub dependencyCount:u32,pub pDependencies:*const VkSubpassDependency2KHR,pub correlatedViewMaskCount:u32,pub pCorrelatedViewMasks:*const u32,}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderPassCreateInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderPassCreateInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkRenderPassInputAttachmentAspectCreateInfo=VkRenderPassInputAttachmentAspectCreateInfoKHR;
#[cfg(feature = "VK_KHR_maintenance2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderPassInputAttachmentAspectCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub aspectReferenceCount:u32,pub pAspectReferences:*const VkInputAttachmentAspectReferenceKHR,}
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderPassInputAttachmentAspectCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderPassInputAttachmentAspectCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkRenderPassMultiviewCreateInfo=VkRenderPassMultiviewCreateInfoKHR;
#[cfg(feature = "VK_KHR_multiview")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderPassMultiviewCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub subpassCount:u32,pub pViewMasks:*const u32,pub dependencyCount:u32,pub pViewOffsets:*const i32,pub correlationMaskCount:u32,pub pCorrelationMasks:*const u32,}
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderPassMultiviewCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_multiview")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderPassMultiviewCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR;}
#[cfg(feature = "VK_EXT_sample_locations")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderPassSampleLocationsBeginInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub attachmentInitialSampleLocationsCount:u32,pub pAttachmentInitialSampleLocations:*const VkAttachmentSampleLocationsEXT,pub postSubpassSampleLocationsCount:u32,pub pPostSubpassSampleLocations:*const VkSubpassSampleLocationsEXT,}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderPassSampleLocationsBeginInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderPassSampleLocationsBeginInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkRenderingAreaInfo=VkRenderingAreaInfoKHR;
#[cfg(feature = "VK_KHR_maintenance5")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderingAreaInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub viewMask:u32,pub colorAttachmentCount:u32,pub pColorAttachmentFormats:*const VkFormat,pub depthAttachmentFormat:VkFormat,pub stencilAttachmentFormat:VkFormat,}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderingAreaInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderingAreaInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDERING_AREA_INFO_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkRenderingAttachmentInfo=VkRenderingAttachmentInfoKHR;
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[derive(Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderingAttachmentInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub imageView:VkImageView,pub imageLayout:VkImageLayout,pub resolveMode:VkResolveModeFlagBitsKHR,pub resolveImageView:VkImageView,pub resolveImageLayout:VkImageLayout,pub loadOp:VkAttachmentLoadOp,pub storeOp:VkAttachmentStoreOp,pub clearValue:VkClearValue,}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderingAttachmentInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderingAttachmentInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkRenderingAttachmentLocationInfo=VkRenderingAttachmentLocationInfoKHR;
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderingAttachmentLocationInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub colorAttachmentCount:u32,pub pColorAttachmentLocations:*const u32,}
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderingAttachmentLocationInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderingAttachmentLocationInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_LOCATION_INFO_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkRenderingInfo=VkRenderingInfoKHR;
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderingInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkRenderingFlagsKHR,pub renderArea:VkRect2D,pub layoutCount:u32,pub viewMask:u32,pub colorAttachmentCount:u32,pub pColorAttachments:*const VkRenderingAttachmentInfoKHR,pub pDepthAttachment:*const VkRenderingAttachmentInfoKHR,pub pStencilAttachment:*const VkRenderingAttachmentInfoKHR,}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderingInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderingInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDERING_INFO_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkRenderingInputAttachmentIndexInfo=VkRenderingInputAttachmentIndexInfoKHR;
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkRenderingInputAttachmentIndexInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub colorAttachmentCount:u32,pub pColorAttachmentInputIndices:*const u32,pub pDepthInputAttachmentIndex:*const u32,pub pStencilInputAttachmentIndex:*const u32,}
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkRenderingInputAttachmentIndexInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkRenderingInputAttachmentIndexInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkResolveImageInfo2=VkResolveImageInfo2KHR;
#[cfg(feature = "VK_KHR_copy_commands2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkResolveImageInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcImage:VkImage,pub srcImageLayout:VkImageLayout,pub dstImage:VkImage,pub dstImageLayout:VkImageLayout,pub regionCount:u32,pub pRegions:*const VkImageResolve2KHR,}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkResolveImageInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkResolveImageInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2_KHR;}
#[cfg(feature = "VK_EXT_sample_locations")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSampleLocationEXT{pub x:core::ffi::c_float,pub y:core::ffi::c_float,}
#[cfg(feature = "VK_EXT_sample_locations")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSampleLocationsInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub sampleLocationsPerPixel:VkSampleCountFlags,pub sampleLocationGridSize:VkExtent2D,pub sampleLocationsCount:u32,pub pSampleLocations:*const VkSampleLocationEXT,}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSampleLocationsInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSampleLocationsInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT;}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSamplerCaptureDescriptorDataInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub sampler:VkSampler,}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSamplerCaptureDescriptorDataInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSamplerCaptureDescriptorDataInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSamplerCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkSamplerCreateFlags,pub magFilter:VkFilter,pub minFilter:VkFilter,pub mipmapMode:VkSamplerMipmapMode,pub addressModeU:VkSamplerAddressMode,pub addressModeV:VkSamplerAddressMode,pub addressModeW:VkSamplerAddressMode,pub mipLodBias:core::ffi::c_float,pub anisotropyEnable:VkBool32,pub maxAnisotropy:core::ffi::c_float,pub compareEnable:VkBool32,pub compareOp:VkCompareOp,pub minLod:core::ffi::c_float,pub maxLod:core::ffi::c_float,pub borderColor:VkBorderColor,pub unnormalizedCoordinates:VkBool32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSamplerCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSamplerCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkSamplerReductionModeCreateInfo=VkSamplerReductionModeCreateInfoEXT;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSamplerReductionModeCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub reductionMode:VkSamplerReductionModeEXT,}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSamplerReductionModeCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSamplerReductionModeCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkSamplerYcbcrConversionCreateInfo=VkSamplerYcbcrConversionCreateInfoKHR;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSamplerYcbcrConversionCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub format:VkFormat,pub ycbcrModel:VkSamplerYcbcrModelConversionKHR,pub ycbcrRange:VkSamplerYcbcrRangeKHR,pub components:VkComponentMapping,pub xChromaOffset:VkChromaLocationKHR,pub yChromaOffset:VkChromaLocationKHR,pub chromaFilter:VkFilter,pub forceExplicitReconstruction:VkBool32,}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSamplerYcbcrConversionCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSamplerYcbcrConversionCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkSamplerYcbcrConversionImageFormatProperties=VkSamplerYcbcrConversionImageFormatPropertiesKHR;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSamplerYcbcrConversionImageFormatPropertiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub combinedImageSamplerDescriptorCount:u32,}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSamplerYcbcrConversionImageFormatPropertiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSamplerYcbcrConversionImageFormatPropertiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR;}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkSamplerYcbcrConversionInfo=VkSamplerYcbcrConversionInfoKHR;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSamplerYcbcrConversionInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub conversion:VkSamplerYcbcrConversion,}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSamplerYcbcrConversionInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSamplerYcbcrConversionInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSemaphoreCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkSemaphoreCreateFlags,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSemaphoreCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSemaphoreCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO;}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSemaphoreGetFdInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub semaphore:VkSemaphore,pub handleType:VkExternalSemaphoreHandleTypeFlagBitsKHR,}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSemaphoreGetFdInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSemaphoreGetFdInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR;}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSemaphoreGetWin32HandleInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub semaphore:VkSemaphore,pub handleType:VkExternalSemaphoreHandleTypeFlagBitsKHR,}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSemaphoreGetWin32HandleInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSemaphoreGetWin32HandleInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkSemaphoreSignalInfo=VkSemaphoreSignalInfoKHR;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSemaphoreSignalInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub semaphore:VkSemaphore,pub value:u64,}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSemaphoreSignalInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSemaphoreSignalInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO_KHR;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkSemaphoreSubmitInfo=VkSemaphoreSubmitInfoKHR;
#[cfg(feature = "VK_KHR_synchronization2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSemaphoreSubmitInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub semaphore:VkSemaphore,pub value:u64,pub stageMask:VkPipelineStageFlags2KHR,pub deviceIndex:u32,}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSemaphoreSubmitInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSemaphoreSubmitInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkSemaphoreTypeCreateInfo=VkSemaphoreTypeCreateInfoKHR;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSemaphoreTypeCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub semaphoreType:VkSemaphoreTypeKHR,pub initialValue:u64,}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSemaphoreTypeCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSemaphoreTypeCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkSemaphoreWaitInfo=VkSemaphoreWaitInfoKHR;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSemaphoreWaitInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkSemaphoreWaitFlagsKHR,pub semaphoreCount:u32,pub pSemaphores:*const VkSemaphore,pub pValues:*const u64,}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSemaphoreWaitInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSemaphoreWaitInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkShaderModuleCreateInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkShaderModuleCreateFlags,pub codeSize:usize,pub pCode:*const u32,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkShaderModuleCreateInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkShaderModuleCreateInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;}
#[cfg(feature = "VK_EXT_validation_cache")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkShaderModuleValidationcacheCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub validationCache:VkValidationCacheEXT,}
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkShaderModuleValidationcacheCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkShaderModuleValidationcacheCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT;}
#[cfg(feature = "VK_AMD_shader_info")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkShaderResourceUsageAMD{pub numUsageVgprs:u32,pub numUsedSgprs:u32,pub ldsSizePerLocalWorkGroup:u32,pub ldsUsageSizeInBytes:usize,pub scratchMemUsageInBytes:usize,}
#[cfg(feature = "VK_AMD_shader_info")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkShaderStatisticsInfoAMD{pub shaderStageMask:VkShaderStageFlags,pub resourceUsage:VkShaderResourceUsageAMD,pub numPhysicalVgprs:u32,pub numPhysicalSgprs:u32,pub numAvailableVgprs:u32,pub numAvailableSgprs:u32,pub computeWorkGroupSize:[u32; 3],}
#[cfg(feature = "VK_KHR_shared_presentable_image")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSharedPresentSurfaceCapabilitiesKHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub sharedPresentSupportedUsageFlags:VkImageUsageFlags,}
#[cfg(feature = "VK_KHR_shared_presentable_image")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSharedPresentSurfaceCapabilitiesKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_shared_presentable_image")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSharedPresentSurfaceCapabilitiesKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSparseBufferMemoryBindInfo{pub buffer:VkBuffer,pub bindCount:u32,pub pBinds:*const VkSparseMemoryBind,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSparseImageFormatProperties{pub aspectMask:VkImageAspectFlags,pub imageGranularity:VkExtent3D,pub flags:VkSparseImageFormatFlags,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkSparseImageFormatProperties2=VkSparseImageFormatProperties2KHR;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSparseImageFormatProperties2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub properties:VkSparseImageFormatProperties,}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSparseImageFormatProperties2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSparseImageFormatProperties2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSparseImageMemoryBind{pub subresource:VkImageSubresource,pub offset:VkOffset3D,pub extent:VkExtent3D,pub memory:VkDeviceMemory,pub memoryOffset:VkDeviceSize,pub flags:VkSparseMemoryBindFlags,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSparseImageMemoryBindInfo{pub image:VkImage,pub bindCount:u32,pub pBinds:*const VkSparseImageMemoryBind,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSparseImageMemoryRequirements{pub formatProperties:VkSparseImageFormatProperties,pub imageMipTailFirstLod:u32,pub imageMipTailSize:VkDeviceSize,pub imageMipTailOffset:VkDeviceSize,pub imageMipTailStride:VkDeviceSize,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkSparseImageMemoryRequirements2=VkSparseImageMemoryRequirements2KHR;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSparseImageMemoryRequirements2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub memoryRequirements:VkSparseImageMemoryRequirements,}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSparseImageMemoryRequirements2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSparseImageMemoryRequirements2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSparseImageOpaqueMemoryBindInfo{pub image:VkImage,pub bindCount:u32,pub pBinds:*const VkSparseMemoryBind,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSparseMemoryBind{pub resourceOffset:VkDeviceSize,pub size:VkDeviceSize,pub memory:VkDeviceMemory,pub memoryOffset:VkDeviceSize,pub flags:VkSparseMemoryBindFlags,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSpecializationInfo{pub mapEntryCount:u32,pub pMapEntries:*const VkSpecializationMapEntry,pub dataSize:usize,pub pData:*const core::ffi::c_void,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSpecializationMapEntry{pub constantID:u32,pub offset:u32,pub size:usize,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkStencilOpState{pub failOp:VkStencilOp,pub passOp:VkStencilOp,pub depthFailOp:VkStencilOp,pub compareOp:VkCompareOp,pub compareMask:u32,pub writeMask:u32,pub reference:u32,}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubmitInfo{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub waitSemaphoreCount:u32,pub pWaitSemaphores:*const VkSemaphore,pub pWaitDstStageMask:*const VkPipelineStageFlags,pub commandBufferCount:u32,pub pCommandBuffers:*const VkCommandBuffer,pub signalSemaphoreCount:u32,pub pSignalSemaphores:*const VkSemaphore,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSubmitInfo{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSubmitInfo{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SUBMIT_INFO;}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]pub type VkSubmitInfo2=VkSubmitInfo2KHR;
#[cfg(feature = "VK_KHR_synchronization2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubmitInfo2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkSubmitFlagsKHR,pub waitSemaphoreInfoCount:u32,pub pWaitSemaphoreInfos:*const VkSemaphoreSubmitInfoKHR,pub commandBufferInfoCount:u32,pub pCommandBufferInfos:*const VkCommandBufferSubmitInfoKHR,pub signalSemaphoreInfoCount:u32,pub pSignalSemaphoreInfos:*const VkSemaphoreSubmitInfoKHR,}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSubmitInfo2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_synchronization2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSubmitInfo2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkSubpassBeginInfo=VkSubpassBeginInfoKHR;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubpassBeginInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub contents:VkSubpassContents,}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSubpassBeginInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSubpassBeginInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubpassDependency{pub srcSubpass:u32,pub dstSubpass:u32,pub srcStageMask:VkPipelineStageFlags,pub dstStageMask:VkPipelineStageFlags,pub srcAccessMask:VkAccessFlags,pub dstAccessMask:VkAccessFlags,pub dependencyFlags:VkDependencyFlags,}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkSubpassDependency2=VkSubpassDependency2KHR;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubpassDependency2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub srcSubpass:u32,pub dstSubpass:u32,pub srcStageMask:VkPipelineStageFlags,pub dstStageMask:VkPipelineStageFlags,pub srcAccessMask:VkAccessFlags,pub dstAccessMask:VkAccessFlags,pub dependencyFlags:VkDependencyFlags,pub viewOffset:i32,}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSubpassDependency2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSubpassDependency2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubpassDescription{pub flags:VkSubpassDescriptionFlags,pub pipelineBindPoint:VkPipelineBindPoint,pub inputAttachmentCount:u32,pub pInputAttachments:*const VkAttachmentReference,pub colorAttachmentCount:u32,pub pColorAttachments:*const VkAttachmentReference,pub pResolveAttachments:*const VkAttachmentReference,pub pDepthStencilAttachment:*const VkAttachmentReference,pub preserveAttachmentCount:u32,pub pPreserveAttachments:*const u32,}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkSubpassDescription2=VkSubpassDescription2KHR;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubpassDescription2KHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkSubpassDescriptionFlags,pub pipelineBindPoint:VkPipelineBindPoint,pub viewMask:u32,pub inputAttachmentCount:u32,pub pInputAttachments:*const VkAttachmentReference2KHR,pub colorAttachmentCount:u32,pub pColorAttachments:*const VkAttachmentReference2KHR,pub pResolveAttachments:*const VkAttachmentReference2KHR,pub pDepthStencilAttachment:*const VkAttachmentReference2KHR,pub preserveAttachmentCount:u32,pub pPreserveAttachments:*const u32,}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSubpassDescription2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSubpassDescription2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkSubpassDescriptionDepthStencilResolve=VkSubpassDescriptionDepthStencilResolveKHR;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubpassDescriptionDepthStencilResolveKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub depthResolveMode:VkResolveModeFlagBitsKHR,pub stencilResolveMode:VkResolveModeFlagBitsKHR,pub pDepthStencilResolveAttachment:*const VkAttachmentReference2KHR,}
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSubpassDescriptionDepthStencilResolveKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSubpassDescriptionDepthStencilResolveKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkSubpassEndInfo=VkSubpassEndInfoKHR;
#[cfg(feature = "VK_KHR_create_renderpass2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubpassEndInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSubpassEndInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSubpassEndInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR;}
#[cfg(feature = "VK_EXT_sample_locations")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubpassSampleLocationsEXT{pub subpassIndex:u32,pub sampleLocationsInfo:VkSampleLocationsInfoEXT,}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkSubresourceHostMemcpySize=VkSubresourceHostMemcpySizeEXT;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubresourceHostMemcpySizeEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub size:VkDeviceSize,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSubresourceHostMemcpySizeEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSubresourceHostMemcpySizeEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SUBRESOURCE_HOST_MEMCPY_SIZE_EXT;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubresourceLayout{pub offset:VkDeviceSize,pub size:VkDeviceSize,pub rowPitch:VkDeviceSize,pub arrayPitch:VkDeviceSize,pub depthPitch:VkDeviceSize,}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkSubresourceLayout2=VkSubresourceLayout2KHR;
#[cfg(feature = "VK_EXT_host_image_copy")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubresourceLayout2EXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub subresourceLayout:VkSubresourceLayout,}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSubresourceLayout2EXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSubresourceLayout2EXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSubresourceLayout2EXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SURESOURCE_LAYOUT_2_EXT;}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSubresourceLayout2EXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SURESOURCE_LAYOUT_2_EXT;}
#[cfg(feature = "VK_KHR_maintenance5")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSubresourceLayout2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub subresourceLayout:VkSubresourceLayout,}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSubresourceLayout2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSubresourceLayout2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_KHR;}
#[cfg(feature = "VK_KHR_surface")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSurfaceCapabilitiesKHR{pub minImageCount:u32,pub maxImageCount:u32,pub currentExtent:VkExtent2D,pub minImageExtent:VkExtent2D,pub maxImageExtent:VkExtent2D,pub maxImageArrayLayers:u32,pub supportedTransforms:VkSurfaceTransformFlagsKHR,pub currentTransform:VkSurfaceTransformFlagBitsKHR,pub supportedCompositeAlpha:VkCompositeAlphaFlagsKHR,pub supportedUsageFlags:VkImageUsageFlags,}
#[cfg(feature = "VK_EXT_display_surface_counter")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSurfaceCapabilities2EXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub minImageCount:u32,pub maxImageCount:u32,pub currentExtent:VkExtent2D,pub minImageExtent:VkExtent2D,pub maxImageExtent:VkExtent2D,pub maxImageArrayLayers:u32,pub supportedTransforms:VkSurfaceTransformFlagsKHR,pub currentTransform:VkSurfaceTransformFlagsKHR,pub supportedCompositeAlpha:VkCompositeAlphaFlagsKHR,pub supportedUsageFlags:VkImageUsageFlags,pub supportedSurfaceCounters:VkSurfaceCounterFlagsEXT,}
#[cfg(feature = "VK_EXT_display_surface_counter")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSurfaceCapabilities2EXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_display_surface_counter")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSurfaceCapabilities2EXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT;}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSurfaceCapabilities2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub surfaceCapabilities:VkSurfaceCapabilitiesKHR,}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSurfaceCapabilities2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSurfaceCapabilities2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR;}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub fullScreenExclusiveSupported:VkBool32,}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSurfaceCapabilitiesFullScreenExclusiveEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSurfaceCapabilitiesFullScreenExclusiveEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT;}
#[cfg(feature = "VK_KHR_surface")]#[derive(Debug, Clone, Copy)] #[rustfmt::skip]#[repr(C)]pub struct VkSurfaceFormatKHR{pub format:VkFormat,pub colorSpace:VkColorSpaceKHR,}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSurfaceFormat2KHR{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub surfaceFormat:VkSurfaceFormatKHR,}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSurfaceFormat2KHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSurfaceFormat2KHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR;}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSurfaceFullScreenExclusiveInfoEXT{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub fullScreenExclusive:VkFullScreenExclusiveEXT,}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSurfaceFullScreenExclusiveInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkSurfaceFullScreenExclusiveInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSurfaceFullScreenExclusiveInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT;}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkSurfaceFullScreenExclusiveInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT;}
#[cfg(all(feature = "VK_EXT_full_screen_exclusive", feature = "VK_KHR_win32_surface"))]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub hmonitor:windows::Win32::Graphics::Gdi::HMONITOR,}
#[cfg(all(feature = "VK_EXT_full_screen_exclusive", feature = "VK_KHR_win32_surface"))]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSurfaceFullScreenExclusiveWin32InfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(all(feature = "VK_EXT_full_screen_exclusive", feature = "VK_KHR_win32_surface"))]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSurfaceFullScreenExclusiveWin32InfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT;}
#[cfg(feature = "VK_EXT_display_control")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSwapchainCounterCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub surfaceCounters:VkSurfaceCounterFlagsEXT,}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSwapchainCounterCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSwapchainCounterCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT;}
#[cfg(feature = "VK_KHR_swapchain")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkSwapchainCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkSwapchainCreateFlagsKHR,pub surface:VkSurfaceKHR,pub minImageCount:u32,pub imageFormat:VkFormat,pub imageColorSpace:VkColorSpaceKHR,pub imageExtent:VkExtent2D,pub imageArrayLayers:u32,pub imageUsage:VkImageUsageFlags,pub imageSharingMode:VkSharingMode,pub queueFamilyIndexCount:u32,pub pQueueFamilyIndices:*const u32,pub preTransform:VkSurfaceTransformFlagBitsKHR,pub compositeAlpha:VkCompositeAlphaFlagBitsKHR,pub presentMode:VkPresentModeKHR,pub clipped:VkBool32,pub oldSwapchain:Option<VkSwapchainKHR>,}
#[cfg(feature = "VK_KHR_swapchain")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkSwapchainCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_swapchain")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkSwapchainCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkTextureLODGatherFormatPropertiesAMD{pub sType:VkStructureType,pub pNext:*mut core::ffi::c_void,pub supportsTextureGatherLODBiasAMD:VkBool32,}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]#[rustfmt::skip]unsafe impl crate::VulkanSinkStructure for VkTextureLODGatherFormatPropertiesAMD{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanSinkStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanSinkStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanSinkStructure>(self)}}}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]#[rustfmt::skip]impl crate::TypedVulkanSinkStructure for VkTextureLODGatherFormatPropertiesAMD{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD;}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]pub type VkTimelineSemaphoreSubmitInfo=VkTimelineSemaphoreSubmitInfoKHR;
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkTimelineSemaphoreSubmitInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub waitSemaphoreValueCount:u32,pub pWaitSemaphoreValues:*const u64,pub signalSemaphoreValueCount:u32,pub pSignalSemaphoreValues:*const u64,}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkTimelineSemaphoreSubmitInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkTimelineSemaphoreSubmitInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR;}
#[cfg(feature = "VK_EXT_validation_cache")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkValidationCacheCreateInfoEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkValidationCacheCreateFlagsEXT,pub initialDataSize:usize,pub pInitialData:*const core::ffi::c_void,}
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkValidationCacheCreateInfoEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkValidationCacheCreateInfoEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT;}
#[cfg(feature = "VK_EXT_validation_flags")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkValidationFlagsEXT{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub disabledValidationCheckCount:u32,pub pDisabledValidationChecks:*mut VkValidationCheckEXT,}
#[cfg(feature = "VK_EXT_validation_flags")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkValidationFlagsEXT{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_EXT_validation_flags")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkValidationFlagsEXT{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT;}
#[derive(Debug, Clone, Copy)] #[rustfmt::skip]#[repr(C)]pub struct VkVertexInputAttributeDescription{pub location:u32,pub binding:u32,pub format:VkFormat,pub offset:u32,}
#[derive(Debug, Clone, Copy)] #[rustfmt::skip]#[repr(C)]pub struct VkVertexInputBindingDescription{pub binding:u32,pub stride:u32,pub inputRate:VkVertexInputRate,}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]pub type VkVertexInputBindingDivisorDescription=VkVertexInputBindingDivisorDescriptionKHR;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkVertexInputBindingDivisorDescriptionEXT{pub binding:u32,pub divisor:u32,}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkVertexInputBindingDivisorDescriptionKHR{pub binding:u32,pub divisor:u32,}
#[cfg(feature = "VK_NN_vi_surface")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkViSurfaceCreateInfoNN{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkViSurfaceCreateFlagsNN,pub window:*mut core::ffi::c_void,}
#[cfg(feature = "VK_NN_vi_surface")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkViSurfaceCreateInfoNN{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_NN_vi_surface")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkViSurfaceCreateInfoNN{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkViewport{pub x:core::ffi::c_float,pub y:core::ffi::c_float,pub width:core::ffi::c_float,pub height:core::ffi::c_float,pub minDepth:core::ffi::c_float,pub maxDepth:core::ffi::c_float,}
#[cfg(feature = "VK_NV_viewport_swizzle")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkViewportSwizzleNV{pub x:VkViewportCoordinateSwizzleNV,pub y:VkViewportCoordinateSwizzleNV,pub z:VkViewportCoordinateSwizzleNV,pub w:VkViewportCoordinateSwizzleNV,}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkViewportWScalingNV{pub xcoeff:core::ffi::c_float,pub ycoeff:core::ffi::c_float,}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]pub type VkVkExportMemoryAllocateInfo=VkVkExportMemoryAllocateInfoKHR;
#[cfg(feature = "VK_KHR_external_memory")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkVkExportMemoryAllocateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub handleTypes:VkExternalMemoryHandleTypeFlagsKHR,}
#[cfg(feature = "VK_KHR_external_memory")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkVkExportMemoryAllocateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_external_memory")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkVkExportMemoryAllocateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR;}
#[cfg(feature = "VK_KHR_wayland_surface")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkWaylandSurfaceCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkWaylandSurfaceCreateFlagsKHR,pub display:*mut core::ffi::c_void,pub surface:*mut core::ffi::c_void,}
#[cfg(feature = "VK_KHR_wayland_surface")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkWaylandSurfaceCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_wayland_surface")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkWaylandSurfaceCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR;}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub acquireCount:u32,pub pAcquireSyncs:*const VkDeviceMemory,pub pAcquireKeys:*const u64,pub pAcquireTimeouts:*const u32,pub releaseCount:u32,pub pReleaseSyncs:*const VkDeviceMemory,pub pReleaseKeys:*const u64,}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkWin32KeyedMutexAcquireReleaseInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkWin32KeyedMutexAcquireReleaseInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR;}
#[cfg(feature = "VK_KHR_win32_surface")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkWin32SurfaceCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkWin32SurfaceCreateFlagsKHR,pub hinstance:windows::Win32::Foundation::HINSTANCE,pub hwnd:windows::Win32::Foundation::HWND,}
#[cfg(feature = "VK_KHR_win32_surface")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkWin32SurfaceCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_win32_surface")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkWin32SurfaceCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR;}
#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkWriteDescriptorSet{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub dstSet:VkDescriptorSet,pub dstBinding:u32,pub dstArrayElement:u32,pub descriptorCount:u32,pub descriptorType:VkDescriptorType,pub pImageInfo:*const VkDescriptorImageInfo,pub pBufferInfo:*const VkDescriptorBufferInfo,pub pTexelBufferView:*const VkBufferView,}
#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkWriteDescriptorSet{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[rustfmt::skip]impl crate::TypedVulkanStructure for VkWriteDescriptorSet{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;}
#[cfg(feature = "VK_EXT_hdr_metadata")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkXYColorEXT{pub x:core::ffi::c_float,pub y:core::ffi::c_float,}
#[cfg(feature = "VK_KHR_xcb_surface")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkXcbSurfaceCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkXcbSurfaceCreateFlagsKHR,pub connection:*mut xcb::ffi::xcb_connection_t,pub window:xcb::x::Window,}
#[cfg(feature = "VK_KHR_xcb_surface")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkXcbSurfaceCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_xcb_surface")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkXcbSurfaceCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR;}
#[cfg(feature = "VK_KHR_xlib_surface")]#[derive(Debug, Clone)] #[rustfmt::skip]#[repr(C)]pub struct VkXlibSurfaceCreateInfoKHR{pub sType:VkStructureType,pub pNext:*const core::ffi::c_void,pub flags:VkXlibSurfaceCreateFlagsKHR,pub dpy:*mut x11::xlib::Display,pub window:x11::xlib::Window,}
#[cfg(feature = "VK_KHR_xlib_surface")]#[rustfmt::skip]unsafe impl crate::VulkanStructure for VkXlibSurfaceCreateInfoKHR{#[inline(always)]fn as_generic(&self)->&crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&Self,&crate::GenericVulkanStructure>(self)}}#[inline(always)]fn as_generic_mut(&mut self)->&mut crate::GenericVulkanStructure{unsafe{core::mem::transmute::<&mut Self, &mut crate::GenericVulkanStructure>(self)}}}
#[cfg(feature = "VK_KHR_xlib_surface")]#[rustfmt::skip]impl crate::TypedVulkanStructure for VkXlibSurfaceCreateInfoKHR{const TYPE: VkStructureType=VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR;}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCopyImageToImage(pub unsafe extern "system" fn(device:VkDevice,pCopyImageToImageInfo:*const VkCopyImageToImageInfoEXT)->VkResult);
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCopyImageToImage{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCopyImageToImage{const NAME_CSTR:&'static core::ffi::CStr=c"vkCopyImageToImage";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCopyImageToImage{const STATIC:Self=Self(vkCopyImageToImage);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCopyImageToMemory(pub unsafe extern "system" fn(device:VkDevice,pCopyImageToMemoryInfo:*const VkCopyImageToMemoryInfoEXT)->VkResult);
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCopyImageToMemory{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCopyImageToMemory{const NAME_CSTR:&'static core::ffi::CStr=c"vkCopyImageToMemory";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCopyImageToMemory{const STATIC:Self=Self(vkCopyImageToMemory);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCopyMemoryToImage(pub unsafe extern "system" fn(device:VkDevice,pCopyMemoryToImageInfo:*const VkCopyMemoryToImageInfoEXT)->VkResult);
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCopyMemoryToImage{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCopyMemoryToImage{const NAME_CSTR:&'static core::ffi::CStr=c"vkCopyMemoryToImage";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCopyMemoryToImage{const STATIC:Self=Self(vkCopyMemoryToImage);}
#[cfg(feature = "Allow1_2APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateRenderPass2(pub unsafe extern "system" fn(device:VkDevice,pCreateInfo:*const VkRenderPassCreateInfo2KHR,pAllocator:*const VkAllocationCallbacks,pRenderPass:*mut VkRenderPass)->VkResult);
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateRenderPass2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateRenderPass2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateRenderPass2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCreateRenderPass2{const STATIC:Self=Self(vkCreateRenderPass2);}
#[cfg(feature = "Allow1_1APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateSamplerYcbcrConversion(pub unsafe extern "system" fn(device:VkDevice,pCreateInfo:VkSamplerYcbcrConversionCreateInfoKHR,pAllocator:*const VkAllocationCallbacks,pYcbcrConversion:*mut VkSamplerYcbcrConversionKHR)->VkResult);
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateSamplerYcbcrConversion{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateSamplerYcbcrConversion{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateSamplerYcbcrConversion";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_1APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCreateSamplerYcbcrConversion{const STATIC:Self=Self(vkCreateSamplerYcbcrConversion);}
#[cfg(feature = "Allow1_1APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkDestroySamplerYcbcrConversion(pub unsafe extern "system" fn(device:VkDevice,ycbcrConversion:VkSamplerYcbcrConversionKHR,pAllocator:*const VkAllocationCallbacks));
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkDestroySamplerYcbcrConversion{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkDestroySamplerYcbcrConversion{const NAME_CSTR:&'static core::ffi::CStr=c"vkDestroySamplerYcbcrConversion";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_1APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkDestroySamplerYcbcrConversion{const STATIC:Self=Self(vkDestroySamplerYcbcrConversion);}
#[cfg(feature = "Allow1_2APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetBufferDeviceAddress(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkBufferDeviceAddressInfoKHR)->VkDeviceAddress);
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetBufferDeviceAddress{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetBufferDeviceAddress{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetBufferDeviceAddress";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkGetBufferDeviceAddress{const STATIC:Self=Self(vkGetBufferDeviceAddress);}
#[cfg(feature = "Allow1_2APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetBufferOpaqueCaptureAddress(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkBufferDeviceAddressInfoKHR)->u64);
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetBufferOpaqueCaptureAddress{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetBufferOpaqueCaptureAddress{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetBufferOpaqueCaptureAddress";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkGetBufferOpaqueCaptureAddress{const STATIC:Self=Self(vkGetBufferOpaqueCaptureAddress);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceBufferMemoryRequirements(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkDeviceBufferMemoryRequirementsKHR,pMemoryRequirements:*mut VkMemoryRequirements2KHR));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceBufferMemoryRequirements{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceBufferMemoryRequirements{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceBufferMemoryRequirements";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkGetDeviceBufferMemoryRequirements{const STATIC:Self=Self(vkGetDeviceBufferMemoryRequirements);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceImageMemoryREquirements(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkDeviceImageMemoryRequirementsKHR,pMemoryRequirements:*mut VkMemoryRequirements2KHR));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceImageMemoryREquirements{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceImageMemoryREquirements{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceImageMemoryREquirements";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkGetDeviceImageMemoryREquirements{const STATIC:Self=Self(vkGetDeviceImageMemoryREquirements);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceImageSparseMemoryRequirements(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkDeviceImageMemoryRequirementsKHR,pSparseMemoryRequirementsCount:*mut u32,pSparseMemoryRequirements:*mut VkSparseImageMemoryRequirements2KHR));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceImageSparseMemoryRequirements{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceImageSparseMemoryRequirements{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceImageSparseMemoryRequirements";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkGetDeviceImageSparseMemoryRequirements{const STATIC:Self=Self(vkGetDeviceImageSparseMemoryRequirements);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceImageSubresourceLayout(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkDeviceImageSubresourceInfoKHR,pLayout:*mut VkSubresourceLayout2KHR));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceImageSubresourceLayout{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceImageSubresourceLayout{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceImageSubresourceLayout";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkGetDeviceImageSubresourceLayout{const STATIC:Self=Self(vkGetDeviceImageSubresourceLayout);}
#[cfg(feature = "Allow1_2APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceMemoryOpaqueCaptureAddress(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkDeviceMemoryOpaqueCaptureAddressInfoKHR)->u64);
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceMemoryOpaqueCaptureAddress{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceMemoryOpaqueCaptureAddress{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceMemoryOpaqueCaptureAddress";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkGetDeviceMemoryOpaqueCaptureAddress{const STATIC:Self=Self(vkGetDeviceMemoryOpaqueCaptureAddress);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetImageSubresourceLayout2(pub unsafe extern "system" fn(device:VkDevice,image:VkImage,pSubresource:*const VkImageSubresource2KHR,pLayout:*mut VkSubresourceLayout2KHR));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetImageSubresourceLayout2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetImageSubresourceLayout2{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetImageSubresourceLayout2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkGetImageSubresourceLayout2{const STATIC:Self=Self(vkGetImageSubresourceLayout2);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetRenderingAreaGranularity(pub unsafe extern "system" fn(device:VkDevice,pRenderingAreaInfo:*const VkRenderingAreaInfoKHR,pGranularity:*mut VkExtent2D));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetRenderingAreaGranularity{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetRenderingAreaGranularity{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetRenderingAreaGranularity";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkGetRenderingAreaGranularity{const STATIC:Self=Self(vkGetRenderingAreaGranularity);}
#[cfg(feature = "Allow1_2APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetSemaphoreCounterValue(pub unsafe extern "system" fn(device:VkDevice,semaphore:VkSemaphore,pValue:*mut u64)->VkResult);
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetSemaphoreCounterValue{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetSemaphoreCounterValue{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetSemaphoreCounterValue";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkGetSemaphoreCounterValue{const STATIC:Self=Self(vkGetSemaphoreCounterValue);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkMapMemory2(pub unsafe extern "system" fn(device:VkDevice,pMemoryMapInfo:*const VkMemoryMapInfoKHR,ppData:*mut *mut core::ffi::c_void)->VkResult);
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkMapMemory2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkMapMemory2{const NAME_CSTR:&'static core::ffi::CStr=c"vkMapMemory2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkMapMemory2{const STATIC:Self=Self(vkMapMemory2);}
#[cfg(feature = "Allow1_2APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkSignalSemaphore(pub unsafe extern "system" fn(device:VkDevice,pSignalInfo:*const VkSemaphoreSignalInfoKHR)->VkResult);
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkSignalSemaphore{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkSignalSemaphore{const NAME_CSTR:&'static core::ffi::CStr=c"vkSignalSemaphore";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkSignalSemaphore{const STATIC:Self=Self(vkSignalSemaphore);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkTransitionImageLayout(pub unsafe extern "system" fn(device:VkDevice,transitionCount:u32,pTransitions:*const VkHostImageLayoutTransitionInfoEXT)->VkResult);
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkTransitionImageLayout{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkTransitionImageLayout{const NAME_CSTR:&'static core::ffi::CStr=c"vkTransitionImageLayout";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkTransitionImageLayout{const STATIC:Self=Self(vkTransitionImageLayout);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkUnmapMemory2(pub unsafe extern "system" fn(device:VkDevice,pMemoryUnmapInfo:*const VkMemoryUnmapInfoKHR)->VkResult);
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkUnmapMemory2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkUnmapMemory2{const NAME_CSTR:&'static core::ffi::CStr=c"vkUnmapMemory2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkUnmapMemory2{const STATIC:Self=Self(vkUnmapMemory2);}
#[cfg(feature = "Allow1_2APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkWaitSemaphores(pub unsafe extern "system" fn(device:VkDevice,pWaitInfo:*const VkSemaphoreWaitInfoKHR,timeout:u64)->VkResult);
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkWaitSemaphores{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkWaitSemaphores{const NAME_CSTR:&'static core::ffi::CStr=c"vkWaitSemaphores";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkWaitSemaphores{const STATIC:Self=Self(vkWaitSemaphores);}
#[cfg(feature = "VK_EXT_acquire_drm_display")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkAcquireDrmDisplayEXT(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,drmFd:i32,display:VkDisplayKHR)->VkResult);
#[cfg(feature = "VK_EXT_acquire_drm_display")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkAcquireDrmDisplayEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_acquire_drm_display")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkAcquireDrmDisplayEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkAcquireDrmDisplayEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkAcquireFullScreenExclusiveModeEXT(pub unsafe extern "system" fn(device:VkDevice,swapchain:VkSwapchainKHR)->VkResult);
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkAcquireFullScreenExclusiveModeEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkAcquireFullScreenExclusiveModeEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkAcquireFullScreenExclusiveModeEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NV_acquire_winrt_display")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkAcquireWinrtDisplayNV(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,display:VkDisplayKHR)->VkResult);
#[cfg(feature = "VK_NV_acquire_winrt_display")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkAcquireWinrtDisplayNV{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NV_acquire_winrt_display")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkAcquireWinrtDisplayNV{const NAME_CSTR:&'static core::ffi::CStr=c"vkAcquireWinrtDisplayNV";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_acquire_xlib_display")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkAcquireXlibDisplayEXT(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,dpy:*mut x11::xlib::Display,display:VkDisplayKHR)->VkResult);
#[cfg(feature = "VK_EXT_acquire_xlib_display")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkAcquireXlibDisplayEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_acquire_xlib_display")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkAcquireXlibDisplayEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkAcquireXlibDisplayEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCopyImageToImageEXT(pub unsafe extern "system" fn(device:VkDevice,pCopyImageToImageInfo:*const VkCopyImageToImageInfoEXT)->VkResult);
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCopyImageToImageEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCopyImageToImageEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCopyImageToImageEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCopyImageToMemoryEXT(pub unsafe extern "system" fn(device:VkDevice,pCopyImageToMemoryInfo:*const VkCopyImageToMemoryInfoEXT)->VkResult);
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCopyImageToMemoryEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCopyImageToMemoryEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCopyImageToMemoryEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCopyMemoryToImageEXT(pub unsafe extern "system" fn(device:VkDevice,pCopyMemoryToImageInfo:*const VkCopyMemoryToImageInfoEXT)->VkResult);
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCopyMemoryToImageEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCopyMemoryToImageEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCopyMemoryToImageEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_MVK_ios_surface")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateIOSSurfaceMVK(pub unsafe extern "system" fn(instance:VkInstance,pCreateInfo:*const VkIOSSurfaceCreateInfoMVK,pAllocator:*const VkAllocationCallbacks,pSurface:*mut VkSurfaceKHR)->VkResult);
#[cfg(feature = "VK_MVK_ios_surface")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateIOSSurfaceMVK{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_MVK_ios_surface")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateIOSSurfaceMVK{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateIOSSurfaceMVK";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_MVK_ios_surface")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCreateIOSSurfaceMVK{const STATIC:Self=Self(vkCreateIOSSurfaceMVK);}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateIndirectCommandsLayoutNVX(pub unsafe extern "system" fn(device:VkDevice,pCreateInfo:*const VkIndirectCommandsLayoutCreateInfoNVX,pAllocator:*const VkAllocationCallbacks,pIndirectCommandsLayout:*mut VkIndirectCommandsLayoutNVX)->VkResult);
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateIndirectCommandsLayoutNVX{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateIndirectCommandsLayoutNVX{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateIndirectCommandsLayoutNVX";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_MVK_macos_surface")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateMacOSSurfaceMVK(pub unsafe extern "system" fn(instance:VkInstance,pCreateInfo:*const VkMacOSSurfaceCreateInfoMVK,pAllocator:*const VkAllocationCallbacks,pSurface:*mut VkSurfaceKHR)->VkResult);
#[cfg(feature = "VK_MVK_macos_surface")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateMacOSSurfaceMVK{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_MVK_macos_surface")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateMacOSSurfaceMVK{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateMacOSSurfaceMVK";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_MVK_macos_surface")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCreateMacOSSurfaceMVK{const STATIC:Self=Self(vkCreateMacOSSurfaceMVK);}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateObjectTableNVX(pub unsafe extern "system" fn(device:VkDevice,pCreateInfo:*const VkObjectTableCreateInfoNVX,pAllocator:*const VkAllocationCallbacks,pObjectTable:*mut VkObjectTableNVX)->VkResult);
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateObjectTableNVX{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateObjectTableNVX{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateObjectTableNVX";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateRenderPass2KHR(pub unsafe extern "system" fn(device:VkDevice,pCreateInfo:*const VkRenderPassCreateInfo2KHR,pAllocator:*const VkAllocationCallbacks,pRenderPass:*mut VkRenderPass)->VkResult);
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateRenderPass2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateRenderPass2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateRenderPass2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateSamplerYcbcrConversionKHR(pub unsafe extern "system" fn(device:VkDevice,pCreateInfo:VkSamplerYcbcrConversionCreateInfoKHR,pAllocator:*const VkAllocationCallbacks,pYcbcrConversion:*mut VkSamplerYcbcrConversionKHR)->VkResult);
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateSamplerYcbcrConversionKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateSamplerYcbcrConversionKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateSamplerYcbcrConversionKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_display_swapchain")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateSharedSwapchainKHR(pub unsafe extern "system" fn(device:VkDevice,swapchainCount:u32,pCreateInfos:*const VkSwapchainCreateInfoKHR,pAllocator:*const VkAllocationCallbacks,pSwapchains:*mut VkSwapchainKHR)->VkResult);
#[cfg(feature = "VK_KHR_display_swapchain")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateSharedSwapchainKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_display_swapchain")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateSharedSwapchainKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateSharedSwapchainKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_validation_cache")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateValidationCacheEXT(pub unsafe extern "system" fn(device:VkDevice,pCreateInfo:*const VkValidationCacheCreateInfoEXT,pAllocator:*const VkAllocationCallbacks,pValidationCache:*mut VkValidationCacheEXT)->VkResult);
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateValidationCacheEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateValidationCacheEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateValidationCacheEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NN_vi_surface")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCreateViSurfaceNN(pub unsafe extern "system" fn(instance:VkInstance,pCreateInfo:*const VkViSurfaceCreateInfoNN,pAllocator:*const VkAllocationCallbacks,pSurface:*mut VkSurfaceKHR)->VkResult);
#[cfg(feature = "VK_NN_vi_surface")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCreateViSurfaceNN{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NN_vi_surface")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCreateViSurfaceNN{const NAME_CSTR:&'static core::ffi::CStr=c"vkCreateViSurfaceNN";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NN_vi_surface")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCreateViSurfaceNN{const STATIC:Self=Self(vkCreateViSurfaceNN);}
#[cfg(feature = "VK_EXT_debug_marker")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkDebugMarkerSetObjectNameEXT(pub unsafe extern "system" fn(device:VkDevice,pNameInfo:*const VkDebugMarkerObjectNameInfoEXT)->VkResult);
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkDebugMarkerSetObjectNameEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkDebugMarkerSetObjectNameEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkDebugMarkerSetObjectNameEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkDebugMarkerSetObjectTagEXT(pub unsafe extern "system" fn(device:VkDevice,pTagInfo:*const VkDebugMarkerObjectTagInfoEXT)->VkResult);
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkDebugMarkerSetObjectTagEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkDebugMarkerSetObjectTagEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkDebugMarkerSetObjectTagEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkDestroyIndirectCommandsLayoutNVX(pub unsafe extern "system" fn(device:VkDevice,indirectCommandsLayout:VkIndirectCommandsLayoutNVX,pAllocator:*const VkAllocationCallbacks));
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkDestroyIndirectCommandsLayoutNVX{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkDestroyIndirectCommandsLayoutNVX{const NAME_CSTR:&'static core::ffi::CStr=c"vkDestroyIndirectCommandsLayoutNVX";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkDestroyObjectTableNVX(pub unsafe extern "system" fn(device:VkDevice,objectTable:VkObjectTableNVX,pAllocator:*const VkAllocationCallbacks));
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkDestroyObjectTableNVX{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkDestroyObjectTableNVX{const NAME_CSTR:&'static core::ffi::CStr=c"vkDestroyObjectTableNVX";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkDestroySamplerYcbcrConversionKHR(pub unsafe extern "system" fn(device:VkDevice,ycbcrConversion:VkSamplerYcbcrConversionKHR,pAllocator:*const VkAllocationCallbacks));
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkDestroySamplerYcbcrConversionKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkDestroySamplerYcbcrConversionKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkDestroySamplerYcbcrConversionKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_validation_cache")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkDestroyValidationCacheEXT(pub unsafe extern "system" fn(device:VkDevice,validationCache:VkValidationCacheEXT,pAllocator:*const VkAllocationCallbacks));
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkDestroyValidationCacheEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkDestroyValidationCacheEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkDestroyValidationCacheEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_display_control")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkDisplayPowerControlEXT(pub unsafe extern "system" fn(device:VkDevice,display:VkDisplayKHR,pDisplayPowerInfo:*const VkDisplayPowerInfoEXT)->VkResult);
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkDisplayPowerControlEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkDisplayPowerControlEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkDisplayPowerControlEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkExportMetalObjectsEXT(pub unsafe extern "system" fn(device:VkDevice,pMetalObjectsInfo:*mut VkExportMetalObjectsInfoEXT));
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkExportMetalObjectsEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_metal_objects")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkExportMetalObjectsEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkExportMetalObjectsEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetBufferDeviceAddressKHR(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkBufferDeviceAddressInfoKHR)->VkDeviceAddress);
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetBufferDeviceAddressKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetBufferDeviceAddressKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetBufferDeviceAddressKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetBufferOpaqueCaptureAddressKHR(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkBufferDeviceAddressInfoKHR)->u64);
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetBufferOpaqueCaptureAddressKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetBufferOpaqueCaptureAddressKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetBufferOpaqueCaptureAddressKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkBufferCaptureDescriptorDataInfoEXT,pData:*mut core::ffi::c_void)->VkResult);
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetBufferOpaqueCaptureDescriptorDataEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDescriptorEXT(pub unsafe extern "system" fn(device:VkDevice,pDescriptorInfo:*const VkDescriptorGetInfoEXT,dataSize:usize,pDescriptor:*mut core::ffi::c_void));
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDescriptorEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDescriptorEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDescriptorEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDescriptorSetLayoutBindingOffsetEXT(pub unsafe extern "system" fn(device:VkDevice,layout:VkDescriptorSetLayout,binding:u32,pOffset:*mut VkDeviceSize));
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDescriptorSetLayoutBindingOffsetEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDescriptorSetLayoutBindingOffsetEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDescriptorSetLayoutBindingOffsetEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDescriptorSetLayoutSizeEXT(pub unsafe extern "system" fn(device:VkDevice,layout:VkDescriptorSetLayout,pLayoutSizeInBytes:*mut VkDeviceSize));
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDescriptorSetLayoutSizeEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDescriptorSetLayoutSizeEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDescriptorSetLayoutSizeEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceBufferMemoryRequirementsKHR(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkDeviceBufferMemoryRequirementsKHR,pMemoryRequirements:*mut VkMemoryRequirements2KHR));
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceBufferMemoryRequirementsKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceBufferMemoryRequirementsKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceBufferMemoryRequirementsKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(all(feature = "VK_EXT_full_screen_exclusive", feature = "VK_KHR_device_group"))]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceGroupSurfacePresentModes2EXT(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,pSurfaceInfo:*const VkPhysicalDeviceSurfaceInfo2KHR,pModes:*mut VkDeviceGroupPresentModeFlagsKHR)->VkResult);
#[cfg(all(feature = "VK_EXT_full_screen_exclusive", feature = "VK_KHR_device_group"))]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceGroupSurfacePresentModes2EXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(all(feature = "VK_EXT_full_screen_exclusive", feature = "VK_KHR_device_group"))]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceGroupSurfacePresentModes2EXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceGroupSurfacePresentModes2EXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceImageMemoryREquirementsKHR(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkDeviceImageMemoryRequirementsKHR,pMemoryRequirements:*mut VkMemoryRequirements2KHR));
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceImageMemoryREquirementsKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceImageMemoryREquirementsKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceImageMemoryREquirementsKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceImageSparseMemoryRequirementsKHR(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkDeviceImageMemoryRequirementsKHR,pSparseMemoryRequirementsCount:*mut u32,pSparseMemoryRequirements:*mut VkSparseImageMemoryRequirements2KHR));
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceImageSparseMemoryRequirementsKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance4")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceImageSparseMemoryRequirementsKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceImageSparseMemoryRequirementsKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceImageSubresourceLayoutKHR(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkDeviceImageSubresourceInfoKHR,pLayout:*mut VkSubresourceLayout2KHR));
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceImageSubresourceLayoutKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceImageSubresourceLayoutKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceImageSubresourceLayoutKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkDeviceMemoryOpaqueCaptureAddressInfoKHR)->u64);
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_buffer_device_address")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDeviceMemoryOpaqueCaptureAddressKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_acquire_drm_display")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetDrmDisplayEXT(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,drmFd:i32,connectorId:u32,pDisplay:*mut VkDisplayKHR)->VkResult);
#[cfg(feature = "VK_EXT_acquire_drm_display")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetDrmDisplayEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_acquire_drm_display")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetDrmDisplayEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetDrmDisplayEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetImageDrmFormatModifierPropertiesEXT(pub unsafe extern "system" fn(device:VkDevice,image:VkImage,pProperites:*mut VkImageDrmFormatModifierPropertiesEXT)->VkResult);
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetImageDrmFormatModifierPropertiesEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetImageDrmFormatModifierPropertiesEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetImageDrmFormatModifierPropertiesEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetImageOpaqueCaptureDescriptorDataEXT(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkImageCaptureDescriptorDataInfoEXT,pData:*mut core::ffi::c_void)->VkResult);
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetImageOpaqueCaptureDescriptorDataEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetImageOpaqueCaptureDescriptorDataEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetImageOpaqueCaptureDescriptorDataEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetImageSubresourceLayout2EXT(pub unsafe extern "system" fn(device:VkDevice,image:VkImage,pSubresource:*const VkImageSubresource2EXT,pLayout:*mut VkSubresourceLayout2EXT));
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetImageSubresourceLayout2EXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetImageSubresourceLayout2EXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetImageSubresourceLayout2EXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetImageSubresourceLayout2KHR(pub unsafe extern "system" fn(device:VkDevice,image:VkImage,pSubresource:*const VkImageSubresource2KHR,pLayout:*mut VkSubresourceLayout2KHR));
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetImageSubresourceLayout2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetImageSubresourceLayout2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetImageSubresourceLayout2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkImageViewCaptureDescriptorDataInfoEXT,pData:*mut core::ffi::c_void)->VkResult);
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetImageViewOpaqueCaptureDescriptorDataEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_external_memory_host")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetMemoryHostPointerPropertiesEXT(pub unsafe extern "system" fn(device:VkDevice,handleType:VkExternalMemoryHandleTypeFlagsKHR,pHostPointer:*const core::ffi::c_void,pMemoryHostPointerProperties:*mut VkMemoryHostPointerPropertiesEXT)->VkResult);
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetMemoryHostPointerPropertiesEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_external_memory_host")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetMemoryHostPointerPropertiesEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetMemoryHostPointerPropertiesEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_GOOGLE_display_timing")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetPastPresentationTimingGOOGLE(pub unsafe extern "system" fn(device:VkDevice,swapchain:VkSwapchainKHR,pPresentationTimingCount:*mut u32,pPresentationTimings:*mut VkPastPresentationTimingGOOGLE)->VkResult);
#[cfg(feature = "VK_GOOGLE_display_timing")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetPastPresentationTimingGOOGLE{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_GOOGLE_display_timing")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetPastPresentationTimingGOOGLE{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetPastPresentationTimingGOOGLE";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,pFeatures:*mut VkDeviceGeneratedCommandsFeaturesNVX,pLimits:*mut VkDeviceGeneratedCommandsLimitsNVX));
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_sample_locations")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,samples:VkSampleCountFlags,pMultisampleProperties:*mut VkMultisamplePropertiesEXT));
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetPhysicalDeviceMultisamplePropertiesEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_display_surface_counter")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,surface:VkSurfaceKHR,pSurfaceCapabilities:*mut VkSurfaceCapabilities2EXT)->VkResult);
#[cfg(feature = "VK_EXT_display_surface_counter")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_display_surface_counter")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetPhysicalDeviceSurfaceCapabilities2EXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,pSurfaceInfo:*const VkPhysicalDeviceSurfaceInfo2KHR,pPresentModeCount:*mut u32,pPresentModes:*mut VkPresentModeKHR)->VkResult);
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetPhysicalDeviceSurfacePresentModes2EXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_acquire_xlib_display")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetRandROutputDisplayEXT(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,dpy:*mut x11::xlib::Display,rrOutput:x11::xrandr::RROutput,pDisplay:*mut VkDisplayKHR)->VkResult);
#[cfg(feature = "VK_EXT_acquire_xlib_display")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetRandROutputDisplayEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_acquire_xlib_display")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetRandROutputDisplayEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetRandROutputDisplayEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_GOOGLE_display_timing")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetRefreshCycleDurationGOOGLE(pub unsafe extern "system" fn(device:VkDevice,swapchain:VkSwapchainKHR,pDisplayTimingProperties:*mut VkRefreshCycleDurationGOOGLE)->VkResult);
#[cfg(feature = "VK_GOOGLE_display_timing")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetRefreshCycleDurationGOOGLE{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_GOOGLE_display_timing")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetRefreshCycleDurationGOOGLE{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetRefreshCycleDurationGOOGLE";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetRenderingAreaGranularityKHR(pub unsafe extern "system" fn(device:VkDevice,pRenderingAreaInfo:*const VkRenderingAreaInfoKHR,pGranularity:*mut VkExtent2D));
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetRenderingAreaGranularityKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetRenderingAreaGranularityKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetRenderingAreaGranularityKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT(pub unsafe extern "system" fn(device:VkDevice,pInfo:*const VkSamplerCaptureDescriptorDataInfoEXT,pData:*mut core::ffi::c_void)->VkResult);
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetSamplerOpaqueCaptureDescriptorDataEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetSemaphoreCounterValueKHR(pub unsafe extern "system" fn(device:VkDevice,semaphore:VkSemaphore,pValue:*mut u64)->VkResult);
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetSemaphoreCounterValueKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetSemaphoreCounterValueKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetSemaphoreCounterValueKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_AMD_shader_info")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetShaderInfoAMD(pub unsafe extern "system" fn(device:VkDevice,pipeline:VkPipeline,shaderStage:VkShaderStageFlags,infoType:VkShaderInfoTypeAMD,pInfoSize:*mut usize,pInfo:*mut core::ffi::c_void)->VkResult);
#[cfg(feature = "VK_AMD_shader_info")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetShaderInfoAMD{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_AMD_shader_info")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetShaderInfoAMD{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetShaderInfoAMD";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_display_control")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetSwapchainCounterEXT(pub unsafe extern "system" fn(device:VkDevice,swapchain:VkSwapchainKHR,counter:VkSurfaceCounterFlagsEXT,pCounterValue:*mut u64)->VkResult);
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetSwapchainCounterEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetSwapchainCounterEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetSwapchainCounterEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_shared_presentable_image")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetSwapchainStatusKHR(pub unsafe extern "system" fn(device:VkDevice,swapchain:VkSwapchainKHR)->VkResult);
#[cfg(feature = "VK_KHR_shared_presentable_image")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetSwapchainStatusKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_shared_presentable_image")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetSwapchainStatusKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetSwapchainStatusKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_validation_cache")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetValidationCacheDataEXT(pub unsafe extern "system" fn(device:VkDevice,validationCache:VkValidationCacheEXT,pDataSize:*mut usize,pData:*mut core::ffi::c_void)->VkResult);
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetValidationCacheDataEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetValidationCacheDataEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetValidationCacheDataEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NV_acquire_winrt_display")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkGetWinrtDisplayNV(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,deviceRelativeId:u32,pDisplay:*mut VkDisplayKHR)->VkResult);
#[cfg(feature = "VK_NV_acquire_winrt_display")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkGetWinrtDisplayNV{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NV_acquire_winrt_display")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkGetWinrtDisplayNV{const NAME_CSTR:&'static core::ffi::CStr=c"vkGetWinrtDisplayNV";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_map_memory2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkMapMemory2KHR(pub unsafe extern "system" fn(device:VkDevice,pMemoryMapInfo:*const VkMemoryMapInfoKHR,ppData:*mut *mut core::ffi::c_void)->VkResult);
#[cfg(feature = "VK_KHR_map_memory2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkMapMemory2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_map_memory2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkMapMemory2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkMapMemory2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_validation_cache")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkMergeValidationCachesEXT(pub unsafe extern "system" fn(device:VkDevice,dstCache:VkValidationCacheEXT,srcCacheCount:u32,pSrcCaches:*const VkValidationCacheEXT)->VkResult);
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkMergeValidationCachesEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_validation_cache")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkMergeValidationCachesEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkMergeValidationCachesEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_display_control")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkRegisterDeviceEventEXT(pub unsafe extern "system" fn(device:VkDevice,pDeviceEventInfo:*const VkDeviceEventInfoEXT,pAllocator:*const VkAllocationCallbacks,pFence:*mut VkFence)->VkResult);
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkRegisterDeviceEventEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkRegisterDeviceEventEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkRegisterDeviceEventEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_display_control")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkRegisterDisplayEventEXT(pub unsafe extern "system" fn(device:VkDevice,pDisplayEventInfo:*const VkDisplayEventInfoEXT,pAllocator:*const VkAllocationCallbacks,pFence:*mut VkFence)->VkResult);
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkRegisterDisplayEventEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_display_control")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkRegisterDisplayEventEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkRegisterDisplayEventEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkRegisterObjectsNVX(pub unsafe extern "system" fn(device:VkDevice,objectTable:VkObjectTableNVX,objectCount:u32,ppObjectTableEntries:*const *const VkObjectTableEntryNVX,pObjectIndices:*const u32)->VkResult);
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkRegisterObjectsNVX{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkRegisterObjectsNVX{const NAME_CSTR:&'static core::ffi::CStr=c"vkRegisterObjectsNVX";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_direct_mode_display")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkReleaseDisplayEXT(pub unsafe extern "system" fn(physicalDevice:VkPhysicalDevice,display:VkDisplayKHR)->VkResult);
#[cfg(feature = "VK_EXT_direct_mode_display")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkReleaseDisplayEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_direct_mode_display")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkReleaseDisplayEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkReleaseDisplayEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkReleaseFullScreenExclusiveModeEXT(pub unsafe extern "system" fn(device:VkDevice,swapchain:VkSwapchainKHR)->VkResult);
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkReleaseFullScreenExclusiveModeEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkReleaseFullScreenExclusiveModeEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkReleaseFullScreenExclusiveModeEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkSignalSemaphoreKHR(pub unsafe extern "system" fn(device:VkDevice,pSignalInfo:*const VkSemaphoreSignalInfoKHR)->VkResult);
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkSignalSemaphoreKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkSignalSemaphoreKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkSignalSemaphoreKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkTransitionImageLayoutEXT(pub unsafe extern "system" fn(device:VkDevice,transitionCount:u32,pTransitions:*const VkHostImageLayoutTransitionInfoEXT)->VkResult);
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkTransitionImageLayoutEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_host_image_copy")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkTransitionImageLayoutEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkTransitionImageLayoutEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_map_memory2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkUnmapMemory2KHR(pub unsafe extern "system" fn(device:VkDevice,pMemoryUnmapInfo:*const VkMemoryUnmapInfoKHR)->VkResult);
#[cfg(feature = "VK_KHR_map_memory2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkUnmapMemory2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_map_memory2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkUnmapMemory2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkUnmapMemory2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkUnregisterObjectsNVX(pub unsafe extern "system" fn(device:VkDevice,objectTable:VkObjectTableNVX,objectCount:u32,pObjectEntryTypes:*const VkObjectEntryTypeNVX,pObjectIndices:*const u32)->VkResult);
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkUnregisterObjectsNVX{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkUnregisterObjectsNVX{const NAME_CSTR:&'static core::ffi::CStr=c"vkUnregisterObjectsNVX";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkWaitSemaphoresKHR(pub unsafe extern "system" fn(device:VkDevice,pWaitInfo:*const VkSemaphoreWaitInfoKHR,timeout:u64)->VkResult);
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkWaitSemaphoresKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_timeline_semaphore")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkWaitSemaphoresKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkWaitSemaphoresKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBeginRenderPass2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pRenderPassBegin:*const VkRenderPassBeginInfo,pSubpassBeginInfo:*const VkSubpassBeginInfoKHR));
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBeginRenderPass2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBeginRenderPass2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBeginRenderPass2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdBeginRenderPass2{const STATIC:Self=Self(vkCmdBeginRenderPass2);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBeginRendering(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pRenderingInfo:*const VkRenderingInfoKHR));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBeginRendering{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBeginRendering{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBeginRendering";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdBeginRendering{const STATIC:Self=Self(vkCmdBeginRendering);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBindDescriptorSets2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pBindDescriptorSetsInfo:*const VkBindDescriptorSetsInfoKHR));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBindDescriptorSets2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBindDescriptorSets2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBindDescriptorSets2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdBindDescriptorSets2{const STATIC:Self=Self(vkCmdBindDescriptorSets2);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBindIndexBuffer2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,buffer:VkBuffer,offset:VkDeviceSize,size:VkDeviceSize,indexType:VkIndexType));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBindIndexBuffer2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBindIndexBuffer2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBindIndexBuffer2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdBindIndexBuffer2{const STATIC:Self=Self(vkCmdBindIndexBuffer2);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBlitImage2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pBlitImageInfo:*const VkBlitImageInfo2KHR));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBlitImage2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBlitImage2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBlitImage2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdBlitImage2{const STATIC:Self=Self(vkCmdBlitImage2);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdCopyBuffer2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pCopyBufferInfo:*const VkCopyBufferInfo2KHR));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdCopyBuffer2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdCopyBuffer2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdCopyBuffer2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdCopyBuffer2{const STATIC:Self=Self(vkCmdCopyBuffer2);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdCopyBufferToImage2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pCopyBufferToImageInfo:*const VkCopyBufferToImageInfo2KHR));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdCopyBufferToImage2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdCopyBufferToImage2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdCopyBufferToImage2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdCopyBufferToImage2{const STATIC:Self=Self(vkCmdCopyBufferToImage2);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdCopyImage2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pCopyImageInfo:*const VkCopyImageInfo2KHR));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdCopyImage2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdCopyImage2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdCopyImage2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdCopyImage2{const STATIC:Self=Self(vkCmdCopyImage2);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdCopyImageToBuffer2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pCopyImageToBufferInfo:*const VkCopyImageToBufferInfo2KHR));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdCopyImageToBuffer2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdCopyImageToBuffer2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdCopyImageToBuffer2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdCopyImageToBuffer2{const STATIC:Self=Self(vkCmdCopyImageToBuffer2);}
#[cfg(feature = "Allow1_2APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdEndRenderPass2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pSubpassEndInfo:*const VkSubpassEndInfoKHR));
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdEndRenderPass2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdEndRenderPass2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdEndRenderPass2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdEndRenderPass2{const STATIC:Self=Self(vkCmdEndRenderPass2);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdEndRendering(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdEndRendering{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdEndRendering{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdEndRendering";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdEndRendering{const STATIC:Self=Self(vkCmdEndRendering);}
#[cfg(feature = "Allow1_2APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdNextSubpass2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pSubpassBeginInfo:*const VkSubpassBeginInfoKHR,pSubpassEndInfo:*const VkSubpassEndInfoKHR));
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdNextSubpass2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdNextSubpass2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdNextSubpass2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_2APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdNextSubpass2{const STATIC:Self=Self(vkCmdNextSubpass2);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdPushConstants2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pPushConstantsInfo:*const VkPushConstantsInfoKHR));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdPushConstants2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdPushConstants2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdPushConstants2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdPushConstants2{const STATIC:Self=Self(vkCmdPushConstants2);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdPushDescriptorSet(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pipelineBindPoint:VkPipelineBindPoint,layout:VkPipelineLayout,set:u32,descriptorWriteCount:u32,pDescriptorWrites:*const VkWriteDescriptorSet));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdPushDescriptorSet{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdPushDescriptorSet{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdPushDescriptorSet";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdPushDescriptorSet{const STATIC:Self=Self(vkCmdPushDescriptorSet);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdPushDescriptorSet2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pPushDescriptorSetInfo:*const VkPushDescriptorSetInfoKHR));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdPushDescriptorSet2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdPushDescriptorSet2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdPushDescriptorSet2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdPushDescriptorSet2{const STATIC:Self=Self(vkCmdPushDescriptorSet2);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdPushDescriptorSetWithTemplate(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,descriptorUpdateTemplate:VkDescriptorUpdateTemplateKHR,layout:VkPipelineLayout,set:u32,pData:*const core::ffi::c_void));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdPushDescriptorSetWithTemplate{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdPushDescriptorSetWithTemplate{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdPushDescriptorSetWithTemplate";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdPushDescriptorSetWithTemplate{const STATIC:Self=Self(vkCmdPushDescriptorSetWithTemplate);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdPushDescriptorSetWithTemplate2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pPushDescriptorSetWithTemplateInfo:*const VkPushDescriptorSetWithTemplateInfoKHR));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdPushDescriptorSetWithTemplate2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdPushDescriptorSetWithTemplate2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdPushDescriptorSetWithTemplate2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdPushDescriptorSetWithTemplate2{const STATIC:Self=Self(vkCmdPushDescriptorSetWithTemplate2);}
#[cfg(feature = "Allow1_3APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdResolveImage2(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pResolveImageInfo:*const VkResolveImageInfo2KHR));
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdResolveImage2{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdResolveImage2{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdResolveImage2";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_3APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdResolveImage2{const STATIC:Self=Self(vkCmdResolveImage2);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdSetLineStipple(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,lineStippleFactor:u32,lineStipplePattern:u16));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdSetLineStipple{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdSetLineStipple{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdSetLineStipple";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdSetLineStipple{const STATIC:Self=Self(vkCmdSetLineStipple);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdSetRenderingAttachmentLocations(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pLocationInfo:*const VkRenderingAttachmentLocationInfoKHR));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdSetRenderingAttachmentLocations{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdSetRenderingAttachmentLocations{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdSetRenderingAttachmentLocations";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdSetRenderingAttachmentLocations{const STATIC:Self=Self(vkCmdSetRenderingAttachmentLocations);}
#[cfg(feature = "Allow1_4APIs")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdSetRenderingInputAttachmentIndices(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pInputAttachmentIndexInfo:*const VkRenderingInputAttachmentIndexInfoKHR));
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdSetRenderingInputAttachmentIndices{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdSetRenderingInputAttachmentIndices{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdSetRenderingInputAttachmentIndices";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "Allow1_4APIs")]#[cfg(all(feature="Implements",not(feature="DynamicLoaded")))]#[rustfmt::skip]impl crate::StaticCallable for PFN_vkCmdSetRenderingInputAttachmentIndices{const STATIC:Self=Self(vkCmdSetRenderingInputAttachmentIndices);}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBeginRenderPass2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pRenderPassBegin:*const VkRenderPassBeginInfo,pSubpassBeginInfo:*const VkSubpassBeginInfoKHR));
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBeginRenderPass2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBeginRenderPass2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBeginRenderPass2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBeginRenderingKHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pRenderingInfo:*const VkRenderingInfoKHR));
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBeginRenderingKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBeginRenderingKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBeginRenderingKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pipelineBindPoint:VkPipelineBindPoint,layout:VkPipelineLayout,set:u32));
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBindDescriptorBuffersEXT(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,bufferCount:u32,pBindingInfos:*const VkDescriptorBufferBindingInfoEXT));
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBindDescriptorBuffersEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBindDescriptorBuffersEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBindDescriptorBuffersEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBindDescriptorSets2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pBindDescriptorSetsInfo:*const VkBindDescriptorSetsInfoKHR));
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBindDescriptorSets2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBindDescriptorSets2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBindDescriptorSets2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBindIndexBuffer2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,buffer:VkBuffer,offset:VkDeviceSize,size:VkDeviceSize,indexType:VkIndexType));
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBindIndexBuffer2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance5")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBindIndexBuffer2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBindIndexBuffer2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdBlitImage2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pBlitImageInfo:*const VkBlitImageInfo2KHR));
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdBlitImage2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdBlitImage2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdBlitImage2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdCopyBuffer2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pCopyBufferInfo:*const VkCopyBufferInfo2KHR));
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdCopyBuffer2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdCopyBuffer2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdCopyBuffer2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdCopyBufferToImage2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pCopyBufferToImageInfo:*const VkCopyBufferToImageInfo2KHR));
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdCopyBufferToImage2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdCopyBufferToImage2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdCopyBufferToImage2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdCopyImage2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pCopyImageInfo:*const VkCopyImageInfo2KHR));
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdCopyImage2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdCopyImage2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdCopyImage2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdCopyImageToBuffer2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pCopyImageToBufferInfo:*const VkCopyImageToBufferInfo2KHR));
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdCopyImageToBuffer2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdCopyImageToBuffer2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdCopyImageToBuffer2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdDebugMarkerBeginEXT(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pMarkerInfo:*const VkDebugMarkerMarkerInfoEXT));
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdDebugMarkerBeginEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdDebugMarkerBeginEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdDebugMarkerBeginEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdDebugMarkerEndEXT(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer));
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdDebugMarkerEndEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdDebugMarkerEndEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdDebugMarkerEndEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdDebugMarkerInsertEXT(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pMarkerInfo:*const VkDebugMarkerMarkerInfoEXT));
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdDebugMarkerInsertEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_debug_marker")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdDebugMarkerInsertEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdDebugMarkerInsertEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdEndRenderPass2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pSubpassEndInfo:*const VkSubpassEndInfoKHR));
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdEndRenderPass2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdEndRenderPass2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdEndRenderPass2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdEndRenderingKHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer));
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdEndRenderingKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdEndRenderingKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdEndRenderingKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdNextSubpass2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pSubpassBeginInfo:*const VkSubpassBeginInfoKHR,pSubpassEndInfo:*const VkSubpassEndInfoKHR));
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdNextSubpass2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_create_renderpass2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdNextSubpass2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdNextSubpass2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdProcessCommandsNVX(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pProcessCommandsInfo:*const VkCmdProcessCommandsInfoNVX));
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdProcessCommandsNVX{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdProcessCommandsNVX{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdProcessCommandsNVX";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdPushConstants2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pPushConstantsInfo:*const VkPushConstantsInfoKHR));
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdPushConstants2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdPushConstants2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdPushConstants2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_push_descriptor")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdPushDescriptorSetKHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pipelineBindPoint:VkPipelineBindPoint,layout:VkPipelineLayout,set:u32,descriptorWriteCount:u32,pDescriptorWrites:*const VkWriteDescriptorSet));
#[cfg(feature = "VK_KHR_push_descriptor")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdPushDescriptorSetKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_push_descriptor")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdPushDescriptorSetKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdPushDescriptorSetKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdPushDescriptorSet2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pPushDescriptorSetInfo:*const VkPushDescriptorSetInfoKHR));
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdPushDescriptorSet2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdPushDescriptorSet2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdPushDescriptorSet2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_push_descriptor")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdPushDescriptorSetWithTemplateKHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,descriptorUpdateTemplate:VkDescriptorUpdateTemplateKHR,layout:VkPipelineLayout,set:u32,pData:*const core::ffi::c_void));
#[cfg(feature = "VK_KHR_push_descriptor")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdPushDescriptorSetWithTemplateKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_push_descriptor")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdPushDescriptorSetWithTemplateKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdPushDescriptorSetWithTemplateKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdPushDescriptorSetWithTemplate2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pPushDescriptorSetWithTemplateInfo:*const VkPushDescriptorSetWithTemplateInfoKHR));
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdPushDescriptorSetWithTemplate2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_maintenance6")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdPushDescriptorSetWithTemplate2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdPushDescriptorSetWithTemplate2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdReserveSpaceForCommandsNVX(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pReserveSpaceInfo:*const VkCmdReserveSpaceForCommandsInfoNVX));
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdReserveSpaceForCommandsNVX{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NVX_device_generated_commands")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdReserveSpaceForCommandsNVX{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdReserveSpaceForCommandsNVX";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdResolveImage2KHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pResolveImageInfo:*const VkResolveImageInfo2KHR));
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdResolveImage2KHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_copy_commands2")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdResolveImage2KHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdResolveImage2KHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdSetDescriptorBufferOffsetsEXT(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pipelineBindPoint:VkPipelineBindPoint,layout:VkPipelineLayout,firstSet:u32,setCount:u32,pBufferIndices:*const u32,pOffsets:*const VkDeviceSize));
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdSetDescriptorBufferOffsetsEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_descriptor_buffer")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdSetDescriptorBufferOffsetsEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdSetDescriptorBufferOffsetsEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_discard_rectangles")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdSetDiscardRectangleEXT(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,firstDiscardRectangle:u32,discardRectangleCount:u32,pDiscardRectangles:*const VkRect2D));
#[cfg(feature = "VK_EXT_discard_rectangles")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdSetDiscardRectangleEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_discard_rectangles")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdSetDiscardRectangleEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdSetDiscardRectangleEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_line_rasterization")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdSetLineStippleKHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,lineStippleFactor:u32,lineStipplePattern:u16));
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdSetLineStippleKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_line_rasterization")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdSetLineStippleKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdSetLineStippleKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdSetRenderingAttachmentLocationsKHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pLocationInfo:*const VkRenderingAttachmentLocationInfoKHR));
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdSetRenderingAttachmentLocationsKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdSetRenderingAttachmentLocationsKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdSetRenderingAttachmentLocationsKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdSetRenderingInputAttachmentIndicesKHR(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pInputAttachmentIndexInfo:*const VkRenderingInputAttachmentIndexInfoKHR));
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdSetRenderingInputAttachmentIndicesKHR{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdSetRenderingInputAttachmentIndicesKHR{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdSetRenderingInputAttachmentIndicesKHR";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_EXT_sample_locations")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdSetSampleLocationsEXT(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pSampleLocationsInfo:*const VkSampleLocationsInfoEXT));
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdSetSampleLocationsEXT{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_EXT_sample_locations")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdSetSampleLocationsEXT{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdSetSampleLocationsEXT";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdSetViewportWScalingNV(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,firstViewport:u32,viewportCount:u32,pViewportWScalings:*const VkViewportWScalingNV));
#[cfg(feature = "VK_NV_clip_space_w_scaling")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdSetViewportWScalingNV{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdSetViewportWScalingNV{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdSetViewportWScalingNV";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}
#[cfg(feature = "VK_AMD_buffer_marker")]#[repr(transparent)]#[derive(Debug, Clone, Copy)]#[rustfmt::skip]pub struct PFN_vkCmdWriteBufferMarkerAMD(pub unsafe extern "system" fn(commandBuffer:VkCommandBuffer,pipelineStage:VkPipelineStageFlags,dstBuffer:VkBuffer,dstOffset:VkDeviceSize,marker:u32));
#[cfg(feature = "VK_AMD_buffer_marker")]#[rustfmt::skip]unsafe impl crate::FromPtr for PFN_vkCmdWriteBufferMarkerAMD{#[inline(always)]unsafe fn from_ptr(p:*const core::ffi::c_void)->Self{unsafe{core::mem::transmute::<*const core::ffi::c_void,Self>(p)}}}
#[cfg(feature = "VK_AMD_buffer_marker")]#[rustfmt::skip]unsafe impl crate::PFN for PFN_vkCmdWriteBufferMarkerAMD{const NAME_CSTR:&'static core::ffi::CStr=c"vkCmdWriteBufferMarkerAMD";#[inline(always)]unsafe fn from_void_fn(p:PFN_vkVoidFunction)->Self{unsafe{core::mem::transmute::<PFN_vkVoidFunction,Self>(p)}}}

#[cfg(all(feature = "Implements", not(feature = "DynamicLoaded")))]
#[cfg_attr(all(not(windows), not(target_os = "macos"), not(feature = "DynamicLoaded")), link(name = "vulkan"))]
#[cfg_attr(all(windows, not(feature = "DynamicLoaded"), feature = "Implements"), link(name = "vulkan-1"))]
#[rustfmt::skip]
unsafe extern "system" {
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCopyImageToImage(device:VkDevice,pCopyImageToImageInfo:*const VkCopyImageToImageInfoEXT)->VkResult;
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCopyImageToMemory(device:VkDevice,pCopyImageToMemoryInfo:*const VkCopyImageToMemoryInfoEXT)->VkResult;
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCopyMemoryToImage(device:VkDevice,pCopyMemoryToImageInfo:*const VkCopyMemoryToImageInfoEXT)->VkResult;
    #[cfg(feature = "Allow1_2APIs")]pub fn vkCreateRenderPass2(device:VkDevice,pCreateInfo:*const VkRenderPassCreateInfo2KHR,pAllocator:*const VkAllocationCallbacks,pRenderPass:*mut VkRenderPass)->VkResult;
    #[cfg(feature = "Allow1_1APIs")]pub fn vkCreateSamplerYcbcrConversion(device:VkDevice,pCreateInfo:VkSamplerYcbcrConversionCreateInfoKHR,pAllocator:*const VkAllocationCallbacks,pYcbcrConversion:*mut VkSamplerYcbcrConversionKHR)->VkResult;
    #[cfg(feature = "Allow1_1APIs")]pub fn vkDestroySamplerYcbcrConversion(device:VkDevice,ycbcrConversion:VkSamplerYcbcrConversionKHR,pAllocator:*const VkAllocationCallbacks);
    #[cfg(feature = "Allow1_2APIs")]pub fn vkGetBufferDeviceAddress(device:VkDevice,pInfo:*const VkBufferDeviceAddressInfoKHR)->VkDeviceAddress;
    #[cfg(feature = "Allow1_2APIs")]pub fn vkGetBufferOpaqueCaptureAddress(device:VkDevice,pInfo:*const VkBufferDeviceAddressInfoKHR)->u64;
    #[cfg(feature = "Allow1_3APIs")]pub fn vkGetDeviceBufferMemoryRequirements(device:VkDevice,pInfo:*const VkDeviceBufferMemoryRequirementsKHR,pMemoryRequirements:*mut VkMemoryRequirements2KHR);
    #[cfg(feature = "Allow1_3APIs")]pub fn vkGetDeviceImageMemoryREquirements(device:VkDevice,pInfo:*const VkDeviceImageMemoryRequirementsKHR,pMemoryRequirements:*mut VkMemoryRequirements2KHR);
    #[cfg(feature = "Allow1_3APIs")]pub fn vkGetDeviceImageSparseMemoryRequirements(device:VkDevice,pInfo:*const VkDeviceImageMemoryRequirementsKHR,pSparseMemoryRequirementsCount:*mut u32,pSparseMemoryRequirements:*mut VkSparseImageMemoryRequirements2KHR);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkGetDeviceImageSubresourceLayout(device:VkDevice,pInfo:*const VkDeviceImageSubresourceInfoKHR,pLayout:*mut VkSubresourceLayout2KHR);
    #[cfg(feature = "Allow1_2APIs")]pub fn vkGetDeviceMemoryOpaqueCaptureAddress(device:VkDevice,pInfo:*const VkDeviceMemoryOpaqueCaptureAddressInfoKHR)->u64;
    #[cfg(feature = "Allow1_4APIs")]pub fn vkGetImageSubresourceLayout2(device:VkDevice,image:VkImage,pSubresource:*const VkImageSubresource2KHR,pLayout:*mut VkSubresourceLayout2KHR);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkGetRenderingAreaGranularity(device:VkDevice,pRenderingAreaInfo:*const VkRenderingAreaInfoKHR,pGranularity:*mut VkExtent2D);
    #[cfg(feature = "Allow1_2APIs")]pub fn vkGetSemaphoreCounterValue(device:VkDevice,semaphore:VkSemaphore,pValue:*mut u64)->VkResult;
    #[cfg(feature = "Allow1_4APIs")]pub fn vkMapMemory2(device:VkDevice,pMemoryMapInfo:*const VkMemoryMapInfoKHR,ppData:*mut *mut core::ffi::c_void)->VkResult;
    #[cfg(feature = "Allow1_2APIs")]pub fn vkSignalSemaphore(device:VkDevice,pSignalInfo:*const VkSemaphoreSignalInfoKHR)->VkResult;
    #[cfg(feature = "Allow1_4APIs")]pub fn vkTransitionImageLayout(device:VkDevice,transitionCount:u32,pTransitions:*const VkHostImageLayoutTransitionInfoEXT)->VkResult;
    #[cfg(feature = "Allow1_4APIs")]pub fn vkUnmapMemory2(device:VkDevice,pMemoryUnmapInfo:*const VkMemoryUnmapInfoKHR)->VkResult;
    #[cfg(feature = "Allow1_2APIs")]pub fn vkWaitSemaphores(device:VkDevice,pWaitInfo:*const VkSemaphoreWaitInfoKHR,timeout:u64)->VkResult;
    #[cfg(feature = "VK_MVK_ios_surface")]pub fn vkCreateIOSSurfaceMVK(instance:VkInstance,pCreateInfo:*const VkIOSSurfaceCreateInfoMVK,pAllocator:*const VkAllocationCallbacks,pSurface:*mut VkSurfaceKHR)->VkResult;
    #[cfg(feature = "VK_MVK_macos_surface")]pub fn vkCreateMacOSSurfaceMVK(instance:VkInstance,pCreateInfo:*const VkMacOSSurfaceCreateInfoMVK,pAllocator:*const VkAllocationCallbacks,pSurface:*mut VkSurfaceKHR)->VkResult;
    #[cfg(feature = "VK_NN_vi_surface")]pub fn vkCreateViSurfaceNN(instance:VkInstance,pCreateInfo:*const VkViSurfaceCreateInfoNN,pAllocator:*const VkAllocationCallbacks,pSurface:*mut VkSurfaceKHR)->VkResult;
    #[cfg(feature = "Allow1_2APIs")]pub fn vkCmdBeginRenderPass2(commandBuffer:VkCommandBuffer,pRenderPassBegin:*const VkRenderPassBeginInfo,pSubpassBeginInfo:*const VkSubpassBeginInfoKHR);
    #[cfg(feature = "Allow1_3APIs")]pub fn vkCmdBeginRendering(commandBuffer:VkCommandBuffer,pRenderingInfo:*const VkRenderingInfoKHR);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCmdBindDescriptorSets2(commandBuffer:VkCommandBuffer,pBindDescriptorSetsInfo:*const VkBindDescriptorSetsInfoKHR);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCmdBindIndexBuffer2(commandBuffer:VkCommandBuffer,buffer:VkBuffer,offset:VkDeviceSize,size:VkDeviceSize,indexType:VkIndexType);
    #[cfg(feature = "Allow1_3APIs")]pub fn vkCmdBlitImage2(commandBuffer:VkCommandBuffer,pBlitImageInfo:*const VkBlitImageInfo2KHR);
    #[cfg(feature = "Allow1_3APIs")]pub fn vkCmdCopyBuffer2(commandBuffer:VkCommandBuffer,pCopyBufferInfo:*const VkCopyBufferInfo2KHR);
    #[cfg(feature = "Allow1_3APIs")]pub fn vkCmdCopyBufferToImage2(commandBuffer:VkCommandBuffer,pCopyBufferToImageInfo:*const VkCopyBufferToImageInfo2KHR);
    #[cfg(feature = "Allow1_3APIs")]pub fn vkCmdCopyImage2(commandBuffer:VkCommandBuffer,pCopyImageInfo:*const VkCopyImageInfo2KHR);
    #[cfg(feature = "Allow1_3APIs")]pub fn vkCmdCopyImageToBuffer2(commandBuffer:VkCommandBuffer,pCopyImageToBufferInfo:*const VkCopyImageToBufferInfo2KHR);
    #[cfg(feature = "Allow1_2APIs")]pub fn vkCmdEndRenderPass2(commandBuffer:VkCommandBuffer,pSubpassEndInfo:*const VkSubpassEndInfoKHR);
    #[cfg(feature = "Allow1_3APIs")]pub fn vkCmdEndRendering(commandBuffer:VkCommandBuffer);
    #[cfg(feature = "Allow1_2APIs")]pub fn vkCmdNextSubpass2(commandBuffer:VkCommandBuffer,pSubpassBeginInfo:*const VkSubpassBeginInfoKHR,pSubpassEndInfo:*const VkSubpassEndInfoKHR);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCmdPushConstants2(commandBuffer:VkCommandBuffer,pPushConstantsInfo:*const VkPushConstantsInfoKHR);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCmdPushDescriptorSet(commandBuffer:VkCommandBuffer,pipelineBindPoint:VkPipelineBindPoint,layout:VkPipelineLayout,set:u32,descriptorWriteCount:u32,pDescriptorWrites:*const VkWriteDescriptorSet);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCmdPushDescriptorSet2(commandBuffer:VkCommandBuffer,pPushDescriptorSetInfo:*const VkPushDescriptorSetInfoKHR);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCmdPushDescriptorSetWithTemplate(commandBuffer:VkCommandBuffer,descriptorUpdateTemplate:VkDescriptorUpdateTemplateKHR,layout:VkPipelineLayout,set:u32,pData:*const core::ffi::c_void);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCmdPushDescriptorSetWithTemplate2(commandBuffer:VkCommandBuffer,pPushDescriptorSetWithTemplateInfo:*const VkPushDescriptorSetWithTemplateInfoKHR);
    #[cfg(feature = "Allow1_3APIs")]pub fn vkCmdResolveImage2(commandBuffer:VkCommandBuffer,pResolveImageInfo:*const VkResolveImageInfo2KHR);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCmdSetLineStipple(commandBuffer:VkCommandBuffer,lineStippleFactor:u32,lineStipplePattern:u16);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCmdSetRenderingAttachmentLocations(commandBuffer:VkCommandBuffer,pLocationInfo:*const VkRenderingAttachmentLocationInfoKHR);
    #[cfg(feature = "Allow1_4APIs")]pub fn vkCmdSetRenderingInputAttachmentIndices(commandBuffer:VkCommandBuffer,pInputAttachmentIndexInfo:*const VkRenderingInputAttachmentIndexInfoKHR);
}
