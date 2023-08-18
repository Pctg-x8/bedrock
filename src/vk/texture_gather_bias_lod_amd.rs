//! VK_AMD_texture_gather_bias_lod extensions

pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: usize = 1;
pub static VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &'static str = "VK_AMD_texture_gather_bias_lod";

use super::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD)]
pub struct VkTextureLODGatherFormatPropertiesAMD {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supportsTextureGatherLODBiasAMD: VkBool32,
}
impl VkTextureLODGatherFormatPropertiesAMD {
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
