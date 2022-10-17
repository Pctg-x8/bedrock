//! VK_NV_external_memory extensions

pub const VK_NV_EXTERNAL_MEMORY_SPEC_VERSION: usize = 1;
pub static VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME: &'static str = "VK_NV_external_memory";

use super::*;
use libc::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV"]
pub struct VkExternalMemoryImageCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV"]
pub struct VkExportMemoryAllocateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}
