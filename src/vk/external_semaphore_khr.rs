pub const VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERISON: usize = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &str = "VK_KHR_external_semaphore";

use super::*;
use derives::promote_1_1;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(78, 0) as _;

#[promote_1_1(suffix = "KHR")]
pub type VkSemaphoreImportFlagsKHR = VkFlags;
vk_bitmask! {
    #[promote_1_1(suffix = "KHR")]
    pub enum VkSemaphoreImportFlagBitsKHR {
        #[promote_1_1]
        pub VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR: 0
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkExportSemaphoreCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
}
