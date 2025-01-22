//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_vertex_attribute_divisor.html

pub const VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: usize = 1;
pub const VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &str = "VK_KHR_vertex_attribute_divisor";

use super::*;
use derives::promote_1_4;

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR: VkStructureType =
    ext_enum_value(526, 0) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR: VkStructureType =
    ext_enum_value(191, 1) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR: VkStructureType =
    ext_enum_value(191, 2) as _;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxVertexAttribDivisor: u32,
    pub supportsNonZeroFirstInstance: VkBool32,
}
impl VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR {
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[promote_1_4(suffix = "KHR")]
pub struct VkVertexInputBindingDivisorDescriptionKHR {
    pub binding: u32,
    pub divisor: u32,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPipelineVertexInputDivisorStateCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub vertexBindingDivisorCount: u32,
    pub pVertexBindingDivisors: *const VkVertexInputBindingDivisorDescriptionKHR,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, VulkanStructure, VulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub vertexAttributeInstanceRateDivisor: VkBool32,
    pub vertexAttributeInstanceRateZeroDivisor: VkBool32,
}
