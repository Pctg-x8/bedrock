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
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_DISJOINT_BIT_KHR: VkImageCreateFlagBits = 0x00000200;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_DISJOINT_BIT: VkImageCreateFlagBits = 0x00000200;
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: u32 = !1;
#[cfg(feature = "Allow1_1APIs")]#[rustfmt::skip]
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = !1;
#[cfg(feature = "VK_KHR_device_group_creation")]
#[rustfmt::skip]
pub const VK_MAX_DEVICE_GROUP_SIZE_KHR: usize = 32;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_MAX_GLOBAL_PRIORITY_SIZE_KHR: usize = 16;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_MAX_GLOBAL_PRIORITY_SIZE: usize = 16;

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
#[cfg(feature = "VK_EXT_debug_report")]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR: VkDebugReportObjectTypeEXT = 1000156000;
#[cfg(feature = "VK_EXT_debug_report")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION: VkDebugReportObjectTypeEXT = 1000156000;

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
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE: VkSamplerAddressMode = 4;

#[rustfmt::skip]
pub type VkSamplerMipmapMode = i32;
#[rustfmt::skip]
pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: VkSamplerMipmapMode = 0;
#[rustfmt::skip]
pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: VkSamplerMipmapMode = 1;

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

#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkAcquireNextImageInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub swapchain: VkSwapchainKHR,
    pub timeout: u64,
    pub semaphore: VkSemaphore,
    pub fence: VkFence,
    pub deviceMask: u32,
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR: VkStructureType = 1000060010;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkAcquireNextImageInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkAcquireNextImageInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkAllocationCallbacks {
    pub pUserData: *mut core::ffi::c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: Option<PFN_vkInternalAllocationNotification>,
    pub pfnInternalFree: Option<PFN_vkInternalFreeNotification>,
}

#[cfg(feature = "VK_KHR_android_surface")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkAndroidSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkAndroidSurfaceCreateFlagsKHR,
    pub window: *mut android::ANativeWindow,
}
#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000008000;
#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkAndroidSurfaceCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkAndroidSurfaceCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pApplicationName: *const core::ffi::c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const core::ffi::c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = 0;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkApplicationInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkApplicationInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_APPLICATION_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkAttachmentDescription {
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub stencilLoadOp: VkAttachmentLoadOp,
    pub stencilStoreOp: VkAttachmentStoreOp,
    pub initialLayout: VkImageLayout,
    pub finalLayout: VkImageLayout,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}

#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_bind_memory2"))]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBindBufferMemoryDeviceGroupInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_bind_memory2"))]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: VkStructureType = 1000060013;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_bind_memory2"))]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBindBufferMemoryDeviceGroupInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_bind_memory2"))]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBindBufferMemoryDeviceGroupInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkBindBufferMemoryDeviceGroupInfo = VkBindBufferMemoryDeviceGroupInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: VkStructureType = VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR;

#[cfg(feature = "VK_KHR_bind_memory2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBindBufferMemoryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub buffer: VkBuffer,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}
#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR: VkStructureType = 1000157000;
#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBindBufferMemoryInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBindBufferMemoryInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkBindBufferMemoryInfo = VkBindBufferMemoryInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO: VkStructureType = VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR;

#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_bind_memory2"))]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBindImageMemoryDeviceGroupInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
    pub splitInstanceBindRegionCount: u32,
    pub pSplitInstanceBindRegions: *const VkRect2D,
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_bind_memory2"))]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: VkStructureType = 1000060014;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_bind_memory2"))]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBindImageMemoryDeviceGroupInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_bind_memory2"))]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBindImageMemoryDeviceGroupInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkBindImageMemoryDeviceGroupInfo = VkBindImageMemoryDeviceGroupInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: VkStructureType = VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR;

#[cfg(feature = "VK_KHR_bind_memory2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBindImageMemoryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}
#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR: VkStructureType = 1000157001;
#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBindImageMemoryInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBindImageMemoryInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkBindImageMemoryInfo = VkBindImageMemoryInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO: VkStructureType = VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR;

#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBindImageMemorySwapchainInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub swapchain: VkSwapchainKHR,
    pub imageIndex: u32,
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: VkStructureType = 1000060009;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBindImageMemorySwapchainInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBindImageMemorySwapchainInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR; }

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBindImagePlaneMemoryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub planeAspect: VkImageAspectFlags,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR: VkStructureType = 1000156002;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBindImagePlaneMemoryInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBindImagePlaneMemoryInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkBindImagePlaneMemoryInfo = VkBindImagePlaneMemoryInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO: VkStructureType = VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBindSparseInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
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
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: VkStructureType = 7;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBindSparseInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBindSparseInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BIND_SPARSE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBufferCopy {
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = 12;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBufferCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBufferCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBufferImageCopy {
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBufferMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: VkStructureType = 44;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBufferMemoryBarrier {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBufferMemoryBarrier { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER; }

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBufferMemoryRequirementsInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub buffer: VkBuffer,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = 1000146000;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBufferMemoryRequirementsInfo2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBufferMemoryRequirementsInfo2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkBufferMemoryRequirementsInfo2 = VkBufferMemoryRequirementsInfo2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBufferViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkBufferViewCreateFlags,
    pub buffer: VkBuffer,
    pub format: VkFormat,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: VkStructureType = 13;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBufferViewCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBufferViewCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO; }

#[derive(Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkClearAttachment {
    pub aspectMask: VkImageAspectFlags,
    pub colorAttachment: u32,
    pub clearValue: VkClearValue,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkClearDepthStencilValue {
    pub depth: core::ffi::c_float,
    pub stencil: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkClearRect {
    pub rect: VkRect2D,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkCommandBufferAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType = 40;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkCommandBufferAllocateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkCommandBufferAllocateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = 42;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkCommandBufferBeginInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkCommandBufferBeginInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkCommandBufferInheritanceInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub renderPass: Option<VkRenderPass>,
    pub subpass: u32,
    pub framebuffer: Option<VkFramebuffer>,
    pub occlusionQueryEnable: VkBool32,
    pub queryFlags: VkQueryControlFlags,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: VkStructureType = 41;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkCommandBufferInheritanceInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkCommandBufferInheritanceInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkCommandPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType = 39;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkCommandPoolCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkCommandPoolCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO; }

#[derive(Debug, Clone, Copy)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkComputePipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: Option<VkPipelineLayout>,
    pub basePipelineHandle: Option<VkPipeline>,
    pub basePipelineIndex: i32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = 29;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkComputePipelineCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkComputePipelineCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkCopyDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcSet: VkDescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: VkStructureType = 36;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkCopyDescriptorSet {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkCopyDescriptorSet { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET; }

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkD3D12FenceSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub waitSemaphoreValuesCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValuesCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR: VkStructureType = 1000078002;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkD3D12FenceSubmitInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkD3D12FenceSubmitInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR; }

#[cfg(feature = "VK_EXT_debug_report")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDebugReportCallbackCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDebugReportFlagsEXT,
    pub pfnCallback: PFN_vkDebugReportCallbackEXT,
    pub pUserData: *mut core::ffi::c_void,
}
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: VkStructureType = 1000011000;
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDebugReportCallbackCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDebugReportCallbackCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT; }

#[cfg(feature = "VK_EXT_debug_utils")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDebugUtilsLabelEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pLabelName: *const core::ffi::c_char,
    pub pColor: [core::ffi::c_float; 4],
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT: VkStructureType = 1000128002;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDebugUtilsLabelEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDebugUtilsLabelEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT; }

#[cfg(feature = "VK_EXT_debug_utils")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
    pub pMessageIdName: *const core::ffi::c_char,
    pub messageIdNumber: i32,
    pub pMessage: *const core::ffi::c_char,
    pub queueLabelCount: u32,
    pub pQueueLabels: *const VkDebugUtilsLabelEXT,
    pub cmdBufLabelCount: u32,
    pub pCmdBufLabels: *const VkDebugUtilsLabelEXT,
    pub objectCount: u32,
    pub pObjects: *const VkDebugUtilsObjectNameInfoEXT,
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: VkStructureType = 1000128003;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDebugUtilsMessengerCallbackDataEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDebugUtilsMessengerCallbackDataEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT; }

#[cfg(feature = "VK_EXT_debug_utils")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
    pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
    pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut core::ffi::c_void,
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: VkStructureType = 1000128004;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDebugUtilsMessengerCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDebugUtilsMessengerCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT; }

#[cfg(feature = "VK_EXT_debug_utils")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub pObjectName: *const core::ffi::c_char,
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT: VkStructureType = 1000128000;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDebugUtilsObjectNameInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDebugUtilsObjectNameInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT; }

#[cfg(feature = "VK_EXT_debug_utils")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDebugUtilsObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub tagName: u64,
    pub tagSize: u64,
    pub pTag: *const core::ffi::c_void,
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT: VkStructureType = 1000128001;
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDebugUtilsObjectTagInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDebugUtilsObjectTagInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorImageInfo {
    pub sampler: Option<VkSampler>,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const VkDescriptorPoolSize,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: VkStructureType = 33;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDescriptorPoolCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDescriptorPoolCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorPoolSize {
    pub r#type: VkDescriptorType,
    pub descriptorCount: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorSetAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType = 34;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDescriptorSetAllocateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDescriptorSetAllocateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: VkShaderStageFlags,
    pub pImmutableSamplers: *const VkSampler,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType = 32;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDescriptorSetLayoutCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDescriptorSetLayoutCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO; }

#[cfg(feature = "VK_KHR_maintenance3")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorSetLayoutSupportKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub supported: VkBool32,
}
#[cfg(feature = "VK_KHR_maintenance3")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: VkStructureType = 1000168001;
#[cfg(feature = "VK_KHR_maintenance3")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkDescriptorSetLayoutSupportKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_maintenance3")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkDescriptorSetLayoutSupportKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDescriptorSetLayoutSupport = VkDescriptorSetLayoutSupportKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR;

#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDescriptorUpdateTemplateCreateFlagsKHR,
    pub descriptorUpdateEntryCount: u32,
    pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntryKHR,
    pub templateType: VkDescriptorUpdateTemplateTypeKHR,
    pub descriptorSetLayout: VkDescriptorSetLayout,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub pipelineLayout: VkPipelineLayout,
    pub set: u32,
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: VkStructureType = 1000085000;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDescriptorUpdateTemplateCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDescriptorUpdateTemplateCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDescriptorUpdateTemplateCreateInfo = VkDescriptorUpdateTemplateCreateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorUpdateTemplateEntryKHR {
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub offset: usize,
    pub stride: usize,
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDescriptorUpdateTemplateEntry = VkDescriptorUpdateTemplateEntryKHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const core::ffi::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const core::ffi::c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = 3;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO; }

#[cfg(feature = "VK_KHR_device_group")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceGroupBindSparseInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub resourceDeviceIndex: u32,
    pub memoryDeviceIndex: u32,
}
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR: VkStructureType = 1000060006;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceGroupBindSparseInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceGroupBindSparseInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDeviceGroupBindSparseInfo = VkDeviceGroupBindSparseInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR;

#[cfg(feature = "VK_KHR_device_group")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceGroupCommandBufferBeginInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub deviceMask: u32,
}
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: VkStructureType = 1000060004;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceGroupCommandBufferBeginInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceGroupCommandBufferBeginInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDeviceGroupCommandBufferBeginInfo = VkDeviceGroupCommandBufferBeginInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR;

#[cfg(feature = "VK_KHR_device_group_creation")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceGroupCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub physicalDeviceCount: u32,
    pub pPhysicalDevices: *const VkPhysicalDevice,
}
#[cfg(feature = "VK_KHR_device_group_creation")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_CREATE_INFO_KHR: VkStructureType = 1000070001;
#[cfg(feature = "VK_KHR_device_group_creation")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceGroupCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_device_group_creation")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceGroupCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDeviceGroupCreateInfo = VkDeviceGroupCreateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_CREATE_INFO_KHR;

#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface"))]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE_KHR],
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface"))]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: VkStructureType = 1000060007;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface"))]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkDeviceGroupPresentCapabilitiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface"))]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkDeviceGroupPresentCapabilitiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR; }

#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceGroupPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub swapchainCount: u32,
    pub pDeviceMasks: *const u32,
    pub mode: VkDeviceGroupPresentModeFlagBitsKHR,
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR: VkStructureType = 1000060011;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceGroupPresentInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceGroupPresentInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR; }

#[cfg(feature = "VK_KHR_device_group")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceGroupRenderPassBeginInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub deviceMask: u32,
    pub deviceRenderAreaCount: u32,
    pub pDeviceRenderAreas: *const VkRect2D,
}
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: VkStructureType = 1000060003;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceGroupRenderPassBeginInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceGroupRenderPassBeginInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDeviceGroupRenderPassBeginInfo = VkDeviceGroupRenderPassBeginInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR;

#[cfg(feature = "VK_KHR_device_group")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceGroupSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphoreDeviceIndices: *const u32,
    pub commandBufferCount: u32,
    pub pCommandBufferDeviceMasks: *const u32,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphoreDeviceIndices: *const u32,
}
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR: VkStructureType = 1000060005;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceGroupSubmitInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceGroupSubmitInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkDeviceGroupSubmitInfo = VkDeviceGroupSubmitInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR;

#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = 1000060012;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceGroupSwapchainCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceGroupSwapchainCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceQueueCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const core::ffi::c_float,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = 2;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceQueueCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceQueueCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[cfg(feature = "VK_KHR_display")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDisplayModeCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDisplayModeCreateFlagsKHR,
    pub parameters: VkDisplayModeParametersKHR,
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR: VkStructureType = 1000002000;
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDisplayModeCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDisplayModeCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR; }

#[cfg(feature = "VK_KHR_display")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDisplayModeParametersKHR {
    pub visibleRegion: VkExtent2D,
    pub refreshRate: u32,
}

#[cfg(feature = "VK_KHR_display")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDisplayModePropertiesKHR {
    pub displayMode: VkDisplayModeKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

#[cfg(feature = "VK_KHR_display")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDisplayPlaneCapabilitiesKHR {
    pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
    pub minSrcPosition: VkOffset2D,
    pub maxSrcPosition: VkOffset2D,
    pub minSrcExtent: VkExtent2D,
    pub maxSrcExtent: VkExtent2D,
    pub minDstPosition: VkOffset2D,
    pub maxDstPosition: VkOffset2D,
    pub minDstExtent: VkExtent2D,
    pub maxDstExtent: VkExtent2D,
}

#[cfg(feature = "VK_KHR_display")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDisplayPlanePropertiesKHR {
    pub currentDisplay: VkDisplayKHR,
    pub currentStackIndex: u32,
}

#[cfg(feature = "VK_KHR_display_swapchain")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDisplayPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcRect: VkRect2D,
    pub dstRect: VkRect2D,
    pub persistent: VkBool32,
}
#[cfg(feature = "VK_KHR_display_swapchain")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR: VkStructureType = 1000003000;
#[cfg(feature = "VK_KHR_display_swapchain")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDisplayPresentInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDisplayPresentInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR; }

#[cfg(feature = "VK_KHR_display")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDisplayPropertiesKHR {
    pub display: VkDisplayKHR,
    pub displayName: *const core::ffi::c_char,
    pub physicalDimensions: VkExtent2D,
    pub physicalResolution: VkExtent2D,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub planeReorderPossible: VkBool32,
    pub persistentContent: VkBool32,
}

#[cfg(feature = "VK_KHR_display")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDisplaySurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDisplaySurfaceCreateFlagsKHR,
    pub displayMode: VkDisplayModeKHR,
    pub planeIndex: u32,
    pub planeStackIndex: u32,
    pub transform: VkSurfaceTransformFlagBitsKHR,
    pub globalAlpha: core::ffi::c_float,
    pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
    pub imageExtent: VkExtent2D,
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000002001;
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDisplaySurfaceCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDisplaySurfaceCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDrawIndexedIndirectCommand {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub firstIndex: u32,
    pub vertexOffset: i32,
    pub firstInstance: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDrawIndirectCommand {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub firstVertex: u32,
    pub firstInstance: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkEventCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkEventCreateFlags,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: VkStructureType = 10;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkEventCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkEventCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EVENT_CREATE_INFO; }

#[cfg(feature = "VK_KHR_external_fence")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExportFenceCreateInfoKHR {
    pub handleTypes: VkExternalFenceHandleTypeFlagsKHR,
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExportFenceCreateInfo = VkExportFenceCreateInfoKHR;

#[cfg(feature = "VK_KHR_external_fence_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExportFenceWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pAttributes: *const windows::Win32::Security::SECURITY_ATTRIBUTES,
    pub dwAccess: u32,
    pub name: windows::core::PCWSTR,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000114001;
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkExportFenceWin32HandleInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkExportFenceWin32HandleInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR; }

#[cfg(feature = "VK_KHR_external_memory")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkVkExportMemoryAllocateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR: VkStructureType = 1000072002;
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkVkExportMemoryAllocateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkVkExportMemoryAllocateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkVkExportMemoryAllocateInfo = VkVkExportMemoryAllocateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR;

#[cfg(feature = "VK_KHR_external_memory_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pAttributes: *const windows::Win32::Security::SECURITY_ATTRIBUTES,
    pub dwAccess: u32,
    pub name: windows::core::PCWSTR,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000073001;
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkExportMemoryWin32HandleInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkExportMemoryWin32HandleInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR; }

