//! VK_EXT_discard_rectangles extensions

pub const VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION: usize = 1;
pub static VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &'static str = "VK_EXT_discard_rectangles";

use super::*;
use crate::PFN;

pub const VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT: VkDynamicState = ext_enum_value(100, 0) as _;

pub type VkDiscardRectangleModeEXT = i32;
pub const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: VkDiscardRectangleModeEXT = 0;
pub const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: VkDiscardRectangleModeEXT = 1;

pub type VkPipelineDiscardRectangleStateCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT)]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxDiscardRectangles: u32,
}
impl VkPhysicalDeviceDiscardRectanglePropertiesEXT {
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
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT)]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
    pub discardRectangleMode: VkDiscardRectangleModeEXT,
    pub discardRectangleCount: u32,
    pub pDiscardRectangles: *const VkRect2D,
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetDiscardRectangleEXT)]
pub struct PFN_vkCmdSetDiscardRectangleEXT(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        firstDiscardRectangle: u32,
        discardRectangleCount: u32,
        pDiscardRectangles: *const VkRect2D,
    ),
);
