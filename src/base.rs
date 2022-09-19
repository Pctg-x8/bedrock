//! Vulkan Base Objects(Instance/PhysicalDevice)

use crate::vk::*;
use crate::VkHandle;
use crate::VkObject;
#[cfg(feature = "Implements")]
use crate::{
    fnconv::FnTransmute,
    vkresolve::{Resolver, ResolverInterface},
    ImageFlags, ImageUsage, VkResultHandler,
};
#[cfg(all(feature = "Implements", feature = "VK_KHR_surface"))]
use crate::{PresentMode, Surface};
use std::ops::*;

#[cfg(feature = "Multithreaded")]
struct LazyCellReadRef<'d, T>(::std::sync::RwLockReadGuard<'d, Option<T>>);
#[cfg(feature = "Multithreaded")]
impl<'d, T> ::std::ops::Deref for LazyCellReadRef<'d, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.as_ref().unwrap()
    }
}

/// Opaque handle to a instance object
#[derive(VkHandle)]
pub struct InstanceObject(VkInstance);
impl VkObject for InstanceObject {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_INSTANCE;
}
unsafe impl Sync for InstanceObject {}
unsafe impl Send for InstanceObject {}
#[cfg(feature = "Implements")]
impl Drop for InstanceObject {
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_instance(self.0, std::ptr::null());
        }
    }
}
impl Instance for InstanceObject {}

/// Opaque handle to a physical device object
///
/// ## Platform Dependent Methods: Presentation Support checking functions
///
/// * `xlib_presentation_support(&self, queue_family: u32, display: *mut x11::xlib::Display, visual: x11::xlib::VisualID) -> bool`: VK_KHR_xlib_surface
/// * `xcb_presentation_support(&self, queue_family: u32, connection: *mut xcb::ffi::xcb_connection_t, visual: xcb::ffi::xcb_visualid_t) -> bool`: VK_KHR_xcb_surface
/// * `wayland_presentation_support(&self, queue_family: u32, display: *mut wayland_client::sys::wl_display) -> bool`: VK_KHR_wayland_surface
/// * `win32_presentation_support(&self, queue_family: u32) -> bool`: VK_KHR_win32_surface
/// * Methods for Android and Mir surfaces are not implemented
#[derive(VkHandle)]
pub struct PhysicalDeviceObject<Owner: Instance>(VkPhysicalDevice, Owner);
impl<Instance: crate::Instance> VkObject for PhysicalDeviceObject<Instance> {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_PHYSICAL_DEVICE;
}
unsafe impl<Owner: Instance + Sync> Sync for PhysicalDeviceObject<Owner> {}
unsafe impl<Owner: Instance + Send> Send for PhysicalDeviceObject<Owner> {}
impl<Owner: Instance> PhysicalDevice for PhysicalDeviceObject<Owner> {}
impl<Owner: Instance> InstanceChild for PhysicalDeviceObject<Owner> {
    type ConcreteInstance = Owner;

    fn instance(&self) -> &Owner {
        &self.1
    }
}
impl<Owner: Instance> InstanceChildTransferrable for PhysicalDeviceObject<Owner> {
    fn transfer_instance(self) -> Self::ConcreteInstance {
        self.1
    }
}
impl<Instance: crate::Instance + Clone> PhysicalDeviceObject<&'_ Instance> {
    /// Clones parent reference
    #[inline]
    pub fn clone_parent(self) -> PhysicalDeviceObject<Instance> {
        PhysicalDeviceObject(self.0, self.1.clone())
    }
}

pub struct IterPhysicalDevices<'i, Source: Instance + 'i>(Vec<VkPhysicalDevice>, usize, &'i Source);
impl<'i, Source: Instance + 'i> Iterator for IterPhysicalDevices<'i, Source> {
    type Item = PhysicalDeviceObject<&'i Source>;

    fn next(&mut self) -> Option<PhysicalDeviceObject<&'i Source>> {
        if self.0.len() <= self.1 {
            None
        } else {
            self.1 += 1;
            Some(PhysicalDeviceObject(self.0[self.1 - 1], self.2))
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len(), Some(self.0.len()))
    }
}
impl<'i, Source: Instance + 'i> ExactSizeIterator for IterPhysicalDevices<'i, Source> {
    fn len(&self) -> usize {
        self.0.len()
    }
    // fn is_empty(&self) -> bool { self.0.len() <= self.1 }
}
impl<'i, Source: Instance + 'i> DoubleEndedIterator for IterPhysicalDevices<'i, Source> {
    fn next_back(&mut self) -> Option<PhysicalDeviceObject<&'i Source>> {
        if self.0.len() <= self.1 {
            None
        } else {
            self.0.pop().map(|p| PhysicalDeviceObject(p, self.2))
        }
    }
}

/// Builder object for constructing a `Instance`
pub struct InstanceBuilder {
    _app_name: std::ffi::CString,
    _engine_name: std::ffi::CString,
    extensions: Vec<std::ffi::CString>,
    layers: Vec<std::ffi::CString>,
    ext_structures: Vec<Box<dyn std::any::Any>>,
    appinfo: VkApplicationInfo,
    cinfo: VkInstanceCreateInfo,
}
impl InstanceBuilder {
    pub fn new(
        app_name: &str,
        app_version: (u32, u32, u32),
        engine_name: &str,
        engine_version: (u32, u32, u32),
    ) -> Self {
        InstanceBuilder {
            _app_name: std::ffi::CString::new(app_name).unwrap(),
            _engine_name: std::ffi::CString::new(engine_name).unwrap(),
            extensions: Vec::new(),
            layers: Vec::new(),
            ext_structures: Vec::new(),
            appinfo: VkApplicationInfo {
                applicationVersion: VK_MAKE_VERSION!(app_version.0, app_version.1, app_version.2),
                engineVersion: VK_MAKE_VERSION!(engine_version.0, engine_version.1, engine_version.2),
                ..Default::default()
            },
            cinfo: VkInstanceCreateInfo { ..Default::default() },
        }
    }
    pub fn set_api_version(&mut self, major: u32, minor: u32, patch: u32) -> &mut Self {
        self.appinfo.apiVersion = VK_MAKE_VERSION!(major, minor, patch);
        self
    }
    pub fn add_extension(&mut self, extension: &str) -> &mut Self {
        self.extensions.push(std::ffi::CString::new(extension).unwrap());
        self
    }
    pub fn add_extensions<'s, Extensions: IntoIterator<Item = &'s str>>(
        &mut self,
        extensions: Extensions,
    ) -> &mut Self {
        for ex in extensions {
            self.add_extension(ex);
        }
        self
    }
    pub fn add_layer(&mut self, layer: &str) -> &mut Self {
        self.layers.push(std::ffi::CString::new(layer).unwrap());
        self
    }
    pub fn add_layers<'s, Layers: IntoIterator<Item = &'s str>>(&mut self, layers: Layers) -> &mut Self {
        for l in layers {
            self.add_layer(l);
        }
        self
    }

