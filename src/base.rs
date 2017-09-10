//! Vulkan Base Objects(Instance/PhysicalDevice)

#![cfg_attr(not(feature = "FeImplements"), allow(dead_code))]

use vk::*;
use std::ffi::CString;
#[cfg(feature = "FeImplements")]
use VkResultHandler;
#[cfg(feature = "FeImplements")]
use std::mem::{uninitialized, zeroed, transmute};
#[cfg(feature = "FeImplements")]
use std::ptr::{null, null_mut};

/// Opaque handle to a instance object
pub struct Instance(VkInstance);
/// Opaque handle to a physical device object
pub struct PhysicalDevice(VkPhysicalDevice);
/// Builder object for constructing a `Instance`
pub struct InstanceBuilder
{
	app_name: CString, engine_name: CString, extensions: Vec<CString>, layers: Vec<CString>,
	appinfo: VkApplicationInfo, cinfo: VkInstanceCreateInfo
}
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
	#[cfg(feature = "FeImplements")]
	pub fn create(mut self) -> ::Result<Instance>
	{
		let (layers, extensions): (Vec<_>, Vec<_>) = (self.layers.iter().map(|x| x.as_ptr()).collect(), self.extensions.iter().map(|x| x.as_ptr()).collect());
		self.appinfo.pApplicationName = self.app_name.as_ptr(); self.appinfo.pEngineName = self.engine_name.as_ptr();
		self.cinfo.enabledLayerCount = layers.len() as _; self.cinfo.ppEnabledLayerNames = layers.as_ptr();
		self.cinfo.enabledExtensionCount = extensions.len() as _; self.cinfo.ppEnabledExtensionNames = extensions.as_ptr();
		self.cinfo.pApplicationInfo = &self.appinfo;
		let mut h = unsafe { zeroed() };
		unsafe { vkCreateInstance(&self.cinfo, null(), &mut h) }.into_result().map(|_| Instance(h))
	}
}
#[cfg(feature = "FeImplements")]
impl Drop for Instance { fn drop(&mut self) { unsafe { vkDestroyInstance(self.0, null()); } } }
#[cfg(feature = "FeImplements")]
impl Instance
{
	/// Return a function pointer for a command
	/// # Failures
	/// If function is not provided by instance or `name` is empty, returns `None`
	pub fn extra_procedure<F: ::fnconv::FnTransmute>(&self, name: &str) -> Option<F>
	{
		if name.is_empty() { None }
		else
		{
			let p = unsafe { vkGetInstanceProcAddr(self.0, CString::new(name).unwrap().as_ptr()) };
			if unsafe { transmute::<_, usize>(p) == 0 } { None } else { unsafe { Some(::fnconv::FnTransmute::from_fn(p)) } }
		}
	}
	/// Enumerates the physical devices accessible to a Vulkan instance
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_INITIALIZATION_FAILED
	pub fn enumerate_physical_devices(&self) -> ::Result<Vec<PhysicalDevice>>
	{
		let mut n = 0;
		unsafe { vkEnumeratePhysicalDevices(self.0, &mut n, null_mut()) }.into_result()?;
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { vkEnumeratePhysicalDevices(self.0, &mut n, v.as_mut_ptr()) }.into_result()
			.map(|_| unsafe { transmute(v) })
	}
	/// Returns up to all of global layer properties
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn enumerate_layer_properties() -> ::Result<Vec<VkLayerProperties>>
	{
		let mut n = 0;
		unsafe { vkEnumerateInstanceLayerProperties(&mut n, null_mut()) }.into_result()?;
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { vkEnumerateInstanceLayerProperties(&mut n, v.as_mut_ptr()) }.into_result().map(|_| v)
	}
	/// Returns up to all of global extension properties
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_LAYER_NOT_PRESENT
	pub fn enumerate_extension_properties(layer_name: &str) -> ::Result<Vec<VkExtensionProperties>>
	{
		let cn = CString::new(layer_name).unwrap();
		let mut n = 0;
		unsafe { vkEnumerateInstanceExtensionProperties(cn.as_ptr(), &mut n, null_mut()) }.into_result()?;
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
		let mut p = unsafe { uninitialized() };
		unsafe { vkGetPhysicalDeviceFeatures(self.0, &mut p) }; p
	}
	/// Lists physical device's format capabilities
	pub fn format_properties(&self, format: VkFormat) -> VkFormatProperties
	{
		let mut p = unsafe { uninitialized() };
		unsafe { vkGetPhysicalDeviceFormatProperties(self.0, format, &mut p) }; p
	}
	/// Lists physical device's image format capabilities
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_FORMAT_NOT_SUPPORTED
	pub fn image_format_properties(&self, format: VkFormat, itype: VkImageType, tiling: VkImageTiling,
		usage: ::ImageUsage, flags: ::ImageFlags) -> ::Result<VkImageFormatProperties>
	{
		let mut p = unsafe { uninitialized() };
		unsafe { vkGetPhysicalDeviceImageFormatProperties(self.0, format, itype, tiling, usage.0, flags.0, &mut p) }
			.into_result().map(|_| p)
	}
	/// Returns properties of a physical device
	pub fn properties(&self) -> VkPhysicalDeviceProperties
	{
		let mut p = unsafe { uninitialized() };
		unsafe { vkGetPhysicalDeviceProperties(self.0, &mut p) }; p
	}
	/// Reports properties of the queues of the specified physical device
	pub fn queue_family_properties(&self) -> ::QueueFamilies
	{
		let mut n = 0;
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(self.0, &mut n, null_mut()) };
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(self.0, &mut n, v.as_mut_ptr()) }; ::QueueFamilies(v)
	}
	/// Reports memory information for the specified physical device
	pub fn memory_properties(&self) -> VkPhysicalDeviceMemoryProperties
	{
		let mut p = unsafe { uninitialized() };
		unsafe { vkGetPhysicalDeviceMemoryProperties(self.0, &mut p) }; p
	}
	/// Retrieve properties of an image format applied to sparse images
	pub fn sparse_image_format_properties(&self, format: VkFormat, itype: VkImageType, samples: VkSampleCountFlags,
		usage: ::ImageUsage, tiling: VkImageTiling) -> Vec<VkSparseImageFormatProperties>
	{
		let mut n = 0;
		unsafe { vkGetPhysicalDeviceSparseImageFormatProperties(self.0, format, itype, samples, usage.0, tiling, &mut n, std::ptr::null_mut()) };
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { vkGetPhysicalDeviceSparseImageFormatProperties(self.0, format, itype, samples, usage.0, tiling, &mut n, v.as_mut_ptr()) };
		v
	}
}
