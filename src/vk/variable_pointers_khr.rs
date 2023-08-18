pub const VK_KHR_VARIABLE_POINTERS_SPEC_VERSION: usize = 1;
pub const VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME: &str = "VK_KHR_variable_pointers";

use super::*;
use derives::promote_1_1;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: VkStructureType =
    ext_enum_value(121, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: VkStructureType =
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceVariablePointersFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub variablePointersStorageBuffer: VkBool32,
    pub variablePointers: VkBool32,
}
impl VkPhysicalDeviceVariablePointersFeaturesKHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = &mut *p.as_mut_ptr();
            x.sType = Self::TYPE;
            x.pNext = core::ptr::null_mut();
        }

        p
    }
}

#[promote_1_1(suffix = "KHR")]
pub type VkPhysicalDeviceVariablePointerFeaturesKHR = VkPhysicalDeviceVariablePointersFeaturesKHR;
