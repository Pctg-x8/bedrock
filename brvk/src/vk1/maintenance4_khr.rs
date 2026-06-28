//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance4.html

pub const VK_KHR_MAINTENANCE_4_SPEC_VERSION: usize = 2;
pub const VK_KHR_MAINTENANCE_4_EXTENSION_NAME: &str = "VK_KHR_maintenance4";

use crate::*;
use derives::{TypedVulkanSinkStructure, TypedVulkanStructure, promote_1_3, vk_ext_command};

#[promote_1_3]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: VkStructureType = ext_enum_value(414, 0) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: VkStructureType = ext_enum_value(414, 1) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: VkStructureType = ext_enum_value(414, 2) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: VkStructureType = ext_enum_value(414, 3) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkPhysicalDeviceMaintenance4FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maintenance4: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkPhysicalDeviceMaintenance4PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxBufferSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkDeviceBufferMemoryRequirementsKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pCreateInfo: *const VkBufferCreateInfo,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkDeviceImageMemoryRequirementsKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pCreateInfo: *const VkImageCreateInfo,
    pub planeAspect: VkImageAspectFlagBits,
}

vk_ext_command! {
    pub fn vkGetDeviceBufferMemoryRequirementsKHR(device: VkDevice, pInfo: *const VkDeviceBufferMemoryRequirementsKHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR);
    suffix = "KHR";
    promote = "1.3";
}

vk_ext_command! {
    pub fn vkGetDeviceImageMemoryRequirementsKHR(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirementsKHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR);
    suffix = "KHR";
    promote = "1.3";
}

vk_ext_command! {
    pub fn vkGetDeviceImageSparseMemoryRequirementsKHR(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirementsKHR, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2KHR);
    suffix = "KHR";
    promote = "1.3";
}
