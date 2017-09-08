
extern crate libc;
// Platform Extras
#[cfg(any(
    feature = "VK_KHR_win32_surface", feature = "VK_KHR_external_memory_win32",
    feature = "VK_KHR_external_semaphore_win32", feature = "VK_KHR_external_fence_win32",
    feature = "VK_NV_external_memory_win32"
))]
extern crate winapi;
#[cfg(any(feature = "VK_KHR_xlib_surface", feature = "VK_EXT_acquire_xlib_display"))]
extern crate x11;

#[macro_use]
mod vk;
use vk::*;
use std::ffi::CString;

#[cfg(feature = "FeImplements")]
mod fnconv;

pub type Result<T> = std::result::Result<T, VkResult>;
pub trait VkResultHandler
{
	fn into_result(self) -> Result<()>;
}
impl VkResultHandler for VkResult
{
	fn into_result(self) -> Result<()> { if self == VK_SUCCESS { Ok(()) } else { Err(self) } }
}

pub struct Instance(VkInstance);
pub struct PhysicalDevice(VkPhysicalDevice);
pub struct InstanceBuilder
{
	app_name: CString, engine_name: CString, extensions: Vec<CString>, layers: Vec<CString>,
	appinfo: VkApplicationInfo, cinfo: VkInstanceCreateInfo
}
#[cfg(feature = "FeImplements")]
impl InstanceBuilder
{
	pub fn new(app_name: &str, app_version: (u32, u32, u32), engine_name: &str, engine_version: (u32, u32, u32)) -> Self
	{
		InstanceBuilder
		{
			app_name: CString::new(app_name).unwrap(), engine_name: CString::new(engine_name).unwrap(),
			extensions: Vec::new(), layers: Vec::new(), appinfo: VkApplicationInfo
			{
				applicationVersion: VK_MAKE_VERSION!(app_version.0, app_version.1, app_version.2),
				engineVersion: VK_MAKE_VERSION!(engine_version.0, engine_version.1, engine_version.2),
				.. Default::default()
			}, cinfo: VkInstanceCreateInfo { .. Default::default() }
		}
	}
	pub fn add_extension(mut self, extension: &str) -> Self
	{
		self.extensions.push(CString::new(extension).unwrap()); self
	}
	pub fn add_extensions<'s, Extensions: IntoIterator<Item = &'s str>>(self, extensions: Extensions) -> Self
	{
		let mut s = self;
		for ex in extensions { s = s.add_extension(ex); } s
	}
	pub fn add_layer(mut self, layer: &str) -> Self
	{
		self.layers.push(CString::new(layer).unwrap()); self
	}
	pub fn add_layers<'s, Layers: IntoIterator<Item = &'s str>>(self, layers: Layers) -> Self
	{
		let mut s = self;
		for l in layers { s = s.add_layer(l); } s
	}
	pub fn create(self) -> Result<Instance>
	{
		let app = VkApplicationInfo
		{
			pApplicationName: self.app_name.as_ptr(), pEngineName: self.engine_name.as_ptr(), .. self.appinfo
		};
		let (layers, extensions): (Vec<_>, Vec<_>) = (self.layers.into_iter().map(|x| x.as_ptr()).collect(), self.extensions.into_iter().map(|x| x.as_ptr()).collect());
		let cinfo = VkInstanceCreateInfo
		{
			enabledLayerCount: layers.len() as _, ppEnabledLayerNames: layers.as_ptr(),
			enabledExtensionCount: extensions.len() as _, ppEnabledExtensionNames: extensions.as_ptr(),
			pApplicationInfo: &app, .. self.cinfo
		};
		let mut h = unsafe { std::mem::zeroed() };
		unsafe { vkCreateInstance(&cinfo, std::ptr::null(), &mut h) }.into_result().map(|_| Instance(h))
	}
}
#[cfg(feature = "FeImplements")]
impl Drop for Instance { fn drop(&mut self) { unsafe { vkDestroyInstance(self.0, std::ptr::null()); } } }
#[cfg(feature = "FeImplements")]
impl Instance
{
	/// Return a function pointer for a command
	/// # Failures
	/// If function is not provided by instance or `name` is empty string, returns `None`
	pub fn extra_procedure<F: fnconv::FnTransmute>(&self, name: &str) -> Option<F>
	{
		if name.is_empty() { None }
		else
		{
			let p = unsafe { vkGetInstanceProcAddr(self.0, CString::new(name).unwrap().as_ptr()) };
			if unsafe { std::mem::transmute::<_, usize>(p) == 0 } { None } else { unsafe { Some(fnconv::FnTransmute::from_fn(p)) } }
		}
	}
	/// Enumerates the physical devices accessible to a Vulkan instance
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_INITIALIZATION_FAILED
	pub fn enumerate_physical_devices(&self) -> Result<Vec<PhysicalDevice>>
	{
		let mut n = 0;
		unsafe { vkEnumeratePhysicalDevices(self.0, &mut n, std::ptr::null_mut()) }.into_result()?;
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { vkEnumeratePhysicalDevices(self.0, &mut n, v.as_mut_ptr()) }.into_result()
			.map(|_| unsafe { std::mem::transmute(v) })
	}
	/// Returns up to all of global layer properties
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn enumerate_layer_properties() -> Result<Vec<VkLayerProperties>>
	{
		let mut n = 0;
		unsafe { vkEnumerateInstanceLayerProperties(&mut n, std::ptr::null_mut()) }.into_result()?;
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { vkEnumerateInstanceLayerProperties(&mut n, v.as_mut_ptr()) }.into_result().map(|_| v)
	}
	/// Returns up to all of global extension properties
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_LAYER_NOT_PRESENT
	pub fn enumerate_extension_properties(layer_name: &str) -> Result<Vec<VkExtensionProperties>>
	{
		let cn = CString::new(layer_name).unwrap();
		let mut n = 0;
		unsafe { vkEnumerateInstanceExtensionProperties(cn.as_ptr(), &mut n, std::ptr::null_mut()) }.into_result()?;
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { vkEnumerateInstanceExtensionProperties(cn.as_ptr(), &mut n, v.as_mut_ptr()) }.into_result().map(|_| v)
	}
}
#[cfg(feature = "FeImplements")]
impl PhysicalDevice
{
	/// Reports capabilities of a physical device.
	pub fn features(&self) -> VkPhysicalDeviceFeatures
	{
		let mut p = unsafe { std::mem::uninitialized() };
		unsafe { vkGetPhysicalDeviceFeatures(self.0, &mut p) }; p
	}
	/// Lists physical device's format capabilities
	pub fn format_properties(&self, format: VkFormat) -> VkFormatProperties
	{
		let mut p = unsafe { std::mem::uninitialize() };
		unsafe { vkGetPhysicalDeviceFormatProperties(self.0, format, &mut p) }; p
	}
	/// Lists physical device's image format capabilities
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_FORMAT_NOT_SUPPORTED
	pub fn image_format_properties(&self, format: VkFormat, itype: VkImageType, tiling: VkImageTiling,
		usage: VkImageUsageFlags, flags: VkImageCreateFlags) -> Result<VkImageFormatProperties>
	{
		let mut p = unsafe { std::mem::uninitialized() };
		unsafe { vkGetPhysicalDeviceImageFormatProperties(self.0, format, itype, tiling, usage, flags, &mut p) }
			.into_result().map(|_| p)
	}
	/// Returns properties of a physical device
	pub fn properties(&self) -> VkPhysicalDeviceProperties
	{
		let mut p = unsafe { std::mem::uninitialized() };
		unsafe { vkGetPhysicalDeviceProperties(self.0, &mut p) }; p
	}
	/// Reports properties of the queues of the specified physical device
	pub fn queue_family_properties(&self) -> Vec<VkQueueFamilyProperties>
	{
		let mut n = 0;
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(self.0, &mut n, std::ptr::null_mut()) };
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(self.0, &mut n, v.as_mut_ptr()) }; v
	}
	/// Reports memory information for the specified physical device
	pub fn memory_properties(&self) -> VkPhysicalDeviceMemoryProperties
	{
		let mut p = unsafe { std::mem::uninitialized() };
		unsafe { vkGetPhysicalDeviceMemoryProperties(self.0, &mut p) }; p
	}
}

