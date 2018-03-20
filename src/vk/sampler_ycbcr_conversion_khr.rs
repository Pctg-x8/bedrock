//! VK_KHR_sampler_ycbcr_conversion extension

pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: usize = 1;
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: &str = "VK_KHR_sampler_ycbcr_conversion";

use super::*; use libc::*;
use std::mem::zeroed;

pub type VkSamplerYcbcrModelConversionKHR = i32;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR: VkSamplerYcbcrModelConversionKHR = 0;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR: VkSamplerYcbcrModelConversionKHR = 1;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR: VkSamplerYcbcrModelConversionKHR = 2;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR: VkSamplerYcbcrModelConversionKHR = 3;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR: VkSamplerYcbcrModelConversionKHR = 4;

pub type VkSamplerYcbcrRangeKHR = i32;
pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR: VkSamplerYcbcrRangeKHR = 0;
pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR: VkSamplerYcbcrRangeKHR = 1;

pub type VkChromaLocationKHR = i32;
pub const VK_CHROMA_LOCATION_COSITED_EVEN_KHR: VkChromaLocationKHR = 0;
pub const VK_CHROMA_LOCATION_MIDPOINT_KHR: VkChromaLocationKHR = 1;

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkSamplerYcbcrConversionCreateInfoKHR
{
    pub sType VkStructureType, pub pNext: *const c_void,
    pub format: VkFormat, pub ycbcrModel: VkSamplerYcbcrModelConversionKHR,
    pub ycbcrRange: VkSamplerYcbcrRange, pub components: VkComponentMapping,
    pub xChromaOffset: VkChromaLocationKHR, pub yChromaOffset: VkChromaLocationKHR,
    pub chromaFilter: VkFilter, pub forceExplicitReconstruction: VkBool32
}
impl Default for VkSamplerYcbcrConversionCreateInfoKHR
{
    fn default() -> Self
    {
        VkSamplerYcbcrConversionCreateInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkSamplerYcbcrConversionInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub conversion: VkSamplerYcbcrConversionKHR
}
impl Default for VkSamplerYcbcrConversionInfoKHR
{
    fn default() -> Self
    {
        VkSamplerYcbcrConversionInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkBindImagePlaneMemoryInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub planeAspect: VkImageAspectFlags
}
impl Default for VkBindImagePlaneMemoryInfoKHR
{
    fn default() -> Self
    {
        VkBindImagePlaneMemoryInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkImagePlaneMemoryRequirementsInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub planeAspect: VkImageAspectFlags
}
impl Default for VkImagePlaneMemoryRequirementsInfoKHR
{
    fn default() -> Self
    {
        VkImagePlaneMemoryRequirementsInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub samplerYcbcrConversion: VkBool32
}
impl Default for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR
{
    fn default() -> Self
    {
        VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkSamplerYcbcrConversionImageFormatPropertiesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub combinedImageSamplerDescriptorCount: u32
}
impl VkSamplerYcbcrConversionImageFormatPropertiesKHR
{
    fn default() -> Self
    {
        VkSamplerYcbcrConversionImageFormatPropertiesKHR
        {
            sType: VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR,
            .. unsafe { zeroed() }
        }
    }
}

pub type PFN_vkCreateSamplerYcbcrConversionKHR = extern "system" fn(device: VkDevice,
    pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo, pAllocator: *const VkAllocationCallbacks,
    pYcbcrConversion: *mut VkSamplerYcbcrConversionKHR) -> VkResult;
pub type PFN_vkDestroySamplerYcbcrConversionKHR = extern "syste" fn(device: VkDevice,
    ycbcrConversion: VkSamplerYcbcrConversion, pAllocator: *const VkAllocationCallbacks);

extern "system"
{
    pub fn vkCreateSamplerYcbcrConversionKHR(device: VkDevice, pCreateInfo: *const VkSamplerYcbcrConversionCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSAmplerYcbcrConversionKHR) -> VkResult;
    pub fn vkDestroySamplerYcbcrConversionKHR(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversionKHR,
        pAllocator: *const VkAllocationCallbacks);
}
