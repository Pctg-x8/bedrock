pub const VK_KHR_EXTERNAL_FENCE_SPEC_VERSION: usize = 1;
pub const VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME: &str = "VK_KHR_external_fence";

use super::*;
use derives::promote_1_1;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(114, 0) as _;

#[promote_1_1(suffix = "KHR")]
pub type VkFenceImportFlagsKHR = VkFlags;
vk_bitmask! {
    #[promote_1_1(suffix = "KHR")]
    pub enum VkFenceImportFlagBitsKHR {
        #[promote_1_1]
        pub VK_FENCE_IMPORT_TEMPORARY_BIT_KHR: 0
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkExportFenceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalFenceHandleTypeFlagsKHR,
}
