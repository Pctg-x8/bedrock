//! https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance5.html

pub const VK_KHR_MAINTENANCE_5_SPEC_VERSION: usize = 1;
pub const VK_KHR_MAINTENANCE_5_EXTENSION_NAME: &'static str = "VK_KHR_maintenance5";

use derives::{promote_1_4, vk_ext_command};

use super::*;

#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR: VkStructureType = ext_enum_value(471, 0) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR: VkStructureType = ext_enum_value(471, 1) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_RENDERING_AREA_INFO_KHR: VkStructureType = ext_enum_value(471, 3) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_SUBRESOURCE_INFO_KHR: VkStructureType = ext_enum_value(471, 4) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR: VkStructureType = ext_enum_value(471, 5) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR: VkStructureType = ext_enum_value(471, 6) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_KHR: VkStructureType = ext_enum_value(339, 2) as _;
#[promote_1_4]
pub const VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_KHR: VkStructureType = ext_enum_value(339, 3) as _;

vk_bitmask! {
    #[promote_1_4(suffix = "KHR")]
    pub enum64 VkBufferUsageFlagBits2KHR {
        #[promote_1_4]
        pub VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT_KHR: 0,
        #[promote_1_4]
        pub VK_BUFFER_USAGE_2_TRANSFER_DST_BIT_KHR: 1,
        #[promote_1_4]
        pub VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR: 2,
        #[promote_1_4]
        pub VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT_KHR: 3,
        #[promote_1_4]
        pub VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT_KHR: 4,
        #[promote_1_4]
        pub VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT_KHR: 5,
        #[promote_1_4]
        pub VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT_KHR: 6,
        #[promote_1_4]
        pub VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT_KHR: 7,
        #[promote_1_4]
        pub VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT_KHR: 8,
    }
}
#[promote_1_4(suffix = "KHR")]
pub type VkBufferUsageFlags2KHR = VkFlags64;

vk_bitmask! {
    #[promote_1_4(suffix = "KHR")]
    pub enum64 VkPipelineCreateFlagBits2KHR {
        #[promote_1_4]
        pub VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT_KHR: 0,
        #[promote_1_4]
        pub VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT_KHR: 1,
        #[promote_1_4]
        pub VK_PIPELINE_CREATE_2_DERIVATIVE_BIT_KHR: 2,
        #[promote_1_4]
        pub VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: 3,
        #[promote_1_4]
        pub VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT_KHR: 4,
    }
}
#[promote_1_4(suffix = "KHR")]
pub type VkPipelineCreateFlags2KHR = VkFlags64;

#[promote_1_4]
pub const VK_FORMAT_A1B5G5R4_UNORM_PACK16_KHR: VkFormat = ext_enum_value(471, 0) as _;
#[promote_1_4]
pub const VK_FORMAT_A8_UNORM_KHR: VkFormat = ext_enum_value(471, 1) as _;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure, VulkanSinkStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceMaintenance5FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maintenance5: VkBool32,
}
impl VkPhysicalDeviceMaintenance5FeaturesKHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(<Self as VulkanSinkStructure>::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPhysicalDeviceMaintenance5PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub earlyFragmentMultisampleCoverageAfterSampleCounting: VkBool32,
    pub earlyFragmentSampleMaskTestBeforeSampleCounting: VkBool32,
    pub depthStencilSwizzleOneSupport: VkBool32,
    pub polygonModePointSize: VkBool32,
    pub nonStrictSinglePixelWideLinesUseParallelogram: VkBool32,
    pub nonStrictWideLinesUseParallelogram: VkBool32,
}
impl VkPhysicalDeviceMaintenance5PropertiesKHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(<Self as VulkanSinkStructure>::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_RENDERING_AREA_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkRenderingAreaInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentFormats: *const VkFormat,
    pub depthAttachmentFormat: VkFormat,
    pub stencilAttachmentFormat: VkFormat,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkImageSubresource2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub imageSubresource: VkImageSubresource,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_IMAGE_SUBRESOURCE_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkDeviceImageSubresourceInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pCreateInfo: *const VkImageCreateInfo,
    pub pSubresource: *const VkImageSubresource2KHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanSinkStructure)]
#[VulkanSinkStructure(type = VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkSubresourceLayout2KHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub subresourceLayout: VkSubresourceLayout,
}
impl VkSubresourceLayout2KHR {
    pub fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let x = p.as_mut_ptr();
            core::ptr::addr_of_mut!((*x).sType).write(Self::TYPE);
            core::ptr::addr_of_mut!((*x).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkPipelineCreateFlags2CreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineCreateFlags2KHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR)]
#[promote_1_4(suffix = "KHR")]
pub struct VkBufferUsageFlags2CreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub usage: VkBufferUsageFlags2KHR,
}

vk_ext_command!(
    pub fn vkCmdBindIndexBuffer2KHR(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, size: VkDeviceSize, indexType: VkIndexType);
    suffix = "KHR";
    promote = "1.4";
);
vk_ext_command!(
    pub fn vkGetRenderingAreaGranularityKHR(device: VkDevice, pRenderingAreaInfo: *const VkRenderingAreaInfoKHR, pGranularity: *mut VkExtent2D);
    suffix = "KHR";
    promote = "1.4";
);
vk_ext_command!(
    pub fn vkGetDeviceImageSubresourceLayoutKHR(device: VkDevice, pInfo: *const VkDeviceImageSubresourceInfoKHR, pLayout: *mut VkSubresourceLayout2KHR);
    suffix = "KHR";
    promote = "1.4";
);
vk_ext_command!(
    pub fn vkGetImageSubresourceLayout2KHR(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource2KHR, pLayout: *mut VkSubresourceLayout2KHR);
    suffix = "KHR";
    promote = "1.4";
);
