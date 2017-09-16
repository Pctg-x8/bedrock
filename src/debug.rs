//! Vulkan Debug Layer Extensions

use vk::*;
use VkHandle;
#[cfg(feature = "FeImplements")] use VkResultHandler;

/// Opaque object to a debug report callback object
pub struct DebugReportCallback(VkDebugReportCallbackEXT, ::Instance);

#[cfg(feature = "FeImplements")]
impl Drop for DebugReportCallback
{
	fn drop(&mut self) { unsafe { vkDestroyDebugReportCallbackEXT(self.1.native_ptr(), self.native_ptr(), ::std::ptr::null()) }; }
}

impl VkHandle for DebugReportCallback { type Handle = VkDebugReportCallbackEXT; fn native_ptr(&self) -> VkDebugReportCallbackEXT { self.0 } }

#[cfg(feature = "FeImplements")]
impl DebugReportCallback
{
	/// Register a debug report callback
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	pub fn new<T>(instance: &::Instance, flags: DebugReportFlags, callback: PFN_vkDebugReportCallbackEXT, user_data: Option<&T>) -> ::Result<Self>
	{
		let s = VkDebugReportCallbackCreateInfoEXT
		{
			flags: flags.0, pfnCallback: callback, pUserData: user_data.map(|x| x as *const T as *mut _).unwrap_or(::std::ptr::null_mut()),
			.. Default::default()
		};
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateDebugReportCallbackEXT(instance.native_ptr(), &s, ::std::ptr::null(), &mut h) }.into_result()
			.map(|_| DebugReportCallback(h, instance.clone()))
	}
}
#[cfg(feature = "FeImplements")]
impl ::Instance
{
	/// Inject its own messages into the debug stream
	pub fn debug_message(&self, flags: DebugReportFlags, object_type: DebugReportObjectType, object: u64, location: ::libc::size_t,
		message_count: i32, layer_prefix: &str, message: &str)
	{
		let lp = ::std::ffi::CString::new(layer_prefix).unwrap();
		let msg = ::std::ffi::CString::new(message).unwrap();
		unsafe { vkDebugReportMessageEXT(self.native_ptr(), flags.0, object_type as _, object, location, message_count, lp.as_ptr(), msg.as_ptr()) };
	}
}

/// Indicates which events will cause this callback to be called
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebugReportFlags(pub VkDebugReportFlagsEXT);
impl DebugReportFlags
{
	/// An error that may cause undefined results, including an application crash
	pub const ERROR: Self = DebugReportFlags(VK_DEBUG_REPORT_ERROR_BIT_EXT);
	/// An unexpected use. e.g. Not destroying objects prior to destroying the containing object or potential inconsistencies between descriptor set layout
	/// and the layout in the corresponding shader, etc
	pub const WARNING: Self = DebugReportFlags(VK_DEBUG_REPORT_WARNING_BIT_EXT);
	/// A potentially non-optimal use of Vulkan. e.g. using `vkCmdClearColorImage` when a RenderPass load_op would have worked
	pub const PERFORMANCE_WARNING: Self = DebugReportFlags(VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT);
	/// An informational message such as resource details that may be handy when debugging an application
	pub const INFORMATION: Self = DebugReportFlags(VK_DEBUG_REPORT_INFORMATION_BIT_EXT);
	/// Diagnostic information from the loader and layers
	pub const DEBUG: Self = DebugReportFlags(VK_DEBUG_REPORT_DEBUG_BIT_EXT);

	/// An error that may cause undefined results, including an application crash
	pub fn error(&self) -> Self { DebugReportFlags(self.0 | Self::ERROR.0) }
	/// An unexpected use. e.g. Not destroying objects prior to destroying the containing object or potential inconsistencies between descriptor set layout
	/// and the layout in the corresponding shader, etc
	pub fn warning(&self) -> Self { DebugReportFlags(self.0 | Self::WARNING.0) }
	/// A potentially non-optimal use of Vulkan. e.g. using `vkCmdClearColorImage` when a RenderPass load_op would have worked
	pub fn performance_warning(&self) -> Self { DebugReportFlags(self.0 | Self::PERFORMANCE_WARNING.0) }
	/// An informational message such as resource details that may be handy when debugging an application
	pub fn information(&self) -> Self { DebugReportFlags(self.0 | Self::INFORMATION.0) }
	/// Diagnostic information from the loader and layers
	pub fn debug(&self) -> Self { DebugReportFlags(self.0 | Self::DEBUG.0) }
}

/// The type of an object passed to the `VkDebugMarkerObjectNameInfoEXT` and `VkDebugMarkerObjectTagInfoEXT` commands
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugReportObjectType
{
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
	DebugReport = VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT as _
}
