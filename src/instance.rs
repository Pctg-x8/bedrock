use core::{
    ffi::CStr,
    mem::MaybeUninit,
    ptr::{null_mut, NonNull},
};

use derives::implements;

use crate::{
    alloc::alloc_sink_buffer,
    ffi_helper::{opt_cstr_ptr, opt_pointer},
    vk::*,
    vkfn::{
        create_instance, destroy_instance, enumerate_instance_extension_properties,
        enumerate_instance_layer_properties, enumerate_instance_version, enumerate_physical_devices,
        get_instance_proc_addr,
    },
    EnumerationResult, ExtensionProperties, InstanceCreateInfo, LayerProperties, PhysicalDevice, Version, PFN,
};

/// Opaque handle to an instance object.
#[repr(transparent)]
pub struct Instance(VkInstance_T);
#[implements]
impl Instance {
    /// Query instance-level version before instance creation.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    #[inline]
    pub fn version() -> crate::Result<Version> {
        #[cfg(feature = "Allow1_1APIs")]
        unsafe {
            let mut sink = 0;
            enumerate_instance_version(&mut sink).into_result()?;
            Ok(Version(sink))
        }
        // fixed to v1.0.0
        #[cfg(not(feature = "Allow1_1APIs"))]
        Ok(Version::V1)
    }

    /// Returns a number of available global layer properties.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline]
    pub fn layer_property_count() -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            enumerate_instance_layer_properties(&mut n, null_mut()).into_result()?;
        }

        Ok(n)
    }

    /// Returns up to requested number of global layer properties.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline]
    pub fn layer_properties(sink: &mut [LayerProperties]) -> crate::Result<(u32, EnumerationResult)> {
        let mut n = sink.len() as _;
        let r = unsafe { enumerate_instance_layer_properties(&mut n, sink.as_mut_ptr()) };
        if r == VK_SUCCESS {
            return Ok((n, EnumerationResult::Complete));
        }
        if r == VK_INCOMPLETE {
            return Ok((n, EnumerationResult::Incomplete));
        }

        Err(r)
    }

    /// Returns all of available global layer properties.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn layer_properties_alloc() -> crate::Result<Vec<LayerProperties>> {
        let n = Self::layer_property_count()?;
        let mut v = unsafe { alloc_sink_buffer(n as _) };
        // ここでは全部とれる(Incompleteの対処は不要)
        Self::layer_properties(&mut v)?;

        Ok(v)
    }

    /// Returns a number of available global extension properties.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_LAYER_NOT_PRESENT`]
    #[inline]
    pub fn extension_property_count(layer_name: Option<&CStr>) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            enumerate_instance_extension_properties(opt_cstr_ptr(layer_name), &mut n, null_mut()).into_result()?;
        }

        Ok(n)
    }

    /// Returns up to requested number of global extension properties.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_LAYER_NOT_PRESENT`]
    pub fn extension_properties(
        layer_name: Option<&CStr>,
        sink: &mut [ExtensionProperties],
    ) -> crate::Result<(u32, EnumerationResult)> {
        let mut n = sink.len() as _;
        let r = unsafe { enumerate_instance_extension_properties(opt_cstr_ptr(layer_name), &mut n, sink.as_mut_ptr()) };
        if r == VK_SUCCESS {
            return Ok((n, EnumerationResult::Complete));
        }
        if r == VK_INCOMPLETE {
            return Ok((n, EnumerationResult::Incomplete));
        }

        Err(r)
    }

    /// Returns all of available global extension properties.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_LAYER_NOT_PRESENT`]
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn extension_properties_alloc(layer_name: Option<&CStr>) -> crate::Result<Vec<ExtensionProperties>> {
        let n = Self::extension_property_count(layer_name)?;
        let mut v = unsafe { alloc_sink_buffer(n as _) };
        // ここでは全部とれる(Incompleteの対処は不要)
        Self::extension_properties(layer_name, &mut v)?;

        Ok(v)
    }

    /// Create a new Vulkan instance.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INITIALIZATION_FAILED`]
    /// * [`VK_ERROR_LAYER_NOT_PRESENT`]
    /// * [`VK_ERROR_EXTENSION_NOT_PRESENT`]
    /// * [`VK_ERROR_INCOMPATIBLE_DRIVER`]
    #[inline]
    pub fn new(info: &InstanceCreateInfo, allocator: Option<&VkAllocationCallbacks>) -> crate::Result<NonNull<Self>> {
        let mut h = MaybeUninit::uninit();
        unsafe {
            create_instance(info as *const _ as _, opt_pointer(allocator), h.as_mut_ptr()).into_result()?;
        }

        Ok(unsafe { NonNull::new_unchecked(h.assume_init() as _) })
    }

    /// Return a function pointer for a command.
    #[inline(always)]
    pub fn proc_addr_raw(&self, name: &core::ffi::CStr) -> Option<PFN_vkVoidFunction> {
        unsafe { get_instance_proc_addr(self as *const _ as _, name.as_ptr()) }
    }

    /// Return a function pointer for a command.
    #[inline(always)]
    pub fn proc_addr<F: PFN>(&self) -> Option<F> {
        self.proc_addr_raw(F::NAME_CSTR).map(|x| unsafe { F::from_void_fn(x) })
    }

    /// Returns a number of physical devices accessible to a Vulkan instance.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INITIALIZATION_FAILED`]
    #[inline]
    pub fn physical_device_count(&self) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            enumerate_physical_devices(self as *const _ as _, &mut n, null_mut()).into_result()?;
        }

        Ok(n)
    }

    /// Enumerates the physical devices accessible to a Vulkan instance.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INITIALIZATION_FAILED`]
    pub fn physical_devices(&self, sink: &mut [NonNull<PhysicalDevice>]) -> crate::Result<(u32, EnumerationResult)> {
        let mut n = sink.len() as _;
        let r = unsafe { enumerate_physical_devices(self as *const _ as _, &mut n, sink.as_mut_ptr() as _) };
        if r == VK_SUCCESS {
            return Ok((n, EnumerationResult::Complete));
        }
        if r == VK_INCOMPLETE {
            return Ok((n, EnumerationResult::Incomplete));
        }

        Err(r)
    }

    /// Enumerates the physical devices accessible to a Vulkan instance.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INITIALIZATION_FAILED`]
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn physical_devices_alloc(&self) -> crate::Result<Vec<NonNull<PhysicalDevice>>> {
        let n = self.physical_device_count()?;
        let mut v = unsafe { alloc_sink_buffer(n as _) };
        // ここでは全部とれる(Incompleteの対処は不要)
        self.physical_devices(&mut v)?;

        Ok(v)
    }

    /// ## Safety
    /// * Accessing the object after calling destroy is undefined behavior.
    /// * allocator must be same instance as passed to [`Instance::new`]
    #[inline]
    pub unsafe fn destroy(&mut self, allocator: Option<&VkAllocationCallbacks>) {
        unsafe {
            destroy_instance(self as *mut _ as _, opt_pointer(allocator));
        }
    }
}