#[cfg(feature = "VK_KHR_external_semaphore")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExportSemaphoreCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleType: VkExternalSemaphoreHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_semaphore")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR: VkStructureType = 1000077000;
#[cfg(feature = "VK_KHR_external_semaphore")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkExportSemaphoreCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_semaphore")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkExportSemaphoreCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExportSemaphoreCreateInfo = VkExportSemaphoreCreateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pAttributes: *const windows::Win32::Security::SECURITY_ATTRIBUTES,
    pub dwAccess: u32,
    pub name: windows::core::PCWSTR,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000078001;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkExportSemaphoreWin32HandleInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkExportSemaphoreWin32HandleInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExtensionProperties {
    pub extensionName: crate::FixedCStrBuffer<VK_MAX_EXTENSION_NAME_SIZE>,
    pub specVersion: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExternalBufferPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHR,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR: VkStructureType = 1000071003;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkExternalBufferPropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkExternalBufferPropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalBufferProperties = VkExternalBufferPropertiesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR;

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExternalFencePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
    pub externalFenceFeatures: VkExternalFenceFeatureFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR: VkStructureType = 1000112001;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkExternalFencePropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkExternalFencePropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalFenceProperties = VkExternalFencePropertiesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR;

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExternalImageFormatPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHR,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERANL_IMAGE_FORMAT_PROPERTIES_KHR: VkStructureType = 1000071001;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkExternalImageFormatPropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkExternalImageFormatPropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXTERANL_IMAGE_FORMAT_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalImageFormatProperties = VkExternalImageFormatPropertiesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERANL_IMAGE_FORMAT_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_EXTERANL_IMAGE_FORMAT_PROPERTIES_KHR;

#[cfg(feature = "VK_KHR_external_memory")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExternalMemoryBufferCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: VkStructureType = 1000072000;
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkExternalMemoryBufferCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkExternalMemoryBufferCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalMemoryBufferCreateInfo = VkExternalMemoryBufferCreateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_external_memory")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExternalMemoryImageCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: VkStructureType = 1000072001;
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkExternalMemoryImageCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkExternalMemoryImageCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalMemoryImageCreateInfo = VkExternalMemoryImageCreateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExternalMemoryPropertiesKHR {
    pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsKHR,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalMemoryProperties = VkExternalMemoryPropertiesKHR;

#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkExternalSemaphorePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR: VkStructureType = 1000076001;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkExternalSemaphorePropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkExternalSemaphorePropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkExternalSemaphoreProperties = VkExternalSemaphorePropertiesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkFenceCreateFlags,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = 8;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkFenceCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkFenceCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO; }

#[cfg(feature = "VK_KHR_external_fence_fd")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkFenceGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR: VkStructureType = 1000115001;
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkFenceGetFdInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkFenceGetFdInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR; }

#[cfg(feature = "VK_KHR_external_fence_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkFenceGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000114002;
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkFenceGetWin32HandleInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkFenceGetWin32HandleInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkFormatProperties {
    pub linearTilingFeatures: VkFormatFeatureFlags,
    pub optimalTilingFeatures: VkFormatFeatureFlags,
    pub bufferFeatures: VkFormatFeatureFlags,
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub formatProperties: VkFormatProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR: VkStructureType = 1000059002;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkFormatProperties2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkFormatProperties2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkFormatProperties2 = VkFormatProperties2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkFramebufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkFramebufferCreateFlags,
    pub renderPass: VkRenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: VkStructureType = 37;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkFramebufferCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkFramebufferCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkGraphicsPipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
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
    pub layout: Option<VkPipelineLayout>,
    pub renderPass: Option<VkRenderPass>,
    pub subpass: u32,
    pub basePipelineHandle: Option<VkPipeline>,
    pub basePipelineIndex: i32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: VkStructureType = 28;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkGraphicsPipelineCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkGraphicsPipelineCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageBlit {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2],
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageCopy {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkImageCreateFlags,
    pub imageType: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: VkSampleCountFlagBits,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: VkImageLayout,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: VkStructureType = 14;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageFormatProperties {
    pub maxExtent: VkExtent3D,
    pub maxMipLevels: u32,
    pub maxArrayLayers: u32,
    pub sampleCounts: VkSampleCountFlags,
    pub maxResourceSize: VkDeviceSize,
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub imageFormatProperties: VkImageFormatProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType = 1000059003;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkImageFormatProperties2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkImageFormatProperties2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkImageFormatProperties2 = VkImageFormatProperties2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: VkStructureType = 45;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageMemoryBarrier {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageMemoryBarrier { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER; }

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageMemoryRequirementsInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = 1000146001;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageMemoryRequirementsInfo2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageMemoryRequirementsInfo2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkImageMemoryRequirementsInfo2 = VkImageMemoryRequirementsInfo2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImagePlaneMemoryRequirementsInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub planeAspect: VkImageAspectFlagBits,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: VkStructureType = 1000156002;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImagePlaneMemoryRequirementsInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImagePlaneMemoryRequirementsInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkImagePlaneMemoryRequirementsInfo = VkImagePlaneMemoryRequirementsInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageResolve {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageSparseMemoryRequirementsInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = 1000146002;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageSparseMemoryRequirementsInfo2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageSparseMemoryRequirementsInfo2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkImageSparseMemoryRequirementsInfo2 = VkImageSparseMemoryRequirementsInfo2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageSubresource {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub arrayLayer: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageSubresourceLayers {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageSubresourceRange {
    pub aspectMask: VkImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub swapchain: VkSwapchainKHR,
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = 1000060008;
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageSwapchainCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageSwapchainCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkImageViewCreateFlags,
    pub image: VkImage,
    pub viewType: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresourceRange: VkImageSubresourceRange,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: VkStructureType = 15;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageViewCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageViewCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO; }

#[cfg(feature = "VK_KHR_maintenance2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageViewUsageCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub sliceOffset: u32,
    pub sliceCount: u32,
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR: VkStructureType = 1000117002;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageViewUsageCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageViewUsageCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkImageViewUsageCreateInfo = VkImageViewUsageCreateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_external_fence_fd")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImportFenceFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlagsKHR,
    pub handleType: VkExternalFenceHandleTypeFlagsKHR,
    pub fd: core::ffi::c_int,
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR: VkStructureType = 1000115000;
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImportFenceFdInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImportFenceFdInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR; }

#[cfg(feature = "VK_KHR_external_fence_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImportFenceWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlagsKHR,
    pub handleType: VkExternalFenceHandleTypeFlagsKHR,
    pub handle: windows::Win32::Foundation::HANDLE,
    pub name: windows::core::PCWSTR,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000114000;
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImportFenceWin32HandleInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImportFenceWin32HandleInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR; }

#[cfg(feature = "VK_KHR_external_memory_fd")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImportMemoryFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
    pub fd: core::ffi::c_int,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR: VkStructureType = 1000074000;
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImportMemoryFdInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImportMemoryFdInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR; }

#[cfg(feature = "VK_KHR_external_memory_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImportMemoryWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
    pub handle: windows::Win32::Foundation::HANDLE,
    pub name: windows::core::PCWSTR,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000073000;
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImportMemoryWin32HandleInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImportMemoryWin32HandleInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR; }

#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImportSemaphoreFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphore: VkSemaphore,
    pub flags: VkSemaphoreImportFlagsKHR,
    pub handleType: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub fd: core::ffi::c_int,
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR: VkStructureType = 1000079000;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImportSemaphoreFdInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImportSemaphoreFdInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR; }

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphore: VkSemaphore,
    pub flags: VkSemaphoreImportFlagsKHR,
    pub handleType: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub handle: windows::Win32::Foundation::HANDLE,
    pub name: windows::core::PCWSTR,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000078000;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImportSemaphoreWin32HandleInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImportSemaphoreWin32HandleInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR; }

#[cfg(feature = "VK_KHR_maintenance2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkInputAttachmentAspectReferenceKHR {
    pub subpass: u32,
    pub inputAttachmentIndex: u32,
    pub aspectMask: VkImageAspectFlags,
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkInputAttachmentAspectReference = VkInputAttachmentAspectReferenceKHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkInstanceCreateFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const core::ffi::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const core::ffi::c_char,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = 1;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkInstanceCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkInstanceCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkLayerProperties {
    pub layerName: crate::FixedCStrBuffer<VK_MAX_EXTENSION_NAME_SIZE>,
    pub specVersion: u32,
    pub implementationVersion: u32,
    pub description: crate::FixedCStrBuffer<VK_MAX_DESCRIPTION_SIZE>,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMappedMemoryRange {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: VkStructureType = 6;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkMappedMemoryRange {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkMappedMemoryRange { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE; }

#[cfg(feature = "VK_KHR_device_group")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryAllocateFlagsInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkMemoryAllocateFlags,
    pub deviceMask: u32,
}
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR: VkStructureType = 1000060000;
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkMemoryAllocateFlagsInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkMemoryAllocateFlagsInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkMemoryAllocateFlagsInfo = VkMemoryAllocateFlagsInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeIndex: u32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = 5;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkMemoryAllocateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkMemoryAllocateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: VkStructureType = 46;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkMemoryBarrier {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkMemoryBarrier { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_BARRIER; }

#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryDedicatedAllocateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: Option<VkImage>,
    pub buffer: Option<VkBuffer>,
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR: VkStructureType = 1000127001;
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkMemoryDedicatedAllocateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkMemoryDedicatedAllocateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkMemoryDedicatedAllocateInfo = VkMemoryDedicatedAllocateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR;

#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryDedicatedRequirementsKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub prefersDedicatedAllocation: VkBool32,
    pub requiresDedicatedAllocation: VkBool32,
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR: VkStructureType = 1000127000;
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkMemoryDedicatedRequirementsKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkMemoryDedicatedRequirementsKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkMemoryDedicatedRequirements = VkMemoryDedicatedRequirementsKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR;

#[cfg(feature = "VK_KHR_external_memory_fd")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryFdPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub memoryTypeBits: u32,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR: VkStructureType = 1000074001;
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkMemoryFdPropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkMemoryFdPropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR; }

#[cfg(feature = "VK_KHR_external_memory_fd")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR: VkStructureType = 1000074002;
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkMemoryGetFdInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkMemoryGetFdInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR; }

#[cfg(feature = "VK_KHR_external_memory_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub memory: VkDeviceMemory,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000073003;
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkMemoryGetWin32HandleInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkMemoryGetWin32HandleInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: u32,
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryRequirements2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub memoryRequirements: VkMemoryRequirements,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR: VkStructureType = 1000146003;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkMemoryRequirements2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkMemoryRequirements2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkMemoryRequirements2 = VkMemoryRequirements2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

#[cfg(feature = "VK_KHR_external_memory_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryWin32HandlePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub memoryTypeBits: u32,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR: VkStructureType = 1000073002;
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkMemoryWin32HandlePropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkMemoryWin32HandlePropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR; }

#[cfg(feature = "VK_EXT_metal_surface")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMetalSurfaceCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkMetalSurfaceCreateFlagsEXT,
    pub pLayer: *const core::ffi::c_void,
}
#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT: VkStructureType = 1000217000;
#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkMetalSurfaceCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkMetalSurfaceCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT; }

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkOffset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[cfg(feature = "VK_KHR_16bit_storage")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDevice16BitStorageFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub storageBuffer16BitAccess: VkBool32,
    pub uniformAndStorageBuffer16BitAccess: VkBool32,
    pub storagePushConstant16: VkBool32,
    pub storageInputOutput16: VkBool32,
}
#[cfg(feature = "VK_KHR_16bit_storage")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: VkStructureType = 1000083000;
#[cfg(feature = "VK_KHR_16bit_storage")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDevice16BitStorageFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_16bit_storage")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDevice16BitStorageFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR; }
#[cfg(feature = "VK_KHR_16bit_storage")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDevice16BitStorageFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_16bit_storage")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDevice16BitStorageFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDevice16BitStorageFeatures = VkPhysicalDevice16BitStorageFeaturesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR;

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceExternalBufferInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkBufferCreateFlags,
    pub usage: VkBufferUsageFlags,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: VkStructureType = 1000071002;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceExternalBufferInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceExternalBufferInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceExternalBufferInfo = VkPhysicalDeviceExternalBufferInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR;

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceExternalFenceInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleType: VkExternalFenceHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: VkStructureType = 1000112000;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceExternalFenceInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceExternalFenceInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceExternalFenceInfo = VkPhysicalDeviceExternalFenceInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR;

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: VkStructureType = 1000071000;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceExternalImageFormatInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceExternalImageFormatInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceExternalImageFormatInfo = VkPhysicalDeviceExternalImageFormatInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR;

#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleType: VkExternalSemaphoreHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: VkStructureType = 1000076000;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceExternalSemaphoreInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceExternalSemaphoreInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceExternalSemaphoreInfo = VkPhysicalDeviceExternalSemaphoreInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
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
#[rustfmt::skip]
impl Default for VkPhysicalDeviceFeatures {
    #[inline(always)]
    fn default() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceFeatures2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub features: VkPhysicalDeviceFeatures,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR: VkStructureType = 1000059000;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceFeatures2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceFeatures2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR; }
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceFeatures2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceFeatures2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceFeatures2 = VkPhysicalDeviceFeatures2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR;

#[cfg(feature = "VK_KHR_device_group_creation")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceGroupPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub physicalDeviceCount: u32,
    pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE_KHR],
    pub subsetAllocation: VkBool32,
}
#[cfg(feature = "VK_KHR_device_group_creation")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: VkStructureType = 1000070000;
#[cfg(feature = "VK_KHR_device_group_creation")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceGroupPropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_device_group_creation")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceGroupPropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceGroupProperties = VkPhysicalDeviceGroupPropertiesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR;

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceImageFormatInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub format: VkFormat,
    pub r#type: VkImageType,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub flags: VkImageCreateFlags,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType = 1000059004;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceImageFormatInfo2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceImageFormatInfo2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceImageFormatInfo2 = VkPhysicalDeviceImageFormatInfo2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
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
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: VkDeviceSize,
    pub sparseAddressSpaceSize: VkDeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
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
    pub maxSamplerLodBias: core::ffi::c_float,
    pub maxSamplerAnisotropy: core::ffi::c_float,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [core::ffi::c_float; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: usize,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: core::ffi::c_float,
    pub maxInterpolationOffset: core::ffi::c_float,
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
    pub timestampPeriod: core::ffi::c_float,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [core::ffi::c_float; 2],
    pub lineWidthRange: [core::ffi::c_float; 2],
    pub pointSizeGranularity: core::ffi::c_float,
    pub lineWidthGranularity: core::ffi::c_float,
    pub strictLines: VkBool32,
    pub standardSampleLocations: VkBool32,
    pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    pub nonCoherentAtomSize: VkDeviceSize,
}

#[cfg(feature = "VK_KHR_maintenance3")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceMaintenance3PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxPerSetDescriptors: u32,
    pub maxMemoryAllocationSize: VkDeviceSize,
}
#[cfg(feature = "VK_KHR_maintenance3")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: VkStructureType = 1000168000;
#[cfg(feature = "VK_KHR_maintenance3")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMaintenance3PropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_maintenance3")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMaintenance3PropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceMaintenance3Properties = VkPhysicalDeviceMaintenance3PropertiesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceMemoryProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: VkStructureType = 1000059006;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMemoryProperties2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMemoryProperties2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceMemoryProperties2 = VkPhysicalDeviceMemoryProperties2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR;

#[cfg(feature = "VK_KHR_multiview")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceMultiviewFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub multiview: VkBool32,
    pub multiviewGeometryShader: VkBool32,
    pub multiviewTessellationShader: VkBool32,
}
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: VkStructureType = 1000053001;
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceMultiviewFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceMultiviewFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR; }
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMultiviewFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMultiviewFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceMultiviewFeatures = VkPhysicalDeviceMultiviewFeaturesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR;

#[cfg(feature = "VK_KHR_multiview")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceMultiviewPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxMultiviewViewCount: u32,
    pub maxMultiviewInstanceIndex: u32,
}
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: VkStructureType = 1000053002;
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMultiviewPropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMultiviewPropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceMultiviewProperties = VkPhysicalDeviceMultiviewPropertiesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR;

#[cfg(feature = "VK_KHR_maintenance2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDevicePointClippingPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub pointClippingBehavior: VkPointClippingBehaviorKHR,
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: VkStructureType = 1000117000;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDevicePointClippingPropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDevicePointClippingPropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDevicePointClippingProperties = VkPhysicalDevicePointClippingPropertiesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: crate::FixedCStrBuffer<VK_MAX_PHYSICAL_DEVICE_NAME_SIZE>,
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub properties: VkPhysicalDeviceProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR: VkStructureType = 1000059001;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceProperties2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceProperties2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceProperties2 = VkPhysicalDeviceProperties2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub samplerYcbcrConversion: VkBool32,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: VkStructureType = 1000156004;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR; }
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceSamplerYcbcrConversionFeatures = VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR;

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub format: VkFormat,
    pub r#type: VkImageType,
    pub samples: VkSampleCountFlagBits,
    pub usage: VkImageUsageFlags,
    pub tiling: VkImageTiling,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType = 1000059008;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceSparseImageFormatInfo2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceSparseImageFormatInfo2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceSparseImageFormatInfo2 = VkPhysicalDeviceSparseImageFormatInfo2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}

#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub surface: VkSurfaceKHR,
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: VkStructureType = 1000119000;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceSurfaceInfo2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceSurfaceInfo2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR; }

#[cfg(feature = "VK_KHR_variable_pointers")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceVariablePointersFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub variablePointersStorageBuffer: VkBool32,
    pub variablePointers: VkBool32,
}
#[cfg(feature = "VK_KHR_variable_pointers")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: VkStructureType = 1000120000;
#[cfg(feature = "VK_KHR_variable_pointers")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceVariablePointersFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_variable_pointers")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceVariablePointersFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR; }
#[cfg(feature = "VK_KHR_variable_pointers")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVariablePointersFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_variable_pointers")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVariablePointersFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceVariablePointersFeatures = VkPhysicalDeviceVariablePointersFeaturesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineCacheCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initialDataSize: usize,
    pub pInitialData: *const core::ffi::c_void,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: VkStructureType = 17;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineCacheCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineCacheCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
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

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineColorBlendStateCreateFlags,
    pub logicOpEnable: VkBool32,
    pub logicOp: VkLogicOp,
    pub attachmentCount: u32,
    pub pAttachments: *const VkPipelineColorBlendAttachmentState,
    pub blendConstants: [core::ffi::c_float; 4],
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: VkStructureType = 26;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineColorBlendStateCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineColorBlendStateCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineDepthStencilStateCreateFlags,
    pub depthTestEnable: VkBool32,
    pub depthWriteEnable: VkBool32,
    pub depthCompareOp: VkCompareOp,
    pub depthBoundsTestEnable: VkBool32,
    pub stencilTestEnable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub minDepthBounds: core::ffi::c_float,
    pub maxDepthBounds: core::ffi::c_float,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: VkStructureType = 25;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineDepthStencilStateCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineDepthStencilStateCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineDynamicStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const VkDynamicState,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: VkStructureType = 27;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineDynamicStateCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineDynamicStateCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlags,
    pub topology: VkPrimitiveTopology,
    pub primitiveRestartEnable: VkBool32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: VkStructureType = 20;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineInputAssemblyStateCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineInputAssemblyStateCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: VkStructureType = 30;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineLayoutCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineLayoutCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: VkSampleCountFlagBits,
    pub sampleShadingEnable: VkBool32,
    pub minSampleShading: core::ffi::c_float,
    pub pSampleMask: *const VkSampleMask,
    pub alphaToCoverageEnable: VkBool32,
    pub alphaToOneEnable: VkBool32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: VkStructureType = 24;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineMultisampleStateCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineMultisampleStateCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineRasterizationStateCreateFlags,
    pub depthClampEnable: VkBool32,
    pub rasterizerDiscardEnable: VkBool32,
    pub polygonMode: VkPolygonMode,
    pub cullMode: VkCullModeFlags,
    pub frontFace: VkFrontFace,
    pub depthBiasEnable: VkBool32,
    pub depthBiasConstantFactor: core::ffi::c_float,
    pub depthBiasClamp: core::ffi::c_float,
    pub depthBiasSlopeFactor: core::ffi::c_float,
    pub lineWidth: core::ffi::c_float,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: VkStructureType = 23;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineRasterizationStateCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineRasterizationStateCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlagBits,
    pub module: Option<VkShaderModule>,
    pub pName: *const core::ffi::c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType = 18;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineShaderStageCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineShaderStageCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO; }

