//! VK_EXT_validation_cache extension

pub const VK_EXT_VALIDATION_CACHE_SPEC_VERSION: usize = 1;
pub const VK_EXT_VALIDATION_CACHE_EXTENSION_NAME: &str = "VK_EXT_validation_cache";
pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkDebugReportObjectTypeEXT =
    VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT;

use super::*;
use crate::PFN;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_VALIDATION_CACHE_EXT)]
pub struct VkValidationCacheEXT(pub u64);

pub const VK_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkObjectType = ext_enum_value(161, 0) as _;

pub type VkValidationCacheHeaderVersionEXT = i32;
pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: VkValidationCacheHeaderVersionEXT = 1;

pub type VkValidationCacheCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT)]
pub struct VkValidationCacheCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkValidationCacheCreateFlagsEXT,
    pub initialDataSize: size_t,
    pub pInitialData: *const c_void,
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT)]
pub struct VkShaderModuleValidationCacheCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub validationCache: VkValidationCacheEXT,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateValidationCacheEXT)]
pub struct PFN_vkCreateValidationCacheEXT(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkValidationCacheCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
        pValidationCache: *mut VkValidationCacheEXT,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyValidationCacheEXT)]
pub struct PFN_vkDestroyValidationCacheEXT(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        validationCache: VkValidationCacheEXT,
        pAllocator: *const VkAllocationCallbacks,
    ),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkMergeValidationCachesEXT)]
pub struct PFN_vkMergeValidationCachesEXT(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        dstCache: VkValidationCacheEXT,
        srcCacheCount: u32,
        pSrcCaches: *const VkValidationCacheEXT,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetValidationCacheDataEXT)]
pub struct PFN_vkGetValidationCacheDataEXT(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        validationCache: VkValidationCacheEXT,
        pDataSize: *mut size_t,
        pData: *mut c_void,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCreateValidationCacheEXT(
        device: VkDevice,
        pCreateInfo: *const VkValidationCacheCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
        pValidationCache: *mut VkValidationCacheEXT,
    ) -> VkResult;
    pub fn vkDestroyValidationCacheEXT(
        device: VkDevice,
        validationCache: VkValidationCacheEXT,
        pAllocator: *const VkAllocationCallbacks,
    );
    pub fn vkMergeValidationCachesEXT(
        device: VkDevice,
        dstCache: VkValidationCacheEXT,
        srcCacheCount: u32,
        pSrcCaches: *const VkValidationCacheEXT,
    ) -> VkResult;
    pub fn vkGetValidationCacheDataEXT(
        device: VkDevice,
        validationCache: VkValidationCacheEXT,
        pDataSize: *mut size_t,
        pData: *mut c_void,
    ) -> VkResult;
}
