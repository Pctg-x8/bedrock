//! VK_EXT_sample_locations extension

pub const VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION: usize = 1;
pub const VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &str = "VK_EXT_sample_locations";

use super::*;
use crate::PFN;

pub const VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT: VkDynamicState = ext_enum_value(144, 0) as _;

vk_bitmask! {
    extending enum VkImageCreateFlagBits {
        pub VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT: 12
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct VkSampleLocationEXT {
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT)]
pub struct VkSampleLocationsInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub sampleLocationsPerPixel: VkSampleCountFlags,
    pub sampleLocationGridSize: VkExtent2D,
    pub sampleLocationsCount: u32,
    pub pSampleLocations: *const VkSampleLocationEXT,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkAttachmentSampleLocationsEXT {
    pub attachmentIndex: u32,
    pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkSubpassSampleLocationsEXT {
    pub subpassIndex: u32,
    pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT)]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachmentInitialSampleLocationsCount: u32,
    pub pAttachmentInitialSampleLocations: *const VkAttachmentSampleLocationsEXT,
    pub postSubpassSampleLocationsCount: u32,
    pub pPostSubpassSampleLocations: *const VkSubpassSampleLocationsEXT,
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT)]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub sampleLocationsEnable: VkBool32,
    pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}

#[repr(C)]
#[derive(Clone, Debug, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT)]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub sampleLocationSampleCounts: VkSampleCountFlags,
    pub maxSampleLocationGridSize: VkExtent2D,
    pub sampleLocationCoordinateRange: [c_float; 2],
    pub sampleLocationSubpixelBits: u32,
    pub variableSampleLocations: VkBool32,
}
impl VkPhysicalDeviceSampleLocationsPropertiesEXT {
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
#[derive(Clone, Debug, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT)]
pub struct VkMultisamplePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxSampleLocationGridSize: VkExtent2D,
}
impl VkMultisamplePropertiesEXT {
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

#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetSampleLocationsEXT)]
pub struct PFN_vkCmdSetSampleLocationsEXT(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pSampleLocationsInfo: *const VkSampleLocationsInfoEXT),
);
#[cfg(feature = "Implements")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetPhysicalDeviceMultisampleProeprtiesEXT)]
pub struct PFN_vkGetPhysicalDeviceMultisampleProeprtiesEXT(
    pub  unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        samples: VkSampleCountFlags,
        pMultisampleProperties: *mut VkMultisamplePropertiesEXT,
    ),
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkCmdSetSampleLocationsEXT(
        commandBuffer: VkCommandBuffer,
        pSampleLocationsInfo: *const VkSampleLocationsInfoEXT,
    );
    pub fn vkGetPhysicalDeviceMultisamplePropertiesEXT(
        physicalDevice: VkPhysicalDevice,
        samples: VkSampleCountFlags,
        pMultisampleProperties: *mut VkMultisamplePropertiesEXT,
    );
}
