//! VK_NV_fill_rectangle extensions

pub const VK_NV_FILL_RECTANGLE_SPEC_VERSION: usize = 1;
pub static VK_NV_FILL_RECTANGLE_EXTENSION_NAME: &'static str = "VK_NV_fill_rectangle";

use super::*;

pub const VK_POLYGON_MODE_FILL_RECTANGLE_NV: VkPolygonMode = ext_enum_value(154, 0) as _;
