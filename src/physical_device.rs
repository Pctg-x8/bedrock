use derives::implements;

use crate::{
    EnumerationResult, ExtensionProperties, Format, ImageFlags, ImageTiling, ImageType, ImageUsageFlags,
    LayerProperties, ffi_helper::opt_cstr_ptr, vk::*,
};
use core::{ffi::CStr, mem::MaybeUninit, ptr::null_mut};

// Vulkan ReExports //
pub type PhysicalDeviceFeatures = VkPhysicalDeviceFeatures;
pub type PhysicalDeviceProperties = VkPhysicalDeviceProperties;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PhysicalDeviceProperties2 = VkPhysicalDeviceProperties2KHR;
pub type FormatProperties = VkFormatProperties;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type FormatProperties2 = VkFormatProperties2KHR;
pub type ImageFormatProperties = VkImageFormatProperties;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type ImageFormatProperties2 = VkImageFormatProperties2KHR;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PhysicalDeviceImageFormatInfo2 = VkPhysicalDeviceImageFormatInfo2KHR;
pub type QueueFamilyProperties = VkQueueFamilyProperties;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type QueueFamilyProperties2 = VkQueueFamilyProperties2;
pub type PhysicalDeviceMemoryProperties = VkPhysicalDeviceMemoryProperties;
#[cfg(feature = "Allow1_1APIs")]
pub type PhysicalDeviceMemoryProperties2 = VkPhysicalDeviceMemoryProperties2;

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalDeviceFeatures2<'r>(
    VkPhysicalDeviceFeatures2KHR,
    core::marker::PhantomData<Option<&'r mut dyn crate::VulkanStructureAsRef>>,
);
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl<'r> PhysicalDeviceFeatures2<'r> {
    pub const fn new(old_features: VkPhysicalDeviceFeatures) -> Self {
        Self(
            VkPhysicalDeviceFeatures2KHR {
                sType: <VkPhysicalDeviceFeatures2KHR as crate::VulkanStructure>::TYPE,
                pNext: core::ptr::null_mut(),
                features: old_features,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'r mut (impl crate::VulkanStructureAsRef + ?Sized)) -> Self {
        self.0.pNext = next.as_generic_mut() as *mut _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl crate::VulkanStructureAsRef for PhysicalDeviceFeatures2<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        crate::VulkanStructureAsRef::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        crate::VulkanStructureAsRef::as_generic_mut(&mut self.0)
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl crate::VulkanSinkStructureAsRef for PhysicalDeviceFeatures2<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanSinkStructure {
        crate::VulkanSinkStructureAsRef::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanSinkStructure {
        crate::VulkanSinkStructureAsRef::as_generic_mut(&mut self.0)
    }
}

/// Opaque handle to a physical device object
#[repr(transparent)]
pub struct PhysicalDevice(VkPhysicalDevice_T);
#[implements]
impl PhysicalDevice {
    /// Reports capabilities of a physical device.
    #[inline]
    pub fn get_features(&self, sink: &mut MaybeUninit<PhysicalDeviceFeatures>) {
        unsafe {
            crate::vkfn::get_physical_device_features(self as *const _ as _, sink.as_mut_ptr());
        }
    }

    /// Reports capabilities of a physical device.
    #[inline]
    pub fn features(&self) -> PhysicalDeviceFeatures {
        let mut sink = MaybeUninit::uninit();
        self.get_features(&mut sink);

        unsafe { sink.assume_init() }
    }

    /// Reports capabilities of a physical device.
    #[inline]
    #[cfg(feature = "Allow1_1APIs")]
    pub fn get_features2(&self, sink: &mut MaybeUninit<PhysicalDeviceFeatures2>) {
        unsafe {
            crate::vkfn::get_physical_device_features2(self as *const _ as _, sink.as_mut_ptr() as _);
        }
    }

    /// Returns properties of a physical device.
    #[inline]
    pub fn get_properties(&self, sink: &mut MaybeUninit<PhysicalDeviceProperties>) {
        unsafe {
            crate::vkfn::get_physical_device_properties(self as *const _ as _, sink.as_mut_ptr());
        }
    }

    /// Returns properties of a physical device.
    #[inline]
    pub fn properties(&self) -> PhysicalDeviceProperties {
        let mut sink = MaybeUninit::uninit();
        self.get_properties(&mut sink);

        unsafe { sink.assume_init() }
    }

    /// Returns properties of a physical device.
    #[inline]
    #[cfg(feature = "Allow1_1APIs")]
    pub fn get_properties2(&self, sink: &mut MaybeUninit<PhysicalDeviceProperties2>) {
        unsafe {
            crate::vkfn::get_physical_device_properties2(self as *const _ as _, sink.as_mut_ptr());
        }
    }

    /// Lists physical device's format capabilities.
    #[inline]
    pub fn get_format_properties(&self, format: Format, sink: &mut MaybeUninit<FormatProperties>) {
        unsafe {
            crate::vkfn::get_physical_device_format_properties(self as *const _ as _, format, sink.as_mut_ptr());
        }
    }

    /// Lists physical device's format capabilities.
    #[inline]
    pub fn format_properties(&self, format: Format) -> FormatProperties {
        let mut sink = MaybeUninit::uninit();
        self.get_format_properties(format, &mut sink);

        unsafe { sink.assume_init() }
    }

    /// Lists physical device's format capabilities.
    #[inline]
    #[cfg(feature = "Allow1_1APIs")]
    pub fn get_format_properties2(&self, format: Format, sink: &mut MaybeUninit<FormatProperties2>) {
        unsafe {
            crate::vkfn::get_physical_device_format_properties2(self as *const _ as _, format, sink.as_mut_ptr());
        }
    }

    /// List physical device's image format capabilities.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_FORMAT_NOT_SUPPORTED`]
    #[inline]
    pub fn get_image_format_properties(
        &self,
        format: Format,
        r#type: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageFlags,
        sink: &mut MaybeUninit<ImageFormatProperties>,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::get_physical_device_image_format_properties(
                self as *const _ as _,
                format,
                r#type as _,
                tiling as _,
                usage.bits(),
                flags.bits(),
                sink.as_mut_ptr(),
            )
            .into_result()
            .map(drop)
        }
    }

    /// List physical device's image format capabilities.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_FORMAT_NOT_SUPPORTED`]
    #[inline]
    pub fn image_format_properties(
        &self,
        format: Format,
        r#type: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageFlags,
    ) -> crate::Result<ImageFormatProperties> {
        let mut sink = MaybeUninit::uninit();
        self.get_image_format_properties(format, r#type, tiling, usage, flags, &mut sink)?;

        Ok(unsafe { sink.assume_init() })
    }

    /// List physical device's image format capabilities.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_FORMAT_NOT_SUPPORTED`]
    /// * [`VK_ERROR_IMAGE_USAGE_IS_NOT_SUPPORTED_KHR`]
    /// * [`VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`]
    /// * [`VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`]
    /// * [`VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`]
    /// * [`VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`]
    #[inline]
    #[cfg(feature = "Allow1_1APIs")]
    pub fn get_image_format_properties2(
        &self,
        info: &PhysicalDeviceImageFormatInfo2,
        sink: &mut MaybeUninit<ImageFormatProperties2>,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::get_physical_device_image_format_properties2(self as *const _ as _, info, sink.as_mut_ptr())
                .into_result()
                .map(drop)
        }
    }

    /// Reports a number of the queues of the specified physical device.
    #[inline]
    pub fn queue_family_property_count(&self) -> u32 {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_physical_device_queue_family_properties(self as *const _ as _, &mut n, null_mut());
        }

        n
    }

    /// Reports properties of the queues of the specified physical device.
    #[inline]
    pub fn queue_family_properties(&self, sink: &mut [QueueFamilyProperties]) -> u32 {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_physical_device_queue_family_properties(self as *const _ as _, &mut n, sink.as_mut_ptr());
        }

        n
    }

    /// Reports properties of the queues of the specified physical device.
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn queue_family_properties_alloc(&self) -> Vec<QueueFamilyProperties> {
        let n = self.queue_family_property_count();
        let mut v = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.queue_family_properties(&mut v);

        v
    }

    /// Reports a number of the queues of the specified physical device.
    #[inline]
    #[cfg(feature = "Allow1_1APIs")]
    pub fn queue_family_property_count2(&self) -> u32 {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_physical_device_queue_family_properties2(self as *const _ as _, &mut n, null_mut());
        }

        n
    }

    /// Reports properties of the queues of the specified physical device.
    #[inline]
    #[cfg(feature = "Allow1_1APIs")]
    pub fn queue_family_properties2(&self, sink: &mut [QueueFamilyProperties2]) -> u32 {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_physical_device_queue_family_properties2(self as *const _ as _, &mut n, sink.as_mut_ptr());
        }

        n
    }

    /// Reports memory information for the specified physical device.
    #[inline]
    pub fn get_memory_properties(&self, sink: &mut MaybeUninit<PhysicalDeviceMemoryProperties>) {
        unsafe {
            crate::vkfn::get_physical_device_memory_properties(self as *const _ as _, sink.as_mut_ptr());
        }
    }

    /// Reports memory information for the specified physical device.
    #[inline]
    pub fn memory_properties(&self) -> PhysicalDeviceMemoryProperties {
        let mut sink = MaybeUninit::uninit();
        self.get_memory_properties(&mut sink);

        unsafe { sink.assume_init() }
    }

    /// Reports memory information for the specified physical device.
    #[inline]
    #[cfg(feature = "Allow1_1APIs")]
    pub fn get_memory_properties2(&self, sink: &mut MaybeUninit<PhysicalDeviceMemoryProperties2>) {
        unsafe {
            crate::vkfn::get_physical_device_memory_properties2(self as *const _ as _, sink.as_mut_ptr());
        }
    }

    /// Returns a number of properties of available physical device layers.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline]
    pub fn layer_property_count(&self) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::enumerate_device_layer_properties(self as *const _ as _, &mut n, null_mut()).into_result()?;
        }

        Ok(n)
    }

    /// Returns properties of available physical device layers.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    pub fn layer_properties(&self, sink: &mut [LayerProperties]) -> crate::Result<(u32, EnumerationResult)> {
        let mut n = sink.len() as _;
        let r =
            unsafe { crate::vkfn::enumerate_device_layer_properties(self as *const _ as _, &mut n, sink.as_mut_ptr()) };

        if r == VK_SUCCESS {
            return Ok((n, EnumerationResult::Complete));
        }
        if r == VK_INCOMPLETE {
            return Ok((n, EnumerationResult::Incomplete));
        }

        Err(r)
    }

    /// Returns properties of available physical device layers.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn layer_properties_alloc(&self) -> crate::Result<Vec<LayerProperties>> {
        let n = self.layer_property_count()?;
        let mut v = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        // ここでは全部とれる(Incompleteの対処は不要)
        self.layer_properties(&mut v)?;

        Ok(v)
    }

    /// Returns a number of properties of available physical device extensions.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_LAYER_NOT_PRESENT`]
    #[inline]
    pub fn extension_property_count(&self, layer_name: Option<&CStr>) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::enumerate_device_extension_properties(
                self as *const _ as _,
                opt_cstr_ptr(layer_name),
                &mut n,
                null_mut(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Returns properties of available physical device extensions.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_LAYER_NOT_PRESENT`]
    pub fn extension_properties(
        &self,
        layer_name: Option<&CStr>,
        sink: &mut [ExtensionProperties],
    ) -> crate::Result<(u32, EnumerationResult)> {
        let mut n = sink.len() as _;
        let r = unsafe {
            crate::vkfn::enumerate_device_extension_properties(
                self as *const _ as _,
                opt_cstr_ptr(layer_name),
                &mut n,
                sink.as_mut_ptr(),
            )
        };

        if r == VK_SUCCESS {
            return Ok((n, EnumerationResult::Complete));
        }
        if r == VK_INCOMPLETE {
            return Ok((n, EnumerationResult::Incomplete));
        }

        Err(r)
    }

    /// Returns properties of available physical device extensions.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_LAYER_NOT_PRESENT`]
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn extension_properties_alloc(&self, layer_name: Option<&CStr>) -> crate::Result<Vec<ExtensionProperties>> {
        let n = self.extension_property_count(layer_name)?;
        let mut v = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        // ここでは全部とれる(Incompleteの対処は不要)
        self.extension_properties(layer_name, &mut v)?;

        Ok(v)
    }

    /// Query physical device for presentation to X11 server using Xlib
    /// # Safety
    /// Provided `display` must be a valid reference
    #[cfg(feature = "VK_KHR_xlib_surface")]
    #[inline]
    pub unsafe fn xlib_presentation_support(
        &self,
        queue_family: u32,
        display: *mut x11::xlib::Display,
        visual: x11::xlib::VisualID,
    ) -> bool {
        unsafe {
            crate::vkfn::get_physical_device_xlib_presentation_support_khr(
                self as *const _ as _,
                queue_family,
                display,
                visual,
            ) != 0
        }
    }

    /// Query physical device for presentation to X11 server using XCB
    /// # Safety
    /// Provided `connection` must be a valid reference
    #[cfg(feature = "VK_KHR_xcb_surface")]
    #[inline]
    pub unsafe fn xcb_presentation_support(
        &self,
        queue_family: u32,
        connection: *mut xcb::ffi::xcb_connection_t,
        visual: xcb::x::Visualid,
    ) -> bool {
        unsafe {
            crate::vkfn::get_physical_device_xcb_presentation_support_khr(
                self as *const _ as _,
                queue_family,
                connection,
                visual,
            ) != 0
        }
    }

    /// Query physical device for presentation to Wayland
    /// # Safety
    /// Provided `display` must be a valid reference
    #[cfg(feature = "VK_KHR_wayland_surface")]
    #[inline]
    pub unsafe fn wayland_presentation_support(&self, queue_family: u32, display: *mut core::ffi::c_void) -> bool {
        unsafe {
            crate::vkfn::get_physical_device_wayland_presentation_support_khr(
                self as *const _ as _,
                queue_family,
                display,
            ) != 0
        }
    }

    /// Query queue family support for presentation on a Win32 display
    #[cfg(feature = "VK_KHR_win32_surface")]
    #[inline]
    pub fn win32_presentation_support(&self, queue_family: u32) -> bool {
        unsafe {
            crate::vkfn::get_physical_device_win32_presentation_support_khr(self as *const _ as _, queue_family) != 0
        }
    }

    /// Query if presentation is supported
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[cfg(feature = "VK_KHR_surface")]
    #[inline]
    pub fn surface_support(&self, queue_family: u32, surface: &crate::Surface) -> crate::Result<bool> {
        let mut f = 0;

        unsafe {
            crate::vkfn::get_physical_device_surface_support_khr(
                self as *const _ as _,
                queue_family,
                surface as *const _ as _,
                &mut f,
            )
            .into_result()?;
        }

        Ok(f != 0)
    }

    /// Query surface capabilities
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[cfg(feature = "VK_KHR_surface")]
    #[inline]
    pub fn get_surface_capabilities(
        &self,
        surface: &crate::Surface,
        sink: &mut MaybeUninit<crate::SurfaceCapabilities>,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::get_physical_device_surface_capabilities_khr(
                self as *const _ as _,
                surface as *const _ as _,
                sink.as_mut_ptr(),
            )
            .into_result()
            .map(drop)
        }
    }

    /// Query surface capabilities
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[cfg(feature = "VK_KHR_surface")]
    #[inline]
    pub fn surface_capabilities(&self, surface: &crate::Surface) -> crate::Result<crate::SurfaceCapabilities> {
        let mut sink = MaybeUninit::uninit();
        unsafe {
            self.get_surface_capabilities(surface, &mut sink)?;
        }

        Ok(unsafe { sink.assume_init() })
    }

    /// Query a count of color formats supported by surface
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[cfg(feature = "VK_KHR_surface")]
    #[inline]
    pub fn surface_format_count(&self, surface: &crate::Surface) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_physical_device_surface_formats_khr(
                self as *const _ as _,
                surface as *const _ as _,
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Query a count of color formats supported by surface
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[cfg(feature = "VK_KHR_surface")]
    pub fn surface_formats(
        &self,
        surface: &crate::Surface,
        sink: &mut [crate::SurfaceFormat],
    ) -> crate::Result<(u32, EnumerationResult)> {
        let mut n = sink.len() as _;
        let r = unsafe {
            crate::vkfn::get_physical_device_surface_formats_khr(
                self.native_ptr(),
                surface.native_ptr(),
                &mut n,
                sink.as_mut_ptr(),
            )
        };

        if r == VK_SUCCESS {
            return Ok((n, EnumerationResult::Complete));
        }
        if r == VK_INCOMPLETE {
            return Ok((n, EnumerationResult::Incomplete));
        }

        Err(r)
    }

    /// Query color formats supported by surface
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[cfg(feature = "VK_KHR_surface")]
    #[cfg(feature = "alloc")]
    pub fn surface_formats_alloc(&self, surface: &crate::Surface) -> crate::Result<Vec<crate::SurfaceFormat>> {
        let n = self.surface_format_count(surface)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        // ここでは全部とれる(Incompleteの対処は不要)
        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.surface_formats(surface, &mut xs)?;

        Ok(xs)
    }

    /// Query a count of supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[cfg(feature = "VK_KHR_surface")]
    #[inline]
    pub fn surface_present_mode_count(&self, surface: &crate::Surface) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_physical_device_surface_present_modes_khr(
                self as *const _ as _,
                surface as *const _ as _,
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Query supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[implements("VK_KHR_surface")]
    #[inline]
    pub fn surface_present_modes(
        &self,
        surface: &crate::Surface,
        sink: &mut [crate::PresentMode],
    ) -> crate::Result<(u32, EnumerationResult)> {
        let mut n = sink.len() as _;
        let r = unsafe {
            crate::vkfn::get_physical_device_surface_present_modes_khr(
                self as *const _ as _,
                surface as *const _ as _,
                &mut n,
                sink.as_mut_ptr() as _,
            )
        };

        if r == VK_SUCCESS {
            return Ok((n, EnumerationResult::Complete));
        }
        if r == VK_INCOMPLETE {
            return Ok((n, EnumerationResult::Incomplete));
        }

        Err(r)
    }

    /// Query supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[cfg(feature = "VK_KHR_surface")]
    #[cfg(feature = "alloc")]
    pub fn surface_present_modes_alloc(&self, surface: &crate::Surface) -> crate::Result<Vec<crate::PresentMode>> {
        let n = self.surface_present_mode_count(surface)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        // ここでは全部とれる(Incompleteの対処は不要)
        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.surface_present_modes(surface, &mut xs)?;

        Ok(xs)
    }
}
