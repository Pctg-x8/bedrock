//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_index_type_uint8.html

pub const VK_KHR_INDEX_TYPE_UINT8_SPEC_VERSION: usize = 1;
pub const VK_KHR_INDEX_TYPE_UINT8_EXTENSION_NAME: &'static str = "VK_KHR_index_type_uint8";

use super::*;
use derives::promote_1_4;

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR: VkStructureType =
    ext_enum_value(266, 0) as _;

#[promote_1_4]
pub const VK_INDEX_TYPE_UINT8_KHR: VkIndexType = ext_enum_value(266, 0) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure, VulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub indexTypeUint8: VkBool32,
}
