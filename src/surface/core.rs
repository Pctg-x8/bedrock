use crate::{ffi_helper::opt_pointer, *};
use bedrock_vk::{self as brvk, TypedVulkanStructure, VkRawHandle};
use derives::{bitflags_newtype, implements};

/// Opaque handle to a surface object
#[derive(VkHandle, VkObject, InstanceChild)]
#[VkObject(type = brvk::VkSurfaceKHR::OBJECT_TYPE)]
pub struct SurfaceObject<Instance: crate::Instance>(pub(crate) brvk::VkSurfaceKHR, #[parent] pub(crate) Instance);
unsafe impl<Instance: crate::Instance + Sync> Sync for SurfaceObject<Instance> {}
unsafe impl<Instance: crate::Instance + Send> Send for SurfaceObject<Instance> {}
#[implements]
impl<Instance: crate::Instance> Drop for SurfaceObject<Instance> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn_wrapper::destroy_surface(self.1.as_transparent_ref(), VkHandleRefMut::dangling(self.0), None);
        }
    }
}
impl<Instance: crate::Instance> Surface for SurfaceObject<Instance> {}
impl<Instance: crate::Instance> SurfaceObject<Instance> {
    #[implements]
    #[inline(always)]
    pub fn new(
        pd: impl crate::PhysicalDevice + crate::InstanceChildTransferrable<ConcreteInstance = Instance>,
        create_info: &(impl SurfaceCreateInfo + ?Sized),
    ) -> crate::Result<Self> {
        Ok(Self(create_info.execute(pd.instance(), None)?, pd.transfer_instance()))
    }
}
impl<Instance: crate::Instance + Clone> SurfaceObject<&'_ Instance> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> SurfaceObject<Instance> {
        let r = SurfaceObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}

pub trait Surface: VkHandle<Handle = brvk::VkSurfaceKHR> + InstanceChild {}
DerefContainerBracketImpl!(for Surface {});

pub trait TransferSurfaceObject {
    type ConcreteSurface: crate::Surface;

    fn transfer_surface(self) -> Self::ConcreteSurface;
}
impl<Parent: VulkanStructureProvider + TransferSurfaceObject, T> TransferSurfaceObject for Extends<Parent, T> {
    type ConcreteSurface = Parent::ConcreteSurface;

    #[inline(always)]
    fn transfer_surface(self) -> Self::ConcreteSurface {
        self.0.transfer_surface()
    }
}

/// Presentation mode supported for a surface
#[repr(i32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PresentMode {
    /// The presentation engine does not wait for a vertical blanking period to update the current image, meaning
    /// this mode may result in visible tearing
    Immediate = brvk::VK_PRESENT_MODE_IMMEDIATE_KHR,
    /// The presentation engine waits for the next vertical blanking period to update the current image.
    /// Tearing cannot be observed. An internal single-entry queue is used to hold pending presentation requests.
    /// If the queue is full when a new presentation request is received, the new request replaces the existing entry, and any images
    /// associated with the prior entry become available for re-use by the application
    Mailbox = brvk::VK_PRESENT_MODE_MAILBOX_KHR,
    /// The presentation engine waits for the next vertical blanking period to update the current image.
    /// Tearing cannot be observed. An internal queue is used to hold pending presentation requests.
    /// New requests are appended to the end of the queue, and one request is removed from the beginning of the queue
    /// and processed during each vertical blanking period in which the queue is non-empty.
    FIFO = brvk::VK_PRESENT_MODE_FIFO_KHR,
    /// The presentation engine generally waits for the next vertical blanking period to update the currnt image.
    /// If a vertical blanking period has already passed since the last update of the current image then the presentation engine
    /// does not wait for another vertical blanking period for the update, meaning this mode may result in visible tearing in this case
    FIFORelaxed = brvk::VK_PRESENT_MODE_FIFO_RELAXED_KHR,
}

