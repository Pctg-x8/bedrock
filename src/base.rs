//! Vulkan Base Objects(Instance/PhysicalDevice)

use derives::{implements, transparent_marked};

use crate::{ffi_helper::opt_pointer, vk::*, VkHandle, VkObject, VulkanStructure, VulkanStructureAsRef};
#[cfg(feature = "Implements")]
use crate::{fnconv::FnTransmute, ImageFlags, ImageUsageFlags};
#[cfg(all(feature = "Implements", feature = "VK_KHR_surface"))]
use crate::{PresentMode, Surface};
use std::{
    ffi::{c_char, CStr},
    ops::*,
};

#[cfg(feature = "Multithreaded")]
struct LazyCellReadRef<'d, T>(::std::sync::RwLockReadGuard<'d, Option<T>>);
#[cfg(feature = "Multithreaded")]
impl<'d, T> ::std::ops::Deref for LazyCellReadRef<'d, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.as_ref().unwrap()
    }
}

/// Query instance-level version before instance creation
/// # Failures
/// On failure, this command returns
///
/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
#[inline]
pub fn instance_version() -> crate::Result<(u16, u16, u16)> {
    #[cfg(feature = "Allow1_1APIs")]
    unsafe {
        let mut sink = 0;
        crate::vkfn::enumerate_instance_version(&mut sink).into_result()?;
        Ok(crate::vk::vk_deserialize_version(sink))
    }
    #[cfg(not(feature = "Allow1_1APIs"))]
    {
        // fixed to v1.0.0
        Ok((1, 0, 0))
    }
}

#[implements]
type InstanceResolvedFn<F> = crate::resolver::ResolvedFnCell<F, VkInstance>;
#[implements]
impl crate::resolver::ResolverInterface for VkInstance {
    unsafe fn load_symbol_unconstrainted<T: crate::resolver::FromPtr>(&self, name: &core::ffi::CStr) -> T {
        T::from_ptr(core::mem::transmute(crate::vkfn::get_instance_proc_addr(
            *self,
            name.as_ptr() as _,
        )))
    }

    unsafe fn load_function_unconstrainted<F: crate::resolver::PFN>(&self, name: &core::ffi::CStr) -> F {
        F::from_void_fn(
            crate::vkfn::get_instance_proc_addr(*self, name.as_ptr() as _)
                .unwrap_or_else(|| panic!("function {:?} not found", name)),
        )
    }
}

#[implements]
struct InstanceExtFunctions {
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    get_physical_device_properties2_khr: InstanceResolvedFn<PFN_vkGetPhysicalDeviceProperties2KHR>,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    get_physical_device_features2_khr: InstanceResolvedFn<PFN_vkGetPhysicalDeviceFeatures2KHR>,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    get_physical_device_format_properties2_khr: InstanceResolvedFn<PFN_vkGetPhysicalDeviceFormatProperties2KHR>,
    #[cfg(feature = "VK_EXT_debug_report")]
    create_debug_report_callback_ext: InstanceResolvedFn<PFN_vkCreateDebugReportCallbackEXT>,
    #[cfg(feature = "VK_EXT_debug_report")]
    destroy_debug_report_callback_ext: InstanceResolvedFn<PFN_vkDestroyDebugReportCallbackEXT>,
    #[cfg(feature = "VK_EXT_debug_report")]
    debug_report_message_ext: InstanceResolvedFn<PFN_vkDebugReportMessageEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    create_debug_utils_messenger_ext: InstanceResolvedFn<PFN_vkCreateDebugUtilsMessengerEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    destroy_debug_utils_messenger_ext: InstanceResolvedFn<PFN_vkDestroyDebugUtilsMessengerEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    set_debug_utils_object_name_ext: InstanceResolvedFn<PFN_vkSetDebugUtilsObjectNameEXT>,
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    get_physical_device_external_fence_properties_khr:
        InstanceResolvedFn<PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR>,
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    get_randr_output_display_ext: InstanceResolvedFn<PFN_vkGetRandROutputDisplayEXT>,
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    acquire_xlib_display_ext: InstanceResolvedFn<PFN_vkAcquireXlibDisplayEXT>,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    get_physical_device_surface_present_modes_2_ext: InstanceResolvedFn<PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT>,
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    get_physical_device_surface_capabilities_2_khr: InstanceResolvedFn<PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR>,
    #[cfg(feature = "VK_EXT_direct_mode_display")]
    release_display_ext: InstanceResolvedFn<PFN_vkReleaseDisplayEXT>,
    #[cfg(feature = "VK_EXT_sample_locations")]
    get_physical_device_multisample_properties_ext: InstanceResolvedFn<PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT>,
}
#[implements]
impl InstanceExtFunctions {
    const fn new(r: VkInstance) -> Self {
        Self {
            #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
            get_physical_device_properties2_khr: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
            get_physical_device_features2_khr: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
            get_physical_device_format_properties2_khr: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_debug_report")]
            create_debug_report_callback_ext: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_debug_report")]
            destroy_debug_report_callback_ext: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_debug_report")]
            debug_report_message_ext: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_debug_utils")]
            create_debug_utils_messenger_ext: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_debug_utils")]
            destroy_debug_utils_messenger_ext: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_debug_utils")]
            set_debug_utils_object_name_ext: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_KHR_external_fence_capabilities")]
            get_physical_device_external_fence_properties_khr: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_acquire_xlib_display")]
            get_randr_output_display_ext: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_acquire_xlib_display")]
            acquire_xlib_display_ext: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            get_physical_device_surface_present_modes_2_ext: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
            get_physical_device_surface_capabilities_2_khr: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_direct_mode_display")]
            release_display_ext: InstanceResolvedFn::new(r),
            #[cfg(feature = "VK_EXT_sample_locations")]
            get_physical_device_multisample_properties_ext: InstanceResolvedFn::new(r),
        }
    }
}

/// Opaque handle to a instance object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VK_OBJECT_TYPE_INSTANCE)]
pub struct InstanceObject {
    #[handle]
    handle: VkInstance,
    #[cfg(feature = "Implements")]
    ext: InstanceExtFunctions,
}
unsafe impl Sync for InstanceObject {}
unsafe impl Send for InstanceObject {}
#[implements]
impl Drop for InstanceObject {
    fn drop(&mut self) {
        unsafe {
            crate::vkfn::destroy_instance(self.handle, core::ptr::null());
        }
    }
}
impl Instance for InstanceObject {
    #[implements("VK_KHR_get_physical_device_properties2")]
    fn get_physical_device_properties2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceProperties2KHR {
        *self.ext.get_physical_device_properties2_khr.resolve()
    }
    #[implements("VK_KHR_get_physical_device_properties2")]
    fn get_physical_device_features2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceFeatures2KHR {
        *self.ext.get_physical_device_features2_khr.resolve()
    }
    #[implements("VK_KHR_get_physical_device_properties2")]
    fn get_physical_device_format_properties2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceFormatProperties2KHR {
        *self.ext.get_physical_device_format_properties2_khr.resolve()
    }

