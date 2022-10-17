//! VK_AMD_shader_info extension

pub const VK_AMD_SHADER_INFO_SPEC_VERSION: usize = 1;
pub const VK_AMD_SHADER_INFO_EXTENSION_NAME: &str = "VK_AMD_shader_info";

use super::*;

pub type VkShaderInfoTypeAMD = i32;
pub const VK_SHADER_INFO_TYPE_STATISTICS_AMD: VkShaderInfoTypeAMD = 0;
pub const VK_SHADER_INFO_TYPE_BINARY_AMD: VkShaderInfoTypeAMD = 1;
pub const VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD: VkShaderInfoTypeAMD = 2;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VkShaderResourceUsageAMD {
    pub numUsageVgprs: u32,
    pub numUsedSgprs: u32,
    pub ldsSizePerLocalWorkGroup: u32,
    pub ldsUsageSizeInBytes: size_t,
    pub scratchMemUsageInBytes: size_t,
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VkShaderStatisticsInfoAMD {
    pub shaderStageMask: VkShaderStageFlags,
    pub resourceUsage: VkShaderResourceUsageAMD,
    pub numPhysicalVgprs: u32,
    pub numPhysicalSgprs: u32,
    pub numAvailableVgprs: u32,
    pub numAvailableSgprs: u32,
    pub computeWorkGroupSize: [u32; 3],
}

pub type PFN_vkGetShaderInfoKHR = extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    shaderStage: VkShaderStageFlags,
    infoType: VkShaderInfoTypeAMD,
    pInfoSize: *mut size_t,
    pInfo: *mut c_void,
) -> VkResult;

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn vkGetShaderInfoKHR(
        device: VkDevice,
        pipeline: VkPipeline,
        shaderStage: VkShaderStageFlags,
        infoType: VkShaderInfoTypeAMD,
        pInfoSize: *mut size_t,
        pInfo: *mut c_void,
    ) -> VkResult;
}
