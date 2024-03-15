//! VK_KHR_synchronization2

use super::*;
use derives::promote_1_3;

pub const VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION: usize = 1;
pub const VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME: &str = "VK_KHR_synchronization2";

#[promote_1_3]
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR: VkStructureType = ext_enum_value(315, 0) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR: VkStructureType = ext_enum_value(315, 1) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR: VkStructureType = ext_enum_value(315, 2) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR: VkStructureType = ext_enum_value(315, 3) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR: VkStructureType = ext_enum_value(315, 4) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR: VkStructureType = ext_enum_value(315, 5) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR: VkStructureType = ext_enum_value(315, 6) as _;
#[promote_1_3]
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: VkStructureType =
    ext_enum_value(315, 7) as _;

#[promote_1_3(suffix = "KHR")]
pub type VkPipelineStageFlags2KHR = VkFlags64;
vk_bitmask! {
    #[promote_1_3(suffix = "KHR")]
    pub enum64 VkPipelineStageFlagBits2KHR {
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR: 0,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR: 1,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR: 2,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR: 3,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR: 4,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR: 5,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR: 6,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR: 7,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR: 8,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR: 9,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR: 10,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR: 11,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR: 12,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR: 13,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_HOST_BIT_KHR: 14,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR: 15,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR: 16,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_COPY_BIT_KHR: 32,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR: 33,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_BLIT_BIT_KHR: 34,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR: 35,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR: 36,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR: 37,
        #[promote_1_3]
        pub VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR: 38
    }
}
#[promote_1_3]
pub const VK_PIPELINE_STAGE_2_NONE_KHR: VkPipelineStageFlagBits2KHR = 0;

#[promote_1_3(suffix = "KHR")]
pub type VkAccessFlags2KHR = VkFlags64;
vk_bitmask! {
    #[promote_1_3(suffix = "KHR")]
    pub enum64 VkAccessFlagBits2KHR {
        #[promote_1_3]
        pub VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR: 0,
        #[promote_1_3]
        pub VK_ACCESS_2_INDEX_READ_BIT_KHR: 1,
        #[promote_1_3]
        pub VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR: 2,
        #[promote_1_3]
        pub VK_ACCESS_2_UNIFORM_READ_BIT_KHR: 3,
        #[promote_1_3]
        pub VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR: 4,
        #[promote_1_3]
        pub VK_ACCESS_2_SHADER_READ_BIT_KHR: 5,
        #[promote_1_3]
        pub VK_ACCESS_2_SHADER_WRITE_BIT_KHR: 6,
        #[promote_1_3]
        pub VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR: 7,
        #[promote_1_3]
        pub VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR: 8,
        #[promote_1_3]
        pub VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR: 9,
        #[promote_1_3]
        pub VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR: 10,
        #[promote_1_3]
        pub VK_ACCESS_2_TRANSFER_READ_BIT_KHR: 11,
        #[promote_1_3]
        pub VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR: 12,
        #[promote_1_3]
        pub VK_ACCESS_2_HOST_READ_BIT_KHR: 13,
        #[promote_1_3]
        pub VK_ACCESS_2_HOST_WRITE_BIT_KHR: 14,
        #[promote_1_3]
        pub VK_ACCESS_2_MEMORY_READ_BIT_KHR: 15,
        #[promote_1_3]
        pub VK_ACCESS_2_MEMORY_WRITE_BIT_KHR: 16,
        #[promote_1_3]
        pub VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR: 32,
        #[promote_1_3]
        pub VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR: 33,
        #[promote_1_3]
        pub VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR: 34,
    }
}
#[promote_1_3]
pub const VK_ACCESS_2_NONE: VkAccessFlagBits2KHR = 0;