    #[implements("VK_EXT_debug_report")]
    fn create_debug_report_callback_ext_fn(&self) -> PFN_vkCreateDebugReportCallbackEXT {
        *self.ext.create_debug_report_callback_ext.resolve()
    }
    #[implements("VK_EXT_debug_report")]
    fn destroy_debug_report_callback_ext_fn(&self) -> PFN_vkDestroyDebugReportCallbackEXT {
        *self.ext.destroy_debug_report_callback_ext.resolve()
    }
    #[implements("VK_EXT_debug_report")]
    fn debug_report_message_ext_fn(&self) -> PFN_vkDebugReportMessageEXT {
        *self.ext.debug_report_message_ext.resolve()
    }

    #[implements("VK_EXT_debug_utils")]
    fn create_debug_utils_messenger_ext_fn(&self) -> PFN_vkCreateDebugUtilsMessengerEXT {
        *self.ext.create_debug_utils_messenger_ext.resolve()
    }
    #[implements("VK_EXT_debug_utils")]
    fn destroy_debug_utils_messenger_ext_fn(&self) -> PFN_vkDestroyDebugUtilsMessengerEXT {
        *self.ext.destroy_debug_utils_messenger_ext.resolve()
    }
    #[implements("VK_EXT_debug_utils")]
    fn set_debug_utils_object_name_ext_fn(&self) -> PFN_vkSetDebugUtilsObjectNameEXT {
        *self.ext.set_debug_utils_object_name_ext.resolve()
    }

    #[implements("VK_KHR_external_fence_capabilities")]
    fn get_physical_device_external_fence_properties_khr_fn(
        &self,
    ) -> PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR {
        *self.ext.get_physical_device_external_fence_properties_khr.resolve()
    }

    #[implements("VK_EXT_acquire_xlib_display")]
    fn get_randr_output_display_ext_fn(&self) -> PFN_vkGetRandROutputDisplayEXT {
        *self.ext.get_randr_output_display_ext.resolve()
    }
    #[implements("VK_EXT_acquire_xlib_display")]
    fn acquire_xlib_display_ext_fn(&self) -> PFN_vkAcquireXlibDisplayEXT {
        *self.ext.acquire_xlib_display_ext.resolve()
    }

    #[implements("VK_EXT_full_screen_exclusive")]
    fn get_physical_device_surface_present_modes_2_ext_fn(&self) -> PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT {
        *self.ext.get_physical_device_surface_present_modes_2_ext.resolve()
    }

    #[implements("VK_KHR_get_surface_capabilities2")]
    fn get_physical_device_surface_capabilities_2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR {
        *self.ext.get_physical_device_surface_capabilities_2_khr.resolve()
    }

    #[implements("VK_EXT_direct_mode_display")]
    fn release_display_ext_fn(&self) -> PFN_vkReleaseDisplayEXT {
        *self.ext.release_display_ext.resolve()
    }

    #[implements("VK_EXT_sample_locations")]
    fn get_physical_device_multisample_properties_ext_fn(&self) -> PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT {
        *self.ext.get_physical_device_multisample_properties_ext.resolve()
    }
}

/// Opaque handle to a physical device object
///
/// ## Platform Dependent Methods: Presentation Support checking functions
///
/// * `xlib_presentation_support(&self, queue_family: u32, display: *mut x11::xlib::Display, visual: x11::xlib::VisualID) -> bool`: VK_KHR_xlib_surface
/// * `xcb_presentation_support(&self, queue_family: u32, connection: *mut xcb::ffi::xcb_connection_t, visual: xcb::ffi::xcb_visualid_t) -> bool`: VK_KHR_xcb_surface
/// * `wayland_presentation_support(&self, queue_family: u32, display: *mut wayland_client::sys::wl_display) -> bool`: VK_KHR_wayland_surface
/// * `win32_presentation_support(&self, queue_family: u32) -> bool`: VK_KHR_win32_surface
/// * Methods for Android and Mir surfaces are not implemented
#[derive(VkHandle, VkObject, crate::InstanceChild, crate::InstanceChildTransferrable, Clone)]
#[VkObject(type = VK_OBJECT_TYPE_PHYSICAL_DEVICE)]
pub struct PhysicalDeviceObject<Owner: Instance>(VkPhysicalDevice, #[parent] Owner);
unsafe impl<Owner: Instance + Sync> Sync for PhysicalDeviceObject<Owner> {}
unsafe impl<Owner: Instance + Send> Send for PhysicalDeviceObject<Owner> {}
impl<Owner: Instance> PhysicalDevice for PhysicalDeviceObject<Owner> {}
impl<Owner: Instance + Clone> PhysicalDeviceObject<&'_ Owner> {
    /// Split the lifetime from owner by cloning it.
    #[inline(always)]
    pub fn clone_parent(&self) -> PhysicalDeviceObject<Owner> {
        PhysicalDeviceObject(self.0, self.1.clone())
    }
}

pub struct IterPhysicalDevices<'i, Source: Instance + 'i + ?Sized>(Vec<VkPhysicalDevice>, usize, &'i Source);
impl<'i, Source: Instance + 'i + ?Sized> Iterator for IterPhysicalDevices<'i, Source> {
    type Item = PhysicalDeviceObject<&'i Source>;

    #[inline(always)]
    fn next(&mut self) -> Option<PhysicalDeviceObject<&'i Source>> {
        if self.0.len() <= self.1 {
            None
        } else {
            self.1 += 1;
            Some(PhysicalDeviceObject(self.0[self.1 - 1], self.2))
        }
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len(), Some(self.0.len()))
    }
}
impl<'i, Source: Instance + 'i + ?Sized> ExactSizeIterator for IterPhysicalDevices<'i, Source> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'i, Source: Instance + 'i + ?Sized> DoubleEndedIterator for IterPhysicalDevices<'i, Source> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<PhysicalDeviceObject<&'i Source>> {
        if self.0.len() <= self.1 {
            None
        } else {
            self.0.pop().map(|p| PhysicalDeviceObject(p, self.2))
        }
    }
}

