//! Vulkan Base Objects(Instance/PhysicalDevice)

#![cfg_attr(not(feature = "Implements"), allow(dead_code))]

use vk::*;
use std::ffi::CString;
use VkHandle;
#[cfg(feature = "Implements")] use ::vkresolve::{Resolver, ResolverInterface};
#[cfg(feature = "Implements")] use VkResultHandler;
#[cfg(feature = "Implements")] use std::ptr::{null, null_mut};
#[cfg(feature = "Implements")] use std::mem::MaybeUninit;
#[cfg(feature = "Implements")] use crate::fnconv::FnTransmute;
#[cfg(    feature = "Multithreaded") ] use std::sync::Arc as RefCounter;
#[cfg(not(feature = "Multithreaded"))] use std::rc::Rc as RefCounter;
#[cfg(    feature = "Multithreaded") ] use std::sync::RwLock as InternallyMutable;
#[cfg(not(feature = "Multithreaded"))] use std::cell::RefCell as InternallyMutable;

#[cfg(feature = "Multithreaded")] struct LazyCellReadRef<'d, T>(::std::sync::RwLockReadGuard<'d, Option<T>>);
#[cfg(feature = "Multithreaded")] impl<'d, T> ::std::ops::Deref for LazyCellReadRef<'d, T>
{
	type Target = T;
	fn deref(&self) -> &T { self.0.as_ref().unwrap() }
}

struct LazyCell<T>(InternallyMutable<Option<T>>);
impl<T> LazyCell<T>
{
	pub fn new() -> Self { LazyCell(From::from(None)) }
	#[cfg(not(feature = "Multithreaded"))]
	pub fn get<F: FnOnce() -> T>(&self, initializer: F) -> ::std::cell::Ref<T>
	{
		if self.0.borrow().is_none() { *self.0.borrow_mut() = Some(initializer()); }
		::std::cell::Ref::map(self.0.borrow(), |o| o.as_ref().unwrap())
	}
	#[cfg(feature = "Multithreaded")]
	pub fn get<F: FnOnce() -> T>(&self, initializer: F) -> LazyCellReadRef<T>
	{
		if self.0.read().expect("Poisoned").is_none() { *self.0.write().expect("Poisoned") = Some(initializer()); }
		LazyCellReadRef(self.0.read().expect("Poisoned"))
	}
}

struct InstanceCell
{
	n: VkInstance, vk_create_descriptor_update_template: LazyCell<PFN_vkCreateDescriptorUpdateTemplate>,
	vk_destroy_descriptor_update_template: LazyCell<PFN_vkDestroyDescriptorUpdateTemplate>
}
/// Opaque handle to a instance object
#[derive(Clone)] pub struct Instance(RefCounter<InstanceCell>);
#[cfg(feature = "Multithreaded")] unsafe impl Sync for Instance {}
/// Opaque handle to a physical device object
/// 
/// ## Platform Dependent Methods: Presentation Support checking functions
/// 
/// * `xlib_presentation_support(&self, queue_family: u32, display: *mut x11::xlib::Display, visual: x11::xlib::VisualID) -> bool`: VK_KHR_xlib_surface
/// * `xcb_presentation_support(&self, queue_family: u32, connection: *mut xcb::ffi::xcb_connection_t, visual: xcb::ffi::xcb_visualid_t) -> bool`: VK_KHR_xcb_surface
/// * `wayland_presentation_support(&self, queue_family: u32, display: *mut wayland_client::sys::wl_display) -> bool`: VK_KHR_wayland_surface
/// * `win32_presentation_support(&self, queue_family: u32) -> bool`: VK_KHR_win32_surface
/// * Methods for Android and Mir surfaces are not implemented
pub struct PhysicalDevice(VkPhysicalDevice, Instance);

pub struct IterPhysicalDevices<'i>(Vec<VkPhysicalDevice>, usize, &'i Instance);
impl<'i> Iterator for IterPhysicalDevices<'i>
{
	type Item = PhysicalDevice;

