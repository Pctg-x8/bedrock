//! Vulkan Surface/Swapchain Extensions

use super::*;
use std::rc::Rc as RefCounter;
use {VkHandle, DeviceChild};
#[cfg(feature = "Implements")] use VkResultHandler;
#[cfg(feature = "Implements")] use std::ptr::null;
#[cfg(feature = "Implements")] use vkresolve::Resolver;

struct SurfaceCell(VkSurfaceKHR, Instance);
/// Opaque handle to a surface object
#[derive(Clone)] pub struct Surface(RefCounter<SurfaceCell>);
struct SwapchainCell
{
	obj: VkSwapchainKHR, dev: Device, _target: Surface, fmt: VkFormat, size: Extent3D
}
/// Opaque handle to a swapchain object
#[derive(Clone)] pub struct Swapchain(RefCounter<SwapchainCell>);

#[cfg(feature = "Implements")]
impl Drop for SurfaceCell
{
	fn drop(&mut self) { unsafe { Resolver::get().destroy_surface_khr(self.1.native_ptr(), self.0, null()); } }
}
#[cfg(feature = "Implements")]
impl Drop for SwapchainCell
{
	fn drop(&mut self) { unsafe { Resolver::get().destroy_swapchain_khr(self.dev.native_ptr(), self.obj, null()); } }
}
impl VkHandle for Surface { type Handle = VkSurfaceKHR; fn native_ptr(&self) -> VkSurfaceKHR { self.0 .0 } }
impl VkHandle for Swapchain { type Handle = VkSwapchainKHR; fn native_ptr(&self) -> VkSwapchainKHR { self.0.obj } }
impl DeviceChild for Swapchain { fn device(&self) -> &Device { &self.0.dev } }

impl Surface
{
	/// Create an surface object by taking raw `VkSurfaceKHR` object.
	/// # Safety
	/// `ptr` must be created from `parent`, and destroyed by this object(not yourself!).
	pub unsafe fn from_raw(ptr: VkSurfaceKHR, parent: &Instance) -> Self
	{
		Surface(SurfaceCell(ptr, parent.clone()).into())
	}
}

