//! Vulkan Base Objects(Instance/PhysicalDevice)

use crate::ffi_helper::{CStrFFIRef, opt_cstr_ptr, opt_pointer, slice_as_ptr_empty_null};
use crate::*;
use derives::implements;

#[cfg(feature = "Multithreaded")]
struct LazyCellReadRef<'d, T>(::std::sync::RwLockReadGuard<'d, Option<T>>);
#[cfg(feature = "Multithreaded")]
impl<'d, T> ::std::ops::Deref for LazyCellReadRef<'d, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.as_ref().unwrap()
    }
}

/// A Value Object represents the Vulkan version number
// Note: (MSB) major | minor | patch (LSB) の順でビットが割り当てられているので合成した状態の比較で正しい順序になる
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(u32);
impl Version {
    /// Version 1.0.0
    pub const V1: Self = Self::new(0, 1, 0, 0);

    /// Construct an object from discrete values
    pub const fn new(variant: u8, major: u16, minor: u16, patch: u16) -> Self {
        Self(crate::vk::VK_MAKE_VERSION(variant, major, minor, patch))
    }

    /// Construct an object from a raw value
    pub const fn from_raw(v: u32) -> Self {
        Self(v)
    }

    /// Gets a raw value from this object
    pub const fn raw(&self) -> u32 {
        self.0
    }

    /// Version variant number
    pub const fn variant(&self) -> u8 {
        crate::vk::VK_VARIANT_VERSION(self.0)
    }

    /// Major version number
    pub const fn major(&self) -> u16 {
        crate::vk::VK_MAJOR_VERSION(self.0)
    }

    /// Minor version number
    pub const fn minor(&self) -> u16 {
        crate::vk::VK_MINOR_VERSION(self.0)
    }

    /// Patch version number
    pub const fn patch(&self) -> u16 {
        crate::vk::VK_PATCH_VERSION(self.0)
    }
}
impl core::fmt::Display for Version {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Variantを含めたバージョン表示
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

/// Query instance-level version before instance creation
/// # Failures
/// On failure, this command returns
///
/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
#[inline]
#[implements]
pub fn instance_version() -> crate::Result<Version> {
    #[cfg(feature = "Allow1_1APIs")]
    unsafe {
        let mut sink = 0;
        crate::vkfn::enumerate_instance_version(&mut sink).into_result()?;
        Ok(Version(sink))
    }
    #[cfg(not(feature = "Allow1_1APIs"))]
    {
        // fixed to v1.0.0
        Ok(Version::V1)
    }
}

#[implements]
type InstanceResolvedFn<F> = crate::resolver::ResolvedFnCell<F, VkInstance>;
#[implements]
impl crate::resolver::ResolverInterface for VkInstance {
    unsafe fn load_symbol_unconstrainted<T: crate::resolver::FromPtr>(&self, name: &core::ffi::CStr) -> T {
        unsafe {
            T::from_ptr(core::mem::transmute(crate::vkfn::get_instance_proc_addr(
                *self,
                name.as_ptr() as _,
            )))
        }
    }

    unsafe fn load_function_unconstrainted<F: crate::resolver::PFN>(&self) -> F {
        unsafe {
            F::from_void_fn(
                crate::vkfn::get_instance_proc_addr(*self, F::NAME_CSTR.as_ptr() as _)
                    .unwrap_or_else(|| panic!("function {:?} not found", F::NAME_CSTR)),
            )
        }
    }
}

#[implements]
struct InstanceExtFunctions {
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    get_physical_device_properties2_khr: InstanceResolvedFn<PFN_vkGetPhysicalDeviceProperties2KHR>,
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    get_physical_device_features2_khr: InstanceResolvedFn<PFN_vkGetPhysicalDeviceFeatures2KHR>,
    #[cfg(not(feature = "Allow1_1APIs"))]
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
            #[cfg(not(feature = "Allow1_1APIs"))]
            #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
            get_physical_device_properties2_khr: InstanceResolvedFn::new(r),
            #[cfg(not(feature = "Allow1_1APIs"))]
            #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
            get_physical_device_features2_khr: InstanceResolvedFn::new(r),
            #[cfg(not(feature = "Allow1_1APIs"))]
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
impl Instance for InstanceObject {}
impl InstanceExtensions for InstanceObject {
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_physical_device_properties2")]
    fn get_physical_device_properties2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceProperties2KHR {
        *self.ext.get_physical_device_properties2_khr.resolve()
    }
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_physical_device_properties2")]
    fn get_physical_device_features2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceFeatures2KHR {
        *self.ext.get_physical_device_features2_khr.resolve()
    }
    #[cfg(not(feature = "Allow1_1APIs"))]
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
impl<Owner: Instance> PhysicalDeviceObject<Owner> {
    pub const unsafe fn manage(handle: VkPhysicalDevice, owner: Owner) -> Self {
        Self(handle, owner)
    }

