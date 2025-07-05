//! VK_NVX_multiview_per_view_attributes extensions

pub const VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: usize = 1;
pub static VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &'static str = "VK_NVX_multiview_per_view_attributes";

use super::*;
use crate::vk2::*;

vk_bitmask! {
    extending enum VkSubpassDescriptionFlagBits {
        pub VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX: 0,
        pub VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX: 1
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub perViewPositionAllComponents: VkBool32,
}
