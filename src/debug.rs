//! Vulkan Debug Layer Extensions

#[cfg(feature = "Implements")]
use crate::Instance;
use crate::VkResultBox;
#[allow(unused_imports)]
use crate::{vk::*, VulkanStructure};
#[allow(unused_imports)]
use crate::{InstanceChild, VkHandle, VkObject};
#[allow(unused_imports)]
use derives::*;

/// Opaque object to a debug report callback object
#[derive(VkHandle)]
#[cfg(feature = "VK_EXT_debug_report")]
pub struct DebugReportCallbackObject<Instance: crate::Instance>(
    pub(crate) VkDebugReportCallbackEXT,
    pub(crate) Instance,
    pub(crate) PFN_vkDestroyDebugReportCallbackEXT,
);
#[cfg(feature = "VK_EXT_debug_report")]
impl<Instance: crate::Instance> VkObject for DebugReportCallbackObject<Instance> {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT;
}
#[cfg(feature = "VK_EXT_debug_report")]
unsafe impl<Instance: crate::Instance + Sync> Sync for DebugReportCallbackObject<Instance> {}
#[cfg(feature = "VK_EXT_debug_report")]
unsafe impl<Instance: crate::Instance + Send> Send for DebugReportCallbackObject<Instance> {}
#[cfg(feature = "VK_EXT_debug_report")]
impl<Instance: crate::Instance> InstanceChild for DebugReportCallbackObject<Instance> {
    type ConcreteInstance = Instance;

    fn instance(&self) -> &Self::ConcreteInstance {
        &self.1
    }
}
#[cfg(all(feature = "VK_EXT_debug_report", feature = "Implements"))]
impl<Instance: crate::Instance> Drop for DebugReportCallbackObject<Instance> {
    fn drop(&mut self) {
        (self.2)(self.1.native_ptr(), self.native_ptr(), std::ptr::null());
    }
}
#[cfg(feature = "VK_EXT_debug_report")]
impl<Instance: crate::Instance> DebugReportCallback for DebugReportCallbackObject<Instance> {}

#[cfg(feature = "VK_EXT_debug_report")]
pub struct DebugReportCallbackBuilder<Instance: crate::Instance> {
    #[cfg_attr(not(feature = "Implements"), allow(dead_code))]
    instance: Instance,
    flags: VkDebugReportFlagsEXT,
    #[cfg_attr(not(feature = "Implements"), allow(dead_code))]
    callback: PFN_vkDebugReportCallbackEXT,
}
#[cfg(feature = "VK_EXT_debug_report")]
impl<Instance: crate::Instance> DebugReportCallbackBuilder<Instance> {
    /// Create a builder object of DebugReportCallbackBuilder from `instance`, called back to `callback`
    pub fn new(instance: Instance, callback: PFN_vkDebugReportCallbackEXT) -> Self {
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
    pub fn create(self) -> crate::Result<DebugReportCallbackObject<Instance>> {
        self.instance
            .new_debug_report_callback(self.flags, self.callback, None::<&mut ()>)
    }
}

#[cfg(feature = "VK_EXT_debug_report")]
pub trait DebugReportCallback: VkHandle<Handle = VkDebugReportCallbackEXT> + InstanceChild {}
#[cfg(feature = "VK_EXT_debug_report")]
DerefContainerBracketImpl!(for DebugReportCallback {});

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
pub type DebugUtilsMessengerCreateInfo = VkDebugUtilsMessengerCreateInfoEXT;

#[cfg(feature = "VK_EXT_debug_utils")]
#[derive(VkHandle)]
pub struct DebugUtilsMessengerObject<Instance: crate::Instance>(
    VkDebugUtilsMessengerEXT,
    Instance,
    PFN_vkDestroyDebugUtilsMessengerEXT,
);
#[cfg(feature = "VK_EXT_debug_utils")]
impl<Instance: crate::Instance> VkObject for DebugUtilsMessengerObject<Instance> {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT;
}
#[cfg(feature = "VK_EXT_debug_utils")]
unsafe impl<Instance: crate::Instance + Sync> Sync for DebugUtilsMessengerObject<Instance> {}
#[cfg(feature = "VK_EXT_debug_utils")]
unsafe impl<Instance: crate::Instance + Send> Send for DebugUtilsMessengerObject<Instance> {}
#[cfg(feature = "VK_EXT_debug_utils")]
impl<Instance: crate::Instance> InstanceChild for DebugUtilsMessengerObject<Instance> {
    type ConcreteInstance = Instance;

