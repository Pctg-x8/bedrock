//! Direct Display Rendering
//! All functionality requires brvk::VK_KHR_display feature.

use crate::*;
use crate::{error::translate_vk_result, ffi_helper::opt_pointer};
use bedrock_vk::{self as brvk, TypedVulkanStructure};
#[allow(unused_imports)]
use derives::*;
use std::ops::Deref;

#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VK_OBJECT_TYPE_DISPLAY_KHR)]
pub struct Display<PhysicalDevice: crate::PhysicalDevice>(pub brvk::VkDisplayKHR, pub PhysicalDevice);

#[repr(transparent)]
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VK_OBJECT_TYPE_DISPLAY_MODE_KHR)]
pub struct DisplayMode(pub brvk::VkDisplayModeKHR);
impl DisplayMode {
    #[implements]
    pub unsafe fn new(
        display: &Display<impl crate::PhysicalDevice>,
        create_info: &brvk::VkDisplayModeCreateInfoKHR,
    ) -> crate::Result<Self> {
        Ok(Self(unsafe { display.create_display_mode_raw(create_info, None)? }))
    }
}

impl<PhysicalDevice: crate::PhysicalDevice> Display<PhysicalDevice> {
    /// Query a count of the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn mode_property_count(&self) -> crate::Result<u32> {
        let mut n = 0;
        translate_vk_result(unsafe {
            brvk::fns::get_display_mode_properties_khr(self.1.native_ptr(), self.0, &mut n, core::ptr::null_mut())
        })?;

