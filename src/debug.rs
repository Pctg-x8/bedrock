//! Vulkan Debug Layer Extensions

use crate::vk::*;
#[cfg(feature = "Implements")]
use crate::VkResultHandler;
use crate::{Instance, VkHandle};
use derives::*;

/// Opaque object to a debug report callback object
#[derive(VkHandle)]
#[cfg(feature = "VK_EXT_debug_report")]
pub struct DebugReportCallback(VkDebugReportCallbackEXT, Instance, PFN_vkDestroyDebugReportCallbackEXT);
#[cfg(all(feature = "VK_EXT_debug_report", feature = "Implements"))]
impl Drop for DebugReportCallback {
    fn drop(&mut self) {
        (self.2)(self.1.native_ptr(), self.native_ptr(), std::ptr::null());
    }
}

#[cfg(feature = "VK_EXT_debug_report")]
pub struct DebugReportCallbackBuilder<'i> {
    #[cfg_attr(not(feature = "Implements"), allow(dead_code))]
    instance: &'i Instance,
    flags: VkDebugReportFlagsEXT,
    #[cfg_attr(not(feature = "Implements"), allow(dead_code))]
    callback: PFN_vkDebugReportCallbackEXT,
}
#[cfg(feature = "VK_EXT_debug_report")]
impl<'i> DebugReportCallbackBuilder<'i> {
    /// Create a builder object of DebugReportCallbackBuilder from `instance`, called back to `callback`
    pub fn new(instance: &'i Instance, callback: PFN_vkDebugReportCallbackEXT) -> Self {
        DebugReportCallbackBuilder {
            instance,
            flags: 0,
            callback,
        }
    }
    /// Reports an error that may cause undefined results, including an application crash
    pub fn report_error(&mut self) -> &mut Self {
        self.flags |= VK_DEBUG_REPORT_ERROR_BIT_EXT;
        self
    }
    /// Reports an unexpected use. e.g. Not destroying objects prior to destroying the containing object or potential inconsistencies between descriptor set layout
    /// and the layout in the corresponding shader, etc
    pub fn report_warning(&mut self) -> &mut Self {
        self.flags |= VK_DEBUG_REPORT_WARNING_BIT_EXT;
        self
    }
    /// Reports a potentially non-optimal use of Vulkan. e.g. using `vkCmdClearColorImage` when a RenderPass load_op would have worked
    pub fn report_performance_warning(&mut self) -> &mut Self {
        self.flags |= VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT;
        self
    }
    /// Reports an informational message such as resource details that may be handy when debugging an application
    pub fn report_information(&mut self) -> &mut Self {
        self.flags |= VK_DEBUG_REPORT_INFORMATION_BIT_EXT;
        self
    }
    /// Reports diagnostic information from the loader and layers
    pub fn report_debug_information(&mut self) -> &mut Self {
        self.flags |= VK_DEBUG_REPORT_DEBUG_BIT_EXT;
        self
    }

    /// Register a debug report callback
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[cfg(feature = "Implements")]
    pub fn create(&mut self) -> crate::Result<DebugReportCallback> {
        DebugReportCallback::new(self.instance, self.flags, self.callback)
    }
}

