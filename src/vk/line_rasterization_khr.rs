//! VK_KHR_line_rasterization

use super::*;
use crate::VulkanStructure;

pub const VK_KHR_LINE_RASTERIZATION_SPEC_VERSION: usize = 1;
pub const VK_KHR_LINE_RASTERIZATION_EXTENSION_NAME: &'static str = "VK_KHR_line_rasterization";

pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR: VkStructureType =
    ext_enum_value(260, 0) as _;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR: VkStructureType =
    ext_enum_value(260, 1) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR: VkStructureType =
    ext_enum_value(260, 2) as _;

pub type VkLineRasterizationModeKHR = core::ffi::c_int;
pub const VK_LINE_RASTERIZATION_MODE_DEFAULT_KHR: VkLineRasterizationModeKHR = 0;
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_KHR: VkLineRasterizationModeKHR = 1;
pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM_KHR: VkLineRasterizationModeKHR = 2;
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_KHR: VkLineRasterizationModeKHR = 3;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR)]
pub struct VkPhysicalDeviceLineRasterizationFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub rectangularLines: VkBool32,
    pub bresenhamLines: VkBool32,
    pub smoothLines: VkBool32,
    pub stippledRectangularLines: VkBool32,
    pub stippledBresenhamLines: VkBool32,
    pub stippledSmoothLines: VkBool32,
}
impl VkPhysicalDeviceLineRasterizationFeaturesKHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut sink = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = sink.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(Self::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        sink
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR)]
pub struct VkPhysicalDeviceLineRasterizationPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub lineSubPixelPrecisionBits: u32,
}
impl VkPhysicalDeviceLineRasterizationPropertiesKHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut sink = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = sink.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(Self::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        sink
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR)]
pub struct VkPipelineRasterizationLineStateCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub lineRasterizationMode: VkLineRasterizationModeKHR,
    pub stippledLineEnable: VkBool32,
    pub lineStippleFactor: u32,
    pub lineStipplePattern: u16,
}

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetLineStippleKHR)]
pub struct PFN_vkCmdSetLineStippleKHR(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineStippleFactor: u32, lineStipplePattern: u16),
);

#[implements]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCmdSetLineStippleKHR(commandBuffer: VkCommandBuffer, lineStippleFactor: u32, lineStipplePattern: u16);
}