        Ok(n)
    }

    /// Query the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn mode_properties(&self, sink: &mut [core::mem::MaybeUninit<DisplayModeProperties>]) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        translate_vk_result(unsafe {
            brvk::fns::get_display_mode_properties_khr(self.1.native_ptr(), self.0, &mut n, sink.as_mut_ptr() as _)
        })?;

        Ok(n)
    }

    /// Query the set of mode properties supported by the display.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements("alloc")]
    pub fn mode_properties_alloc(&self) -> crate::Result<Vec<DisplayModeProperties>> {
        let n = self.mode_property_count()? as usize;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = Vec::with_capacity(n);
        self.mode_properties(&mut xs.spare_capacity_mut()[..n])?;
        unsafe {
            xs.set_len(n);
        }

        Ok(xs)
    }

    /// Release access to an acquired brvk::VkDisplayKHR
    #[implements("VK_EXT_direct_mode_display")]
    pub fn release(&self)
    where
        PhysicalDevice::ConcreteInstance: InstanceDirectModeDisplayExtension,
    {
        unsafe {
            self.1.instance().release_display_ext_fn().0(self.1.native_ptr(), self.native_ptr());
        }
    }

    /// Acquire access to a brvk::VkDisplayKHR using Xlib
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_INITIALIZATION_FAILED`
    #[implements("VK_EXT_acquire_xlib_display")]
    pub fn acquire_xlib_display(&self, dpy: *mut x11::xlib::Display) -> crate::Result<()>
    where
        PhysicalDevice::ConcreteInstance: InstanceAcquireXlibDisplayExtension,
    {
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
    /// * brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// * brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///
    /// # Safety
    /// no guarantee will be provided (simply calls under api)
    #[implements]
    #[inline]
    pub unsafe fn create_display_mode_raw(
        &self,
        info: &brvk::VkDisplayModeCreateInfoKHR,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkDisplayModeKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        translate_vk_result(unsafe {
            brvk::fns::create_display_mode_khr(
                self.1.native_ptr(),
                self.native_ptr(),
                info,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Create a display mode
    /// # Failures
    /// On failure, this command returns
    ///
    /// * brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// * brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    pub fn create_display_mode(&self, params: brvk::VkDisplayModeParametersKHR) -> crate::Result<DisplayMode> {
        let cinfo = brvk::VkDisplayModeCreateInfoKHR {
            sType: brvk::VkDisplayModeCreateInfoKHR::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            parameters: params,
        };

        Ok(DisplayMode(unsafe { self.create_display_mode_raw(&cinfo, None)? }))
    }
}

#[repr(transparent)]
pub struct DisplayProperties(brvk::VkDisplayPropertiesKHR);
impl DisplayProperties {
    pub const fn supported_transforms(&self) -> SurfaceTransformFlags {
        SurfaceTransformFlags(self.0.supportedTransforms)
    }

    pub const fn plane_reorder_possible(&self) -> bool {
        self.0.planeReorderPossible != 0
    }

    pub const fn persistent_content(&self) -> bool {
        self.0.persistentContent != 0
    }

    pub const fn display_name(&self) -> Option<&core::ffi::CStr> {
        if self.0.displayName.is_null() {
            return None;
        }

        Some(unsafe { core::ffi::CStr::from_ptr(self.0.displayName) })
    }
}

pub struct DisplayPropertiesWithPhysicalDeviceRef<PhysicalDevice: crate::PhysicalDevice>(
    pub(crate) DisplayProperties,
    pub(crate) PhysicalDevice,
);
impl<PhysicalDevice: crate::PhysicalDevice> From<DisplayPropertiesWithPhysicalDeviceRef<PhysicalDevice>>
    for brvk::VkDisplayPropertiesKHR
{
    fn from(v: DisplayPropertiesWithPhysicalDeviceRef<PhysicalDevice>) -> Self {
        v.0.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> Deref for DisplayPropertiesWithPhysicalDeviceRef<PhysicalDevice> {
    type Target = brvk::VkDisplayPropertiesKHR;
    fn deref(&self) -> &brvk::VkDisplayPropertiesKHR {
        &self.0.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> AsRef<brvk::VkDisplayPropertiesKHR>
    for DisplayPropertiesWithPhysicalDeviceRef<PhysicalDevice>
{
    fn as_ref(&self) -> &brvk::VkDisplayPropertiesKHR {
        &self.0.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> DisplayPropertiesWithPhysicalDeviceRef<PhysicalDevice> {
    /// A handle that is used to refer to the display described here.
    /// This handle will be valid for the lifetime of the Vulkan instance.
    pub const fn display(&self) -> Display<&PhysicalDevice> {
        Display(self.0.0.display, &self.1)
    }

    /// The name of the display.
    pub const fn display_name(&self) -> &core::ffi::CStr {
        unsafe { core::ffi::CStr::from_ptr(self.0.0.displayName) }
    }

    /// Whether the planes on this display can have their z order changed.
    pub const fn can_reorder_plane(&self) -> bool {
        self.0.0.planeReorderPossible == brvk::VK_TRUE
    }

    /// Whether the display supports self-refresh/internal buffering.
    pub const fn has_persistent_content(&self) -> bool {
        self.0.0.persistentContent == brvk::VK_TRUE
    }
}

pub type DisplayPlaneProperties = brvk::VkDisplayPlanePropertiesKHR;

pub struct DisplayPlanePropertiesWithPhysicalDeviceRef<PhysicalDevice: crate::PhysicalDevice>(
    pub(crate) brvk::VkDisplayPlanePropertiesKHR,
    pub(crate) PhysicalDevice,
);
impl<PhysicalDevice: crate::PhysicalDevice> From<DisplayPlanePropertiesWithPhysicalDeviceRef<PhysicalDevice>>
    for brvk::VkDisplayPlanePropertiesKHR
{
    fn from(v: DisplayPlanePropertiesWithPhysicalDeviceRef<PhysicalDevice>) -> Self {
        v.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> Deref for DisplayPlanePropertiesWithPhysicalDeviceRef<PhysicalDevice> {
    type Target = brvk::VkDisplayPlanePropertiesKHR;
    fn deref(&self) -> &brvk::VkDisplayPlanePropertiesKHR {
        &self.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> AsRef<brvk::VkDisplayPlanePropertiesKHR>
    for DisplayPlanePropertiesWithPhysicalDeviceRef<PhysicalDevice>
{
    fn as_ref(&self) -> &brvk::VkDisplayPlanePropertiesKHR {
        &self.0
    }
}
impl<PhysicalDevice: crate::PhysicalDevice> DisplayPlanePropertiesWithPhysicalDeviceRef<PhysicalDevice> {
    /// The handle of the display the plane is currently associated with.
    /// If the plane is not currently attached to any displays, this will be `None`
    pub const fn current_display(&self) -> Option<Display<&PhysicalDevice>> {
        if self.0.currentDisplay.0.get() == 0 {
            None
        } else {
            Some(Display(self.0.currentDisplay, &self.1))
        }
    }
}

#[repr(transparent)]
pub struct DisplayModeProperties(brvk::VkDisplayModePropertiesKHR);
impl From<brvk::VkDisplayModePropertiesKHR> for DisplayModeProperties {
    fn from(v: brvk::VkDisplayModePropertiesKHR) -> Self {
        Self(v)
    }
}
impl From<DisplayModeProperties> for brvk::VkDisplayModePropertiesKHR {
    fn from(v: DisplayModeProperties) -> Self {
        v.0
    }
}
impl Deref for DisplayModeProperties {
    type Target = brvk::VkDisplayModePropertiesKHR;
    fn deref(&self) -> &brvk::VkDisplayModePropertiesKHR {
        &self.0
    }
}
impl AsRef<brvk::VkDisplayModePropertiesKHR> for DisplayModeProperties {
    fn as_ref(&self) -> &brvk::VkDisplayModePropertiesKHR {
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
    Opaque = brvk::VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR,
    /// A global alpha value must be specified that will be applied to all pixels in the source image
    Global = brvk::VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR,
    /// The alpha value will be determined by the alpha channel of the source image's pixels.
    /// If the source format contains no alpha values, no blending will be applied.
    /// The source alpha values are not premultiplied into the source image's other color channels
    PerPixel = brvk::VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR,
    /// This is equivalent to `PerPixel` except the source alpha values are assumed to be premultiplied into the source image's other color channels
    PrePixelPremultiplied = brvk::VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR,
}

#[repr(transparent)]
pub struct DisplayModeCreateInfo(brvk::VkDisplayModeCreateInfoKHR);
impl DisplayModeCreateInfo {
    pub const fn new(parameters: brvk::VkDisplayModeParametersKHR) -> Self {
        Self(brvk::VkDisplayModeCreateInfoKHR {
            sType: brvk::VkDisplayModeCreateInfoKHR::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            parameters,
        })
    }
}
