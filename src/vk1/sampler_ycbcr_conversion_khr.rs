pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: usize = 14;
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: &str = "VK_KHR_sampler_ycbcr_conversion";

use super::*;

use crate::vk2::*;
use derives::{promote_1_1, vk_ext_command};

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: VkStructureType = ext_enum_value(157, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR: VkStructureType = ext_enum_value(157, 1) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR: VkStructureType = ext_enum_value(157, 2) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: VkStructureType = ext_enum_value(157, 3) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: VkStructureType =
    ext_enum_value(157, 4) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: VkStructureType =
    ext_enum_value(157, 5) as _;

#[promote_1_1]
pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR: VkObjectType = ext_enum_value(157, 0) as _;

#[promote_1_1]
pub const VK_FORMAT_G8B8G8R8_422_UNORM_KHR: VkFormat = ext_enum_value(157, 0) as _;
#[promote_1_1]
pub const VK_FORMAT_B8G8R8G8_422_UNORM_KHR: VkFormat = ext_enum_value(157, 1) as _;
#[promote_1_1]
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR: VkFormat = ext_enum_value(157, 2) as _;
#[promote_1_1]
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR: VkFormat = ext_enum_value(157, 3) as _;
#[promote_1_1]
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR: VkFormat = ext_enum_value(157, 4) as _;
#[promote_1_1]
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR: VkFormat = ext_enum_value(157, 5) as _;
#[promote_1_1]
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR: VkFormat = ext_enum_value(157, 6) as _;
#[promote_1_1]
pub const VK_FORMAT_R10X6_UNORM_PACK16_KHR: VkFormat = ext_enum_value(157, 7) as _;
#[promote_1_1]
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR: VkFormat = ext_enum_value(157, 8) as _;
#[promote_1_1]
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: VkFormat = ext_enum_value(157, 9) as _;
#[promote_1_1]
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: VkFormat = ext_enum_value(157, 10) as _;
#[promote_1_1]
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: VkFormat = ext_enum_value(157, 11) as _;
#[promote_1_1]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: VkFormat = ext_enum_value(157, 12) as _;
#[promote_1_1]
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_2PACK16_KHR: VkFormat = ext_enum_value(157, 13) as _;
#[promote_1_1]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: VkFormat = ext_enum_value(157, 14) as _;
#[promote_1_1]
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: VkFormat = ext_enum_value(157, 15) as _;
#[promote_1_1]
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: VkFormat = ext_enum_value(157, 16) as _;
#[promote_1_1]
pub const VK_FORMAT_R12X4_UNORM_PACK16_KHR: VkFormat = ext_enum_value(157, 17) as _;
#[promote_1_1]
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR: VkFormat = ext_enum_value(157, 18) as _;
#[promote_1_1]
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: VkFormat = ext_enum_value(157, 19) as _;
#[promote_1_1]
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: VkFormat = ext_enum_value(157, 20) as _;
#[promote_1_1]
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: VkFormat = ext_enum_value(157, 21) as _;
#[promote_1_1]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: VkFormat = ext_enum_value(157, 22) as _;
#[promote_1_1]
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: VkFormat = ext_enum_value(157, 23) as _;
#[promote_1_1]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: VkFormat = ext_enum_value(157, 24) as _;
#[promote_1_1]
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: VkFormat = ext_enum_value(157, 25) as _;
#[promote_1_1]
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: VkFormat = ext_enum_value(157, 26) as _;
#[promote_1_1]
pub const VK_FORMAT_G16B16G16R16_422_UNORM_KHR: VkFormat = ext_enum_value(157, 27) as _;
#[promote_1_1]
pub const VK_FORMAT_B16G16R16G16_422_UNORM_KHR: VkFormat = ext_enum_value(157, 28) as _;
#[promote_1_1]
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR: VkFormat = ext_enum_value(157, 29) as _;
#[promote_1_1]
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR: VkFormat = ext_enum_value(157, 30) as _;
#[promote_1_1]
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR: VkFormat = ext_enum_value(157, 31) as _;
#[promote_1_1]
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR: VkFormat = ext_enum_value(157, 32) as _;
#[promote_1_1]
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR: VkFormat = ext_enum_value(157, 33) as _;

#[repr(C)]
#[derive(Clone, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkSamplerYcbcrConversionCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub ycbcrModel: VkSamplerYcbcrModelConversionKHR,
    pub ycbcrRange: VkSamplerYcbcrRangeKHR,
    pub components: VkComponentMapping,
    pub xChromaOffset: VkChromaLocationKHR,
    pub yChromaOffset: VkChromaLocationKHR,
    pub chromaFilter: VkFilter,
    pub forceExplicitReconstruction: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkSamplerYcbcrConversionInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub conversion: VkSamplerYcbcrConversionKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkBindImagePlaneMemoryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub planeAspect: VkImageAspectFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkImagePlaneMemoryRequirementsInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub planeAspect: VkImageAspectFlagBits,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    /// Sampler color conversion supported
    pub samplerYcbcrConversion: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkSamplerYcbcrConversionImageFormatPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub combinedImageSamplerDescriptorCount: u32,
}

vk_ext_command! {
    pub fn vkCreateSamplerYcbcrConversionKHR(device: VkDevice, pCreateInfo: *const VkSamplerYcbcrConversionCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSamplerYcbcrConversionKHR) -> VkResult;
    suffix = "KHR";
    promote = "1.1";
}

vk_ext_command! {
    pub fn vkDestroySamplerYcbcrConversionKHR(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversionKHR, pAllocator: *const VkAllocationCallbacks);
    suffix = "KHR";
    promote = "1.1";
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkSamplerYcbcrConversionKHR(pub u64);

#[promote_1_1(suffix = "KHR")]
pub type VkSamplerYcbcrModelConversionKHR = i32;
#[promote_1_1]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR: VkSamplerYcbcrModelConversionKHR = 0;
#[promote_1_1]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR: VkSamplerYcbcrModelConversionKHR = 1;
#[promote_1_1]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR: VkSamplerYcbcrModelConversionKHR = 2;
#[promote_1_1]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR: VkSamplerYcbcrModelConversionKHR = 3;
#[promote_1_1]
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR: VkSamplerYcbcrModelConversionKHR = 4;

#[promote_1_1(suffix = "KHR")]
pub type VkSamplerYcbcrRangeKHR = i32;
#[promote_1_1]
pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR: VkSamplerYcbcrRangeKHR = 0;
#[promote_1_1]
pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR: VkSamplerYcbcrRangeKHR = 1;

#[promote_1_1(suffix = "KHR")]
pub type VkChromaLocationKHR = i32;
#[promote_1_1]
pub const VK_CHROMA_LOCATION_COSITED_EVEN_KHR: VkChromaLocationKHR = 0;
#[promote_1_1]
pub const VK_CHROMA_LOCATION_MIDPOINT_KHR: VkChromaLocationKHR = 1;

cfg_if::cfg_if! {
    if #[cfg(feature = "VK_EXT_debug_report")] {
        pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT: crate::vk2::VkDebugReportObjectTypeEXT = ext_enum_value(157, 0) as _;
        #[cfg(feature = "Allow1_1APIs")]
        pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT: crate::vk2::VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT;
    }
}