/// Presentation transforms supported on a device
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags_newtype]
pub struct SurfaceTransformFlags(pub(crate) brvk::VkSurfaceTransformFlagsKHR);
impl SurfaceTransformFlags {
    /// The image content is presented without being transformed
    pub const IDENTITY: Self = Self(brvk::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR);
    /// The image content is rotated 90 degrees clockwise
    pub const ROTATE_90: Self = Self(brvk::VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR);
    /// The image content is rotated 180 degrees clockwise
    pub const ROTATE_180: Self = Self(brvk::VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR);
    /// The image content is rotated 270 degrees clockwise
    pub const ROTATE_270: Self = Self(brvk::VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR);
    /// The image content is mirrored horizontally
    pub const HORIZONTAL_MIRROR: Self = Self(brvk::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR);
    /// The image content is mirrored horizontally, then rotated 90 degrees clockwise
    pub const HORIZONTAL_MIRROR_ROTATE_90: Self = Self(brvk::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR);
    /// The image content is mirrored horizontally, then rotated 180 degrees clockwise
    pub const HORIZONTAL_MIRROR_ROTATE_180: Self =
        Self(brvk::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR);
    /// The image content is mirrored horizontally, then rotated 270 degrees clockwise
    pub const HORIZONTAL_MIRROR_ROTATE_270: Self =
        Self(brvk::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR);
    /// The presentation transform is not specified, and is instead determined by platform-specific considerations and mechanisms outside Vulkan
    pub const INHERIT: Self = Self(brvk::VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR);
}

/// Alpha compositing modes supported on a device
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags_newtype]
pub struct CompositeAlphaFlags(brvk::VkCompositeAlphaFlagsKHR);
impl CompositeAlphaFlags {
    /// The alpha channel, if it exists, of the image is ignored in the compositing process
    pub const OPAQUE: Self = Self(brvk::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR);
    /// The alpha channel, if it exists, of the images is respected in the compositing process.
    /// The non-alpha channels of the image are expected to already be multiplied by the alpha channel by the application
    pub const PRE_MULTIPLIED: Self = Self(brvk::VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR);
    /// The alpha channel, if it exists, of the images is respected in the compositing process.
    /// The non-alpha channels of the image are not expected to already be multiplied by the alpha channel by the application;
    /// instead, the compositor will multiply the non-alpha channels of the image by the alpha channel during compositing
    pub const POST_MULTIPLIED: Self = Self(brvk::VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR);
    /// The way in which the presentation engine treats the alpha channel in the images is unknown to the Vulkan API.
    /// Instead, the application is responsible for setting the composite alpha blending mode using native window system commands
    pub const INHERIT: Self = Self(brvk::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR);
}

#[repr(transparent)]
pub struct SurfaceCapabilities(pub brvk::VkSurfaceCapabilitiesKHR);
impl SurfaceCapabilities {
    /// The presentation transforms supported for the surface
    pub const fn supported_transforms(&self) -> SurfaceTransformFlags {
        SurfaceTransformFlags(self.0.supportedTransforms)
    }

    /// The surface's current transform relative to the presentation engine's natural orientation
    pub const fn current_transform(&self) -> SurfaceTransformFlags {
        SurfaceTransformFlags(self.0.currentTransform)
    }

    /// The alpha compositing modes supported by the presentation engine for the surface
    pub const fn supported_composite_alpha(&self) -> CompositeAlphaFlags {
        CompositeAlphaFlags(self.0.supportedCompositeAlpha)
    }

    /// The ways the application can use the presentable images of a swapchain
    pub const fn supported_usage_flags(&self) -> ImageUsageFlags {
        unsafe { core::mem::transmute(self.0.supportedUsageFlags) }
    }
}

pub use brvk::VkSurfaceFormatKHR as SurfaceFormat;