#[promote_1_3(suffix = "KHR")]
pub type VkSubmitFlagsKHR = VkFlags;
vk_bitmask! {
    #[promote_1_3(suffix = "KHR")]
    pub enum VkSubmitFlagBitsKHR {
        #[promote_1_3]
        pub VK_SUBMIT_PROTECTED_BIT_KHR: 0
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkMemoryBarrier2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcStageMask: VkPipelineStageFlags2KHR,
    pub srcAccessMask: VkAccessFlags2KHR,
    pub dstStageMask: VkPipelineStageFlags2KHR,
    pub dstAccessMask: VkAccessFlags2KHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkBufferMemoryBarrier2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcStageMask: VkPipelineStageFlags2KHR,
    pub srcAccessMask: VkAccessFlags2KHR,
    pub dstStageMask: VkPipelineStageFlags2KHR,
    pub dstAccessMask: VkAccessFlags2KHR,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkImageMemoryBarrier2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcStageMask: VkPipelineStageFlags2KHR,
    pub srcAccessMask: VkAccessFlags2KHR,
    pub dstStageMask: VkPipelineStageFlags2KHR,
    pub dstAccessMask: VkAccessFlags2KHR,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkDependencyInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dependencyFlags: VkDependencyFlags,
    pub memoryBarrierCount: u32,
    pub pMemoryBarriers: *const VkMemoryBarrier2KHR,
    pub bufferMemoryBarrierCount: u32,
    pub pBufferMemoryBarriers: *const VkBufferMemoryBarrier2KHR,
    pub imageMemoryBarrierCount: u32,
    pub pImageMemoryBarriers: *const VkImageMemoryBarrier2KHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkSubmitInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSubmitFlagsKHR,
    pub waitSemaphoreInfoCount: u32,
    pub pWaitSemaphoreInfos: *const VkSemaphoreSubmitInfoKHR,
    pub commandBufferInfoCount: u32,
    pub pCommandBufferInfos: *const VkCommandBufferSubmitInfoKHR,
    pub signalSemaphoreInfoCount: u32,
    pub pSignalSemaphoreInfos: *const VkSemaphoreSubmitInfoKHR,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkSemaphoreSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub value: u64,
    pub stageMask: VkPipelineStageFlags2KHR,
    pub deviceIndex: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkCommandBufferSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub commandBuffer: VkCommandBuffer,
    pub deviceMask: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR)]
#[promote_1_3(suffix = "KHR")]
pub struct VkPhysicalDeviceSynchronization2FeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub synchronization2: VkBool32,
}

#[implements]
#[promote_1_3(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetEvent2KHR)]
pub struct PFN_vkCmdSetEvent2KHR(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        event: VkEvent,
        pDependencyInfo: *const VkDependencyInfoKHR,
    ),
);

#[implements]
#[promote_1_3(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdResetEvent2KHR)]
pub struct PFN_vkCmdResetEvent2KHR(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2KHR),
);

#[implements]
#[promote_1_3(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdWaitEvents2KHR)]
pub struct PFN_vkCmdWaitEvents2KHR(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        eventCount: u32,
        pEvents: *const VkEvent,
        pDependencyInfos: *const VkDependencyInfoKHR,
    ),
);

#[implements]
#[promote_1_3(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdPipelineBarrier2KHR)]
pub struct PFN_vkCmdPipelineBarrier2KHR(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfoKHR),
);

#[implements]
#[promote_1_3(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdWriteTimestamp2KHR)]
pub struct PFN_vkCmdWriteTimestamp2KHR(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        stage: VkPipelineStageFlags2KHR,
        queryPool: VkQueryPool,
        query: u32,
    ),
);

#[implements]
#[promote_1_3(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, StaticCallable, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkQueueSubmit2KHR)]
pub struct PFN_vkQueueSubmit2KHR(
    pub  unsafe extern "system" fn(
        queue: VkQueue,
        submitCount: u32,
        pSubmits: *const VkSubmitInfo2KHR,
        fence: VkFence,
    ) -> VkResult,
);

#[implements]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    #[promote_1_3(suffix = "KHR")]
    pub fn vkCmdSetEvent2KHR(
        commandBuffer: VkCommandBuffer,
        event: VkEvent,
        pDependencyInfo: *const VkDependencyInfoKHR,
    );
    #[promote_1_3(suffix = "KHR")]
    pub fn vkCmdResetEvent2KHR(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2KHR);
    #[promote_1_3(suffix = "KHR")]
    pub fn vkCmdWaitEvents2KHR(
        commandBuffer: VkCommandBuffer,
        eventCount: u32,
        pEvents: *const VkEvent,
        pDependencyInfos: *const VkDependencyInfoKHR,
    );
    #[promote_1_3(suffix = "KHR")]
    pub fn vkCmdPipelineBarrier2KHR(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfoKHR);
    #[promote_1_3(suffix = "KHR")]
    pub fn vkCmdWriteTimestamp2KHR(
        commandBuffer: VkCommandBuffer,
        stage: VkPipelineStageFlags2KHR,
        queryPool: VkQueryPool,
        query: u32,
    );
    #[promote_1_3(suffix = "KHR")]
    pub fn vkQueueSubmit2KHR(
        queue: VkQueue,
        submitCount: u32,
        pSubmits: *const VkSubmitInfo2KHR,
        fence: VkFence,
    ) -> VkResult;
}
