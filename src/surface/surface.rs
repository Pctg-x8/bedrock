use core::{mem::MaybeUninit, ptr::NonNull};

use derives::{bitflags_newtype, implements};

use crate::{ImageUsageFlags, VkObject, VulkanStructure, vk::*};

use super::{Extent2D, Instance, ffi_helper::opt_pointer};

/// Opaque handle to a surface object
#[repr(transparent)]
pub struct Surface(VkSurfaceKHR_T);
#[implements]
impl Surface {
    pub unsafe fn destroy(&mut self, instance: &Instance, allocation_callbacks: Option<&VkAllocationCallbacks>) {
        unsafe {
            crate::vkfn::destroy_surface_khr(
                instance as *const _ as _,
                self as *mut _ as _,
                opt_pointer(allocation_callbacks),
            )
        }
    }
}

pub type SurfaceFormat = VkSurfaceFormatKHR;

/// Presentation mode supported for a surface
#[repr(i32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PresentMode {
    /// The presentation engine does not wait for a vertical blanking period to update the current image, meaning
    /// this mode may result in visible tearing
    Immediate = VK_PRESENT_MODE_IMMEDIATE_KHR,
    /// The presentation engine waits for the next vertical blanking period to update the current image.
    /// Tearing cannot be observed. An internal single-entry queue is used to hold pending presentation requests.
    /// If the queue is full when a new presentation request is received, the new request replaces the existing entry, and any images
    /// associated with the prior entry become available for re-use by the application
    Mailbox = VK_PRESENT_MODE_MAILBOX_KHR,
    /// The presentation engine waits for the next vertical blanking period to update the current image.
    /// Tearing cannot be observed. An internal queue is used to hold pending presentation requests.
    /// New requests are appended to the end of the queue, and one request is removed from the beginning of the queue
    /// and processed during each vertical blanking period in which the queue is non-empty.
    FIFO = VK_PRESENT_MODE_FIFO_KHR,
    /// The presentation engine generally waits for the next vertical blanking period to update the currnt image.
    /// If a vertical blanking period has already passed since the last update of the current image then the presentation engine
    /// does not wait for another vertical blanking period for the update, meaning this mode may result in visible tearing in this case
    FIFORelaxed = VK_PRESENT_MODE_FIFO_RELAXED_KHR,
}

/// Presentation transforms supported on a device
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags_newtype]
pub struct SurfaceTransformFlags(VkSurfaceTransformFlagsKHR);
impl SurfaceTransformFlags {
    /// The image content is presented without being transformed
    pub const IDENTITY: Self = Self(VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR);
    /// The image content is rotated 90 degrees clockwise
    pub const ROTATE_90: Self = Self(VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR);
    /// The image content is rotated 180 degrees clockwise
    pub const ROTATE_180: Self = Self(VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR);
    /// The image content is rotated 270 degrees clockwise
    pub const ROTATE_270: Self = Self(VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR);
    /// The image content is mirrored horizontally
    pub const HORIZONTAL_MIRROR: Self = Self(VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR);
    /// The image content is mirrored horizontally, then rotated 90 degrees clockwise
    pub const HORIZONTAL_MIRROR_ROTATE_90: Self = Self(VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR);
    /// The image content is mirrored horizontally, then rotated 180 degrees clockwise
    pub const HORIZONTAL_MIRROR_ROTATE_180: Self = Self(VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR);
    /// The image content is mirrored horizontally, then rotated 270 degrees clockwise
    pub const HORIZONTAL_MIRROR_ROTATE_270: Self = Self(VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR);
    /// The presentation transform is not specified, and is instead determined by platform-specific considerations and mechanisms outside Vulkan
    pub const INHERIT: Self = Self(VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR);
}

/// Alpha compositing modes supported on a device
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags_newtype]
pub struct CompositeAlphaFlags(VkCompositeAlphaFlagsKHR);
impl CompositeAlphaFlags {
    /// The alpha channel, if it exists, of the image is ignored in the compositing process
    pub const OPAQUE: Self = Self(VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR);
    /// The alpha channel, if it exists, of the images is respected in the compositing process.
    /// The non-alpha channels of the image are expected to already be multiplied by the alpha channel by the application
    pub const PRE_MULTIPLIED: Self = Self(VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR);
    /// The alpha channel, if it exists, of the images is respected in the compositing process.
    /// The non-alpha channels of the image are not expected to already be multiplied by the alpha channel by the application;
    /// instead, the compositor will multiply the non-alpha channels of the image by the alpha channel during compositing
    pub const POST_MULTIPLIED: Self = Self(VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR);
    /// The way in which the presentation engine treats the alpha channel in the images is unknown to the Vulkan API.
    /// Instead, the application is responsible for setting the composite alpha blending mode using native window system commands
    pub const INHERIT: Self = Self(VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR);
}

// specification extensions
pub type SurfaceCapabilities = VkSurfaceCapabilitiesKHR;
impl SurfaceCapabilities {
    /// The presentation transforms supported for the surface
    pub const fn supported_transforms(&self) -> SurfaceTransformFlags {
        SurfaceTransformFlags(self.supportedTransforms)
    }

    /// The alpha compositing modes supported by the presentation engine for the surface
    pub const fn supported_composite_alpha(&self) -> CompositeAlphaFlags {
        CompositeAlphaFlags(self.supportedCompositeAlpha)
    }

    /// The ways the application can use the presentable images of a swapchain
    pub const fn supported_usage_flags(&self) -> ImageUsageFlags {
        unsafe { core::mem::transmute(self.supportedUsageFlags) }
    }
}