	fn next(&mut self) -> Option<PhysicalDevice>
	{
		if self.0.len() <= self.1 { None }
		else { self.1 += 1; Some(PhysicalDevice(self.0[self.1 - 1], self.2.clone())) }
	}
	fn size_hint(&self) -> (usize, Option<usize>) { (self.0.len(), Some(self.0.len())) }
}
impl<'i> ExactSizeIterator for IterPhysicalDevices<'i>
{
	fn len(&self) -> usize { self.0.len() }
	// fn is_empty(&self) -> bool { self.0.len() <= self.1 }
}
impl<'i> DoubleEndedIterator for IterPhysicalDevices<'i>
{
	fn next_back(&mut self) -> Option<PhysicalDevice>
	{
		if self.0.len() <= self.1 { None } else { self.0.pop().map(|p| PhysicalDevice(p, self.2.clone())) }
	}
}

#[cfg(feature = "Implements")]
impl Drop for InstanceCell { fn drop(&mut self) { unsafe { Resolver::get().destroy_instance(self.n, ::std::ptr::null()); } } }

impl VkHandle for Instance { type Handle = VkInstance; fn native_ptr(&self) -> VkInstance { self.0.n } }
impl VkHandle for PhysicalDevice { type Handle = VkPhysicalDevice; fn native_ptr(&self) -> VkPhysicalDevice { self.0 } }

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
	pub fn add_extension(&mut self, extension: &str) -> &mut Self
	{
		self.extensions.push(CString::new(extension).unwrap()); self
	}
	pub fn add_extension_zerotermed(&mut self, extension: &str) -> &mut Self
	{
		self.extensions.push(unsafe { ::std::ffi::CStr::from_ptr(extension.as_ptr() as *const _) }.to_owned()); self
	}
	pub fn add_extensions<'s, Extensions: IntoIterator<Item = &'s str>>(&mut self, extensions: Extensions) -> &mut Self
	{
		for ex in extensions { self.add_extension(ex); } self
	}
	pub fn add_layer(&mut self, layer: &str) -> &mut Self
	{
		self.layers.push(CString::new(layer).unwrap()); self
	}
	pub fn add_layers<'s, Layers: IntoIterator<Item = &'s str>>(&mut self, layers: Layers) -> &mut Self
	{
		for l in layers { self.add_layer(l); } self
	}
	/// [feature = "Implements"] Create a new Vulkan instance
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_INITIALIZATION_FAILED`
	/// * `VK_ERROR_LAYER_NOT_PRESENT`
	/// * `VK_ERROR_EXTENSION_NOT_PRESENT`
	/// * `VK_ERROR_INCOMPATIBLE_DRIVER`
	#[cfg(feature = "Implements")]
	pub fn create(&mut self) -> ::Result<Instance>
	{
		let layers: Vec<_> = self.layers.iter().map(|x| x.as_ptr()).collect();
		let extensions: Vec<_> = self.extensions.iter().map(|x| x.as_ptr()).collect();
		self.appinfo.pApplicationName = self.app_name.as_ptr(); self.appinfo.pEngineName = self.engine_name.as_ptr();
		self.cinfo.enabledLayerCount = layers.len() as _; self.cinfo.enabledExtensionCount = extensions.len() as _;
		self.cinfo.ppEnabledLayerNames = if layers.is_empty() { 0 as _ } else { layers.as_ptr() };
		self.cinfo.ppEnabledExtensionNames = if extensions.is_empty() { 0 as _ } else { extensions.as_ptr() };
		self.cinfo.pApplicationInfo = &self.appinfo;
		let mut h = VK_NULL_HANDLE as _;
		Ok(unsafe { Resolver::get().create_instance(&self.cinfo, ::std::ptr::null(), &mut h) }.into_result().map(|_| Instance(RefCounter::new(InstanceCell
		{
			n: h, vk_create_descriptor_update_template: LazyCell::new(), vk_destroy_descriptor_update_template: LazyCell::new()
		}))).unwrap())
	}
}
/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl Instance
{
	/// Return a function pointer for a command
	/// # Failures
	/// If function is not provided by instance or `name` is empty, returns `None`
	pub fn extra_procedure<F: ::fnconv::FnTransmute>(&self, name: &str) -> Option<F>
	{
		if name.is_empty() { return None; }

		unsafe
		{
			Resolver::get().get_instance_proc_addr(self.native_ptr(), CString::new(name).unwrap().as_ptr())
				.map(|f| FnTransmute::from_fn(f))
		}
	}
	/// Enumerates the physical devices accessible to a Vulkan instance
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_INITIALIZATION_FAILED`
	pub fn enumerate_physical_devices(&self) -> ::Result<Vec<PhysicalDevice>>
	{
		self.iter_physical_devices().map(|iter| iter.collect())
	}
	/// Lazyly enumerates the physical devices accessible to a Vulkan instance
	/// 
	/// # Failures
	/// 
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_INITIALIZATION_FAILED`
	pub fn iter_physical_devices(&self) -> ::Result<IterPhysicalDevices>
	{
		let mut n = 0;
		unsafe { Resolver::get().enumerate_physical_devices(self.native_ptr(), &mut n, null_mut()).into_result()?; }
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _); }
		unsafe { Resolver::get().enumerate_physical_devices(self.native_ptr(), &mut n, v.as_mut_ptr()).into_result()?; }
		Ok(IterPhysicalDevices(v, 0, self))
	}
	/// Returns up to all of global layer properties
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn enumerate_layer_properties() -> ::Result<Vec<VkLayerProperties>>
	{
		let mut n = 0;
		unsafe { Resolver::get().enumerate_instance_layer_properties(&mut n, null_mut()) }.into_result()?;
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { Resolver::get().enumerate_instance_layer_properties(&mut n, v.as_mut_ptr()) }.into_result().map(|_| v)
	}
	/// Returns up to all of global extension properties
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_LAYER_NOT_PRESENT`
	pub fn enumerate_extension_properties(layer_name: Option<&str>) -> ::Result<Vec<VkExtensionProperties>>
	{
		let cn = layer_name.map(|s| CString::new(s).unwrap());
		let cptr = cn.as_ref().map(|s| s.as_ptr()).unwrap_or(null());
		unsafe {
			let mut n = 0;
			Resolver::get().enumerate_instance_extension_properties(cptr, &mut n, null_mut()).into_result()?;
			let mut v = Vec::with_capacity(n as _); v.set_len(n as _);
			Resolver::get().enumerate_instance_extension_properties(cptr, &mut n, v.as_mut_ptr()).into_result()?;
			return Ok(v);
		}
	}
}
#[cfg(feature = "Implements")]
impl Instance
{
	pub(crate) unsafe fn create_descriptor_update_template(&self, device: VkDevice, info: &VkDescriptorUpdateTemplateCreateInfo,
		alloc: *const VkAllocationCallbacks, handle: &mut VkDescriptorUpdateTemplate) -> VkResult
	{
		let f = self.0.vk_create_descriptor_update_template
			.get(|| self.extra_procedure("vkCreateDescriptorUpdateTemplate").unwrap());
		f(device, info, alloc, handle)
	}
	pub(crate) unsafe fn destroy_descriptor_update_template(&self, device: VkDevice, handle: VkDescriptorUpdateTemplate,
		alloc: *const VkAllocationCallbacks)
	{
		let f = self.0.vk_destroy_descriptor_update_template
			.get(|| self.extra_procedure("vkDestroyDescriptorUpdateTemplate").unwrap());
		f(device, handle, alloc)
	}
}
/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl PhysicalDevice
{
	pub fn parent(&self) -> &Instance { &self.1 }
	/// Reports capabilities of a physical device.
	pub fn features(&self) -> VkPhysicalDeviceFeatures
	{
		unsafe {
			let mut p = MaybeUninit::uninit();
			Resolver::get().get_physical_device_features(self.0, p.as_mut_ptr());
			
			p.assume_init()
		}
	}
	/// Lists physical device's format capabilities
	pub fn format_properties(&self, format: VkFormat) -> VkFormatProperties
	{
		unsafe {
			let mut p = MaybeUninit::uninit();
			Resolver::get().get_physical_device_format_properties(self.0, format, p.as_mut_ptr());
			
			p.assume_init()
		}
	}
	/// Lists physical device's image format capabilities
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_FORMAT_NOT_SUPPORTED`
	pub fn image_format_properties(&self, format: VkFormat, itype: VkImageType, tiling: VkImageTiling,
		usage: ::ImageUsage, flags: ::ImageFlags) -> ::Result<VkImageFormatProperties>
	{
		unsafe {
			let mut p = MaybeUninit::uninit();
			Resolver::get().get_physical_device_image_format_properties(self.0, format, itype,
				tiling, usage.0, flags.0, p.as_mut_ptr()).into_result()?;
			
			Ok(p.assume_init())
		}
	}
	/// Returns properties of a physical device
	pub fn properties(&self) -> VkPhysicalDeviceProperties {
		unsafe {
			let mut p = MaybeUninit::uninit();
			Resolver::get().get_physical_device_properties(self.0, p.as_mut_ptr());
			
			p.assume_init()
		}
	}
	/// Reports properties of the queues of the specified physical device
	pub fn queue_family_properties(&self) -> ::QueueFamilies
	{
		unsafe {
			let mut n = 0;
			Resolver::get().get_physical_device_queue_family_properties(self.0, &mut n, null_mut());
			let mut v = ::preserve(n as _);
			Resolver::get().get_physical_device_queue_family_properties(self.0, &mut n, v.as_mut_ptr());
			return ::QueueFamilies(v);
		}
	}
	/// Reports memory information for the specified physical device
	pub fn memory_properties(&self) -> MemoryProperties
	{
		unsafe
		{
			let mut p = MaybeUninit::uninit();
			Resolver::get().get_physical_device_memory_properties(self.0, p.as_mut_ptr());
			return MemoryProperties(p.assume_init());
		}
	}
	/// Retrieve properties of an image format applied to sparse images
	pub fn sparse_image_format_properties(&self, format: VkFormat, itype: VkImageType, samples: VkSampleCountFlags,
		usage: ::ImageUsage, tiling: VkImageTiling) -> Vec<VkSparseImageFormatProperties>
	{
		unsafe
		{ 
			let mut n = 0;
			Resolver::get().get_physical_device_sparse_image_format_properties(self.0, format, itype, samples,
				usage.0, tiling, &mut n, null_mut());
			let mut v = ::preserve(n as _);
			Resolver::get().get_physical_device_sparse_image_format_properties(self.0, format, itype, samples,
				usage.0, tiling, &mut n, v.as_mut_ptr());
			return v;
		}
	}

	/// [feature = "VK_EXT_sample_locations"]
	#[cfg(feature = "VK_EXT_sample_locations")]
	pub fn multisample_properties(&self, samples: VkSampleCountFlags) -> VkMultisamplePropertiesEXT
	{
		unsafe {
			let mut r = MaybeUninit::uninit();
			Resolver::get().get_physical_device_multisample_properties_ext(self.0, samples, r.as_mut_ptr());
			
			r.assume_init()
		}
	}
}

/// [feature = "VK_KHR_surface" and feature = "Implements"] Surface functions
#[cfg(all(feature = "Implements", feature = "VK_KHR_surface"))]
impl PhysicalDevice
{
	/// Query if presentation is supported
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_SURFACE_LOST_KHR`
	pub fn surface_support(&self, queue_family: u32, surface: &::Surface) -> ::Result<bool> {
		let mut f = false as _;
		unsafe {
			Resolver::get().get_physical_device_surface_support_khr(self.0, queue_family, surface.native_ptr(), &mut f)
				.into_result()?;
			return Ok(f != 0);
		}
	}
	/// Query surface capabilities
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_SURFACE_LOST_KHR`
	pub fn surface_capabilities(&self, surface: &::Surface) -> ::Result<VkSurfaceCapabilitiesKHR>
	{
		let mut s = MaybeUninit::uninit();
		unsafe
		{
			Resolver::get()
				.get_physical_device_surface_capabilities_khr(self.0, surface.native_ptr(), s.as_mut_ptr())
				.into_result()
				.map(move |_| s.assume_init())
		}
	}
	/// Query color formats supported by surface
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_SURFACE_LOST_KHR`
	pub fn surface_formats(&self, surface: &::Surface) -> ::Result<Vec<VkSurfaceFormatKHR>>
	{
		unsafe
		{
			let mut n = 0;
			Resolver::get().get_physical_device_surface_formats_khr(self.0, surface.native_ptr(), &mut n, null_mut()).into_result()?;
			let mut v = ::preserve(n as _);
			Resolver::get().get_physical_device_surface_formats_khr(self.0, surface.native_ptr(), &mut n, v.as_mut_ptr())
				.into_result()
				.map(move |_| v)
		}
	}
	/// Query supported presentation modes
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_SURFACE_LOST_KHR`
	pub fn surface_present_modes(&self, surface: &::Surface) -> ::Result<Vec<::PresentMode>>
	{
		let mut n = 0;
		unsafe { Resolver::get().get_physical_device_surface_present_modes_khr(self.0, surface.native_ptr(), &mut n, ::std::ptr::null_mut()) }.into_result()?;
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { Resolver::get().get_physical_device_surface_present_modes_khr(self.0, surface.native_ptr(), &mut n, v.as_mut_ptr()) }.into_result()
			.map(|_| unsafe { ::std::mem::transmute(v) })
	}
	
	/// #[feature = "VK_KHR_get_surface_capabilities2"]
	#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
	pub fn surface_capabilities2(&self, info: &VkPhysicalDeviceSurfaceInfo2KHR) -> ::Result<VkSurfaceCapabilities2KHR>
	{
		let mut d = unsafe { std::mem::uninitialized() };
		unsafe
		{
			Resolver::get().get_physical_device_surface_capabilities2_khr(self.0, info as *const _, &mut d as *mut _)
				.into_result()?;
		}
		
		return Ok(d);
	}
}

/// [cfg(feature = "Implements")] Querying to the physical device for presentation with Platform servers.
#[cfg(feature = "Implements")]
impl PhysicalDevice {
	/// [feature = "VK_KHR_xlib_surface"] Query physical device for presentation to X11 server using Xlib
	#[cfg(feature = "VK_KHR_xlib_surface")]
	pub fn xlib_presentation_support(&self, queue_family: u32, display: *mut ::x11::xlib::Display, visual: ::x11::xlib::VisualID) -> bool {
		unsafe { Resolver::get().get_physical_device_xlib_presentation_support_khr(self.0, queue_family, display, visual) != 0 }
	}
	/// [feature = "VK_KHR_xcb_surface"] Query physical device for presentation to X11 server using XCB
	#[cfg(feature = "VK_KHR_xcb_surface")]
	pub fn xcb_presentation_support(&self, queue_family: u32, connection: *mut ::xcb::ffi::xcb_connection_t, visual: ::xcb::ffi::xcb_visualid_t) -> bool
	{
		unsafe { Resolver::get().get_physical_device_xcb_presentation_support_khr(self.0, queue_family, connection, visual) != 0 }
	}
	/// [feature = "VK_KHR_wayland_surface"] Query physical device for presentation to Wayland
	#[cfg(feature = "VK_KHR_wayland_surface")]
	pub fn wayland_presentation_support(&self, queue_family: u32, display: *mut ::wayland_client::sys::wl_display) -> bool
	{
		unsafe { Resolver::get().get_physical_device_wayland_presentation_support_khr(self.0, queue_family, display) != 0 }
	}
	/// [feature = "VK_KHR_win32_surface"] Query queue family support for presentation on a Win32 display
	#[cfg(feature = "VK_KHR_win32_surface")]
	pub fn win32_presentation_support(&self, queue_family: u32) -> bool
	{
		unsafe { Resolver::get().get_physical_device_win32_presentation_support_khr(self.0, queue_family) != 0 }
	}
}

/// feature = "VK_KHR_display" functions (required to enable the "Implements" feature)
#[cfg(all(feature = "VK_KHR_display", feature = "Implements"))]
impl PhysicalDevice
{
	/// Query information about the available displays
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn display_properties(&self) -> ::Result<Vec<VkDisplayPropertiesKHR>>
	{
		unsafe 
		{
			let mut n = 0;
			Resolver::get().get_physical_device_display_properties_khr(self.0, &mut n, null_mut()).into_result()?;
			let mut v = ::preserve(n as _);
			Resolver::get().get_physical_device_display_properties_khr(self.0, &mut n, v.as_mut_ptr())
				.into_result()
				.map(move |_| v)
		}
	}
	/// Query the plane properties
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn display_plane_properties(&self) -> ::Result<Vec<VkDisplayPlanePropertiesKHR>>
	{
		unsafe
		{
			let mut n = 0;
			Resolver::get().get_physical_device_display_plane_properties_khr(self.0, &mut n, null_mut()).into_result()?;
			let mut v = ::preserve(n as _);
			Resolver::get().get_physical_device_display_plane_properties_khr(self.0, &mut n, v.as_mut_ptr())
				.into_result()
				.map(move |_| v)
		}
	}
	/// Query the list of displays a plane supports
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn display_plane_supported_displays(&self, index: u32) -> ::Result<Vec<VkDisplayKHR>>
	{
		unsafe
		{
			let mut n = 0;
			Resolver::get().get_display_plane_supported_displays_khr(self.0, index, &mut n, null_mut()).into_result()?;
			let mut v = ::preserve(n as _);
			Resolver::get().get_display_plane_supported_displays_khr(self.0, index, &mut n, v.as_mut_ptr())
				.into_result()
				.map(move |_| v)
		}
	}
	/// Query the set of mode properties supported by the display
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn display_mode_properties(&self, display: VkDisplayKHR) -> ::Result<Vec<VkDisplayModePropertiesKHR>>
	{
		unsafe
		{
			let mut n = 0;
			Resolver::get().get_display_mode_properties_khr(self.0, display, &mut n, null_mut()).into_result()?;
			let mut v = ::preserve(n as _);
			Resolver::get().get_display_mode_properties_khr(self.0, display, &mut n, v.as_mut_ptr())
				.into_result()
				.map(move |_| v)
		}
	}
	/// Create a display mode
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_INITIALIZATION_FAILED`
	pub fn new_display_mode(&self, display: VkDisplayKHR, region: ::Extent2D, refresh_rate: u32) -> ::Result<VkDisplayModeKHR>
	{
		let cinfo = VkDisplayModeCreateInfoKHR
		{
			parameters: VkDisplayModeParametersKHR { visibleRegion: unsafe { ::std::mem::transmute(region) }, refreshRate: refresh_rate },
			.. Default::default()
		};
		let mut h = VK_NULL_HANDLE as _;
		unsafe
		{
			Resolver::get().create_display_mode_khr(self.0, display, &cinfo, ::std::ptr::null(), &mut h)
				.into_result()
				.map(move |_| h)
		}
	}
	/// Query capabilities of a mode and plane combination
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn display_plane_capabilities(&self, mode: VkDisplayModeKHR, plane_index: u32) -> ::Result<VkDisplayPlaneCapabilitiesKHR>
	{
		let mut s = MaybeUninit::uninit();
		unsafe
		{
			Resolver::get().get_display_plane_capabilities_khr(self.0, mode, plane_index, s.as_mut_ptr())
				.into_result()
				.map(move |_| s.assume_init())
		}
	}
}

