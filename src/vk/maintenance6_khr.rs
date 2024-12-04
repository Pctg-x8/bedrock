//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance6.html

pub const VK_KHR_MAINTENANCE_6_SPEC_VERSION: usize = 1;
pub const VK_KHR_MAINTENANCE_6_EXTENSION_NAME: &'static str = "VK_KHR_maintenance6";

use derives::vk_ext_command;

use super::*;

pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR: VkStructureType = ext_enum_value(546, 0) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR: VkStructureType = ext_enum_value(546, 1) as _;
pub const VK_STRUCTURE_TYPE_BIND_MEMORY_STATUS_KHR: VkStructureType = ext_enum_value(546, 2) as _;
pub const VK_STRUCTURE_TYPE_BIND_DESCRIPTOR_SETS_INFO_KHR: VkStructureType = ext_enum_value(546, 3) as _;
pub const VK_STRUCTURE_TYPE_PUSH_CONSTANTS_INFO_KHR: VkStructureType = ext_enum_value(546, 4) as _;
#[cfg(feature = "VK_KHR_push_descriptor")]
pub const VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_INFO_KHR: VkStructureType = ext_enum_value(546, 5) as _;
#[cfg(feature = "VK_KHR_push_descriptor")]
pub const VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR: VkStructureType = ext_enum_value(546, 6) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure, VulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR)]
pub struct VkPhysicalDeviceMaintenance6FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maintenance6: VkBool32,
}
impl VkPhysicalDeviceMaintenance6FeaturesKHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(<Self as VulkanSinkStructure>::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR)]
pub struct VkPhysicalDeviceMaintenance6PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub blockTexelViewCompatibleMultipleLayers: VkBool32,
    pub maxCombinedImageSamplerDescriptorCount: u32,
    pub fragmentShadingRateClampCombinerInputs: VkBool32,
}
impl VkPhysicalDeviceMaintenance6PropertiesKHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(<Self as VulkanSinkStructure>::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_MEMORY_STATUS_KHR)]
pub struct VkBindMemoryStatusKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pResult: *mut VkResult,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_DESCRIPTOR_SETS_INFO_KHR)]
pub struct VkBindDescriptorSetsInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub stageFlags: VkShaderStageFlags,
    pub layout: VkPipelineLayout,
    pub firstSet: u32,
    pub descriptorSetCount: u32,
    pub pDescriptorSets: *const VkDescriptorSet,
    pub dynamicOffsetCount: u32,
    pub pDynamicOffsets: *const u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PUSH_CONSTANTS_INFO_KHR)]
pub struct VkPushConstantsInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub layout: VkPipelineLayout,
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
    pub pValues: *const core::ffi::c_void,
}

#[cfg(feature = "VK_KHR_push_descriptor")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_INFO_KHR)]
pub struct VkPushDescriptorSetInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub stageFlags: VkShaderStageFlags,
    pub layout: VkPipelineLayout,
    pub set: u32,
    pub descriptorWriteCount: u32,
    pub pDescriptorWrites: *const VkWriteDescriptorSet,
}

#[cfg(feature = "VK_KHR_push_descriptor")]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR)]
pub struct VkPushDescriptorSetWithTemplateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pub layout: VkPipelineLayout,
    pub set: u32,
    pub pData: *const core::ffi::c_void,
}

vk_ext_command!(
    pub fn vkCmdBindDescriptorSets2KHR(commandBuffer: VkCommandBuffer, pBindDescriptorSetsInfo: *const VkBindDescriptorSetsInfoKHR);
    suffix = "KHR";
);
vk_ext_command!(
    pub fn vkCmdPushConstants2KHR(commandBuffer: VkCommandBuffer, pPushConstantsInfo: *const VkPushConstantsInfoKHR);
    suffix = "KHR";
);
#[cfg(feature = "VK_KHR_push_descriptor")]
vk_ext_command!(
    pub fn vkCmdPushDescriptorSet2KHR(commandBuffer: VkCommandBuffer, pPushDEscriptorSetInof: *const VkPushDescriptorSetInfoKHR);
    suffix = "KHR";
);
#[cfg(feature = "VK_KHR_push_descriptor")]
vk_ext_command!(
    pub fn vkCmdPushDescriptorSetWithTemplate2KHR(commandBuffer: VkCommandBuffer, pPushDescriptorSetWithTemplateInfo: *const VkPushDescriptorSetWithTemplateInfoKHR);
    suffix = "KHR";
);
