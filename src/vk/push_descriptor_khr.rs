//! VK_KHR_push_descriptor extensions

pub const VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION: usize = 1;
pub static VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &'static str = "VK_KHR_push_descriptor";

use super::*;
use crate::PFN;

vk_bitmask! {
    extending enum VkDescriptorSetLayoutCreateFlagBits {
        pub VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR: 0
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR)]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxPushDescriptors: u32,
}
impl VkPhysicalDevicePushDescriptorPropertiesKHR {
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
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdPushDescriptorSetKHR)]
pub struct PFN_vkCmdPushDescriptorSetKHR(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        set: u32,
        descriptorWriteCount: u32,
        pDescriptorWrites: *const VkWriteDescriptorSet,
    ),
);

cfg_if! {
    if #[cfg(feature = "VK_KHR_descriptor_update_template")] {
        pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR: VkDescriptorUpdateTemplateTypeKHR = 1;

        #[cfg(feature = "Implements")]
        #[repr(transparent)]
        #[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
        #[pfn_of(vkCmdPushDescriptorSetWithTemplateKHR)]
        pub struct PFN_vkCmdPushDescriptorSetWithTemplateKHR(
            pub unsafe extern "system" fn(
                commandBuffer: VkCommandBuffer,
                descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
                layout: VkPipelineLayout,
                set: u32,
                pData: *const c_void
            ),
        );
    }
}
