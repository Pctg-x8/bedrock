//! VK_KHR_descriptor_update_template extensions

pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: usize = 1;
pub static VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &'static str = "VK_KHR_descriptor_update_template";

use super::*;

pub type VkDescriptorUpdateTemplateKHR = VkDescriptorUpdateTemplate;

pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = extern "system" fn(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: VkPipelineLayout, set: u32, pData: *const c_void);
