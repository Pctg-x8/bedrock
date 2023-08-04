pub const VK_KHR_MAINTENANCE_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_MAINTENANCE_2_EXTENSION_NAME: &str = "VK_KHR_maintenance2";

use super::*;
use derives::promote_1_1;

vk_bitmask! {
    extending enum VkImageCreateFlagBits {
        #[promote_1_1]
        pub VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: 7,
        #[promote_1_1]
        pub VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR: 8
    }
}

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: VkStructureType =
    ext_enum_value(118, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: VkStructureType =
    ext_enum_value(118, 1) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(118, 2) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: VkStructureType =
    ext_enum_value(118, 3) as _;

#[promote_1_1]
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = ext_enum_value(118, 0) as _;
#[promote_1_1]
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: VkImageLayout = ext_enum_value(118, 1) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDevicePointClippingPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pointClippingBehavior: VkPointClippingBehaviorKHR,
}

#[promote_1_1(suffix = "KHR")]
pub type VkPointClippingBehaviorKHR = i32;
#[promote_1_1]
pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR: VkPointClippingBehaviorKHR = 0;
#[promote_1_1]
pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_KHR: VkPointClippingBehaviorKHR = 1;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkRenderPassInputAttachmentAspectCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub aspectReferenceCount: u32,
    pub pAspectReferences: *const VkInputAttachmentAspectReferenceKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[promote_1_1(suffix = "KHR")]
pub struct VkInputAttachmentAspectReferenceKHR {
    pub subpass: u32,
    pub inputAttachmentIndex: u32,
    pub aspectMask: VkImageAspectFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkImageViewUsageCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub sliceOffset: u32,
    pub sliceCount: u32,
}

#[promote_1_1(suffix = "KHR")]
pub type VkTessellationDomainOriginKHR = i32;
#[promote_1_1]
pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR: VkTessellationDomainOriginKHR = 0;
#[promote_1_1]
pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR: VkTessellationDomainOriginKHR = 1;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub domainOrigin: VkTessellationDomainOriginKHR,
}
