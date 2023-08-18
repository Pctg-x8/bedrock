//! VK_EXT_display_control extensions

pub const VK_EXT_DISPLAY_CONTROL_SPEC_VERSION: usize = 1;
pub static VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME: &'static str = "VK_EXT_display_control";

use super::*;
use crate::PFN;

pub type VkDisplayPowerStateEXT = i32;
pub const VK_DISPLAY_POWER_STATE_OFF_EXT: VkDisplayPowerStateEXT = 0;
pub const VK_DISPLAY_POWER_STATE_SUSPEND_EXT: VkDisplayPowerStateEXT = 1;
pub const VK_DISPLAY_POWER_STATE_ON_EXT: VkDisplayPowerStateEXT = 2;

pub type VkDeviceEventTypeEXT = i32;
pub const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: VkDeviceEventTypeEXT = 0;

pub type VkDisplayEventTypeEXT = i32;
pub const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: VkDisplayEventTypeEXT = 0;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT)]
pub struct VkDisplayPowerInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub powerState: VkDisplayPowerStateEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT)]
pub struct VkDeviceEventInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceEvent: VkDeviceEventTypeEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT)]
pub struct VkDisplayEventInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub displayEvent: VkDisplayEventTypeEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT)]
pub struct VkSwapchainCounterCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub surfaceCounters: VkSurfaceCounterFlagsEXT,
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDisplayPowerControlEXT)]
pub struct PFN_vkDisplayPowerControlEXT(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        display: VkDisplayKHR,
        pDisplayPowerInfo: *const VkDisplayPowerInfoEXT,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkRegisterDeviceEventEXT)]
pub struct PFN_vkRegisterDeviceEventEXT(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pDeviceEventInfo: *const VkDeviceEventInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
        pFence: *mut VkFence,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkRegisterDisplayEventEXT)]
pub struct PFN_vkRegisterDisplayEventEXT(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        display: VkDisplayKHR,
        pDisplayEventInfo: *const VkDisplayEventInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
        pFence: *mut VkFence,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetSwapchainCounterEXT)]
pub struct PFN_vkGetSwapchainCounterEXT(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        counter: VkSurfaceCounterFlagsEXT,
        pCounterValue: *mut u64,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkDisplayPowerControlEXT(
        device: VkDevice,
        display: VkDisplayKHR,
        pDisplayPowerInfo: *const VkDisplayPowerInfoEXT,
    ) -> VkResult;
    pub fn vkRegisterDeviceEventEXT(
        device: VkDevice,
        pDeviceEventInfo: *const VkDeviceEventInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
        pFence: *mut VkFence,
    ) -> VkResult;
    pub fn vkRegisterDisplayEventEXT(
        device: VkDevice,
        display: VkDisplayKHR,
        pDisplayEventInfo: *const VkDisplayEventInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
        pFence: *mut VkFence,
    ) -> VkResult;
    pub fn vkGetSwapchainCounterEXT(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        counter: VkSurfaceCounterFlagsEXT,
        pCounterValue: *mut u64,
    ) -> VkResult;
}
