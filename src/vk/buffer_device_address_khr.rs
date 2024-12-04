//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_buffer_device_address.html

pub const VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: usize = 1;
pub const VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &'static str = "VK_KHR_buffer_device_address";

use super::*;
use derives::{promote_1_2, vk_ext_command};

#[promote_1_2]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: VkStructureType =
    ext_enum_value(258, 0) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR: VkStructureType = ext_enum_value(245, 1) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: VkStructureType =
    ext_enum_value(258, 2) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: VkStructureType =
    ext_enum_value(258, 3) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: VkStructureType =
    ext_enum_value(258, 4) as _;

#[promote_1_2]
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: VkResult = VkResult::ext_err_value(258, 0);

vk_bitmask! {
    extending enum VkBufferCreateFlagBits {
        #[promote_1_2]
        pub VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: 4,
    }
}

vk_bitmask! {
    extending enum VkBufferUsageFlagBits {
        #[promote_1_2]
        pub VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR: 17,
    }
}

vk_bitmask! {
    extending enum VkMemoryAllocateFlagBits {
        #[promote_1_2]
        pub VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR: 1,
        #[promote_1_2]
        pub VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: 2,
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure, VulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR)]
#[promote_1_2(suffix = "KHR")]
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub bufferDeviceAddress: VkBool32,
    pub bufferDeviceAddressCaptureReplay: VkBool32,
    pub bufferDeviceAddressMultiDevice: VkBool32,
}
impl VkPhysicalDeviceBufferDeviceAddressFeaturesKHR {
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR)]
#[promote_1_2(suffix = "KHR")]
pub struct VkBufferDeviceAddressInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub buffer: VkBuffer,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR)]
#[promote_1_2(suffix = "KHR")]
pub struct VkBufferOpaqueCaptureAddressCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub opaqueCaptureAddress: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR)]
#[promote_1_2(suffix = "KHR")]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub opaqueCaptureAddress: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR)]
#[promote_1_2(suffix = "KHR")]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub memory: VkDeviceMemory,
}

vk_ext_command!(
    pub fn vkGetBufferDeviceAddressKHR(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfoKHR) -> VkDeviceAddress;
    suffix = "KHR";
    promote = "1.2";
);
vk_ext_command!(
    pub fn vkGetBufferOpaqueCaptureAddressKHR(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfoKHR) -> u64;
    suffix = "KHR";
    promote = "1.2";
);
vk_ext_command!(
    pub fn vkGetDeviceMemoryOpaqueCaptureAddressKHR(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfoKHR) -> u64;
    suffix = "KHR";
    promote = "1.2";
);
