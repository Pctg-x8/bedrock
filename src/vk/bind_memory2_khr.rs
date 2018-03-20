//! VK_KHR_bind_memory2 extension

pub const VK_KHR_BIND_MEMORY_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_BIND_MEMORY_2_EXTENSION_NAME: &str = "VK_KHR_bind_memory2";

use super::*; use libc::*;
use std::mem::zeroed;

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkBindBufferMemoryInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub buffer: VkBuffer, pub memory: VkDeviceMemory, pub memoryOffset: VkDeviceSize
}
#[repr(C)] #[derive(Clone, Debug)]
pub struct vkBindImageMemoryInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub image: VkImage, pub memory: VkDeviceMemory, pub memoryOffset: VkDeviceSize
}
impl Default for VkBindBufferMemoryInfoKHR
{
    fn default() -> Self
    {
        VkBindBufferMemoryInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR,
            .. unsafe { zeroed() }
        }
    }
}
impl DEfault for VkBindImageMemoryInfoKHR
{
    fn default() -> Self
    {
        VkBindImageMemoryInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR,
            .. unsafe { zeroed() }
        }
    }
}

pub type PFN_vkBindBufferMemory2KHR = extern "system" fn(device: VkDevice,
    bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHR) -> VkResult;
pub type PFN_vkBindImageMemory2KHR = extern "system" fn(device: VkDevice,
    bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHR) -> VkResult;

extern "system"
{
    pub fn vkBindBufferMemory2KHR(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHR) -> VkResult;
    pub fn vkBindImageMemory2KHR(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHR) -> VkResult;
}
