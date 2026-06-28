//! Vulkan Debug Layer Extensions
use bedrock_vk as brvk;

#[allow(unused_imports)]
use crate::*;

#[cfg(feature = "VK_EXT_debug_report")]
mod report;
#[cfg(feature = "VK_EXT_debug_report")]
pub use self::report::*;

#[cfg(feature = "VK_EXT_debug_utils")]
mod utils;
#[cfg(feature = "VK_EXT_debug_utils")]
pub use self::utils::*;

/// The type of an object passed to the `VkDebugMarkerObjectNameInfoEXT` and `VkDebugMarkerObjectTagInfoEXT` commands
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(feature = "VK_EXT_debug_report", feature = "VK_EXT_debug_marker"))]
pub enum DebugReportObjectType {
    /// An unknown object
    Unknown = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT,
    /// A `brvk::VkInstance`
    Instance = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT,
    /// A `brvk::VkPhysicalDevice`
    PhysicalDevice = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT,
    /// A `brvk::VkDevice`
    Device = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT,
    /// A `VkQueue`
    Queue = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT,
    /// A `VkSemaphore`
    Semaphore = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT,
    /// A `brvk::VkCommandBuffer`
    CommandBuffer = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT,
    /// A `brvk::VkFence`
    Fence = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT,
    /// A `brvk::VkDeviceMemory`
    DeviceMemory = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT,
    /// A `brvk::VkBuffer`
    Buffer = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT,
    /// A `VkImage`
    Image = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT,
    /// A `VkEvent`
    Event = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT,
    /// A `VkQueryPool`
    QueryPool = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT,
    /// A `brvk::VkBufferView`
    BufferView = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT,
    /// A `VkImageView`
    ImageView = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT,
    /// A `VkSHaderModule`
    ShaderModule = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT,
    /// A `VkPipeineCache`
    PipelineCache = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT,
    /// A `brvk::VkPipelineLayout`
    PipelineLayout = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT,
    /// A `VkRenderPass`
    RenderPass = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT,
    /// A `brvk::VkPipeline`
    Pipeline = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT,
    /// A `brvk::VkDescriptorSetLayout`
    DescriptorSetLayout = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT,
    /// A `VkSampler`
    Sampler = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT,
    /// A `brvk::VkDescriptorPool`
    DescriptorPool = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT,
    /// A `brvk::VkDescriptorSet`
    DescriptorSet = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT,
    /// A `VkFramebuffer`
    Framebuffer = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT,
    /// A `brvk::VkCommandPool`
    CommandPool = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT,
    /// A `VkSurfaceKHR`
    Surface = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT,
    /// A `VkSwapchainKHR`
    Swapchain = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT,
    /// A `VkDebugReportCallbackEXT`
    DebugReport = brvk::VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT,
}
