//! VK_KHR_descriptor_update_template extensions

pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: usize = 1;
pub static VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &str = "VK_KHR_descriptor_update_template";

use derives::promote_1_1;

use super::*;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: VkStructureType = ext_enum_value(86, 0) as _;
#[promote_1_1]
pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR: VkObjectType = ext_enum_value(86, 0) as _;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkDescriptorUpdateTemplateKHR(pub u64);

#[promote_1_1(suffix = "KHR")]
pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkFlags;
vk_bitmask! {
    #[promote_1_1(suffix = "KHR")]
    pub enum VkDescriptorUpdateTemplateCreateFlagBitsKHR {}
}

#[promote_1_1(suffix = "KHR")]
pub type VkDescriptorUpdateTemplateTypeKHR = i32;
/// Create descriptor update template for descriptor set updates
#[promote_1_1]
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR: VkDescriptorUpdateTemplateTypeKHR = 0;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[promote_1_1(suffix = "KHR")]
pub struct VkDescriptorUpdateTemplateEntryKHR {
    /// Binding within the destination descriptor set to write
    pub dstBinding: u32,
    /// Array element within the destination binding to write
    pub dstArrayElement: u32,
    /// Number of descriptors to write
    pub descriptorCount: u32,
    /// Descriptor type to write
    pub descriptorType: VkDescriptorType,
    /// Offset into pData where the descriptors to update are stored
    pub offset: usize,
    /// Stride between two descriptors in pData when writing more than one descriptor
    pub stride: usize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorUpdateTemplateCreateFlagsKHR,
    /// Number of descriptor update entries to use for the update template
    pub descriptorUpdateEntryCount: u32,
    /// Descriptor update entries for the template
    pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntryKHR,
    pub templateType: VkDescriptorUpdateTemplateTypeKHR,
    pub descriptorSetLayout: VkDescriptorSetLayout,
    pub pipelineBindPoint: VkPipelineBindPoint,
    /// If used for push descriptors, this is the only allowed layout
    pub pipelineLayout: VkPipelineLayout,
    pub set: u32,
}

#[promote_1_1(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateDescriptorUpdateTemplateKHR)]
pub struct PFN_vkCreateDescriptorUpdateTemplateKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR,
    ) -> VkResult,
);
#[promote_1_1(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyDescriptorUpdateTemplateKHR)]
pub struct PFN_vkDestroyDescriptorUpdateTemplateKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
        pAllocator: *const VkAllocationCallbacks,
    ),
);
#[promote_1_1(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkUpdateDescriptorSetWithTemplateKHR)]
pub struct PFN_vkUpdateDescriptorSetWithTemplateKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        descriptorSet: VkDescriptorSet,
        descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
        pData: *const c_void,
    ),
);

cfg_if! {
    if #[cfg(feature = "VK_EXT_debug_report")] {
        pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT: VkDebugReportObjectTypeEXT = ext_enum_value(86, 0) as _;
        #[cfg(feature = "Allow1_1APIs")]
        pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT: VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT;
    }
}
