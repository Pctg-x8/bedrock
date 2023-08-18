//! VK_KHR_swapchain extension

pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: usize = 68;
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_swapchain";

use super::*;
#[allow(unused_imports)]
use libc::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_SWAPCHAIN_KHR)]
pub struct VkSwapchainKHR(pub u64);

pub const VK_OBJECT_TYPE_SWAPCHAIN_KHR: VkObjectType = ext_enum_value(2, 0) as _;

pub type VkSwapchainCreateFlagsKHR = VkFlags;
vk_bitmask! {
    pub enum VkSwapchainCreateFlagBitsKHR {
        pub VK_SWAPCHAIN_CREATE_BIND_SFR_BIT_KHX: 0
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR)]
pub struct VkSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: VkSurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: VkFormat,
    pub imageColorSpace: VkColorSpaceKHR,
    pub imageExtent: VkExtent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: VkImageUsageFlags,
    pub imageSharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: VkSurfaceTransformFlagsKHR,
    pub compositeAlpha: VkCompositeAlphaFlagsKHR,
    pub presentMode: VkPresentModeKHR,
    pub clipped: VkBool32,
    pub oldSwapchain: VkSwapchainKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR)]
pub struct VkPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut VkResult,
}

#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateSwapchainKHR)]
pub struct PFN_vkCreateSwapchainKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkSwapchainCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSwapchain: *mut VkSwapchainKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroySwapchainKHR)]
pub struct PFN_vkDestroySwapchainKHR(
    pub unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks),
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetSwapchainImagesKHR)]
pub struct PFN_vkGetSwapchainImagesKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        pSwapchainImageCount: *mut u32,
        pSwapchainImages: *mut VkImage,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkAcquireNextImageKHR)]
pub struct PFN_vkAcquireNextImageKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence,
        pImageIndex: *mut u32,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkQueuePresentKHR)]
pub struct PFN_vkQueuePresentKHR(
    pub unsafe extern "system" fn(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCreateSwapchainKHR(
        device: VkDevice,
        pCreateInfo: *const VkSwapchainCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSwapchain: *mut VkSwapchainKHR,
    ) -> VkResult;
    pub fn vkDestroySwapchainKHR(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetSwapchainImagesKHR(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        pSwapchainImageCount: *mut u32,
        pSwapchainImages: *mut VkImage,
    ) -> VkResult;
    pub fn vkAcquireNextImageKHR(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence,
        pImageIndex: *mut u32,
    ) -> VkResult;
    pub fn vkQueuePresentKHR(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;
}
