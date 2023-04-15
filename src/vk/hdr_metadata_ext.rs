//! VK_EXT_hdr_metadata extensions

pub const VK_EXT_HDR_METADATA_SPEC_VERSION: usize = 1;
pub static VK_EXT_HDR_METADATA_EXTENSION_NAME: &'static str = "VK_EXT_hdr_metadata";

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkXYColorEXT {
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_HDR_METADATA_EXT)]
pub struct VkHdrMetadataEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub displayPrimaryRed: VkXYColorEXT,
    pub displayPrimaryGreen: VkXYColorEXT,
    pub displayPrimaryBlur: VkXYColorEXT,
    pub whitePoint: VkXYColorEXT,
    pub maxLuminance: c_float,
    pub minLuminance: c_float,
    pub maxContentLightLevel: c_float,
    pub maxFrameAverageLightLevel: c_float,
}

pub type PFN_vkSetHdrMetadataEXT = extern "system" fn(
    device: VkDevice,
    swapchainCount: u32,
    pSwapchains: *const VkSwapchainKHR,
    pMetadata: *const VkHdrMetadataEXT,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkSetHdrMetadataEXT(
        device: VkDevice,
        swapchainCount: u32,
        pSwapchains: *const VkSwapchainKHR,
        pMetadata: *const VkHdrMetadataEXT,
    );
}