#[cfg(all(feature = "VK_EXT_debug_report", feature = "Implements"))]
impl DebugReportCallback {
    /// Register a debug report callback
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    fn new(
        instance: &Instance,
        flags: VkDebugReportFlagsEXT,
        callback: PFN_vkDebugReportCallbackEXT,
    ) -> crate::Result<Self> {
        let ctor: PFN_vkCreateDebugReportCallbackEXT = instance
            .extra_procedure("vkCreateDebugReportCallbackEXT")
            .expect("Requiring vkCreateDebugReportCallbackEXT function");
        let dtor: PFN_vkDestroyDebugReportCallbackEXT = instance
            .extra_procedure("vkDestroyDebugReportCallbackEXT")
            .expect("Requiring vkDestroyDebugReportCallbackEXT function");
        let s = VkDebugReportCallbackCreateInfoEXT {
            flags,
            pfnCallback: callback,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        ctor(instance.native_ptr(), &s, std::ptr::null(), &mut h)
            .into_result()
            .map(|_| DebugReportCallback(h, instance.clone(), dtor))
    }
}
#[cfg(all(feature = "VK_EXT_debug_report", feature = "Implements"))]
impl Instance {
    /// Inject its own messages into the debug stream
    pub fn debug_message(
        &self,
        flags: VkDebugReportFlagsEXT,
        object_type: DebugReportObjectType,
        object: u64,
        location: libc::size_t,
        message_count: i32,
        layer_prefix: &str,
        message: &str,
    ) {
        let (lp, msg) = (
            std::ffi::CString::new(layer_prefix).unwrap(),
            std::ffi::CString::new(message).unwrap(),
        );
        let msgf: PFN_vkDebugReportMessageEXT = self
            .extra_procedure("vkDebugReportMessageEXT")
            .expect("Requiring vkDebugReportMessageEXT function");
        msgf(
            self.native_ptr(),
            flags,
            object_type as _,
            object,
            location,
            message_count,
            lp.as_ptr(),
            msg.as_ptr(),
        );
    }
}

/// The type of an object passed to the `VkDebugMarkerObjectNameInfoEXT` and `VkDebugMarkerObjectTagInfoEXT` commands
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(feature = "VK_EXT_debug_report", feature = "VK_EXT_debug_marker"))]
pub enum DebugReportObjectType {
    /// An unknown object
    Unknown = VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT as _,
    /// A `VkInstance`
    Instance = VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT as _,
    /// A `VkPhysicalDevice`
    PhysicalDevice = VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT as _,
    /// A `VkDevice`
    Device = VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT as _,
    /// A `VkQueue`
    Queue = VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT as _,
    /// A `VkSemaphore`
    Semaphore = VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT as _,
    /// A `VkCommandBuffer`
    CommandBuffer = VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT as _,
    /// A `VkFence`
    Fence = VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT as _,
    /// A `VkDeviceMemory`
    DeviceMemory = VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT as _,
    /// A `VkBuffer`
    Buffer = VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT as _,
    /// A `VkImage`
    Image = VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT as _,
    /// A `VkEvent`
    Event = VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT as _,
    /// A `VkQueryPool`
    QueryPool = VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT as _,
    /// A `VkBufferView`
    BufferView = VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT as _,
    /// A `VkImageView`
    ImageView = VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT as _,
    /// A `VkSHaderModule`
    ShaderModule = VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT as _,
    /// A `VkPipeineCache`
    PipelineCache = VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT as _,
    /// A `VkPipelineLayout`
    PipelineLayout = VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT as _,
    /// A `VkRenderPass`
    RenderPass = VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT as _,
    /// A `VkPipeline`
    Pipeline = VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT as _,
    /// A `VkDescriptorSetLayout`
    DescriptorSetLayout = VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT as _,
    /// A `VkSampler`
    Sampler = VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT as _,
    /// A `VkDescriptorPool`
    DescriptorPool = VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT as _,
    /// A `VkDescriptorSet`
    DescriptorSet = VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT as _,
    /// A `VkFramebuffer`
    Framebuffer = VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT as _,
    /// A `VkCommandPool`
    CommandPool = VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT as _,
    /// A `VkSurfaceKHR`
    Surface = VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT as _,
    /// A `VkSwapchainKHR`
    Swapchain = VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT as _,
    /// A `VkDebugReportCallbackEXT`
    DebugReport = VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT as _,
}

#[cfg(feature = "VK_EXT_debug_utils")]
pub type MessengerCreateInfo = VkDebugUtilsMessengerCreateInfoEXT;

#[cfg(feature = "VK_EXT_debug_utils")]
#[derive(VkHandle)]
pub struct Messenger(VkDebugUtilsMessengerEXT, Instance, PFN_vkDestroyDebugUtilsMessengerEXT);
#[cfg(all(feature = "VK_EXT_debug_utils", feature = "Implements"))]
impl Drop for Messenger {
    fn drop(&mut self) {
        (self.2)(self.1.native_ptr(), self.native_ptr(), std::ptr::null());
    }
}

#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageSeverityFlags(VkDebugUtilsMessageSeverityFlagsEXT);
#[cfg(feature = "VK_EXT_debug_utils")]
impl MessageSeverityFlags {
    /// Empty flag set
    pub const EMPTY: Self = Self(0);
    /// The most verbose output indicating all diagnostic messages
    /// from the Vulkan loader, layers, and drivers should be captured.
    pub const VERBOSE: Self = Self(VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT);
    /// An informational message such as resource details that may be handy when debugging an application.
    pub const INFO: Self = Self(VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT);
    /// Use of Vulkan that *may* expose an app bug.
    /// Such cases may not be immediately harmful, such as a fragment shader outptting to a location with no attachment.
    /// Other cases *may* point to behavior that is almost certainly bad when unintended
    /// such as using an image whose memory has not been filled.
    /// In general if you see a warning but you know that the behavior is intended/desired,
    /// then simply ignore the warning.
    pub const WARNING: Self = Self(VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT);
    /// The application has violated a valid usage condiiton of the specification.
    pub const ERROR: Self = Self(VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT);
    /// All flags set
    pub const ALL: Self = Self::VERBOSE.and(Self::INFO).and(Self::WARNING).and(Self::ERROR);

