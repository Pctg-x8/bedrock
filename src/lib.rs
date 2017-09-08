
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

mod vk;
use vk::*;
use std::ffi::CString;

pub type Result<T> = std::result::Result<T, VkResult>;
pub trait VkResultHandler
{
	fn into_result(self) -> Result<()>;
}
impl VkResultHandler for VkResult
{
	fn into_result(self) -> Result<()> { if self == VK_SUCCESS { Ok(()) } else { Err(self) } }
}

pub struct InstanceBuilder
{
	app_name: CString, engine_name: CString, extensions: Vec<CString>, layers: Vec<CString>,
	appinfo: VkApplicationInfo, cinfo: VkInstanceCreateInfo
}
pub struct Instance(VkInstance);
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
	pub fn add_extensions<Extensions: IntoIterator<Item = &str>>(mut self, extensions: Extensions) -> Self
	{
		for ex in extensions { self.add_extension(ex) } self
	}
	pub fn add_layer(mut self, layer: &str) -> Self
	{
		self.layers.push(CString::new(layer).unwrap()); self
	}
	pub fn add_layers<Layers: IntoIterator<Item = &str>>(mut self, layers: Layers) -> Self
	{
		for l in layers { self.add_layer(l) } self
	}
	pub fn create(self) -> Result<Instance>
	{
		let app = VkApplicationInfo
		{
			pApplicationName: self.app_name.as_ptr(), pEngineName: self.engine_name.as_ptr(),
			.. self.appinfo
		};
		let (layers, extensions): (Vec<_>, Vec<_>) = (self.layers.into_iter().map(CString::as_ptr).collect(), self.extensions.into_iter().map(CString::as_ptr));
		let cinfo = VkInstanceCreateInfo
		{
			enabledLayerCount: layers.len() as _, ppEnabledLayers: layers.as_ptr(),
			enabledExtensionCount: extensions.len() as _, ppEnabledExtensions: extensions.as_ptr(),
			pApplicationInfo: &app, .. self.cinfo
		};
		let mut h = 0 as _;
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
	fn get_instance_proc_addr<F>(&self, name: &str) -> Option<F>
	{
		if name.is_empty() { None }
		else
		{
			let p = unsafe { vkGetInstanceProcAddr(self.0, CString::new(name).unwrap().as_ptr()) };
			if p.is_null() { None } else { unsafe { Some(std::mem::transmute(p)) } }
		}
	}
}

