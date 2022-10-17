//! VK_NV_fragment_coverage_to_color extensions

pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: usize = 1;
pub static VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &'static str = "VK_NV_fragment_coverage_to_color";

use super::*;
use libc::*;

pub type VkPipelineCoverageToColorStateCreateFlagsNV = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV"]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
    pub coverageToColorEnable: VkBool32,
    pub coverageToColorLocation: u32,
}
