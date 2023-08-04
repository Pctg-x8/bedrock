pub const VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION: usize = 1;
pub const VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &str = "VK_KHR_external_memory";

use derives::promote_1_1;

use super::*;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: VkStructureType = ext_enum_value(73, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(73, 1) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR: VkStructureType = ext_enum_value(73, 2) as _;

#[promote_1_1(suffix = "KHR")]
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR: VkResult = -(ext_enum_value(73, 3) as VkResult);

#[promote_1_1(suffix = "KHR")]
pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: u32 = unsafe { std::mem::transmute(-1) };

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkExternalMemoryImageCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkExternalMemoryBufferCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkExportMemoryAllocateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
