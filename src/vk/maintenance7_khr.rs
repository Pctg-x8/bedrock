//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance7.html

pub const VK_KHR_MAINTENANCE_7_SPEC_VERSION: usize = 1;
pub const VK_KHR_MAINTENANCE_7_EXTENSION_NAME: &'static str = "VK_KHR_maintenance7";

use super::*;

pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR: VkStructureType = ext_enum_value(563, 0) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR: VkStructureType = ext_enum_value(563, 1) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR: VkStructureType =
    ext_enum_value(563, 2) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR: VkStructureType = ext_enum_value(563, 3) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR: VkStructureType =
    ext_enum_value(563, 4) as _;

pub type VkPhysicalDeviceLayeredApiKHR = i32;
pub const VK_PHYSICAL_DEVICE_LAYERED_API_VULKAN_KHR: VkPhysicalDeviceLayeredApiKHR = 0;
pub const VK_PHYSICAL_DEVICE_LAYERED_API_D3D12_KHR: VkPhysicalDeviceLayeredApiKHR = 1;
pub const VK_PHYSICAL_DEVICE_LAYERED_API_METAL_KHR: VkPhysicalDeviceLayeredApiKHR = 2;
pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGL_KHR: VkPhysicalDeviceLayeredApiKHR = 3;
pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGLES_KHR: VkPhysicalDeviceLayeredApiKHR = 4;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR)]
pub struct VkPhysicalDeviceMaintenance7FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maintenance7: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR)]
pub struct VkPhysicalDeviceMaintenance7PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub robustFragmentShadingRateAttachmentAccess: VkBool32,
    pub separateDepthStencilAttachmentAccess: VkBool32,
    pub maxDescriptorSetTotalUniformBuffersDynamic: u32,
    pub maxDescriptorSetTotalStorageBuffersDynamic: u32,
    pub maxDescriptorSetTotalBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindTotalUniformBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindTotalStorageBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindTotalBuffersDynamic: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR)]
pub struct VkPhysicalDeviceLayeredApiPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub vendorID: u32,
    pub deviceID: u32,
    pub layeredAPI: VkPhysicalDeviceLayeredApiKHR,
    pub deviceName: FixedCStrBuffer<VK_MAX_PHYSICAL_DEVICE_NAME_SIZE>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR)]
pub struct VkPhysicalDeviceLayeredApiPropertiesListKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub layeredApiCount: u32,
    pub pLayeredApis: *mut VkPhysicalDeviceLayeredApiPropertiesKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR)]
pub struct VkPhysicalDeviceLayeredApiVulkanPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub properties: VkPhysicalDeviceProperties2,
}