    fn instance(&self) -> &Self::ConcreteInstance {
        &self.1
    }
}
#[cfg(all(feature = "VK_EXT_debug_utils", feature = "Implements"))]
impl<Instance: crate::Instance> Drop for DebugUtilsMessengerObject<Instance> {
    fn drop(&mut self) {
        (self.2)(self.1.native_ptr(), self.native_ptr(), std::ptr::null());
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
impl<Instance: crate::Instance> DebugUtilsMessenger for DebugUtilsMessengerObject<Instance> {}

#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebugUtilsMessageSeverityFlags(VkDebugUtilsMessageSeverityFlagsEXT);
#[cfg(feature = "VK_EXT_debug_utils")]
impl DebugUtilsMessageSeverityFlags {
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
pub enum DebugUtilsMessageSeverityFlag {
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
pub struct DebugUtilsMessageTypeFlags(VkDebugUtilsMessageTypeFlagsEXT);
#[cfg(feature = "VK_EXT_debug_utils")]
impl DebugUtilsMessageTypeFlags {
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
impl DebugUtilsMessengerCreateInfo {
    pub fn new(callback: PFN_vkDebugUtilsMessengerCallbackEXT) -> Self {
        Self {
            sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            pNext: std::ptr::null(),
            flags: 0,
            messageSeverity: DebugUtilsMessageSeverityFlags::ALL.0,
            messageType: DebugUtilsMessageTypeFlags::ALL.0,
            pfnUserCallback: callback,
            pUserData: std::ptr::null_mut(),
        }
    }
    pub fn filter_severity(mut self, severity: DebugUtilsMessageSeverityFlags) -> Self {
        self.messageSeverity = severity.0;
        self
    }
    pub fn filter_type(mut self, ty: DebugUtilsMessageTypeFlags) -> Self {
        self.messageType = ty.0;
        self
    }
    pub fn user_data<T>(mut self, p: *mut T) -> Self {
        self.pUserData = p as _;
        self
    }

    #[cfg(feature = "Implements")]
    /// Create a debug messenger object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    pub fn create<Instance: crate::Instance>(
        &self,
        instance: Instance,
    ) -> super::Result<DebugUtilsMessengerObject<Instance>> {
        let create_fn: PFN_vkCreateDebugUtilsMessengerEXT = instance
            .extra_procedure("vkCreateDebugUtilsMessengerEXT")
            .expect("Requiring vkCreateDebugUtilsMessengerEXT function");
        let destroy_fn: PFN_vkDestroyDebugUtilsMessengerEXT = instance
            .extra_procedure("vkDestroyDebugUtilsMessengerEXT")
            .expect("Requiring vkDestroyDebugUtilsMessengerEXT function");

        let mut h = VK_NULL_HANDLE as _;
        VkResultBox(create_fn(instance.native_ptr(), self, std::ptr::null(), &mut h))
            .into_result()
            .map(|_| DebugUtilsMessengerObject(h, instance, destroy_fn))
    }
}

#[cfg(feature = "VK_EXT_debug_utils")]
pub trait DebugUtilsMessenger: VkHandle<Handle = VkDebugUtilsMessengerEXT> + InstanceChild {}
#[cfg(feature = "VK_EXT_debug_utils")]
DerefContainerBracketImpl!(for DebugUtilsMessenger {});

#[cfg(feature = "VK_EXT_debug_utils")]
/// thin pointer to generic handle(u64) conversion helper
pub unsafe trait PointerHandleConversion {
    fn conv(self) -> u64;
}
#[cfg(feature = "VK_EXT_debug_utils")]
unsafe impl<T> PointerHandleConversion for *const T {
    fn conv(self) -> u64 {
        self as usize as _
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
unsafe impl<T> PointerHandleConversion for *mut T {
    fn conv(self) -> u64 {
        self as usize as _
    }
}

#[cfg(feature = "VK_EXT_debug_utils")]
#[repr(transparent)]
pub struct DebugUtilsObjectNameInfo<'d>(
    VkDebugUtilsObjectNameInfoEXT,
    std::marker::PhantomData<Option<&'d std::ffi::CStr>>,
);
#[cfg(feature = "VK_EXT_debug_utils")]
impl<'d> DebugUtilsObjectNameInfo<'d> {
    pub fn new<H: VkHandle + VkObject + ?Sized>(handle: &H, name: Option<&'d std::ffi::CStr>) -> Self
    where
        H::Handle: PointerHandleConversion,
    {
        Self::new_raw(H::TYPE, handle.native_ptr().conv(), name)
    }
    pub fn new_raw(ty: VkObjectType, handle: u64, name: Option<&'d std::ffi::CStr>) -> Self {
        DebugUtilsObjectNameInfo(
            VkDebugUtilsObjectNameInfoEXT {
                sType: VkDebugUtilsObjectNameInfoEXT::TYPE,
                pNext: std::ptr::null(),
                objectType: ty,
                objectHandle: handle,
                pObjectName: name.map_or_else(std::ptr::null, |s| s.as_ptr()),
            },
            std::marker::PhantomData,
        )
    }

    #[cfg(feature = "Implements")]
    /// Give a user-friendly name to an object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn apply(&self, device: &(impl crate::Device + crate::InstanceChild)) -> crate::Result<()> {
        let name_setter: PFN_vkSetDebugUtilsObjectNameEXT = device
            .instance()
            .extra_procedure("vkSetDebugUtilsObjectNameEXT")
            .expect("no vkSetDebugUtilsObjectNameEXT found");
        VkResultBox(name_setter(device.native_ptr(), &self.0))
            .into_result()
            .map(drop)
    }
}
