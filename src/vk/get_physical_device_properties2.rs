//! VK_KHR_get_physical_device_properties2

pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &str = "VK_KHR_get_physical_device_properties2";

use super::*;

pub type VkPhysicalDeviceFeatures2KHR = VkPhysicalDeviceFeatures2;
pub type VkPhysicalDeviceProperties2KHR = VkPhysicalDeviceProperties2;
pub type VkFormatProperties2KHR = VkFormatProperties2;
pub type VkImageFormatProperties2KHR = VkImageFormatProperties2;
pub type VkPhysicalDeviceImageFormatInfo2KHR = VkPhysicalDeviceImageFormatInfo2;
pub type VkQueueFamilyProperties2KHR = VkQueueFamilyProperties2;
pub type VkPhysicalDeviceMemoryProperites2KHR = VkPhysicalDeviceMemoryProperties2;
pub type VkSparseImageFormatProperites2KHR = VkSparseImageFormatProperties2;
pub type VkPhysicalDeviceSparseImageFormatInfo2KHR = VkPhysicalDeviceSparseImageFormatInfo2;

pub type PFN_vkGetPhysicalDeviceFeatures2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);
pub type PFN_vkGetPhysicalDeviceProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2, pImageFormatProperties: *mut VkImageFormatProperties2) -> VkResult;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);
pub type PFN_vkGetPhysicalDeviceMemoryProeprties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2);

#[cfg(feature = "Implements")]
extern "system"
{
    fn vkGetPhysicalDeviceFeatures2KHR(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);
    fn vkGetPhysicalDeviceProperties2KHR(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);
    fn vkGetPhysicalDeviceFormatProperties2KHR(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);
    fn vkGetPhysicalDeviceImageFormatProperties2KHR(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2, pImageFormatProperties: *mut VkImageFormatProperties2) -> VkResult;
    fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);
    fn vkGetPhysicalDeviceMemoryProperties2KHR(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);
    fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2K, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2);
}
