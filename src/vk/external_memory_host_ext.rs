//! VK_EXT_external_memory_host extension

pub const VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: usize = 1;
pub const VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: &str = "VK_EXT_external_memory_host";

use super::*;
use std::mem::zeroed;

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkImportMemoryHostPointerInfoEXT
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlags, pub pHostPointer: *mut c_void
}
impl Default for VkImportMemoryHostPointerInfoEXT
{
    fn default() -> Self
    {
        VkImportMemoryHostPointerInfoEXT
        {
            sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkMemoryHostPointerPropertiesEXT
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub memoryTypeBits: u32
}
impl Default for VkMemoryHostPointerPropertiesEXT
{
    fn default() -> Self
    {
        VkMemoryHostPointerPropertiesEXT
        {
            sType: VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT,
            .. unsafe { zeroed() }
        }
    }
}

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub minImportedHostPointerAlignment: VkDeviceSize
}
impl Default for VkPhysicalDeviceExternalMemoryHostPropertiesEXT
{
    fn default() -> Self
    {
        VkPhysicalDeviceExternalMemoryHostPropertiesEXT
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
            .. unsafe { zeroed() }
        }
    }
}

pub type PFN_vkGetMemoryHostPointerPropertiesEXT = extern "system" fn(device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlags, pHostPointer: *const c_void,
    pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT) -> VkResult;

extern "system"
{
    pub fn vkGetMemoryHostPointerPropertiesEXT(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlags,
        pHostPointer: *const c_void, pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT) -> VkResult;
}
