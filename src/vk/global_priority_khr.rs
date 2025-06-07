//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_global_priority.html

pub const VK_KHR_GLOBAL_PRIORITY_SPEC_VERISON: usize = 1;
pub const VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME: &'static str = "VK_KHR_global_priority";

use super::*;
use derives::promote_1_4;

#[promote_1_4]
pub const VK_MAX_GLOBAL_PRIORITY_SIZE_KHR: usize = 16;

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: VkStructureType = ext_enum_value(175, 0) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: VkStructureType =
    ext_enum_value(389, 0) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: VkStructureType = ext_enum_value(389, 1) as _;

#[promote_1_4]
pub const VK_ERROR_NOT_PERMITTED_KHR: VkResult = VkResult::ext_err_value(175, 1);

#[promote_1_4(suffix = "KHR")]
pub type VkQueueGlobalPriorityKHR = i32;
#[promote_1_4]
pub const VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR: VkQueueGlobalPriorityKHR = 128;
#[promote_1_4]
pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR: VkQueueGlobalPriorityKHR = 256;
#[promote_1_4]
pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR: VkQueueGlobalPriorityKHR = 512;
#[promote_1_4]
pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR: VkQueueGlobalPriorityKHR = 1024;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub globalPriority: VkQueueGlobalPriorityKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure, TypedVulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub globalPriorityQuery: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkQueueFamilyGlobalPriorityPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub priorityCount: u32,
    pub priorities: [VkQueueGlobalPriorityKHR; VK_MAX_GLOBAL_PRIORITY_SIZE_KHR],
}
