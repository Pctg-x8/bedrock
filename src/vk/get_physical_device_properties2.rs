//! VK_KHR_get_physical_device_properties2

pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &str = "VK_KHR_get_physical_device_properties2";

use super::*;

pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR: VkStructureType = ext_enum_value(60, 0) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 1) as _;
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 2) as _;
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 3) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType = ext_enum_value(60, 4) as _;
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 5) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 6) as _;
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 7) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType =
    ext_enum_value(60, 8) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR)]
pub struct VkFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub formatProperties: VkFormatProperties,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR)]
pub struct VkPhysicalDeviceFeatures2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub features: VkPhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR)]
pub struct VkPhysicalDeviceProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkPhysicalDeviceProperties,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR)]
pub struct VkImageFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageFormatProperties: VkImageFormatProperties,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR)]
pub struct VkPhysicalDeviceImageFormatInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub r#type: VkImageType,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub flags: VkImageCreateFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR)]
pub struct VkQueueFamilyProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub queueFamilyProperties: VkQueueFamilyProperties,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR)]
pub struct VkPhysicalDeviceMemoryProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR)]
pub struct VkSparseImageFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkSparseImageFormatProperties,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub r#type: VkImageType,
    pub samples: VkSampleCountFlagBits,
    pub usage: VkImageUsageFlags,
    pub tiling: VkImageTiling,
}

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkGetPhysicalDeviceFeatures2KHR"]
pub struct PFN_vkGetPhysicalDeviceFeatures2KHR(
    pub extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkGetPhysicalDeviceProperties2KHR"]
pub struct PFN_vkGetPhysicalDeviceProperties2KHR(
    pub extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkGetPhysicalDeviceFormatProperties2KHR"]
pub struct PFN_vkGetPhysicalDeviceFormatProperties2KHR(
    pub  extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        format: VkFormat,
        pFormatProperties: *mut VkFormatProperties2KHR,
    ),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkGetPhysicalDeviceImageFormatProperties2KHR"]
pub struct PFN_vkGetPhysicalDeviceImageFormatProperties2KHR(
    pub  extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR,
        pImageFormatProperties: *mut VkImageFormatProperties2KHR,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkGetPhysicalDeviceQueueFamilyProperties2KHR"]
pub struct PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR(
    pub  extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32,
        pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR,
    ),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkGetPhysicalDeviceMemoryProperties2KHR"]
pub struct PFN_vkGetPhysicalDeviceMemoryProperties2KHR(
    pub  extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR,
    ),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[org_name = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR"]
pub struct PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
    pub  extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR,
        pPropertyCount: *mut u32,
        pProperties: *mut VkSparseImageFormatProperties2KHR,
    ),
);

cfg_if! {
    if #[cfg(feature = "Allow1_1APIs")] {
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR;
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR;
        pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR;
        pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR;
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR;
        pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR;
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR;
        pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR;
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR;

        pub type VkPhysicalDeviceFeatures2 = VkPhysicalDeviceFeatures2KHR;
        pub type VkPhysicalDeviceProperties2 = VkPhysicalDeviceProperties2KHR;
        pub type VkFormatProperties2 = VkFormatProperties2KHR;
        pub type VkImageFormatProperties2 = VkImageFormatProperties2KHR;
        pub type VkPhysicalDeviceImageFormatInfo2 = VkPhysicalDeviceImageFormatInfo2KHR;
        pub type VkQueueFamilyProperties2 = VkQueueFamilyProperties2KHR;
        pub type VkPhysicalDeviceMemoryProperties2 = VkPhysicalDeviceMemoryProperties2KHR;
        pub type VkSparseImageFormatProperties2 = VkSparseImageFormatProperties2KHR;
        pub type VkPhysicalDeviceSparseImageFormatInfo2 = VkPhysicalDeviceSparseImageFormatInfo2KHR;

        pub type PFN_vkGetPhysicalDeviceFeatures2 = PFN_vkGetPhysicalDeviceFeatures2KHR;
        pub type PFN_vkGetPhysicalDeviceProperties2 = PFN_vkGetPhysicalDeviceProperties2KHR;
        pub type PFN_vkGetPhysicalDeviceFormatProperties2 = PFN_vkGetPhysicalDeviceFormatProperties2KHR;
        pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = PFN_vkGetPhysicalDeviceImageFormatProperties2KHR;
        pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR;
        pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = PFN_vkGetPhysicalDeviceMemoryProperties2KHR;
        pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR;
    }
}
