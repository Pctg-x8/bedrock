//! VK_EXT_validation_cache extension

pub const VK_EXT_VALIDATION_CACHE_SPEC_VERSION: usize = 1;
pub const VK_EXT_VALIDATION_CACHE_EXTENSION_NAME: &str = "VK_EXT_validation_cache";
pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkDebugReportObjectTypeEXT
    = VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT;

use libc::*; use super::*;
use std::mem::zeroed;

#[cfg(target_pointer_width = "64")] mod nd_handle_base_ts
{
    pub enum VkValidationCacheEXT {}
}
pub type VkValidationCacheEXT = VK_NON_DISPATCHABLE_HANDLE!(VkValidationCacheEXT);

pub type VkValidationCacheHeaderVersionEXT = i32;
pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: VkValidationCacheHeaderVersionEXT = 1;

pub type VkValidationCacheCreateFlagsEXT = VkFlags;

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkValidationCacheCreateInfoEXT
{
    pub sType: VkStructureType, pub pNext: *const c_void, pub flags: VkValidationCacheCreateFlagsEXT,
    pub initialDataSize: size_t, pub pInitialData: *const c_void
}
impl Default for VkValidationCacheCreateInfoEXT
{
    fn default() -> Self
    {
        VkValidationCacheCreateInfoEXT
        {
            sType: VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkShaderModuleValidationCacheCreateInfoEXT
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub validationCache: VkValidationCacheEXT
}

pub type PFN_vkCreateValidationCacheEXT = extern "system" fn(device: VkDevice,
    pCreateInfo: *const VkValidationCacheCreateInfoEXT, pAllocator: *const VkAllocationCallbacks,
    pValidationCache: *mut VkValidationCacheEXT) -> VkResult;
pub type PFN_vkDestroyValidationCacheEXT = extern "system" fn(device: VkDevice,
    validationCache: VkValidationCacheEXT, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkMergeValidationCachesEXT = extern "system" fn(device: VkDevice,
    dstCache: VkValidationCacheEXT, srcCacheCount: u32, pSrcCaches: *const VkValidationCacheEXT) -> VkResult;
pub type PFN_vkGetValidationCacheDataEXT = extern "system" fn(device: VkDevice,
    validationCache: VkValidationCacheEXT, pDataSize: *mut size_t, pData: *mut c_void) -> VkResult;

extern "system"
{
    pub fn vkCreateValidationCacheEXT(device: VkDevice, pCreateInfo: *const VkValidationCacheCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks, pValidationCache: *mut VkValidationCacheEXT) -> VkResult;
    pub fn vkDestroyValidationCacheEXT(device: VkDevice, validationCache: VkValidationCacheEXT,
        pAllocator: *const VkAllocationCallbacks);
    pub fn vkMergeValidationCachesEXT(device: VkDevice, dstCache: VkValidationCacheEXT,
        srcCacheCount: u32, pSrcCaches: *const VkValidationCacheEXT) -> VkResult;
    pub fn vkGetValidationCacheDataEXT(device: VkDevice, validationCache: VkValidationCacheEXT,
        pDataSize: *mut size_t, pData: *mut c_void) -> VkResult;
}
