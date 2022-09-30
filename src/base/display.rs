//! Direct Display Rendering
//! All functionality requires VK_KHR_display feature.

use crate::{vk::*, VkObject};
#[cfg(feature = "Implements")]
use crate::{Resolver, ResolverInterface, VkHandle, VkResultHandler};
#[allow(unused_imports)]
use derives::*;
use std::ops::Deref;

#[derive(VkHandle)]
pub struct Display<PhysicalDevice: crate::PhysicalDevice>(pub(crate) VkDisplayKHR, pub(crate) PhysicalDevice);
impl<PhysicalDevice: crate::PhysicalDevice> VkObject for Display<PhysicalDevice> {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_DISPLAY_KHR;
}

#[repr(transparent)]
#[derive(VkHandle)]
pub struct DisplayMode(pub(crate) VkDisplayModeKHR);
impl VkObject for DisplayMode {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_DISPLAY_MODE_KHR;
}

impl<PhysicalDevice: crate::PhysicalDevice> Display<PhysicalDevice> {
    /// Query the set of mode properties supported by the display.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[cfg(feature = "Implements")]
    pub fn mode_properties(&self) -> crate::Result<Vec<DisplayModeProperties>> {
        unsafe {
            let mut n = 0;
            Resolver::get()
                .get_display_mode_properties_khr(self.1.native_ptr(), self.0, &mut n, std::ptr::null_mut())
                .into_result()?;
            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            Resolver::get()
                .get_display_mode_properties_khr(self.1.native_ptr(), self.0, &mut n, v.as_mut_ptr() as *mut _)
                .into_result()
                .map(move |_| v)
        }
    }

    /// Release access to an acquired VkDisplayKHR
    #[cfg(all(feature = "Implements", feature = "VK_EXT_direct_mode_display"))]
    pub fn release(&self) {
        let fp: PFN_vkReleaseDisplayEXT = physical_device
            .parent()
            .extra_procedure("vkReleaseDisplayEXT")
            .expect("no vkReleaseDisplayEXT exported?");
        fp(self.1.native_ptr(), self.native_ptr());
    }

    /// Acquire access to a VkDisplayKHR using Xlib
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INITIALIZATION_FAILED`
    #[cfg(all(feature = "Implements", feature = "VK_EXT_acquire_xlib_display"))]
    pub fn acquire_xlib_display(&self, dpy: *mut x11::xlib::Display) -> crate::Result<()> {
        let fp: PFN_vkAcquireXlibDisplayEXT = self
            .1
            .parent()
            .extra_procedure("vkAcquireXlibDisplayEXT")
            .expect("no vkAcquireXlibDisplayEXT exported?");
        fp(self.1.native_ptr(), dpy, self.native_ptr()).into_result()
    }

    /// Create a display mode
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[cfg(feature = "Implements")]
    pub fn create_display_mode(&self, params: VkDisplayModeParametersKHR) -> crate::Result<DisplayMode> {
        unsafe {
            let cinfo = VkDisplayModeCreateInfoKHR {
                parameters: params,
                ..Default::default()
            };
            let mut h = VK_NULL_HANDLE as _;
            Resolver::get()
                .create_display_mode_khr(self.1.native_ptr(), self.native_ptr(), &cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(move |_| DisplayMode(h))
        }
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
    pub fn display(&self) -> Display<&PhysicalDevice> {
        Display(self.display, &self.1)
    }

    /// The name of the display.
    pub fn display_name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(self.displayName) }
    }

    /// Whether the planes on this display can have their z order changed.
    pub fn can_reorder_plane(&self) -> bool {
        self.planeReorderPossible == VK_TRUE
    }

    /// Whether the display supports self-refresh/internal buffering.
    pub fn has_persistent_content(&self) -> bool {
        self.persistentContent == VK_TRUE
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
    pub fn current_display(&self) -> Option<Display<&PhysicalDevice>> {
        if self.currentDisplay == VK_NULL_HANDLE as _ {
            None
        } else {
            Some(Display(self.currentDisplay, &self.1))
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
    pub fn display_mode(&self) -> DisplayMode {
        DisplayMode(self.displayMode)
    }
}

/// Alpha blending type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayPlaneAlpha {
    /// The source image will be treated as opaque
    Opaque = VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR as _,
    /// A global alpha value must be specified that will be applied to all pixels in the source image
    Global = VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR as _,
    /// The alpha value will be determined by the alpha channel of the source image's pixels.
    /// If the source format contains no alpha values, no blending will be applied.
    /// The source alpha values are not premultiplied into the source image's other color channels
    PerPixel = VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR as _,
    /// This is equivalent to `PerPixel` except the source alpha values are assumed to be premultiplied into the source image's other color channels
    PrePixelPremultiplied = VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR as _,
}
