//! VK_KHR_image_format_list extension

pub const VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: usize = 1;
pub const VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME: &str = "VK_KHR_iamge_format_list";

use super::*; use libc::*;
use std::mem::zeroed;

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkImageFormatListCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub viewFormatCount: u32, pub pViewFormats: *const VkFormat
}
impl Default for VkImageFormatListCreateInfoKHR
{
    fn default() -> Self
    {
        VkImageFormatListCreateInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR,
            .. unsafe { zeroed() }
        }
    }
}
