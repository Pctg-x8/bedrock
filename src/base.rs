//! Vulkan Base Objects(Instance/PhysicalDevice)

use crate::error::translate_vk_result;
use crate::ffi_helper::{CStrFFIRef, slice_as_ptr_empty_null};
use crate::*;
use bedrock_vk::{self as brvk, TypedVulkanStructure};
use core::ffi::CStr;
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
    pub const V1: Self = Self::new(1, 0, 0);

    /// Construct an object from discrete values
    #[inline(always)]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self::with_variant(0, major, minor, patch)
    }

    /// Construct an object from discrete values
    pub const fn with_variant(variant: u8, major: u16, minor: u16, patch: u16) -> Self {
        Self(brvk::VK_MAKE_VERSION(variant, major, minor, patch))
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
        brvk::VK_VARIANT_VERSION(self.0)
    }

    /// Major version number
    pub const fn major(&self) -> u16 {
        brvk::VK_MAJOR_VERSION(self.0)
    }

    /// Minor version number
    pub const fn minor(&self) -> u16 {
        brvk::VK_MINOR_VERSION(self.0)
    }

    /// Patch version number
    pub const fn patch(&self) -> u16 {
        brvk::VK_PATCH_VERSION(self.0)
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
/// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
#[inline]
#[implements]
pub fn instance_version() -> crate::Result<Version> {
    #[cfg(feature = "Allow1_1APIs")]
    unsafe {
        use crate::error::translate_vk_result;

        let mut sink = 0;
        translate_vk_result(brvk::fns::enumerate_instance_version(&mut sink))?;
        Ok(Version(sink))
    }
    #[cfg(not(feature = "Allow1_1APIs"))]
    {
        // fixed to v1.0.0
        Ok(Version::V1)
    }
}

/// Returns up to all of global layer properties
/// # Failures
/// On failure, this command returns
///
/// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
#[implements("alloc")]
pub fn instance_layer_properties_alloc() -> crate::Result<Vec<brvk::VkLayerProperties>> {
    let n = crate::vkfn_wrapper::instance_layer_property_count()?;
    if n == 0 {
        // no items
        return Ok(crate::alloc::empty_sink_buffer());
    }

    let mut xs = crate::alloc::reserve(n as _);
    let r = crate::vkfn_wrapper::instance_layer_properties(xs.spare_capacity_mut())?.assert_complete();
    unsafe {
        xs.set_len(r as _);
    }

    Ok(xs)
}

/// Returns up to all of global extension properties
/// # Failures
/// On failure, this command returns
///
/// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// * `brvk::VK_ERROR_LAYER_NOT_PRESENT`
#[implements("alloc")]
pub fn instance_extension_properties_alloc(
    layer_name: Option<&CStr>,
) -> crate::Result<Vec<brvk::VkExtensionProperties>> {
    let n = crate::vkfn_wrapper::instance_extension_property_count(layer_name)? as usize;
    if n == 0 {
        // no items
        return Ok(crate::alloc::empty_sink_buffer());
    }

    let mut xs = crate::alloc::reserve(n);
    let r = crate::vkfn_wrapper::instance_extension_properties(layer_name, xs.spare_capacity_mut())?.assert_complete();
    unsafe {
        xs.set_len(r as _);
    }

    Ok(xs)
}

/// Returns up to all of global extension properties
/// # Failures
/// On failure, this command returns
///
/// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
/// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// * `brvk::VK_ERROR_LAYER_NOT_PRESENT`
#[implements("alloc")]
pub fn instance_extension_properties_str_alloc(
    layer_name: Option<&str>,
) -> crate::Result<Vec<brvk::VkExtensionProperties>> {
    instance_extension_properties_alloc(layer_name.map(|s| crate::alloc::str_to_cstr(s).unwrap()).as_deref())
}

#[implements]
type InstanceResolvedFn<F> = brvk::ResolvedFnCell<F, InstanceResolverImpl>;
#[implements]
#[repr(transparent)]
pub struct InstanceResolverImpl(pub brvk::VkInstance);
#[implements]
impl brvk::ResolverInterface for InstanceResolverImpl {
    #[tracing::instrument(
        name = "<brvk::VkInstance as ResolverInterface>::load_symbol_unconstrainted",
        skip(self)
    )]
    unsafe fn load_symbol_unconstrainted(&self, name: &core::ffi::CStr) -> core::ptr::NonNull<core::ffi::c_void> {
        match unsafe { brvk::fns::get_instance_proc_addr(self.0, name.as_ptr().cast()) } {
            Some(x) => unsafe { core::ptr::NonNull::new_unchecked(x as _) },
            None => {
                tracing::error!("instance function not found, bedrock could not continue");
                std::process::abort();
            }
        }
    }

    #[tracing::instrument(
        name = "<brvk::VkInstance as ResolverInterface>::load_function_unconstrainted",
        skip(self)
    )]
    unsafe fn load_function_unconstrainted(&self, name: &core::ffi::CStr) -> brvk::PFN_vkVoidFunction {
        match unsafe { brvk::fns::get_instance_proc_addr(self.0, name.as_ptr().cast()) } {
            Some(x) => x,
            None => {
                tracing::error!("instance function not found, bedrock could not continue");
                std::process::abort();
            }
        }
    }
}

