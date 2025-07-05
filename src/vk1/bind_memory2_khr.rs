pub const VK_KHR_BIND_MEMORY_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_BIND_MEMORY_2_EXTENSION_NAME: &str = "VK_KHR_bind_memory2";

use super::*;
use crate::vk2::*;
use derives::{promote_1_1, vk_ext_command};

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR: crate::vk2::VkStructureType = ext_enum_value(158, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR: crate::vk2::VkStructureType = ext_enum_value(158, 1) as _;

vk_ext_command! {
    pub fn vkBindBufferMemory2KHR(device: crate::vk2::VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHR) -> crate::vk2::VkResult;
    suffix = "KHR";
    promote = "1.1";
}

vk_ext_command! {
    pub fn vkBindImageMemory2KHR(device: crate::vk2::VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHR) -> crate::vk2::VkResult;
    suffix = "KHR";
    promote = "1.1";
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkBindBufferMemoryInfoKHR {
    pub sType: crate::vk2::VkStructureType,
    pub pNext: *const c_void,
    pub buffer: crate::vk2::VkBuffer,
    pub memory: crate::vk2::VkDeviceMemory,
    pub memoryOffset: crate::vk2::VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkBindImageMemoryInfoKHR {
    pub sType: crate::vk2::VkStructureType,
    pub pNext: *const c_void,
    pub image: crate::vk2::VkImage,
    pub memory: crate::vk2::VkDeviceMemory,
    pub memoryOffset: crate::vk2::VkDeviceSize,
}