/// Creation Procedures
#[cfg(feature = "Implements")]
impl Surface
{
	/// Create a `Surface` object for an X11 window, using the Xlib client-side library
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	#[cfg(feature = "VK_KHR_xlib_surface")]
	pub fn new_xlib(instance: &Instance, display: *mut x11::xlib::Display, window: x11::xlib::Window) -> ::Result<Self>
	{
		let cinfo = VkXlibSurfaceCreateInfoKHR { dpy: display, window, .. Default::default() };
		let mut h = VK_NULL_HANDLE as _;
		unsafe { Resolver::get().create_xlib_surface_khr(instance.native_ptr(), &cinfo, null(), &mut h) }
			.into_result()
			.map(|_| Surface(SurfaceCell(h, instance.clone()).into()))
	}
	/// Create a `Surface` object for a X11 window, using the XCB client-side library
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	#[cfg(feature = "VK_KHR_xcb_surface")]
	pub fn new_xcb(instance: &Instance, connection: *mut xcb::ffi::xcb_connection_t, window: xcb::ffi::xcb_window_t)
		-> ::Result<Self>
	{
		let cinfo = VkXcbSurfaceCreateInfoKHR { connection, window, .. Default::default() };
		let mut h = VK_NULL_HANDLE as _;
		unsafe { Resolver::get().create_xcb_surface_khr(instance.native_ptr(), &cinfo, null(), &mut h) }
			.into_result()
			.map(|_| Surface(SurfaceCell(h, instance.clone()).into()))
	}
	/// Create a `Surface` object for a Wayland window
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	#[cfg(feature = "VK_KHR_wayland_surface")]
	pub fn new_wayland(instance: &Instance,
		display: *mut wayland_client::sys::wl_display, surface: *mut wayland_client::sys::wl_proxy)
		-> ::Result<Self>
	{
		let cinfo = VkWaylandSurfaceCreateInfoKHR { display, surface, .. Default::default() };
		let mut h = VK_NULL_HANDLE as _;
		unsafe { Resolver::get().create_wayland_surface_khr(instance.native_ptr(), &cinfo, null(), &mut h) }
			.into_result()
			.map(|_| Surface(SurfaceCell(h, instance.clone()).into()))
	}
	/// Create a `Surface` object for an Android native window
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	#[cfg(feature = "VK_KHR_android_surface")]
	pub fn new_android(instance: &Instance, window: *mut android::ANativeWindow) -> ::Result<Self>
	{
		let cinfo = VkAndroidSurfaceCreateInfoKHR { window, .. Default::default() };
		let mut h = VK_NULL_HANDLE as _;
		unsafe { Resolver::get().create_android_surface_khr(instance.native_ptr(), &cinfo, null(), &mut h) }
			.into_result()
			.map(|_| Surface(SurfaceCell(h, instance.clone()).into()))
	}
	/// Create a `Surface` object for an Win32 native window
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	#[cfg(feature = "VK_KHR_win32_surface")]
	pub fn new_win32(instance: &::Instance,
		hinstance: winapi::shared::minwindef::HINSTANCE, hwnd: winapi::shared::windef::HWND)
		-> ::Result<Self>
	{
		let cinfo = VkWin32SurfaceCreateInfoKHR { hinstance, hwnd, .. Default::default() };
		let mut h = VK_NULL_HANDLE as _;
		unsafe { Resolver::get().create_win32_surface_khr(instance.native_ptr(), &cinfo, null(), &mut h) }
			.into_result()
			.map(|_| Surface(SurfaceCell(h, instance.clone()).into()))
	}
	/// Create a `Surface` object for an macOS native window
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	#[cfg(feature = "VK_MVK_macos_surface")]
	pub fn new_macos(instance: &Instance, view_ptr: *const libc::c_void) -> ::Result<Self>
	{
		let cinfo = VkMacOSSurfaceCreateInfoMVK { pView: view_ptr, .. Default::default() };
		let mut h = VK_NULL_HANDLE as _;
		unsafe { Resolver::get().create_macos_surface_mvk(instance.native_ptr(), &cinfo, null(), &mut h) }
			.into_result()
			.map(|_| Surface(SurfaceCell(h, instance.clone()).into()))
	}
	/// Create a `Surface` object representing a display plane and mode
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	#[cfg(feature = "VK_KHR_display")]
	#[allow(clippy::too_many_arguments)]
	pub fn new_display_plane(instance: &Instance, mode: VkDisplayModeKHR, plane_index: u32, plane_stack_index: u32,
		transform: SurfaceTransform, global_alpha: f32, alpha_mode: DisplayPlaneAlpha, extent: Extent2D)
		-> ::Result<Self>
	{
		let cinfo = VkDisplaySurfaceCreateInfoKHR
		{
			displayMode: mode, planeIndex: plane_index, planeStackIndex: plane_stack_index,
			transform: transform as _, globalAlpha: global_alpha, alphaMode: alpha_mode as _,
			imageExtent: extent.into(),
			.. Default::default()
		};
		let mut h = VK_NULL_HANDLE as _;
		unsafe { Resolver::get().create_display_plane_surface_khr(instance.native_ptr(), &cinfo, null(), &mut h) }
			.into_result()
			.map(|_| Surface(SurfaceCell(h, instance.clone()).into()))
	}
}

/// Builder object to construct a `Swapchain`
pub struct SwapchainBuilder<'d>(VkSwapchainCreateInfoKHR, &'d Surface);
impl<'d> SwapchainBuilder<'d>
{
	pub fn new(surface: &'d Surface, min_image_count: u32, format: &VkSurfaceFormatKHR,
		extent: &Extent2D, usage: ImageUsage) -> Self
	{
		SwapchainBuilder(VkSwapchainCreateInfoKHR
		{
			surface: surface.native_ptr(), minImageCount: min_image_count, imageFormat: format.format,
			imageColorSpace: format.colorSpace, imageExtent: extent.clone().into(),
			imageArrayLayers: 1, imageUsage: usage.0, imageSharingMode: VK_SHARING_MODE_EXCLUSIVE,
			preTransform: SurfaceTransform::Inherit as _, compositeAlpha: CompositeAlpha::Inherit as _,
			presentMode: PresentMode::FIFO as _,
			.. Default::default()
		}, surface)
	}
	pub fn array_layers(&mut self, layers: u32) -> &mut Self { self.0.imageArrayLayers = layers; self }
	pub fn share(&mut self, queue_families: &[u32]) -> &mut Self
	{
		self.0.imageSharingMode =
			if queue_families.is_empty() { VK_SHARING_MODE_EXCLUSIVE } else { VK_SHARING_MODE_CONCURRENT };
		self.0.queueFamilyIndexCount = queue_families.len() as _;
		self.0.pQueueFamilyIndices = queue_families.as_ptr();
		self
	}
	pub fn pre_transform(&mut self, tf: SurfaceTransform) -> &mut Self { self.0.preTransform = tf as _; self }
	pub fn composite_alpha(&mut self, a: CompositeAlpha) -> &mut Self { self.0.compositeAlpha = a as _; self }
	pub fn present_mode(&mut self, mode: PresentMode) -> &mut Self { self.0.presentMode = mode as _; self }
	/// Enables whether the Vulkan implementation is allowed to discard rendering operations
	/// that affect regions of the surface which aren't visible
	pub fn enable_clip(&mut self) -> &mut Self { self.0.clipped = true as _; self }

