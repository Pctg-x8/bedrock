//! VK_EXT_blend_operation_advanced extensions

pub const VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: usize = 2;
pub static VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &'static str = "VK_EXT_blend_operation_advanced";

use super::*;

pub const VK_BlEND_OP_ZERO_EXT: VkBlendOp = ext_enum_value(149, 0) as _;
pub const VK_BLEND_OP_SRC_EXT: VkBlendOp = ext_enum_value(149, 1) as _;
pub const VK_BLEND_OP_DST_EXT: VkBlendOp = ext_enum_value(149, 2) as _;
pub const VK_BLEND_OP_SRC_OVER_EXT: VkBlendOp = ext_enum_value(149, 3) as _;
pub const VK_BLEND_OP_DST_OVER_EXT: VkBlendOp = ext_enum_value(149, 4) as _;
pub const VK_BLEND_OP_SRC_IN_EXT: VkBlendOp = ext_enum_value(149, 5) as _;
pub const VK_BLEND_OP_DST_IN_EXT: VkBlendOp = ext_enum_value(149, 6) as _;
pub const VK_BLEND_OP_SRC_OUT_EXT: VkBlendOp = ext_enum_value(149, 7) as _;
pub const VK_BLEND_OP_DST_OUT_EXT: VkBlendOp = ext_enum_value(149, 8) as _;
pub const VK_BLEND_OP_SRC_ATOP_EXT: VkBlendOp = ext_enum_value(149, 9) as _;
pub const VK_BLEND_OP_DST_ATOP_EXT: VkBlendOp = ext_enum_value(149, 10) as _;
pub const VK_BLEND_OP_XOR_EXT: VkBlendOp = ext_enum_value(149, 11) as _;
pub const VK_BLEND_OP_MULTIPLY_EXT: VkBlendOp = ext_enum_value(149, 12) as _;
pub const VK_BLEND_OP_SCREEN_EXT: VkBlendOp = ext_enum_value(149, 13) as _;
pub const VK_BLEND_OP_OVERLAY_EXT: VkBlendOp = ext_enum_value(149, 14) as _;
pub const VK_BLEND_OP_DARKEN_EXT: VkBlendOp = ext_enum_value(149, 15) as _;
pub const VK_BLEND_OP_LIGHTEN_EXT: VkBlendOp = ext_enum_value(149, 16) as _;
pub const VK_BLEND_OP_COLORDODGE_EXT: VkBlendOp = ext_enum_value(149, 17) as _;
pub const VK_BLEND_OP_COLORBURN_EXT: VkBlendOp = ext_enum_value(149, 18) as _;
pub const VK_BLEND_OP_HARDLIGHT_EXT: VkBlendOp = ext_enum_value(149, 19) as _;
pub const VK_BLEND_OP_SOFTLIGHT_EXT: VkBlendOp = ext_enum_value(149, 20) as _;
pub const VK_BLEND_OP_DIFFERENCE_EXT: VkBlendOp = ext_enum_value(149, 21) as _;
pub const VK_BLEND_OP_EXCLUSION_EXT: VkBlendOp = ext_enum_value(149, 22) as _;
pub const VK_BLEND_OP_INVERT_EXT: VkBlendOp = ext_enum_value(149, 23) as _;
pub const VK_BLEND_OP_INVERT_RGB_EXT: VkBlendOp = ext_enum_value(149, 24) as _;
pub const VK_BLEND_OP_LINEARDODGE_EXT: VkBlendOp = ext_enum_value(149, 25) as _;
pub const VK_BLEND_OP_LINEARBURN_EXT: VkBlendOp = ext_enum_value(149, 26) as _;
pub const VK_BLEND_OP_VIVIDLIGHT_EXT: VkBlendOp = ext_enum_value(149, 27) as _;
pub const VK_BLEND_OP_LINEARLIGHT_EXT: VkBlendOp = ext_enum_value(149, 28) as _;
pub const VK_BLEND_OP_PINLIGHT_EXT: VkBlendOp = ext_enum_value(149, 29) as _;
pub const VK_BLEND_OP_HARDMIX_EXT: VkBlendOp = ext_enum_value(149, 30) as _;
pub const VK_BLEND_OP_HSL_HUE_EXT: VkBlendOp = ext_enum_value(149, 31) as _;
pub const VK_BLEND_OP_HSL_SATURATION_EXT: VkBlendOp = ext_enum_value(149, 32) as _;
pub const VK_BLEND_OP_HSL_COLOR_EXT: VkBlendOp = ext_enum_value(149, 33) as _;
pub const VK_BLEND_OP_HSL_LUMINOSITY_EXT: VkBlendOp = ext_enum_value(149, 34) as _;
pub const VK_BLEND_OP_PLUS_EXT: VkBlendOp = ext_enum_value(149, 35) as _;
pub const VK_BLEND_OP_PLUS_CLAMPED_EXT: VkBlendOp = ext_enum_value(149, 36) as _;
pub const VK_BLEND_OP_PLUS_CLAMPED_ALHPA_EXT: VkBlendOp = ext_enum_value(149, 37) as _;
pub const VK_BLEND_OP_PLUS_DARKER_EXT: VkBlendOp = ext_enum_value(149, 38) as _;
pub const VK_BLEND_OP_MINUS_EXT: VkBlendOp = ext_enum_value(149, 39) as _;
pub const VK_BLEND_OP_MINUS_CLAMPED_EXT: VkBlendOp = ext_enum_value(149, 40) as _;
pub const VK_BLEND_OP_CONTRAST_EXT: VkBlendOp = ext_enum_value(149, 41) as _;
pub const VK_BLEND_OP_INVERT_OVG_EXT: VkBlendOp = ext_enum_value(149, 42) as _;
pub const VK_BLEND_OP_RED_EXT: VkBlendOp = ext_enum_value(149, 43) as _;
pub const VK_BLEND_OP_GREEN_EXT: VkBlendOp = ext_enum_value(149, 44) as _;
pub const VK_BLEND_OP_BLUE_EXT: VkBlendOp = ext_enum_value(149, 45) as _;

pub type VkBlendOverlapEXT = i32;
pub const VK_BLEND_OVERLAP_UNCORRELATED_EXT: VkBlendOverlapEXT = 0;
pub const VK_BLEND_OVERLAP_DISJOINT_EXT: VkBlendOverlapEXT = 1;
pub const VK_BLEND_OVERLAP_CONJOINT_EXT: VkBlendOverlapEXT = 2;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub advancedBlendCoherentOperations: VkBool32,
}
impl VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
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

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT)]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub advancedBlendMaxColorAttachments: u32,
    pub advancedBlendIndependentBlend: VkBool32,
    pub advancedBlendNonPremultipliedSrcColor: VkBool32,
    pub advancedBlendNonPremultipliedDstColor: VkBool32,
    pub advancedBlendCorrelatedOverlap: VkBool32,
    pub advancedBlendAllOperations: VkBool32,
}
impl VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
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

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT)]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcPremultiplied: VkBool32,
    pub dstPremultiplied: VkBool32,
    pub blendOverlap: VkBlendOverlapEXT,
}
