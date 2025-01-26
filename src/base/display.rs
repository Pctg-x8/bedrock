//! Direct Display Rendering
//! All functionality requires VK_KHR_display feature.

#[implements]
use crate::VkHandle;
use crate::{vk::*, VkObject, VulkanStructure};
#[allow(unused_imports)]
use derives::*;
use std::ops::Deref;

use super::opt_pointer;

#[derive(VkHandle, VkObject)]
#[VkObject(type = VK_OBJECT_TYPE_DISPLAY_KHR)]
pub struct Display<PhysicalDevice: crate::PhysicalDevice>(pub VkDisplayKHR, pub PhysicalDevice);

#[repr(transparent)]
#[derive(VkHandle, VkObject)]
#[VkObject(type = VK_OBJECT_TYPE_DISPLAY_MODE_KHR)]
pub struct DisplayMode(pub VkDisplayModeKHR);
impl DisplayMode {
    #[implements]
    pub unsafe fn new(
        display: &Display<impl crate::PhysicalDevice>,
        create_info: &VkDisplayModeCreateInfoKHR,
    ) -> crate::Result<Self> {
        Ok(Self(display.create_display_mode_raw(create_info, None)?))
    }
}

impl<PhysicalDevice: crate::PhysicalDevice> Display<PhysicalDevice> {
    /// Query a count of the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn mode_property_count(&self) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_display_mode_properties_khr(self.1.native_ptr(), self.0, &mut n, core::ptr::null_mut())
                .into_result()?;
        }

        Ok(n)
    }

    /// Query the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn mode_properties(&self, sink: &mut [DisplayModeProperties]) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_display_mode_properties_khr(self.1.native_ptr(), self.0, &mut n, sink.as_mut_ptr() as _)
                .into_result()?;
        }

        Ok(n)
    }

    /// Query the set of mode properties supported by the display.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements("alloc")]
    pub fn mode_properties_alloc(&self) -> crate::Result<Vec<DisplayModeProperties>> {
        let n = self.mode_property_count()?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.mode_properties(&mut xs)?;

        Ok(xs)
    }

    /// Release access to an acquired VkDisplayKHR
    #[implements("VK_EXT_direct_mode_display")]
    pub fn release(&self) {
        use crate::Instance;

        unsafe {
            self.1.instance().release_display_ext_fn().0(self.1.native_ptr(), self.native_ptr());
        }
    }

    /// Acquire access to a VkDisplayKHR using Xlib
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INITIALIZATION_FAILED`
    #[implements("VK_EXT_acquire_xlib_display")]
    pub fn acquire_xlib_display(&self, dpy: *mut x11::xlib::Display) -> crate::Result<()> {
        use crate::Instance;

        unsafe {
            self.1.instance().acquire_xlib_display_ext_fn().0(self.1.native_ptr(), dpy, self.native_ptr())
                .into_result()
                .map(drop)
        }
    }

    /// Create a display mode
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///
    /// # Safety
    /// no guarantee will be provided (simply calls under api)
    #[implements]
    #[inline]
    pub unsafe fn create_display_mode_raw(
        &self,
        info: &VkDisplayModeCreateInfoKHR,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkDisplayModeKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_display_mode_khr(
            self.1.native_ptr(),
            self.native_ptr(),
            info,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;

        Ok(h.assume_init())
    }

    /// Create a display mode
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    pub fn create_display_mode(&self, params: VkDisplayModeParametersKHR) -> crate::Result<DisplayMode> {
        let cinfo = VkDisplayModeCreateInfoKHR {
            sType: VkDisplayModeCreateInfoKHR::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            parameters: params,
        };

        Ok(DisplayMode(unsafe { self.create_display_mode_raw(&cinfo, None)? }))
    }
}

pub struct DisplayProperties<PhysicalDevice: crate::PhysicalDevice>(
    pub(crate) VkDisplayPropertiesKHR,
    pub(crate) PhysicalDevice,
);
impl<PhysicalDevice: crate::PhysicalDevice> From<DisplayProperties<PhysicalDevice>> for VkDisplayPropertiesKHR {
    fn from(v: DisplayProperties<PhysicalDevice>) -> Self {
        v.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> Deref for DisplayProperties<PhysicalDevice> {
    type Target = VkDisplayPropertiesKHR;
    fn deref(&self) -> &VkDisplayPropertiesKHR {
        &self.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> AsRef<VkDisplayPropertiesKHR> for DisplayProperties<PhysicalDevice> {
    fn as_ref(&self) -> &VkDisplayPropertiesKHR {
        &self.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> DisplayProperties<PhysicalDevice> {
    /// A handle that is used to refer to the display described here.
    /// This handle will be valid for the lifetime of the Vulkan instance.
    pub const fn display(&self) -> Display<&PhysicalDevice> {
        Display(self.0.display, &self.1)
    }

    /// The name of the display.
    pub const fn display_name(&self) -> &core::ffi::CStr {
        unsafe { core::ffi::CStr::from_ptr(self.0.displayName) }
    }

    /// Whether the planes on this display can have their z order changed.
    pub const fn can_reorder_plane(&self) -> bool {
        self.0.planeReorderPossible == VK_TRUE
    }

    /// Whether the display supports self-refresh/internal buffering.
    pub const fn has_persistent_content(&self) -> bool {
        self.0.persistentContent == VK_TRUE
    }
}

pub struct DisplayPlaneProperties<PhysicalDevice: crate::PhysicalDevice>(
    pub(crate) VkDisplayPlanePropertiesKHR,
    pub(crate) PhysicalDevice,
);
impl<PhysicalDevice: crate::PhysicalDevice> From<DisplayPlaneProperties<PhysicalDevice>>
    for VkDisplayPlanePropertiesKHR
{
    fn from(v: DisplayPlaneProperties<PhysicalDevice>) -> Self {
        v.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> Deref for DisplayPlaneProperties<PhysicalDevice> {
    type Target = VkDisplayPlanePropertiesKHR;
    fn deref(&self) -> &VkDisplayPlanePropertiesKHR {
        &self.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> AsRef<VkDisplayPlanePropertiesKHR>
    for DisplayPlaneProperties<PhysicalDevice>
{
    fn as_ref(&self) -> &VkDisplayPlanePropertiesKHR {
        &self.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> DisplayPlaneProperties<PhysicalDevice> {
    /// The handle of the display the plane is currently associated with.
    /// If the plane is not currently attached to any displays, this will be `None`
    pub const fn current_display(&self) -> Option<Display<&PhysicalDevice>> {
        if self.0.currentDisplay.0 == 0 {
            None
        } else {
            Some(Display(self.0.currentDisplay, &self.1))
        }
    }
}

#[repr(transparent)]
pub struct DisplayModeProperties(VkDisplayModePropertiesKHR);
impl From<VkDisplayModePropertiesKHR> for DisplayModeProperties {
    fn from(v: VkDisplayModePropertiesKHR) -> Self {
        Self(v)
    }
}
impl From<DisplayModeProperties> for VkDisplayModePropertiesKHR {
    fn from(v: DisplayModeProperties) -> Self {
        v.0
    }
}
impl Deref for DisplayModeProperties {
    type Target = VkDisplayModePropertiesKHR;
    fn deref(&self) -> &VkDisplayModePropertiesKHR {
        &self.0
    }
}
impl AsRef<VkDisplayModePropertiesKHR> for DisplayModeProperties {
    fn as_ref(&self) -> &VkDisplayModePropertiesKHR {
        &self.0
    }
}
impl DisplayModeProperties {
    /// A handle to the display mode described in this structure.
    /// This handle will be valid for the lifetime of the Vulkan instance.
    pub const fn display_mode(&self) -> DisplayMode {
        DisplayMode(self.0.displayMode)
    }
}

/// Alpha blending type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayPlaneAlpha {
    /// The source image will be treated as opaque
    Opaque = VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR,
    /// A global alpha value must be specified that will be applied to all pixels in the source image
    Global = VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR,
    /// The alpha value will be determined by the alpha channel of the source image's pixels.
    /// If the source format contains no alpha values, no blending will be applied.
    /// The source alpha values are not premultiplied into the source image's other color channels
    PerPixel = VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR,
    /// This is equivalent to `PerPixel` except the source alpha values are assumed to be premultiplied into the source image's other color channels
    PrePixelPremultiplied = VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR,
}

impl VkDisplayModeCreateInfoKHR {
    pub const fn new(parameters: VkDisplayModeParametersKHR) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            parameters,
        }
    }
}
