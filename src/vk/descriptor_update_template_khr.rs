//! VK_KHR_descriptor_update_template extensions

pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: usize = 1;
pub static VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &str = "VK_KHR_descriptor_update_template";

use super::*;

pub type VkDescriptorUpdateTemplateKHR = VkDescriptorUpdateTemplate;

pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = extern "system" fn(
    commandBuffer: VkCommandBuffer,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
    layout: VkPipelineLayout,
    set: u32,
    pData: *const c_void,
);

cfg_if! {
    if #[cfg(feature = "VK_EXT_debug_report")] {
        pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT: VkDebugReportObjectTypeEXT = ext_enum_value(86, 0);
        #[cfg(feature = "Allow1_1APIs")]
        pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT: VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT;
    }
}