#[cfg(feature = "VK_KHR_maintenance2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub domainOrigin: VkTessellationDomainOriginKHR,
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: VkStructureType = 1000117003;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineTessellationDomainOriginStateCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineTessellationDomainOriginStateCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkPipelineTessellationDomainOriginStateCreateInfo = VkPipelineTessellationDomainOriginStateCreateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineTessellationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: VkStructureType = 21;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineTessellationStateCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineTessellationStateCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: VkStructureType = 19;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineVertexInputStateCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineVertexInputStateCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineViewportStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineViewportStateCreateFlags,
    pub viewportCount: u32,
    pub pViewports: *const VkViewport,
    pub scissorCount: u32,
    pub pScissors: *const VkRect2D,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: VkStructureType = 22;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineViewportStateCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineViewportStateCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO; }

#[cfg(feature = "VK_KHR_swapchain")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut VkResult,
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR: VkStructureType = 1000001001;
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPresentInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPresentInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPushConstantRange {
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkQueryPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkQueryPoolCreateFlags,
    pub queryType: VkQueryType,
    pub queryCount: u32,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: VkStructureType = 11;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkQueryPoolCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkQueryPoolCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkQueueFamilyProperties {
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkQueueFamilyProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub queueFamilyProperties: VkQueueFamilyProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR: VkStructureType = 1000059005;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkQueueFamilyProperties2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkQueueFamilyProperties2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkQueueFamilyProperties2 = VkQueueFamilyProperties2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkRenderPassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub renderPass: VkRenderPass,
    pub framebuffer: VkFramebuffer,
    pub renderArea: VkRect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const VkClearValue,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: VkStructureType = 43;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkRenderPassBeginInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkRenderPassBeginInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkRenderPassCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: VkStructureType = 38;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkRenderPassCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkRenderPassCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO; }

#[cfg(feature = "VK_KHR_maintenance2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkRenderPassInputAttachmentAspectCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub aspectReferenceCount: u32,
    pub pAspectReferences: *const VkInputAttachmentAspectReferenceKHR,
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: VkStructureType = 1000117001;
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkRenderPassInputAttachmentAspectCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkRenderPassInputAttachmentAspectCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkRenderPassInputAttachmentAspectCreateInfo = VkRenderPassInputAttachmentAspectCreateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_multiview")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkRenderPassMultiviewCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub subpassCount: u32,
    pub pViewMasks: *const u32,
    pub dependencyCount: u32,
    pub pViewOffsets: *const i32,
    pub correlationMaskCount: u32,
    pub pCorrelationMasks: *const u32,
}
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: VkStructureType = 1000053000;
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkRenderPassMultiviewCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_multiview")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkRenderPassMultiviewCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkRenderPassMultiviewCreateInfo = VkRenderPassMultiviewCreateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSamplerCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSamplerCreateFlags,
    pub magFilter: VkFilter,
    pub minFilter: VkFilter,
    pub mipmapMode: VkSamplerMipmapMode,
    pub addressModeU: VkSamplerAddressMode,
    pub addressModeV: VkSamplerAddressMode,
    pub addressModeW: VkSamplerAddressMode,
    pub mipLodBias: core::ffi::c_float,
    pub anisotropyEnable: VkBool32,
    pub maxAnisotropy: core::ffi::c_float,
    pub compareEnable: VkBool32,
    pub compareOp: VkCompareOp,
    pub minLod: core::ffi::c_float,
    pub maxLod: core::ffi::c_float,
    pub borderColor: VkBorderColor,
    pub unnormalizedCoordinates: VkBool32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: VkStructureType = 31;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSamplerCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSamplerCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO; }

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSamplerYcbcrConversionCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub format: VkFormat,
    pub ycbcrModel: VkSamplerYcbcrModelConversionKHR,
    pub ycbcrRange: VkSamplerYcbcrRangeKHR,
    pub components: VkComponentMapping,
    pub xChromaOffset: VkChromaLocationKHR,
    pub yChromaOffset: VkChromaLocationKHR,
    pub chromaFilter: VkFilter,
    pub forceExplicitReconstruction: VkBool32,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: VkStructureType = 1000156000;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSamplerYcbcrConversionCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSamplerYcbcrConversionCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSamplerYcbcrConversionCreateInfo = VkSamplerYcbcrConversionCreateInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSamplerYcbcrConversionImageFormatPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub combinedImageSamplerDescriptorCount: u32,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: VkStructureType = 1000156005;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkSamplerYcbcrConversionImageFormatPropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkSamplerYcbcrConversionImageFormatPropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSamplerYcbcrConversionImageFormatProperties = VkSamplerYcbcrConversionImageFormatPropertiesKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR;

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSamplerYcbcrConversionInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub conversion: VkSamplerYcbcrConversion,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR: VkStructureType = 1000156001;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSamplerYcbcrConversionInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSamplerYcbcrConversionInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSamplerYcbcrConversionInfo = VkSamplerYcbcrConversionInfoKHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO: VkStructureType = VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSemaphoreCreateFlags,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: VkStructureType = 9;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSemaphoreCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSemaphoreCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO; }

#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSemaphoreGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR: VkStructureType = 1000079001;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSemaphoreGetFdInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSemaphoreGetFdInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR; }

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphore: VkSemaphore,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000078003;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSemaphoreGetWin32HandleInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSemaphoreGetWin32HandleInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkShaderModuleCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub codeSize: usize,
    pub pCode: *const u32,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType = 16;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkShaderModuleCreateInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkShaderModuleCreateInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSparseBufferMemoryBindInfo {
    pub buffer: VkBuffer,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSparseImageFormatProperties {
    pub aspectMask: VkImageAspectFlags,
    pub imageGranularity: VkExtent3D,
    pub flags: VkSparseImageFormatFlags,
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSparseImageFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub properties: VkSparseImageFormatProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType = 1000059007;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkSparseImageFormatProperties2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkSparseImageFormatProperties2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSparseImageFormatProperties2 = VkSparseImageFormatProperties2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSparseImageMemoryBind {
    pub subresource: VkImageSubresource,
    pub offset: VkOffset3D,
    pub extent: VkExtent3D,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSparseImageMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseImageMemoryBind,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSparseImageMemoryRequirements {
    pub formatProperties: VkSparseImageFormatProperties,
    pub imageMipTailFirstLod: u32,
    pub imageMipTailSize: VkDeviceSize,
    pub imageMipTailOffset: VkDeviceSize,
    pub imageMipTailStride: VkDeviceSize,
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSparseImageMemoryRequirements2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub memoryRequirements: VkSparseImageMemoryRequirements,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: VkStructureType = 1000146004;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkSparseImageMemoryRequirements2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkSparseImageMemoryRequirements2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR; }
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub type VkSparseImageMemoryRequirements2 = VkSparseImageMemoryRequirements2KHR;
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: VkStructureType = VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSparseImageOpaqueMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSparseMemoryBind {
    pub resourceOffset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: usize,
    pub pData: *const core::ffi::c_void,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: usize,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkStencilOpState {
    pub failOp: VkStencilOp,
    pub passOp: VkStencilOp,
    pub depthFailOp: VkStencilOp,
    pub compareOp: VkCompareOp,
    pub compareMask: u32,
    pub writeMask: u32,
    pub reference: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub pWaitDstStageMask: *const VkPipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const VkCommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = 4;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSubmitInfo {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSubmitInfo { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SUBMIT_INFO; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSubpassDescription {
    pub flags: VkSubpassDescriptionFlags,
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

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSubresourceLayout {
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub rowPitch: VkDeviceSize,
    pub arrayPitch: VkDeviceSize,
    pub depthPitch: VkDeviceSize,
}

#[cfg(feature = "VK_KHR_surface")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
}

#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSurfaceCapabilities2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub surfaceCapabilities: VkSurfaceCapabilitiesKHR,
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR: VkStructureType = 1000119001;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkSurfaceCapabilities2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkSurfaceCapabilities2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR; }

#[cfg(feature = "VK_KHR_surface")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}

#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSurfaceFormat2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub surfaceFormat: VkSurfaceFormatKHR,
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR: VkStructureType = 1000119002;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkSurfaceFormat2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkSurfaceFormat2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR; }

#[cfg(feature = "VK_KHR_swapchain")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: VkSurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: VkFormat,
    pub imageColorSpace: VkColorSpaceKHR,
    pub imageExtent: VkExtent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: VkImageUsageFlags,
    pub imageSharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: VkSurfaceTransformFlagBitsKHR,
    pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
    pub presentMode: VkPresentModeKHR,
    pub clipped: VkBool32,
    pub oldSwapchain: Option<VkSwapchainKHR>,
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = 1000001000;
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSwapchainCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSwapchainCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR; }

#[derive(Debug, Clone, Copy)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VkVertexInputRate,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkViewport {
    pub x: core::ffi::c_float,
    pub y: core::ffi::c_float,
    pub width: core::ffi::c_float,
    pub height: core::ffi::c_float,
    pub minDepth: core::ffi::c_float,
    pub maxDepth: core::ffi::c_float,
}

#[cfg(feature = "VK_KHR_wayland_surface")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkWaylandSurfaceCreateFlagsKHR,
    pub display: *mut core::ffi::c_void,
    pub surface: *mut core::ffi::c_void,
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000006000;
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkWaylandSurfaceCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkWaylandSurfaceCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR; }

#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub acquireCount: u32,
    pub pAcquireSyncs: *const VkDeviceMemory,
    pub pAcquireKeys: *const u64,
    pub pAcquireTimeouts: *const u32,
    pub releaseCount: u32,
    pub pReleaseSyncs: *const VkDeviceMemory,
    pub pReleaseKeys: *const u64,
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: VkStructureType = 1000075000;
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkWin32KeyedMutexAcquireReleaseInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkWin32KeyedMutexAcquireReleaseInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR; }

#[cfg(feature = "VK_KHR_win32_surface")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: windows::Win32::Foundation::HINSTANCE,
    pub hwnd: windows::Win32::Foundation::HWND,
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000009000;
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkWin32SurfaceCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkWin32SurfaceCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR; }

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkWriteDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub pImageInfo: *const VkDescriptorImageInfo,
    pub pBufferInfo: *const VkDescriptorBufferInfo,
    pub pTexelBufferView: *const VkBufferView,
}
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: VkStructureType = 35;
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkWriteDescriptorSet {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkWriteDescriptorSet { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET; }

#[cfg(feature = "VK_KHR_xcb_surface")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb::ffi::xcb_connection_t,
    pub window: xcb::x::Window,
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000005000;
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkXcbSurfaceCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkXcbSurfaceCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR; }

#[cfg(feature = "VK_KHR_xlib_surface")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkXlibSurfaceCreateFlagsKHR,
    pub dpy: *mut x11::xlib::Display,
    pub window: x11::xlib::Window,
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000004000;
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkXlibSurfaceCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkXlibSurfaceCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR; }

#[cfg(feature = "VK_KHR_synchronization2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryBarrier2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcStageMask: VkPipelineStageFlags2KHR,
    pub srcAccessMask: VkAccessFlags2KHR,
    pub dstStageMask: VkPipelineStageFlags2KHR,
    pub dstAccessMask: VkAccessFlags2KHR,
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR: VkStructureType = 1000314000;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkMemoryBarrier2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkMemoryBarrier2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR; }
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkMemoryBarrier2 = VkMemoryBarrier2KHR;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR;

#[cfg(feature = "VK_KHR_synchronization2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBufferMemoryBarrier2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcStageMask: VkPipelineStageFlags2KHR,
    pub srcAccessMask: VkAccessFlags2KHR,
    pub dstStageMask: VkPipelineStageFlags2KHR,
    pub dstAccessMask: VkAccessFlags2KHR,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR: VkStructureType = 1000314001;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBufferMemoryBarrier2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBufferMemoryBarrier2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR; }
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkBufferMemoryBarrier2 = VkBufferMemoryBarrier2KHR;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR;

#[cfg(feature = "VK_KHR_synchronization2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageMemoryBarrier2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcStageMask: VkPipelineStageFlags2KHR,
    pub srcAccessMask: VkAccessFlags2KHR,
    pub dstStageMask: VkPipelineStageFlags2KHR,
    pub dstAccessMask: VkAccessFlags2KHR,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR: VkStructureType = 1000314002;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageMemoryBarrier2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageMemoryBarrier2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR; }
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkImageMemoryBarrier2 = VkImageMemoryBarrier2KHR;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR;

#[cfg(feature = "VK_KHR_synchronization2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDependencyInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub dependencyFlags: VkDependencyFlags,
    pub memoryBarrierCount: u32,
    pub pMemoryBarriers: *const VkMemoryBarrier2KHR,
    pub bufferMemoryBarrierCount: u32,
    pub pBufferMemoryBarriers: *const VkBufferMemoryBarrier2KHR,
    pub imageMemoryBarrierCount: u32,
    pub pImageMemoryBarriers: *const VkImageMemoryBarrier2KHR,
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR: VkStructureType = 1000314003;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDependencyInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDependencyInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR; }
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkDependencyInfo = VkDependencyInfoKHR;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO: VkStructureType = VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR;

#[cfg(feature = "VK_KHR_synchronization2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSubmitInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSubmitFlagsKHR,
    pub waitSemaphoreInfoCount: u32,
    pub pWaitSemaphoreInfos: *const VkSemaphoreSubmitInfoKHR,
    pub commandBufferInfoCount: u32,
    pub pCommandBufferInfos: *const VkCommandBufferSubmitInfoKHR,
    pub signalSemaphoreInfoCount: u32,
    pub pSignalSemaphoreInfos: *const VkSemaphoreSubmitInfoKHR,
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR: VkStructureType = 1000314004;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSubmitInfo2KHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSubmitInfo2KHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR; }
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkSubmitInfo2 = VkSubmitInfo2KHR;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2: VkStructureType = VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR;

#[cfg(feature = "VK_KHR_synchronization2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSemaphoreSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphore: VkSemaphore,
    pub value: u64,
    pub stageMask: VkPipelineStageFlags2KHR,
    pub deviceIndex: u32,
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR: VkStructureType = 1000314005;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSemaphoreSubmitInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSemaphoreSubmitInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR; }
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkSemaphoreSubmitInfo = VkSemaphoreSubmitInfoKHR;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR;

#[cfg(feature = "VK_KHR_synchronization2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkCommandBufferSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub commandBuffer: VkCommandBuffer,
    pub deviceMask: u32,
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR: VkStructureType = 1000314006;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkCommandBufferSubmitInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkCommandBufferSubmitInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR; }
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkCommandBufferSubmitInfo = VkCommandBufferSubmitInfoKHR;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO: VkStructureType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR;

