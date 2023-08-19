//! VK_NV_dedicated_allocation extensions

pub const VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION: usize = 1;
pub static VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static str = "VK_NV_dedicated_allocation";

use super::*;

pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: VkStructureType = ext_enum_value(27, 0) as _;
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: VkStructureType = ext_enum_value(27, 1) as _;
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: VkStructureType = ext_enum_value(27, 2) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV)]
pub struct VkDedicatedAllocationImageCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dedicatedAllocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV)]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dedicatedAllocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub buffer: VkBuffer,
}
