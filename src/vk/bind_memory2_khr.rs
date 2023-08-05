pub const VK_KHR_BIND_MEMORY_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_BIND_MEMORY_2_EXTENSION_NAME: &str = "VK_KHR_bind_memory2";

use super::*;
use derives::promote_1_1;

#[promote_1_1(suffix = "KHR")]
pub type PFN_vkBindBufferMemory2KHR =
    extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHR) -> VkResult;
#[promote_1_1(suffix = "KHR")]
pub type PFN_vkBindImageMemory2KHR =
    extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHR) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    #[promote_1_1(suffix = "KHR")]
    pub fn vkBindBufferMemory2KHR(
        device: VkDevice,
        bindInfoCount: u32,
        pBindInfos: *const VkBindBufferMemoryInfoKHR,
    ) -> VkResult;
    #[promote_1_1(suffix = "KHR")]
    pub fn vkBindImageMemory2KHR(
        device: VkDevice,
        bindInfoCount: u32,
        pBindInfos: *const VkBindImageMemoryInfoKHR,
    ) -> VkResult;
}

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR: VkStructureType = ext_enum_value(158, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR: VkStructureType = ext_enum_value(158, 1) as _;

vk_bitmask! {
    extending enum VkImageCreateFlagBits {
        #[promote_1_1]
        pub VK_IMAGE_CREATE_ALIAS_BIT_KHR: 10
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkBindBufferMemoryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkBindImageMemoryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}
