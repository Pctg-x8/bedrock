//! VK_KHR_maintenance2 extension

pub const VK_KHR_MAINTENANCE2_SPEC_VERSION: usize = 1;
pub const VK_KHR_MAINTENANCE2_EXTENSION_NAME: &str = "VK_KHR_maintenance2";

use super::*; use libc::*;
use std::mem::zeroed;

pub type VkPointClippingBehaviorKHR = i32;
pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR: VkPointClippingBehaviorKHR = 0;
pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR: VkPointClippingBehaviorKHR = 1;

pub type VkTessellationDomainOriginKHR = i32;
pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR: VkTessellationDomainOriginKHR = 0;
pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR: VkTessellationDomainOriginKHR = 1;

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkPhysicalDevicePointClippingPropertiesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub pointClippingBehavior: VkPointClippingBehaviorKHR
}
impl Default for VkPhysicalDevicePointClippingPropertiesKHR
{
    fn default() -> Self
    {
        VkPhysicalDevicePointClippingPropertiesKHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug, PartialEq, Eq)]
pub struct VkInputAttachmentAspectReferenceKHR
{
    pub subpass: u32, pub inputAttachmentIndex: u32, pub aspectMask: VkImageAspectFlags
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkRenderPassInputAttachmentAspectInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub aspectReferenceCount: u32, pub pAspectReferneces: *const VkInputAttachmentAspectReferenceKHR
}
impl Default for VkRenderPassInputAttachmentAspectInfoKHR
{
    fn default() -> Self
    {
        VkRenderPassInputAttachmentAspectInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_INFO_KHR,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkImageViewUsageCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void, pub usage: VkImageUsageFlags
}
impl Default for VkImageViewUsageCreateInfoKHR
{
    fn default() -> Self
    {
        VkImageViewUsageCreateInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub domainOrigin: VkTessellationDomainOriginKHR
}
impl Default for VkPipelineTessellationDomainOriginStateCreateInfoKHR
{
    fn default() -> Self
    {
        VkPipelineTessellationDomainOriginStateCreateInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR,
            .. unsafe { zeroed() }
        }
    }
}
