//! VK_EXT_image_drm_format_modifier

pub const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: usize = 1;
pub const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: &'static str = "VK_EXT_image_drm_format_modifier";

use super::*;
use crate::ffi_helper::ArrayFFIExtensions;
use crate::PFN;

pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: VkStructureType = ext_enum_value(159, 0) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: VkStructureType =
    ext_enum_value(159, 2) as _;
pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: VkStructureType =
    ext_enum_value(159, 3) as _;
pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: VkStructureType =
    ext_enum_value(159, 4) as _;
pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: VkStructureType = ext_enum_value(159, 5) as _;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: VkStructureType = ext_enum_value(159, 6) as _;

pub const VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT: VkImageTiling = ext_enum_value(159, 0) as _;

vk_bitmask! {
    extending enum VkImageAspectFlagBits {
        pub VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT: 7,
        pub VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT: 8,
        pub VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT: 9,
        pub VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT: 10,
    }
}

#[repr(C)]
pub struct VkDrmFormatModifierPropertiesEXT {
    pub drmFormatModifier: u64,
    pub drmFormatModifierPlaneCount: u32,
    pub drmFormatModifierTilingFeatures: VkFormatFeatureFlags,
}

#[repr(C)]
#[derive(VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT)]
pub struct VkDrmFormatModifierPropertiesListEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub drmFormatModifierCount: u32,
    pub pDrmFormatModifierProperties: *mut VkDrmFormatModifierPropertiesEXT,
}
impl VkDrmFormatModifierPropertiesListEXT {
    #[inline]
    pub const fn new_unsized() -> Self {
        Self {
            sType: Self::TYPE,
            pNext: std::ptr::null_mut(),
            drmFormatModifierCount: 0,
            pDrmFormatModifierProperties: std::ptr::null_mut(),
        }
    }

    #[inline]
    pub const fn with_sink(self, ptr: *mut VkDrmFormatModifierPropertiesEXT) -> Self {
        Self {
            pDrmFormatModifierProperties: ptr,
            ..self
        }
    }

    #[inline]
    pub unsafe fn properties(&self) -> &[VkDrmFormatModifierPropertiesEXT] {
        std::slice::from_raw_parts(self.pDrmFormatModifierProperties, self.drmFormatModifierCount as _)
    }

    #[inline]
    pub unsafe fn properties_mut(&mut self) -> &mut [VkDrmFormatModifierPropertiesEXT] {
        std::slice::from_raw_parts_mut(self.pDrmFormatModifierProperties, self.drmFormatModifierCount as _)
    }
}

#[repr(C)]
#[derive(VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT)]
pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub drmFormatModifier: u64,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

#[repr(C)]
#[derive(VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT)]
pub struct VkImageDrmFormatModifierListCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub drmFormatModifierCount: u32,
    pub pDrmFormatModifiers: *const u64,
}
impl VkImageDrmFormatModifierListCreateInfoEXT {
    pub const unsafe fn from_values(modifiers: &[u64]) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: std::ptr::null(),
            drmFormatModifierCount: modifiers.len() as _,
            pDrmFormatModifiers: modifiers.as_ptr_empty_null(),
        }
    }
}

#[repr(C)]
#[derive(VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT)]
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub drmFormatModifier: u64,
    pub drmFormatModifierPlaneCount: u32,
    pub pPlaneLayouts: *const VkSubresourceLayout,
}

#[repr(C)]
#[derive(VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT)]
pub struct VkImageDrmFormatModifierPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub drmFormatModifier: u64,
}
impl VkImageDrmFormatModifierPropertiesEXT {
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

#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[repr(C)]
pub struct VkDrmFormatModifierProperties2EXT {
    pub drmFormatModifier: u64,
    pub drmFormatModifierPlaneCount: u32,
    pub drmFormatModifierTilingFeatures: VkFormatFeatureFlags2,
}

#[cfg(feature = "VK_KHR_format_feature_flags2")]
#[repr(C)]
#[derive(VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT)]
pub struct VkDrmFormatModifierPropertiesList2EXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub drmFormatModifierCount: u32,
    pub pDrmFormatModifierProperties: *mut VkDrmFormatModifierProperties2EXT,
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetImageDrmFormatModifierPropertiesEXT)]
pub struct PFN_vkGetImageDrmFormatModifierPropertiesEXT(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        image: VkImage,
        pProperties: *mut VkImageDrmFormatModifierPropertiesEXT,
    ) -> VkResult,
);

#[cfg(all(feature = "Implements", not(feature = "DynamicLoaded")))]
extern "system" {
    pub fn vkGetImageDrmFormatModifierPropertiesEXT(
        device: VkDevice,
        image: VkImage,
        pProperties: *mut VkImageDrmFormatModifierPropertiesEXT,
    ) -> VkResult;
}
