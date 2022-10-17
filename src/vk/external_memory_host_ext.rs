//! VK_EXT_external_memory_host extension

pub const VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: usize = 1;
pub const VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: &str = "VK_EXT_external_memory_host";

use super::*;

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT"]
pub struct VkImportMemoryHostPointerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlags,
    pub pHostPointer: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT"]
pub struct VkMemoryHostPointerPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT"]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub minImportedHostPointerAlignment: VkDeviceSize,
}

pub type PFN_vkGetMemoryHostPointerPropertiesEXT = extern "system" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlags,
    pHostPointer: *const c_void,
    pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetMemoryHostPointerPropertiesEXT(
        device: VkDevice,
        handleType: VkExternalMemoryHandleTypeFlags,
        pHostPointer: *const c_void,
        pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT,
    ) -> VkResult;
}
