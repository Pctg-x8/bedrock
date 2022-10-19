//! VK_KHX_device_group_creation extensions

pub const VK_KHX_DEVICE_GROUP_CREATION_SPEC_VERSION: usize = 1;
pub static VK_KHX_DEVICE_GROUP_CREATION_EXTENSION_NAME: &'static str = "VK_KHX_device_group_creation";

use super::*;

pub const VK_MAX_DEVICE_GROUP_SIZE_KHX: usize = VK_MAX_DEVICE_GROUP_SIZE;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHX"]
pub struct VkPhysicalDeviceGroupPropertiesKHX {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub physicalDeviceCount: u32,
    pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE_KHX],
    pub subsetAllocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHX"]
pub struct VkDeviceGroupDeviceCreateInfoKHX {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub physicalDeviceCount: u32,
    pub pPhysicalDevices: *const VkPhysicalDevice,
}

pub type PFN_vkEnumeratePhysicalDeviceGroupsKHX = extern "system" fn(
    instance: VkInstance,
    pPhysicalDeviceGroupCount: *mut u32,
    pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHX,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkEnumeratePhysicalDeviceGroupsKHX(
        instance: VkInstance,
        pPhysicalDeviceGroupCount: *mut u32,
        pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHX,
    ) -> VkResult;
}
