//! VK_KHR_push_descriptor extensions

pub const VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION: usize = 1;
pub static VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &str = "VK_KHR_push_descriptor";

use super::*;
use crate::vk2::*;
use derives::{promote_1_4, vk_ext_command};

vk_bitmask! {
    extending enum VkDescriptorSetLayoutCreateFlagBits {
        #[promote_1_4]
        pub VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR: 0
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxPushDescriptors: u32,
}

vk_ext_command! {
    pub fn vkCmdPushDescriptorSetKHR(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet);
    suffix = "KHR";
    promote = "1.4";
}

cfg_if::cfg_if! {
    if #[cfg(feature = "VK_KHR_descriptor_update_template")] {
        #[promote_1_4(suffix = "KHR")]
        pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR: VkDescriptorUpdateTemplateTypeKHR = 1;

        vk_ext_command! {
            pub fn vkCmdPushDescriptorSetWithTemplateKHR(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: VkPipelineLayout, set: u32, pData: *const c_void);
            suffix = "KHR";
            promote = "1.4";
        }
    }
}
