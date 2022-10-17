//! VK_EXT_validation_flags extensions

pub const VK_EXT_VALIDATION_FLAGS_SPEC_VERSION: usize = 1;
pub static VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME: &'static str = "VK_EXT_validation_flags";

use super::*;
use libc::*;

pub type VkValidationCheckEXT = i32;
pub const VK_VALIDATION_CHECK_ALL_EXT: VkValidationCheckEXT = 0;
pub const VK_VALIDATION_CHECK_SHADERS_EXT: VkValidationCheckEXT = 1;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT"]
pub struct VkValidationFlagsEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub disabledValidationCheckCount: u32,
    pub pDisabledValidationChecks: *mut VkValidationCheckEXT,
}
