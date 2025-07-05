//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_map_memory2.html

pub const VK_KHR_MAP_MEMORY_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_MAP_MEMORY_2_EXTENSION_NAME: &'static str = "VK_KHR_map_memory2";

use super::*;
use crate::vk2::*;
use derives::{promote_1_4, vk_ext_command};

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_MEMORY_MAP_INFO_KHR: VkStructureType = ext_enum_value(272, 0) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO_KHR: VkStructureType = ext_enum_value(272, 1) as _;

vk_bitmask! {
    #[promote_1_4(suffix = "KHR")]
    pub enum VkMemoryUnmapFlagBitsKHR {
        #[promote_1_4]
        pub VK_MEMORY_UNMAP_RESERVE_BIT_KHR: 0,
    }
}

#[promote_1_4(suffix = "KHR")]
pub type VkMemoryUnmapFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_MAP_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkMemoryMapInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkMemoryUnmapFlagsKHR,
    pub memory: VkDeviceMemory,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkMemoryUnmapInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkMemoryUnmapFlagsKHR,
    pub memory: VkDeviceMemory,
}

vk_ext_command! {
    pub fn vkMapMemory2KHR(device: VkDevice, pMemoryMapInfo: *const VkMemoryMapInfoKHR, ppData: *mut *mut core::ffi::c_void) -> VkResult;
    suffix = "KHR";
    promote = "1.4";
}

vk_ext_command! {
    pub fn vkUnmapMemory2KHR(device: VkDevice, pMemoryUnmapInfo: *const VkMemoryUnmapInfoKHR) -> VkResult;
    suffix = "KHR";
    promote = "1.4";
}
