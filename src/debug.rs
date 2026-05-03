//! Vulkan Debug Layer Extensions

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
    Unknown = VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT,
    /// A `VkInstance`
    Instance = VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT,
    /// A `VkPhysicalDevice`
    PhysicalDevice = VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT,
    /// A `VkDevice`
    Device = VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT,
    /// A `VkQueue`
    Queue = VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT,
    /// A `VkSemaphore`
    Semaphore = VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT,
    /// A `VkCommandBuffer`
    CommandBuffer = VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT,
    /// A `VkFence`
    Fence = VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT,
    /// A `VkDeviceMemory`
    DeviceMemory = VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT,
    /// A `VkBuffer`
    Buffer = VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT,
    /// A `VkImage`
    Image = VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT,
    /// A `VkEvent`
    Event = VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT,
    /// A `VkQueryPool`
    QueryPool = VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT,
    /// A `VkBufferView`
    BufferView = VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT,
    /// A `VkImageView`
    ImageView = VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT,
    /// A `VkSHaderModule`
    ShaderModule = VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT,
    /// A `VkPipeineCache`
    PipelineCache = VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT,
    /// A `VkPipelineLayout`
    PipelineLayout = VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT,
    /// A `VkRenderPass`
    RenderPass = VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT,
    /// A `VkPipeline`
    Pipeline = VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT,
    /// A `VkDescriptorSetLayout`
    DescriptorSetLayout = VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT,
    /// A `VkSampler`
    Sampler = VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT,
    /// A `VkDescriptorPool`
    DescriptorPool = VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT,
    /// A `VkDescriptorSet`
    DescriptorSet = VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT,
    /// A `VkFramebuffer`
    Framebuffer = VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT,
    /// A `VkCommandPool`
    CommandPool = VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT,
    /// A `VkSurfaceKHR`
    Surface = VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT,
    /// A `VkSwapchainKHR`
    Swapchain = VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT,
    /// A `VkDebugReportCallbackEXT`
    DebugReport = VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT,
}
