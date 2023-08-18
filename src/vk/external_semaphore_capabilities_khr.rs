pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: usize = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &str = "VK_KHR_external_semaphore_capabilities";

use derives::promote_1_1;

use super::*;
use crate::PFN;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: VkStructureType = ext_enum_value(77, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR: VkStructureType = ext_enum_value(77, 1) as _;

#[promote_1_1(suffix = "KHR")]
pub type VkExternalSemaphoreHandleTypeFlagsKHR = VkFlags;
vk_bitmask! {
    #[promote_1_1(suffix = "KHR")]
    pub enum VkExternalSemaphoreHandleTypeFlagBitsKHR {
        #[promote_1_1]
        pub VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: 0,
        #[promote_1_1]
        pub VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: 1,
        #[promote_1_1]
        pub VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: 2,
        #[promote_1_1]
        pub VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR: 3,
        #[promote_1_1]
        pub VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR: 4
    }
}

// promoted special alias
#[cfg(feature = "Allow1_1APIs")]
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBits =
    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT;

#[promote_1_1(suffix = "KHR")]
pub type VkExternalSemaphoreFeatureFlagsKHR = VkFlags;
vk_bitmask! {
    #[promote_1_1(suffix = "KHR")]
    pub enum VkExternalSemaphoreFeatureFlagBitsKHR {
        #[promote_1_1]
        pub VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR: 0,
        #[promote_1_1]
        pub VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR: 1
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkExternalSemaphorePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagsKHR,
}

#[cfg(feature = "Implements")]
#[promote_1_1(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceExternalSemaphorePropertiesKHR)]
pub struct PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHR,
        pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHR,
    ),
);