	/// Create a swapchain
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_DEVICE_LOST`
	/// * `VK_ERROR_SURFACE_LOST_KHR`
	/// * `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
	#[cfg(feature = "Implements")]
	pub fn create(&self, device: &Device) -> ::Result<Swapchain>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe { Resolver::get().create_swapchain_khr(device.native_ptr(), &self.0, null(), &mut h) }
			.into_result()
			.map(|_| Swapchain(SwapchainCell
			{
				obj: h, dev: device.clone(), _target: self.1.clone(), fmt: self.0.imageFormat,
				size: Extent3D(self.0.imageExtent.width, self.0.imageExtent.height, 1)
			}.into()))
	}
}

impl Swapchain
{
	pub fn format(&self) -> VkFormat { self.0.fmt }
	pub fn size(&self) -> &Extent3D { &self.0.size }
}

use {Fence, Semaphore};
/// A semaphore or a fence
pub enum CompletionHandler<'s>
{
	/// A Host synchronizer(aka Fence)
	Host(&'s Fence),
	/// A Queue synchronizer(aka Semaphore)
	Queue(&'s Semaphore)
}
impl<'s> From<&'s Fence> for CompletionHandler<'s>
{
	fn from(f: &'s Fence) -> Self { CompletionHandler::Host(f) }
}
impl<'s> From<&'s Semaphore> for CompletionHandler<'s>
{
	fn from(s: &'s Semaphore) -> Self { CompletionHandler::Queue(s) }
}

#[cfg(feature = "Implements")]
impl Swapchain
{
	/// Retrieve the index of the next available presentation image
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_DEVICE_LOST`
	/// * `VK_ERROR_OUT_OF_DATE_KHR`
	/// * `VK_ERROR_SURFACE_LOST_KHR`
	pub fn acquire_next(&self, timeout: Option<u64>, completion: CompletionHandler) -> ::Result<u32>
	{
		let (semaphore, fence) = match completion
		{
			CompletionHandler::Host(f) => (VK_NULL_HANDLE as _, f.native_ptr()),
			CompletionHandler::Queue(s) => (s.native_ptr(), VK_NULL_HANDLE as _)
		};
		let mut n = 0;
		unsafe
		{
			Resolver::get().acquire_next_image_khr(self.device().native_ptr(),
				self.native_ptr(), timeout.unwrap_or(std::u64::MAX), semaphore, fence, &mut n)
		}.into_result().map(|_| n)
	}
	/// Queue an image for presentation
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_DEVICE_LOST`
	/// * `VK_ERROR_OUT_OF_DATE_KHR`
	/// * `VK_ERROR_SURFACE_LOST_KHR`
	pub fn queue_present(&self, queue: &Queue, index: u32, wait_semaphores: &[&Semaphore]) -> ::Result<()>
	{
		let mut res = 0;
		let wait_semaphores = wait_semaphores.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();
		let pinfo = VkPresentInfoKHR
		{
			waitSemaphoreCount: wait_semaphores.len() as _, pWaitSemaphores: wait_semaphores.as_ptr(),
			swapchainCount: 1, pSwapchains: &self.native_ptr(), pImageIndices: &index, pResults: &mut res,
			.. Default::default()
		};
		unsafe { Resolver::get().queue_present_khr(queue.native_ptr(), &pinfo) }
			.into_result()
			.and_then(|_| res.into_result())
	}
}
#[cfg(feature = "Implements")]
impl Queue
{
	/// Queue images for presentation
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_DEVICE_LOST`
	/// * `VK_ERROR_OUT_OF_DATE_KHR`
	/// * `VK_ERROR_SURFACE_LOST_KHR`
	pub fn present(&self, swapchains: &[(&Swapchain, u32)], wait_semaphores: &[&Semaphore]) -> ::Result<Vec<VkResult>>
	{
		let mut res = vec![0; swapchains.len()];
		let wait_semaphores = wait_semaphores.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();
		let (swapchains, indices): (Vec<_>, Vec<_>) = swapchains.iter().map(|&(ref x, n)| (x.native_ptr(), n)).unzip();
		let pinfo = VkPresentInfoKHR
		{
			waitSemaphoreCount: wait_semaphores.len() as _, pWaitSemaphores: wait_semaphores.as_ptr(),
			swapchainCount: swapchains.len() as _, pSwapchains: swapchains.as_ptr(), pImageIndices: indices.as_ptr(),
			pResults: res.as_mut_ptr(),
			.. Default::default()
		};
		unsafe { Resolver::get().queue_present_khr(self.native_ptr(), &pinfo) }.into_result().map(|_| res)
	}
}

/// Presentation mode supported for a surface
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PresentMode
{
	/// The presentatino engine does not wait for a vertical blanking period to update the current image, meaning
	/// this mode may result in visible tearing
	Immediate = VK_PRESENT_MODE_IMMEDIATE_KHR as _,
	/// The presentation engine waits for the next vertical blanking period to update the current image.
	/// Tearing cannot be observed. An internal single-entry queue is used to hold pending presentation requests.
	/// If the queue is full when a new presentation request is received, the new request replaces the existing entry, and any images
	/// associated with the prior entry become available for re-use by the application
	Mailbox = VK_PRESENT_MODE_MAILBOX_KHR as _,
	/// The presentation engine waits for the next vertical blanking period to update the current image.
	/// Tearing cannot be observed. An internal queue is used to hold pending presentation requests.
	/// New requests are appended to the end of the queue, and one request is removed from the beginning of the queue
	/// and processed during each vertical blanking period in which the queue is non-empty.
	FIFO = VK_PRESENT_MODE_FIFO_KHR as _,
	/// The presentation engine generally waits for the next vertical blanking period to update the currnt image.
	/// If a vertical blanking period has already passed since the last update of the current image then the presentation engine
	/// does not wait for another vertical blanking period for the update, meaning this mode may result in visible tearing in this case
	FIFORelaxed = VK_PRESENT_MODE_FIFO_RELAXED_KHR as _
}

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SurfaceTransform
{
	/// The image content is presented without being transformed
	Identity = VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR as _,
	/// The image content is rotated 90 degrees clockwise
	Rotate90 = VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR as _,
	/// The image content is rotated 180 degrees clockwise
	Rotate180 = VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR as _,
	/// The image content is rotated 270 degrees clockwise
	Rotate270 = VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR as _,
	/// The image content is mirrored horizontally
	HorizontalMirror = VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR as _,
	/// The image content is mirrored horizontally, then rotated 90 degrees clockwise
	HorizontalMirrorRotate90 = VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR as _,
	/// The image content is mirrored horizontally, then rotated 180 degrees clockwise
	HorizontalMirrorRotate180 = VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR as _,
	/// The image content is mirrored horizontally, then rotated 270 degrees clockwise
	HorizontalMirrorRotate270 = VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR as _,
	/// The presentation transform is not specified, and is instead determined by platform-specific considerations and mechanisms outside Vulkan
	Inherit = VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR as _
}
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeAlpha
{
	/// The alpha channel, if it exists, of the image is ignored in the compositing process
	Opaque = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR as _,
	/// The alpha channel, if it exists, of the images is respected in the compositing process.
	/// The non-alpha channels of the image are expected to already be multiplied by the alpha channel by the application
	PreMultiplied = VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR as _,
	/// The alpha channel, if it exists, of the images is respected in the compositing process.
	/// The non-alpha channels of the image are not expected to already be multiplied by the alpha channel by the application;
	/// instead, the compositor will multiply the non-alpha channels of the image by the alpha channel during compositing
	PostMultiplied = VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR as _,
	/// The way in which the presentation engine treats the alpha channel in the images is unknown to the Vulkan API.
	/// Instead, the application is responsible for setting the composite alpha blending mode using native window system commands
	Inherit = VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR as _
}
/// Alpha blending type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayPlaneAlpha
{
	/// The source image will be treated as opaque
	Opaque = VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR as _,
	/// A global alpha value must be specified that will be applied to all pixels in the source image
	Global = VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR as _,
	/// The alpha value will be determined by the alpha channel of the source image's pixels.
	/// If the source format contains no alpha values, no blending will be applied.
	/// The source alpha values are not premultiplied into the source image's other color channels
	PerPixel = VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR as _,
	/// This is equivalent to `PerPixel` except the source alpha values are assumed to be premultiplied into the source image's other color channels
	PrePixelPremultiplied = VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR as _
}

impl SurfaceTransform
{
	/// Does the value contains this bits
	pub fn contains(self, value: u32) -> bool { (value | self as u32) != 0 }
}
impl CompositeAlpha
{
	/// Does the value contains this bits
	pub fn contains(self, value: u32) -> bool { (value | self as u32) != 0 }
}
