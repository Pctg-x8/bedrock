//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_descriptor_indexing.html

pub const VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: usize = 2;
pub const VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME: &str = "VK_EXT_descriptor_indexing";

use super::*;
use crate::vk2::*;
use derives::promote_1_2;

#[promote_1_2]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: VkStructureType =
    ext_enum_value(162, 0) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: VkStructureType =
    ext_enum_value(162, 1) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: VkStructureType =
    ext_enum_value(162, 2) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: VkStructureType =
    ext_enum_value(162, 3) as _;
#[promote_1_2]
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: VkStructureType =
    ext_enum_value(162, 4) as _;

#[promote_1_2]
pub const VK_ERROR_FRAGMENTATION_EXT: VkResult = VkResult::ext_err_value(162, 0);

vk_bitmask! {
    #[promote_1_2(suffix = "EXT")]
    pub enum VkDescriptorBindingFlagBitsEXT {
        #[promote_1_2]
        pub VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT: 0,
        #[promote_1_2]
        pub VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT: 1,
        #[promote_1_2]
        pub VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT: 2,
        #[promote_1_2]
        pub VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT: 3,
    }
}
#[promote_1_2(suffix = "EXT")]
pub type VkDescriptorBindingFlagsEXT = VkFlags;

vk_bitmask! {
    extending enum VkDescriptorPoolCreateFlagBits {
        #[promote_1_2]
        pub VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT: 2,
    }
}

vk_bitmask! {
    extending enum VkDescriptorSetLayoutCreateFlagBits {
        #[promote_1_2]
        pub VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT: 1,
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT)]
#[promote_1_2(suffix = "EXT")]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub bindingCount: u32,
    pub pBindingFlags: *const VkDescriptorBindingFlagsEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure, TypedVulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT)]
#[promote_1_2(suffix = "EXT")]
pub struct VkPhysicalDeviceDescriptorIndexingFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
    pub descriptorBindingPartiallyBound: VkBool32,
    pub descriptorBindingVariableDescriptorCount: VkBool32,
    pub runtimeDescriptorArray: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT)]
#[promote_1_2(suffix = "EXT")]
pub struct VkPhysicalDeviceDescriptorIndexingPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxUpdateAfterBindDescriptorsInAllPools: u32,
    pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
    pub robustBufferAccessUpdateAfterBind: VkBool32,
    pub quadDivergentImplicitLod: VkBool32,
    pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
    pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
    pub maxPerStageUpdateAfterBindResources: u32,
    pub maxDescriptorSetUpdateAfterBindSamplers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
    pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
    pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT)]
#[promote_1_2(suffix = "EXT")]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub descriptorSetCount: u32,
    pub pDescriptorCounts: *const u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, TypedVulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT)]
#[promote_1_2(suffix = "EXT")]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupportEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxVariableDescriptorCount: u32,
}