#[cfg(feature = "VK_KHR_synchronization2")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceSynchronization2FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub synchronization2: VkBool32,
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: VkStructureType = 1000314007;
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceSynchronization2FeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceSynchronization2FeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR; }
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceSynchronization2FeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceSynchronization2FeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR; }
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceSynchronization2Features = VkPhysicalDeviceSynchronization2FeaturesKHR;
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR;
#[cfg(any(feature = "VK_KHR_external_fence_capabilities", feature = "VK_KHR_external_memory_capabilities", feature = "VK_KHR_external_semaphore_capabilities"))]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceIDPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE],
    pub driverUUID: [u8; VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE_KHR],
    pub deviceNodeMask: u32,
    pub deviceLUIDValid: VkBool32,
}
#[cfg(any(feature = "VK_KHR_external_fence_capabilities", feature = "VK_KHR_external_memory_capabilities", feature = "VK_KHR_external_semaphore_capabilities"))]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR: VkStructureType = 1000071004;
#[cfg(any(feature = "VK_KHR_external_fence_capabilities", feature = "VK_KHR_external_memory_capabilities", feature = "VK_KHR_external_semaphore_capabilities"))]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceIDPropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(any(feature = "VK_KHR_external_fence_capabilities", feature = "VK_KHR_external_memory_capabilities", feature = "VK_KHR_external_semaphore_capabilities"))]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceIDPropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR; }
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

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateInstance(pub unsafe extern "system" fn(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateInstance {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateInstance";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateInstance {
    const STATIC: Self = Self(vkCreateInstance);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyInstance(pub unsafe extern "system" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyInstance {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyInstance";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyInstance {
    const STATIC: Self = Self(vkDestroyInstance);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkEnumeratePhysicalDevices(pub unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkEnumeratePhysicalDevices {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkEnumeratePhysicalDevices";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkEnumeratePhysicalDevices {
    const STATIC: Self = Self(vkEnumeratePhysicalDevices);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceFeatures(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceFeatures {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceFeatures";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceFeatures {
    const STATIC: Self = Self(vkGetPhysicalDeviceFeatures);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceFormatProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceFormatProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceFormatProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceFormatProperties {
    const STATIC: Self = Self(vkGetPhysicalDeviceFormatProperties);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceImageFormatProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceImageFormatProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceImageFormatProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceImageFormatProperties {
    const STATIC: Self = Self(vkGetPhysicalDeviceImageFormatProperties);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceProperties {
    const STATIC: Self = Self(vkGetPhysicalDeviceProperties);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceQueueFamilyProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceQueueFamilyProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceQueueFamilyProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceQueueFamilyProperties {
    const STATIC: Self = Self(vkGetPhysicalDeviceQueueFamilyProperties);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceMemoryProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceMemoryProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceMemoryProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceMemoryProperties {
    const STATIC: Self = Self(vkGetPhysicalDeviceMemoryProperties);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetInstanceProcAddr(pub unsafe extern "system" fn(instance: VkInstance, pName: *const core::ffi::c_char) -> Option<PFN_vkVoidFunction>);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetInstanceProcAddr {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetInstanceProcAddr";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetInstanceProcAddr {
    const STATIC: Self = Self(vkGetInstanceProcAddr);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDeviceProcAddr(pub unsafe extern "system" fn(device: VkDevice, pName: *const core::ffi::c_char) -> Option<PFN_vkVoidFunction>);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDeviceProcAddr {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDeviceProcAddr";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetDeviceProcAddr {
    const STATIC: Self = Self(vkGetDeviceProcAddr);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateDevice(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateDevice {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateDevice";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateDevice {
    const STATIC: Self = Self(vkCreateDevice);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyDevice(pub unsafe extern "system" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyDevice {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyDevice";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyDevice {
    const STATIC: Self = Self(vkDestroyDevice);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkEnumerateInstanceExtensionProperties(pub unsafe extern "system" fn(pLayerName: *const core::ffi::c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkEnumerateInstanceExtensionProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkEnumerateInstanceExtensionProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkEnumerateInstanceExtensionProperties {
    const STATIC: Self = Self(vkEnumerateInstanceExtensionProperties);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkEnumerateDeviceExtensionProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pLayerName: *const core::ffi::c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkEnumerateDeviceExtensionProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkEnumerateDeviceExtensionProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkEnumerateDeviceExtensionProperties {
    const STATIC: Self = Self(vkEnumerateDeviceExtensionProperties);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkEnumerateInstanceLayerProperties(pub unsafe extern "system" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkEnumerateInstanceLayerProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkEnumerateInstanceLayerProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkEnumerateInstanceLayerProperties {
    const STATIC: Self = Self(vkEnumerateInstanceLayerProperties);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkEnumerateDeviceLayerProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkEnumerateDeviceLayerProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkEnumerateDeviceLayerProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkEnumerateDeviceLayerProperties {
    const STATIC: Self = Self(vkEnumerateDeviceLayerProperties);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDeviceQueue(pub unsafe extern "system" fn(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDeviceQueue {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDeviceQueue";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetDeviceQueue {
    const STATIC: Self = Self(vkGetDeviceQueue);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkQueueSubmit(pub unsafe extern "system" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: Option<VkFence>) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkQueueSubmit {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkQueueSubmit";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkQueueSubmit {
    const STATIC: Self = Self(vkQueueSubmit);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkQueueWaitIdle(pub unsafe extern "system" fn(queue: VkQueue) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkQueueWaitIdle {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkQueueWaitIdle";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkQueueWaitIdle {
    const STATIC: Self = Self(vkQueueWaitIdle);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDeviceWaitIdle(pub unsafe extern "system" fn(device: VkDevice) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDeviceWaitIdle {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDeviceWaitIdle";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDeviceWaitIdle {
    const STATIC: Self = Self(vkDeviceWaitIdle);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkAllocateMemory(pub unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkAllocateMemory {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkAllocateMemory";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkAllocateMemory {
    const STATIC: Self = Self(vkAllocateMemory);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkFreeMemory(pub unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkFreeMemory {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkFreeMemory";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkFreeMemory {
    const STATIC: Self = Self(vkFreeMemory);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkMapMemory(pub unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut core::ffi::c_void) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkMapMemory {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkMapMemory";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkMapMemory {
    const STATIC: Self = Self(vkMapMemory);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkUnmapMemory(pub unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkUnmapMemory {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkUnmapMemory";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkUnmapMemory {
    const STATIC: Self = Self(vkUnmapMemory);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkFlushMappedMemoryRanges(pub unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkFlushMappedMemoryRanges {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkFlushMappedMemoryRanges";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkFlushMappedMemoryRanges {
    const STATIC: Self = Self(vkFlushMappedMemoryRanges);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkInvalidateMappedMemoryRanges(pub unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkInvalidateMappedMemoryRanges {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkInvalidateMappedMemoryRanges";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkInvalidateMappedMemoryRanges {
    const STATIC: Self = Self(vkInvalidateMappedMemoryRanges);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDeviceMemoryCommitment(pub unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pCommitmentMemoryInBytes: *mut VkDeviceSize));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDeviceMemoryCommitment {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDeviceMemoryCommitment";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetDeviceMemoryCommitment {
    const STATIC: Self = Self(vkGetDeviceMemoryCommitment);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkBindBufferMemory(pub unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkBindBufferMemory {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkBindBufferMemory";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkBindBufferMemory {
    const STATIC: Self = Self(vkBindBufferMemory);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkBindImageMemory(pub unsafe extern "system" fn(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkBindImageMemory {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkBindImageMemory";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkBindImageMemory {
    const STATIC: Self = Self(vkBindImageMemory);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetBufferMemoryRequirements(pub unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetBufferMemoryRequirements {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetBufferMemoryRequirements";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetBufferMemoryRequirements {
    const STATIC: Self = Self(vkGetBufferMemoryRequirements);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetImageMemoryRequirements(pub unsafe extern "system" fn(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetImageMemoryRequirements {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetImageMemoryRequirements";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetImageMemoryRequirements {
    const STATIC: Self = Self(vkGetImageMemoryRequirements);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetImageSparseMemoryRequirements(pub unsafe extern "system" fn(device: VkDevice, image: VkImage, pSparseMemoryRequirementsCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetImageSparseMemoryRequirements {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetImageSparseMemoryRequirements";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetImageSparseMemoryRequirements {
    const STATIC: Self = Self(vkGetImageSparseMemoryRequirements);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceSparseImageFormatProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSparseImageFormatProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceSparseImageFormatProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceSparseImageFormatProperties {
    const STATIC: Self = Self(vkGetPhysicalDeviceSparseImageFormatProperties);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkQueueBindSparse(pub unsafe extern "system" fn(queue: VkQueue, bindInfoCount: u32, pBindInfos: *const VkBindSparseInfo, fence: Option<VkFence>) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkQueueBindSparse {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkQueueBindSparse";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkQueueBindSparse {
    const STATIC: Self = Self(vkQueueBindSparse);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateFence(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateFence {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateFence";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateFence {
    const STATIC: Self = Self(vkCreateFence);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyFence(pub unsafe extern "system" fn(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyFence {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyFence";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyFence {
    const STATIC: Self = Self(vkDestroyFence);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkResetFences(pub unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkResetFences {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkResetFences";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkResetFences {
    const STATIC: Self = Self(vkResetFences);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetFenceStatus(pub unsafe extern "system" fn(device: VkDevice, fence: VkFence) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetFenceStatus {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetFenceStatus";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetFenceStatus {
    const STATIC: Self = Self(vkGetFenceStatus);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkWaitForFences(pub unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkWaitForFences {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkWaitForFences";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkWaitForFences {
    const STATIC: Self = Self(vkWaitForFences);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateSemaphore(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateSemaphore {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateSemaphore";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateSemaphore {
    const STATIC: Self = Self(vkCreateSemaphore);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroySemaphore(pub unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroySemaphore {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroySemaphore";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroySemaphore {
    const STATIC: Self = Self(vkDestroySemaphore);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateEvent(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateEvent {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateEvent";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateEvent {
    const STATIC: Self = Self(vkCreateEvent);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyEvent(pub unsafe extern "system" fn(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyEvent {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyEvent";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyEvent {
    const STATIC: Self = Self(vkDestroyEvent);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetEventStatus(pub unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetEventStatus {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetEventStatus";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetEventStatus {
    const STATIC: Self = Self(vkGetEventStatus);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkSetEvent(pub unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkSetEvent {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkSetEvent";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkSetEvent {
    const STATIC: Self = Self(vkSetEvent);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkResetEvent(pub unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkResetEvent {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkResetEvent";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkResetEvent {
    const STATIC: Self = Self(vkResetEvent);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateQueryPool(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateQueryPool {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateQueryPool";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateQueryPool {
    const STATIC: Self = Self(vkCreateQueryPool);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyQueryPool(pub unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyQueryPool {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyQueryPool";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyQueryPool {
    const STATIC: Self = Self(vkDestroyQueryPool);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetQueryPoolResults(pub unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut core::ffi::c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetQueryPoolResults {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetQueryPoolResults";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetQueryPoolResults {
    const STATIC: Self = Self(vkGetQueryPoolResults);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateBuffer(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateBuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateBuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateBuffer {
    const STATIC: Self = Self(vkCreateBuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyBuffer(pub unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyBuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyBuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyBuffer {
    const STATIC: Self = Self(vkDestroyBuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateBufferView(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateBufferView {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateBufferView";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateBufferView {
    const STATIC: Self = Self(vkCreateBufferView);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyBufferView(pub unsafe extern "system" fn(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyBufferView {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyBufferView";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyBufferView {
    const STATIC: Self = Self(vkDestroyBufferView);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateImage(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateImage {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateImage";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateImage {
    const STATIC: Self = Self(vkCreateImage);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyImage(pub unsafe extern "system" fn(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyImage {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyImage";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyImage {
    const STATIC: Self = Self(vkDestroyImage);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetImageSubresourceLayout(pub unsafe extern "system" fn(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetImageSubresourceLayout {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetImageSubresourceLayout";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetImageSubresourceLayout {
    const STATIC: Self = Self(vkGetImageSubresourceLayout);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateImageView(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateImageView {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateImageView";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateImageView {
    const STATIC: Self = Self(vkCreateImageView);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyImageView(pub unsafe extern "system" fn(device: VkDevice, view: VkImageView, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyImageView {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyImageView";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyImageView {
    const STATIC: Self = Self(vkDestroyImageView);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateShaderModule(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pModule: *mut VkShaderModule) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateShaderModule {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateShaderModule";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateShaderModule {
    const STATIC: Self = Self(vkCreateShaderModule);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyShaderModule(pub unsafe extern "system" fn(device: VkDevice, module: VkShaderModule, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyShaderModule {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyShaderModule";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyShaderModule {
    const STATIC: Self = Self(vkDestroyShaderModule);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreatePipelineCache(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreatePipelineCache {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreatePipelineCache";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreatePipelineCache {
    const STATIC: Self = Self(vkCreatePipelineCache);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyPipelineCache(pub unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyPipelineCache {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyPipelineCache";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyPipelineCache {
    const STATIC: Self = Self(vkDestroyPipelineCache);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPipelineCacheData(pub unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut core::ffi::c_void) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPipelineCacheData {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPipelineCacheData";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPipelineCacheData {
    const STATIC: Self = Self(vkGetPipelineCacheData);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkMergePipelineCaches(pub unsafe extern "system" fn(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkMergePipelineCaches {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkMergePipelineCaches";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkMergePipelineCaches {
    const STATIC: Self = Self(vkMergePipelineCaches);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateGraphicsPipelines(pub unsafe extern "system" fn(device: VkDevice, pipelineCache: Option<VkPipelineCache>, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateGraphicsPipelines {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateGraphicsPipelines";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateGraphicsPipelines {
    const STATIC: Self = Self(vkCreateGraphicsPipelines);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateComputePipelines(pub unsafe extern "system" fn(device: VkDevice, pipelineCache: Option<VkPipelineCache>, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateComputePipelines {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateComputePipelines";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateComputePipelines {
    const STATIC: Self = Self(vkCreateComputePipelines);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyPipeline(pub unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyPipeline {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyPipeline";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyPipeline {
    const STATIC: Self = Self(vkDestroyPipeline);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreatePipelineLayout(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreatePipelineLayout {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreatePipelineLayout";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreatePipelineLayout {
    const STATIC: Self = Self(vkCreatePipelineLayout);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyPipelineLayout(pub unsafe extern "system" fn(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyPipelineLayout {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyPipelineLayout";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyPipelineLayout {
    const STATIC: Self = Self(vkDestroyPipelineLayout);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateSampler(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateSampler {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateSampler";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateSampler {
    const STATIC: Self = Self(vkCreateSampler);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroySampler(pub unsafe extern "system" fn(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroySampler {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroySampler";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroySampler {
    const STATIC: Self = Self(vkDestroySampler);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateDescriptorSetLayout(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateDescriptorSetLayout {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateDescriptorSetLayout";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateDescriptorSetLayout {
    const STATIC: Self = Self(vkCreateDescriptorSetLayout);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyDescriptorSetLayout(pub unsafe extern "system" fn(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyDescriptorSetLayout {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyDescriptorSetLayout";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyDescriptorSetLayout {
    const STATIC: Self = Self(vkDestroyDescriptorSetLayout);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateDescriptorPool(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pPool: *mut VkDescriptorPool) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateDescriptorPool {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateDescriptorPool";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateDescriptorPool {
    const STATIC: Self = Self(vkCreateDescriptorPool);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyDescriptorPool(pub unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyDescriptorPool {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyDescriptorPool";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyDescriptorPool {
    const STATIC: Self = Self(vkDestroyDescriptorPool);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkResetDescriptorPool(pub unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkResetDescriptorPool {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkResetDescriptorPool";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkResetDescriptorPool {
    const STATIC: Self = Self(vkResetDescriptorPool);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkAllocateDescriptorSets(pub unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkAllocateDescriptorSets {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkAllocateDescriptorSets";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkAllocateDescriptorSets {
    const STATIC: Self = Self(vkAllocateDescriptorSets);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkFreeDescriptorSets(pub unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkFreeDescriptorSets {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkFreeDescriptorSets";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkFreeDescriptorSets {
    const STATIC: Self = Self(vkFreeDescriptorSets);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkUpdateDescriptorSets(pub unsafe extern "system" fn(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkUpdateDescriptorSets {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkUpdateDescriptorSets";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkUpdateDescriptorSets {
    const STATIC: Self = Self(vkUpdateDescriptorSets);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateFramebuffer(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateFramebuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateFramebuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateFramebuffer {
    const STATIC: Self = Self(vkCreateFramebuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyFramebuffer(pub unsafe extern "system" fn(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyFramebuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyFramebuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyFramebuffer {
    const STATIC: Self = Self(vkDestroyFramebuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateRenderPass(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateRenderPass {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateRenderPass";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateRenderPass {
    const STATIC: Self = Self(vkCreateRenderPass);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyRenderPass(pub unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyRenderPass {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyRenderPass";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyRenderPass {
    const STATIC: Self = Self(vkDestroyRenderPass);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetRenderAreaGranularity(pub unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetRenderAreaGranularity {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetRenderAreaGranularity";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetRenderAreaGranularity {
    const STATIC: Self = Self(vkGetRenderAreaGranularity);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateCommandPool(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateCommandPool {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateCommandPool";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateCommandPool {
    const STATIC: Self = Self(vkCreateCommandPool);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyCommandPool(pub unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyCommandPool {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyCommandPool";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyCommandPool {
    const STATIC: Self = Self(vkDestroyCommandPool);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkResetCommandPool(pub unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkResetCommandPool {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkResetCommandPool";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkResetCommandPool {
    const STATIC: Self = Self(vkResetCommandPool);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkAllocateCommandBuffers(pub unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkAllocateCommandBuffers {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkAllocateCommandBuffers";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkAllocateCommandBuffers {
    const STATIC: Self = Self(vkAllocateCommandBuffers);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkFreeCommandBuffers(pub unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkFreeCommandBuffers {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkFreeCommandBuffers";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkFreeCommandBuffers {
    const STATIC: Self = Self(vkFreeCommandBuffers);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkBeginCommandBuffer(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkBeginCommandBuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkBeginCommandBuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkBeginCommandBuffer {
    const STATIC: Self = Self(vkBeginCommandBuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkEndCommandBuffer(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkEndCommandBuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkEndCommandBuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkEndCommandBuffer {
    const STATIC: Self = Self(vkEndCommandBuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkResetCommandBuffer(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult);
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkResetCommandBuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkResetCommandBuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkResetCommandBuffer {
    const STATIC: Self = Self(vkResetCommandBuffer);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkEnumerateInstanceVersion(pub unsafe extern "system" fn(pApiVersion: *mut u32) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkEnumerateInstanceVersion {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkEnumerateInstanceVersion";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkEnumerateInstanceVersion {
    const STATIC: Self = Self(vkEnumerateInstanceVersion);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroySurfaceKHR(pub unsafe extern "system" fn(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroySurfaceKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroySurfaceKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroySurfaceKHR {
    const STATIC: Self = Self(vkDestroySurfaceKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceSurfaceSupportKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSurfaceSupportKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceSurfaceSupportKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceSurfaceSupportKHR {
    const STATIC: Self = Self(vkGetPhysicalDeviceSurfaceSupportKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR {
    const STATIC: Self = Self(vkGetPhysicalDeviceSurfaceCapabilitiesKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceSurfaceFormatsKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatsCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSurfaceFormatsKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceSurfaceFormatsKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceSurfaceFormatsKHR {
    const STATIC: Self = Self(vkGetPhysicalDeviceSurfaceFormatsKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceSurfacePresentModesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSurfacePresentModesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceSurfacePresentModesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceSurfacePresentModesKHR {
    const STATIC: Self = Self(vkGetPhysicalDeviceSurfacePresentModesKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateSwapchainKHR(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateSwapchainKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateSwapchainKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateSwapchainKHR {
    const STATIC: Self = Self(vkCreateSwapchainKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroySwapchainKHR(pub unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroySwapchainKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroySwapchainKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroySwapchainKHR {
    const STATIC: Self = Self(vkDestroySwapchainKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetSwapchainImagesKHR(pub unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetSwapchainImagesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetSwapchainImagesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetSwapchainImagesKHR {
    const STATIC: Self = Self(vkGetSwapchainImagesKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkAcquireNextImageKHR(pub unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: Option<VkSemaphore>, fence: Option<VkFence>, pImageIndex: *mut u32) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkAcquireNextImageKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkAcquireNextImageKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkAcquireNextImageKHR {
    const STATIC: Self = Self(vkAcquireNextImageKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkQueuePresentKHR(pub unsafe extern "system" fn(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkQueuePresentKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkQueuePresentKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkQueuePresentKHR {
    const STATIC: Self = Self(vkQueuePresentKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceDisplayPropertiesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceDisplayPropertiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceDisplayPropertiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceDisplayPropertiesKHR {
    const STATIC: Self = Self(vkGetPhysicalDeviceDisplayPropertiesKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR {
    const STATIC: Self = Self(vkGetPhysicalDeviceDisplayPlanePropertiesKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDisplayPlaneSupportedDisplaysKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDisplayPlaneSupportedDisplaysKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDisplayPlaneSupportedDisplaysKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetDisplayPlaneSupportedDisplaysKHR {
    const STATIC: Self = Self(vkGetDisplayPlaneSupportedDisplaysKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDisplayModePropertiesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDisplayModePropertiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDisplayModePropertiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetDisplayModePropertiesKHR {
    const STATIC: Self = Self(vkGetDisplayModePropertiesKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateDisplayModeKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateDisplayModeKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateDisplayModeKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateDisplayModeKHR {
    const STATIC: Self = Self(vkCreateDisplayModeKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDisplayPlaneCapabilitiesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDisplayPlaneCapabilitiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDisplayPlaneCapabilitiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetDisplayPlaneCapabilitiesKHR {
    const STATIC: Self = Self(vkGetDisplayPlaneCapabilitiesKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateDisplayPlaneSurfaceKHR(pub unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateDisplayPlaneSurfaceKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateDisplayPlaneSurfaceKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateDisplayPlaneSurfaceKHR {
    const STATIC: Self = Self(vkCreateDisplayPlaneSurfaceKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display_swapchain")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateSharedSwapchainKHR(pub unsafe extern "system" fn(device: VkDevice, swapchainCount: u32, pCreateInfos: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchains: *mut VkSwapchainKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_display_swapchain")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateSharedSwapchainKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateSharedSwapchainKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xlib_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateXlibSurfaceKHR(pub unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateXlibSurfaceKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateXlibSurfaceKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateXlibSurfaceKHR {
    const STATIC: Self = Self(vkCreateXlibSurfaceKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xlib_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut x11::xlib::Display, visualID: x11::xlib::VisualID) -> VkBool32);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceXlibPresentationSupportKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR {
    const STATIC: Self = Self(vkGetPhysicalDeviceXlibPresentationSupportKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xcb_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateXcbSurfaceKHR(pub unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateXcbSurfaceKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateXcbSurfaceKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateXcbSurfaceKHR {
    const STATIC: Self = Self(vkCreateXcbSurfaceKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xcb_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb::ffi::xcb_connection_t, visual_id: xcb::x::Visualid) -> VkBool32);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceXcbPresentationSupportKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR {
    const STATIC: Self = Self(vkGetPhysicalDeviceXcbPresentationSupportKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_wayland_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateWaylandSurfaceKHR(pub unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateWaylandSurfaceKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateWaylandSurfaceKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateWaylandSurfaceKHR {
    const STATIC: Self = Self(vkCreateWaylandSurfaceKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_wayland_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut core::ffi::c_void) -> VkBool32);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceWaylandPresentationSupportKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR {
    const STATIC: Self = Self(vkGetPhysicalDeviceWaylandPresentationSupportKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_android_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateAndroidSurfaceKHR(pub unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateAndroidSurfaceKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateAndroidSurfaceKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateAndroidSurfaceKHR {
    const STATIC: Self = Self(vkCreateAndroidSurfaceKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_win32_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateWin32SurfaceKHR(pub unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateWin32SurfaceKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateWin32SurfaceKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateWin32SurfaceKHR {
    const STATIC: Self = Self(vkCreateWin32SurfaceKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_win32_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceWin32PresentationSupportKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR {
    const STATIC: Self = Self(vkGetPhysicalDeviceWin32PresentationSupportKHR);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_metal_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateMetalSurfaceEXT(pub unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkMetalSurfaceCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateMetalSurfaceEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateMetalSurfaceEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateMetalSurfaceEXT {
    const STATIC: Self = Self(vkCreateMetalSurfaceEXT);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetMemoryWin32HandleKHR(pub unsafe extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut windows::Win32::Foundation::HANDLE) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetMemoryWin32HandleKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetMemoryWin32HandleKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetMemoryWin32HandlePropertiesKHR(pub unsafe extern "system" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagsKHR, handle: windows::Win32::Foundation::HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetMemoryWin32HandlePropertiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetMemoryWin32HandlePropertiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetMemoryFdKHR(pub unsafe extern "system" fn(device: VkDevice, pGetFdInfo: *const VkMemoryGetFdInfoKHR, pFd: *mut core::ffi::c_int) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetMemoryFdKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetMemoryFdKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetMemoryFdPropertiesKHR(pub unsafe extern "system" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagsKHR, fd: core::ffi::c_int, pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetMemoryFdPropertiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetMemoryFdPropertiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkImportSemaphoreWin32HandleKHR(pub unsafe extern "system" fn(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkImportSemaphoreWin32HandleKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkImportSemaphoreWin32HandleKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetSemaphoreWin32HandleKHR(pub unsafe extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut windows::Win32::Foundation::HANDLE) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetSemaphoreWin32HandleKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetSemaphoreWin32HandleKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkImportSemaphoreFdKHR(pub unsafe extern "system" fn(device: VkDevice, pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkImportSemaphoreFdKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkImportSemaphoreFdKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetSemaphoreFdKHR(pub unsafe extern "system" fn(device: VkDevice, pGetFdInfo: *const VkSemaphoreGetFdInfoKHR, pFd: *mut core::ffi::c_int) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetSemaphoreFdKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetSemaphoreFdKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkImportFenceWin32HandleKHR(pub unsafe extern "system" fn(device: VkDevice, pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkImportFenceWin32HandleKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkImportFenceWin32HandleKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetFenceWin32HandleKHR(pub unsafe extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR, pHandle: *mut windows::Win32::Foundation::HANDLE) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetFenceWin32HandleKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetFenceWin32HandleKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkImportFenceFdKHR(pub unsafe extern "system" fn(device: VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkImportFenceFdKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkImportFenceFdKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetFenceFdKHR(pub unsafe extern "system" fn(device: VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut core::ffi::c_int) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetFenceFdKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetFenceFdKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceSurfaceCapabilities2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceSurfaceFormats2KHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceFormatsCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormat2KHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSurfaceFormats2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceSurfaceFormats2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_report")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateDebugReportCallbackEXT(pub unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pCallback: *mut VkDebugReportCallbackEXT) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateDebugReportCallbackEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateDebugReportCallbackEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_report")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyDebugReportCallbackEXT(pub unsafe extern "system" fn(instance: VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyDebugReportCallbackEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyDebugReportCallbackEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_report")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDebugReportMessageEXT(pub unsafe extern "system" fn(instance: VkInstance, flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: usize, messageCode: i32, pLayerPrefix: *const core::ffi::c_char, pMessage: *const core::ffi::c_char));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_report")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDebugReportMessageEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDebugReportMessageEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkSetDebugUtilsObjectNameEXT(pub unsafe extern "system" fn(device: VkDevice, pNameInfo: *const VkDebugUtilsObjectNameInfoEXT) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkSetDebugUtilsObjectNameEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkSetDebugUtilsObjectNameEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkSetDebugUtilsObjectTagEXT(pub unsafe extern "system" fn(device: VkDevice, pTagInfo: *const VkDebugUtilsObjectTagInfoEXT) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkSetDebugUtilsObjectTagEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkSetDebugUtilsObjectTagEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkQueueBeginDebugUtilsLabelEXT(pub unsafe extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkQueueBeginDebugUtilsLabelEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkQueueBeginDebugUtilsLabelEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkQueueEndDebugUtilsLabelEXT(pub unsafe extern "system" fn(queue: VkQueue));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkQueueEndDebugUtilsLabelEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkQueueEndDebugUtilsLabelEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkQueueInsertDebugUtilsLabelEXT(pub unsafe extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkQueueInsertDebugUtilsLabelEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkQueueInsertDebugUtilsLabelEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateDebugUtilsMessengerEXT(pub unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pDebugUtilsMessenger: *mut VkDebugUtilsMessengerEXT) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateDebugUtilsMessengerEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateDebugUtilsMessengerEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyDebugUtilsMessengerEXT(pub unsafe extern "system" fn(instance: VkInstance, debugUtilsMessenger: VkDebugUtilsMessengerEXT, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyDebugUtilsMessengerEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyDebugUtilsMessengerEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkSubmitDebugUtilsMessageEXT(pub unsafe extern "system" fn(instance: VkInstance, messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT, messageTypes: VkDebugUtilsMessageTypeFlagsEXT, pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkSubmitDebugUtilsMessageEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkSubmitDebugUtilsMessageEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceExternalBufferPropertiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceExternalBufferProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceExternalBufferProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceExternalBufferProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceExternalBufferProperties {
    const STATIC: Self = Self(vkGetPhysicalDeviceExternalBufferProperties);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHR, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceExternalSemaphoreProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHR, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceExternalSemaphoreProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceExternalSemaphoreProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceExternalSemaphoreProperties {
    const STATIC: Self = Self(vkGetPhysicalDeviceExternalSemaphoreProperties);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfoKHR, pExternalFenceProperties: *mut VkExternalFencePropertiesKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceExternalFencePropertiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceExternalFenceProperties(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfoKHR, pExternalFenceProperties: *mut VkExternalFencePropertiesKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceExternalFenceProperties {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceExternalFenceProperties";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceExternalFenceProperties {
    const STATIC: Self = Self(vkGetPhysicalDeviceExternalFenceProperties);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR(pub unsafe extern "system" fn(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDeviceGroupPeerMemoryFeaturesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDeviceGroupPeerMemoryFeatures(pub unsafe extern "system" fn(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDeviceGroupPeerMemoryFeatures {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDeviceGroupPeerMemoryFeatures";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetDeviceGroupPeerMemoryFeatures {
    const STATIC: Self = Self(vkGetDeviceGroupPeerMemoryFeatures);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDeviceGroupPresentCapabilitiesKHR(pub unsafe extern "system" fn(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDeviceGroupPresentCapabilitiesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDeviceGroupPresentCapabilitiesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDeviceGroupSurfacePresentModesKHR(pub unsafe extern "system" fn(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDeviceGroupSurfacePresentModesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDeviceGroupSurfacePresentModesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDevicePresentRectanglesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDevicePresentRectanglesKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDevicePresentRectanglesKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_swapchain")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkAcquireNextImage2KHR(pub unsafe extern "system" fn(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHR, pImageIndex: *mut u32) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkAcquireNextImage2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkAcquireNextImage2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceFeatures2KHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceFeatures2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceFeatures2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceFeatures2(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceFeatures2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceFeatures2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceFeatures2 {
    const STATIC: Self = Self(vkGetPhysicalDeviceFeatures2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceProperties2KHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceProperties2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceProperties2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceProperties2(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceProperties2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceProperties2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceProperties2 {
    const STATIC: Self = Self(vkGetPhysicalDeviceProperties2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceFormatProperties2KHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceFormatProperties2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceFormatProperties2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceFormatProperties2(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceFormatProperties2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceFormatProperties2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceFormatProperties2 {
    const STATIC: Self = Self(vkGetPhysicalDeviceFormatProperties2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceImageFormatProperties2KHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut VkImageFormatProperties2KHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceImageFormatProperties2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceImageFormatProperties2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceImageFormatProperties2(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut VkImageFormatProperties2KHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceImageFormatProperties2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceImageFormatProperties2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceImageFormatProperties2 {
    const STATIC: Self = Self(vkGetPhysicalDeviceImageFormatProperties2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceQueueFamilyProperties2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceQueueFamilyProperties2(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceQueueFamilyProperties2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceQueueFamilyProperties2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceQueueFamilyProperties2 {
    const STATIC: Self = Self(vkGetPhysicalDeviceQueueFamilyProperties2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceMemoryProperties2KHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceMemoryProperties2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceMemoryProperties2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceMemoryProperties2(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceMemoryProperties2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceMemoryProperties2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceMemoryProperties2 {
    const STATIC: Self = Self(vkGetPhysicalDeviceMemoryProperties2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceSparseImageFormatProperties2(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceSparseImageFormatProperties2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 {
    const STATIC: Self = Self(vkGetPhysicalDeviceSparseImageFormatProperties2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group_creation")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkEnumeratePhysicalDeviceGroupKHR(pub unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group_creation")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkEnumeratePhysicalDeviceGroupKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkEnumeratePhysicalDeviceGroupKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkEnumeratePhysicalDeviceGroup(pub unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkEnumeratePhysicalDeviceGroup {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkEnumeratePhysicalDeviceGroup";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkEnumeratePhysicalDeviceGroup {
    const STATIC: Self = Self(vkEnumeratePhysicalDeviceGroup);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetImageMemoryRequirements2KHR(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetImageMemoryRequirements2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetImageMemoryRequirements2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetImageMemoryRequirements2(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetImageMemoryRequirements2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetImageMemoryRequirements2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetImageMemoryRequirements2 {
    const STATIC: Self = Self(vkGetImageMemoryRequirements2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetBufferMemoryRequirements2KHR(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetBufferMemoryRequirements2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetBufferMemoryRequirements2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetBufferMemoryRequirements2(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetBufferMemoryRequirements2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetBufferMemoryRequirements2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetBufferMemoryRequirements2 {
    const STATIC: Self = Self(vkGetBufferMemoryRequirements2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetImageSparseMemoryRequirements2KHR(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2KHR, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetImageSparseMemoryRequirements2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetImageSparseMemoryRequirements2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetImageSparseMemoryRequirements2(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2KHR, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetImageSparseMemoryRequirements2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetImageSparseMemoryRequirements2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetImageSparseMemoryRequirements2 {
    const STATIC: Self = Self(vkGetImageSparseMemoryRequirements2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_bind_memory2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkBindBufferMemory2KHR(pub unsafe extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkBindBufferMemory2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkBindBufferMemory2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkBindBufferMemory2(pub unsafe extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkBindBufferMemory2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkBindBufferMemory2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkBindBufferMemory2 {
    const STATIC: Self = Self(vkBindBufferMemory2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_bind_memory2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkBindImageMemory2KHR(pub unsafe extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_bind_memory2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkBindImageMemory2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkBindImageMemory2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkBindImageMemory2(pub unsafe extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkBindImageMemory2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkBindImageMemory2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkBindImageMemory2 {
    const STATIC: Self = Self(vkBindImageMemory2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateDescriptorUpdateTemplateKHR(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateDescriptorUpdateTemplateKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateDescriptorUpdateTemplateKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateDescriptorUpdateTemplate(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateDescriptorUpdateTemplate {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateDescriptorUpdateTemplate";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateDescriptorUpdateTemplate {
    const STATIC: Self = Self(vkCreateDescriptorUpdateTemplate);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyDescriptorUpdateTemplateKHR(pub unsafe extern "system" fn(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyDescriptorUpdateTemplateKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyDescriptorUpdateTemplateKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyDescriptorUpdateTemplate(pub unsafe extern "system" fn(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyDescriptorUpdateTemplate {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyDescriptorUpdateTemplate";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroyDescriptorUpdateTemplate {
    const STATIC: Self = Self(vkDestroyDescriptorUpdateTemplate);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkUpdateDescriptorSetWithTemplateKHR(pub unsafe extern "system" fn(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const core::ffi::c_void));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkUpdateDescriptorSetWithTemplateKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkUpdateDescriptorSetWithTemplateKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkUpdateDescriptorSetWithTemplate(pub unsafe extern "system" fn(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const core::ffi::c_void));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkUpdateDescriptorSetWithTemplate {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkUpdateDescriptorSetWithTemplate";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkUpdateDescriptorSetWithTemplate {
    const STATIC: Self = Self(vkUpdateDescriptorSetWithTemplate);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateSamplerYcbcrConversionKHR(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: VkSamplerYcbcrConversionCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSamplerYcbcrConversionKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateSamplerYcbcrConversionKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateSamplerYcbcrConversionKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateSamplerYcbcrConversion(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: VkSamplerYcbcrConversionCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSamplerYcbcrConversionKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateSamplerYcbcrConversion {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateSamplerYcbcrConversion";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCreateSamplerYcbcrConversion {
    const STATIC: Self = Self(vkCreateSamplerYcbcrConversion);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroySamplerYcbcrConversionKHR(pub unsafe extern "system" fn(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversionKHR, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroySamplerYcbcrConversionKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroySamplerYcbcrConversionKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroySamplerYcbcrConversion(pub unsafe extern "system" fn(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversionKHR, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroySamplerYcbcrConversion {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroySamplerYcbcrConversion";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkDestroySamplerYcbcrConversion {
    const STATIC: Self = Self(vkDestroySamplerYcbcrConversion);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_maintenance1")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkTrimCommandPoolKHR(pub unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_maintenance1")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkTrimCommandPoolKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkTrimCommandPoolKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkTrimCommandPool(pub unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkTrimCommandPool {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkTrimCommandPool";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkTrimCommandPool {
    const STATIC: Self = Self(vkTrimCommandPool);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_maintenance3")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDescriptorSetLayoutSupportKHR(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_maintenance3")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDescriptorSetLayoutSupportKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDescriptorSetLayoutSupportKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDescriptorSetLayoutSupport(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDescriptorSetLayoutSupport {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDescriptorSetLayoutSupport";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetDescriptorSetLayoutSupport {
    const STATIC: Self = Self(vkGetDescriptorSetLayoutSupport);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkQueueSubmit2KHR(pub unsafe extern "system" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo2KHR, fence: Option<VkFence>) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkQueueSubmit2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkQueueSubmit2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkQueueSubmit2(pub unsafe extern "system" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo2KHR, fence: Option<VkFence>) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkQueueSubmit2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkQueueSubmit2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkQueueSubmit2 {
    const STATIC: Self = Self(vkQueueSubmit2);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdBindPipeline(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdBindPipeline {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdBindPipeline";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdBindPipeline {
    const STATIC: Self = Self(vkCmdBindPipeline);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetViewport(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetViewport {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetViewport";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetViewport {
    const STATIC: Self = Self(vkCmdSetViewport);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetScissor(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetScissor {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetScissor";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetScissor {
    const STATIC: Self = Self(vkCmdSetScissor);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetLineWidth(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineWidth: core::ffi::c_float));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetLineWidth {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetLineWidth";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetLineWidth {
    const STATIC: Self = Self(vkCmdSetLineWidth);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetDepthBias(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: core::ffi::c_float, depthBiasClamp: core::ffi::c_float, depthBiasSlopeFactor: core::ffi::c_float));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetDepthBias {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetDepthBias";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetDepthBias {
    const STATIC: Self = Self(vkCmdSetDepthBias);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetBlendConstants(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, blendConstants: *const core::ffi::c_float));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetBlendConstants {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetBlendConstants";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetBlendConstants {
    const STATIC: Self = Self(vkCmdSetBlendConstants);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetDepthBounds(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, minDepthBounds: core::ffi::c_float, maxDepthBounds: core::ffi::c_float));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetDepthBounds {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetDepthBounds";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetDepthBounds {
    const STATIC: Self = Self(vkCmdSetDepthBounds);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetStencilCompareMask(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetStencilCompareMask {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetStencilCompareMask";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetStencilCompareMask {
    const STATIC: Self = Self(vkCmdSetStencilCompareMask);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetStencilWriteMask(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetStencilWriteMask {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetStencilWriteMask";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetStencilWriteMask {
    const STATIC: Self = Self(vkCmdSetStencilWriteMask);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetStencilReference(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetStencilReference {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetStencilReference";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetStencilReference {
    const STATIC: Self = Self(vkCmdSetStencilReference);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdBindDescriptorSets(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdBindDescriptorSets {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdBindDescriptorSets";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdBindDescriptorSets {
    const STATIC: Self = Self(vkCmdBindDescriptorSets);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdBindIndexBuffer(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdBindIndexBuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdBindIndexBuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdBindIndexBuffer {
    const STATIC: Self = Self(vkCmdBindIndexBuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdBindVertexBuffers(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdBindVertexBuffers {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdBindVertexBuffers";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdBindVertexBuffers {
    const STATIC: Self = Self(vkCmdBindVertexBuffers);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdDraw(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdDraw {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdDraw";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdDraw {
    const STATIC: Self = Self(vkCmdDraw);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdDrawIndexed(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdDrawIndexed {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdDrawIndexed";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdDrawIndexed {
    const STATIC: Self = Self(vkCmdDrawIndexed);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdDrawIndirect(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdDrawIndirect {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdDrawIndirect";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdDrawIndirect {
    const STATIC: Self = Self(vkCmdDrawIndirect);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdDrawIndexedIndirect(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdDrawIndexedIndirect {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdDrawIndexedIndirect";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdDrawIndexedIndirect {
    const STATIC: Self = Self(vkCmdDrawIndexedIndirect);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdDispatch(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdDispatch {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdDispatch";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdDispatch {
    const STATIC: Self = Self(vkCmdDispatch);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdDispatchIndirect(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdDispatchIndirect {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdDispatchIndirect";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdDispatchIndirect {
    const STATIC: Self = Self(vkCmdDispatchIndirect);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdCopyBuffer(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdCopyBuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdCopyBuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdCopyBuffer {
    const STATIC: Self = Self(vkCmdCopyBuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdCopyImage(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdCopyImage {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdCopyImage";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdCopyImage {
    const STATIC: Self = Self(vkCmdCopyImage);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdBlitImage(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filters: VkFilter));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdBlitImage {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdBlitImage";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdBlitImage {
    const STATIC: Self = Self(vkCmdBlitImage);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdCopyBufferToImage(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdCopyBufferToImage {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdCopyBufferToImage";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdCopyBufferToImage {
    const STATIC: Self = Self(vkCmdCopyBufferToImage);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdCopyImageToBuffer(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdCopyImageToBuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdCopyImageToBuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdCopyImageToBuffer {
    const STATIC: Self = Self(vkCmdCopyImageToBuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdUpdateBuffer(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const core::ffi::c_void));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdUpdateBuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdUpdateBuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdUpdateBuffer {
    const STATIC: Self = Self(vkCmdUpdateBuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdFillBuffer(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, data: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdFillBuffer {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdFillBuffer";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdFillBuffer {
    const STATIC: Self = Self(vkCmdFillBuffer);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdClearColorImage(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdClearColorImage {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdClearColorImage";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdClearColorImage {
    const STATIC: Self = Self(vkCmdClearColorImage);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdClearDepthStencilImage(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdClearDepthStencilImage {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdClearDepthStencilImage";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdClearDepthStencilImage {
    const STATIC: Self = Self(vkCmdClearDepthStencilImage);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdClearAttachments(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdClearAttachments {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdClearAttachments";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdClearAttachments {
    const STATIC: Self = Self(vkCmdClearAttachments);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdResolveImage(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdResolveImage {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdResolveImage";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdResolveImage {
    const STATIC: Self = Self(vkCmdResolveImage);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetEvent(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetEvent {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetEvent";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetEvent {
    const STATIC: Self = Self(vkCmdSetEvent);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdResetEvent(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdResetEvent {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdResetEvent";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdResetEvent {
    const STATIC: Self = Self(vkCmdResetEvent);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdWaitEvents(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdWaitEvents {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdWaitEvents";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdWaitEvents {
    const STATIC: Self = Self(vkCmdWaitEvents);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdPipelineBarrier(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdPipelineBarrier {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdPipelineBarrier";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdPipelineBarrier {
    const STATIC: Self = Self(vkCmdPipelineBarrier);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdBeginQuery(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdBeginQuery {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdBeginQuery";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdBeginQuery {
    const STATIC: Self = Self(vkCmdBeginQuery);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdEndQuery(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdEndQuery {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdEndQuery";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdEndQuery {
    const STATIC: Self = Self(vkCmdEndQuery);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdResetQueryPool(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdResetQueryPool {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdResetQueryPool";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdResetQueryPool {
    const STATIC: Self = Self(vkCmdResetQueryPool);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdWriteTimestamp(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlags, queryPool: VkQueryPool, query: u32));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdWriteTimestamp {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdWriteTimestamp";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdWriteTimestamp {
    const STATIC: Self = Self(vkCmdWriteTimestamp);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdCopyQueryPoolResults(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdCopyQueryPoolResults {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdCopyQueryPoolResults";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdCopyQueryPoolResults {
    const STATIC: Self = Self(vkCmdCopyQueryPoolResults);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdPushConstants(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineLayout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const core::ffi::c_void));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdPushConstants {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdPushConstants";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdPushConstants {
    const STATIC: Self = Self(vkCmdPushConstants);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdBeginRenderPass(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdBeginRenderPass {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdBeginRenderPass";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdBeginRenderPass {
    const STATIC: Self = Self(vkCmdBeginRenderPass);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdNextSubpass(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdNextSubpass {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdNextSubpass";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdNextSubpass {
    const STATIC: Self = Self(vkCmdNextSubpass);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdEndRenderPass(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdEndRenderPass {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdEndRenderPass";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdEndRenderPass {
    const STATIC: Self = Self(vkCmdEndRenderPass);
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdExecuteCommands(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer));
#[cfg(feature = "Implements")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdExecuteCommands {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdExecuteCommands";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Implements")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdExecuteCommands {
    const STATIC: Self = Self(vkCmdExecuteCommands);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdBeginDebugUtilsLabelEXT(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdBeginDebugUtilsLabelEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdBeginDebugUtilsLabelEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdEndDebugUtilsLabelEXT(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdEndDebugUtilsLabelEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdEndDebugUtilsLabelEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdInsertDebugUtilsLabelEXT(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_debug_utils")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdInsertDebugUtilsLabelEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdInsertDebugUtilsLabelEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetDeviceMaskKHR(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, deviceMask: u32));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetDeviceMaskKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetDeviceMaskKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetDeviceMask(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, deviceMask: u32));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetDeviceMask {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetDeviceMask";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetDeviceMask {
    const STATIC: Self = Self(vkCmdSetDeviceMask);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdDispatchBaseKHR(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_device_group")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdDispatchBaseKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdDispatchBaseKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdDispatchBase(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdDispatchBase {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdDispatchBase";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdDispatchBase {
    const STATIC: Self = Self(vkCmdDispatchBase);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetEvent2KHR(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, pDependencyInfo: *const VkDependencyInfoKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetEvent2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetEvent2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetEvent2(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, pDependencyInfo: *const VkDependencyInfoKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetEvent2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetEvent2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdSetEvent2 {
    const STATIC: Self = Self(vkCmdSetEvent2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdResetEvent2KHR(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdResetEvent2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdResetEvent2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdResetEvent2(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2KHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdResetEvent2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdResetEvent2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdResetEvent2 {
    const STATIC: Self = Self(vkCmdResetEvent2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdWaitEvents2KHR(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, pDependencyInfos: *const VkDependencyInfoKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdWaitEvents2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdWaitEvents2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdWaitEvents2(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, pDependencyInfos: *const VkDependencyInfoKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdWaitEvents2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdWaitEvents2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdWaitEvents2 {
    const STATIC: Self = Self(vkCmdWaitEvents2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdPipelineBarrier2KHR(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfoKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdPipelineBarrier2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdPipelineBarrier2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdPipelineBarrier2(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfoKHR));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdPipelineBarrier2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdPipelineBarrier2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdPipelineBarrier2 {
    const STATIC: Self = Self(vkCmdPipelineBarrier2);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdWriteTimestamp2KHR(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2KHR, queryPool: VkQueryPool, query: u32));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_synchronization2")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdWriteTimestamp2KHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdWriteTimestamp2KHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdWriteTimestamp2(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2KHR, queryPool: VkQueryPool, query: u32));
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdWriteTimestamp2 {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdWriteTimestamp2";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkCmdWriteTimestamp2 {
    const STATIC: Self = Self(vkCmdWriteTimestamp2);
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

#[cfg(feature = "VK_KHR_display_swapchain")]
#[rustfmt::skip]
pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_display_swapchain";
#[cfg(feature = "VK_KHR_display_swapchain")]
#[rustfmt::skip]
pub const VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: usize = 1;

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

#[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
#[rustfmt::skip]
pub const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME: &str = "VK_KHR_sampler_mirror_clamp_to_edge";
#[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
#[rustfmt::skip]
pub const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION: usize = 1;

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

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: &str = "VK_KHR_sampler_ycbcr_conversion";
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[rustfmt::skip]
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: usize = 14;

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

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &str = "VK_KHR_buffer_device_address";
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: &str = "VK_KHR_timeline_semaphore";
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: VkResult = VkResult(-1000257000);
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: VkResult = VkResult(-1000257000);

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

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBufferDeviceAddressInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub buffer: VkBuffer,
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR: VkStructureType = 1000244001;
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBufferDeviceAddressInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBufferDeviceAddressInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkBufferDeviceAddressInfo = VkBufferDeviceAddressInfoKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR;

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBufferOpaqueCaptureAddressCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub opaqueCaptureAddress: u64,
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: VkStructureType = 1000257002;
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBufferOpaqueCaptureAddressCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBufferOpaqueCaptureAddressCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkBufferOpaqueCaptureAddressCreateInfo = VkBufferOpaqueCaptureAddressCreateInfoKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub memory: VkDeviceMemory,
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: VkStructureType = 1000257004;
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceMemoryOpaqueCaptureAddressInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceMemoryOpaqueCaptureAddressInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkDeviceMemoryOpaqueCaptureAddressInfo = VkDeviceMemoryOpaqueCaptureAddressInfoKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR;

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub opaqueCaptureAddress: u64,
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: VkStructureType = 1000257003;
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkMemoryOpaqueCaptureAddressAllocateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkMemoryOpaqueCaptureAddressAllocateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkMemoryOpaqueCaptureAddressAllocateInfo = VkMemoryOpaqueCaptureAddressAllocateInfoKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR;

#[cfg(feature = "VK_KHR_buffer_device_address")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub bufferDeviceAddress: VkBool32,
    pub bufferDeviceAddressCaptureReplay: VkBool32,
    pub bufferDeviceAddressMultiDevice: VkBool32,
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: VkStructureType = 1000257000;
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceBufferDeviceAddressFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceBufferDeviceAddressFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR; }
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceBufferDeviceAddressFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceBufferDeviceAddressFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceBufferDeviceAddressFeatures = VkPhysicalDeviceBufferDeviceAddressFeaturesKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR;

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceTimelineSemaphoreFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub timelineSemaphore: VkBool32,
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: VkStructureType = 1000207000;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceTimelineSemaphoreFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceTimelineSemaphoreFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR; }
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceTimelineSemaphoreFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceTimelineSemaphoreFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceTimelineSemaphoreFeatures = VkPhysicalDeviceTimelineSemaphoreFeaturesKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR;

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceTimelineSemaphorePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxTimelineSemaphoreValueDifference: u64,
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: VkStructureType = 1000207001;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceTimelineSemaphorePropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceTimelineSemaphorePropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceTimelineSemaphoreProperties = VkPhysicalDeviceTimelineSemaphorePropertiesKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR;

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
pub struct VkPhysicalDeviceVulkan11Features {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub storageBuffer16BitAccess: VkBool32,
    pub uniformAndStorageBuffer16BitAccess: VkBool32,
    pub storagePushConstant16: VkBool32,
    pub storageInputOutput16: VkBool32,
    pub multiview: VkBool32,
    pub multiviewGeometryShader: VkBool32,
    pub multiviewTessellationShader: VkBool32,
    pub variablePointersStorageBuffer: VkBool32,
    pub variablePointers: VkBool32,
    pub protectedMemory: VkBool32,
    pub samplerYcbcrConversion: VkBool32,
    pub shaderDrawParameters: VkBool32,
}
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: VkStructureType = 49;
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceVulkan11Features {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
impl crate::TypedVulkanStructure for VkPhysicalDeviceVulkan11Features { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES; }
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVulkan11Features {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVulkan11Features { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES; }
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
impl Default for VkPhysicalDeviceVulkan11Features {
    #[inline(always)]
    fn default() -> Self {
        let mut p = core::mem::MaybeUninit::<Self>::zeroed();
        unsafe { core::ptr::addr_of_mut!((*p.as_mut_ptr()).sType).write(VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES); }
        unsafe { p.assume_init() }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
pub struct VkPhysicalDeviceVulkan11Properties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE],
    pub driverUUID: [u8; VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE],
    pub deviceNodeMask: u32,
    pub deviceLUIDValid: VkBool32,
    pub subgroupSize: u32,
    pub subgroupSupportedStages: VkShaderStageFlags,
    pub subgroupSupportedOperations: VkSubgroupFeatureFlags,
    pub subgroupQuadOperationsInAllStages: VkBool32,
    pub pointClippingBehavior: VkPointClippingBehavior,
    pub maxMultiviewViewCount: u32,
    pub maxMultiviewInstanceIndex: u32,
    pub protectedNoFault: VkBool32,
    pub maxPerSetDescriptors: u32,
    pub maxMemoryAllocationSize: VkDeviceSize,
}
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: VkStructureType = 50;
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVulkan11Properties {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[rustfmt::skip]
#[cfg(feature = "Allow1_2APIs")]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVulkan11Properties { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES; }

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSemaphoreSignalInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphore: VkSemaphore,
    pub value: u64,
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO_KHR: VkStructureType = 1000207005;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSemaphoreSignalInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSemaphoreSignalInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkSemaphoreSignalInfo = VkSemaphoreSignalInfoKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO_KHR;

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSemaphoreTypeCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphoreType: VkSemaphoreTypeKHR,
    pub initialValue: u64,
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO_KHR: VkStructureType = 1000207002;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSemaphoreTypeCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSemaphoreTypeCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkSemaphoreTypeCreateInfo = VkSemaphoreTypeCreateInfoKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSemaphoreWaitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSemaphoreWaitFlagsKHR,
    pub semaphoreCount: u32,
    pub pSemaphores: *const VkSemaphore,
    pub pValues: *const u64,
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO_KHR: VkStructureType = 1000207004;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSemaphoreWaitInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSemaphoreWaitInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkSemaphoreWaitInfo = VkSemaphoreWaitInfoKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO: VkStructureType = VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO_KHR;

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkTimelineSemaphoreSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub waitSemaphoreValueCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValueCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: VkStructureType = 1000207003;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkTimelineSemaphoreSubmitInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkTimelineSemaphoreSubmitInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkTimelineSemaphoreSubmitInfo = VkTimelineSemaphoreSubmitInfoKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO: VkStructureType = VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR;

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetBufferDeviceAddressKHR(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfoKHR) -> VkDeviceAddress);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetBufferDeviceAddressKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetBufferDeviceAddressKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetBufferDeviceAddress(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfoKHR) -> VkDeviceAddress);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetBufferDeviceAddress {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetBufferDeviceAddress";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetBufferDeviceAddress {
    const STATIC: Self = Self(vkGetBufferDeviceAddress);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetBufferOpaqueCaptureAddressKHR(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfoKHR) -> u64);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetBufferOpaqueCaptureAddressKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetBufferOpaqueCaptureAddressKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetBufferOpaqueCaptureAddress(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfoKHR) -> u64);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetBufferOpaqueCaptureAddress {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetBufferOpaqueCaptureAddress";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetBufferOpaqueCaptureAddress {
    const STATIC: Self = Self(vkGetBufferOpaqueCaptureAddress);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfoKHR) -> u64);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_buffer_device_address")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDeviceMemoryOpaqueCaptureAddressKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDeviceMemoryOpaqueCaptureAddress(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfoKHR) -> u64);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDeviceMemoryOpaqueCaptureAddress {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDeviceMemoryOpaqueCaptureAddress";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetDeviceMemoryOpaqueCaptureAddress {
    const STATIC: Self = Self(vkGetDeviceMemoryOpaqueCaptureAddress);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetSemaphoreCounterValueKHR(pub unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetSemaphoreCounterValueKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetSemaphoreCounterValueKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetSemaphoreCounterValue(pub unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetSemaphoreCounterValue {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetSemaphoreCounterValue";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkGetSemaphoreCounterValue {
    const STATIC: Self = Self(vkGetSemaphoreCounterValue);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkSignalSemaphoreKHR(pub unsafe extern "system" fn(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkSignalSemaphoreKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkSignalSemaphoreKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkSignalSemaphore(pub unsafe extern "system" fn(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfoKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkSignalSemaphore {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkSignalSemaphore";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkSignalSemaphore {
    const STATIC: Self = Self(vkSignalSemaphore);
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkWaitSemaphoresKHR(pub unsafe extern "system" fn(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfoKHR, timeout: u64) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkWaitSemaphoresKHR {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkWaitSemaphoresKHR";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkWaitSemaphores(pub unsafe extern "system" fn(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfoKHR, timeout: u64) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkWaitSemaphores {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkWaitSemaphores";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}
#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
impl crate::StaticCallable for PFN_vkWaitSemaphores {
    const STATIC: Self = Self(vkWaitSemaphores);
}

#[cfg(feature = "VK_KHR_image_format_list")]
#[rustfmt::skip]
pub const VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME: &str = "VK_KHR_image_format_list";
#[cfg(feature = "VK_KHR_image_format_list")]
#[rustfmt::skip]
pub const VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_image_format_list")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageFormatListCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub viewFormatCount: u32,
    pub pViewFormats: *const VkFormat,
}
#[cfg(feature = "VK_KHR_image_format_list")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR: VkStructureType = 1000147000;
#[cfg(feature = "VK_KHR_image_format_list")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageFormatListCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_image_format_list")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageFormatListCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub type VkImageFormatListCreateInfo = VkImageFormatListCreateInfoKHR;
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &str = "VK_KHR_vertex_attribute_divisor";
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxVertexAttribDivisor: u32,
    pub supportsNonZeroFirstInstance: VkBool32,
}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR: VkStructureType = 1000190000;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceVertexAttributeDivisorProperties = VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR;

#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkVertexInputBindingDivisorDescriptionKHR {
    pub binding: u32,
    pub divisor: u32,
}
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkVertexInputBindingDivisorDescription = VkVertexInputBindingDivisorDescriptionKHR;

#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineVertexInputDivisorStateCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub vertexBindingDivisorCount: u32,
    pub pVertexBindingDivisors: *const VkVertexInputBindingDivisorDescriptionKHR,
}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR: VkStructureType = 1000190001;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineVertexInputDivisorStateCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineVertexInputDivisorStateCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkPipelineVertexInputDivisorStateCreateInfo = VkPipelineVertexInputDivisorStateCreateInfoKHR;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub vertexAttributeInstanceRateDivisor: VkBool32,
    pub vertexAttributeInstanceRateZeroDivisor: VkBool32,
}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR: VkStructureType = 1000190002;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR; }
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR; }
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceVertexAttributeDivisorFeatures = VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR;

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

#[cfg(feature = "VK_KHR_global_priority")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub globalPriority: VkQueueGlobalPriorityKHR,
}
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: VkStructureType = 1000174000;
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceQueueGlobalPriorityCreateInfoKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceQueueGlobalPriorityCreateInfoKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR; }
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkDeviceQueueGlobalPriorityCreateInfo = VkDeviceQueueGlobalPriorityCreateInfoKHR;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR;

#[cfg(feature = "VK_KHR_global_priority")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub globalPriorityQuery: VkBool32,
}
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: VkStructureType = 1000388000;
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR; }
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR; }
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkPhysicalDeviceGlobalPriorityQueryFeatures = VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR;

#[cfg(feature = "VK_KHR_global_priority")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkQueueFamilyGlobalPriorityPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub priorityCount: u32,
    pub priorities: [VkQueueGlobalPriorityKHR; VK_MAX_GLOBAL_PRIORITY_SIZE_KHR],
}
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: VkStructureType = 1000388001;
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkQueueFamilyGlobalPriorityPropertiesKHR {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_KHR_global_priority")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkQueueFamilyGlobalPriorityPropertiesKHR { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR; }
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub type VkQueueFamilyGlobalPriorityProperties = VkQueueFamilyGlobalPriorityPropertiesKHR;
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR;

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

#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &str = "VK_EXT_blend_operation_advanced";
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: usize = 2;

#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub const VK_EXT_VALIDATION_CACHE_EXTENSION_NAME: &str = "VK_EXT_validation_cache";
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub const VK_EXT_VALIDATION_CACHE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub const VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME: &str = "VK_EXT_validation_flags";
#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub const VK_EXT_VALIDATION_FLAGS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_AMD_buffer_marker")]
#[rustfmt::skip]
pub const VK_AMD_BUFFER_MARKER_EXTENSION_NAME: &str = "VK_AMD_buffer_marker";
#[cfg(feature = "VK_AMD_buffer_marker")]
#[rustfmt::skip]
pub const VK_AMD_BUFFER_MARKER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_NV_acquire_winrt_display")]
#[rustfmt::skip]
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &str = "VK_NV_acquire_winrt_display";
#[cfg(feature = "VK_NV_acquire_winrt_display")]
#[rustfmt::skip]
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub type VkValidationCacheCreateFlagsEXT = VkFlags;
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub type VkValidationCacheCreateFlagBitsEXT = VkFlags;

#[cfg(feature = "VK_EXT_validation_cache")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
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
pub type VkValidationCacheHeaderVersionEXT = i32;
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: VkValidationCacheHeaderVersionEXT = 1;

#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub type VkValidationCheckEXT = i32;
#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub const VK_VALIDATION_CHECK_ALL_EXT: VkValidationCheckEXT = 0;
#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub const VK_VALIDATION_CHECK_SHADERS_EXT: VkValidationCheckEXT = 1;

#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceBlendOperationAdvanccedFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub advancedBlendCoherentOperations: VkBool32,
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: VkStructureType = 1000148000;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceBlendOperationAdvanccedFeaturesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceBlendOperationAdvanccedFeaturesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT; }
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceBlendOperationAdvanccedFeaturesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceBlendOperationAdvanccedFeaturesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT; }

#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub advancedBlendMaxColorAttachments: u32,
    pub advancedBlendIndependentBlend: VkBool32,
    pub advancedBlendNonPremultipliedSrcColor: VkBool32,
    pub advancedBlendNonPremultipliedDstColor: VkBool32,
    pub advancedBlendCorrelatedOverlap: VkBool32,
    pub advancedBlendAllOperations: VkBool32,
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: VkStructureType = 1000148001;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT; }

#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcPremultiplied: VkBool32,
    pub dstPremultiplied: VkBool32,
    pub blendOverlap: VkBlendOverlapEXT,
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: VkStructureType = 1000148002;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineColorBlendAdvancedStateCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT; }

#[cfg(feature = "VK_EXT_validation_cache")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkShaderModuleValidationcacheCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub validationCache: VkValidationCacheEXT,
}
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: VkStructureType = 1000160001;
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkShaderModuleValidationcacheCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkShaderModuleValidationcacheCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT; }

#[cfg(feature = "VK_EXT_validation_cache")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkValidationCacheCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkValidationCacheCreateFlagsEXT,
    pub initialDataSize: usize,
    pub pInitialData: *const core::ffi::c_void,
}
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT: VkStructureType = 1000160000;
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkValidationCacheCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkValidationCacheCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT; }

#[cfg(feature = "VK_EXT_validation_flags")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkValidationFlagsEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub disabledValidationCheckCount: u32,
    pub pDisabledValidationChecks: *mut VkValidationCheckEXT,
}
#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT: VkStructureType = 1000061000;
#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkValidationFlagsEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_validation_flags")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkValidationFlagsEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT; }

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_acquire_drm_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkAcquireDrmDisplayEXT(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, drmFd: i32, display: VkDisplayKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_acquire_drm_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkAcquireDrmDisplayEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkAcquireDrmDisplayEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_NV_acquire_winrt_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkAcquireWinrtDisplayNV(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_NV_acquire_winrt_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkAcquireWinrtDisplayNV {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkAcquireWinrtDisplayNV";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkAcquireXlibDisplayEXT(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, dpy: *mut x11::xlib::Display, display: VkDisplayKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkAcquireXlibDisplayEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkAcquireXlibDisplayEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_validation_cache")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCreateValidationCacheEXT(pub unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkValidationCacheCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pValidationCache: *mut VkValidationCacheEXT) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCreateValidationCacheEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCreateValidationCacheEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_validation_cache")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkDestroyValidationCacheEXT(pub unsafe extern "system" fn(device: VkDevice, validationCache: VkValidationCacheEXT, pAllocator: *const VkAllocationCallbacks));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkDestroyValidationCacheEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkDestroyValidationCacheEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_acquire_drm_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDrmDisplayEXT(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, drmFd: i32, connectorId: u32, pDisplay: *mut VkDisplayKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_acquire_drm_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDrmDisplayEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDrmDisplayEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetRandROutputDisplayEXT(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, dpy: *mut x11::xlib::Display, rrOutput: x11::xrandr::RROutput, pDisplay: *mut VkDisplayKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetRandROutputDisplayEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetRandROutputDisplayEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_validation_cache")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetValidationCacheDataEXT(pub unsafe extern "system" fn(device: VkDevice, validationCache: VkValidationCacheEXT, pDataSize: *mut usize, pData: *mut core::ffi::c_void) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetValidationCacheDataEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetValidationCacheDataEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_NV_acquire_winrt_display")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetWinrtDisplayNV(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, deviceRelativeId: u32, pDisplay: *mut VkDisplayKHR) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_NV_acquire_winrt_display")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetWinrtDisplayNV {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetWinrtDisplayNV";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_validation_cache")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkMergeValidationCachesEXT(pub unsafe extern "system" fn(device: VkDevice, dstCache: VkValidationCacheEXT, srcCacheCount: u32, pSrcCaches: *const VkValidationCacheEXT) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_validation_cache")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkMergeValidationCachesEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkMergeValidationCachesEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_AMD_buffer_marker")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdWriteBufferMarkerAMD(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlags, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, marker: u32));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_AMD_buffer_marker")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdWriteBufferMarkerAMD {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdWriteBufferMarkerAMD";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

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

#[cfg(feature = "VK_EXT_layer_settings")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkLayerSettingEXT {
    pub pLayerName: *const core::ffi::c_char,
    pub pSettingName: *const core::ffi::c_char,
    pub r#type: VkLayerSettingTypeEXT,
    pub valueCount: u32,
    pub pValues: *const core::ffi::c_void,
}

#[cfg(feature = "VK_EXT_layer_settings")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkLayerSettingsCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub settingCount: u32,
    pub pSettings: *const VkLayerSettingEXT,
}
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_LAYER_SETTINGS_CREATE_INFO_EXT: VkStructureType = 1000496000;
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkLayerSettingsCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_layer_settings")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkLayerSettingsCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_LAYER_SETTINGS_CREATE_INFO_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_EXT_DESCRIPTOR_BUFFER_EXTENSION_NAME: &str = "VK_EXT_descriptor_buffer";
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_EXT_DESCRIPTOR_BUFFER_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceDescriptorBufferPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub combinedImageSamplerDescriptorSingleArray: VkBool32,
    pub bufferlessPushDescriptors: VkBool32,
    pub allowSamplerImageViewPostSubmitCreation: VkBool32,
    pub descriptorBufferOffsetAlignment: VkDeviceSize,
    pub maxDescriptorBufferBindings: u32,
    pub maxResourceDescriptorBufferBindings: u32,
    pub maxSamplerDescriptorBufferBindings: u32,
    pub maxEmbeddedImmutableSamplerBindings: u32,
    pub maxEmbeddedImmutableSamplers: u32,
    pub bufferCaptureReplayDescriptorDataSize: usize,
    pub imageCaptureReplayDescriptorDataSize: usize,
    pub imageViewCaptureReplayDescriptorDataSize: usize,
    pub samplerCaptureReplayDescriptorDataSize: usize,
    pub accelerationStructureCaptureReplayDescriptorDataSize: usize,
    pub samplerDescriptorSize: usize,
    pub combinedImageSamplerDescriptorSize: usize,
    pub sampledImageDescriptorSize: usize,
    pub storageImageDescriptorSize: usize,
    pub uniformTexelBufferDescriptorSize: usize,
    pub robustUniformTexelBufferDescriptorSize: usize,
    pub storageTexelBufferDescriptorSize: usize,
    pub robustStorageTexelBufferDescriptorSize: usize,
    pub uniformBufferDescriptorSize: usize,
    pub robustUniformBufferDescriptorSize: usize,
    pub storageBufferDescriptorSize: usize,
    pub robustStorageBufferDescriptorSize: usize,
    pub inputAttachmentDescriptorSize: usize,
    pub accelerationStructureDescriptorSize: usize,
    pub maxSamplerDescriptorBufferRange: VkDeviceSize,
    pub maxResourceDescriptorBufferRange: VkDeviceSize,
    pub samplerDescriptorBufferAddressSpaceSize: VkDeviceSize,
    pub resourceDescriptorBufferAddressSpaceSize: VkDeviceSize,
    pub descriptorBufferAddressSpaceSize: VkDeviceSize,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT: VkStructureType = 1000316000;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDescriptorBufferPropertiesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDescriptorBufferPropertiesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceDescriptorBuferDensityMapPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub combinedImageSamplerDensityMapDescriptorSize: usize,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT: VkStructureType = 1000316001;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDescriptorBuferDensityMapPropertiesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDescriptorBuferDensityMapPropertiesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceDescriptorBufferFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub descriptorBuffer: VkBool32,
    pub descriptorBufferCaptureReplay: VkBool32,
    pub descriptorBufferImageLayoutIgnored: VkBool32,
    pub descriptorBufferPushDescriptors: VkBool32,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT: VkStructureType = 1000316002;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPhysicalDeviceDescriptorBufferFeaturesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPhysicalDeviceDescriptorBufferFeaturesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT; }
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceDescriptorBufferFeaturesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceDescriptorBufferFeaturesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorAddressInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub address: VkDeviceAddress,
    pub range: VkDeviceSize,
    pub format: VkFormat,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT: VkStructureType = 1000316003;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkDescriptorAddressInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkDescriptorAddressInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorBufferBindingInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub address: VkDeviceAddress,
    pub usage: VkBufferUsageFlags,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT: VkStructureType = 1000316011;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkDescriptorBufferBindingInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkDescriptorBufferBindingInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub buffer: VkBuffer,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT: VkStructureType = 1000316012;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkDescriptorBufferBindingPushDescriptorBufferHandleEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT; }

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

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDescriptorGetInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub r#type: VkDescriptorType,
    pub data: VkDescriptorDataEXT,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT: VkStructureType = 1000316004;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDescriptorGetInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDescriptorGetInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkBufferCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub buffer: VkBuffer,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType = 1000316005;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkBufferCaptureDescriptorDataInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkBufferCaptureDescriptorDataInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType = 1000316006;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageCaptureDescriptorDataInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageCaptureDescriptorDataInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImageViewCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub imageView: VkImageView,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType = 1000316007;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImageViewCaptureDescriptorDataInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImageViewCaptureDescriptorDataInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSamplerCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub sampler: VkSampler,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType = 1000316008;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSamplerCaptureDescriptorDataInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSamplerCaptureDescriptorDataInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT; }

#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkOpaqueCaptureDescriptorDataCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub opaqueCaptureDescriptorData: *const core::ffi::c_void,
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT: VkStructureType = 1000316010;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkOpaqueCaptureDescriptorDataCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkOpaqueCaptureDescriptorDataCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT; }

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDescriptorSetLayoutSizeEXT(pub unsafe extern "system" fn(device: VkDevice, layout: VkDescriptorSetLayout, pLayoutSizeInBytes: *mut VkDeviceSize));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDescriptorSetLayoutSizeEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDescriptorSetLayoutSizeEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDescriptorSetLayoutBindingOffsetEXT(pub unsafe extern "system" fn(device: VkDevice, layout: VkDescriptorSetLayout, binding: u32, pOffset: *mut VkDeviceSize));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDescriptorSetLayoutBindingOffsetEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDescriptorSetLayoutBindingOffsetEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetDescriptorEXT(pub unsafe extern "system" fn(device: VkDevice, pDescriptorInfo: *const VkDescriptorGetInfoEXT, dataSize: usize, pDescriptor: *mut core::ffi::c_void));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetDescriptorEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetDescriptorEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdBindDescriptorBuffersEXT(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, bufferCount: u32, pBindingInfos: *const VkDescriptorBufferBindingInfoEXT));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdBindDescriptorBuffersEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdBindDescriptorBuffersEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSetDescriptorBufferOffsetsEXT(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, setCount: u32, pBufferIndices: *const u32, pOffsets: *const VkDeviceSize));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSetDescriptorBufferOffsetsEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSetDescriptorBufferOffsetsEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferCaptureDescriptorDataInfoEXT, pData: *mut core::ffi::c_void) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetBufferOpaqueCaptureDescriptorDataEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetImageOpaqueCaptureDescriptorDataEXT(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageCaptureDescriptorDataInfoEXT, pData: *mut core::ffi::c_void) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetImageOpaqueCaptureDescriptorDataEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetImageOpaqueCaptureDescriptorDataEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageViewCaptureDescriptorDataInfoEXT, pData: *mut core::ffi::c_void) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetImageViewOpaqueCaptureDescriptorDataEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT(pub unsafe extern "system" fn(device: VkDevice, pInfo: *const VkSamplerCaptureDescriptorDataInfoEXT, pData: *mut core::ffi::c_void) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_descriptor_buffer")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetSamplerOpaqueCaptureDescriptorDataEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
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
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT_BIT_EXT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000080;
#[cfg(feature = "VK_EXT_external_memory_host")]
#[cfg(feature = "VK_KHR_external_memory")]
#[rustfmt::skip]
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT: VkExternalMemoryHandleTypeFlagBitsKHR = 0x00000100;

#[cfg(feature = "VK_EXT_external_memory_host")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkImportMemoryHostPointerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
    pub pHostPointer: *mut core::ffi::c_void,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT: VkStructureType = 1000178000;
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkImportMemoryHostPointerInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkImportMemoryHostPointerInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT; }

#[cfg(feature = "VK_EXT_external_memory_host")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMemoryHostPointerPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub memoryTypeBits: u32,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT: VkStructureType = 1000178001;
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkMemoryHostPointerPropertiesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkMemoryHostPointerPropertiesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT; }

#[cfg(feature = "VK_EXT_external_memory_host")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub minImportedHostPointerAlignment: VkDeviceSize,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: VkStructureType = 1000178002;
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceExternalMemoryHostPropertiesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT; }

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_external_memory_host")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetMemoryHostPointerPropertiesEXT(pub unsafe extern "system" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagsKHR, pHostPointer: *const core::ffi::c_void, pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT) -> VkResult);
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_external_memory_host")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetMemoryHostPointerPropertiesEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetMemoryHostPointerPropertiesEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &str = "VK_EXT_vertex_attribute_divisor";
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxVertexAttribDivisor: u32,
}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: VkStructureType = 1000190000;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT; }

#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkVertexInputBindingDivisorDescriptionEXT {
    pub binding: u32,
    pub divisor: u32,
}

#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub vertexBindingDivisorCount: u32,
    pub pVertexBindingDivisors: *const VkVertexInputBindingDivisorDescriptionEXT,
}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: VkStructureType = 1000190001;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineVertexInputDivisorStateCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineVertexInputDivisorStateCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT; }

#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &str = "VK_EXT_sample_locations";
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT: VkImageCreateFlagBits = 0x00001000;

#[cfg(feature = "VK_EXT_sample_locations")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSampleLocationEXT {
    pub x: core::ffi::c_float,
    pub y: core::ffi::c_float,
}

#[cfg(feature = "VK_EXT_sample_locations")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSampleLocationsInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub sampleLocationsPerPixel: VkSampleCountFlags,
    pub sampleLocationGridSize: VkExtent2D,
    pub sampleLocationsCount: u32,
    pub pSampleLocations: *const VkSampleLocationEXT,
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT: VkStructureType = 1000143000;
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkSampleLocationsInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkSampleLocationsInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT; }

#[cfg(feature = "VK_EXT_sample_locations")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkAttachmentSampleLocationsEXT {
    pub attachmentIndex: u32,
    pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}

#[cfg(feature = "VK_EXT_sample_locations")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkSubpassSampleLocationsEXT {
    pub subpassIndex: u32,
    pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}

#[cfg(feature = "VK_EXT_sample_locations")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub attachmentInitialSampleLocationsCount: u32,
    pub pAttachmentInitialSampleLocations: *const VkAttachmentSampleLocationsEXT,
    pub postSubpassSampleLocationsCount: u32,
    pub pPostSubpassSampleLocations: *const VkSubpassSampleLocationsEXT,
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: VkStructureType = 1000143001;
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkRenderPassSampleLocationsBeginInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkRenderPassSampleLocationsBeginInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT; }

#[cfg(feature = "VK_EXT_sample_locations")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub sampleLocationsEnable: VkBool32,
    pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: VkStructureType = 1000143002;
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineSampleLocationsStateCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineSampleLocationsStateCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT; }

#[cfg(feature = "VK_EXT_sample_locations")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub sampleLocationSampleCounts: VkSampleCountFlags,
    pub maxSampleLocationGridSize: VkExtent2D,
    pub sampleLocationCoordinateRange: [c_float; 2],
    pub sampleLocationSubpixelBits: u32,
    pub variableSampleLocations: VkBool32,
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: VkStructureType = 1000143003;
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceSampleLocationsPropertiesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceSampleLocationsPropertiesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT; }

#[cfg(feature = "VK_EXT_sample_locations")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkMultisamplePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxSampleLocationGridSize: VkExtent2D,
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT: VkStructureType = 1000143004;
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkMultisamplePropertiesEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkMultisamplePropertiesEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT; }

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_sample_locations")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkCmdSampleLocationsEXT(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pSampleLocationsInfo: *const VkSampleLocationsInfoEXT));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkCmdSampleLocationsEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkCmdSampleLocationsEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_sample_locations")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, samples: VkSampleCountFlags, pMultisampleProperties: *mut VkMultisamplePropertiesEXT));
#[cfg(feature = "Implements")]
#[cfg(feature = "VK_EXT_sample_locations")]
#[rustfmt::skip]
unsafe impl crate::PFN for PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT {
    const NAME_CSTR: &'static core::ffi::CStr = c"vkGetPhysicalDeviceMultisamplePropertiesEXT";

    #[inline(always)]
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self {
        unsafe { core::mem::transmute::<PFN_vkVoidFunction, Self>(p) }
    }
}

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

#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
    pub coverageToColorEnable: VkBool32,
    pub coverageToColorLocation: u32,
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: VkStructureType = 1000149000;
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineCoverageToColorStateCreateInfoNV {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineCoverageToColorStateCreateInfoNV { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV; }

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

#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineCoverageModulationStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
    pub coverageModulationMode: VkCoverageModulationModeNV,
    pub coverageModulationTableEnable: VkBool32,
    pub coverageModulationTableCount: u32,
    pub pCoverageModulationTable: *const core::ffi::c_float,
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: VkStructureType = 1000152000;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineCoverageModulationStateCreateInfoNV {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineCoverageModulationStateCreateInfoNV { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV; }

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

#[cfg(feature = "VK_EXT_global_priority")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkDeviceQueueGlobalPriorityCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub globalPriority: VkQueueGlobalPriorityEXT,
}
#[cfg(feature = "VK_EXT_global_priority")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: VkStructureType = 1000174000;
#[cfg(feature = "VK_EXT_global_priority")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkDeviceQueueGlobalPriorityCreateInfoEXT {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_global_priority")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkDeviceQueueGlobalPriorityCreateInfoEXT { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT; }

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

#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub perViewPositionAllComponents: VkBool32,
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: VkStructureType = 1000097000;
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
#[rustfmt::skip]
unsafe impl crate::VulkanSinkStructure for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure { unsafe { core::mem::transmute(self) } }
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
#[rustfmt::skip]
impl crate::TypedVulkanSinkStructure for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX; }

#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &str = "VK_NV_viewport_swizzle";
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION: usize = 1;

#[cfg(feature = "VK_viewport_swizzle_NV")]
#[rustfmt::skip]
pub type VkViewportCoordinateSwizzleviewport_swizzle = i32;
#[cfg(feature = "VK_viewport_swizzle_NV")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_viewport_swizzle: VkViewportCoordinateSwizzleviewport_swizzle = 0;
#[cfg(feature = "VK_viewport_swizzle_NV")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_viewport_swizzle: VkViewportCoordinateSwizzleviewport_swizzle = 1;
#[cfg(feature = "VK_viewport_swizzle_NV")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_viewport_swizzle: VkViewportCoordinateSwizzleviewport_swizzle = 2;
#[cfg(feature = "VK_viewport_swizzle_NV")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_viewport_swizzle: VkViewportCoordinateSwizzleviewport_swizzle = 3;
#[cfg(feature = "VK_viewport_swizzle_NV")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_viewport_swizzle: VkViewportCoordinateSwizzleviewport_swizzle = 4;
#[cfg(feature = "VK_viewport_swizzle_NV")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_viewport_swizzle: VkViewportCoordinateSwizzleviewport_swizzle = 5;
#[cfg(feature = "VK_viewport_swizzle_NV")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_viewport_swizzle: VkViewportCoordinateSwizzleviewport_swizzle = 6;
#[cfg(feature = "VK_viewport_swizzle_NV")]
#[rustfmt::skip]
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_viewport_swizzle: VkViewportCoordinateSwizzleviewport_swizzle = 7;

#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub type VkPipelineViewportSwizzleStateCreateFlagsNV = VkFlags;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub type VkPipelineViewportSwizzleStateCreateFlagBitsNV = VkFlags;

#[cfg(feature = "VK_NV_viewport_swizzle")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkViewportSwizzleNV {
    pub x: VkViewportCoordinateSwizzleNV,
    pub y: VkViewportCoordinateSwizzleNV,
    pub z: VkViewportCoordinateSwizzleNV,
    pub w: VkViewportCoordinateSwizzleNV,
}

#[cfg(feature = "VK_NV_viewport_swizzle")]
#[derive(Debug, Clone)]
#[repr(C)]
#[rustfmt::skip]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub viewportCount: u32,
    pub pViewportSwizzles: *const VkViewportSwizzleNV,
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: VkStructureType = 1000098000;
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
unsafe impl crate::VulkanStructure for VkPipelineViewportSwizzleStateCreateInfoNV {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[rustfmt::skip]
impl crate::TypedVulkanStructure for VkPipelineViewportSwizzleStateCreateInfoNV { const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV; }

#[cfg(all(feature = "Implements", not(feature = "DynamicLoaded")))]
#[cfg_attr(all(not(windows), not(target_os = "macos"), not(feature = "DynamicLoaded")), link(name = "vulkan"))]
#[cfg_attr(all(windows, not(feature = "DynamicLoaded"), feature = "Implements"), link(name = "vulkan-1"))]
#[rustfmt::skip]
unsafe extern "system" {
    pub fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;
    pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
    pub fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;
    pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
    pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
    pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;
    pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
    pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
    pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const core::ffi::c_char) -> Option<PFN_vkVoidFunction>;
    pub fn vkGetDeviceProcAddr(device: VkDevice, pName: *const core::ffi::c_char) -> Option<PFN_vkVoidFunction>;
    pub fn vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;
    pub fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
    pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const core::ffi::c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
    pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const core::ffi::c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
    pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
    pub fn vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
    pub fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
    pub fn vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: Option<VkFence>) -> VkResult;
    pub fn vkQueueWaitIdle(queue: VkQueue) -> VkResult;
    pub fn vkDeviceWaitIdle(device: VkDevice) -> VkResult;
    pub fn vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;
    pub fn vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
    pub fn vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut core::ffi::c_void) -> VkResult;
    pub fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory);
    pub fn vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
    pub fn vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
    pub fn vkGetDeviceMemoryCommitment(device: VkDevice, memory: VkDeviceMemory, pCommitmentMemoryInBytes: *mut VkDeviceSize);
    pub fn vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
    pub fn vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
    pub fn vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);
    pub fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
    pub fn vkGetImageSparseMemoryRequirements(device: VkDevice, image: VkImage, pSparseMemoryRequirementsCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
    pub fn vkQueueBindSparse(queue: VkQueue, bindInfoCount: u32, pBindInfos: *const VkBindSparseInfo, fence: Option<VkFence>) -> VkResult;
    pub fn vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
    pub fn vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
    pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult;
    pub fn vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;
    pub fn vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;
    pub fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateEvent(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;
    pub fn vkDestroyEvent(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkCreateQueryPool(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;
    pub fn vkDestroyQueryPool(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetQueryPoolResults(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut core::ffi::c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;
    pub fn vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;
    pub fn vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;
    pub fn vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;
    pub fn vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetImageSubresourceLayout(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);
    pub fn vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;
    pub fn vkDestroyImageView(device: VkDevice, view: VkImageView, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pModule: *mut VkShaderModule) -> VkResult;
    pub fn vkDestroyShaderModule(device: VkDevice, module: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;
    pub fn vkDestroyPipelineCache(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut core::ffi::c_void) -> VkResult;
    pub fn vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;
    pub fn vkCreateGraphicsPipelines(device: VkDevice, pipelineCache: Option<VkPipelineCache>, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
    pub fn vkCreateComputePipelines(device: VkDevice, pipelineCache: Option<VkPipelineCache>, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
    pub fn vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;
    pub fn vkDestroyPipelineLayout(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;
    pub fn vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateDescriptorSetLayout(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;
    pub fn vkDestroyDescriptorSetLayout(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pPool: *mut VkDescriptorPool) -> VkResult;
    pub fn vkDestroyDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;
    pub fn vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;
    pub fn vkFreeDescriptorSets(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;
    pub fn vkUpdateDescriptorSets(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
    pub fn vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;
    pub fn vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
    pub fn vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
    pub fn vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;
    pub fn vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;
    pub fn vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;
    pub fn vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;
    pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult;
    pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkEnumerateInstanceVersion(pApiVersion: *mut u32) -> VkResult;
    #[cfg(feature = "VK_KHR_surface")]
    pub fn vkDestroySurfaceKHR(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "VK_KHR_surface")]
    pub fn vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;
    #[cfg(feature = "VK_KHR_surface")]
    pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_surface")]
    pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatsCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_surface")]
    pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_swapchain")]
    pub fn vkCreateSwapchainKHR(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_swapchain")]
    pub fn vkDestroySwapchainKHR(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "VK_KHR_swapchain")]
    pub fn vkGetSwapchainImagesKHR(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult;
    #[cfg(feature = "VK_KHR_swapchain")]
    pub fn vkAcquireNextImageKHR(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: Option<VkSemaphore>, fence: Option<VkFence>, pImageIndex: *mut u32) -> VkResult;
    #[cfg(feature = "VK_KHR_swapchain")]
    pub fn vkQueuePresentKHR(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    pub fn vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    pub fn vkGetDisplayModePropertiesKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    pub fn vkCreateDisplayModeKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    pub fn vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    pub fn vkCreateDisplayPlaneSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_xlib_surface")]
    pub fn vkCreateXlibSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_xlib_surface")]
    pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut x11::xlib::Display, visualID: x11::xlib::VisualID) -> VkBool32;
    #[cfg(feature = "VK_KHR_xcb_surface")]
    pub fn vkCreateXcbSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_xcb_surface")]
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb::ffi::xcb_connection_t, visual_id: xcb::x::Visualid) -> VkBool32;
    #[cfg(feature = "VK_KHR_wayland_surface")]
    pub fn vkCreateWaylandSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_wayland_surface")]
    pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut core::ffi::c_void) -> VkBool32;
    #[cfg(feature = "VK_KHR_android_surface")]
    pub fn vkCreateAndroidSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_win32_surface")]
    pub fn vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_win32_surface")]
    pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;
    #[cfg(feature = "VK_EXT_metal_surface")]
    pub fn vkCreateMetalSurfaceEXT(instance: VkInstance, pCreateInfo: *const VkMetalSurfaceCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetPhysicalDeviceExternalBufferProperties(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetPhysicalDeviceExternalSemaphoreProperties(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHR, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetPhysicalDeviceExternalFenceProperties(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfoKHR, pExternalFenceProperties: *mut VkExternalFencePropertiesKHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetDeviceGroupPeerMemoryFeatures(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetPhysicalDeviceFeatures2(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetPhysicalDeviceProperties2(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetPhysicalDeviceFormatProperties2(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2KHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetPhysicalDeviceImageFormatProperties2(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut VkImageFormatProperties2KHR) -> VkResult;
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetPhysicalDeviceQueueFamilyProperties2(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetPhysicalDeviceMemoryProperties2(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties2(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2KHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkEnumeratePhysicalDeviceGroup(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHR) -> VkResult;
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetImageMemoryRequirements2(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetBufferMemoryRequirements2(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetImageSparseMemoryRequirements2(device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2KHR, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2KHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkBindBufferMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHR) -> VkResult;
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkBindImageMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHR) -> VkResult;
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkCreateDescriptorUpdateTemplate(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> VkResult;
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkDestroyDescriptorUpdateTemplate(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkUpdateDescriptorSetWithTemplate(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const core::ffi::c_void);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkCreateSamplerYcbcrConversion(device: VkDevice, pCreateInfo: VkSamplerYcbcrConversionCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSamplerYcbcrConversionKHR) -> VkResult;
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkDestroySamplerYcbcrConversion(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversionKHR, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkTrimCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkGetDescriptorSetLayoutSupport(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport);
    #[cfg(feature = "Allow1_3APIs")]
    pub fn vkQueueSubmit2(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo2KHR, fence: Option<VkFence>) -> VkResult;
    pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
    pub fn vkCmdSetViewport(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);
    pub fn vkCmdSetScissor(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);
    pub fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: core::ffi::c_float);
    pub fn vkCmdSetDepthBias(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: core::ffi::c_float, depthBiasClamp: core::ffi::c_float, depthBiasSlopeFactor: core::ffi::c_float);
    pub fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: *const core::ffi::c_float);
    pub fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: core::ffi::c_float, maxDepthBounds: core::ffi::c_float);
    pub fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
    pub fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
    pub fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
    pub fn vkCmdBindDescriptorSets(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);
    pub fn vkCmdBindIndexBuffer(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
    pub fn vkCmdBindVertexBuffers(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
    pub fn vkCmdDraw(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
    pub fn vkCmdDrawIndexed(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
    pub fn vkCmdDrawIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    pub fn vkCmdDrawIndexedIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    pub fn vkCmdDispatch(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    pub fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
    pub fn vkCmdCopyBuffer(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
    pub fn vkCmdCopyImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);
    pub fn vkCmdBlitImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filters: VkFilter);
    pub fn vkCmdCopyBufferToImage(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
    pub fn vkCmdCopyImageToBuffer(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);
    pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const core::ffi::c_void);
    pub fn vkCmdFillBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, data: u32);
    pub fn vkCmdClearColorImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    pub fn vkCmdClearDepthStencilImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    pub fn vkCmdClearAttachments(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);
    pub fn vkCmdResolveImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);
    pub fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    pub fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    pub fn vkCmdWaitEvents(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    pub fn vkCmdPipelineBarrier(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    pub fn vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);
    pub fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
    pub fn vkCmdResetQueryPool(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
    pub fn vkCmdWriteTimestamp(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlags, queryPool: VkQueryPool, query: u32);
    pub fn vkCmdCopyQueryPoolResults(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);
    pub fn vkCmdPushConstants(commandBuffer: VkCommandBuffer, pipelineLayout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const core::ffi::c_void);
    pub fn vkCmdBeginRenderPass(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
    pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
    pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer);
    pub fn vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkCmdSetDeviceMask(commandBuffer: VkCommandBuffer, deviceMask: u32);
    #[cfg(feature = "Allow1_1APIs")]
    pub fn vkCmdDispatchBase(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    #[cfg(feature = "Allow1_3APIs")]
    pub fn vkCmdSetEvent2(commandBuffer: VkCommandBuffer, event: VkEvent, pDependencyInfo: *const VkDependencyInfoKHR);
    #[cfg(feature = "Allow1_3APIs")]
    pub fn vkCmdResetEvent2(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2KHR);
    #[cfg(feature = "Allow1_3APIs")]
    pub fn vkCmdWaitEvents2(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, pDependencyInfos: *const VkDependencyInfoKHR);
    #[cfg(feature = "Allow1_3APIs")]
    pub fn vkCmdPipelineBarrier2(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfoKHR);
    #[cfg(feature = "Allow1_3APIs")]
    pub fn vkCmdWriteTimestamp2(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2KHR, queryPool: VkQueryPool, query: u32);
    #[cfg(feature = "Allow1_2APIs")]
    pub fn vkGetBufferDeviceAddress(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfoKHR) -> VkDeviceAddress;
    #[cfg(feature = "Allow1_2APIs")]
    pub fn vkGetBufferOpaqueCaptureAddress(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfoKHR) -> u64;
    #[cfg(feature = "Allow1_2APIs")]
    pub fn vkGetDeviceMemoryOpaqueCaptureAddress(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfoKHR) -> u64;
    #[cfg(feature = "Allow1_2APIs")]
    pub fn vkGetSemaphoreCounterValue(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;
    #[cfg(feature = "Allow1_2APIs")]
    pub fn vkSignalSemaphore(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfoKHR) -> VkResult;
    #[cfg(feature = "Allow1_2APIs")]
    pub fn vkWaitSemaphores(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfoKHR, timeout: u64) -> VkResult;
}
