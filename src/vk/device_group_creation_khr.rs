//! VK_KHX_device_group_creation extensions

pub const VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: usize = 1;
pub static VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: &'static str = "VK_KHR_device_group_creation";

use super::*;

pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: VkStructureType = ext_enum_value(71, 0) as _;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(71, 1) as _;

pub const VK_MAX_DEVICE_GROUP_SIZE_KHR: usize = 32;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR)]
pub struct VkPhysicalDeviceGroupPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub physicalDeviceCount: u32,
    pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE_KHR],
    pub subsetAllocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR)]
pub struct VkDeviceGroupDeviceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub physicalDeviceCount: u32,
    pub pPhysicalDevices: *const VkPhysicalDevice,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkEnumeratePhysicalDeviceGroupsKHR)]
pub struct PFN_vkEnumeratePhysicalDeviceGroupsKHR(
    pub  unsafe extern "system" fn(
        instance: VkInstance,
        pPhysicalDeviceGroupCount: *mut u32,
        pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHR,
    ) -> VkResult,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkEnumeratePhysicalDeviceGroupsKHR(
        instance: VkInstance,
        pPhysicalDeviceGroupCount: *mut u32,
        pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHR,
    ) -> VkResult;
}

cfg_if! {
    if #[cfg(feature = "Allow1_1APIs")] {
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR;
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR;

        pub const VK_MAX_DEVICE_GROUP_SIZE: usize = VK_MAX_DEVICE_GROUP_SIZE_KHR;

        pub type VkPhysicalDeviceGroupProperties = VkPhysicalDeviceGroupPropertiesKHR;
        pub type VkDeviceGroupDeviceCreateInfo = VkDeviceGroupDeviceCreateInfoKHR;
        pub type PFN_vkEnumeratePhysicalDeviceGroups = PFN_vkEnumeratePhysicalDeviceGroupsKHR;

        #[cfg(feature = "Implements")]
        #[cfg(not(feature = "DynamicLoaded"))]
        extern "system" {
            pub fn vkEnumeratePhysicalDeviceGroups(
                instance: VkInstance,
                pPhysicalDeviceGroupCount: *mut u32,
                pPhysicalDeviceGropuProperties: *mut VkPhysicalDeviceGroupProperties
            ) -> VkResult;
        }
    }
}
