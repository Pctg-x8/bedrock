//! Device Extension - https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_copy_commands2.html

pub const VK_KHR_COPY_COMMANDS_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME: &str = "VK_KHR_copy_commands2";

use crate::*;
use derives::{TypedVulkanStructure, promote_1_3, vk_ext_command};

#[promote_1_3]
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2_KHR: VkStructureType = ext_enum_value(338, 0) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2_KHR: VkStructureType = ext_enum_value(338, 1) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2_KHR: VkStructureType = ext_enum_value(338, 2) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2_KHR: VkStructureType = ext_enum_value(338, 3) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2_KHR: VkStructureType = ext_enum_value(338, 4) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2_KHR: VkStructureType = ext_enum_value(338, 5) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_BUFFER_COPY_2_KHR: VkStructureType = ext_enum_value(338, 6) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_IMAGE_COPY_2_KHR: VkStructureType = ext_enum_value(338, 7) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_IMAGE_BLIT_2_KHR: VkStructureType = ext_enum_value(338, 8) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2_KHR: VkStructureType = ext_enum_value(338, 9) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2_KHR: VkStructureType = ext_enum_value(338, 10) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_COPY_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkBufferCopy2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkCopyBufferInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcBuffer: VkBuffer,
    pub dstBuffer: VkBuffer,
    pub regionCount: u32,
    pub pRegions: *const VkBufferCopy2KHR,
}

#[repr(C)]
#[derive(Clone, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_COPY_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkImageCopy2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkCopyImageInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageCopy2KHR,
}

#[repr(C)]
#[derive(Clone, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkBufferImageCopy2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkCopyBufferToImageInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcBuffer: VkBuffer,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkBufferImageCopy2KHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkCopyImageToBufferInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstBuffer: VkBuffer,
    pub regionCount: u32,
    pub pRegions: *const VkBufferImageCopy2KHR,
}

#[repr(C)]
#[derive(Clone, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_BLIT_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkImageBlit2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffests: [VkOffset3D; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkBlitImageInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageBlit2KHR,
    pub filter: VkFilter,
}

#[repr(C)]
#[derive(Clone, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkImageResolve2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkResolveImageInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageResolve2KHR,
}

vk_ext_command! {
    pub fn vkCmdCopyBuffer2KHR(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2KHR);
    suffix = "KHR";
    promote = "1.3";
}

vk_ext_command! {
    pub fn vkCmdCopyImage2KHR(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2KHR);
    suffix = "KHR";
    promote = "1.3";
}

vk_ext_command! {
    pub fn vkCmdCopyBufferToImage2KHR(commandBuffer: VkCommandBuffer, pCopyBuferToImageInfo: *const VkCopyBufferToImageInfo2KHR);
    suffix = "KHR";
    promote = "1.3";
}

vk_ext_command! {
    pub fn vkCmdCopyImageToBuffer2KHR(commandBuffer: VkCommandBuffer, pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2KHR);
    suffix = "KHR";
    promote = "1.3";
}

vk_ext_command! {
    pub fn vkCmdBlitImage2KHR(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2KHR);
    suffix = "KHR";
    promote = "1.3";
}

vk_ext_command! {
    pub fn vkCmdResolveImage2KHR(commandBuffer: VkCommandBuffer, pResolveImageInfo: *const VkResolveImageInfo2KHR);
    suffix = "KHR";
    promote = "1.3";
}
