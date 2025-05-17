//! VK_KHR_swapchain extension

pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: usize = 68;
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_swapchain";

use derives::vk_ext_command;

use super::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_SWAPCHAIN_KHR)]
pub struct VkSwapchainKHR(pub u64);
#[implements]
impl crate::VkDeviceChildNonExtDestroyable for VkSwapchainKHR {
    unsafe fn destroy(self, device: crate::vk::VkDevice, allocator: *const crate::vk::VkAllocationCallbacks) {
        unsafe { crate::vkfn::destroy_swapchain_khr(device, self, allocator) }
    }
}

pub const VK_OBJECT_TYPE_SWAPCHAIN_KHR: VkObjectType = ext_enum_value(2, 0) as _;

pub const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = ext_enum_value(2, 0) as _;
pub const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR: VkStructureType = ext_enum_value(2, 1) as _;

pub const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR: VkImageLayout = ext_enum_value(2, 2) as _;

pub const VK_SUBOPTIMAL_KHR: VkResult = VkResult::ext_value(2, 3);
pub const VK_ERROR_OUT_OF_DATE_KHR: VkResult = VkResult::ext_err_value(2, 4);

pub type VkSwapchainCreateFlagsKHR = VkFlags;
vk_bitmask! {
    pub enum VkSwapchainCreateFlagBitsKHR {
        pub VK_SWAPCHAIN_CREATE_BIND_SFR_BIT_KHX: 0
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
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
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
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

vk_ext_command! {
    pub fn vkCreateSwapchainKHR(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult;
    static_callable;
}

vk_ext_command! {
    pub fn vkDestroySwapchainKHR(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);
    static_callable;
}

vk_ext_command! {
    pub fn vkGetSwapchainImagesKHR(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult;
    static_callable;
}

vk_ext_command! {
    pub fn vkAcquireNextImageKHR(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32) -> VkResult;
    static_callable;
}

vk_ext_command! {
    pub fn vkQueuePresentKHR(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;
    static_callable;
}