pub trait SurfaceCreateInfo {
    #[implements]
    unsafe fn execute(
        &self,
        instance: &Instance,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<NonNull<Surface>>;
}

#[cfg(feature = "VK_KHR_xlib_surface")]
pub type XlibSurfaceCreateInfo = VkXlibSurfaceCreateInfoKHR;
#[cfg(feature = "VK_KHR_xlib_surface")]
impl XlibSurfaceCreateInfo {
    /// # Safety
    /// Provided `display` must be a valid reference
    pub const unsafe fn new(display: *mut x11::xlib::Display, window: x11::xlib::Window) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            dpy: display,
            window,
        }
    }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
impl SurfaceCreateInfo for XlibSurfaceCreateInfo {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        instance: &Instance,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<NonNull<Surface>> {
        let mut h = MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_xlib_surface_khr(
                instance as *const _ as _,
                self,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { NonNull::new_unchecked(h.assume_init() as _) })
    }
}

#[cfg(feature = "VK_KHR_xcb_surface")]
pub type XcbSurfaceCreateInfo = VkXcbSurfaceCreateInfoKHR;
#[cfg(feature = "VK_KHR_xcb_surface")]
impl XcbSurfaceCreateInfo {
    /// # Safety
    /// Provided `connection` must be a valid reference
    pub const unsafe fn new(connection: *mut xcb::ffi::xcb_connection_t, window: xcb::x::Window) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            connection,
            window,
        }
    }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
impl SurfaceCreateInfo for XcbSurfaceCreateInfo {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        instance: &Instance,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<NonNull<Surface>> {
        let mut h = MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_xcb_surface_khr(
                instance as *const _ as _,
                self,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { NonNull::new_unchecked(h.assume_init() as _) })
    }
}

#[cfg(feature = "VK_KHR_wayland_surface")]
pub type WaylandSurfaceCreateInfo = VkWaylandSurfaceCreateInfoKHR;
#[cfg(feature = "VK_KHR_wayland_surface")]
impl WaylandSurfaceCreateInfo {
    /// # Safety
    /// Provided `display` and `surface` must be a valid reference
    pub const unsafe fn new(display: *mut core::ffi::c_void, surface: *mut core::ffi::c_void) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            display,
            surface,
        }
    }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
impl SurfaceCreateInfo for WaylandSurfaceCreateInfo {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        instance: &Instance,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<NonNull<Surface>> {
        let mut h = MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_wayland_surface_khr(
                instance as *const _ as _,
                self,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { NonNull::new_unchecked(h.assume_init() as _) })
    }
}

#[cfg(feature = "VK_KHR_android_surface")]
pub type AndroidSurfaceCreateInfo = VkAndroidSurfaceCreateInfoKHR;
#[cfg(feature = "VK_KHR_android_surface")]
impl AndroidSurfaceCreateInfo {
    /// # Safety
    /// Provided `window` must be a valid reference
    pub const unsafe fn new(window: *mut android::ANativeWindow) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            window,
        }
    }
}
#[cfg(feature = "VK_KHR_android_surface")]
impl SurfaceCreateInfo for AndroidSurfaceCreateInfo {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        instance: &Instance,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<NonNull<Surface>> {
        let mut h = MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_android_surface_khr(
                instance as *const _ as _,
                self,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { NonNull::new_unchecked(h.assume_init() as _) })
    }
}

#[cfg(feature = "VK_KHR_win32_surface")]
pub type Win32SurfaceCreateInfo = VkWin32SurfaceCreateInfoKHR;
#[cfg(feature = "VK_KHR_win32_surface")]
impl Win32SurfaceCreateInfo {
    pub const fn new(hinstance: windows::Win32::Foundation::HINSTANCE, hwnd: windows::Win32::Foundation::HWND) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            hinstance,
            hwnd,
        }
    }
}
#[cfg(feature = "VK_KHR_win32_surface")]
impl SurfaceCreateInfo for Win32SurfaceCreateInfo {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        instance: &Instance,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<NonNull<Surface>> {
        let mut h = MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_win32_surface_khr(
                instance as *const _ as _,
                self,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { NonNull::new_unchecked(h.assume_init() as _) })
    }
}

#[cfg(feature = "VK_EXT_metal_surface")]
pub type MetalSurfaceCreateInfo = VkMetalSurfaceCreateInfoEXT;
#[cfg(feature = "VK_EXT_metal_surface")]
impl MetalSurfaceCreateInfo {
    /// # Safety
    /// Provided `layer` must be a valid reference
    pub const unsafe fn new(layer: *const core::ffi::c_void) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            pLayer: layer,
        }
    }
}
#[cfg(feature = "VK_EXT_metal_surface")]
impl SurfaceCreateInfo for MetalSurfaceCreateInfo {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        instance: &Instance,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<NonNull<Surface>> {
        let mut h = MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_metal_surface_ext(
                instance as *const _ as _,
                self,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { NonNull::new_unchecked(h.assume_init() as _) })
    }
}

#[cfg(feature = "VK_KHR_display")]
pub type DisplaySurfaceCreateInfo = VkDisplaySurfaceCreateInfoKHR;
#[cfg(feature = "VK_KHR_display")]
impl DisplaySurfaceCreateInfo {
    pub const fn new(
        mode: &super::DisplayMode,
        plane_index: u32,
        plane_stack_index: u32,
        transform: SurfaceTransformFlags,
        global_alpha: f32,
        alpha_mode: super::DisplayPlaneAlpha,
        extent: Extent2D,
    ) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            displayMode: mode.0,
            planeIndex: plane_index,
            planeStackIndex: plane_stack_index,
            transform: transform.bits(),
            globalAlpha: global_alpha,
            alphaMode: alpha_mode as _,
            imageExtent: extent,
        }
    }
}
