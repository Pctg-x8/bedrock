//! VK_KHR_image_format_list extension

pub const VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: usize = 1;
pub const VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME: &str = "VK_KHR_iamge_format_list";

use super::*;
use libc::*;

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR"]
pub struct VkImageFormatListCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub viewFormatCount: u32,
    pub pViewFormats: *const VkFormat,
}