#[transparent_marked]
pub struct ApplicationInfo<'d>(VkApplicationInfo, core::marker::PhantomData<&'d CStr>);
impl<'d> ApplicationInfo<'d> {
    #[inline(always)]
    pub const fn new(
        app_name: &'d CStr,
        app_version: (u16, u16, u16),
        engine_name: &'d CStr,
        engine_version: (u16, u16, u16),
    ) -> Self {
        Self(
            VkApplicationInfo {
                sType: VkApplicationInfo::TYPE,
                pNext: core::ptr::null(),
                apiVersion: VK_API_VERSION_1_0,
                pApplicationName: app_name.as_ptr(),
                applicationVersion: VK_MAKE_VERSION(app_version.0, app_version.1, app_version.2),
                pEngineName: engine_name.as_ptr(),
                engineVersion: VK_MAKE_VERSION(engine_version.0, engine_version.1, engine_version.2),
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub const fn api_version(mut self, major: u16, minor: u16, patch: u16) -> Self {
        self.0.apiVersion = VK_MAKE_VERSION(major, minor, patch);
        self
    }
}

/// Builder object for constructing a `Instance`
pub struct InstanceBuilder<'d> {
    extensions: Vec<*const c_char>,
    layers: Vec<*const c_char>,
    ext_structures: Vec<Box<dyn crate::ext::VulkanStructureAsRef + 'static>>,
    cinfo: VkInstanceCreateInfo,
    _refs: core::marker::PhantomData<(&'d CStr, &'d ApplicationInfo<'d>)>,
}
impl<'d> InstanceBuilder<'d> {
    #[inline]
    pub const fn new(app_info: &'d ApplicationInfo) -> Self {
        Self {
            extensions: Vec::new(),
            layers: Vec::new(),
            ext_structures: Vec::new(),
            cinfo: VkInstanceCreateInfo {
                sType: VkInstanceCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                pApplicationInfo: app_info as *const _ as _,
                enabledLayerCount: 0,
                ppEnabledLayerNames: core::ptr::null(),
                enabledExtensionCount: 0,
                ppEnabledExtensionNames: core::ptr::null(),
            },
            _refs: core::marker::PhantomData,
        }
    }

    pub fn add_extension(&mut self, extension: &'d CStr) -> &mut Self {
        self.extensions.push(extension.as_ptr());
        self
    }
    pub fn add_extensions(&mut self, extensions: impl IntoIterator<Item = &'d CStr>) -> &mut Self {
        self.extensions.extend(extensions.into_iter().map(CStr::as_ptr));
        self
    }
    pub fn add_layer(&mut self, layer: &'d CStr) -> &mut Self {
        self.layers.push(layer.as_ptr());
        self
    }
    pub fn add_layers(&mut self, layers: impl IntoIterator<Item = &'d CStr>) -> &mut Self {
        self.layers.extend(layers.into_iter().map(CStr::as_ptr));
        self
    }

    pub fn add_ext_structure<S: crate::ext::VulkanStructure + 'static>(&mut self, ext: S) -> &mut Self {
        self.ext_structures.push(Box::new(ext) as _);
        self
    }

    #[cfg(feature = "VK_KHR_portability_enumeration")]
    pub fn enumerate_portability(&mut self) -> &mut Self {
        self.add_extension(c"VK_KHR_portability_enumeration");
        self.cinfo.flags |= VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR;
        self
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
    #[implements]
    pub fn create(mut self) -> crate::Result<InstanceObject> {
        use crate::ffi_helper::slice_as_ptr_empty_null;

        crate::ext::chain(
            &mut self.cinfo,
            self.ext_structures.iter_mut().map(VulkanStructureAsRef::as_generic_mut),
        );
        self.cinfo.enabledLayerCount = self.layers.len() as _;
        self.cinfo.enabledExtensionCount = self.extensions.len() as _;
        self.cinfo.ppEnabledLayerNames = slice_as_ptr_empty_null(&self.layers);
        self.cinfo.ppEnabledExtensionNames = slice_as_ptr_empty_null(&self.extensions);

        unsafe { InstanceObject::new_raw(&self.cinfo) }
    }
}

impl InstanceObject {
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
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    pub unsafe fn new_raw(info: &VkInstanceCreateInfo) -> crate::Result<Self> {
        Ok(Self::manage(new_instance_raw(info, None)?))
    }

    /// Constructs from raw handle
    /// # Safety
    /// the handle must be valid and not freed
    pub const unsafe fn manage(handle: VkInstance) -> Self {
        Self {
            handle,
            #[cfg(feature = "Implements")]
            ext: InstanceExtFunctions::new(handle),
        }
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(self) -> VkInstance {
        let v = self.handle;
        core::mem::forget(self);

        v
    }
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
///
/// # Safety
/// no guarantees will be provided (simply calls under api)
#[implements]
pub unsafe fn new_instance_raw(
    info: &VkInstanceCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkInstance> {
    let mut h = core::mem::MaybeUninit::uninit();

    crate::vkfn::create_instance(info, opt_pointer(allocation_callbacks), h.as_mut_ptr()).into_result()?;
    Ok(h.assume_init())
}

/// Returns up to all of global layer properties
/// # Failures
/// On failure, this command returns
///
/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
#[implements]
pub fn enumerate_layer_properties() -> crate::Result<Vec<VkLayerProperties>> {
    unsafe {
        let mut n = 0;
        crate::vkfn::enumerate_instance_layer_properties(&mut n, core::ptr::null_mut()).into_result()?;
        if n == 0 {
            // no items
            return Ok(Vec::new());
        }

        let mut v = Vec::with_capacity(n as _);
        v.set_len(n as _);
        crate::vkfn::enumerate_instance_layer_properties(&mut n, v.as_mut_ptr()).into_result()?;

        Ok(v)
    }
}

/// Returns up to all of global extension properties
/// # Failures
/// On failure, this command returns
///
/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// * `VK_ERROR_LAYER_NOT_PRESENT`
#[implements]
pub fn enumerate_extension_properties_cstr(layer_name: Option<&CStr>) -> crate::Result<Vec<VkExtensionProperties>> {
    let ln_ptr = layer_name.map_or_else(core::ptr::null, CStr::as_ptr);

    unsafe {
        let mut n = 0;
        crate::vkfn::enumerate_instance_extension_properties(ln_ptr, &mut n, core::ptr::null_mut()).into_result()?;
        if n == 0 {
            // no items
            return Ok(Vec::new());
        }

        let mut v = Vec::with_capacity(n as _);
        v.set_len(n as _);
        crate::vkfn::enumerate_instance_extension_properties(ln_ptr, &mut n, v.as_mut_ptr()).into_result()?;

        Ok(v)
    }
}

/// Returns up to all of global extension properties
/// # Failures
/// On failure, this command returns
///
/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// * `VK_ERROR_LAYER_NOT_PRESENT`
#[implements]
pub fn enumerate_extension_properties(layer_name: Option<&str>) -> crate::Result<Vec<VkExtensionProperties>> {
    enumerate_extension_properties_cstr(layer_name.map(|s| std::ffi::CString::new(s).unwrap()).as_deref())
}

/// A Vulkan Instance interface
pub trait Instance: VkHandle<Handle = VkInstance> {
    /// Return a function pointer for a command
    /// # Failures
    /// If function is not provided by instance or `name` is empty, returns `None`
    #[deprecated = "do not use this directly(this does not provide caching)"]
    #[implements]
    fn extra_procedure<F: FnTransmute>(&self, name: &str) -> Option<F> {
        if name.is_empty() {
            return None;
        }

        unsafe {
            let fn_cstr = std::ffi::CString::new(name).unwrap();
            crate::vkfn::get_instance_proc_addr(self.native_ptr(), fn_cstr.as_ptr()).map(|f| FnTransmute::from_fn(f))
        }
    }

    /// Enumerates the physical devices accessible to a Vulkan instance
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_INITIALIZATION_FAILED`
    #[implements]
    fn enumerate_physical_devices(&self) -> crate::Result<Vec<PhysicalDeviceObject<&Self>>> {
        self.iter_physical_devices().map(Iterator::collect)
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
    #[implements]
    fn iter_physical_devices(&self) -> crate::Result<IterPhysicalDevices<Self>> {
        unsafe {
            let mut n = 0;
            crate::vkfn::enumerate_physical_devices(self.native_ptr(), &mut n, std::ptr::null_mut()).into_result()?;
            if n == 0 {
                // no items
                return Ok(IterPhysicalDevices(Vec::new(), 0, self));
            }

            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            crate::vkfn::enumerate_physical_devices(self.native_ptr(), &mut n, v.as_mut_ptr())
                .into_result()
                .map(move |_| IterPhysicalDevices(v, 0, self))
        }
    }

    /// Inject its own messages into the debug stream
    #[implements("VK_EXT_debug_report")]
    #[inline]
    fn debug_message(
        &self,
        flags: VkDebugReportFlagsEXT,
        object_type: crate::DebugReportObjectType,
        object: u64,
        location: usize,
        message_count: i32,
        layer_prefix: &CStr,
        message: &CStr,
    ) {
        unsafe {
            self.debug_report_message_ext_fn().0(
                self.native_ptr(),
                flags,
                object_type as _,
                object,
                location,
                message_count,
                layer_prefix.as_ptr(),
                message.as_ptr(),
            );
        }
    }

    // Extension Function Providers

    #[implements("VK_KHR_get_physical_device_properties2")]
    fn get_physical_device_properties2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceProperties2KHR;
    #[implements("VK_KHR_get_physical_device_properties2")]
    fn get_physical_device_features2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceFeatures2KHR;
    #[implements("VK_KHR_get_physical_device_properties2")]
    fn get_physical_device_format_properties2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceFormatProperties2KHR;

    #[implements("VK_EXT_debug_report")]
    fn create_debug_report_callback_ext_fn(&self) -> PFN_vkCreateDebugReportCallbackEXT;
    #[implements("VK_EXT_debug_report")]
    fn destroy_debug_report_callback_ext_fn(&self) -> PFN_vkDestroyDebugReportCallbackEXT;
    #[implements("VK_EXT_debug_report")]
    fn debug_report_message_ext_fn(&self) -> PFN_vkDebugReportMessageEXT;

    #[implements("VK_EXT_debug_utils")]
    fn create_debug_utils_messenger_ext_fn(&self) -> PFN_vkCreateDebugUtilsMessengerEXT;
    #[implements("VK_EXT_debug_utils")]
    fn destroy_debug_utils_messenger_ext_fn(&self) -> PFN_vkDestroyDebugUtilsMessengerEXT;
    #[implements("VK_EXT_debug_utils")]
    fn set_debug_utils_object_name_ext_fn(&self) -> PFN_vkSetDebugUtilsObjectNameEXT;

    #[implements("VK_KHR_external_fence_capabilities")]
    fn get_physical_device_external_fence_properties_khr_fn(&self)
        -> PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR;

    #[implements("VK_EXT_acquire_xlib_display")]
    fn get_randr_output_display_ext_fn(&self) -> PFN_vkGetRandROutputDisplayEXT;
    #[implements("VK_EXT_acquire_xlib_display")]
    fn acquire_xlib_display_ext_fn(&self) -> PFN_vkAcquireXlibDisplayEXT;

    #[implements("VK_EXT_full_screen_exclusive")]
    fn get_physical_device_surface_present_modes_2_ext_fn(&self) -> PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT;

    #[implements("VK_KHR_get_surface_capabilities2")]
    fn get_physical_device_surface_capabilities_2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR;

    #[implements("VK_EXT_direct_mode_display")]
    fn release_display_ext_fn(&self) -> PFN_vkReleaseDisplayEXT;

    #[implements("VK_EXT_sample_locations")]
    fn get_physical_device_multisample_properties_ext_fn(&self) -> PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT;
}
DerefContainerBracketImpl!(for Instance {
    #[implements("VK_KHR_get_physical_device_properties2")]
    ForwardFnPtr!(deref get_physical_device_properties2_khr_fn -> PFN_vkGetPhysicalDeviceProperties2KHR);
    #[implements("VK_KHR_get_physical_device_properties2")]
    ForwardFnPtr!(deref get_physical_device_features2_khr_fn -> PFN_vkGetPhysicalDeviceFeatures2KHR);
    #[implements("VK_KHR_get_physical_device_properties2")]
    ForwardFnPtr!(deref get_physical_device_format_properties2_khr_fn -> PFN_vkGetPhysicalDeviceFormatProperties2KHR);

    #[implements("VK_EXT_debug_report")]
    ForwardFnPtr!(deref create_debug_report_callback_ext_fn -> PFN_vkCreateDebugReportCallbackEXT);
    #[implements("VK_EXT_debug_report")]
    ForwardFnPtr!(deref destroy_debug_report_callback_ext_fn -> PFN_vkDestroyDebugReportCallbackEXT);
    #[implements("VK_EXT_debug_report")]
    ForwardFnPtr!(deref debug_report_message_ext_fn -> PFN_vkDebugReportMessageEXT);

    #[implements("VK_EXT_debug_utils")]
    ForwardFnPtr!(deref create_debug_utils_messenger_ext_fn -> PFN_vkCreateDebugUtilsMessengerEXT);
    #[implements("VK_EXT_debug_utils")]
    ForwardFnPtr!(deref destroy_debug_utils_messenger_ext_fn -> PFN_vkDestroyDebugUtilsMessengerEXT);
    #[implements("VK_EXT_debug_utils")]
    ForwardFnPtr!(deref set_debug_utils_object_name_ext_fn -> PFN_vkSetDebugUtilsObjectNameEXT);

    #[implements("VK_KHR_external_fence_capabilities")]
    ForwardFnPtr!(deref get_physical_device_external_fence_properties_khr_fn -> PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR);

    #[implements("VK_EXT_acquire_xlib_display")]
    ForwardFnPtr!(deref get_randr_output_display_ext_fn -> PFN_vkGetRandROutputDisplayEXT);
    #[implements("VK_EXT_acquire_xlib_display")]
    ForwardFnPtr!(deref acquire_xlib_display_ext_fn -> PFN_vkAcquireXlibDisplayEXT);

    #[implements("VK_EXT_full_screen_exclusive")]
    ForwardFnPtr!(deref get_physical_device_surface_present_modes_2_ext_fn -> PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT);

    #[implements("VK_KHR_get_surface_capabilities2")]
    ForwardFnPtr!(deref get_physical_device_surface_capabilities_2_khr_fn -> PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR);

    #[implements("VK_EXT_direct_mode_display")]
    ForwardFnPtr!(deref release_display_ext_fn -> PFN_vkReleaseDisplayEXT);

    #[implements("VK_EXT_sample_locations")]
    ForwardFnPtr!(deref get_physical_device_multisample_properties_ext_fn -> PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT);
});
GuardsImpl!(for Instance {
    #[implements("VK_KHR_get_physical_device_properties2")]
    ForwardFnPtr!(deref get_physical_device_properties2_khr_fn -> PFN_vkGetPhysicalDeviceProperties2KHR);
    #[implements("VK_KHR_get_physical_device_properties2")]
    ForwardFnPtr!(deref get_physical_device_features2_khr_fn -> PFN_vkGetPhysicalDeviceFeatures2KHR);
    #[implements("VK_KHR_get_physical_device_properties2")]
    ForwardFnPtr!(deref get_physical_device_format_properties2_khr_fn -> PFN_vkGetPhysicalDeviceFormatProperties2KHR);

    #[implements("VK_EXT_debug_report")]
    ForwardFnPtr!(deref create_debug_report_callback_ext_fn -> PFN_vkCreateDebugReportCallbackEXT);
    #[implements("VK_EXT_debug_report")]
    ForwardFnPtr!(deref destroy_debug_report_callback_ext_fn -> PFN_vkDestroyDebugReportCallbackEXT);
    #[implements("VK_EXT_debug_report")]
    ForwardFnPtr!(deref debug_report_message_ext_fn -> PFN_vkDebugReportMessageEXT);

    #[implements("VK_EXT_debug_utils")]
    ForwardFnPtr!(deref create_debug_utils_messenger_ext_fn -> PFN_vkCreateDebugUtilsMessengerEXT);
    #[implements("VK_EXT_debug_utils")]
    ForwardFnPtr!(deref destroy_debug_utils_messenger_ext_fn -> PFN_vkDestroyDebugUtilsMessengerEXT);
    #[implements("VK_EXT_debug_utils")]
    ForwardFnPtr!(deref set_debug_utils_object_name_ext_fn -> PFN_vkSetDebugUtilsObjectNameEXT);

    #[implements("VK_KHR_external_fence_capabilities")]
    ForwardFnPtr!(deref get_physical_device_external_fence_properties_khr_fn -> PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR);

    #[implements("VK_EXT_acquire_xlib_display")]
    ForwardFnPtr!(deref get_randr_output_display_ext_fn -> PFN_vkGetRandROutputDisplayEXT);
    #[implements("VK_EXT_acquire_xlib_display")]
    ForwardFnPtr!(deref acquire_xlib_display_ext_fn -> PFN_vkAcquireXlibDisplayEXT);

    #[implements("VK_EXT_full_screen_exclusive")]
    ForwardFnPtr!(deref get_physical_device_surface_present_modes_2_ext_fn -> PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT);

    #[implements("VK_KHR_get_surface_capabilities2")]
    ForwardFnPtr!(deref get_physical_device_surface_capabilities_2_khr_fn -> PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR);

    #[implements("VK_EXT_direct_mode_display")]
    ForwardFnPtr!(deref release_display_ext_fn -> PFN_vkReleaseDisplayEXT);

    #[implements("VK_EXT_sample_locations")]
    ForwardFnPtr!(deref get_physical_device_multisample_properties_ext_fn -> PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT);
});

/// A PhysicalDevice interface
pub trait PhysicalDevice: VkHandle<Handle = VkPhysicalDevice> + InstanceChild {
    /// Returns properties of available physical device layers
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn enumerate_layer_properties(&self) -> crate::Result<Vec<VkLayerProperties>> {
        let mut count = 0;
        unsafe {
            crate::vkfn::enumerate_device_layer_properties(self.native_ptr(), &mut count, core::ptr::null_mut())
                .into_result()?;
        }
        if count == 0 {
            // no items
            return Ok(Vec::new());
        }

        let mut v = Vec::with_capacity(count as _);
        unsafe {
            v.set_len(count as _);
        }
        unsafe {
            crate::vkfn::enumerate_device_layer_properties(self.native_ptr(), &mut count, v.as_mut_ptr())
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
    #[implements]
    fn enumerate_extension_properties_cstr(
        &self,
        layer_name: Option<&CStr>,
    ) -> crate::Result<Vec<VkExtensionProperties>> {
        let ln_ptr = layer_name.map_or_else(core::ptr::null, CStr::as_ptr);

        unsafe {
            let mut n = 0;
            crate::vkfn::enumerate_device_extension_properties(
                self.native_ptr(),
                ln_ptr,
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
            if n == 0 {
                // no items
                return Ok(Vec::new());
            }

            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            crate::vkfn::enumerate_device_extension_properties(self.native_ptr(), ln_ptr, &mut n, v.as_mut_ptr())
                .into_result()?;

            Ok(v)
        }
    }

    /// Returns properties of available physical device extensions
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_LAYER_NOT_PRESENT`
    #[implements]
    #[inline]
    fn enumerate_extension_properties(&self, layer_name: Option<&str>) -> crate::Result<Vec<VkExtensionProperties>> {
        self.enumerate_extension_properties_cstr(layer_name.map(|s| std::ffi::CString::new(s).unwrap()).as_deref())
    }

    /// Reports capabilities of a physical device.
    #[implements]
    #[inline]
    fn features(&self) -> VkPhysicalDeviceFeatures {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_physical_device_features(self.native_ptr(), p.as_mut_ptr());

            p.assume_init()
        }
    }

    /// Lists physical device's format capabilities
    #[implements]
    #[inline]
    fn format_properties(&self, format: VkFormat) -> VkFormatProperties {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_physical_device_format_properties(self.native_ptr(), format, p.as_mut_ptr());

            p.assume_init()
        }
    }

    /// Lists physical device's format capabilities
    /// # Safety
    /// Caller must guarantee that all write operations to `out` are safe.
    #[implements("VK_KHR_get_physical_device_properties2")]
    #[inline]
    unsafe fn format_properties2(&self, format: VkFormat, out: &mut VkFormatProperties2KHR) {
        self.instance().get_physical_device_format_properties2_khr_fn().0(self.native_ptr(), format, out)
    }

    /// Lists physical device's image format capabilities
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_FORMAT_NOT_SUPPORTED`
    #[implements]
    #[inline]
    fn image_format_properties(
        &self,
        format: VkFormat,
        itype: VkImageType,
        tiling: VkImageTiling,
        usage: ImageUsageFlags,
        flags: ImageFlags,
    ) -> crate::Result<VkImageFormatProperties> {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_physical_device_image_format_properties(
                self.native_ptr(),
                format,
                itype,
                tiling,
                usage.bits(),
                flags.0,
                p.as_mut_ptr(),
            )
            .into_result()?;

            Ok(p.assume_init())
        }
    }

    /// Returns properties of a physical device
    #[implements]
    #[inline]
    fn properties(&self) -> VkPhysicalDeviceProperties {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_physical_device_properties(self.native_ptr(), p.as_mut_ptr());

            p.assume_init()
        }
    }

    /// Reports properties of the queues of the specified physical device
    #[implements]
    fn queue_family_properties(&self) -> QueueFamilies {
        unsafe {
            let mut n = 0;
            crate::vkfn::get_physical_device_queue_family_properties(self.native_ptr(), &mut n, core::ptr::null_mut());
            if n == 0 {
                // no items
                return QueueFamilies(Vec::new());
            }

            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            crate::vkfn::get_physical_device_queue_family_properties(self.native_ptr(), &mut n, v.as_mut_ptr());

            QueueFamilies(v)
        }
    }

    /// Reports memory information for the specified physical device
    #[implements]
    #[inline]
    fn memory_properties(&self) -> MemoryProperties {
        let mut p = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_physical_device_memory_properties(self.native_ptr(), p.as_mut_ptr());

            MemoryProperties(p.assume_init())
        }
    }

    /// Retrieve properties of an image format applied to sparse images
    #[implements]
    fn sparse_image_format_properties(
        &self,
        format: VkFormat,
        itype: VkImageType,
        samples: VkSampleCountFlags,
        usage: ImageUsageFlags,
        tiling: VkImageTiling,
    ) -> Vec<VkSparseImageFormatProperties> {
        unsafe {
            let mut n = 0;
            crate::vkfn::get_physical_device_sparse_image_format_properties(
                self.native_ptr(),
                format,
                itype,
                samples,
                usage.bits(),
                tiling,
                &mut n,
                core::ptr::null_mut(),
            );
            if n == 0 {
                // no items
                return Vec::new();
            }

            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            crate::vkfn::get_physical_device_sparse_image_format_properties(
                self.native_ptr(),
                format,
                itype,
                samples,
                usage.bits(),
                tiling,
                &mut n,
                v.as_mut_ptr(),
            );

            v
        }
    }

    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[implements("VK_EXT_sample_locations")]
    unsafe fn multisample_properties(
        &self,
        samples: VkSampleCountFlags,
        sink: &mut core::mem::MaybeUninit<VkMultisamplePropertiesEXT>,
    ) {
        self.instance().get_physical_device_multisample_properties_ext_fn().0(
            self.native_ptr(),
            samples,
            sink.as_mut_ptr(),
        );
    }

    /// Function for querying external fence handle capabilities
    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[implements("VK_KHR_external_fence_capabilities")]
    #[inline]
    unsafe fn external_fence_properties(
        &self,
        info: &VkPhysicalDeviceExternalFenceInfoKHR,
        sink: &mut core::mem::MaybeUninit<VkExternalFencePropertiesKHR>,
    ) {
        self.instance().get_physical_device_external_fence_properties_khr_fn().0(
            self.native_ptr(),
            info,
            sink.as_mut_ptr(),
        );
    }

    /// Query if presentation is supported
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline]
    fn surface_support(&self, queue_family: u32, surface: &(impl Surface + ?Sized)) -> crate::Result<bool> {
        let mut f = 0;

        unsafe {
            crate::vkfn::get_physical_device_surface_support_khr(
                self.native_ptr(),
                queue_family,
                surface.native_ptr(),
                &mut f,
            )
            .into_result()?;

            Ok(f != 0)
        }
    }

    /// Query surface capabilities
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline]
    fn surface_capabilities(&self, surface: &(impl Surface + ?Sized)) -> crate::Result<VkSurfaceCapabilitiesKHR> {
        let mut s = std::mem::MaybeUninit::uninit();

        unsafe {
            crate::vkfn::get_physical_device_surface_capabilities_khr(
                self.native_ptr(),
                surface.native_ptr(),
                s.as_mut_ptr(),
            )
            .into_result()?;

            Ok(s.assume_init())
        }
    }

    /// Query color formats supported by surface
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    fn surface_formats(&self, surface: &(impl Surface + ?Sized)) -> crate::Result<Vec<VkSurfaceFormatKHR>> {
        unsafe {
            let mut n = 0;
            crate::vkfn::get_physical_device_surface_formats_khr(
                self.native_ptr(),
                surface.native_ptr(),
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
            if n == 0 {
                // no items
                return Ok(Vec::new());
            }

            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            crate::vkfn::get_physical_device_surface_formats_khr(
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
    #[implements("VK_KHR_surface")]
    fn surface_present_modes(&self, surface: &(impl Surface + ?Sized)) -> crate::Result<Vec<PresentMode>> {
        unsafe {
            let mut n = 0;
            crate::vkfn::get_physical_device_surface_present_modes_khr(
                self.native_ptr(),
                surface.native_ptr(),
                &mut n,
                std::ptr::null_mut(),
            )
            .into_result()?;
            if n == 0 {
                // no items
                return Ok(Vec::new());
            }

            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            crate::vkfn::get_physical_device_surface_present_modes_khr(
                self.native_ptr(),
                surface.native_ptr(),
                &mut n,
                v.as_mut_ptr(),
            )
            .into_result()?;

            Ok(core::mem::transmute(v))
        }
    }

    /// Query physical device for presentation to X11 server using Xlib
    /// # Safety
    /// Provided `display` must be a valid reference
    #[implements("VK_KHR_xlib_surface")]
    #[inline]
    unsafe fn xlib_presentation_support(
        &self,
        queue_family: u32,
        display: *mut x11::xlib::Display,
        visual: x11::xlib::VisualID,
    ) -> bool {
        crate::vkfn::get_physical_device_xlib_presentation_support_khr(self.native_ptr(), queue_family, display, visual)
            != 0
    }

    /// Query physical device for presentation to X11 server using XCB
    /// # Safety
    /// Provided `connection` must be a valid reference
    #[implements("VK_KHR_xcb_surface")]
    #[inline]
    unsafe fn xcb_presentation_support(
        &self,
        queue_family: u32,
        connection: *mut xcb::ffi::xcb_connection_t,
        visual: xcb::x::Visualid,
    ) -> bool {
        crate::vkfn::get_physical_device_xcb_presentation_support_khr(
            self.native_ptr(),
            queue_family,
            connection,
            visual,
        ) != 0
    }

    /// Query physical device for presentation to Wayland
    /// # Safety
    /// Provided `display` must be a valid reference
    #[implements("VK_KHR_wayland_surface")]
    #[inline]
    unsafe fn wayland_presentation_support(&self, queue_family: u32, display: *mut core::ffi::c_void) -> bool {
        crate::vkfn::get_physical_device_wayland_presentation_support_khr(self.native_ptr(), queue_family, display) != 0
    }

    /// Query queue family support for presentation on a Win32 display
    #[implements("VK_KHR_win32_surface")]
    #[inline]
    fn win32_presentation_support(&self, queue_family: u32) -> bool {
        unsafe { crate::vkfn::get_physical_device_win32_presentation_support_khr(self.native_ptr(), queue_family) != 0 }
    }

    /// Query the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    fn display_mode_properties(&self, display: VkDisplayKHR) -> crate::Result<Vec<VkDisplayModePropertiesKHR>> {
        unsafe {
            let mut n = 0;
            crate::vkfn::get_display_mode_properties_khr(self.native_ptr(), display, &mut n, core::ptr::null_mut())
                .into_result()?;
            if n == 0 {
                // no items
                return Ok(Vec::new());
            }

            let mut v = Vec::with_capacity(n as _);
            v.set_len(n as _);
            crate::vkfn::get_display_mode_properties_khr(self.native_ptr(), display, &mut n, v.as_mut_ptr())
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
    ///
    /// # Safety
    /// no guarantee will be provided (simply calls under api)
    #[implements("VK_KHR_display")]
    #[inline]
    unsafe fn new_display_mode_raw(
        &self,
        display: VkDisplayKHR,
        info: &VkDisplayModeCreateInfoKHR,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkDisplayModeKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_display_mode_khr(
            self.native_ptr(),
            display,
            info,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;

        Ok(h.assume_init())
    }

    /// Query capabilities of a mode and plane combination
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline]
    fn display_plane_capabilities(
        &self,
        mode: VkDisplayModeKHR,
        plane_index: u32,
    ) -> crate::Result<VkDisplayPlaneCapabilitiesKHR> {
        let mut s = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_display_plane_capabilities_khr(self.native_ptr(), mode, plane_index, s.as_mut_ptr())
                .into_result()?;

            Ok(s.assume_init())
        }
    }

    /// Query information about the available displays.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements("VK_KHR_display")]
    fn display_properties(&self) -> crate::Result<Vec<DisplayProperties<&Self>>> {
        unsafe {
            let mut n = 0;
            crate::vkfn::get_physical_device_display_properties_khr(self.native_ptr(), &mut n, core::ptr::null_mut())
                .into_result()?;
            if n == 0 {
                // no items
                return Ok(Vec::new());
            }

            let mut v = Vec::with_capacity(n as usize);
            v.set_len(n as usize);
            crate::vkfn::get_physical_device_display_properties_khr(
                self.native_ptr(),
                &mut n,
                v.as_mut_ptr() as *mut _,
            )
            .into_result()?;

            Ok(v.into_iter().map(|x| DisplayProperties(x, self)).collect())
        }
    }

    /// Query the plane properties.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements("VK_KHR_display")]
    fn display_plane_properties(&self) -> crate::Result<Vec<DisplayPlaneProperties<&Self>>> {
        unsafe {
            let mut n = 0;
            crate::vkfn::get_physical_device_display_plane_properties_khr(
                self.native_ptr(),
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
            if n == 0 {
                // no items
                return Ok(Vec::new());
            }

            let mut v = Vec::with_capacity(n as usize);
            v.set_len(n as usize);
            crate::vkfn::get_physical_device_display_plane_properties_khr(
                self.native_ptr(),
                &mut n,
                v.as_mut_ptr() as *mut _,
            )
            .into_result()?;

            Ok(v.into_iter().map(|x| DisplayPlaneProperties(x, self)).collect())
        }
    }

    /// Query the list of displays a plane supports.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements("VK_KHR_display")]
    fn display_plane_supported_displays(&self, plane_index: u32) -> crate::Result<Vec<Display<&Self>>> {
        unsafe {
            let mut n = 0;
            crate::vkfn::get_display_plane_supported_displays_khr(
                self.native_ptr(),
                plane_index,
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
            if n == 0 {
                // no items
                return Ok(Vec::new());
            }

            let mut v = Vec::with_capacity(n as usize);
            v.set_len(n as usize);
            crate::vkfn::get_display_plane_supported_displays_khr(
                self.native_ptr(),
                plane_index,
                &mut n,
                v.as_mut_ptr() as *mut _,
            )
            .into_result()?;

            Ok(v.into_iter().map(|x| Display(x, self)).collect())
        }
    }

    /// Query the VkDisplayKHR corresponding to an X11 RandR Output
    /// # Failures
    /// On failure, this command returns
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///
    /// # Safety
    /// Provided `dpy` must be a valid reference
    #[implements("VK_EXT_acquire_xlib_display")]
    #[inline]
    unsafe fn get_randr_output_display(
        self,
        dpy: *mut x11::xlib::Display,
        rr_output: x11::xrandr::RROutput,
    ) -> crate::Result<Display<Self>>
    where
        Self: Sized,
    {
        let mut d = core::mem::MaybeUninit::uninit();

        self.instance().get_randr_output_display_ext_fn().0(self.native_ptr(), dpy, rr_output, d.as_mut_ptr())
            .into_result()?;

        Ok(Display(d.assume_init(), self))
    }

    /// Create a `Surface` object representing a display plane and mode
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantee will be provided (simply calls under api)
    #[implements("VK_KHR_display", "VK_KHR_surface")]
    #[inline]
    unsafe fn new_surface_for_display_plane_raw(
        &self,
        info: &VkDisplaySurfaceCreateInfoKHR,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_display_plane_surface_khr(
            self.instance().native_ptr(),
            info,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;

        Ok(h.assume_init())
    }

    /// Reports capabilities of a surface on a physical device
    ///
    /// # Failures
    ///
    /// On failure, this command returns
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    ///
    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[implements("VK_KHR_get_surface_capabilities2")]
    #[inline]
    unsafe fn surface_capabilities2(
        &self,
        surface_info: &VkPhysicalDeviceSurfaceInfo2KHR,
        sink: &mut core::mem::MaybeUninit<VkSurfaceCapabilities2KHR>,
    ) -> crate::Result<()> {
        self.instance().get_physical_device_surface_capabilities_2_khr_fn().0(
            self.native_ptr(),
            surface_info,
            sink.as_mut_ptr(),
        )
        .into_result()
        .map(drop)
    }

    /// Returns properties of a physical device
    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[implements("VK_KHR_get_physical_device_properties2")]
    #[inline]
    unsafe fn properties2(&self, sink: &mut core::mem::MaybeUninit<VkPhysicalDeviceProperties2KHR>) {
        self.instance().get_physical_device_properties2_khr_fn().0(self.native_ptr(), sink.as_mut_ptr());
    }

    /// Reports capabilities of a physical device
    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[implements("VK_KHR_get_physical_device_properties2")]
    #[inline]
    unsafe fn features2(&self, sink: &mut core::mem::MaybeUninit<VkPhysicalDeviceFeatures2KHR>) {
        self.instance().get_physical_device_features2_khr_fn().0(self.native_ptr(), sink.as_mut_ptr());
    }

    /// Query supported presentation modes.
    ///
    /// # Failures
    ///
    /// On failure, this command returns
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[implements("VK_EXT_full_screen_exclusive")]
    fn surface_present_modes2(
        &self,
        surface_info: &VkPhysicalDeviceSurfaceInfo2KHR,
    ) -> crate::Result<Vec<VkPresentModeKHR>> {
        let mut n = 0;
        unsafe {
            self.instance().get_physical_device_surface_present_modes_2_ext_fn().0(
                self.native_ptr(),
                surface_info,
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
        }
        if n == 0 {
            // no items
            return Ok(Vec::new());
        }

        let mut x = Vec::with_capacity(n as _);
        unsafe {
            x.set_len(n as _);
            self.instance().get_physical_device_surface_present_modes_2_ext_fn().0(
                self.native_ptr(),
                surface_info,
                &mut n,
                x.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(x)
    }
}
DerefContainerBracketImpl!(for PhysicalDevice {});
GuardsImpl!(for PhysicalDevice {});

pub trait InstanceChild {
    type ConcreteInstance: Instance;

    fn instance(&self) -> &Self::ConcreteInstance;
}
DerefContainerBracketImpl!(for InstanceChild {
    type ConcreteInstance = T::ConcreteInstance;

    #[inline(always)]
    fn instance(&self) -> &Self::ConcreteInstance { T::instance(self) }
});
GuardsImpl!(for InstanceChild {
    type ConcreteInstance = T::ConcreteInstance;

    #[inline(always)]
    fn instance(&self) -> &Self::ConcreteInstance { T::instance(&self) }
});

pub trait InstanceChildTransferrable: InstanceChild {
    fn transfer_instance(self) -> Self::ConcreteInstance;
}
impl<T> InstanceChildTransferrable for &'_ T
where
    T: InstanceChild,
    T::ConcreteInstance: Clone,
{
    #[inline(always)]
    fn transfer_instance(self) -> Self::ConcreteInstance {
        self.instance().clone()
    }
}

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl VkExternalFencePropertiesKHR {
    pub const fn is_exportable(&self) -> bool {
        (self.externalFenceFeatures & VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR) != 0
    }

    pub const fn is_importable(&self) -> bool {
        (self.externalFenceFeatures & VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR) != 0
    }
}

/// Device memory properties
#[repr(transparent)]
pub struct MemoryProperties(VkPhysicalDeviceMemoryProperties);
impl MemoryProperties {
    #[inline(always)]
    pub fn types(&self) -> &[VkMemoryType] {
        &self.0.memoryTypes[..self.0.memoryTypeCount as _]
    }
    #[inline(always)]
    pub fn heaps(&self) -> &[VkMemoryHeap] {
        &self.0.memoryHeaps[..self.0.memoryHeapCount as _]
    }

    #[inline]
    pub fn find_type_index(
        &self,
        mask: MemoryPropertyFlags,
        exclude: MemoryPropertyFlags,
        index_mask: u32,
    ) -> Option<u32> {
        self.types().iter().enumerate().find_map(|(i, mt)| {
            (index_mask & (1u32 << i) != 0 && (mt.propertyFlags & mask.0) != 0 && (mt.propertyFlags & exclude.0) == 0)
                .then_some(i as _)
        })
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
}

/// Bitmask specifying properties for a memory type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MemoryPropertyFlags(pub VkMemoryPropertyFlags);
impl MemoryPropertyFlags {
    /// Empty set
    pub const EMPTY: Self = MemoryPropertyFlags(0);
    /// Memory allocated with this type is the most efficient for device access
    pub const DEVICE_LOCAL: Self = MemoryPropertyFlags(VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
    /// Memory allocated with this type can be mapped for host access using `vkMapMemory`
    pub const HOST_VISIBLE: Self = MemoryPropertyFlags(VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT);
    /// The host cache management commands `vkFlushMappedMemoryRanges` and `vkInvalidateMappedMemoryRanges`
    /// are not needed to flush host writes to the device or make device writes visible to the host, respectively.
    pub const HOST_COHERENT: Self = MemoryPropertyFlags(VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
    /// Memory allocated with this type is cached on the host.
    /// Host memory accesses to uncached memory are slower than to cached memory, however uncached memory is always host coherent
    pub const HOST_CACHED: Self = MemoryPropertyFlags(VK_MEMORY_PROPERTY_HOST_CACHED_BIT);
    /// The memory type only allows device access to the memory.
    pub const LAZILY_ALLOCATED: Self = MemoryPropertyFlags(VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT);

    #[inline]
    /// Memory allocated with this type is the most efficient for device access
    pub const fn device_local(self) -> Self {
        Self(self.0 | Self::DEVICE_LOCAL.0)
    }
    #[inline]
    /// Memory allocated with this type can be mapped for host access using `vkMapMemory`
    pub const fn host_visible(self) -> Self {
        Self(self.0 | Self::HOST_VISIBLE.0)
    }
    #[inline]
    /// The host cache management commands `vkFlushMappedmemoryRanges` and `vkInvalidateMappedMemoryRanges`
    /// are not needed to flush host writes to the device or make device writes visible to the host, respectively.
    pub const fn host_coherent(self) -> Self {
        Self(self.0 | Self::HOST_COHERENT.0)
    }
    #[inline]
    /// Memory allocated with this type is cached on the host.
    /// Host memory accesses to uncached memory are slower than to cached memory, however uncached memory is always host coherent
    pub const fn host_cached(self) -> Self {
        Self(self.0 | Self::HOST_CACHED.0)
    }
    #[inline]
    /// The memory type only allows device access to the memory.
    pub const fn lazily_allocated(self) -> Self {
        Self(self.0 | Self::LAZILY_ALLOCATED.0)
    }

    #[inline]
    pub const fn bits(self) -> VkMemoryPropertyFlags {
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
    pub fn find_matching_index(&self, flags: QueueFlags) -> Option<u32> {
        self.0
            .iter()
            .position(|q| (q.queueFlags & flags.0) != 0)
            .map(|x| x as _)
    }

    /// Find a queue family index containing specified bitflags
    pub fn find_another_matching_index(&self, flags: QueueFlags, exclude: u32) -> Option<u32> {
        self.0
            .iter()
            .enumerate()
            .find_map(|(n, &VkQueueFamilyProperties { queueFlags, .. })| {
                ((queueFlags & flags.0) != 0 && exclude != n as u32).then_some(n as _)
            })
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
