use derives::{bitflags_newtype, implements};

use crate::{
    vk::*, Extends, ImageUsageFlags, InstanceChild, VkHandle, VkObject, VkRawHandle, VulkanStructure,
    VulkanStructureProvider,
};

use super::ffi_helper::opt_pointer;

/// Opaque handle to a surface object
#[derive(VkHandle, VkObject, InstanceChild)]
#[VkObject(type = VkSurfaceKHR::OBJECT_TYPE)]
pub struct SurfaceObject<Instance: crate::Instance>(pub(crate) VkSurfaceKHR, #[parent] pub(crate) Instance);
unsafe impl<Instance: crate::Instance + Sync> Sync for SurfaceObject<Instance> {}
unsafe impl<Instance: crate::Instance + Send> Send for SurfaceObject<Instance> {}
#[implements]
impl<Instance: crate::Instance> Drop for SurfaceObject<Instance> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn::destroy_surface_khr(self.1.native_ptr(), self.0, core::ptr::null());
        }
    }
}
impl<Instance: crate::Instance> Surface for SurfaceObject<Instance> {}
impl<Instance: crate::Instance> SurfaceObject<Instance> {
    #[implements]
    #[inline(always)]
    pub unsafe fn new<
        PhysicalDevice: crate::PhysicalDevice + crate::InstanceChildTransferrable<ConcreteInstance = Instance>,
    >(
        pd: PhysicalDevice,
        create_info: &(impl SurfaceCreateInfo + ?Sized),
    ) -> crate::Result<Self> {
        Ok(Self(create_info.execute(&pd, None)?, pd.transfer_instance()))
    }
}
impl<Instance: crate::Instance + Clone> SurfaceObject<&'_ Instance> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> SurfaceObject<Instance> {
        SurfaceObject(self.0, self.1.clone())
    }
}

pub trait Surface: VkHandle<Handle = VkSurfaceKHR> + InstanceChild {}
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
impl VkSurfaceCapabilitiesKHR {
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
        pd: &(impl crate::PhysicalDevice + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR>;
}

#[cfg(feature = "VK_KHR_xlib_surface")]
impl VkXlibSurfaceCreateInfoKHR {
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
impl SurfaceCreateInfo for VkXlibSurfaceCreateInfoKHR {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        pd: &(impl crate::PhysicalDevice + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_xlib_surface_khr(
            pd.instance().native_ptr(),
            self,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;

        Ok(h.assume_init())
    }
}

#[cfg(feature = "VK_KHR_xcb_surface")]
impl VkXcbSurfaceCreateInfoKHR {
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
impl SurfaceCreateInfo for VkXcbSurfaceCreateInfoKHR {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        pd: &(impl crate::PhysicalDevice + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_xcb_surface_khr(
            pd.instance().native_ptr(),
            self,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;

        Ok(h.assume_init())
    }
}

#[cfg(feature = "VK_KHR_wayland_surface")]
impl VkWaylandSurfaceCreateInfoKHR {
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
impl SurfaceCreateInfo for VkWaylandSurfaceCreateInfoKHR {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        pd: &(impl crate::PhysicalDevice + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_wayland_surface_khr(
            pd.instance().native_ptr(),
            self,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;

        Ok(h.assume_init())
    }
}

#[cfg(feature = "VK_KHR_android_surface")]
impl VkAndroidSurfaceCreateInfoKHR {
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
impl SurfaceCreateInfo for VkAndroidSurfaceCreateInfoKHR {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        pd: &(impl crate::PhysicalDevice + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_android_surface_khr(
            pd.instance().native_ptr(),
            self,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;

        Ok(h.assume_init())
    }
}

#[cfg(feature = "VK_KHR_win32_surface")]
impl VkWin32SurfaceCreateInfoKHR {
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
impl SurfaceCreateInfo for VkWin32SurfaceCreateInfoKHR {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        pd: &(impl crate::PhysicalDevice + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_win32_surface_khr(
            pd.instance().native_ptr(),
            self,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;

        Ok(h.assume_init())
    }
}

#[cfg(feature = "VK_EXT_metal_surface")]
impl VkMetalSurfaceCreateInfoEXT {
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
impl SurfaceCreateInfo for VkMetalSurfaceCreateInfoEXT {
    #[implements]
    #[inline(always)]
    unsafe fn execute(
        &self,
        pd: &(impl crate::PhysicalDevice + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_metal_surface_ext(
            pd.instance().native_ptr(),
            self,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;

        Ok(h.assume_init())
    }
}

#[cfg(feature = "VK_KHR_display")]
impl VkDisplaySurfaceCreateInfoKHR {
    pub const fn new(
        mode: &super::DisplayMode,
        plane_index: u32,
        plane_stack_index: u32,
        transform: SurfaceTransformFlags,
        global_alpha: f32,
        alpha_mode: super::DisplayPlaneAlpha,
        extent: VkExtent2D,
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