    pub fn add_ext_structure<S: crate::ext::VulkanStructure + 'static>(&mut self, ext: S) -> &mut Self {
        self.ext_structures.push(Box::new(ext) as _);
        self
    }

    pub fn create_info(&self) -> &VkInstanceCreateInfo {
        &self.cinfo
    }
    pub fn application_info(&self) -> &VkApplicationInfo {
        &self.appinfo
    }

    /// Create a new Vulkan instance
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
    pub fn create(&mut self) -> crate::Result<InstanceObject> {
        // construct ext chains
        if !self.ext_structures.is_empty() {
            for n in 0..self.ext_structures.len() - 1 {
                let next_ptr = self.ext_structures[n + 1].as_ref() as *const _ as _;
                let current = unsafe {
                    &mut *(self.ext_structures[n].as_mut() as *mut _ as *mut crate::ext::GenericVulkanStructure)
                };

                current.pNext = next_ptr;
            }
            unsafe {
                let mut last_ptr = &mut *(self.ext_structures.last_mut().unwrap().as_mut() as *mut _
                    as *mut crate::ext::GenericVulkanStructure);
                last_ptr.pNext = std::ptr::null();
            }
        }
        self.cinfo.pNext = self
            .ext_structures
            .first()
            .map_or_else(std::ptr::null, |s| s.as_ref() as *const _ as _);

        let layers: Vec<_> = self.layers.iter().map(|x| x.as_ptr()).collect();
        let extensions: Vec<_> = self.extensions.iter().map(|x| x.as_ptr()).collect();
        self.appinfo.pApplicationName = self._app_name.as_ptr();
        self.appinfo.pEngineName = self._engine_name.as_ptr();
        self.cinfo.enabledLayerCount = layers.len() as _;
        self.cinfo.enabledExtensionCount = extensions.len() as _;
        self.cinfo.ppEnabledLayerNames = if layers.is_empty() { 0 as _ } else { layers.as_ptr() };
        self.cinfo.ppEnabledExtensionNames = if extensions.is_empty() {
            0 as _
        } else {
            extensions.as_ptr()
        };
        self.cinfo.pApplicationInfo = &self.appinfo;
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_instance(&self.cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(|_| InstanceObject(h))
        }
    }
}

/// Returns up to all of global layer properties
/// # Failures
/// On failure, this command returns
///
/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
#[cfg(feature = "Implements")]
pub fn enumerate_layer_properties() -> crate::Result<Vec<VkLayerProperties>> {
    let mut n = 0;
    unsafe {
        Resolver::get()
            .enumerate_instance_layer_properties(&mut n, std::ptr::null_mut())
            .into_result()?;
    }
    let mut v = Vec::with_capacity(n as _);
    unsafe { v.set_len(n as _) };
    unsafe {
        Resolver::get()
            .enumerate_instance_layer_properties(&mut n, v.as_mut_ptr())
            .into_result()
            .map(|_| v)
    }
}

/// Returns up to all of global extension properties
/// # Failures
/// On failure, this command returns
///
/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// * `VK_ERROR_LAYER_NOT_PRESENT`
#[cfg(feature = "Implements")]
pub fn enumerate_extension_properties(layer_name: Option<&str>) -> crate::Result<Vec<VkExtensionProperties>> {
    let cn = layer_name.map(|s| std::ffi::CString::new(s).unwrap());
    let cptr = cn.as_ref().map(|s| s.as_ptr()).unwrap_or_else(std::ptr::null);
    unsafe {
        let mut n = 0;
        Resolver::get()
            .enumerate_instance_extension_properties(cptr, &mut n, std::ptr::null_mut())
            .into_result()?;
        let mut v = Vec::with_capacity(n as _);
        v.set_len(n as _);
        Resolver::get()
            .enumerate_instance_extension_properties(cptr, &mut n, v.as_mut_ptr())
            .into_result()
            .map(|_| v)
    }
}