    /// The most verbose output indicating all diagnostic messages
    /// from the Vulkan loader, layers, and drivers should be captured.
    pub const fn and_verbose(self) -> Self {
        self.and(Self::VERBOSE)
    }
    /// An informational message such as resource details that may be handy when debugging an application.
    pub const fn and_info(self) -> Self {
        self.and(Self::INFO)
    }
    /// Use of Vulkan that *may* expose an app bug.
    /// Such cases may not be immediately harmful, such as a fragment shader outptting to a location with no attachment.
    /// Other cases *may* point to behavior that is almost certainly bad when unintended
    /// such as using an image whose memory has not been filled.
    /// In general if you see a warning but you know that the behavior is intended/desired,
    /// then simply ignore the warning.
    pub const fn and_warning(self) -> Self {
        self.and(Self::WARNING)
    }
    /// The application has violated a valid usage condiiton of the specification.
    pub const fn and_error(self) -> Self {
        self.and(Self::ERROR)
    }

    /// const fn version of `bitor`
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageSeverityFlag {
    /// The most verbose output indicating all diagnostic messages
    /// from the Vulkan loader, layers, and drivers should be captured.
    Verbose = VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT,
    /// An informational message such as resource details that may be handy when debugging an application.
    Info = VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT,
    /// Use of Vulkan that *may* expose an app bug.
    /// Such cases may not be immediately harmful, such as a fragment shader outptting to a location with no attachment.
    /// Other cases *may* point to behavior that is almost certainly bad when unintended
    /// such as using an image whose memory has not been filled.
    /// In general if you see a warning but you know that the behavior is intended/desired,
    /// then simply ignore the warning.
    Warning = VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT,
    /// The application has violated a valid usage condiiton of the specification.
    Error = VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
}

#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageTypeFlags(VkDebugUtilsMessageTypeFlagsEXT);
#[cfg(feature = "VK_EXT_debug_utils")]
impl MessageTypeFlags {
    /// Empty flag set
    pub const EMPTY: Self = Self(0);
    /// Some general event has occured.
    /// This is typically a non-specification, non-performance event.
    pub const GENERAL: Self = Self(VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT);
    /// Something has occured during validation against the Vulkan specification that may indicate invalid behavior.
    pub const VALIDATION: Self = Self(VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT);
    /// A potentially non-optimal use of Vulkan,
    /// e.g. using `vkCmdClearColorImage` when setting `VkAttachmentDescription::loadOp` to `VK_ATTACHMENT_LOAD_OP_CLEAR`
    /// would have worked.
    pub const PERFORMANCE: Self = Self(VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT);
    /// All flags set
    pub const ALL: Self = Self::GENERAL.and(Self::VALIDATION).and(Self::PERFORMANCE);

    /// Some general event has occured.
    /// This is typically a non-specification, non-performance event.
    pub const fn and_general(self) -> Self {
        self.and(Self::GENERAL)
    }
    /// Something has occured during validation against the Vulkan specification that may indicate invalid behavior.
    pub const fn and_validation(self) -> Self {
        self.and(Self::VALIDATION)
    }
    /// A potentially non-optimal use of Vulkan,
    /// e.g. using `vkCmdClearColorImage` when setting `VkAttachmentDescription::loadOp` to `VK_ATTACHMENT_LOAD_OP_CLEAR`
    /// would have worked.
    pub const fn and_performance(self) -> Self {
        self.and(Self::PERFORMANCE)
    }

    /// const fn version of `bitor`
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

#[cfg(feature = "VK_EXT_debug_utils")]
impl MessengerCreateInfo {
    pub fn new(callback: PFN_vkDebugUtilsMessengerCallbackEXT) -> Self {
        Self {
            sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            pNext: std::ptr::null(),
            flags: 0,
            messageSeverity: MessageSeverityFlags::ALL.0,
            messageType: MessageTypeFlags::ALL.0,
            pfnUserCallback: callback,
            pUserData: std::ptr::null_mut(),
        }
    }
    pub fn filter_severity(mut self, severity: MessageSeverityFlags) -> Self {
        self.messageSeverity = severity.0;
        self
    }
    pub fn filter_type(mut self, ty: MessageTypeFlags) -> Self {
        self.messageType = ty.0;
        self
    }
    pub fn user_data<T>(mut self, p: *mut T) -> Self {
        self.pUserData = p as _;
        self
    }

    #[cfg(feature = "Implements")]
    /// [Implements] Create a debug messenger object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    pub fn create(&self, instance: &Instance) -> super::Result<Messenger> {
        let create_fn: PFN_vkCreateDebugUtilsMessengerEXT = instance
            .extra_procedure("vkCreateDebugUtilsMessengerEXT")
            .expect("Requiring vkCreateDebugUtilsMessengerEXT function");
        let destroy_fn: PFN_vkDestroyDebugUtilsMessengerEXT = instance
            .extra_procedure("vkDestroyDebugUtilsMessengerEXT")
            .expect("Requiring vkDestroyDebugUtilsMessengerEXT function");

        let mut h = VK_NULL_HANDLE as _;
        create_fn(instance.native_ptr(), self, std::ptr::null(), &mut h)
            .into_result()
            .map(|_| Messenger(h, instance.clone(), destroy_fn))
    }
}
