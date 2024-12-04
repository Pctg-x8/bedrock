//! VK_KHR_get_physical_device_properties2

pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &str = "VK_KHR_get_physical_device_properties2";

use derives::{promote_1_1, vk_ext_command};

use super::*;

#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR: VkStructureType = ext_enum_value(60, 0) as _;
#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 1) as _;
#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 2) as _;
#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 3) as _;
#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType = ext_enum_value(60, 4) as _;
#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 5) as _;
#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 6) as _;
#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType = ext_enum_value(60, 7) as _;
#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType =
    ext_enum_value(60, 8) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub formatProperties: VkFormatProperties,
}
impl VkFormatProperties2KHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(Self::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceFeatures2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub features: VkPhysicalDeviceFeatures,
}
impl VkPhysicalDeviceFeatures2KHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(<Self as VulkanSinkStructure>::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkPhysicalDeviceProperties,
}
impl VkPhysicalDeviceProperties2KHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(Self::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkImageFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageFormatProperties: VkImageFormatProperties,
}
impl VkImageFormatProperties2KHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(Self::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR)]
#[promote_1_1(suffix = "KHR")]
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
#[derive(Debug, Clone, PartialEq, Eq, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkQueueFamilyProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub queueFamilyProperties: VkQueueFamilyProperties,
}
impl VkQueueFamilyProperties2KHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(Self::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceMemoryProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}
impl VkPhysicalDeviceMemoryProperties2KHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(Self::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkSparseImageFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkSparseImageFormatProperties,
}
impl VkSparseImageFormatProperties2KHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(Self::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub r#type: VkImageType,
    pub samples: VkSampleCountFlagBits,
    pub usage: VkImageUsageFlags,
    pub tiling: VkImageTiling,
}

vk_ext_command!(
    pub fn vkGetPhysicalDeviceFeatures2KHR(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR);
    suffix = "KHR";
    promote = "1.1";
);

vk_ext_command!(
    pub fn vkGetPhysicalDeviceProperties2KHR(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR);
    suffix = "KHR";
    promote = "1.1";
);

vk_ext_command!(
    pub fn vkGetPhysicalDeviceFormatProperties2KHR(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2KHR);
    suffix = "KHR";
    promote = "1.1";
);

vk_ext_command!(
    pub fn vkGetPhysicalDeviceImageFormatProperties2KHR(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut VkImageFormatProperties2KHR) -> VkResult;
    suffix = "KHR";
    promote = "1.1";
);

vk_ext_command!(
    pub fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR);
    suffix = "KHR";
    promote = "1.1";
);

vk_ext_command!(
    pub fn vkGetPhysicalDeviceMemoryProperties2KHR(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR);
    suffix = "KHR";
    promote = "1.1";
);

vk_ext_command!(
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2KHR);
    suffix = "KHR";
    promote = "1.1";
);
