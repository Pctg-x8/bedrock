pub const VK_KHR_16BIT_STORAGE_SPEC_VERISON: usize = 1;
pub const VK_KHR_16BIT_STORAGE_EXTENSION_NAME: &str = "VK_KHR_16bit_storage";

use derives::promote_1_1;

use super::*;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: VkStructureType = ext_enum_value(84, 0) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkPhysicalDevice16BitStorageFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    /// 16-bit integer/floating-point variables supported in BufferBlock
    pub storageBuffer16BitAccess: VkBool32,
    /// 16-bit integer/floating-point variables supported in BufferBlock and Bloc
    pub uniformAndStorageBuffer16BitAccess: VkBool32,
    /// 16-bit integer/floating-point variables supported in PushConstants
    pub storagePushConstant16: VkBool32,
    /// 16-bit integer/floating-point variables supported in shader inputs and outputs
    pub storageInputOutput16: VkBool32,
}
