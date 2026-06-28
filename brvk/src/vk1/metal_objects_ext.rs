//! VK_EXT_metal_objects extensions

use derives::vk_ext_command;

use super::*;
use crate::vk2::*;

pub const VK_EXT_METAL_OBJECTS_SPEC_VERSION: usize = 2;
pub const VK_EXT_METAL_OBJECTS_EXTENSION_NAME: &'static str = "VK_EXT_metal_objects";

pub const VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT: VkStructureType = ext_enum_value(312, 0) as _;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT: VkStructureType = ext_enum_value(312, 1) as _;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT: VkStructureType = ext_enum_value(312, 2) as _;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: VkStructureType = ext_enum_value(312, 3) as _;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT: VkStructureType = ext_enum_value(312, 4) as _;
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT: VkStructureType = ext_enum_value(312, 5) as _;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT: VkStructureType = ext_enum_value(312, 6) as _;
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT: VkStructureType = ext_enum_value(312, 7) as _;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT: VkStructureType = ext_enum_value(312, 8) as _;
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT: VkStructureType = ext_enum_value(312, 9) as _;
pub const VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT: VkStructureType = ext_enum_value(312, 10) as _;
pub const VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT: VkStructureType = ext_enum_value(312, 11) as _;

vk_bitmask! {
    pub enum VkExportMetalObjectTypeFlagBitsEXT {
        pub VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT: 0,
        pub VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT: 1,
        pub VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT: 2,
        pub VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT: 3,
        pub VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT: 4,
        pub VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT: 5
    }
}
pub type VkExportMetalObjectTypeFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECT_CREATE_INFO_EXT)]
pub struct VkExportMetalObjectCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub exportObjectType: VkExportMetalObjectTypeFlagBitsEXT,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_METAL_OBJECTS_INFO_EXT)]
pub struct VkExportMetalObjectsInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_METAL_DEVICE_INFO_EXT)]
pub struct VkExportMetalDeviceInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    /// `pub mtlDevice: id<MTLDevice>`
    pub mtlDevice: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_METAL_COMMAND_QUEUE_INFO_EXT)]
pub struct VkExportMetalCommandQueueInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub queue: VkQueue,
    /// `pub mtlCommandQueue: id<MTLCommandQueue>`
    pub mtlCommandQueue: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_METAL_BUFFER_INFO_EXT)]
pub struct VkExportMetalBufferInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub memory: VkDeviceMemory,
    /// `pub mtlBuffer: id<MTLBuffer>`
    pub mtlBuffer: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_METAL_BUFFER_INFO_EXT)]
pub struct VkImportMetalBufferInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    /// `pub mtlBuffer: id<MTLBuffer>`
    pub mtlBuffer: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_METAL_TEXTURE_INFO_EXT)]
pub struct VkExportMetalTextureInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
    pub imageView: VkImageView,
    pub bufferView: VkBufferView,
    pub plane: VkImageAspectFlagBits,
    /// `pub mtlTexture: id<MTLTexture>`
    pub mtlTexture: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_METAL_TEXTURE_INFO_EXT)]
pub struct VkImportMetalTextureInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub plane: VkImageAspectFlagBits,
    /// `pub mtlTexture: id<MTLTexture>`
    pub mtlTexture: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_METAL_IO_SURFACE_INFO_EXT)]
pub struct VkExportMetalIOSurfaceInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
    /// `pub ioSurface: IOSurfaceRef`
    pub ioSurface: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_METAL_IO_SURFACE_INFO_EXT)]
pub struct VkImportMetalIOSurfaceInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    /// `pub ioSurface: IOSurfaceRef`
    pub ioSurface: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_EXPORT_METAL_SHARED_EVENT_INFO_EXT)]
pub struct VkExportMetalSharedEventInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphore: VkSemaphore,
    pub event: VkEvent,
    /// `pub mtlSharedEvent: id<MTLSharedEvent>`
    pub mtlSharedEvent: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMPORT_METAL_SHARED_EVENT_INFO_EXT)]
pub struct VkImportMetalSharedEventInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    /// `pub mtlSharedEvent: id<MTLSharedEvent>`
    pub mtlSharedEvent: *mut core::ffi::c_void,
}

vk_ext_command! {
    pub fn vkExportMetalObjectsEXT(device: VkDevice, pMetalObjectsInfo: *mut VkExportMetalObjectsInfoEXT);
}
