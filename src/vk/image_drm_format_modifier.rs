//! VK_EXT_image_drm_format_modifier

pub const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: usize = 1;
pub const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: &'static str = "VK_EXT_image_drm_format_modifier";

use super::*;

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
            pDrmFormatModifiers: modifiers.as_ptr(),
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

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetImageDrmFormatModifierPropertiesEXT)]
pub struct PFN_vkGetImageDrmFormatModifierPropertiesEXT(
    pub  extern "system" fn(
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
