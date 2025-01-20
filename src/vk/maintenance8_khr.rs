//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance8.html

pub const VK_KHR_MAINTENANCE_8_SPEC_VERSION: usize = 1;
pub const VK_KHR_MAINTENANCE_8_EXTENSION_NAME: &'static str = "VK_KHR_maintenance8";

use super::*;

pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR: VkStructureType = ext_enum_value(575, 0) as _;
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_ACCESS_FLAGS_3_KHR: VkStructureType = ext_enum_value(575, 2) as _;

pub type VkAccessFlags3KHR = VkFlags64;
pub type VkAccessFlagBits3KHR = VkFlags64;
pub const VK_ACCESS_3_NONE_KHR: VkAccessFlagBits3KHR = 0;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure, VulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR)]
pub struct VkPhysicalDeviceMaintenance8FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maintenance8: VkBool32,
}
impl VkPhysicalDeviceMaintenance8FeaturesKHR {
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
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_BARRIER_ACCESS_FLAGS_3_KHR)]
pub struct VkMemoryBarrierAccessFlags3KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcAccessMask3: VkAccessFlags3KHR,
    pub dstAccessMask3: VkAccessFlags3KHR,
}
