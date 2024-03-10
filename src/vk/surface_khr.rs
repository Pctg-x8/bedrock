//! VK_KHR_surface extension

pub const VK_KHR_SURFACE_SPEC_VERSION: usize = 25;
pub const VK_KHR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_surface";

use super::*;
use crate::PFN;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_SURFACE_KHR)]
pub struct VkSurfaceKHR(pub u64);

pub const VK_OBJECT_TYPE_SURFACE_KHR: VkObjectType = ext_enum_value(1, 0) as _;

pub const VK_ERROR_SURFACE_LOST_KHR: VkResult = VkResult::ext_err_value(1, 0);
pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult = VkResult::ext_err_value(1, 1);

pub type VkColorSpaceKHR = i32;
pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = 0;
pub const VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT: VkColorSpaceKHR = 100_0104_001;
pub const VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT: VkColorSpaceKHR = 100_0104_002;
pub const VK_COLOR_SPACE_DCI_P3_LINEAR_EXT: VkColorSpaceKHR = 100_0104_003;
pub const VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT: VkColorSpaceKHR = 100_0104_004;
pub const VK_COLOR_SPACE_BT709_LINEAR_EXT: VkColorSpaceKHR = 100_0104_005;
pub const VK_COLOR_SPACE_BT709_NONLINEAR_EXT: VkColorSpaceKHR = 100_0104_006;
pub const VK_COLOR_SPACE_BT2020_LINEAR_EXT: VkColorSpaceKHR = 100_0104_007;
pub const VK_COLOR_SPACE_HDR10_ST2084_EXT: VkColorSpaceKHR = 100_0104_008;
pub const VK_COLOR_SPACE_DOLBYVISION_EXT: VkColorSpaceKHR = 100_0104_009;
pub const VK_COLOR_SPACE_KDR10_HLG_EXT: VkColorSpaceKHR = 100_0104_010;
pub const VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT: VkColorSpaceKHR = 100_0104_011;
pub const VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT: VkColorSpaceKHR = 100_0104_012;
pub const VK_COLOR_SPACE_PASS_THROUGH_EXT: VkColorSpaceKHR = 100_0104_013;

pub type VkPresentModeKHR = i32;
pub const VK_PRESENT_MODE_IMMEDIATE_KHR: VkPresentModeKHR = 0;
pub const VK_PRESENT_MODE_MAILBOX_KHR: VkPresentModeKHR = 1;
pub const VK_PRESENT_MODE_FIFO_KHR: VkPresentModeKHR = 2;
pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: VkPresentModeKHR = 3;
pub const VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR: VkPresentModeKHR = 100_0111_000;
pub const VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR: VkPresentModeKHR = 100_0111_001;

pub type VkSurfaceTransformFlagsKHR = VkFlags;
vk_bitmask! {
    pub enum VkSurfaceTransformFlagBitsKHR {
        pub VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: 0,
        pub VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: 1,
        pub VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: 2,
        pub VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: 3,
        pub VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: 4,
        pub VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: 5,
        pub VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: 6,
        pub VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: 7,
        pub VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: 8
    }
}

pub type VkCompositeAlphaFlagsKHR = VkFlags;
vk_bitmask! {
    pub enum VkCompositeAlphaFlagBitsKHR {
        pub VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: 0,
        pub VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: 1,
        pub VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: 2,
        pub VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: 3
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroySurfaceKHR)]
pub struct PFN_vkDestroySurfaceKHR(
    pub unsafe extern "system" fn(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceSurfaceSupportKHR)]
pub struct PFN_vkGetPhysicalDeviceSurfaceSupportKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        surface: VkSurfaceKHR,
        pSupported: *mut VkBool32,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceSurfaceCapabilitiesKHR)]
pub struct PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceSurfaceFormatsKHR)]
pub struct PFN_vkGetPhysicalDeviceSurfaceFormatsKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceFormatCount: *mut u32,
        pSurfaceFormats: *mut VkSurfaceFormatKHR,
    ) -> VkResult,
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceSurfacePresentModesKHR)]
pub struct PFN_vkGetPhysicalDeviceSurfacePresentModesKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkDestroySurfaceKHR(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetPhysicalDeviceSurfaceSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        surface: VkSurfaceKHR,
        pSupported: *mut VkBool32,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceFormatCount: *mut u32,
        pSurfaceFormats: *mut VkSurfaceFormatKHR,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR,
    ) -> VkResult;
}
