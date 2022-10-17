//! VK_EXT_sample_locations extension

pub const VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION: usize = 1;
pub const VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &str = "VK_EXT_sample_locations";

use super::*;
use libc::*;
use std::mem::zeroed;

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct VkSampleLocationEXT {
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkSampleLocationsInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub sampleLocationsPerPixel: VkSampleCountFlags,
    pub sampleLocationGridSize: VkExtent2D,
    pub sampleLocationsCount: u32,
    pub pSampleLocations: *const VkSampleLocationEXT,
}
impl Default for VkSampleLocationsInfoEXT {
    fn default() -> Self {
        VkSampleLocationsInfoEXT {
            sType: VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT,
            ..unsafe { zeroed() }
        }
    }
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
#[derive(Clone, Debug)]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachmentInitialSampleLocationsCount: u32,
    pub pAttachmentInitialSampleLocations: *const VkAttachmentSampleLocationsEXT,
    pub postSubpassSampleLocationsCount: u32,
    pub pPostSubpassSampleLocations: *const VkSubpassSampleLocationsEXT,
}
impl Default for VkRenderPassSampleLocationsBeginInfoEXT {
    fn default() -> Self {
        VkRenderPassSampleLocationsBeginInfoEXT {
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
            ..unsafe { zeroed() }
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub sampleLocationsEnable: VkBool32,
    pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
impl Default for VkPipelineSampleLocationsStateCreateInfoEXT {
    fn default() -> Self {
        VkPipelineSampleLocationsStateCreateInfoEXT {
            sType: VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
            ..unsafe { zeroed() }
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub sampleLocationSampleCounts: VkSampleCountFlags,
    pub maxSampleLocationGridSize: VkExtent2D,
    pub sampleLocationCoordinateRange: [c_float; 2],
    pub sampleLocationSubpixelBits: u32,
    pub variableSampleLocations: VkBool32,
}
impl Default for VkPhysicalDeviceSampleLocationsPropertiesEXT {
    fn default() -> Self {
        VkPhysicalDeviceSampleLocationsPropertiesEXT {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT,
            ..unsafe { zeroed() }
        }
    }
}

#[repr(C)]
pub struct VkMultisamplePropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxSampleLocationGridSize: VkExtent2D,
}
impl Default for VkMultisamplePropertiesEXT {
    fn default() -> Self {
        VkMultisamplePropertiesEXT {
            sType: VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT,
            ..unsafe { zeroed() }
        }
    }
}

pub type PFN_vkCmdSetSampleLocationsEXT =
    extern "system" fn(commandBuffer: VkCommandBuffer, pSampleLocationsInfo: *const VkSampleLocationsInfoEXT);
pub type PFN_vkGetPhysicalDeviceMultisampleProeprtiesEXT = extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    samples: VkSampleCountFlags,
    pMultisampleProperties: *mut VkMultisamplePropertiesEXT,
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
