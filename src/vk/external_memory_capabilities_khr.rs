pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: usize = 1;
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &str = "VK_KHR_external_memory_capabilities";

use derives::promote_1_1;

use super::*;
use crate::PFN;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: VkStructureType =
    ext_enum_value(72, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: VkStructureType = ext_enum_value(72, 1) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: VkStructureType = ext_enum_value(72, 2) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR: VkStructureType = ext_enum_value(72, 3) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR: VkStructureType = ext_enum_value(72, 4) as _;

#[promote_1_1]
pub const VK_LUID_SIZE_KHR: usize = 8;

#[promote_1_1(suffix = "KHR")]
pub type VkExternalMemoryHandleTypeFlagsKHR = VkFlags;
vk_bitmask! {
    #[promote_1_1(suffix = "KHR")]
    pub enum VkExternalMemoryHandleTypeFlagBitsKHR {
        #[promote_1_1]
        pub VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: 0,
        #[promote_1_1]
        pub VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: 1,
        #[promote_1_1]
        pub VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: 2,
        #[promote_1_1]
        pub VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR: 3,
        #[promote_1_1]
        pub VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR: 4,
        #[promote_1_1]
        pub VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR: 5,
        #[promote_1_1]
        pub VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR: 6
    }
}

#[promote_1_1(suffix = "KHR")]
pub type VkExternalMemoryFeatureFlagsKHR = VkFlags;
vk_bitmask! {
    #[promote_1_1(suffix = "KHR")]
    pub enum VkExternalMemoryFeatureFlagBitsKHR {
        #[promote_1_1]
        pub VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR: 0,
        #[promote_1_1]
        pub VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR: 1,
        #[promote_1_1]
        pub VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR: 2
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[promote_1_1(suffix = "KHR")]
pub struct VkExternalMemoryPropertiesKHR {
    pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsKHR,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkExternalImageFormatPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceExternalBufferInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub usage: VkBufferUsageFlags,
    pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkExternalBufferPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceIDPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE],
    pub driverUUID: [u8; VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE_KHR],
    pub deviceNodeMask: u32,
    pub deviceLUIDValid: VkBool32,
}

#[promote_1_1(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceExternalBufferPropertiesKHR)]
pub struct PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR,
        pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR,
    ),
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    #[promote_1_1(suffix = "KHR")]
    pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHR(
        physicalDevice: VkPhysicalDevice,
        pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR,
        pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR,
    );
}
