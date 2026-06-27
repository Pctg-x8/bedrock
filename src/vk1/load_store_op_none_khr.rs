//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_load_store_op_none.html

pub const VK_KHR_LOAD_STORE_OP_NONE_SPEC_VERSION: usize = 1;
pub const VK_KHR_LOAD_STORE_OP_NONE_EXTENSION_NAME: &str = "VK_KHR_load_store_op_none";

use super::*;
use crate::vk2::*;
use derives::promote_1_4;

#[promote_1_4]
pub const VK_ATTACHMENT_LOAD_OP_NONE_KHR: VkAttachmentLoadOp = ext_enum_value(401, 0) as _;

#[promote_1_4]
#[cfg(not(feature = "VK_KHR_dynamic_rendering"))]
pub const VK_ATTACHMENT_STORE_OP_NONE_KHR: VkAttachmentStoreOp = ext_enum_value(302, 0) as _;