pub trait SurfaceCreateInfo {
    #[implements]
    fn execute(
        &self,
        instance: &(impl VkHandle<Handle = brvk::VkInstance> + ?Sized),
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkSurfaceKHR>;
}

#[cfg(feature = "VK_KHR_xlib_surface")]
#[repr(transparent)]
pub struct XlibSurfaceCreateInfo(brvk::VkXlibSurfaceCreateInfoKHR);
#[cfg(feature = "VK_KHR_xlib_surface")]
impl XlibSurfaceCreateInfo {
    /// # Safety
    /// Provided `display` must be a valid reference
    pub const unsafe fn new(display: *mut x11::xlib::Display, window: x11::xlib::Window) -> Self {
        Self(brvk::VkXlibSurfaceCreateInfoKHR {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            dpy: display,
            window,
        })
    }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
impl SurfaceCreateInfo for XlibSurfaceCreateInfo {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        instance: &(impl VkHandle<Handle = brvk::VkInstance> + ?Sized),
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            brvk::create_xlib_surface_khr(
                instance.native_ptr(),
                &self.0,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }
}

#[cfg(feature = "VK_KHR_xcb_surface")]
#[repr(transparent)]
pub struct XcbSurfaceCreateInfo(brvk::VkXcbSurfaceCreateInfoKHR);
#[cfg(feature = "VK_KHR_xcb_surface")]
impl XcbSurfaceCreateInfo {
    /// # Safety
    /// Provided `connection` must be a valid reference
    pub const unsafe fn new(connection: *mut xcb::ffi::xcb_connection_t, window: xcb::x::Window) -> Self {
        Self(brvk::VkXcbSurfaceCreateInfoKHR {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            connection,
            window,
        })
    }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
impl SurfaceCreateInfo for XcbSurfaceCreateInfo {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        instance: &(impl VkHandle<Handle = brvk::VkInstance> + ?Sized),
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            brvk::create_xcb_surface_khr(
                instance.native_ptr(),
                &self.0,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }
}

#[cfg(feature = "VK_KHR_wayland_surface")]
#[repr(transparent)]
pub struct WaylandSurfaceCreateInfo(brvk::VkWaylandSurfaceCreateInfoKHR);
#[cfg(feature = "VK_KHR_wayland_surface")]
impl WaylandSurfaceCreateInfo {
    /// # Safety
    /// Provided `display` and `surface` must be a valid reference
    pub const unsafe fn new(display: *mut core::ffi::c_void, surface: *mut core::ffi::c_void) -> Self {
        Self(brvk::VkWaylandSurfaceCreateInfoKHR {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            display,
            surface,
        })
    }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
impl SurfaceCreateInfo for WaylandSurfaceCreateInfo {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        instance: &(impl VkHandle<Handle = brvk::VkInstance> + ?Sized),
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            brvk::create_wayland_surface_khr(
                instance.native_ptr(),
                &self.0,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }
}

#[cfg(feature = "VK_KHR_android_surface")]
#[repr(transparent)]
pub struct AndroidSurfaceCreateInfo(brvk::VkAndroidSurfaceCreateInfoKHR);
#[cfg(feature = "VK_KHR_android_surface")]
impl AndroidSurfaceCreateInfo {
    /// # Safety
    /// Provided `window` must be a valid reference
    pub const unsafe fn new(window: *mut android::ANativeWindow) -> Self {
        Self(brvk::VkAndroidSurfaceCreateInfoKHR {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            window,
        })
    }
}
#[cfg(feature = "VK_KHR_android_surface")]
impl SurfaceCreateInfo for AndroidSurfaceCreateInfo {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        instance: &(impl VkHandle<Handle = brvk::VkInstance> + ?Sized),
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            brvk::create_android_surface_khr(
                instance.native_ptr(),
                &self.0,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }
}

#[cfg(feature = "VK_KHR_win32_surface")]
#[repr(transparent)]
pub struct Win32SurfaceCreateInfo(brvk::VkWin32SurfaceCreateInfoKHR);
#[cfg(feature = "VK_KHR_win32_surface")]
impl Win32SurfaceCreateInfo {
    pub const fn new(hinstance: windows::Win32::Foundation::HINSTANCE, hwnd: windows::Win32::Foundation::HWND) -> Self {
        Self(brvk::VkWin32SurfaceCreateInfoKHR {
            sType: brvk::VkWin32SurfaceCreateInfoKHR::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            hinstance,
            hwnd,
        })
    }
}
#[cfg(feature = "VK_KHR_win32_surface")]
impl SurfaceCreateInfo for Win32SurfaceCreateInfo {
    #[implements]
    #[inline]
    fn execute(
        &self,
        instance: &(impl VkHandle<Handle = brvk::VkInstance> + ?Sized),
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();
        crate::error::translate_vk_result(unsafe {
            brvk::fns::create_win32_surface_khr(
                instance.native_ptr(),
                &self.0,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
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
        instance: &(impl VkHandle<Handle = brvk::VkInstance> + ?Sized),
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            brvk::create_metal_surface_ext(
                instance.native_ptr(),
                self,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }
}

#[cfg(feature = "VK_KHR_display")]
#[repr(transparent)]
pub struct DisplaySurfaceCreateInfo(pub brvk::VkDisplaySurfaceCreateInfoKHR);
#[cfg(feature = "VK_KHR_display")]
impl DisplaySurfaceCreateInfo {
    pub const fn new(
        mode: &super::DisplayMode,
        plane_index: u32,
        plane_stack_index: u32,
        transform: SurfaceTransformFlags,
        global_alpha: f32,
        alpha_mode: super::DisplayPlaneAlpha,
        extent: brvk::VkExtent2D,
    ) -> Self {
        Self(brvk::VkDisplaySurfaceCreateInfoKHR {
            sType: brvk::VkDisplaySurfaceCreateInfoKHR::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            displayMode: mode.0,
            planeIndex: plane_index,
            planeStackIndex: plane_stack_index,
            transform: transform.bits(),
            globalAlpha: global_alpha,
            alphaMode: alpha_mode as _,
            imageExtent: extent,
        })
    }
}