/// Device memory properties
pub struct MemoryProperties(VkPhysicalDeviceMemoryProperties);
impl MemoryProperties
{
	pub fn find_type_index(&self, mask: MemoryPropertyFlags, exclude: MemoryPropertyFlags,
		index_mask: u32) -> Option<u32>
	{
		self.0.memoryTypes[..self.0.memoryTypeCount as usize].iter().enumerate()
			.filter(|(i, _)| (index_mask & (1u32 << i)) != 0)
			.filter(|(_, mt)| (mt.propertyFlags & mask.0) != 0 && (mt.propertyFlags & exclude.0) == 0)
			.map(|(x, _)| x as u32).next()
	}
	pub fn find_device_local_index(&self, index_mask: u32) -> Option<u32> {
		self.find_type_index(MemoryPropertyFlags::DEVICE_LOCAL, MemoryPropertyFlags::LAZILY_ALLOCATED, index_mask)
	}
	pub fn find_lazily_allocated_device_local_index(&self, index_mask: u32) -> Option<u32> {
		self.find_type_index(MemoryPropertyFlags::DEVICE_LOCAL.lazily_allocated(),
			MemoryPropertyFlags::EMPTY, index_mask)
	}
	pub fn find_host_visible_index(&self, index_mask: u32) -> Option<u32> {
		self.find_type_index(MemoryPropertyFlags::HOST_VISIBLE, MemoryPropertyFlags::EMPTY, index_mask)
	}
	pub fn is_coherent(&self, index: u32) -> bool {
		(self.0.memoryTypes[index as usize].propertyFlags & MemoryPropertyFlags::HOST_COHERENT.0) != 0
	}
	pub fn is_cached(&self, index: u32) -> bool {
		(self.0.memoryTypes[index as usize].propertyFlags & MemoryPropertyFlags::HOST_CACHED.0) != 0
	}

