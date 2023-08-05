pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME: &str = "VK_KHR_get_memory_requirements2";

use super::*;
use derives::promote_1_1;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = ext_enum_value(147, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = ext_enum_value(147, 1) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = ext_enum_value(147, 2) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR: VkStructureType = ext_enum_value(147, 3) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: VkStructureType = ext_enum_value(147, 4) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkBufferMemoryRequirementsInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkImageMemoryRequirementsInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkImageSparseMemoryRequirementsInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkMemoryRequirements2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryRequirements: VkMemoryRequirements,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkSparseImageMemoryRequirements2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryRequirements: VkSparseImageMemoryRequirements,
}

#[promote_1_1(suffix = "KHR")]
pub type PFN_vkGetImageMemoryRequirements2KHR = extern "system" fn(
    device: VkDevice,
    pInfo: *const VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
#[promote_1_1(suffix = "KHR")]
pub type PFN_vkGetBufferMemoryRequirements2KHR = extern "system" fn(
    device: VkDevice,
    pInfo: *const VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
#[promote_1_1(suffix = "KHR")]
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = extern "system" fn(
    device: VkDevice,
    pInfo: *const VkImageSparseMemoryRequirementsInfo2,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    #[promote_1_1(suffix = "KHR")]
    pub fn vkGetImageMemoryRequirements2KHR(
        device: VkDevice,
        pInfo: *const VkImageMemoryRequirementsInfo2,
        pMemoryRequirements: *mut VkMemoryRequirements2,
    );
    #[promote_1_1(suffix = "KHR")]
    pub fn vkGetBufferMemoryRequirements2KHR(
        device: VkDevice,
        pInfo: *const VkBufferMemoryRequirementsInfo2,
        pMemoryRequirements: *mut VkMemoryRequirements2,
    );
    #[promote_1_1(suffix = "KHR")]
    pub fn vkGetImageSparseMemoryRequirements2KHR(
        device: VkDevice,
        pInfo: *const VkImageSparseMemoryRequirementsInfo2,
        pSparseMemoryRequirementCount: *mut u32,
        pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
    );
}