pub const VK_KHR_MAINTENANCE_3_SPEC_VERSION: usize = 1;
pub const VK_KHR_MAINTENANCE_3_EXTENSION_NAME: &str = "VK_KHR_maintenance3";

use super::*;
use derives::promote_1_1;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: VkStructureType = ext_enum_value(169, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: VkStructureType = ext_enum_value(169, 1) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceMaintenance3PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxPerSetDescriptors: u32,
    pub maxMemoryAllocationSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkDescriptorSetLayoutSupportKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supported: VkBool32,
}

#[promote_1_1(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetDescriptorSetLayoutSupportKHR)]
pub struct PFN_vkGetDescriptorSetLayoutSupportKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
        pSupport: *mut VkDescriptorSetLayoutSupportKHR,
    ),
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    #[promote_1_1(suffix = "KHR")]
    pub fn vkGetDescriptorSetLayoutSupportKHR(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
        pSupport: *mut VkDescriptorSetLayoutSupportKHR,
    );
}