	pub fn types(&self) -> MemoryTypeIter { MemoryTypeIter(&self.0, 0) }
	pub fn heaps(&self) -> MemoryHeapIter { MemoryHeapIter(&self.0, 0) }
}
/// Iterating each elements of memory types
pub struct MemoryTypeIter<'d>(&'d VkPhysicalDeviceMemoryProperties, usize);
impl<'d> Iterator for MemoryTypeIter<'d>
{
	type Item = &'d VkMemoryType;
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.1 < self.0.memoryTypeCount as usize
		{
			let r = &self.0.memoryTypes[self.1]; self.1 += 1;
			Some(r)
		}
		else { None }
	}
}
/// Iterating each elements of memory heaps
pub struct MemoryHeapIter<'d>(&'d VkPhysicalDeviceMemoryProperties, usize);
impl<'d> Iterator for MemoryHeapIter<'d>
{
	type Item = &'d VkMemoryHeap;
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.1 < self.0.memoryHeapCount as usize
		{
			let r = &self.0.memoryHeaps[self.1]; self.1 += 1;
			Some(r)
		}
		else { None }
	}
}

/// Bitmask specifying properties for a memory type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryPropertyFlags(VkMemoryPropertyFlags);
impl MemoryPropertyFlags {
	/// Empty set
	pub const EMPTY: Self = MemoryPropertyFlags(0);
	/// Memory allocated with this type is the most efficient for device access
	pub const DEVICE_LOCAL: Self = MemoryPropertyFlags(VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
	/// Memory allocated with this type can be mapped for host access using `vkMapMemory`
	pub const HOST_VISIBLE: Self = MemoryPropertyFlags(VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT);
	/// The host cache management commands `vkFlushMappedmemoryRanges` and `vkInvalidateMappedMemoryRanges`
	/// are not needed to flush host writes to the device or make device writes visible to the host, respectively.
	pub const HOST_COHERENT: Self = MemoryPropertyFlags(VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
	/// Memory allocated with this type is cached on the host.
	/// Host memory accesses to uncached memory are slower than to cached memory, however uncached memory is always host coherent
	pub const HOST_CACHED: Self = MemoryPropertyFlags(VK_MEMORY_PROPERTY_HOST_CACHED_BIT);
	/// The memory type only allows device access to the memory.
	pub const LAZILY_ALLOCATED: Self = MemoryPropertyFlags(VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT);

	/// Memory allocated with this type is the most efficient for device access
	pub fn device_local(mut self) -> Self { self.0 |= Self::DEVICE_LOCAL.0; self }
	/// Memory allocated with this type can be mapped for host access using `vkMapMemory`
	pub fn host_visible(mut self) -> Self { self.0 |= Self::HOST_VISIBLE.0; self }
	/// The host cache management commands `vkFlushMappedmemoryRanges` and `vkInvalidateMappedMemoryRanges`
	/// are not needed to flush host writes to the device or make device writes visible to the host, respectively.
	pub fn host_coherent(mut self) -> Self { self.0 |= Self::HOST_COHERENT.0; self }
	/// Memory allocated with this type is cached on the host.
	/// Host memory accesses to uncached memory are slower than to cached memory, however uncached memory is always host coherent
	pub fn host_cached(mut self) -> Self { self.0 |= Self::HOST_CACHED.0; self }
	/// The memory type only allows device access to the memory.
	pub fn lazily_allocated(mut self) -> Self { self.0 |= Self::LAZILY_ALLOCATED.0; self }

	pub fn bits(self) -> VkMemoryPropertyFlags { self.0 }
}
