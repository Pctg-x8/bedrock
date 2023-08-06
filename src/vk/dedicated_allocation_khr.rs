pub const VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION: usize = 3;
pub const VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME: &str = "VK_KHR_dedicated_allocation";

use super::*;
use derives::promote_1_1;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR: VkStructureType = ext_enum_value(128, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR: VkStructureType = ext_enum_value(128, 1) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkMemoryDedicatedRequirementsKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub prefersDedicatedAllocation: VkBool32,
    pub requiresDedicatedAllocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkMemoryDedicatedAllocateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    /// Image that this allocation will be bound to
    pub image: VkImage,
    /// Buffer that this allocation will be bound to
    pub buffer: VkBuffer,
}
