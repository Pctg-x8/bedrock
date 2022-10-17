//! VK_AMD_texture_gather_bias_lod extensions

pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: usize = 1;
pub static VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &'static str = "VK_AMD_texture_gather_bias_lod";

use super::*;
use libc::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD"]
pub struct VkTextureLODGatherFormatPropertiesAMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supportsTextureGatherLODBiasAMD: VkBool32,
}
