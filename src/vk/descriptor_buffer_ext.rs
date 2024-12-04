//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_descriptor_buffer.html

pub const VK_EXT_DESCRIPTOR_BUFFER_SPEC_VERSION: usize = 1;
pub const VK_EXT_DESCRIPTOR_BUFFER_EXTENSION_NAME: &'static str = "VK_EXT_descriptor_buffer";

use super::*;
use derives::vk_ext_command;

pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT: VkStructureType =
    ext_enum_value(317, 0) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT: VkStructureType =
    ext_enum_value(317, 1) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT: VkStructureType =
    ext_enum_value(317, 2) as _;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT: VkStructureType = ext_enum_value(317, 3) as _;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT: VkStructureType = ext_enum_value(317, 4) as _;
pub const VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType = ext_enum_value(317, 5) as _;
pub const VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType = ext_enum_value(317, 6) as _;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType = ext_enum_value(317, 7) as _;
pub const VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: VkStructureType = ext_enum_value(317, 8) as _;
pub const VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT: VkStructureType =
    ext_enum_value(317, 10) as _;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT: VkStructureType = ext_enum_value(317, 11) as _;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT: VkStructureType =
    ext_enum_value(317, 12) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT)]
pub struct VkPhysicalDeviceDescriptorBufferPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub combinedImageSamplerDescriptorSingleArray: VkBool32,
    pub bufferlessPushDescriptors: VkBool32,
    pub allowSamplerImageViewPostSubmitCreation: VkBool32,
    pub descriptorBufferOffsetAlignment: VkDeviceSize,
    pub maxDescriptorBufferBindings: u32,
    pub maxResourceDescriptorBufferBindings: u32,
    pub maxSamplerDescriptorBufferBindings: u32,
    pub maxEmbeddedImmutableSamplerBindings: u32,
    pub maxEmbeddedImmutableSamplers: u32,
    pub bufferCaptureReplayDescriptorDataSize: usize,
    pub imageCaptureReplayDescriptorDataSize: usize,
    pub imageViewCaptureReplayDescriptorDataSize: usize,
    pub samplerCaptureReplayDescriptorDataSize: usize,
    pub accelerationStructureCaptureReplayDescriptorDataSize: usize,
    pub samplerDescriptorSize: usize,
    pub combinedImageSamplerDescriptorSize: usize,
    pub sampledImageDescriptorSize: usize,
    pub storageImageDescriptorSize: usize,
    pub uniformTexelBufferDescriptorSize: usize,
    pub robustUniformTexelBufferDescriptorSize: usize,
    pub storageTexelBufferDescriptorSize: usize,
    pub robustStorageTexelBufferDescriptorSize: usize,
    pub uniformBufferDescriptorSize: usize,
    pub robustUniformBufferDescriptorSize: usize,
    pub storageBufferDescriptorSize: usize,
    pub robustStorageBufferDescriptorSize: usize,
    pub inputAttachmentDescriptorSize: usize,
    pub accelerationStructureDescriptorSize: usize,
    pub maxSamplerDescriptorBufferRange: VkDeviceSize,
    pub maxResourceDescriptorBufferRange: VkDeviceSize,
    pub samplerDescriptorBufferAddressSpaceSize: VkDeviceSize,
    pub resourceDescriptorBufferAddressSpaceSize: VkDeviceSize,
    pub descriptorBufferAddressSpaceSize: VkDeviceSize,
}
impl VkPhysicalDeviceDescriptorBufferPropertiesEXT {
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
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT)]
pub struct VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub combinedImageSamplerDensityMapDescriptorSize: usize,
}
impl VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure, VulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT)]
pub struct VkPhysicalDeviceDescriptorBufferFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub descriptorBuffer: VkBool32,
    pub descriptorBufferCaptureReplay: VkBool32,
    pub descriptorBufferImageLayoutIgnored: VkBool32,
    pub descriptorBufferPushDescriptors: VkBool32,
}
impl VkPhysicalDeviceDescriptorBufferFeaturesEXT {
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
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_ADDRESS_INFO_EXT)]
pub struct VkDescriptorAddressInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub address: VkDeviceAddress,
    pub range: VkDeviceSize,
    pub format: VkFormat,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_INFO_EXT)]
pub struct VkDescriptorBufferBindingInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub address: VkDeviceAddress,
    pub usage: VkBufferUsageFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT)]
pub struct VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub buffer: VkBuffer,
}

#[repr(C)]
pub union VkDescriptorDataEXT {
    pub pSampler: *const VkSampler,
    pub pCombinedImageSampler: *const VkDescriptorImageInfo,
    pub pInputAttachmentImage: *const VkDescriptorImageInfo,
    pub pSampledImage: *const VkDescriptorImageInfo,
    pub pStorageImage: *const VkDescriptorImageInfo,
    pub pUniformTexelBuffer: *const VkDescriptorAddressInfoEXT,
    pub pStorageTexelBuffer: *const VkDescriptorAddressInfoEXT,
    pub pUniformBuffer: *const VkDescriptorAddressInfoEXT,
    pub pStorageBuffer: *const VkDescriptorAddressInfoEXT,
    pub accelerationStructure: VkDeviceAddress,
}

#[repr(C)]
#[derive(VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DESCRIPTOR_GET_INFO_EXT)]
pub struct VkDescriptorGetInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub r#type: VkDescriptorType,
    pub data: VkDescriptorDataEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT)]
pub struct VkBufferCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub buffer: VkBuffer,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT)]
pub struct VkImageCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT)]
pub struct VkImageViewCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub imageView: VkImageView,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT)]
pub struct VkSamplerCaptureDescriptorDataInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub sampler: VkSampler,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT)]
pub struct VkOpaqueCaptureDescriptorDataCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub opaqueCaptureDescriptorData: *const core::ffi::c_void,
}

vk_ext_command!(
    pub fn vkGetDescriptorSetLayoutSizeEXT(device: VkDevice, layout: VkDescriptorSetLayout, pLayoutSizeInBytes: *mut VkDeviceSize);
    suffix = "EXT";
);
vk_ext_command!(
    pub fn vkGetDescriptorSetLayoutBindingOffsetEXT(device: VkDevice, layout: VkDescriptorSetLayout, binding: u32, pOffset: *mut VkDeviceSize);
    suffix = "EXT";
);
vk_ext_command!(
    pub fn vkGetDescriptorEXT(device: VkDevice, pDescriptorInfo: *const VkDescriptorGetInfoEXT, dataSize: usize, pDescriptor: *mut core::ffi::c_void);
    suffix = "EXT";
);
vk_ext_command!(
    pub fn vkCmdBindDescriptorBuffersEXT(commandBuffer: VkCommandBuffer, bufferCount: u32, pBindingInfos: *const VkDescriptorBufferBindingInfoEXT);
    suffix = "EXT";
);
vk_ext_command!(
    pub fn vkCmdSetDescriptorBufferOffsetsEXT(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, setCount: u32, pBufferIndices: *const u32, pOffsets: *const VkDeviceSize);
    suffix = "EXT";
);
vk_ext_command!(
    pub fn vkCmdBindDescriptorBufferEmbeddedSamplersEXT(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32);
    suffix = "EXT";
);
vk_ext_command!(
    pub fn vkGetBufferOpaqueCaptureDescriptorDataEXT(device: VkDevice, pInfo: *const VkBufferCaptureDescriptorDataInfoEXT, pData: *mut core::ffi::c_void) -> VkResult;
    suffix = "EXT";
);
vk_ext_command!(
    pub fn vkGetImageOpaqueCaptureDescriptorDataEXT(device: VkDevice, pInfo: *const VkImageCaptureDescriptorDataInfoEXT, pData: *mut core::ffi::c_void) -> VkResult;
    suffix = "EXT";
);
vk_ext_command!(
    pub fn vkGetImageViewOpaqueCaptureDescriptorDataEXT(device: VkDevice, pInfo: *const VkImageViewCaptureDescriptorDataInfoEXT, pData: *mut core::ffi::c_void) -> VkResult;
    suffix = "EXT";
);
vk_ext_command!(
    pub fn vkGetSamplerOpaqueCaptureDescriptorDataEXT(device: VkDevice, pInfo: *const VkSamplerCaptureDescriptorDataInfoEXT, pData: *mut core::ffi::c_void) -> VkResult;
    suffix = "EXT";
);
