pub const VK_KHR_MULTIVIEW_SPEC_VERSION: usize = 1;
pub const VK_KHR_MULTIVIEW_EXTENSION_NAME: &str = "VK_KHR_multiview";

const EXT_NUMBER: u16 = 54;

use super::*;

pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: VkStructureType = ext_enum_value(EXT_NUMBER, 0) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: VkStructureType =
    ext_enum_value(EXT_NUMBER, 1) as _;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: VkStructureType =
    ext_enum_value(EXT_NUMBER, 2) as _;

pub const VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR: VkDependencyFlagBits = ext_enum_value(EXT_NUMBER, 0) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR)]
pub struct VkRenderPassMultiviewCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub subpassCount: u32,
    pub pViewMasks: *const u32,
    pub dependencyCount: u32,
    pub pViewOffsets: *const i32,
    pub correlationMaskCount: u32,
    pub pCorrelationMasks: *const u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR)]
pub struct VkPhysicalDeviceMultiviewFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    /// Multiple views in a renderpass
    pub multiview: VkBool32,
    /// Multiple views in a renderpass w/ geometry shader
    pub multiviewGeometryShader: VkBool32,
    /// Multiple views in a renderpass w/ tessellation shader
    pub multiviewTessellationShader: VkBool32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR)]
pub struct VkPhysicalDeviceMultiviewPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    /// max number of views in a subpass
    pub maxMultiviewViewCount: u32,
    /// max instance index for a draw in a multiview subpass
    pub maxMultiviewInstanceIndex: u32,
}

cfg_if! {
    if #[cfg(feature = "Allow1_1APIs")] {
        pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO: VkStructureType = VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR;
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR;
        pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR;

        pub const VK_DEPENDENCY_VIEW_LOCAL_BIT: VkDependencyFlagBits = VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR;

        pub type VkRenderPassMultiviewCreateInfo = VkRenderPassMultiviewCreateInfoKHR;
        pub type VkPhysicalDeviceMultiviewFeatures = VkPhysicalDeviceMultiviewFeaturesKHR;
        pub type VkPhysicalDeviceMultiviewProperties = VkPhysicalDeviceMultiviewPropertiesKHR;
    }
}
