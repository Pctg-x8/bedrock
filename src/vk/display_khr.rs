//! VK_KHR_display extensions

pub const VK_KHR_DISPLAY_SPEC_VERSION: usize = 21;
pub const VK_KHR_DISPLAY_EXTENSION_NAME: &str = "VK_KHR_display";

use super::*;
use crate::PFN;
#[allow(unused_imports)]
use libc::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_DISPLAY_KHR)]
pub struct VkDisplayKHR(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_DISPLAY_MODE_KHR)]
pub struct VkDisplayModeKHR(pub u64);

pub const VK_OBJECT_TYPE_DISPLAY_KHR: VkObjectType = ext_enum_value(3, 0) as _;
pub const VK_OBJECT_TYPE_DISPLAY_MODE_KHR: VkObjectType = ext_enum_value(3, 1) as _;

pub type VkDisplayPlaneAlphaFlagsKHR = VkFlags;
pub const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = 0x01;
pub const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = 0x02;
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = 0x04;
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = 0x08;
pub type VkDisplayModeCreateFlagsKHR = VkFlags;
pub type VkDisplaySurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayPropertiesKHR {
    pub display: VkDisplayKHR,
    pub displayName: *const c_char,
    pub physicalDimensions: VkExtent2D,
    pub physicalResolution: VkExtent2D,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub planeReorderPossible: VkBool32,
    pub persistentContent: VkBool32,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayModeParametersKHR {
    pub visibleRegion: VkExtent2D,
    pub refreshRate: u32,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayModePropertiesKHR {
    pub displayMode: VkDisplayModeKHR,
    pub parameters: VkDisplayModeParametersKHR,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR)]
pub struct VkDisplayModeCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplayModeCreateFlagsKHR,
    pub parameters: VkDisplayModeParametersKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayPlaneCapabilitiesKHR {
    pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
    pub minSrcPosition: VkOffset2D,
    pub maxSrcPosition: VkOffset2D,
    pub minSrcExtent: VkExtent2D,
    pub maxSrcExtent: VkExtent2D,
    pub minDstPosition: VkOffset2D,
    pub maxDstPosition: VkOffset2D,
    pub minDstExtent: VkExtent2D,
    pub maxDstExtent: VkExtent2D,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayPlanePropertiesKHR {
    pub currentDisplay: VkDisplayKHR,
    pub currentStackIndex: u32,
}
#[repr(C)]
#[derive(Debug, Clone, PartialEq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR)]
pub struct VkDisplaySurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplaySurfaceCreateFlagsKHR,
    pub displayMode: VkDisplayModeKHR,
    pub planeIndex: u32,
    pub planeStackIndex: u32,
    pub transform: VkSurfaceTransformFlagsKHR,
    pub globalAlpha: c_float,
    pub alphaMode: VkDisplayPlaneAlphaFlagsKHR,
    pub imageExtent: VkExtent2D,
}

#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceDisplayPropertiesKHR)]
pub struct PFN_vkGetPhysicalDeviceDisplayPropertiesKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayPropertiesKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceDisplayPlanePropertiesKHR)]
pub struct PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pProertyCount: *mut u32,
        pProperties: *mut VkDisplayPlanePropertiesKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetDisplayPlaneSupportedDisplaysKHR)]
pub struct PFN_vkGetDisplayPlaneSupportedDisplaysKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        planeIndex: u32,
        pDisplayCount: *mut u32,
        pDisplays: *mut VkDisplayKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetDisplayModePropertiesKHR)]
pub struct PFN_vkGetDisplayModePropertiesKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        display: VkDisplayKHR,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayModePropertiesKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateDisplayModeKHR)]
pub struct PFN_vkCreateDisplayModeKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        display: VkDisplayKHR,
        pCreateInfo: *const VkDisplayModeCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pMode: *mut VkDisplayModeKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetDisplayPlaneCapabilitiesKHR)]
pub struct PFN_vkGetDisplayPlaneCapabilitiesKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        mode: VkDisplayModeKHR,
        planeIndex: u32,
        pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateDisplayPlaneSurfaceKHR)]
pub struct PFN_vkCreateDisplayPlaneSurfaceKHR(
    pub  unsafe extern "system" fn(
        instance: VkInstance,
        pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayPropertiesKHR,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayPlanePropertiesKHR,
    ) -> VkResult;
    pub fn vkGetDisplayPlaneSupportedDisplaysKHR(
        physicalDevice: VkPhysicalDevice,
        planeIndex: u32,
        pDisplayCount: *mut u32,
        pDisplays: *mut VkDisplayKHR,
    ) -> VkResult;
    pub fn vkGetDisplayModePropertiesKHR(
        physicalDevice: VkPhysicalDevice,
        display: VkDisplayKHR,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayModePropertiesKHR,
    ) -> VkResult;
    pub fn vkCreateDisplayModeKHR(
        physicalDevice: VkPhysicalDevice,
        display: VkDisplayKHR,
        pCreateInfo: *const VkDisplayModeCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pMode: *mut VkDisplayModeKHR,
    ) -> VkResult;
    pub fn vkGetDisplayPlaneCapabilitiesKHR(
        physicalDevice: VkPhysicalDevice,
        mode: VkDisplayModeKHR,
        planeIndex: u32,
        pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR,
    ) -> VkResult;
    pub fn vkCreateDisplayPlaneSurfaceKHR(
        instance: VkInstance,
        pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
}