#[repr(transparent)]
pub struct ApplicationInfo<'d>(brvk::VkApplicationInfo, core::marker::PhantomData<&'d CStr>);
impl<'d> ApplicationInfo<'d> {
    #[inline(always)]
    pub const fn new(app_name: &'d CStr, app_version: Version, engine_name: &'d CStr, engine_version: Version) -> Self {
        Self(
            brvk::VkApplicationInfo {
                sType: brvk::VkApplicationInfo::TYPE,
                pNext: core::ptr::null(),
                apiVersion: brvk::VK_API_VERSION_1_0,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct InstanceCreateFlags(brvk::VkInstanceCreateFlags);
impl InstanceCreateFlags {
    /// Empty bits.
    pub const EMPTY: Self = Self(0);

    #[cfg(feature = "VK_KHR_portability_enumeration")]
    /// The instance will enumerate available Vulkan Portability-compliant physical devices and groups
    /// in addition to the Vulkan physical devices and groups that are enumerated by default.
    pub const ENUMERATE_PORTABILITY: Self = Self(brvk::VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR);
}

#[repr(transparent)]
#[derive(Clone)]
pub struct InstanceCreateInfo<'d>(
    brvk::VkInstanceCreateInfo,
    core::marker::PhantomData<(
        Option<&'d dyn brvk::VulkanStructure>,
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
            brvk::VkInstanceCreateInfo {
                sType: brvk::VkInstanceCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                pApplicationInfo: core::ptr::from_ref(application_info).cast(),
                enabledLayerCount: layers.len() as _,
                ppEnabledLayerNames: slice_as_ptr_empty_null(layers) as _,
                enabledExtensionCount: extensions.len() as _,
                ppEnabledExtensionNames: slice_as_ptr_empty_null(extensions) as _,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// raw must be a valid brvk::VkInstanceCreateInfo struct.
    pub const unsafe fn from_raw(raw: brvk::VkInstanceCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkInstanceCreateInfo {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }

    pub const fn flags(mut self, flags: InstanceCreateFlags) -> Self {
        self.0.flags = flags.bits();
        self
    }
}

/// Opaque handle to a instance object
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VK_OBJECT_TYPE_INSTANCE)]
pub struct InstanceObject {
    #[handle]
    handle: brvk::VkInstance,
    #[cfg(feature = "Implements")]
    ext: InstanceExtFunctions,
}
unsafe impl Sync for InstanceObject {}
unsafe impl Send for InstanceObject {}
#[implements]
impl Drop for InstanceObject {
    #[inline(always)]
    fn drop(&mut self) {
        crate::vkfn_wrapper::destroy_instance(self.as_transparent_ref_mut(), None);
    }
}
impl Instance for InstanceObject {}
impl InstanceObject {
    /// Create a new Vulkan instance
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_INITIALIZATION_FAILED`
    /// * `brvk::VK_ERROR_LAYER_NOT_PRESENT`
    /// * `brvk::VK_ERROR_EXTENSION_NOT_PRESENT`
    /// * `brvk::VK_ERROR_INCOMPATIBLE_DRIVER`
    #[implements]
    #[inline]
    pub fn new(info: &InstanceCreateInfo) -> crate::Result<Self> {
        Ok(unsafe { Self::manage(crate::vkfn_wrapper::create_instance(info, None)?) })
    }

    /// Constructs from raw handle
    /// # Safety
    /// the handle must be valid and not freed
    pub const unsafe fn manage(handle: brvk::VkInstance) -> Self {
        Self {
            handle,
            #[cfg(feature = "Implements")]
            ext: InstanceExtFunctions::new(handle),
        }
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(self) -> brvk::VkInstance {
        let v = self.handle;
        core::mem::forget(self);

        v
    }
}

/// A Vulkan Instance interface
pub trait Instance: VkHandle<Handle = brvk::VkInstance> {
    /// Return a function pointer for a command
    #[deprecated = "do not use this directly(this does not provide caching!)"]
    #[implements]
    #[inline]
    fn extra_procedure(&self, name: &CStr) -> Option<brvk::PFN_vkVoidFunction> {
        unsafe { brvk::fns::get_instance_proc_addr(self.native_ptr(), name.as_ptr()) }
    }

    /// Counts the physical devices accessible to a Vulkan instance
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_INITIALIZATION_FAILED`]
    #[implements]
    #[inline(always)]
    fn physical_device_count(&self) -> crate::Result<u32> {
        crate::vkfn_wrapper::physical_device_count(self.as_transparent_ref())
    }

    /// Enumerates the physical devices accessible to a Vulkan instance
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_INITIALIZATION_FAILED`]
    #[implements]
    #[inline(always)]
    fn enumerate_physical_devices(
        &self,
        sink: &mut [core::mem::MaybeUninit<brvk::VkPhysicalDevice>],
    ) -> crate::Result<ArrayQueryResult<u32>> {
        crate::vkfn_wrapper::enumerate_physical_devices(self.as_transparent_ref(), sink)
    }

    /// Lazyly enumerates the physical devices accessible to a Vulkan instance
    ///
    /// # Failures
    ///
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_INITIALIZATION_FAILED`
    #[implements("alloc")]
    fn iter_physical_devices<'a>(&'a self) -> crate::Result<IterPhysicalDevices<'a, Self>> {
        let n = self.physical_device_count()? as usize;
        if n == 0 {
            // no items
            return Ok(IterPhysicalDevices(crate::alloc::empty_sink_buffer(), 0, self));
        }

        let mut xs = Vec::with_capacity(n);
        let r = self
            .enumerate_physical_devices(xs.spare_capacity_mut())?
            .assert_complete();
        unsafe {
            xs.set_len(r as _);
        }

        Ok(IterPhysicalDevices(xs, 0, self))
    }

    /// Enumerates the physical devices accessible to a Vulkan instance
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_INITIALIZATION_FAILED`
    #[implements("alloc")]
    fn enumerate_physical_devices_alloc(&self) -> crate::Result<Vec<PhysicalDeviceObject<&Self>>> {
        self.iter_physical_devices().map(crate::alloc::collect_vec)
    }
}
DerefContainerWithGuardsBracketImpl!(for Instance {});

/// Extension function caches
#[implements]
struct InstanceExtFunctions {
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    get_physical_device_properties2_khr: InstanceResolvedFn<brvk::PFN_vkGetPhysicalDeviceProperties2KHR>,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    get_physical_device_features2_khr: InstanceResolvedFn<brvk::PFN_vkGetPhysicalDeviceFeatures2KHR>,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    get_physical_device_format_properties2_khr: InstanceResolvedFn<brvk::PFN_vkGetPhysicalDeviceFormatProperties2KHR>,
    #[cfg(feature = "VK_EXT_debug_report")]
    create_debug_report_callback_ext: InstanceResolvedFn<brvk::PFN_vkCreateDebugReportCallbackEXT>,
    #[cfg(feature = "VK_EXT_debug_report")]
    destroy_debug_report_callback_ext: InstanceResolvedFn<brvk::PFN_vkDestroyDebugReportCallbackEXT>,
    #[cfg(feature = "VK_EXT_debug_report")]
    debug_report_message_ext: InstanceResolvedFn<brvk::PFN_vkDebugReportMessageEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    create_debug_utils_messenger_ext: InstanceResolvedFn<brvk::PFN_vkCreateDebugUtilsMessengerEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    destroy_debug_utils_messenger_ext: InstanceResolvedFn<brvk::PFN_vkDestroyDebugUtilsMessengerEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    set_debug_utils_object_name_ext: InstanceResolvedFn<brvk::PFN_vkSetDebugUtilsObjectNameEXT>,
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    get_physical_device_external_fence_properties_khr:
        InstanceResolvedFn<brvk::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR>,
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    get_randr_output_display_ext: InstanceResolvedFn<brvk::PFN_vkGetRandROutputDisplayEXT>,
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    acquire_xlib_display_ext: InstanceResolvedFn<brvk::PFN_vkAcquireXlibDisplayEXT>,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    get_physical_device_surface_present_modes_2_ext:
        InstanceResolvedFn<brvk::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT>,
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    get_physical_device_surface_capabilities_2_khr:
        InstanceResolvedFn<brvk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR>,
    #[cfg(feature = "VK_EXT_direct_mode_display")]
    release_display_ext: InstanceResolvedFn<brvk::PFN_vkReleaseDisplayEXT>,
    #[cfg(feature = "VK_EXT_sample_locations")]
    get_physical_device_multisample_properties_ext:
        InstanceResolvedFn<brvk::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT>,
}
#[implements]
impl InstanceExtFunctions {
    const fn new(r: brvk::VkInstance) -> Self {
        Self {
            #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
            get_physical_device_properties2_khr: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
            get_physical_device_features2_khr: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
            get_physical_device_format_properties2_khr: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_debug_report")]
            create_debug_report_callback_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_debug_report")]
            destroy_debug_report_callback_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_debug_report")]
            debug_report_message_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_debug_utils")]
            create_debug_utils_messenger_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_debug_utils")]
            destroy_debug_utils_messenger_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_debug_utils")]
            set_debug_utils_object_name_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_KHR_external_fence_capabilities")]
            get_physical_device_external_fence_properties_khr: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_acquire_xlib_display")]
            get_randr_output_display_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_acquire_xlib_display")]
            acquire_xlib_display_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            get_physical_device_surface_present_modes_2_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
            get_physical_device_surface_capabilities_2_khr: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_direct_mode_display")]
            release_display_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
            #[cfg(feature = "VK_EXT_sample_locations")]
            get_physical_device_multisample_properties_ext: InstanceResolvedFn::new(InstanceResolverImpl(r)),
        }
    }
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub trait InstanceGetPhysicalDeviceProperties2Extension: Instance {
    #[implements]
    fn get_physical_device_properties2_khr_fn(&self) -> brvk::PFN_vkGetPhysicalDeviceProperties2KHR;
    #[implements]
    fn get_physical_device_features2_khr_fn(&self) -> brvk::PFN_vkGetPhysicalDeviceFeatures2KHR;
    #[implements]
    fn get_physical_device_format_properties2_khr_fn(&self) -> brvk::PFN_vkGetPhysicalDeviceFormatProperties2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
DerefContainerWithGuardsBracketImpl!(for InstanceGetPhysicalDeviceProperties2Extension {
    #[implements]
    ForwardFnPtr!(deref get_physical_device_properties2_khr_fn -> brvk::PFN_vkGetPhysicalDeviceProperties2KHR);
    #[implements]
    ForwardFnPtr!(deref get_physical_device_features2_khr_fn -> brvk::PFN_vkGetPhysicalDeviceFeatures2KHR);
    #[implements]
    ForwardFnPtr!(deref get_physical_device_format_properties2_khr_fn -> brvk::PFN_vkGetPhysicalDeviceFormatProperties2KHR);
});
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl InstanceGetPhysicalDeviceProperties2Extension for InstanceObject {
    #[implements]
    #[inline(always)]
    fn get_physical_device_properties2_khr_fn(&self) -> brvk::PFN_vkGetPhysicalDeviceProperties2KHR {
        *self.ext.get_physical_device_properties2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn get_physical_device_features2_khr_fn(&self) -> brvk::PFN_vkGetPhysicalDeviceFeatures2KHR {
        *self.ext.get_physical_device_features2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn get_physical_device_format_properties2_khr_fn(&self) -> brvk::PFN_vkGetPhysicalDeviceFormatProperties2KHR {
        *self.ext.get_physical_device_format_properties2_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_debug_report")]
pub trait InstanceDebugReportExtension: Instance {
    #[implements]
    fn create_debug_report_callback_ext_fn(&self) -> brvk::PFN_vkCreateDebugReportCallbackEXT;
    #[implements]
    fn destroy_debug_report_callback_ext_fn(&self) -> brvk::PFN_vkDestroyDebugReportCallbackEXT;
    #[implements]
    fn debug_report_message_ext_fn(&self) -> brvk::PFN_vkDebugReportMessageEXT;

    /// Register a debug report callback
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    ///
    /// # Safety
    /// must not execute in parallel with any Vulkan commands
    #[implements]
    #[inline]
    unsafe fn new_debug_report_callback_raw(
        &self,
        info: &crate::DebugReportCallbackCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkDebugReportCallbackEXT> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            self.create_debug_report_callback_ext_fn().0(
                self.native_ptr(),
                info.as_raw_ref(),
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Inject its own messages into the debug stream
    #[implements]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn debug_message(
        &self,
        flags: brvk::VkDebugReportFlagsEXT,
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
    #[implements]
    #[inline]
    unsafe fn destroy_debug_report_callback_raw(
        &self,
        obj: brvk::VkDebugReportCallbackEXT,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) {
        unsafe {
            self.destroy_debug_report_callback_ext_fn().0(
                self.native_ptr(),
                obj,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
            );
        }
    }
}
#[cfg(feature = "VK_EXT_debug_report")]
DerefContainerWithGuardsBracketImpl!(for InstanceDebugReportExtension {
    #[implements]
    ForwardFnPtr!(deref create_debug_report_callback_ext_fn -> brvk::PFN_vkCreateDebugReportCallbackEXT);
    #[implements]
    ForwardFnPtr!(deref destroy_debug_report_callback_ext_fn -> brvk::PFN_vkDestroyDebugReportCallbackEXT);
    #[implements]
    ForwardFnPtr!(deref debug_report_message_ext_fn -> brvk::PFN_vkDebugReportMessageEXT);
});
#[cfg(feature = "VK_EXT_debug_report")]
impl InstanceDebugReportExtension for InstanceObject {
    #[implements]
    #[inline(always)]
    fn create_debug_report_callback_ext_fn(&self) -> brvk::PFN_vkCreateDebugReportCallbackEXT {
        *self.ext.create_debug_report_callback_ext.resolve()
    }
    #[implements]
    #[inline(always)]
    fn destroy_debug_report_callback_ext_fn(&self) -> brvk::PFN_vkDestroyDebugReportCallbackEXT {
        *self.ext.destroy_debug_report_callback_ext.resolve()
    }
    #[implements]
    #[inline(always)]
    fn debug_report_message_ext_fn(&self) -> brvk::PFN_vkDebugReportMessageEXT {
        *self.ext.debug_report_message_ext.resolve()
    }
}

#[cfg(feature = "VK_EXT_debug_utils")]
pub trait InstanceDebugUtilsExtension: Instance {
    #[implements]
    fn create_debug_utils_messenger_ext_fn(&self) -> brvk::PFN_vkCreateDebugUtilsMessengerEXT;
    #[implements]
    fn destroy_debug_utils_messenger_ext_fn(&self) -> brvk::PFN_vkDestroyDebugUtilsMessengerEXT;
    #[implements]
    fn set_debug_utils_object_name_ext_fn(&self) -> brvk::PFN_vkSetDebugUtilsObjectNameEXT;

    /// Create a debug messenger object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    ///
    /// # Safety
    /// must not be executed in parallel with any Vulkan commands
    #[implements]
    #[inline]
    unsafe fn new_debug_utils_messenger_raw(
        &self,
        info: &crate::DebugUtilsMessengerCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkDebugUtilsMessengerEXT> {
        let mut h = core::mem::MaybeUninit::uninit();
        crate::error::translate_vk_result(unsafe {
            self.create_debug_utils_messenger_ext_fn().0(
                self.native_ptr(),
                core::ptr::from_ref(info).cast(),
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Destroy a debug messenger object
    /// # Safety
    /// * must be created from this Instance object
    /// * must not execute in parallel with any Vulkan commands
    #[implements]
    #[inline]
    unsafe fn destroy_debug_utils_messenger_raw(
        &self,
        obj: brvk::VkDebugUtilsMessengerEXT,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) {
        unsafe {
            self.destroy_debug_utils_messenger_ext_fn().0(
                self.native_ptr(),
                obj,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
            );
        }
    }

    /// Give an application-defined name to an object
    ///
    /// # Failure
    ///
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_UNKNOWN`]
    /// * [`brvk::VK_ERROR_VALIDATION_FAILED`]
    ///
    /// # Safety
    ///
    /// * `objectHandle` in `info` must be associated with the `device`
    /// * `objectHandle` in `info` must be externally synchronizewd
    #[implements]
    #[inline]
    unsafe fn set_debug_utils_object_name_ext(
        &self,
        device: VkHandleRef<brvk::VkDevice>,
        info: &DebugUtilsObjectNameInfo,
    ) -> crate::Result<()> {
        crate::error::translate_vk_result(unsafe { self.set_debug_utils_object_name_ext_fn().0(device.0, &info.0) })?;

        Ok(())
    }
}
#[cfg(feature = "VK_EXT_debug_utils")]
DerefContainerWithGuardsBracketImpl!(for InstanceDebugUtilsExtension {
    #[implements]
    ForwardFnPtr!(deref create_debug_utils_messenger_ext_fn -> brvk::PFN_vkCreateDebugUtilsMessengerEXT);
    #[implements]
    ForwardFnPtr!(deref destroy_debug_utils_messenger_ext_fn -> brvk::PFN_vkDestroyDebugUtilsMessengerEXT);
    #[implements]
    ForwardFnPtr!(deref set_debug_utils_object_name_ext_fn -> brvk::PFN_vkSetDebugUtilsObjectNameEXT);
});
#[cfg(feature = "VK_EXT_debug_utils")]
impl InstanceDebugUtilsExtension for InstanceObject {
    #[implements]
    #[inline(always)]
    fn create_debug_utils_messenger_ext_fn(&self) -> brvk::PFN_vkCreateDebugUtilsMessengerEXT {
        *self.ext.create_debug_utils_messenger_ext.resolve()
    }
    #[implements]
    #[inline(always)]
    fn destroy_debug_utils_messenger_ext_fn(&self) -> brvk::PFN_vkDestroyDebugUtilsMessengerEXT {
        *self.ext.destroy_debug_utils_messenger_ext.resolve()
    }
    #[implements]
    #[inline(always)]
    fn set_debug_utils_object_name_ext_fn(&self) -> brvk::PFN_vkSetDebugUtilsObjectNameEXT {
        *self.ext.set_debug_utils_object_name_ext.resolve()
    }
}

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub trait InstanceExternalFenceCapabilitiesExtension: Instance {
    #[implements]
    fn get_physical_device_external_fence_properties_khr_fn(
        &self,
    ) -> brvk::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR;
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
DerefContainerWithGuardsBracketImpl!(for InstanceExternalFenceCapabilitiesExtension {
    #[implements]
    ForwardFnPtr!(deref get_physical_device_external_fence_properties_khr_fn -> brvk::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR);
});
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl InstanceExternalFenceCapabilitiesExtension for InstanceObject {
    #[implements]
    #[inline(always)]
    fn get_physical_device_external_fence_properties_khr_fn(
        &self,
    ) -> brvk::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR {
        *self.ext.get_physical_device_external_fence_properties_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_acquire_xlib_display")]
pub trait InstanceAcquireXlibDisplayExtension: Instance {
    #[implements]
    fn get_randr_output_display_ext_fn(&self) -> brvk::PFN_vkGetRandROutputDisplayEXT;
    #[implements]
    fn acquire_xlib_display_ext_fn(&self) -> brvk::PFN_vkAcquireXlibDisplayEXT;
}
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
DerefContainerWithGuardsBracketImpl!(for InstanceAcquireXlibDisplayExtension {
    #[implements]
    ForwardFnPtr!(deref get_randr_output_display_ext_fn -> brvk::PFN_vkGetRandROutputDisplayEXT);
    #[implements]
    ForwardFnPtr!(deref acquire_xlib_display_ext_fn -> brvk::PFN_vkAcquireXlibDisplayEXT);
});
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
impl InstanceAcquireXlibDisplayExtension for InstanceObject {
    #[implements]
    #[inline(always)]
    fn get_randr_output_display_ext_fn(&self) -> brvk::PFN_vkGetRandROutputDisplayEXT {
        *self.ext.get_randr_output_display_ext.resolve()
    }
    #[implements]
    #[inline(always)]
    fn acquire_xlib_display_ext_fn(&self) -> brvk::PFN_vkAcquireXlibDisplayEXT {
        *self.ext.acquire_xlib_display_ext.resolve()
    }
}

#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub trait InstanceFullScreenExclusiveExtension: Instance {
    #[implements]
    fn get_physical_device_surface_present_modes_2_ext_fn(
        &self,
    ) -> brvk::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT;

    /// # Safety
    ///
    /// surface in `surface_info` and `physical_device` must be created from the same instance.
    #[implements]
    #[inline]
    unsafe fn get_physical_device_surface_present_mode2_count(
        &self,
        physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
        surface_info: &brvk::VkPhysicalDeviceSurfaceInfo2KHR,
    ) -> crate::Result<u32> {
        let mut n = 0;
        translate_vk_result(unsafe {
            self.get_physical_device_surface_present_modes_2_ext_fn().0(
                physical_device.0,
                surface_info,
                &mut n,
                core::ptr::null_mut(),
            )
        })?;

        Ok(n)
    }

    /// # Safety
    ///
    /// surface in `surface_info` and `physical_device` must be created from the same instance.
    #[implements]
    #[inline]
    unsafe fn get_physical_device_surface_present_modes2(
        &self,
        physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
        surface_info: &brvk::VkPhysicalDeviceSurfaceInfo2KHR,
        sink: &mut [core::mem::MaybeUninit<brvk::VkPresentModeKHR>],
    ) -> crate::Result<ArrayQueryResult<u32>> {
        let mut n = sink.len() as _;
        let r = ArrayQueryResult::from_vk_result(unsafe {
            self.get_physical_device_surface_present_modes_2_ext_fn().0(
                physical_device.0,
                surface_info,
                &mut n,
                sink.as_mut_ptr().cast(),
            )
        })?;

        Ok(r.with_result(n))
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
DerefContainerWithGuardsBracketImpl!(for InstanceFullScreenExclusiveExtension {
    #[implements]
    ForwardFnPtr!(deref get_physical_device_surface_present_modes_2_ext_fn -> brvk::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT);
});
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl InstanceFullScreenExclusiveExtension for InstanceObject {
    #[implements]
    #[inline(always)]
    fn get_physical_device_surface_present_modes_2_ext_fn(
        &self,
    ) -> brvk::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT {
        *self.ext.get_physical_device_surface_present_modes_2_ext.resolve()
    }
}

#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub trait InstanceGetSurfaceCapabilities2Extension: Instance {
    #[implements]
    fn get_physical_device_surface_capabilities_2_khr_fn(&self)
    -> brvk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR;

    /// # Safety
    ///
    /// surface in `surface_info` and `physical_device` must be created from the same instance.
    #[implements]
    #[inline(always)]
    unsafe fn get_physical_device_surface_capabilities2(
        &self,
        physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
        surface_info: &brvk::VkPhysicalDeviceSurfaceInfo2KHR,
        sink: &mut core::mem::MaybeUninit<brvk::VkSurfaceCapabilities2KHR>,
    ) -> crate::Result<()> {
        translate_vk_result(unsafe {
            self.get_physical_device_surface_capabilities_2_khr_fn().0(
                physical_device.0,
                surface_info,
                sink.as_mut_ptr(),
            )
        })?;

        Ok(())
    }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
DerefContainerWithGuardsBracketImpl!(for InstanceGetSurfaceCapabilities2Extension {
    #[implements]
    ForwardFnPtr!(deref get_physical_device_surface_capabilities_2_khr_fn -> brvk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR);
});
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl InstanceGetSurfaceCapabilities2Extension for InstanceObject {
    #[implements]
    #[inline(always)]
    fn get_physical_device_surface_capabilities_2_khr_fn(
        &self,
    ) -> brvk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR {
        *self.ext.get_physical_device_surface_capabilities_2_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_direct_mode_display")]
pub trait InstanceDirectModeDisplayExtension: Instance {
    #[implements]
    fn release_display_ext_fn(&self) -> brvk::PFN_vkReleaseDisplayEXT;
}
#[cfg(feature = "VK_EXT_direct_mode_display")]
DerefContainerWithGuardsBracketImpl!(for InstanceDirectModeDisplayExtension {
    #[implements]
    ForwardFnPtr!(deref release_display_ext_fn -> brvk::PFN_vkReleaseDisplayEXT);
});
#[cfg(feature = "VK_EXT_direct_mode_display")]
impl InstanceDirectModeDisplayExtension for InstanceObject {
    #[implements]
    #[inline(always)]
    fn release_display_ext_fn(&self) -> brvk::PFN_vkReleaseDisplayEXT {
        *self.ext.release_display_ext.resolve()
    }
}

#[cfg(feature = "VK_EXT_sample_locations")]
pub trait InstanceSampleLocationsExtension: Instance {
    #[implements]
    fn get_physical_device_multisample_properties_ext_fn(
        &self,
    ) -> brvk::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
DerefContainerWithGuardsBracketImpl!(for InstanceSampleLocationsExtension {
    #[implements]
    ForwardFnPtr!(deref get_physical_device_multisample_properties_ext_fn -> brvk::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT);
});
#[cfg(feature = "VK_EXT_sample_locations")]
impl InstanceSampleLocationsExtension for InstanceObject {
    #[implements]
    #[inline(always)]
    fn get_physical_device_multisample_properties_ext_fn(
        &self,
    ) -> brvk::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT {
        *self.ext.get_physical_device_multisample_properties_ext.resolve()
    }
}

/// Opaque handle to a physical device object
///
/// ## Platform Dependent Methods: Presentation Support checking functions
///
/// * `xlib_presentation_support(&self, queue_family: u32, display: *mut x11::xlib::Display, visual: x11::xlib::VisualID) -> bool`: brvk::VK_KHR_xlib_surface
/// * `xcb_presentation_support(&self, queue_family: u32, connection: *mut xcb::ffi::xcb_connection_t, visual: xcb::ffi::xcb_visualid_t) -> bool`: brvk::VK_KHR_xcb_surface
/// * `wayland_presentation_support(&self, queue_family: u32, display: *mut wayland_client::sys::wl_display) -> bool`: brvk::VK_KHR_wayland_surface
/// * `win32_presentation_support(&self, queue_family: u32) -> bool`: brvk::VK_KHR_win32_surface
/// * Methods for Android and Mir surfaces are not implemented
#[derive(VkHandle, VkObject, crate::InstanceChild, crate::InstanceChildTransferrable, Clone)]
#[VkObject(type = brvk::VK_OBJECT_TYPE_PHYSICAL_DEVICE)]
pub struct PhysicalDeviceObject<Owner: Instance>(brvk::VkPhysicalDevice, #[parent] Owner);
unsafe impl<Owner: Instance + Sync> Sync for PhysicalDeviceObject<Owner> {}
unsafe impl<Owner: Instance + Send> Send for PhysicalDeviceObject<Owner> {}
impl<Owner: Instance> PhysicalDevice for PhysicalDeviceObject<Owner> {}
impl<Owner: Instance> PhysicalDeviceObject<Owner> {
    /// # Safety
    ///
    /// passed handles must be valid, and owned by the same instance as `owner`.
    pub const unsafe fn manage(handle: brvk::VkPhysicalDevice, owner: Owner) -> Self {
        Self(handle, owner)
    }

    pub const fn unmanage(self) -> (brvk::VkPhysicalDevice, Owner) {
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

pub struct IterPhysicalDevices<'i, Source: Instance + 'i + ?Sized>(Vec<brvk::VkPhysicalDevice>, usize, &'i Source);
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

/// A PhysicalDevice interface
pub trait PhysicalDevice: VkHandle<Handle = brvk::VkPhysicalDevice> + InstanceChild {
    /// Returns a count of properties of available physical device layers
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn layer_property_count(&self) -> crate::Result<u32> {
        crate::vkfn_wrapper::device_layer_property_count(self.as_transparent_ref())
    }

    /// Returns properties of available physical device layers
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn enumerate_layer_properties(
        &self,
        sink: &mut [core::mem::MaybeUninit<brvk::VkLayerProperties>],
    ) -> crate::Result<ArrayQueryResult<u32>> {
        crate::vkfn_wrapper::enumerate_device_layer_properties(self.as_transparent_ref(), sink)
    }

    /// Returns properties of available physical device layers
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("alloc")]
    fn enumerate_layer_properties_alloc(&self) -> crate::Result<Vec<brvk::VkLayerProperties>> {
        let count = self.layer_property_count()? as usize;
        if count == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(count);
        let r = self
            .enumerate_layer_properties(xs.spare_capacity_mut())?
            .assert_complete();
        unsafe {
            xs.set_len(r as _);
        }

        Ok(xs)
    }

    /// Returns a count of properties of available physical device extensions
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_LAYER_NOT_PRESENT`]
    #[implements]
    #[inline(always)]
    fn extension_property_count(&self, layer_name: Option<&CStr>) -> crate::Result<u32> {
        crate::vkfn_wrapper::device_extension_property_count(self.as_transparent_ref(), layer_name)
    }

    /// Returns properties of available physical device extensions
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_LAYER_NOT_PRESENT`]
    #[implements]
    #[inline(always)]
    fn enumerate_extension_properties(
        &self,
        layer_name: Option<&CStr>,
        sink: &mut [core::mem::MaybeUninit<brvk::VkExtensionProperties>],
    ) -> crate::Result<ArrayQueryResult<u32>> {
        crate::vkfn_wrapper::enumerate_device_extension_properties(self.as_transparent_ref(), layer_name, sink)
    }

    /// Returns properties of available physical device extensions
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_LAYER_NOT_PRESENT`
    #[implements("alloc")]
    fn enumerate_extension_properties_alloc(
        &self,
        layer_name: Option<&CStr>,
    ) -> crate::Result<Vec<brvk::VkExtensionProperties>> {
        let n = self.extension_property_count(layer_name)? as usize;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(n);
        let r = self
            .enumerate_extension_properties(layer_name, xs.spare_capacity_mut())?
            .assert_complete();
        unsafe {
            xs.set_len(r as _);
        }

        Ok(xs)
    }

    /// Returns properties of available physical device extensions
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_LAYER_NOT_PRESENT`
    #[implements("alloc")]
    #[inline]
    fn enumerate_extension_properties_str_alloc(
        &self,
        layer_name: Option<&str>,
    ) -> crate::Result<Vec<brvk::VkExtensionProperties>> {
        self.enumerate_extension_properties_alloc(layer_name.map(|s| crate::alloc::str_to_cstr(s).unwrap()).as_deref())
    }

    /// Reports capabilities of a physical device.
    #[implements]
    #[inline]
    fn features(&self) -> brvk::VkPhysicalDeviceFeatures {
        let mut p = std::mem::MaybeUninit::uninit();
        crate::vkfn_wrapper::get_physical_device_features(self.as_transparent_ref(), &mut p);

        unsafe { p.assume_init() }
    }

    /// Reports capabilities of a physical device
    #[implements("Allow1_1APIs")]
    #[inline(always)]
    fn features2(&self, sink: &mut core::mem::MaybeUninit<PhysicalDeviceFeatures2>) {
        crate::vkfn_wrapper::get_physical_device_features2(self.as_transparent_ref(), sink)
    }

    /// Reports capabilities of a physical device
    #[implements("VK_KHR_get_physical_device_properties2")]
    #[inline]
    fn features2_khr(&self, sink: &mut core::mem::MaybeUninit<PhysicalDeviceFeatures2>)
    where
        Self::ConcreteInstance: InstanceGetPhysicalDeviceProperties2Extension,
    {
        unsafe {
            self.instance().get_physical_device_features2_khr_fn().0(self.native_ptr(), sink.as_mut_ptr().cast());
        }
    }

    /// Returns properties of a physical device
    #[implements]
    #[inline]
    fn properties(&self) -> brvk::VkPhysicalDeviceProperties {
        let mut p = std::mem::MaybeUninit::uninit();
        crate::vkfn_wrapper::get_physical_device_properties(self.as_transparent_ref(), &mut p);

        unsafe { p.assume_init() }
    }

    /// Returns properties of a physical device
    #[implements("Allow1_1APIs")]
    #[inline(always)]
    fn properties2(&self, sink: &mut core::mem::MaybeUninit<brvk::VkPhysicalDeviceProperties2KHR>) {
        crate::vkfn_wrapper::get_physical_device_properties2(self.as_transparent_ref(), sink)
    }

    /// Returns properties of a physical device
    #[implements("VK_KHR_get_physical_device_properties2")]
    #[inline]
    fn properties2_khr(&self, sink: &mut core::mem::MaybeUninit<brvk::VkPhysicalDeviceProperties2KHR>)
    where
        Self::ConcreteInstance: InstanceGetPhysicalDeviceProperties2Extension,
    {
        unsafe {
            self.instance().get_physical_device_properties2_khr_fn().0(self.native_ptr(), sink.as_mut_ptr());
        }
    }

    /// Lists physical device's format capabilities
    #[implements]
    #[inline]
    fn format_properties(&self, format: brvk::VkFormat) -> brvk::VkFormatProperties {
        let mut p = std::mem::MaybeUninit::uninit();
        crate::vkfn_wrapper::get_physical_device_format_properties(self.as_transparent_ref(), format, &mut p);

        unsafe { p.assume_init() }
    }

    /// Lists physical device's format capabilities
    #[implements("Allow1_1APIs")]
    #[inline(always)]
    fn format_properties2(
        &self,
        format: brvk::VkFormat,
        out: &mut core::mem::MaybeUninit<brvk::VkFormatProperties2KHR>,
    ) {
        crate::vkfn_wrapper::get_physical_device_format_properties2(self.as_transparent_ref(), format, out)
    }

    /// Lists physical device's format capabilities
    #[implements("VK_KHR_get_physical_device_properties2")]
    #[inline]
    fn format_properties2_khr(
        &self,
        format: brvk::VkFormat,
        out: &mut core::mem::MaybeUninit<brvk::VkFormatProperties2KHR>,
    ) where
        Self::ConcreteInstance: InstanceGetPhysicalDeviceProperties2Extension,
    {
        unsafe {
            self.instance().get_physical_device_format_properties2_khr_fn().0(
                self.native_ptr(),
                format,
                out.as_mut_ptr(),
            );
        }
    }

    /// Lists physical device's image format capabilities
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_FORMAT_NOT_SUPPORTED`
    #[implements]
    #[inline]
    fn image_format_properties(
        &self,
        format: brvk::VkFormat,
        itype: brvk::VkImageType,
        tiling: brvk::VkImageTiling,
        usage: ImageUsageFlags,
        flags: ImageFlags,
    ) -> crate::Result<brvk::VkImageFormatProperties> {
        let mut p = std::mem::MaybeUninit::uninit();
        crate::vkfn_wrapper::get_physical_device_image_format_properties(
            self.as_transparent_ref(),
            format,
            itype,
            tiling,
            usage,
            flags,
            &mut p,
        )?;

        Ok(unsafe { p.assume_init() })
    }

    /// Reports a count of properties of the queues of the specified physical device
    #[implements]
    #[inline(always)]
    fn queue_family_property_count(&self) -> u32 {
        crate::vkfn_wrapper::get_physical_device_queue_family_property_count(self.as_transparent_ref())
    }

    /// Reports properties of the queues of the specified physical device
    #[implements]
    #[inline(always)]
    fn queue_family_properties(&self, sink: &mut [core::mem::MaybeUninit<QueueFamilyProperties>]) -> u32 {
        crate::vkfn_wrapper::get_physical_device_queue_family_properties(self.as_transparent_ref(), sink)
    }

    /// Reports properties of the queues of the specified physical device
    #[implements("alloc")]
    fn queue_family_properties_alloc(&self) -> QueueFamilies {
        let n = self.queue_family_property_count() as usize;
        if n == 0 {
            // no items
            return QueueFamilies(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(n);
        let filled = self.queue_family_properties(xs.spare_capacity_mut());
        unsafe {
            xs.set_len(filled as _);
        }

        QueueFamilies(xs)
    }

    /// Reports memory information for the specified physical device
    #[implements]
    #[inline]
    fn memory_properties(&self) -> MemoryProperties {
        let mut p = core::mem::MaybeUninit::uninit();
        crate::vkfn_wrapper::get_physical_device_memory_properties(self.as_transparent_ref(), &mut p);

        unsafe { MemoryProperties(p.assume_init()) }
    }

    /// Retrieve a count of properties of an image format spplied to sparse images
    #[implements]
    #[inline(always)]
    fn sparse_image_format_property_count(
        &self,
        format: brvk::VkFormat,
        image_type: brvk::VkImageType,
        samples: brvk::VkSampleCountFlags,
        usage: ImageUsageFlags,
        tiling: brvk::VkImageTiling,
    ) -> u32 {
        crate::vkfn_wrapper::get_physical_device_sparse_image_format_property_count(
            self.as_transparent_ref(),
            format,
            image_type,
            samples,
            usage,
            tiling,
        )
    }

    /// Retrieve properties of an image format applied to sparse images
    #[implements]
    #[inline(always)]
    fn sparse_image_format_properties(
        &self,
        format: brvk::VkFormat,
        image_type: brvk::VkImageType,
        samples: brvk::VkSampleCountFlags,
        usage: ImageUsageFlags,
        tiling: brvk::VkImageTiling,
        sink: &mut [core::mem::MaybeUninit<brvk::VkSparseImageFormatProperties>],
    ) -> u32 {
        crate::vkfn_wrapper::get_physical_device_sparse_image_format_properties(
            self.as_transparent_ref(),
            format,
            image_type,
            samples,
            usage,
            tiling,
            sink,
        )
    }

    /// Retrieve properties of an image format applied to sparse images
    #[implements]
    #[cfg(feature = "alloc")]
    fn sparse_image_format_properties_alloc(
        &self,
        format: brvk::VkFormat,
        itype: brvk::VkImageType,
        samples: brvk::VkSampleCountFlags,
        usage: ImageUsageFlags,
        tiling: brvk::VkImageTiling,
    ) -> Vec<brvk::VkSparseImageFormatProperties> {
        let n = self.sparse_image_format_property_count(format, itype, samples, usage, tiling);
        if n == 0 {
            // no items
            return crate::alloc::empty_sink_buffer();
        }

        let mut xs = crate::alloc::reserve(n as _);
        let filled =
            self.sparse_image_format_properties(format, itype, samples, usage, tiling, xs.spare_capacity_mut());
        unsafe {
            xs.set_len(filled as _);
        }

        xs
    }

    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[implements("VK_EXT_sample_locations")]
    unsafe fn multisample_properties(
        &self,
        samples: brvk::VkSampleCountFlags,
        sink: &mut core::mem::MaybeUninit<brvk::VkMultisamplePropertiesEXT>,
    ) where
        Self::ConcreteInstance: InstanceSampleLocationsExtension,
    {
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
    #[inline(always)]
    unsafe fn external_fence_properties(
        &self,
        info: &brvk::VkPhysicalDeviceExternalFenceInfoKHR,
        sink: &mut core::mem::MaybeUninit<brvk::VkExternalFencePropertiesKHR>,
    ) where
        Self::ConcreteInstance: InstanceExternalFenceCapabilitiesExtension,
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
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline(always)]
    fn surface_support(&self, queue_family: u32, surface: &(impl Surface + ?Sized)) -> crate::Result<bool> {
        unsafe {
            crate::vkfn_wrapper::get_physical_device_surface_support(
                self.as_transparent_ref(),
                queue_family,
                surface.as_transparent_ref(),
            )
        }
    }

    /// Query surface capabilities
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline]
    fn surface_capabilities(&self, surface: &(impl Surface + ?Sized)) -> crate::Result<SurfaceCapabilities> {
        let mut s = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn_wrapper::get_physical_device_surface_capabilities(
                self.as_transparent_ref(),
                surface.as_transparent_ref(),
                &mut s,
            )?;
        }

        Ok(unsafe { s.assume_init() })
    }

    /// Reports capabilities of a surface on a physical device
    ///
    /// # Failures
    ///
    /// On failure, this command returns
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_SURFACE_LOST_KHR`]
    ///
    /// # Safety
    /// Caller must guarantee that all write operations to `sink` and its `pNext` fields are safe
    #[implements("VK_KHR_get_surface_capabilities2")]
    #[inline(always)]
    unsafe fn surface_capabilities2(
        &self,
        surface_info: &brvk::VkPhysicalDeviceSurfaceInfo2KHR,
        sink: &mut core::mem::MaybeUninit<brvk::VkSurfaceCapabilities2KHR>,
    ) -> crate::Result<()>
    where
        Self::ConcreteInstance: InstanceGetSurfaceCapabilities2Extension,
    {
        unsafe {
            self.instance()
                .get_physical_device_surface_capabilities2(self.as_transparent_ref(), surface_info, sink)
        }
    }

    /// Query a count of color formats supported by surface
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline(always)]
    fn surface_format_count(
        &self,
        surface: &(impl VkHandle<Handle = brvk::VkSurfaceKHR> + ?Sized),
    ) -> crate::Result<u32> {
        unsafe {
            crate::vkfn_wrapper::get_physical_device_surface_format_count(
                self.as_transparent_ref(),
                surface.as_transparent_ref(),
            )
        }
    }

    /// Query a count of color formats supported by surface
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline(always)]
    fn surface_formats(
        &self,
        surface: &(impl VkHandle<Handle = brvk::VkSurfaceKHR> + ?Sized),
        sink: &mut [core::mem::MaybeUninit<brvk::VkSurfaceFormatKHR>],
    ) -> crate::Result<ArrayQueryResult<u32>> {
        unsafe {
            crate::vkfn_wrapper::get_physical_device_surface_formats(
                self.as_transparent_ref(),
                surface.as_transparent_ref(),
                sink,
            )
        }
    }

    /// Query color formats supported by surface
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface", "alloc")]
    fn surface_formats_alloc(&self, surface: &(impl Surface + ?Sized)) -> crate::Result<Vec<brvk::VkSurfaceFormatKHR>> {
        let n = self.surface_format_count(surface)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(n as _);
        let filled = self
            .surface_formats(surface, xs.spare_capacity_mut())?
            .assert_complete();
        unsafe {
            xs.set_len(filled as _);
        }

        Ok(xs)
    }

    /// Query a count of supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline(always)]
    fn surface_present_mode_count(
        &self,
        surface: &(impl VkHandle<Handle = brvk::VkSurfaceKHR> + ?Sized),
    ) -> crate::Result<u32> {
        unsafe {
            crate::vkfn_wrapper::get_physical_device_surface_present_mode_count(
                self.as_transparent_ref(),
                surface.as_transparent_ref(),
            )
        }
    }

    /// Query supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface")]
    #[inline(always)]
    fn surface_present_modes(
        &self,
        surface: &(impl VkHandle<Handle = brvk::VkSurfaceKHR> + ?Sized),
        sink: &mut [core::mem::MaybeUninit<PresentMode>],
    ) -> crate::Result<ArrayQueryResult<u32>> {
        unsafe {
            crate::vkfn_wrapper::get_physical_device_surface_present_modes(
                self.as_transparent_ref(),
                surface.as_transparent_ref(),
                sink,
            )
        }
    }

    /// Query supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_surface", "alloc")]
    fn surface_present_modes_alloc(&self, surface: &(impl Surface + ?Sized)) -> crate::Result<Vec<PresentMode>> {
        let n = self.surface_present_mode_count(surface)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(n as _);
        let filled = self
            .surface_present_modes(surface, xs.spare_capacity_mut())?
            .assert_complete();
        unsafe {
            xs.set_len(filled as _);
        }

        Ok(xs)
    }

    /// Query physical device for presentation to X11 server using Xlib
    ///
    /// # Safety
    ///
    /// Provided `display` must be a valid reference
    #[implements("VK_KHR_xlib_surface")]
    #[inline(always)]
    unsafe fn xlib_presentation_support(
        &self,
        queue_family: u32,
        display: *mut x11::xlib::Display,
        visual: x11::xlib::VisualID,
    ) -> bool {
        unsafe {
            crate::vkfn_wrapper::get_physical_device_xlib_presentation_support(
                self.as_transparent_ref(),
                queue_family,
                display,
                visual,
            )
        }
    }

    /// Query physical device for presentation to X11 server using XCB
    ///
    /// # Safety
    ///
    /// Provided `connection` must be a valid reference
    #[implements("VK_KHR_xcb_surface")]
    #[inline(always)]
    unsafe fn xcb_presentation_support(
        &self,
        queue_family: u32,
        connection: *mut xcb::ffi::xcb_connection_t,
        visual: xcb::x::Visualid,
    ) -> bool {
        unsafe {
            crate::vkfn_wrapper::get_physical_device_xcb_presentation_support(
                self.as_transparent_ref(),
                queue_family,
                connection,
                visual,
            )
        }
    }

    /// Query physical device for presentation to Wayland
    ///
    /// # Safety
    ///
    /// Provided `display` must be a valid reference
    #[implements("VK_KHR_wayland_surface")]
    #[inline(always)]
    unsafe fn wayland_presentation_support(&self, queue_family: u32, display: *mut core::ffi::c_void) -> bool {
        unsafe {
            crate::vkfn_wrapper::get_physical_device_wayland_presentation_support(
                self.as_transparent_ref(),
                queue_family,
                display,
            )
        }
    }

    /// Query queue family support for presentation on a Win32 display
    #[implements("VK_KHR_win32_surface")]
    #[inline(always)]
    fn win32_presentation_support(&self, queue_family: u32) -> bool {
        crate::vkfn_wrapper::get_physical_device_win32_presentation_support(self.as_transparent_ref(), queue_family)
    }

    /// Query a count of the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline(always)]
    fn display_mode_property_count(&self, display: VkHandleRef<brvk::VkDisplayKHR>) -> crate::Result<u32> {
        unsafe { crate::vkfn_wrapper::get_display_mode_property_count(self.as_transparent_ref(), display) }
    }

    /// Query the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline(always)]
    fn display_mode_properties(
        &self,
        display: VkHandleRef<brvk::VkDisplayKHR>,
        sink: &mut [core::mem::MaybeUninit<DisplayModeProperties>],
    ) -> crate::Result<ArrayQueryResult<u32>> {
        unsafe { crate::vkfn_wrapper::get_display_mode_properties(self.as_transparent_ref(), display, sink) }
    }

    /// Query the set of mode properties supported by the display
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display", "alloc")]
    fn display_mode_properties_alloc(
        &self,
        display: VkHandleRef<brvk::VkDisplayKHR>,
    ) -> crate::Result<Vec<DisplayModeProperties>> {
        let n = self.display_mode_property_count(display)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(n as _);
        let res = self
            .display_mode_properties(display, xs.spare_capacity_mut())?
            .assert_complete();
        unsafe {
            xs.set_len(res as _);
        }

        Ok(xs)
    }

    /// Create a display mode
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_INITIALIZATION_FAILED`
    ///
    /// # Safety
    /// no guarantee will be provided (simply calls under api)
    #[implements("VK_KHR_display")]
    #[inline]
    unsafe fn new_display_mode_raw(
        &self,
        display: brvk::VkDisplayKHR,
        info: &brvk::VkDisplayModeCreateInfoKHR,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkDisplayModeKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        translate_vk_result(unsafe {
            brvk::fns::create_display_mode_khr(
                self.native_ptr(),
                display,
                info,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Query capabilities of a mode and plane combination
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline]
    fn display_plane_capabilities(
        &self,
        mode: VkHandleRefMut<brvk::VkDisplayModeKHR>,
        plane_index: u32,
    ) -> crate::Result<brvk::VkDisplayPlaneCapabilitiesKHR> {
        let mut s = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn_wrapper::get_display_plane_capabilities(self.as_transparent_ref(), mode, plane_index, &mut s)?;
        }

        Ok(unsafe { s.assume_init() })
    }

    /// Query a count of information about the available displays
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline(always)]
    fn display_property_count(&self) -> crate::Result<u32> {
        crate::vkfn_wrapper::get_physical_device_display_property_count(self.as_transparent_ref())
    }

    /// Query information about the available displays
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline(always)]
    fn display_properties(
        &self,
        sink: &mut [core::mem::MaybeUninit<DisplayProperties>],
    ) -> crate::Result<ArrayQueryResult<u32>> {
        crate::vkfn_wrapper::get_physical_device_display_properties(self.as_transparent_ref(), sink)
    }

    /// Query information about the available displays.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// * brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements("VK_KHR_display", "alloc")]
    fn display_properties_alloc(&self) -> crate::Result<Vec<DisplayPropertiesWithPhysicalDeviceRef<&Self>>> {
        let n = self.display_property_count()?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(n as _);
        let r = self.display_properties(xs.spare_capacity_mut())?.assert_complete();
        unsafe {
            xs.set_len(r as _);
        }

        Ok(crate::alloc::collect_vec(
            xs.into_iter()
                .map(move |x| DisplayPropertiesWithPhysicalDeviceRef(x, self)),
        ))
    }

    /// Query a count of the plane properties
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline(always)]
    fn display_plane_property_count(&self) -> crate::Result<u32> {
        crate::vkfn_wrapper::get_physical_device_display_plane_property_count(self.as_transparent_ref())
    }

    /// Query the plane properties
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline(always)]
    fn display_plane_properties(
        &self,
        sink: &mut [core::mem::MaybeUninit<brvk::VkDisplayPlanePropertiesKHR>],
    ) -> crate::Result<ArrayQueryResult<u32>> {
        crate::vkfn_wrapper::get_physical_device_display_plane_properties(self.as_transparent_ref(), sink)
    }

    /// Query the plane properties.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// * brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements("VK_KHR_display", "alloc")]
    fn display_plane_properties_alloc(&self) -> crate::Result<Vec<DisplayPlanePropertiesWithPhysicalDeviceRef<&Self>>> {
        let n = self.display_plane_property_count()?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(n as _);
        let r = self
            .display_plane_properties(xs.spare_capacity_mut())?
            .assert_complete();
        unsafe {
            xs.set_len(r as _);
        }

        Ok(crate::alloc::collect_vec(
            xs.into_iter()
                .map(move |x| DisplayPlanePropertiesWithPhysicalDeviceRef(x, self)),
        ))
    }

    /// Query a count of the list of displays a plane supports
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline(always)]
    fn display_plane_supported_display_count(&self, plane_index: u32) -> crate::Result<u32> {
        crate::vkfn_wrapper::get_display_plane_supported_display_count(self.as_transparent_ref(), plane_index)
    }

    /// Query the list of displays a plane supports
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_KHR_display")]
    #[inline(always)]
    fn display_plane_supported_displays(
        &self,
        plane_index: u32,
        sink: &mut [core::mem::MaybeUninit<brvk::VkDisplayKHR>],
    ) -> crate::Result<ArrayQueryResult<u32>> {
        crate::vkfn_wrapper::get_display_plane_supported_displays(self.as_transparent_ref(), plane_index, sink)
    }

    /// Query the list of displays a plane supports.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// * brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements("VK_KHR_display", "alloc")]
    fn display_plane_supported_displays_alloc(&self, plane_index: u32) -> crate::Result<Vec<Display<&Self>>> {
        let n = self.display_plane_supported_display_count(plane_index)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(n as _);
        let r = self
            .display_plane_supported_displays(plane_index, xs.spare_capacity_mut())?
            .assert_complete();
        unsafe {
            xs.set_len(r as _);
        }

        Ok(crate::alloc::collect_vec(xs.into_iter().map(move |x| Display(x, self))))
    }

    /// Create a `Surface` object representing a display plane and mode
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantee will be provided (simply calls under api)
    #[implements("VK_KHR_display", "VK_KHR_surface")]
    #[inline]
    unsafe fn new_surface_for_display_plane_raw(
        &self,
        info: &brvk::VkDisplaySurfaceCreateInfoKHR,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkSurfaceKHR> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            brvk::fns::create_display_plane_surface_khr(
                self.instance().native_ptr(),
                info,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Query the brvk::VkDisplayKHR corresponding to an X11 RandR Output
    /// # Failures
    /// On failure, this command returns
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
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
        Self::ConcreteInstance: InstanceAcquireXlibDisplayExtension,
    {
        let mut d = core::mem::MaybeUninit::uninit();

        crate::error::translate_vk_result(unsafe {
            self.instance().get_randr_output_display_ext_fn().0(self.native_ptr(), dpy, rr_output, d.as_mut_ptr())
        })?;

        Ok(Display(unsafe { d.assume_init() }, self))
    }

    /// Query a count of supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_SURFACE_LOST_KHR`]
    #[implements("VK_EXT_full_screen_exclusive")]
    #[inline(always)]
    fn surface_present_mode2_count(&self, surface_info: &brvk::VkPhysicalDeviceSurfaceInfo2KHR) -> crate::Result<u32>
    where
        Self::ConcreteInstance: InstanceFullScreenExclusiveExtension,
    {
        unsafe {
            self.instance()
                .get_physical_device_surface_present_mode2_count(self.as_transparent_ref(), surface_info)
        }
    }

    /// Query supported presentation modes
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_SURFACE_LOST_KHR`]
    #[implements("VK_EXT_full_screen_exclusive")]
    #[inline(always)]
    fn surface_present_modes2(
        &self,
        surface_info: &brvk::VkPhysicalDeviceSurfaceInfo2KHR,
        sink: &mut [core::mem::MaybeUninit<brvk::VkPresentModeKHR>],
    ) -> crate::Result<ArrayQueryResult<u32>>
    where
        Self::ConcreteInstance: InstanceFullScreenExclusiveExtension,
    {
        unsafe {
            self.instance()
                .get_physical_device_surface_present_modes2(self.as_transparent_ref(), surface_info, sink)
        }
    }

    /// Query supported presentation modes.
    ///
    /// # Failures
    ///
    /// On failure, this command returns
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_SURFACE_LOST_KHR`]
    #[implements("VK_EXT_full_screen_exclusive", "alloc")]
    fn surface_present_modes2_alloc(
        &self,
        surface_info: &brvk::VkPhysicalDeviceSurfaceInfo2KHR,
    ) -> crate::Result<Vec<brvk::VkPresentModeKHR>>
    where
        Self::ConcreteInstance: InstanceFullScreenExclusiveExtension,
    {
        let n = self.surface_present_mode2_count(surface_info)?;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(n as _);
        let r = self.surface_present_modes2(surface_info, xs.spare_capacity_mut())?;
        assert!(!r.is_incomplete);
        unsafe {
            xs.set_len(r.result as _);
        }

        Ok(xs)
    }
}
DerefContainerWithGuardsBracketImpl!(for PhysicalDevice {});

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
    #[inline(always)]
    fn transfer_instance(self) -> Self::ConcreteInstance {
        self.instance().clone()
    }
}

pub type PhysicalDeviceFeatures = brvk::VkPhysicalDeviceFeatures;
#[cfg(feature = "Allow1_2APIs")]
pub type PhysicalDeviceVulkan11Features = brvk::VkPhysicalDeviceVulkan11Features;

pub type PhysicalDeviceProperties = brvk::VkPhysicalDeviceProperties;
#[cfg(feature = "Allow1_2APIs")]
pub type PhysicalDeviceVulkan11Properties = brvk::VkPhysicalDeviceVulkan11Properties;

#[repr(transparent)]
pub struct PhysicalDeviceMemoryProperties(brvk::VkPhysicalDeviceMemoryProperties);
impl PhysicalDeviceMemoryProperties {
    #[inline(always)]
    pub fn types(&self) -> &[MemoryType] {
        unsafe { core::mem::transmute::<&[_], &[_]>(&self.0.memoryTypes[..self.0.memoryTypeCount as _]) }
    }

    #[inline(always)]
    pub fn heaps(&self) -> &[MemoryHeap] {
        unsafe { core::mem::transmute::<&[_], &[_]>(&self.0.memoryHeaps[..self.0.memoryHeapCount as _]) }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MemoryType(brvk::VkMemoryType);
impl MemoryType {
    pub const fn property_flags(&self) -> MemoryPropertyFlags {
        MemoryPropertyFlags(self.0.propertyFlags)
    }

    pub const fn heap_index(&self) -> u32 {
        self.0.heapIndex
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MemoryHeap(brvk::VkMemoryHeap);
impl MemoryHeap {
    pub const fn flags(&self) -> MemoryHeapFlags {
        MemoryHeapFlags(self.0.flags)
    }

    pub const fn size(&self) -> brvk::VkDeviceSize {
        self.0.size
    }
}

/// Device memory properties
#[repr(transparent)]
pub struct MemoryProperties(PhysicalDeviceMemoryProperties);
impl MemoryProperties {
    #[inline(always)]
    pub fn types(&self) -> &[MemoryType] {
        self.0.types()
    }
    #[inline(always)]
    pub fn heaps(&self) -> &[MemoryHeap] {
        self.0.heaps()
    }

    #[inline]
    pub fn find_type_index(
        &self,
        mask: MemoryPropertyFlags,
        exclude: MemoryPropertyFlags,
        index_mask: u32,
    ) -> Option<u32> {
        self.types().iter().enumerate().find_map(|(i, mt)| {
            (index_mask & (1u32 << i) != 0
                && mt.property_flags().has_all(mask)
                && !mt.property_flags().has_any(exclude))
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
        self.0.types()[index as usize]
            .property_flags()
            .has_any(MemoryPropertyFlags::HOST_COHERENT)
    }
    pub fn is_cached(&self, index: u32) -> bool {
        self.0.types()[index as usize]
            .property_flags()
            .has_any(MemoryPropertyFlags::HOST_CACHED)
    }
}

/// Bitmask specifying properties for a memory type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct MemoryPropertyFlags(brvk::VkMemoryPropertyFlags);
impl MemoryPropertyFlags {
    /// Empty set
    pub const EMPTY: Self = MemoryPropertyFlags(0);

    /// Memory allocated with this type is the most efficient for device access
    pub const DEVICE_LOCAL: Self = MemoryPropertyFlags(brvk::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
    /// Memory allocated with this type can be mapped for host access using `vkMapMemory`
    pub const HOST_VISIBLE: Self = MemoryPropertyFlags(brvk::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT);
    /// The host cache management commands `vkFlushMappedMemoryRanges` and `vkInvalidateMappedMemoryRanges`
    /// are not needed to flush host writes to the device or make device writes visible to the host, respectively.
    pub const HOST_COHERENT: Self = MemoryPropertyFlags(brvk::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
    /// Memory allocated with this type is cached on the host.
    /// Host memory accesses to uncached memory are slower than to cached memory, however uncached memory is always host coherent
    pub const HOST_CACHED: Self = MemoryPropertyFlags(brvk::VK_MEMORY_PROPERTY_HOST_CACHED_BIT);
    /// The memory type only allows device access to the memory.
    pub const LAZILY_ALLOCATED: Self = MemoryPropertyFlags(brvk::VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT);

    #[cfg(feature = "Allow1_1APIs")]
    /// The memory type only allows device access to the memory, and allows protected queue operations to access the memory.
    pub const PROTECTED: Self = Self(brvk::VK_MEMORY_PROPERTY_PROTECTED_BIT);
}

/// Bitmask specifying attribute flags for a heap
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct MemoryHeapFlags(brvk::VkMemoryHeapFlags);
impl MemoryHeapFlags {
    /// Empty set
    pub const EMPTY: Self = Self(0);

    /// The heap corresponds to device-local memory.
    /// Device-local memory may have different performance characteristics than host-local memory,
    /// and may support different memory property flags.
    pub const DEVICE_LOCAL: Self = Self(brvk::VK_MEMORY_HEAP_DEVICE_LOCAL_BIT);

    #[cfg(feature = "Allow1_1APIs")]
    /// In a logical device representing more than one physical device,
    /// there is a per-physical device instance of the heap memory.
    /// By default, an allocation from such a heap will be replicated to each physical device's instance of the heap.
    pub const MULTI_INSTANCE: Self = Self(brvk::VK_MEMORY_HEAP_MULTI_INSTANCE_BIT);
}

/// List of queue families
pub struct QueueFamilies(pub Vec<QueueFamilyProperties>);
impl QueueFamilies {
    /// Find a queue family index containing specified bitflags
    pub fn find_matching_index(&self, flags: QueueFlags) -> Option<u32> {
        self.0
            .iter()
            .position(|q| q.queue_flags().has_all(flags))
            .map(|x| x as _)
    }

    /// Find a queue family index containing specified bitflags
    pub fn find_another_matching_index(&self, flags: QueueFlags, exclude: u32) -> Option<u32> {
        self.0
            .iter()
            .enumerate()
            .find_map(|(n, p)| (p.queue_flags().has_all(flags) && exclude != n as u32).then_some(n as _))
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
        self.0[family_index as usize].queue_count()
    }

    /// Unsigned integer count of meaningful bits in the timestamps written via `vkCmdWriteTimestamp`
    pub fn timestamp_valid_bits(&self, family_index: u32) -> u32 {
        self.0[family_index as usize].timestamp_valid_bits()
    }

    /// Minimum granularity supported for image transfer operations on the queues in selected queue family
    pub fn minimum_image_transfer_granularity(&self, family_index: u32) -> &brvk::VkExtent3D {
        self.0[family_index as usize].min_image_transfer_granularity()
    }
}

#[repr(transparent)]
pub struct QueueFamilyProperties(brvk::VkQueueFamilyProperties);
impl QueueFamilyProperties {
    pub const fn queue_flags(&self) -> QueueFlags {
        QueueFlags(self.0.queueFlags)
    }

    pub const fn queue_count(&self) -> u32 {
        self.0.queueCount
    }

    pub const fn timestamp_valid_bits(&self) -> u32 {
        self.0.timestampValidBits
    }

    pub const fn min_image_transfer_granularity(&self) -> &brvk::VkExtent3D {
        &self.0.minImageTransferGranularity
    }
}

/// Set of bit of queue flags
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[bitflags_newtype]
pub struct QueueFlags(brvk::VkQueueFlags);
impl QueueFlags {
    /// Empty bits
    pub const EMPTY: Self = QueueFlags(0);
    /// Supports only graphics operations
    pub const GRAPHICS: Self = QueueFlags(brvk::VK_QUEUE_GRAPHICS_BIT);
    /// Supports only compute operations
    pub const COMPUTE: Self = QueueFlags(brvk::VK_QUEUE_COMPUTE_BIT);
    /// Supports only transfer operations
    pub const TRANSFER: Self = QueueFlags(brvk::VK_QUEUE_TRANSFER_BIT);
    /// Supports only sparse memory management operations
    pub const SPARSE_BINDING: Self = QueueFlags(brvk::VK_QUEUE_SPARSE_BINDING_BIT);
}

#[cfg(feature = "VK_KHR_display")]
mod display;
#[cfg(feature = "VK_KHR_display")]
pub use self::display::*;
