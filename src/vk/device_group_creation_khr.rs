//! VK_KHX_device_group_creation extensions

pub const VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: usize = 1;
pub static VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: &'static str = "VK_KHR_device_group_creation";

use derives::{promote_1_1, vk_ext_command};

use super::*;

#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: VkStructureType = ext_enum_value(71, 0) as _;
#[promote_1_1(suffix = "KHR")]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(71, 1) as _;

pub const VK_MAX_DEVICE_GROUP_SIZE_KHR: usize = 32;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceGroupPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub physicalDeviceCount: u32,
    pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE_KHR],
    pub subsetAllocation: VkBool32,
}
impl VkPhysicalDeviceGroupPropertiesKHR {
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
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkDeviceGroupDeviceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub physicalDeviceCount: u32,
    pub pPhysicalDevices: *const VkPhysicalDevice,
}

vk_ext_command!(
    pub fn vkEnumeratePhysicalDeviceGroupsKHR(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHR) -> VkResult;
    suffix = "KHR";
    promote = "1.1";
);