    pub const fn unmanage(self) -> (VkPhysicalDevice, Owner) {
        let handle = self.0;
        let owner = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (handle, owner)
    }
}
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

#[repr(transparent)]
pub struct ApplicationInfo<'d>(VkApplicationInfo, core::marker::PhantomData<&'d core::ffi::CStr>);
impl<'d> ApplicationInfo<'d> {
    #[inline(always)]
    pub const fn new(
        app_name: &'d core::ffi::CStr,
        app_version: Version,
        engine_name: &'d core::ffi::CStr,
        engine_version: Version,
    ) -> Self {
        Self(
            VkApplicationInfo {
                sType: VkApplicationInfo::TYPE,
                pNext: core::ptr::null(),
                apiVersion: VK_API_VERSION_1_0,
                pApplicationName: app_name.as_ptr(),
                applicationVersion: app_version.0,
                pEngineName: engine_name.as_ptr(),
                engineVersion: engine_version.0,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub const fn api_version(mut self, version: Version) -> Self {
        self.0.apiVersion = version.0;
        self
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct InstanceCreateInfo<'d>(
    VkInstanceCreateInfo,
    core::marker::PhantomData<(
        Option<&'d dyn VulkanStructureAsRef>,
        &'d ApplicationInfo<'d>,
        &'d [CStrFFIRef<'d>],
    )>,
);
impl<'d> InstanceCreateInfo<'d> {
    pub const fn new(
        application_info: &'d ApplicationInfo<'d>,
        layers: &'d [CStrFFIRef<'d>],
        extensions: &'d [CStrFFIRef<'d>],
    ) -> Self {
        Self(
            VkInstanceCreateInfo {
                sType: VkInstanceCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                pApplicationInfo: &application_info.0 as *const _,
                enabledLayerCount: layers.len() as _,
                ppEnabledLayerNames: slice_as_ptr_empty_null(layers) as _,
                enabledExtensionCount: extensions.len() as _,
                ppEnabledExtensionNames: slice_as_ptr_empty_null(extensions) as _,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkInstanceCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkInstanceCreateInfo {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl VulkanStructureAsRef + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
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
    #[implements]
    #[inline]
    pub fn new(info: &InstanceCreateInfo) -> crate::Result<Self> {
        unsafe { Ok(Self::manage(new_instance_raw(info, None)?)) }
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
    info: &InstanceCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkInstance> {
    let mut h = core::mem::MaybeUninit::uninit();

    unsafe {
        crate::vkfn::create_instance(&info.0, opt_pointer(allocation_callbacks), h.as_mut_ptr()).into_result()?;
    }
    Ok(unsafe { h.assume_init() })
}

/// Returns a count of all of global layer properties
/// # Failures
/// On failure, this command returns
///
/// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
/// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
#[implements]
#[inline]
pub fn instance_layer_property_count() -> crate::Result<u32> {
    let mut n = 0;
    unsafe {
        crate::vkfn::enumerate_instance_layer_properties(&mut n, core::ptr::null_mut()).into_result()?;
    }

    Ok(n)
}

/// Returns up to all of global layer properties
/// # Failures
/// On failure, this command returns
///
/// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
/// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
#[implements]
#[inline]
pub fn instance_layer_properties(sink: &mut [VkLayerProperties]) -> crate::Result<u32> {
    let mut n = sink.len() as _;
    unsafe {
        crate::vkfn::enumerate_instance_layer_properties(&mut n, sink.as_mut_ptr()).into_result()?;
    }

    Ok(n)
}

/// Returns up to all of global layer properties
/// # Failures
/// On failure, this command returns
///
/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
#[implements("alloc")]
pub fn enumerate_layer_properties_alloc() -> crate::Result<Vec<VkLayerProperties>> {
    let n = instance_layer_property_count()?;
    if n == 0 {
        // no items
        return Ok(crate::alloc::empty_sink_buffer());
    }

    let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
    instance_layer_properties(&mut xs)?;

    Ok(xs)
}

/// Returns a count up to all of global extension properties
/// # Failures
/// On failure, this command returns
///
/// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
/// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
/// * [`VK_ERROR_LAYER_NOT_PRESENT`]
#[implements]
#[inline]
pub fn instance_extension_property_count_cstr(layer_name: Option<&core::ffi::CStr>) -> crate::Result<u32> {
    let mut n = 0;
    unsafe {
        crate::vkfn::enumerate_instance_extension_properties(opt_cstr_ptr(layer_name), &mut n, core::ptr::null_mut())
            .into_result()?;
    }

    Ok(n)
}

/// Returns up to all of global extension properties
/// # Failures
/// On failure, this command returns
///
/// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
/// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
/// * [`VK_ERROR_LAYER_NOT_PRESENT`]
#[implements]
#[inline]
pub fn instance_extension_properties_cstr(
    layer_name: Option<&core::ffi::CStr>,
    sink: &mut [VkExtensionProperties],
) -> crate::Result<u32> {
    let mut n = sink.len() as _;
    unsafe {
        crate::vkfn::enumerate_instance_extension_properties(opt_cstr_ptr(layer_name), &mut n, sink.as_mut_ptr())
            .into_result()?;
    }

    Ok(n)
}

/// Returns up to all of global extension properties
/// # Failures
/// On failure, this command returns
///
/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// * `VK_ERROR_LAYER_NOT_PRESENT`
#[implements("alloc")]
pub fn instance_extension_properties_cstr_alloc(
    layer_name: Option<&core::ffi::CStr>,
) -> crate::Result<Vec<VkExtensionProperties>> {
    let n = instance_extension_property_count_cstr(layer_name)?;
    if n == 0 {
        // no items
        return Ok(crate::alloc::empty_sink_buffer());
    }

    let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
    instance_extension_properties_cstr(layer_name, &mut xs)?;

    Ok(xs)
}

/// Returns up to all of global extension properties
/// # Failures
/// On failure, this command returns
///
/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// * `VK_ERROR_LAYER_NOT_PRESENT`
#[implements("alloc")]
pub fn instance_extension_properties(layer_name: Option<&str>) -> crate::Result<Vec<VkExtensionProperties>> {
    instance_extension_properties_cstr_alloc(layer_name.map(|s| crate::alloc::str_to_cstr(s).unwrap()).as_deref())
}

/// A Vulkan Instance interface
pub trait Instance: VkHandle<Handle = VkInstance> {
    /// Return a function pointer for a command
    #[deprecated = "do not use this directly(this does not provide caching!)"]
    #[implements]
    #[inline]
    fn extra_procedure(&self, name: &core::ffi::CStr) -> Option<PFN_vkVoidFunction> {
        unsafe { crate::vkfn::get_instance_proc_addr(self.native_ptr(), name.as_ptr()) }
    }

    /// Counts the physical devices accessible to a Vulkan instance
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INITIALIZATION_FAILED`]
    #[implements]
    #[inline]
    fn physical_device_count(&self) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::enumerate_physical_devices(self.native_ptr(), &mut n, core::ptr::null_mut()).into_result()?;
        }

        Ok(n)
    }

    /// Enumerates the physical devices accessible to a Vulkan instance
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INITIALIZATION_FAILED`]
    #[implements]
    #[inline]
    fn enumerate_physical_devices(&self, sink: &mut [VkPhysicalDevice]) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::enumerate_physical_devices(self.native_ptr(), &mut n, sink.as_mut_ptr()).into_result()?;
        }

        Ok(n)
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
    #[implements("alloc")]
    fn iter_physical_devices(&self) -> crate::Result<IterPhysicalDevices<Self>> {
        let n = self.physical_device_count()?;
        if n == 0 {
            // no items
            return Ok(IterPhysicalDevices(crate::alloc::empty_sink_buffer(), 0, self));
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.enumerate_physical_devices(&mut xs)?;

        Ok(IterPhysicalDevices(xs, 0, self))
    }

    /// Enumerates the physical devices accessible to a Vulkan instance
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_INITIALIZATION_FAILED`
    #[implements("alloc")]
    fn enumerate_physical_devices_alloc(&self) -> crate::Result<Vec<PhysicalDeviceObject<&Self>>> {
        self.iter_physical_devices().map(crate::alloc::collect_vec)
    }
}
DerefContainerWithGuardsBracketImpl!(for Instance {});
/// VkInstance Extension Function Providers
pub trait InstanceExtensions: VkHandle<Handle = VkInstance> {
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_physical_device_properties2")]
    fn get_physical_device_properties2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceProperties2KHR;
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_physical_device_properties2")]
    fn get_physical_device_features2_khr_fn(&self) -> PFN_vkGetPhysicalDeviceFeatures2KHR;
    #[cfg(not(feature = "Allow1_1APIs"))]
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

    /// Register a debug report callback
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    ///
    /// # Safety
    /// must not execute in parallel with any Vulkan commands
    #[implements("VK_EXT_debug_report")]
    #[inline]
    unsafe fn new_debug_report_callback_raw(
        &self,
        info: &crate::DebugReportCallbackCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkDebugReportCallbackEXT> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            self.create_debug_report_callback_ext_fn().0(
                self.native_ptr(),
                info.as_raw_ref(),
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
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
        layer_prefix: &core::ffi::CStr,
        message: &core::ffi::CStr,
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

    /// Destroy a debug report callback object
    /// # Safety
    /// * must be created from this Instance object
    /// * must not execute in parallel with any Vulkan commands
    #[implements("VK_EXT_debug_report")]
    #[inline]
    unsafe fn destroy_debug_report_callback_raw(
        &self,
        obj: VkDebugReportCallbackEXT,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) {
        unsafe {
            self.destroy_debug_report_callback_ext_fn().0(self.native_ptr(), obj, opt_pointer(allocation_callbacks));
        }
    }

    /// Create a debug messenger object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    ///
    /// # Safety
    /// must not be executed in parallel with any Vulkan commands
    #[implements("VK_EXT_debug_utils")]
    #[inline]
    unsafe fn new_debug_utils_messenger_raw(
        &self,
        info: &crate::DebugUtilsMessengerCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkDebugUtilsMessengerEXT> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            self.create_debug_utils_messenger_ext_fn().0(
                self.native_ptr(),
                info,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Destroy a debug messenger object
    /// # Safety
    /// * must be created from this Instance object
    /// * must not execute in parallel with any Vulkan commands
    #[implements("VK_EXT_debug_utils")]
    #[inline]
    unsafe fn destroy_debug_utils_messenger_raw(
        &self,
        obj: VkDebugUtilsMessengerEXT,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) {
        unsafe {
            self.destroy_debug_utils_messenger_ext_fn().0(self.native_ptr(), obj, opt_pointer(allocation_callbacks));
        }
    }
}
DerefContainerWithGuardsBracketImpl!(for InstanceExtensions {
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_physical_device_properties2")]
    ForwardFnPtr!(deref get_physical_device_properties2_khr_fn -> PFN_vkGetPhysicalDeviceProperties2KHR);
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_physical_device_properties2")]
    ForwardFnPtr!(deref get_physical_device_features2_khr_fn -> PFN_vkGetPhysicalDeviceFeatures2KHR);
    #[cfg(not(feature = "Allow1_1APIs"))]
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
    /// Returns a count of properties of available physical device layers
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn layer_property_count(&self) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::enumerate_device_layer_properties(self.native_ptr(), &mut n, core::ptr::null_mut())
                .into_result()?;
        }

        Ok(n)
    }

    /// Returns properties of available physical device layers
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn enumerate_layer_properties(&self, sink: &mut [VkLayerProperties]) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::enumerate_device_layer_properties(self.native_ptr(), &mut n, sink.as_mut_ptr())
                .into_result()?;
        }

        Ok(n)
    }

    /// Returns properties of available physical device layers
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("alloc")]
    fn enumerate_layer_properties_alloc(&self) -> crate::Result<Vec<VkLayerProperties>> {
        let count = self.layer_property_count()?;
        if count == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(count as _) };
        self.enumerate_layer_properties(&mut xs)?;

        Ok(xs)
    }

    /// Returns a count of properties of available physical device extensions
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_LAYER_NOT_PRESENT`]
    #[implements]
    #[inline]
    fn extension_property_count_cstr(&self, layer_name: Option<&core::ffi::CStr>) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::enumerate_device_extension_properties(
                self.native_ptr(),
                opt_cstr_ptr(layer_name),
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Returns properties of available physical device extensions
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_LAYER_NOT_PRESENT`]
    #[implements]
    #[inline]
    fn enumerate_extension_properties_cstr(
        &self,
        layer_name: Option<&core::ffi::CStr>,
        sink: &mut [VkExtensionProperties],
    ) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::enumerate_device_extension_properties(
                self.native_ptr(),
                opt_cstr_ptr(layer_name),
                &mut n,
                sink.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Returns properties of available physical device extensions
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_LAYER_NOT_PRESENT`
    #[implements("alloc")]
    fn enumerate_extension_properties_cstr_alloc(
        &self,
        layer_name: Option<&core::ffi::CStr>,
    ) -> crate::Result<Vec<VkExtensionProperties>> {
        let n = self.extension_property_count_cstr(layer_name)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.enumerate_extension_properties_cstr(layer_name, &mut xs)?;

        Ok(xs)
    }

    /// Returns properties of available physical device extensions
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_LAYER_NOT_PRESENT`
    #[implements("alloc")]
    #[inline]
    fn enumerate_extension_properties(&self, layer_name: Option<&str>) -> crate::Result<Vec<VkExtensionProperties>> {
        self.enumerate_extension_properties_cstr_alloc(
            layer_name.map(|s| crate::alloc::str_to_cstr(s).unwrap()).as_deref(),
        )
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
    #[implements("Allow1_1APIs")]
    #[inline]
    unsafe fn format_properties2(&self, format: VkFormat, out: &mut VkFormatProperties2KHR) {
        unsafe {
            crate::vkfn::get_physical_device_format_properties2(self.native_ptr(), format, out);
        }
    }

    /// Lists physical device's format capabilities
    /// # Safety
    /// Caller must guarantee that all write operations to `out` are safe.
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_physical_device_properties2")]
    #[inline]
    unsafe fn format_properties2(&self, format: VkFormat, out: &mut VkFormatProperties2KHR)
    where
        Self::ConcreteInstance: InstanceExtensions,
    {
        unsafe {
            self.instance().get_physical_device_format_properties2_khr_fn().0(self.native_ptr(), format, out);
        }
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
                flags.bits(),
                p.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { p.assume_init() })
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

    /// Reports a count of properties of the queues of the specified physical device
    #[implements]
    #[inline]
    fn queue_family_property_count(&self) -> u32 {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_physical_device_queue_family_properties(self.native_ptr(), &mut n, core::ptr::null_mut());
        }

        n
    }

    /// Reports properties of the queues of the specified physical device
    #[implements]
    #[inline]
    fn queue_family_properties(&self, sink: &mut [VkQueueFamilyProperties]) -> u32 {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_physical_device_queue_family_properties(self.native_ptr(), &mut n, sink.as_mut_ptr());
        }

        n
    }

    /// Reports properties of the queues of the specified physical device
    #[implements]
    #[cfg(feature = "alloc")]
    fn queue_family_properties_alloc(&self) -> QueueFamilies {
        let n = self.queue_family_property_count();
        if n == 0 {
            // no items
            return QueueFamilies(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.queue_family_properties(&mut xs);

        QueueFamilies(xs)
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

    /// Retrieve a count of properties of an image format spplied to sparse images
    #[implements]
    #[inline]
    fn sparse_image_format_property_count(
        &self,
        format: VkFormat,
        image_type: VkImageType,
        samples: VkSampleCountFlags,
        usage: ImageUsageFlags,
        tiling: VkImageTiling,
    ) -> u32 {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_physical_device_sparse_image_format_properties(
                self.native_ptr(),
                format,
                image_type,
                samples,
                usage.bits(),
                tiling,
                &mut n,
                core::ptr::null_mut(),
            );
        }

        n
    }

    /// Retrieve properties of an image format applied to sparse images
    #[implements]
    #[inline]
    fn sparse_image_format_properties(
        &self,
        format: VkFormat,
        image_type: VkImageType,
        samples: VkSampleCountFlags,
        usage: ImageUsageFlags,
        tiling: VkImageTiling,
        sink: &mut [VkSparseImageFormatProperties],
    ) -> u32 {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_physical_device_sparse_image_format_properties(
                self.native_ptr(),
                format,
                image_type,
                samples,
                usage.bits(),
                tiling,
                &mut n,
                sink.as_mut_ptr(),
            );
        }

        n
    }

    /// Retrieve properties of an image format applied to sparse images
    #[implements]
    #[cfg(feature = "alloc")]
    fn sparse_image_format_properties_alloc(
        &self,
        format: VkFormat,
        itype: VkImageType,
        samples: VkSampleCountFlags,
        usage: ImageUsageFlags,
        tiling: VkImageTiling,
    ) -> Vec<VkSparseImageFormatProperties> {
        let n = self.sparse_image_format_property_count(format, itype, samples, usage, tiling);
        if n == 0 {
            // no items
            return crate::alloc::empty_sink_buffer();
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.sparse_image_format_properties(format, itype, samples, usage, tiling, &mut xs);

        xs
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
    ) where
        Self::ConcreteInstance: InstanceExtensions,
    {
        unsafe {
            self.instance().get_physical_device_external_fence_properties_khr_fn().0(
                self.native_ptr(),
                info,
                sink.as_mut_ptr(),
            );
        }
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

    /// Query a count of color formats supported by surface
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline]
    fn surface_format_count(&self, surface: &(impl VkHandle<Handle = VkSurfaceKHR> + ?Sized)) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_physical_device_surface_formats_khr(
                self.native_ptr(),
                surface.native_ptr(),
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
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline]
    fn surface_formats(
        &self,
        surface: &(impl VkHandle<Handle = VkSurfaceKHR> + ?Sized),
        sink: &mut [VkSurfaceFormatKHR],
    ) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_physical_device_surface_formats_khr(
                self.native_ptr(),
                surface.native_ptr(),
                &mut n,
                sink.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Query color formats supported by surface
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface", "alloc")]
    fn surface_formats_alloc(&self, surface: &(impl Surface + ?Sized)) -> crate::Result<Vec<VkSurfaceFormatKHR>> {
        let n = self.surface_format_count(surface)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.surface_formats(surface, &mut xs)?;

        Ok(xs)
    }

    /// Query a count of supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline]
    fn surface_present_mode_count(
        &self,
        surface: &(impl VkHandle<Handle = VkSurfaceKHR> + ?Sized),
    ) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_physical_device_surface_present_modes_khr(
                self.native_ptr(),
                surface.native_ptr(),
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
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline]
    fn surface_present_modes(
        &self,
        surface: &(impl VkHandle<Handle = VkSurfaceKHR> + ?Sized),
        sink: &mut [PresentMode],
    ) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_physical_device_surface_present_modes_khr(
                self.native_ptr(),
                surface.native_ptr(),
                &mut n,
                sink.as_mut_ptr() as _,
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Query supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface", "alloc")]
    fn surface_present_modes_alloc(&self, surface: &(impl Surface + ?Sized)) -> crate::Result<Vec<PresentMode>> {
        let n = self.surface_present_mode_count(surface)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.surface_present_modes(surface, &mut xs)?;

        Ok(xs)
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
        unsafe {
            crate::vkfn::get_physical_device_xlib_presentation_support_khr(
                self.native_ptr(),
                queue_family,
                display,
                visual,
            ) != 0
        }
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
        unsafe {
            crate::vkfn::get_physical_device_xcb_presentation_support_khr(
                self.native_ptr(),
                queue_family,
                connection,
                visual,
            ) != 0
        }
    }

    /// Query physical device for presentation to Wayland
    /// # Safety
    /// Provided `display` must be a valid reference
    #[implements("VK_KHR_wayland_surface")]
    #[inline]
    unsafe fn wayland_presentation_support(&self, queue_family: u32, display: *mut core::ffi::c_void) -> bool {
        unsafe {
            crate::vkfn::get_physical_device_wayland_presentation_support_khr(self.native_ptr(), queue_family, display)
                != 0
        }
    }

    /// Query queue family support for presentation on a Win32 display
    #[implements("VK_KHR_win32_surface")]
    #[inline]
    fn win32_presentation_support(&self, queue_family: u32) -> bool {
        unsafe { crate::vkfn::get_physical_device_win32_presentation_support_khr(self.native_ptr(), queue_family) != 0 }
    }

    /// Query a count of the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline]
    fn display_mode_property_count(&self, display: VkDisplayKHR) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_display_mode_properties_khr(self.native_ptr(), display, &mut n, core::ptr::null_mut())
                .into_result()?;
        }

        Ok(n)
    }

    /// Query the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline]
    fn display_mode_properties(
        &self,
        display: VkDisplayKHR,
        sink: &mut [VkDisplayModePropertiesKHR],
    ) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_display_mode_properties_khr(self.native_ptr(), display, &mut n, sink.as_mut_ptr())
                .into_result()?;
        }

        Ok(n)
    }

    /// Query the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display", "alloc")]
    fn display_mode_properties_alloc(&self, display: VkDisplayKHR) -> crate::Result<Vec<VkDisplayModePropertiesKHR>> {
        let n = self.display_mode_property_count(display)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.display_mode_properties(display, &mut xs)?;

        Ok(xs)
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

        unsafe {
            crate::vkfn::create_display_mode_khr(
                self.native_ptr(),
                display,
                info,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
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

    /// Query a count of information about the available displays
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline]
    fn display_property_count(&self) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_physical_device_display_properties_khr(self.native_ptr(), &mut n, core::ptr::null_mut())
                .into_result()?;
        }

        Ok(n)
    }

    /// Query information about the available displays
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline]
    fn display_properties(&self, sink: &mut [VkDisplayPropertiesKHR]) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_physical_device_display_properties_khr(self.native_ptr(), &mut n, sink.as_mut_ptr())
                .into_result()?;
        }

        Ok(n)
    }

    /// Query information about the available displays.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements("VK_KHR_display", "alloc")]
    fn display_properties_alloc(&self) -> crate::Result<Vec<DisplayPropertiesWithPhysicalDeviceRef<&Self>>> {
        let n = self.display_property_count()?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.display_properties(&mut xs)?;

        Ok(crate::alloc::collect_vec(
            xs.into_iter()
                .map(move |x| DisplayPropertiesWithPhysicalDeviceRef(x, self)),
        ))
    }

    /// Query a count of the plane properties
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline]
    fn display_plane_property_count(&self) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_physical_device_display_plane_properties_khr(
                self.native_ptr(),
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Query the plane properties
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline]
    fn display_plane_properties(&self, sink: &mut [VkDisplayPlanePropertiesKHR]) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_physical_device_display_plane_properties_khr(self.native_ptr(), &mut n, sink.as_mut_ptr())
                .into_result()?;
        }

        Ok(n)
    }

    /// Query the plane properties.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements("VK_KHR_display", "alloc")]
    fn display_plane_properties_alloc(&self) -> crate::Result<Vec<DisplayPlanePropertiesWithPhysicalDeviceRef<&Self>>> {
        let n = self.display_plane_property_count()?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.display_plane_properties(&mut xs)?;

        Ok(crate::alloc::collect_vec(
            xs.into_iter()
                .map(move |x| DisplayPlanePropertiesWithPhysicalDeviceRef(x, self)),
        ))
    }

    /// Query a count of the list of displays a plane supports
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline]
    fn display_plane_supported_display_count(&self, plane_index: u32) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_display_plane_supported_displays_khr(
                self.native_ptr(),
                plane_index,
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Query the list of displays a plane supports
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline]
    fn display_plane_supported_displays(&self, plane_index: u32, sink: &mut [VkDisplayKHR]) -> crate::Result<u32> {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_display_plane_supported_displays_khr(
                self.native_ptr(),
                plane_index,
                &mut n,
                sink.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Query the list of displays a plane supports.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements("VK_KHR_display", "alloc")]
    fn display_plane_supported_displays_alloc(&self, plane_index: u32) -> crate::Result<Vec<Display<&Self>>> {
        let n = self.display_plane_supported_display_count(plane_index)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.display_plane_supported_displays(plane_index, &mut xs)?;

        Ok(crate::alloc::collect_vec(xs.into_iter().map(move |x| Display(x, self))))
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

        unsafe {
            crate::vkfn::create_display_plane_surface_khr(
                self.instance().native_ptr(),
                info,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
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
        Self::ConcreteInstance: InstanceExtensions,
    {
        let mut d = core::mem::MaybeUninit::uninit();

        self.instance().get_randr_output_display_ext_fn().0(self.native_ptr(), dpy, rr_output, d.as_mut_ptr())
            .into_result()?;

        Ok(Display(d.assume_init(), self))
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
    ) -> crate::Result<()>
    where
        Self::ConcreteInstance: InstanceExtensions,
    {
        unsafe {
            self.instance().get_physical_device_surface_capabilities_2_khr_fn().0(
                self.native_ptr(),
                surface_info,
                sink.as_mut_ptr(),
            )
            .into_result()
            .map(drop)
        }
    }

    /// Returns properties of a physical device
    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[implements("Allow1_1APIs")]
    #[inline]
    unsafe fn properties2(&self, sink: &mut core::mem::MaybeUninit<VkPhysicalDeviceProperties2KHR>) {
        unsafe {
            crate::vkfn::get_physical_device_properties2(self.native_ptr(), sink.as_mut_ptr());
        }
    }

    /// Returns properties of a physical device
    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_physical_device_properties2")]
    #[inline]
    unsafe fn properties2(&self, sink: &mut core::mem::MaybeUninit<VkPhysicalDeviceProperties2KHR>)
    where
        Self::ConcreteInstance: InstanceExtensions,
    {
        unsafe {
            self.instance().get_physical_device_properties2_khr_fn().0(self.native_ptr(), sink.as_mut_ptr());
        }
    }

    /// Reports capabilities of a physical device
    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[implements("Allow1_1APIs")]
    #[inline]
    unsafe fn features2(&self, sink: &mut core::mem::MaybeUninit<VkPhysicalDeviceFeatures2KHR>) {
        unsafe {
            crate::vkfn::get_physical_device_features2(self.native_ptr(), sink.as_mut_ptr());
        }
    }

    /// Reports capabilities of a physical device
    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_physical_device_properties2")]
    #[inline]
    unsafe fn features2(&self, sink: &mut core::mem::MaybeUninit<VkPhysicalDeviceFeatures2KHR>)
    where
        Self::ConcreteInstance: InstanceExtensions,
    {
        unsafe {
            self.instance().get_physical_device_features2_khr_fn().0(self.native_ptr(), sink.as_mut_ptr());
        }
    }

    /// Query a count of supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[implements("VK_EXT_full_screen_exclusive")]
    #[inline]
    fn surface_present_mode2_count(&self, surface_info: &VkPhysicalDeviceSurfaceInfo2KHR) -> crate::Result<u32>
    where
        Self::ConcreteInstance: InstanceExtensions,
    {
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

        Ok(n)
    }

    /// Query supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[implements("VK_EXT_full_screen_exclusive")]
    #[inline]
    fn surface_present_modes2(
        &self,
        surface_info: &VkPhysicalDeviceSurfaceInfo2KHR,
        sink: &mut [VkPresentModeKHR],
    ) -> crate::Result<u32>
    where
        Self::ConcreteInstance: InstanceExtensions,
    {
        let mut n = sink.len() as _;
        unsafe {
            self.instance().get_physical_device_surface_present_modes_2_ext_fn().0(
                self.native_ptr(),
                surface_info,
                &mut n,
                sink.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Query supported presentation modes.
    ///
    /// # Failures
    ///
    /// On failure, this command returns
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[implements("VK_EXT_full_screen_exclusive", "alloc")]
    fn surface_present_modes2_alloc(
        &self,
        surface_info: &VkPhysicalDeviceSurfaceInfo2KHR,
    ) -> crate::Result<Vec<VkPresentModeKHR>>
    where
        Self::ConcreteInstance: InstanceExtensions,
    {
        let n = self.surface_present_mode2_count(surface_info)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.surface_present_modes2(surface_info, &mut xs)?;

        Ok(xs)
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

pub type PhysicalDeviceFeatures = VkPhysicalDeviceFeatures;
pub type PhysicalDeviceProperties = VkPhysicalDeviceProperties;
pub type PhysicalDeviceMemoryProperties = VkPhysicalDeviceMemoryProperties;
impl PhysicalDeviceMemoryProperties {
    #[inline(always)]
    pub fn types(&self) -> &[MemoryType] {
        &self.memoryTypes[..self.memoryTypeCount as _]
    }

    #[inline(always)]
    pub fn heaps(&self) -> &[MemoryHeap] {
        &self.memoryHeaps[..self.memoryHeapCount as _]
    }
}

pub type MemoryType = VkMemoryType;
impl MemoryType {
    pub const fn property_flags(&self) -> MemoryPropertyFlags {
        MemoryPropertyFlags(self.propertyFlags)
    }
}

pub type MemoryHeap = VkMemoryHeap;
impl MemoryHeap {
    pub const fn flags(&self) -> MemoryHeapFlags {
        MemoryHeapFlags(self.flags)
    }
}

/// Device memory properties
#[repr(transparent)]
pub struct MemoryProperties(VkPhysicalDeviceMemoryProperties);
impl MemoryProperties {
    #[inline(always)]
    pub fn types(&self) -> &[MemoryType] {
        &self.0.memoryTypes[..self.0.memoryTypeCount as _]
    }
    #[inline(always)]
    pub fn heaps(&self) -> &[MemoryHeap] {
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
            (index_mask & (1u32 << i) != 0 && mt.property_flags().has(mask) && !mt.property_flags().has(exclude))
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
            MemoryPropertyFlags::DEVICE_LOCAL | MemoryPropertyFlags::LAZILY_ALLOCATED,
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
        self.0.memoryTypes[index as usize]
            .property_flags()
            .has(MemoryPropertyFlags::HOST_COHERENT)
    }
    pub fn is_cached(&self, index: u32) -> bool {
        self.0.memoryTypes[index as usize]
            .property_flags()
            .has(MemoryPropertyFlags::HOST_CACHED)
    }
}

/// Bitmask specifying properties for a memory type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct MemoryPropertyFlags(VkMemoryPropertyFlags);
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

    #[cfg(feature = "Allow1_1APIs")]
    /// The memory type only allows device access to the memory, and allows protected queue operations to access the memory.
    pub const PROTECTED: Self = Self(VK_MEMORY_PROPERTY_PROTECTED_BIT);
}

/// Bitmask specifying attribute flags for a heap
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct MemoryHeapFlags(VkMemoryHeapFlags);
impl MemoryHeapFlags {
    /// Empty set
    pub const EMPTY: Self = Self(0);

    /// The heap corresponds to device-local memory.
    /// Device-local memory may have different performance characteristics than host-local memory,
    /// and may support different memory property flags.
    pub const DEVICE_LOCAL: Self = Self(VK_MEMORY_HEAP_DEVICE_LOCAL_BIT);

    #[cfg(feature = "Allow1_1APIs")]
    /// In a logical device representing more than one physical device,
    /// there is a per-physical device instance of the heap memory.
    /// By default, an allocation from such a heap will be replicated to each physical device's instance of the heap.
    pub const MULTI_INSTANCE: Self = Self(VK_MEMORY_HEAP_MULTI_INSTANCE_BIT);
}

/// List of queue families
pub struct QueueFamilies(pub Vec<QueueFamilyProperties>);
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
            .find_map(|(n, &QueueFamilyProperties { queueFlags, .. })| {
                ((queueFlags & flags.0) != 0 && exclude != n as u32).then_some(n as _)
            })
    }

    /// Number of queue families
    pub fn count(&self) -> u32 {
        self.0.len() as _
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &QueueFamilyProperties> {
        self.0.iter()
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
    pub fn minimum_image_transfer_granularity(&self, family_index: u32) -> &Extent3D {
        &self.0[family_index as usize].minImageTransferGranularity
    }
}

pub type QueueFamilyProperties = VkQueueFamilyProperties;
impl QueueFamilyProperties {
    pub const fn queue_flags(&self) -> QueueFlags {
        QueueFlags(self.queueFlags)
    }
}

/// Set of bit of queue flags
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[bitflags_newtype]
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
}

#[cfg(feature = "VK_KHR_display")]
mod display;
#[cfg(feature = "VK_KHR_display")]
pub use self::display::*;
