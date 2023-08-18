pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: usize = 1;
pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &str = "VK_KHR_external_fence_capabilities";

use super::*;
use crate::PFN;
use derives::promote_1_1;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: VkStructureType = ext_enum_value(113, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR: VkStructureType = ext_enum_value(113, 1) as _;

#[promote_1_1(suffix = "KHR")]
pub type VkExternalFenceHandleTypeFlagsKHR = VkFlags;
vk_bitmask! {
    #[promote_1_1(suffix = "KHR")]
    pub enum VkExternalFenceHandleTypeFlagBitsKHR {
        #[promote_1_1]
        pub VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: 0,
        #[promote_1_1]
        pub VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: 1,
        #[promote_1_1]
        pub VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: 2,
        #[promote_1_1]
        pub VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR: 3
    }
}

#[promote_1_1(suffix = "KHR")]
pub type VkExternalFenceFeatureFlagsKHR = VkFlags;
vk_bitmask! {
    #[promote_1_1(suffix = "KHR")]
    pub enum VkExternalFenceFeatureFlagBitsKHR {
        #[promote_1_1]
        pub VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR: 0,
        #[promote_1_1]
        pub VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR: 1
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceExternalFenceInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkExternalFencePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
    pub externalFenceFeatures: VkExternalFenceFeatureFlagsKHR,
}
impl VkExternalFencePropertiesKHR {
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

#[cfg(feature = "Implements")]
#[promote_1_1(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceExternalFencePropertiesKHR)]
pub struct PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfoKHR,
        pExternalFenceProperties: *mut VkExternalFencePropertiesKHR,
    ),
);