/// A Vulkan Instance interface
pub trait Instance: VkHandle<Handle = VkInstance> {
    /// Return a function pointer for a command
    /// # Failures
    /// If function is not provided by instance or `name` is empty, returns `None`
    #[cfg(feature = "Implements")]
    fn extra_procedure<F: FnTransmute>(&self, name: &str) -> Option<F> {
        if name.is_empty() {
            return None;
        }

        unsafe {
            let fn_cstr = std::ffi::CString::new(name).unwrap();
            Resolver::get()
                .get_instance_proc_addr(self.native_ptr(), fn_cstr.as_ptr())
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
    #[cfg(feature = "Implements")]
    fn enumerate_physical_devices(&self) -> crate::Result<Vec<PhysicalDeviceObject<&Self>>>
    where
        Self: Sized,
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
    #[cfg(feature = "Implements")]
    fn iter_physical_devices(&self) -> crate::Result<IterPhysicalDevices<Self>>
    where
        Self: Sized,
    {
        unsafe {
            let mut n = 0;
            Resolver::get()
                .enumerate_physical_devices(self.native_ptr(), &mut n, std::ptr::null_mut())
                .into_result()?;
            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            Resolver::get()
                .enumerate_physical_devices(self.native_ptr(), &mut n, v.as_mut_ptr())
                .into_result()
                .map(move |_| IterPhysicalDevices(v, 0, self))
        }
    }

    #[cfg(feature = "Implements")]
    unsafe fn create_descriptor_update_template(
        &self,
        device: VkDevice,
        info: &VkDescriptorUpdateTemplateCreateInfo,
        alloc: *const VkAllocationCallbacks,
        handle: *mut VkDescriptorUpdateTemplate,
    ) -> VkResult {
        let f: PFN_vkCreateDescriptorUpdateTemplate = self
            .extra_procedure("vkCreateDescriptorUpdateTemplate")
            .expect("No vkCreateDescriptorUpdateTemplate found");
        (f)(device, info, alloc, handle)
    }

    #[cfg(feature = "Implements")]
    unsafe fn destroy_descriptor_update_template(
        &self,
        device: VkDevice,
        handle: VkDescriptorUpdateTemplate,
        alloc: *const VkAllocationCallbacks,
    ) {
        let f: PFN_vkDestroyDescriptorUpdateTemplate = self
            .extra_procedure("vkDestroyDescriptorUpdateTemplate")
            .expect("No vkDestroyDescriptorUpdateTemplate");
        (f)(device, handle, alloc)
    }

    /// Register a debug report callback
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[cfg(feature = "VK_EXT_debug_report")]
    #[cfg(feature = "Implements")]
    fn new_debug_report_callback(
        self,
        flags: VkDebugReportFlagsEXT,
        callback: PFN_vkDebugReportCallbackEXT,
    ) -> crate::Result<crate::DebugReportCallbackObject<Self>>
    where
        Self: Sized,
    {
        let ctor: PFN_vkCreateDebugReportCallbackEXT = self
            .extra_procedure("vkCreateDebugReportCallbackEXT")
            .expect("Requiring vkCreateDebugReportCallbackEXT function");
        let dtor: PFN_vkDestroyDebugReportCallbackEXT = self
            .extra_procedure("vkDestroyDebugReportCallbackEXT")
            .expect("Requiring vkDestroyDebugReportCallbackEXT function");
        let s = VkDebugReportCallbackCreateInfoEXT {
            flags,
            pfnCallback: callback,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        ctor(self.native_ptr(), &s, std::ptr::null(), &mut h)
            .into_result()
            .map(|_| crate::DebugReportCallbackObject(h, self, dtor))
    }

    /// Inject its own messages into the debug stream
    #[cfg(feature = "VK_EXT_debug_report")]
    #[cfg(feature = "Implements")]
    fn debug_message(
        &self,
        flags: VkDebugReportFlagsEXT,
        object_type: crate::DebugReportObjectType,
        object: u64,
        location: libc::size_t,
        message_count: i32,
        layer_prefix: &str,
        message: &str,
    ) {
        let (lp, msg) = (
            std::ffi::CString::new(layer_prefix).unwrap(),
            std::ffi::CString::new(message).unwrap(),
        );
        let msgf: PFN_vkDebugReportMessageEXT = self
            .extra_procedure("vkDebugReportMessageEXT")
            .expect("Requiring vkDebugReportMessageEXT function");
        msgf(
            self.native_ptr(),
            flags,
            object_type as _,
            object,
            location,
            message_count,
            lp.as_ptr(),
            msg.as_ptr(),
        );
    }
}
DerefContainerBracketImpl!(for Instance {});

/// A PhysicalDevice interface
pub trait PhysicalDevice: VkHandle<Handle = VkPhysicalDevice> + InstanceChild {
    /// Returns properties of available physical device layers
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn enumerate_layer_properties(&self) -> crate::Result<Vec<VkLayerProperties>> {
        let mut count = 0;
        unsafe {
            Resolver::get()
                .enumerate_device_layer_properties(self.native_ptr(), &mut count, std::ptr::null_mut())
                .into_result()?;
        }
        let mut v = Vec::with_capacity(count as _);
        unsafe {
            v.set_len(count as _);
        }
        unsafe {
            Resolver::get()
                .enumerate_device_layer_properties(self.native_ptr(), &mut count, v.as_mut_ptr())
                .into_result()
                .map(move |_| v)
        }
    }

    /// Returns properties of available physical device extensions
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_LAYER_NOT_PRESENT`
    #[cfg(feature = "Implements")]
    fn enumerate_extension_properties(&self, layer_name: Option<&str>) -> crate::Result<Vec<VkExtensionProperties>> {
        let cn = layer_name.map(|s| std::ffi::CString::new(s).unwrap());
        let cptr = cn.as_ref().map(|s| s.as_ptr()).unwrap_or_else(std::ptr::null);
        unsafe {
            let mut n = 0;
            Resolver::get()
                .enumerate_device_extension_properties(self.native_ptr(), cptr, &mut n, std::ptr::null_mut())
                .into_result()?;
            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            Resolver::get()
                .enumerate_device_extension_properties(self.native_ptr(), cptr, &mut n, v.as_mut_ptr())
                .into_result()
                .map(move |_| v)
        }
    }

    /// Reports capabilities of a physical device.
    #[cfg(feature = "Implements")]
    fn features(&self) -> VkPhysicalDeviceFeatures {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get().get_physical_device_features(self.native_ptr(), p.as_mut_ptr());

            p.assume_init()
        }
    }

    /// Lists physical device's format capabilities
    #[cfg(feature = "Implements")]
    fn format_properties(&self, format: VkFormat) -> VkFormatProperties {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get().get_physical_device_format_properties(self.native_ptr(), format, p.as_mut_ptr());

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
    #[cfg(feature = "Implements")]
    fn image_format_properties(
        &self,
        format: VkFormat,
        itype: VkImageType,
        tiling: VkImageTiling,
        usage: ImageUsage,
        flags: ImageFlags,
    ) -> crate::Result<VkImageFormatProperties> {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get()
                .get_physical_device_image_format_properties(
                    self.native_ptr(),
                    format,
                    itype,
                    tiling,
                    usage.0,
                    flags.0,
                    p.as_mut_ptr(),
                )
                .into_result()
                .map(|_| p.assume_init())
        }
    }

    /// Returns properties of a physical device
    #[cfg(feature = "Implements")]
    fn properties(&self) -> VkPhysicalDeviceProperties {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get().get_physical_device_properties(self.native_ptr(), p.as_mut_ptr());

            p.assume_init()
        }
    }

    /// Reports properties of the queues of the specified physical device
    #[cfg(feature = "Implements")]
    fn queue_family_properties(&self) -> QueueFamilies {
        unsafe {
            let mut n = 0;
            Resolver::get().get_physical_device_queue_family_properties(
                self.native_ptr(),
                &mut n,
                std::ptr::null_mut(),
            );
            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            Resolver::get().get_physical_device_queue_family_properties(self.native_ptr(), &mut n, v.as_mut_ptr());

            QueueFamilies(v)
        }
    }

    /// Reports memory information for the specified physical device
    #[cfg(feature = "Implements")]
    fn memory_properties(&self) -> MemoryProperties {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get().get_physical_device_memory_properties(self.native_ptr(), p.as_mut_ptr());

            MemoryProperties(p.assume_init())
        }
    }

    /// Retrieve properties of an image format applied to sparse images
    #[cfg(feature = "Implements")]
    fn sparse_image_format_properties(
        &self,
        format: VkFormat,
        itype: VkImageType,
        samples: VkSampleCountFlags,
        usage: ImageUsage,
        tiling: VkImageTiling,
    ) -> Vec<VkSparseImageFormatProperties> {
        unsafe {
            let mut n = 0;
            Resolver::get().get_physical_device_sparse_image_format_properties(
                self.native_ptr(),
                format,
                itype,
                samples,
                usage.0,
                tiling,
                &mut n,
                std::ptr::null_mut(),
            );
            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            Resolver::get().get_physical_device_sparse_image_format_properties(
                self.native_ptr(),
                format,
                itype,
                samples,
                usage.0,
                tiling,
                &mut n,
                v.as_mut_ptr(),
            );

            v
        }
    }

    #[cfg(feature = "VK_EXT_sample_locations")]
    #[cfg(feature = "Implements")]
    fn multisample_properties(&self, samples: VkSampleCountFlags) -> VkMultisamplePropertiesEXT {
        let mut r = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get().get_physical_device_multisample_properties_ext(self.native_ptr(), samples, r.as_mut_ptr());

            r.assume_init()
        }
    }

    /// Function for querying external fence handle capabilities
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    #[cfg(feature = "Implements")]
    fn external_fence_properties(&self, handle_type: crate::ExternalFenceFdType) -> ExternalFenceProperties {
        let mut r = std::mem::MaybeUninit::uninit();
        unsafe {
            *r.as_mut_ptr() = Default::default();
        }
        let f: PFN_vkGetPhysicalDeviceExternalFenceProperties = self
            .1
            .extra_procedure("vkGetPhysicalDeviceExternalFenceProperties")
            .expect("no vkGetPhysicalDeviceExternalFenceProperties exported?");
        (f)(
            self.native_ptr(),
            &VkPhysicalDeviceExternalFenceInfo {
                handleType: handle_type as _,
                ..Default::default()
            },
            r.as_mut_ptr(),
        );
        unsafe { From::from(r.assume_init()) }
    }

    /// Query if presentation is supported
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[cfg(all(feature = "Implements", feature = "VK_KHR_surface"))]
    fn surface_support(&self, queue_family: u32, surface: &impl Surface) -> crate::Result<bool> {
        let mut f = false as _;
        unsafe {
            Resolver::get()
                .get_physical_device_surface_support_khr(self.native_ptr(), queue_family, surface.native_ptr(), &mut f)
                .into_result()
                .map(|_| f != 0)
        }
    }

    /// Query surface capabilities
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[cfg(all(feature = "Implements", feature = "VK_KHR_surface"))]
    fn surface_capabilities(&self, surface: &impl Surface) -> crate::Result<VkSurfaceCapabilitiesKHR> {
        let mut s = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get()
                .get_physical_device_surface_capabilities_khr(self.native_ptr(), surface.native_ptr(), s.as_mut_ptr())
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
    #[cfg(all(feature = "Implements", feature = "VK_KHR_surface"))]
    fn surface_formats(&self, surface: &impl Surface) -> crate::Result<Vec<VkSurfaceFormatKHR>> {
        unsafe {
            let mut n = 0;
            Resolver::get()
                .get_physical_device_surface_formats_khr(
                    self.native_ptr(),
                    surface.native_ptr(),
                    &mut n,
                    std::ptr::null_mut(),
                )
                .into_result()?;
            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            Resolver::get()
                .get_physical_device_surface_formats_khr(
                    self.native_ptr(),
                    surface.native_ptr(),
                    &mut n,
                    v.as_mut_ptr(),
                )
                .into_result()?;

            Ok(v)
        }
    }

    /// Query supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[cfg(all(feature = "Implements", feature = "VK_KHR_surface"))]
    fn surface_present_modes(&self, surface: &impl Surface) -> crate::Result<Vec<PresentMode>> {
        unsafe {
            let mut n = 0;
            Resolver::get()
                .get_physical_device_surface_present_modes_khr(
                    self.native_ptr(),
                    surface.native_ptr(),
                    &mut n,
                    std::ptr::null_mut(),
                )
                .into_result()?;
            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            Resolver::get()
                .get_physical_device_surface_present_modes_khr(
                    self.native_ptr(),
                    surface.native_ptr(),
                    &mut n,
                    v.as_mut_ptr(),
                )
                .into_result()
                .map(|_| std::mem::transmute(v))
        }
    }

    /// [feature = "VK_KHR_xlib_surface"] Query physical device for presentation to X11 server using Xlib
    #[cfg(feature = "VK_KHR_xlib_surface")]
    #[cfg(feature = "Implements")]
    fn xlib_presentation_support(
        &self,
        queue_family: u32,
        display: *mut x11::xlib::Display,
        visual: x11::xlib::VisualID,
    ) -> bool {
        unsafe {
            Resolver::get().get_physical_device_xlib_presentation_support_khr(
                self.native_ptr(),
                queue_family,
                display,
                visual,
            ) != 0
        }
    }

    /// [feature = "VK_KHR_xcb_surface"] Query physical device for presentation to X11 server using XCB
    #[cfg(feature = "VK_KHR_xcb_surface")]
    #[cfg(feature = "Implements")]
    fn xcb_presentation_support(
        &self,
        queue_family: u32,
        connection: *mut xcb::ffi::xcb_connection_t,
        visual: xcb::ffi::xcb_visualid_t,
    ) -> bool {
        unsafe {
            Resolver::get().get_physical_device_xcb_presentation_support_khr(
                self.native_ptr(),
                queue_family,
                connection,
                visual,
            ) != 0
        }
    }

    /// [feature = "VK_KHR_wayland_surface"] Query physical device for presentation to Wayland
    #[cfg(feature = "VK_KHR_wayland_surface")]
    #[cfg(feature = "Implements")]
    fn wayland_presentation_support(&self, queue_family: u32, display: *mut wayland_client::sys::wl_display) -> bool {
        unsafe {
            Resolver::get().get_physical_device_wayland_presentation_support_khr(
                self.native_ptr(),
                queue_family,
                display,
            ) != 0
        }
    }

    /// [feature = "VK_KHR_win32_surface"] Query queue family support for presentation on a Win32 display
    #[cfg(feature = "VK_KHR_win32_surface")]
    #[cfg(feature = "Implements")]
    fn win32_presentation_support(&self, queue_family: u32) -> bool {
        unsafe {
            Resolver::get().get_physical_device_win32_presentation_support_khr(self.native_ptr(), queue_family) != 0
        }
    }

    /// Create a `Surface` object for an X11 window, using the Xlib client-side library
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "VK_KHR_xlib_surface")]
    fn new_surface_xlib(
        self,
        display: *mut x11::xlib::Display,
        window: x11::xlib::Window,
    ) -> crate::Result<crate::SurfaceObject<Self>>
    where
        Self: Sized,
    {
        let cinfo = VkXlibSurfaceCreateInfoKHR {
            dpy: display,
            window,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_xlib_surface_khr(self.instance().native_ptr(), &cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(|_| crate::SurfaceObject(h, self))
        }
    }

    /// Create a `Surface` object for a X11 window, using the XCB client-side library
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "VK_KHR_xcb_surface")]
    fn new_surface_xcb(
        self,
        connection: *mut xcb::ffi::xcb_connection_t,
        window: xcb::ffi::xcb_window_t,
    ) -> crate::Result<crate::SurfaceObject<Self>>
    where
        Self: Sized,
    {
        let cinfo = VkXcbSurfaceCreateInfoKHR {
            connection,
            window,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_xcb_surface_khr(self.instance().native_ptr(), &cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(|_| crate::SurfaceObject(h, self))
        }
    }

    /// Create a `Surface` object for a Wayland window
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "VK_KHR_wayland_surface")]
    fn new_surface_wayland(
        self,
        display: *mut wayland_client::sys::wl_display,
        surface: *mut wayland_client::sys::wl_proxy,
    ) -> crate::Result<crate::SurfaceObject<Self>>
    where
        Self: Sized,
    {
        let cinfo = VkWaylandSurfaceCreateInfoKHR {
            display,
            surface,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_wayland_surface_khr(self.instance().native_ptr(), &cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(|_| crate::SurfaceObject(h, self))
        }
    }

    /// Create a `Surface` object for an Android native window
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "VK_KHR_android_surface")]
    fn new_surface_android(self, window: *mut android::ANativeWindow) -> crate::Result<crate::SurfaceObject<Self>>
    where
        Self: Sized,
    {
        let cinfo = VkAndroidSurfaceCreateInfoKHR {
            window,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_android_surface_khr(self.instance().native_ptr(), &cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(|_| crate::SurfaceObject(h, self))
        }
    }

    /// Create a `Surface` object for an Win32 native window
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "VK_KHR_win32_surface")]
    fn new_surface_win32(
        self,
        hinstance: winapi::shared::minwindef::HINSTANCE,
        hwnd: winapi::shared::windef::HWND,
    ) -> crate::Result<crate::SurfaceObject<Self::ConcreteInstance>>
    where
        Self: Sized + InstanceChildTransferrable,
    {
        let cinfo = VkWin32SurfaceCreateInfoKHR {
            hinstance,
            hwnd,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_win32_surface_khr(self.instance().native_ptr(), &cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(|_| crate::SurfaceObject(h, self.transfer_instance()))
        }
    }

    /// Create a `Surface` object for an macOS native window
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "VK_MVK_macos_surface")]
    fn new_surface_macos(self, view_ptr: *const libc::c_void) -> crate::Result<crate::SurfaceObject<Self::ConcreteInstance>>
    where
        Self: Sized + InstanceChildTransferrable,
    {
        let cinfo = VkMacOSSurfaceCreateInfoMVK {
            pView: view_ptr,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_macos_surface_mvk(self.instance().native_ptr(), &cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(|_| crate::SurfaceObject(h, self.transfer_instance()))
        }
    }

    /// Query the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "VK_KHR_display")]
    #[cfg(feature = "Implements")]
    fn display_mode_properties(&self, display: VkDisplayKHR) -> crate::Result<Vec<VkDisplayModePropertiesKHR>> {
        unsafe {
            let mut n = 0;
            Resolver::get()
                .get_display_mode_properties_khr(self.native_ptr(), display, &mut n, std::ptr::null_mut())
                .into_result()?;
            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            Resolver::get()
                .get_display_mode_properties_khr(self.native_ptr(), display, &mut n, v.as_mut_ptr())
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
    #[cfg(feature = "VK_KHR_display")]
    #[cfg(feature = "Implements")]
    fn new_display_mode(
        &self,
        display: VkDisplayKHR,
        region: VkExtent2D,
        refresh_rate: u32,
    ) -> crate::Result<VkDisplayModeKHR> {
        let cinfo = VkDisplayModeCreateInfoKHR {
            parameters: VkDisplayModeParametersKHR {
                visibleRegion: region,
                refreshRate: refresh_rate,
            },
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_display_mode_khr(self.native_ptr(), display, &cinfo, std::ptr::null(), &mut h)
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
    #[cfg(feature = "VK_KHR_display")]
    #[cfg(feature = "Implements")]
    fn display_plane_capabilities(
        &self,
        mode: VkDisplayModeKHR,
        plane_index: u32,
    ) -> crate::Result<VkDisplayPlaneCapabilitiesKHR> {
        let mut s = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get()
                .get_display_plane_capabilities_khr(self.native_ptr(), mode, plane_index, s.as_mut_ptr())
                .into_result()
                .map(move |_| s.assume_init())
        }
    }

    /// Query information about the available displays.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[cfg(feature = "VK_KHR_display")]
    #[cfg(feature = "Implements")]
    fn display_properties(&self) -> crate::Result<Vec<DisplayProperties>> {
        unsafe {
            let mut n = 0;
            Resolver::get()
                .get_physical_device_display_properties_khr(self.native_ptr(), &mut n, std::ptr::null_mut())
                .into_result()?;
            let mut v = Vec::with_capacity(n as usize);
            v.set_len(n as usize);
            Resolver::get()
                .get_physical_device_display_properties_khr(self.native_ptr(), &mut n, v.as_mut_ptr() as *mut _)
                .into_result()
                .map(move |_| v)
        }
    }

    /// Query the plane properties.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[cfg(feature = "VK_KHR_display")]
    #[cfg(feature = "Implements")]
    fn display_plane_properties(&self) -> crate::Result<Vec<DisplayPlaneProperties>> {
        unsafe {
            let mut n = 0;
            Resolver::get()
                .get_physical_device_display_plane_properties_khr(self.native_ptr(), &mut n, std::ptr::null_mut())
                .into_result()?;
            let mut v = Vec::with_capacity(n as usize);
            v.set_len(n as usize);
            Resolver::get()
                .get_physical_device_display_plane_properties_khr(self.native_ptr(), &mut n, v.as_mut_ptr() as *mut _)
                .into_result()
                .map(move |_| v)
        }
    }

    /// Query the list of displays a plane supports.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[cfg(feature = "VK_KHR_display")]
    #[cfg(feature = "Implements")]
    fn display_plane_supported_displays(&self, plane_index: u32) -> crate::Result<Vec<Display>> {
        unsafe {
            let mut n = 0;
            Resolver::get()
                .get_display_plane_supported_displays_khr(self.native_ptr(), plane_index, &mut n, std::ptr::null_mut())
                .into_result()?;
            let mut v = Vec::with_capacity(n as usize);
            v.set_len(n as usize);
            Resolver::get()
                .get_display_plane_supported_displays_khr(
                    self.native_ptr(),
                    plane_index,
                    &mut n,
                    v.as_mut_ptr() as *mut _,
                )
                .into_result()
                .map(move |_| v)
        }
    }

    /// Create a display mode
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[cfg(feature = "VK_KHR_display")]
    #[cfg(feature = "Implements")]
    fn create_display_mode(&self, display: &Display, params: VkDisplayModeParametersKHR) -> crate::Result<DisplayMode> {
        unsafe {
            let cinfo = VkDisplayModeCreateInfoKHR {
                parameters: params,
                ..Default::default()
            };
            let mut h = VK_NULL_HANDLE as _;
            Resolver::get()
                .create_display_mode_khr(
                    self.native_ptr(),
                    display.native_ptr(),
                    &cinfo,
                    std::ptr::null(),
                    &mut h,
                )
                .into_result()
                .map(move |_| DisplayMode(h))
        }
    }

    /// Query the VkDisplayKHR corresponding to an X11 RandR Output
    /// # Failures
    /// On failure, this command returns
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    #[cfg(feature = "Implements")]
    fn get_randr_output_display(
        &self,
        dpy: *mut x11::xlib::Display,
        rr_output: x11::xrandr::RROutput,
    ) -> crate::Result<Display> {
        let fp: PFN_vkGetRandROutputDisplayEXT = self
            .parent()
            .extra_procedure("vkGetRandROutputDisplayEXT")
            .expect("no vkGetRandROutputDisplayEXT exported?");
        let mut d = std::mem::MaybeUninit::uninit();
        fp(self.native_ptr(), dpy, rr_output, d.as_mut_ptr())
            .into_result()
            .map(move |_| unsafe { Display(d.assume_init()) })
    }

    /// Create a `Surface` object representing a display plane and mode
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    #[cfg(feature = "VK_KHR_display")]
    #[cfg(feature = "VK_KHR_surface")]
    #[allow(clippy::too_many_arguments)]
    fn new_display_plane(
        self,
        mode: &DisplayMode,
        plane_index: u32,
        plane_stack_index: u32,
        transform: crate::SurfaceTransform,
        global_alpha: f32,
        alpha_mode: DisplayPlaneAlpha,
        extent: VkExtent2D,
    ) -> crate::Result<crate::SurfaceObject<Self::ConcreteInstance>>
    where
        Self: Sized + InstanceChildTransferrable,
    {
        let cinfo = VkDisplaySurfaceCreateInfoKHR {
            displayMode: mode.0,
            planeIndex: plane_index,
            planeStackIndex: plane_stack_index,
            transform: transform as _,
            globalAlpha: global_alpha,
            alphaMode: alpha_mode as _,
            imageExtent: extent,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;

        unsafe {
            Resolver::get()
                .create_display_plane_surface_khr(self.instance().native_ptr(), &cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(|_| crate::SurfaceObject(h, self.transfer_instance()))
        }
    }

    #[cfg(all(feature = "VK_KHR_get_surface_capabilities2", feature = "Implements"))]
    fn surface_capabilities2(
        &self,
        surface_info: &VkPhysicalDeviceSurfaceInfo2KHR,
    ) -> crate::Result<VkSurfaceCapabilities2KHR> {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::Resolver::get()
                .get_physical_device_surface_capabilities2_khr(self.native_ptr(), surface_info, p.as_mut_ptr())
                .into_result()
                .map(move |_| p.assume_init())
        }
    }

    #[cfg(all(feature = "VK_KHR_get_physical_device_properties2", feature = "Implements"))]
    /// Returns properties of a physical device
    fn properties2(&self) -> VkPhysicalDeviceProperties2 {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::Resolver::get().get_physical_device_properties2(self.native_ptr(), p.as_mut_ptr());
            p.assume_init()
        }
    }
}
DerefContainerBracketImpl!(for PhysicalDevice {});

pub trait InstanceChild {
    type ConcreteInstance: Instance;

    fn instance(&self) -> &Self::ConcreteInstance;
}
DerefContainerBracketImpl!(for InstanceChild {
    type ConcreteInstance = T::ConcreteInstance;

    fn instance(&self) -> &Self::ConcreteInstance { T::instance(self) }
});

pub trait InstanceChildTransferrable: InstanceChild {
    fn transfer_instance(self) -> Self::ConcreteInstance;
}
impl<T> InstanceChildTransferrable for &'_ T
where
    T: InstanceChild,
    T::ConcreteInstance: Clone,
{
    fn transfer_instance(self) -> Self::ConcreteInstance {
        self.instance().clone()
    }
}

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
mod external_fence_capabilities_khr {
    use crate::vk::*;

    #[repr(transparent)]
    /// Structure describing supported external fence handle features
    pub struct ExternalFenceProperties(VkExternalFenceProperties);
    impl From<VkExternalFenceProperties> for ExternalFenceProperties {
        fn from(v: VkExternalFenceProperties) -> Self {
            Self(v)
        }
    }
    impl From<ExternalFenceProperties> for VkExternalFenceProperties {
        fn from(v: ExternalFenceProperties) -> Self {
            v.0
        }
    }
    impl AsRef<VkExternalFenceProperties> for ExternalFenceProperties {
        fn as_ref(&self) -> &VkExternalFenceProperties {
            &self.0
        }
    }
    impl std::ops::Deref for ExternalFenceProperties {
        type Target = VkExternalFenceProperties;
        fn deref(&self) -> &VkExternalFenceProperties {
            &self.0
        }
    }
    impl ExternalFenceProperties {
        pub fn export_from_imported_handle_types(&self) -> crate::ExternalFenceHandleTypes {
            crate::ExternalFenceHandleTypes(self.0.exportFromImportedHandleTypes)
        }
        pub fn compatible_handle_types(&self) -> crate::ExternalFenceHandleTypes {
            crate::ExternalFenceHandleTypes(self.0.compatibleHandleTypes)
        }
        pub fn features(&self) -> ExternalFenceFeatureFlags {
            ExternalFenceFeatureFlags(self.0.externalFenceFeatures)
        }
    }
    impl std::fmt::Debug for ExternalFenceProperties {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_struct("ExternalFenceProperties")
                .field("pNext", &self.pNext)
                .field(
                    "export_from_imported_handle_types",
                    &self.export_from_imported_handle_types(),
                )
                .field("compatible_handle_types", &self.compatible_handle_types())
                .field("features", &self.features())
                .finish()
        }
    }
    #[repr(transparent)]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    /// Bitfield describing features of an external fence handle type
    pub struct ExternalFenceFeatureFlags(pub VkExternalFenceFeatureFlags);
    impl ExternalFenceFeatureFlags {
        pub const fn contains_exportable(self) -> bool {
            (self.0 & VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT) != 0
        }
        pub const fn contains_importable(self) -> bool {
            (self.0 & VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT) != 0
        }
    }
    impl std::fmt::Debug for ExternalFenceFeatureFlags {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut bit_strings = Vec::with_capacity(2);
            if self.contains_exportable() {
                bit_strings.push("EXPORTABLE");
            }
            if self.contains_importable() {
                bit_strings.push("IMPORTABLE");
            }

            write!(
                fmt,
                "ExternalFenceFeatureFlags(0x{:02x}: {})",
                self.0,
                bit_strings.join("/")
            )
        }
    }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub use self::external_fence_capabilities_khr::*;

/// Device memory properties
#[repr(transparent)]
pub struct MemoryProperties(VkPhysicalDeviceMemoryProperties);
impl MemoryProperties {
    pub fn find_type_index(
        &self,
        mask: MemoryPropertyFlags,
        exclude: MemoryPropertyFlags,
        index_mask: u32,
    ) -> Option<u32> {
        self.0.memoryTypes[..self.0.memoryTypeCount as usize]
            .iter()
            .enumerate()
            .filter(|(i, _)| (index_mask & (1u32 << i)) != 0)
            .filter(|(_, mt)| (mt.propertyFlags & mask.0) != 0 && (mt.propertyFlags & exclude.0) == 0)
            .map(|(x, _)| x as u32)
            .next()
    }
    pub fn find_device_local_index(&self, index_mask: u32) -> Option<u32> {
        self.find_type_index(
            MemoryPropertyFlags::DEVICE_LOCAL,
            MemoryPropertyFlags::LAZILY_ALLOCATED,
            index_mask,
        )
    }
    pub fn find_lazily_allocated_device_local_index(&self, index_mask: u32) -> Option<u32> {
        self.find_type_index(
            MemoryPropertyFlags::DEVICE_LOCAL.lazily_allocated(),
            MemoryPropertyFlags::EMPTY,
            index_mask,
        )
    }
    pub fn find_host_visible_index(&self, index_mask: u32) -> Option<u32> {
        self.find_type_index(
            MemoryPropertyFlags::HOST_VISIBLE,
            MemoryPropertyFlags::EMPTY,
            index_mask,
        )
    }
    pub fn is_coherent(&self, index: u32) -> bool {
        (self.0.memoryTypes[index as usize].propertyFlags & MemoryPropertyFlags::HOST_COHERENT.0) != 0
    }
    pub fn is_cached(&self, index: u32) -> bool {
        (self.0.memoryTypes[index as usize].propertyFlags & MemoryPropertyFlags::HOST_CACHED.0) != 0
    }

    pub fn types(&self) -> MemoryTypeIter {
        MemoryTypeIter(&self.0, 0)
    }
    pub fn heaps(&self) -> MemoryHeapIter {
        MemoryHeapIter(&self.0, 0)
    }
}

/// Iterating each elements of memory types
pub struct MemoryTypeIter<'d>(&'d VkPhysicalDeviceMemoryProperties, usize);
impl<'d> Iterator for MemoryTypeIter<'d> {
    type Item = &'d VkMemoryType;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < self.0.memoryTypeCount as usize {
            let r = &self.0.memoryTypes[self.1];
            self.1 += 1;
            Some(r)
        } else {
            None
        }
    }
}

/// Iterating each elements of memory heaps
pub struct MemoryHeapIter<'d>(&'d VkPhysicalDeviceMemoryProperties, usize);
impl<'d> Iterator for MemoryHeapIter<'d> {
    type Item = &'d VkMemoryHeap;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < self.0.memoryHeapCount as usize {
            let r = &self.0.memoryHeaps[self.1];
            self.1 += 1;
            Some(r)
        } else {
            None
        }
    }
}

/// Bitmask specifying properties for a memory type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
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
    pub fn device_local(mut self) -> Self {
        self.0 |= Self::DEVICE_LOCAL.0;
        self
    }
    /// Memory allocated with this type can be mapped for host access using `vkMapMemory`
    pub fn host_visible(mut self) -> Self {
        self.0 |= Self::HOST_VISIBLE.0;
        self
    }
    /// The host cache management commands `vkFlushMappedmemoryRanges` and `vkInvalidateMappedMemoryRanges`
    /// are not needed to flush host writes to the device or make device writes visible to the host, respectively.
    pub fn host_coherent(mut self) -> Self {
        self.0 |= Self::HOST_COHERENT.0;
        self
    }
    /// Memory allocated with this type is cached on the host.
    /// Host memory accesses to uncached memory are slower than to cached memory, however uncached memory is always host coherent
    pub fn host_cached(mut self) -> Self {
        self.0 |= Self::HOST_CACHED.0;
        self
    }
    /// The memory type only allows device access to the memory.
    pub fn lazily_allocated(mut self) -> Self {
        self.0 |= Self::LAZILY_ALLOCATED.0;
        self
    }

    pub fn bits(self) -> VkMemoryPropertyFlags {
        self.0
    }
}
impl BitOr for MemoryPropertyFlags {
    type Output = MemoryPropertyFlags;
    fn bitor(self, other: Self) -> Self {
        MemoryPropertyFlags(self.0 | other.0)
    }
}
impl BitOrAssign for MemoryPropertyFlags {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

/// List of queue families
pub struct QueueFamilies(pub Vec<VkQueueFamilyProperties>);
impl QueueFamilies {
    /// Find a queue family index containing specified bitflags
    #[allow(non_snake_case)]
    pub fn find_matching_index(&self, flags: QueueFlags) -> Option<u32> {
        self.0
            .iter()
            .position(|q| (q.queueFlags & flags.0) != 0)
            .map(|x| x as _)
    }
    /// Find a queue family index containing specified bitflags
    #[allow(non_snake_case)]
    pub fn find_another_matching_index(&self, flags: QueueFlags, exclude: u32) -> Option<u32> {
        self.0
            .iter()
            .enumerate()
            .find(|&(n, &VkQueueFamilyProperties { queueFlags, .. })| {
                (queueFlags & flags.0) != 0 && exclude != n as u32
            })
            .map(|(n, _)| n as _)
    }
    /// Number of queue families
    pub fn count(&self) -> u32 {
        self.0.len() as _
    }
    /// Number of queues in selected queue family
    pub fn queue_count(&self, family_index: u32) -> u32 {
        self.0[family_index as usize].queueCount
    }
    /// Unsigned integer count of meaningful bits in the timestamps written via `vkCmdWriteTimestamp`
    pub fn timestamp_valid_bits(&self, family_index: u32) -> u32 {
        self.0[family_index as usize].timestampValidBits
    }
    /// Minimum granularity supported for image transfer operations on the queues in selected queue family
    pub fn minimum_image_transfer_granularity(&self, family_index: u32) -> &VkExtent3D {
        &self.0[family_index as usize].minImageTransferGranularity
    }
}

/// Set of bit of queue flags
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct QueueFlags(VkQueueFlags);
impl QueueFlags {
    /// Empty bits
    pub const EMPTY: Self = QueueFlags(0);
    /// Supports only graphics operations
    pub const GRAPHICS: Self = QueueFlags(VK_QUEUE_GRAPHICS_BIT);
    /// Supports only compute operations
    pub const COMPUTE: Self = QueueFlags(VK_QUEUE_COMPUTE_BIT);
    /// Supports only transfer operations
    pub const TRANSFER: Self = QueueFlags(VK_QUEUE_TRANSFER_BIT);
    /// Supports only sparse memory management operations
    pub const SPARSE_BINDING: Self = QueueFlags(VK_QUEUE_SPARSE_BINDING_BIT);
    /// Supports graphics operations
    pub const fn graphics(self) -> Self {
        QueueFlags(self.bits() | Self::GRAPHICS.0)
    }
    /// Supports compute operations
    pub const fn compute(self) -> Self {
        QueueFlags(self.0 | Self::COMPUTE.0)
    }
    /// Supports transfer operations
    pub const fn transfer(self) -> Self {
        QueueFlags(self.0 | Self::TRANSFER.0)
    }
    /// Supports sparse memory management operatinons
    pub const fn sparse_binding(self) -> Self {
        QueueFlags(self.0 | Self::SPARSE_BINDING.0)
    }

    pub const fn bits(self) -> VkQueueFlags {
        self.0
    }
}

#[cfg(feature = "VK_KHR_display")]
mod display;
#[cfg(feature = "VK_KHR_display")]
pub use self::display::*;

mod ext_query;
pub use self::ext_query::*;
