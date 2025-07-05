#![allow(clippy::inconsistent_digit_grouping)]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, dead_code)]

//! Vulkan API Definitions **VER** with some extensions

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
