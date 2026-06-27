//! Vulkan Device and Queues

use crate::ffi_helper::{CStrFFIRef, slice_as_ptr_empty_null};
use crate::*;
use derives::implements;

#[implements]
#[allow(dead_code)]
type DeviceResolvedFn<F> = crate::resolver::ResolvedFnCell<F, VkDevice>;
#[implements]
impl crate::resolver::ResolverInterface for VkDevice {
    #[inline(always)]
    unsafe fn load_symbol_unconstrainted<T: crate::resolver::FromPtr>(&self, name: &core::ffi::CStr) -> T {
        unsafe {
            T::from_ptr(core::mem::transmute::<Option<PFN_vkVoidFunction>, *const _>(
                crate::vkfn::get_device_proc_addr(*self, name.as_ptr() as _),
            ))
        }
    }

    #[tracing::instrument(name = "<VkDevice as ResolverInterface>::load_function_unconstrainted", skip(self), fields(name = ?F::NAME_CSTR))]
    unsafe fn load_function_unconstrainted<F: crate::resolver::PFN>(&self) -> F {
        match unsafe { crate::vkfn::get_device_proc_addr(*self, F::NAME_CSTR.as_ptr() as _) } {
            Some(x) => unsafe { F::from_void_fn(x) },
            None => {
                tracing::error!("device function not found, bedrock could not continue");
                std::process::abort();
            }
        }
    }
}

/// Opaque handle to a device object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VK_OBJECT_TYPE_DEVICE)]
pub struct DeviceObject<Instance> {
    #[handle]
    handle: VkDevice,
    parent: Instance,
    #[cfg(feature = "Implements")]
    ext: DeviceExtFunctions,
}
impl<Instance> DeviceObject<Instance> {
    pub const fn wrap_handle(handle: VkDevice, parent: Instance) -> Self {
        Self {
            handle,
            parent,
            #[cfg(feature = "Implements")]
            ext: DeviceExtFunctions::new(handle),
        }
    }
}
unsafe impl<Instance: Sync> Sync for DeviceObject<Instance> {}
unsafe impl<Instance: Send> Send for DeviceObject<Instance> {}
#[implements]
impl<Instance> Drop for DeviceObject<Instance> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn::destroy_device(self.handle, std::ptr::null());
        }
    }
}
impl<Instance: crate::Instance> InstanceChild for DeviceObject<Instance> {
    type ConcreteInstance = Instance;

    #[inline(always)]
    fn instance(&self) -> &Self::ConcreteInstance {
        &self.parent
    }
}
impl<Instance: crate::Instance> Device for DeviceObject<Instance> {}
impl<Instance: crate::Instance + Clone> DeviceObject<&'_ Instance> {
    /// Clones parent reference
    #[inline]
    pub fn clone_parent(self) -> DeviceObject<Instance> {
        let r = DeviceObject {
            handle: self.handle,
            parent: self.parent.clone(),
            #[cfg(feature = "Implements")]
            ext: unsafe { core::ptr::read(&self.ext) },
        };
        // disable running VkDevice destruction
        std::mem::forget(self);

        r
    }
}
impl<Instance: crate::Instance> DeviceObject<Instance> {
    /// Create a new device instance
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_INITIALIZATION_FAILED`
    /// * `VK_ERROR_EXTENSION_NOT_PRESENT`
    /// * `VK_ERROR_FEATURE_NOT_PRESENT`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_DEVICE_LOST`
    #[implements]
    pub fn new<
        PhysicalDevice: crate::PhysicalDevice + crate::InstanceChildTransferrable<ConcreteInstance = Instance>,
    >(
        physical_device: PhysicalDevice,
        info: &DeviceCreateInfo,
    ) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_device(physical_device.native_ptr(), &info.0, core::ptr::null(), h.as_mut_ptr())
                .into_result()?;

            Ok(Self::wrap_handle(h.assume_init(), physical_device.transfer_instance()))
        }
    }

    /// Constructs from raw handle
    /// # Safety
    /// the handle must be valid and not freed
    pub const unsafe fn manage(handle: VkDevice, parent: Instance) -> Self {
        Self::wrap_handle(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub fn unmanage(mut self) -> (VkDevice, Instance) {
        let h = self.handle;
        let p = unsafe { core::ptr::read(&self.parent) };
        #[cfg(feature = "Implements")]
        unsafe {
            core::ptr::drop_in_place(&mut self.ext);
        }
        core::mem::forget(self);

        (h, p)
    }
}

/// Valid handle of a [`VkDevice`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VkHandle)]
pub struct DeviceHandle(VkDevice);
impl DeviceHandle {
    /// Constructs a [`DeviceHandle`] from a raw [`VkDevice`] handle.
    ///
    /// # Safety
    ///
    /// `raw` must be a valid device handle.
    pub const unsafe fn from_raw(raw: VkDevice) -> Self {
        Self(raw)
    }

    /// Converts this [`DeviceHandle`] back into a raw [`VkDevice`] handle.
    pub const fn into_raw(self) -> VkDevice {
        self.0
    }
}

/// Family Index, Queue Priorities
#[repr(transparent)]
pub struct DeviceQueueCreateInfo<'d>(VkDeviceQueueCreateInfo, core::marker::PhantomData<&'d [f32]>);
impl<'d> DeviceQueueCreateInfo<'d> {
    pub const fn new(family_index: u32, priorities: &'d [f32]) -> Self {
        Self(
            VkDeviceQueueCreateInfo {
                sType: VkDeviceQueueCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                queueFamilyIndex: family_index,
                queueCount: priorities.len() as _,
                pQueuePriorities: slice_as_ptr_empty_null(priorities),
            },
            core::marker::PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DeviceCreateInfo<'d>(
    VkDeviceCreateInfo,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        Option<&'d dyn VulkanStructure>,
        &'d [DeviceQueueCreateInfo<'d>],
        &'d [CStrFFIRef<'d>],
        &'d [CStrFFIRef<'d>],
        Option<&'d VkPhysicalDeviceFeatures>,
    )>,
);
impl<'d> DeviceCreateInfo<'d> {
    pub const fn new(
        queue_infos: &'d [DeviceQueueCreateInfo<'d>],
        layers: &'d [CStrFFIRef<'d>],
        extensions: &'d [CStrFFIRef<'d>],
    ) -> Self {
        Self(
            VkDeviceCreateInfo {
                sType: VkDeviceCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                queueCreateInfoCount: queue_infos.len() as _,
                pQueueCreateInfos: slice_as_ptr_empty_null(queue_infos) as _,
                enabledLayerCount: layers.len() as _,
                ppEnabledLayerNames: slice_as_ptr_empty_null(layers) as _,
                enabledExtensionCount: extensions.len() as _,
                ppEnabledExtensionNames: slice_as_ptr_empty_null(extensions) as _,
                pEnabledFeatures: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkDeviceCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: VkDeviceCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkDeviceCreateInfo {
        self.0
    }

    pub const fn with_features(mut self, features: &'d VkPhysicalDeviceFeatures) -> Self {
        self.0.pEnabledFeatures = features as *const _ as _;
        self
    }

    pub fn with_next(mut self, next: &'d (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Clone)]
pub struct PhysicalDeviceFeatures2<'r>(
    VkPhysicalDeviceFeatures2KHR,
    core::marker::PhantomData<Option<&'r mut dyn VulkanStructure>>,
);
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl<'r> PhysicalDeviceFeatures2<'r> {
    pub const fn new(old_features: VkPhysicalDeviceFeatures) -> Self {
        Self(
            VkPhysicalDeviceFeatures2KHR {
                sType: <VkPhysicalDeviceFeatures2KHR as TypedVulkanStructure>::TYPE,
                pNext: core::ptr::null_mut(),
                features: old_features,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'r mut (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic_mut() as *mut _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl VulkanStructure for PhysicalDeviceFeatures2<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        VulkanStructure::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        VulkanStructure::as_generic_mut(&mut self.0)
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl VulkanSinkStructure for PhysicalDeviceFeatures2<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanSinkStructure {
        VulkanSinkStructure::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanSinkStructure {
        VulkanSinkStructure::as_generic_mut(&mut self.0)
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PhysicalDeviceSynchronization2Features<'r>(
    VkPhysicalDeviceSynchronization2FeaturesKHR,
    core::marker::PhantomData<Option<&'r mut dyn VulkanStructure>>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'r> PhysicalDeviceSynchronization2Features<'r> {
    pub const fn new(enabled: bool) -> Self {
        Self(
            VkPhysicalDeviceSynchronization2FeaturesKHR {
                sType: <VkPhysicalDeviceSynchronization2FeaturesKHR as TypedVulkanStructure>::TYPE,
                pNext: core::ptr::null_mut(),
                synchronization2: enabled as _,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'r mut (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic_mut() as *mut _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
unsafe impl VulkanStructure for PhysicalDeviceSynchronization2Features<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        VulkanStructure::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        VulkanStructure::as_generic_mut(&mut self.0)
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
unsafe impl VulkanSinkStructure for PhysicalDeviceSynchronization2Features<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanSinkStructure {
        VulkanSinkStructure::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanSinkStructure {
        VulkanSinkStructure::as_generic_mut(&mut self.0)
    }
}

pub trait Device: VkHandle<Handle = VkDevice> + InstanceChild {
    /// Get a queue handle from a device
    #[implements]
    fn queue(&self, family_index: u32, queue_index: u32) -> QueueObject<&Self> {
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_device_queue(self.native_ptr(), family_index, queue_index, h.as_mut_ptr());
            QueueObject(h.assume_init(), self)
        }
    }

    /// Create a new fence object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_fence_raw(
        &self,
        info: &FenceCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkFence> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_fence(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new queue semaphore object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_semaphore_raw(
        &self,
        info: &SemaphoreCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSemaphore> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_semaphore(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new event object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_event_raw(
        &self,
        info: &EventCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkEvent> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_event(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Allocate device memory
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INVALID_EXTERNAL_HANDLE`]
    /// * [`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
    #[implements]
    #[inline]
    fn allocate_memory(
        &self,
        info: &MemoryAllocateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkDeviceMemory> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::allocate_memory(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new buffer object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_buffer_raw(
        &self,
        info: &BufferCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkBuffer> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_buffer(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new buffer view object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_buffer_view_raw(
        &self,
        info: &BufferViewCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkBufferView> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_buffer_view(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new sampler object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_TOO_MANY_OBJECTS`]
    #[implements]
    #[inline]
    fn new_sampler_raw(
        &self,
        info: &SamplerCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSampler> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_sampler(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new image object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_COMPRESSION_EXHAUSTED_EXT`]
    /// * [`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
    #[implements]
    #[inline]
    fn new_image_raw(
        &self,
        info: &ImageCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkImage> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_image(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new image view from an existing image
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
    #[implements]
    #[inline]
    fn new_image_view_raw(
        &self,
        info: &ImageViewCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkImageView> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_image_view(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new render pass object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_render_pass(
        &self,
        info: &RenderPassCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkRenderPass> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_render_pass(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new render pass object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements("Allow1_2APIs")]
    #[inline]
    fn new_render_pass2(
        &self,
        info: &RenderPassCreateInfo2,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkRenderPass> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_render_pass2(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new framebuffer object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_framebuffer_raw(
        &self,
        info: &FramebufferCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkFramebuffer> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_framebuffer(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Creates a new shader module object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INVALID_SHADER_NV`]
    #[implements]
    #[inline]
    fn new_shader_module_raw(
        &self,
        info: &ShaderModuleCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkShaderModule> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_shader_module(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new pipeline cache
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_pipeline_cache_raw(
        &self,
        info: &PipelineCacheCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkPipelineCache> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_pipeline_cache(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new pipeline layout object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_pipeline_layout_raw(
        &self,
        info: &PipelineLayoutCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkPipelineLayout> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_pipeline_layout(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create graphics pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    unsafe fn new_graphics_pipelines_raw(
        &self,
        infos: &[GraphicsPipelineCreateInfo],
        cache: Option<VkPipelineCache>,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
        objects: &mut [core::mem::MaybeUninit<VkPipeline>],
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::create_graphics_pipelines(
                self.native_ptr(),
                cache,
                infos.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(infos) as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                objects.as_mut_ptr() as _,
            )
            .into_result()
            .map(drop)
        }
    }

    /// Create graphics pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("alloc")]
    fn new_graphics_pipelines<'s>(
        &'s self,
        infos: &[GraphicsPipelineCreateInfo],
        cache: Option<&(impl crate::VkHandle<Handle = VkPipelineCache> + ?Sized)>,
    ) -> crate::Result<Vec<crate::PipelineObject<&'s Self>>> {
        let mut hs = vec![core::mem::MaybeUninit::uninit(); infos.len()];

        unsafe {
            self.new_graphics_pipelines_raw(infos, cache.map(VkHandle::native_ptr), None, &mut hs)?;
        }

        Ok(crate::alloc::collect_vec(hs.into_iter().map(move |h| unsafe {
            crate::PipelineObject::manage(h.assume_init(), self)
        })))
    }

    /// Create graphics pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn new_graphics_pipeline_array<'s, const N: usize>(
        &'s self,
        infos: &[GraphicsPipelineCreateInfo; N],
        cache: Option<&(impl crate::VkHandle<Handle = VkPipelineCache> + ?Sized)>,
    ) -> crate::Result<[crate::PipelineObject<&'s Self>; N]> {
        let mut hs = [core::mem::MaybeUninit::<VkPipeline>::uninit(); N];

        unsafe {
            self.new_graphics_pipelines_raw(infos, cache.map(VkHandle::native_ptr), None, &mut hs)?;
        }

        Ok(core::array::from_fn(move |n| unsafe {
            crate::PipelineObject::manage(hs[n].assume_init(), self)
        }))
    }

    /// Create compute pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    unsafe fn new_compute_pipelines_raw(
        &self,
        infos: &[ComputePipelineCreateInfo],
        cache: Option<VkPipelineCache>,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
        objects: &mut [core::mem::MaybeUninit<VkPipeline>],
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::create_compute_pipelines(
                self.native_ptr(),
                cache,
                infos.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(infos) as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                objects.as_mut_ptr() as _,
            )
            .into_result()
            .map(drop)
        }
    }

    /// Create compute pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("alloc")]
    fn new_compute_pipelines<'s>(
        &'s self,
        infos: &[ComputePipelineCreateInfo],
        cache: Option<&(impl crate::VkHandle<Handle = VkPipelineCache> + ?Sized)>,
    ) -> crate::Result<Vec<crate::PipelineObject<&'s Self>>> {
        let mut pipelines = vec![core::mem::MaybeUninit::uninit(); infos.len()];

        unsafe {
            self.new_compute_pipelines_raw(infos, cache.map(VkHandle::native_ptr), None, &mut pipelines)?;
        }

        Ok(crate::alloc::collect_vec(pipelines.into_iter().map(move |h| unsafe {
            crate::PipelineObject::manage(h.assume_init(), self)
        })))
    }

    /// Create compute pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn new_compute_pipeline_array<'s, const N: usize>(
        &'s self,
        infos: &[ComputePipelineCreateInfo; N],
        cache: Option<&(impl crate::VkHandle<Handle = VkPipelineCache> + ?Sized)>,
    ) -> crate::Result<[crate::PipelineObject<&'s Self>; N]> {
        let mut pipelines = [core::mem::MaybeUninit::uninit(); N];

        unsafe {
            self.new_compute_pipelines_raw(infos, cache.map(VkHandle::native_ptr), None, &mut pipelines)?;
        }

        Ok(core::array::from_fn(move |n| unsafe {
            crate::PipelineObject::manage(pipelines[n].assume_init(), self)
        }))
    }

    /// Create a new command pool object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_command_pool_raw(
        &self,
        info: &CommandPoolCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkCommandPool> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_command_pool(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new query pool object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_query_pool_raw(
        &self,
        info: &QueryPoolCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkQueryPool> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_query_pool(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Allocate command buffers from an existing command pool
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    ///
    /// # Safety
    /// A `VkCommandPool` specified in the `info` argument must be created from this device
    #[implements]
    #[inline]
    unsafe fn allocate_command_buffers<'s>(
        &'s self,
        info: &CommandBufferAllocateInfo,
        sink: &mut [core::mem::MaybeUninit<CommandBufferObject<&'s Self>>],
    ) -> crate::Result<()> {
        assert_eq!(info.0.commandBufferCount as usize, sink.len());

        unsafe {
            crate::vkfn::allocate_command_buffers(self.native_ptr(), info as *const _ as _, sink.as_mut_ptr() as _)
                .into_result()
                .map(drop)
        }
    }

    /// Allocate command buffers from an existing command pool
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    ///
    /// # Safety
    /// A `VkCommandPool` specified in the `info` argument must be created from this device
    #[implements("alloc")]
    unsafe fn allocate_command_buffers_alloc<'s>(
        &'s self,
        info: &CommandBufferAllocateInfo,
    ) -> crate::Result<Vec<CommandBufferObject<&'s Self>>> {
        let mut sink = Vec::with_capacity(info.0.commandBufferCount as _);
        unsafe {
            self.allocate_command_buffers(info, sink.spare_capacity_mut())?;
            sink.set_len(info.0.commandBufferCount as _);
        }

        Ok(sink)
    }

    /// Allocate command buffers from an existing command pool
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    ///
    /// # Safety
    /// A `VkCommandPool` specified in the `info` argument must be created from this device
    #[implements]
    #[inline]
    unsafe fn allocate_command_buffer_array<'s, const N: usize>(
        &'s self,
        info: &CommandBufferFixedCountAllocateInfo<N>,
    ) -> crate::Result<[CommandBufferObject<&'s Self>; N]> {
        let mut sink = [core::mem::MaybeUninit::<CommandBufferObject<&'s Self>>::uninit(); N];
        unsafe {
            crate::vkfn::allocate_command_buffers(self.native_ptr(), info as *const _ as _, sink.as_mut_ptr() as _)
                .into_result()?;
        }

        Ok(core::array::from_fn(|n| unsafe { sink[n].assume_init() }))
    }

    /// Invalidate `MappedMemoryRange`s
    /// Invalidating the memory range allows that device writes to the memory ranges
    /// which have been made visible to the `VK_ACCESS_HOST_WRITE_BIT` and `VK_ACCESS_HOST_READ_BIT`
    /// are made visible to the host
    /// # Safety
    /// Memory object in `ranges` must be currently host mapped
    #[implements]
    #[inline]
    unsafe fn invalidate_memory_range(&self, ranges: &[MappedMemoryRange]) -> crate::Result<()> {
        unsafe {
            crate::vkfn::invalidate_mapped_memory_ranges(
                self.native_ptr(),
                ranges.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(ranges) as _,
            )
            .into_result()
            .map(drop)
        }
    }

    /// Flush `MappedMemoryRange`s
    /// Flushing the memory range allows that host writes to the memory ranges can
    /// be made available to device access
    /// # Safety
    /// Memory object in `ranges` must be currently host mapped
    #[implements]
    #[inline]
    unsafe fn flush_mapped_memory_ranges(&self, ranges: &[MappedMemoryRange]) -> crate::Result<()> {
        unsafe {
            crate::vkfn::flush_mapped_memory_ranges(
                self.native_ptr(),
                ranges.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(ranges) as _,
            )
            .into_result()
            .map(drop)
        }
    }

    /// Update the contents of descriptor set objects
    ///
    /// # Safety
    ///
    /// `writes` and `copies` must be a valid array of [`VkWriteDescriptorSet`] and [`VkCopyDescriptorSet`] structs, respectively.
    #[implements]
    #[inline]
    unsafe fn update_descriptor_sets_raw(&self, writes: &[VkWriteDescriptorSet], copies: &[VkCopyDescriptorSet]) {
        unsafe {
            crate::vkfn::update_descriptor_sets(
                self.native_ptr(),
                writes.len() as _,
                slice_as_ptr_empty_null(writes),
                copies.len() as _,
                slice_as_ptr_empty_null(copies),
            )
        }
    }

    /// Update the contents of descriptor set objects
    #[implements("alloc")]
    fn update_descriptor_sets(&self, writes: &[DescriptorSetWriteInfo], copies: &[DescriptorSetCopyInfo]) {
        unsafe {
            self.update_descriptor_sets_raw(
                &crate::alloc::collect_vec(writes.iter().map(DescriptorSetWriteInfo::make_structure)),
                &crate::alloc::collect_vec(copies.iter().map(DescriptorSetCopyInfo::make_structure)),
            );
        }
    }

    /// Wait for a object to become idle
    /// # Safety
    /// All VkQueue objects created from this device must be externally synchronized.
    #[implements]
    #[inline]
    unsafe fn wait(&self) -> crate::Result<()> {
        unsafe { crate::vkfn::device_wait_idle(self.native_ptr()).into_result().map(drop) }
    }

    /// Query the memory requirements for a sparse image
    /// # Safety
    /// `sink_head_ptr` must be a valid pointer for read/write operations.
    #[implements("Allow1_1APIs")]
    #[inline]
    unsafe fn get_image_sparse_memory_requirements2_count(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2,
        count_sink: &mut core::mem::MaybeUninit<u32>,
        sink_head_ptr: *mut VkSparseImageMemoryRequirements2KHR,
    ) {
        unsafe {
            crate::vkfn::get_image_sparse_memory_requirements2(
                self.native_ptr(),
                &info.0,
                count_sink.as_mut_ptr(),
                sink_head_ptr,
            )
        }
    }

    /// Single binding for a buffer
    /// # Safety
    /// `VkBuffer` and `VkDeviceMemory` must be valid and created from this device object
    #[implements]
    #[inline(always)]
    unsafe fn bind_buffer_raw(
        &self,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
    ) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::bind_buffer_memory(self.native_ptr(), buffer, memory, offset) }
    }

    /// Multiple Binding for Buffers
    #[implements("Allow1_1APIs")]
    #[inline(always)]
    fn bind_buffers(&self, bounds: &[BindBufferMemoryInfo]) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::bind_buffer_memory2(self.native_ptr(), bounds) }
    }

    /// Single binding for an image
    /// # Safety
    /// `VkImage` and `VkDeviceMemory` must be valid and created from this device object
    #[implements]
    #[inline(always)]
    unsafe fn bind_image_raw(&self, image: VkImage, memory: VkDeviceMemory, offset: VkDeviceSize) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::bind_image_memory(self.native_ptr(), image, memory, offset) }
    }

    /// Multiple Binding for Images
    #[implements("Allow1_1APIs")]
    #[inline(always)]
    fn bind_images(&self, bounds: &[BindImageMemoryInfo]) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::bind_image_memory2(self.native_ptr(), bounds) }
    }

    /// Wait for one or more fences to become signaled, returns `Ok(true)` if operation is timed out
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[implements]
    fn wait_multiple_fences(
        &self,
        objects: &[VkHandleRef<VkFence>],
        wait_all: bool,
        timeout: Option<u64>,
    ) -> crate::Result<bool> {
        let vr = unsafe {
            crate::vkfn::wait_for_fences(
                self.native_ptr(),
                objects.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(objects) as _,
                wait_all as _,
                timeout.unwrap_or(u64::MAX),
            )
        };

        match vr {
            VK_SUCCESS => Ok(false),
            VK_TIMEOUT => Ok(true),
            _ => Err(vr),
        }
    }

    /// Resets one or more fence objects
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline]
    fn reset_multiple_fences(&self, objects: &[VkHandleRefMut<VkFence>]) -> crate::Result<()> {
        unsafe {
            crate::vkfn::reset_fences(
                self.native_ptr(),
                objects.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(objects) as _,
            )
            .into_result()
            .map(drop)
        }
    }

    /// Create a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    /// * [`VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`]
    /// * [`VK_ERROR_INITIALIZATION_FAILED`]
    /// * [`VK_ERROR_COMPRESSION_EXHAUSTED_EXT`]
    #[implements("VK_KHR_swapchain")]
    #[inline]
    fn new_swapchain_raw(
        &self,
        info: &SwapchainCreateInfo,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkSwapchainKHR> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_swapchain_khr(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Give a user-friendly name to an object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_EXT_debug_utils")]
    fn set_object_name(&self, info: &crate::DebugUtilsObjectNameInfo) -> crate::Result<()>
    where
        Self::ConcreteInstance: InstanceDebugUtilsExtension,
    {
        unsafe {
            self.instance().set_debug_utils_object_name_ext_fn().0(self.native_ptr(), &info.0)
                .into_result()
                .map(drop)
        }
    }

    /// Create a new descriptor update template
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements("Allow1_1APIs")]
    unsafe fn new_descriptor_update_template_raw(
        &self,
        info: &VkDescriptorUpdateTemplateCreateInfoKHR,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkDescriptorUpdateTemplateKHR> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_descriptor_update_template(
                self.native_ptr(),
                info,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }
        Ok(unsafe { h.assume_init() })
    }

    /// Query the current state of a timeline semaphore
    /// # Failure
    ///
    /// * [`VK_ERROR_DEVICE_LOST`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_UNKNOWN`]
    /// * [`VK_ERROR_VALIDATION_FAILED`]
    #[implements("Allow1_2APIs")]
    fn get_semaphore_counter_value(
        &self,
        semaphore: &(impl VkHandle<Handle = VkSemaphore> + ?Sized),
    ) -> crate::Result<u64> {
        let mut sink = core::mem::MaybeUninit::uninit();

        unsafe {
            crate::vkfn::get_semaphore_counter_value(self.native_ptr(), semaphore.native_ptr(), sink.as_mut_ptr())
                .into_result()?;
        }

        Ok(unsafe { sink.assume_init() })
    }

    /// Signal a timeline semaphore on the host
    /// # Failure
    ///
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_UNKNOWN`]
    /// * [`VK_ERROR_VALIDATION_FAILED`]
    #[implements("Allow1_2APIs")]
    fn signal_semaphore(&self, info: &SemaphoreSignalInfo) -> crate::Result<()> {
        unsafe {
            crate::vkfn::signal_semaphore(self.native_ptr(), info as *const _ as _)
                .into_result()
                .map(drop)
        }
    }

    /// Wait for timeline semaphores on the host
    ///
    /// Returns `false` if timed out
    /// # Failure
    ///
    /// * [`VK_ERROR_DEVICE_LOST`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_UNKNOWN`]
    /// * [`VK_ERROR_VALIDATION_FAILED`]
    #[implements("Allow1_2APIs")]
    fn wait_semaphores(&self, info: &SemaphoreWaitInfo, timeout: u64) -> crate::Result<bool> {
        match unsafe { crate::vkfn::wait_semaphores(self.native_ptr(), info as *const _ as _, timeout) } {
            r if r == VK_SUCCESS => Ok(true),
            r if r == VK_TIMEOUT => Ok(false),
            r => Err(r),
        }
    }
}
DerefContainerWithGuardsBracketImpl!(for Device {});

/// Extension function caches
#[implements]
struct DeviceExtFunctions {
    #[cfg(feature = "VK_KHR_maintenance1")]
    trim_command_pool_khr: DeviceResolvedFn<PFN_vkTrimCommandPoolKHR>,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    create_descriptor_update_template_khr: DeviceResolvedFn<PFN_vkCreateDescriptorUpdateTemplateKHR>,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    destroy_descriptor_update_template_khr: DeviceResolvedFn<PFN_vkDestroyDescriptorUpdateTemplateKHR>,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    update_descriptor_set_with_template_khr: DeviceResolvedFn<PFN_vkUpdateDescriptorSetWithTemplateKHR>,
    #[cfg(feature = "VK_KHR_bind_memory2")]
    bind_buffer_memory2_khr: DeviceResolvedFn<PFN_vkBindBufferMemory2KHR>,
    #[cfg(feature = "VK_KHR_bind_memory2")]
    bind_image_memory2_khr: DeviceResolvedFn<PFN_vkBindImageMemory2KHR>,
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    get_image_drm_format_modifier_properties_ext: DeviceResolvedFn<PFN_vkGetImageDrmFormatModifierPropertiesEXT>,
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    get_fence_fd_khr: DeviceResolvedFn<PFN_vkGetFenceFdKHR>,
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    import_fence_fd_khr: DeviceResolvedFn<PFN_vkImportFenceFdKHR>,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    acquire_full_screen_exclusive_mode_ext: DeviceResolvedFn<PFN_vkAcquireFullScreenExclusiveModeEXT>,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    release_full_screen_exclusive_mode_ext: DeviceResolvedFn<PFN_vkReleaseFullScreenExclusiveModeEXT>,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    get_memory_fd_khr: DeviceResolvedFn<PFN_vkGetMemoryFdKHR>,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    get_memory_fd_properties_khr: DeviceResolvedFn<PFN_vkGetMemoryFdPropertiesKHR>,
    #[cfg(feature = "VK_EXT_external_memory_host")]
    get_memory_host_pointer_properties_ext: DeviceResolvedFn<PFN_vkGetMemoryHostPointerPropertiesEXT>,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    import_semaphore_win32_handle_khr: DeviceResolvedFn<PFN_vkImportSemaphoreWin32HandleKHR>,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    get_semaphore_win32_handle_khr: DeviceResolvedFn<PFN_vkGetSemaphoreWin32HandleKHR>,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    get_memory_win32_handle_khr: DeviceResolvedFn<PFN_vkGetMemoryWin32HandleKHR>,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    get_memory_win32_handle_properties_khr: DeviceResolvedFn<PFN_vkGetMemoryWin32HandlePropertiesKHR>,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    get_buffer_memory_requirements_2_khr: DeviceResolvedFn<PFN_vkGetBufferMemoryRequirements2KHR>,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    get_image_memory_requirements_2_khr: DeviceResolvedFn<PFN_vkGetImageMemoryRequirements2KHR>,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    get_image_sparse_memory_requirements_2_khr: DeviceResolvedFn<PFN_vkGetImageSparseMemoryRequirements2KHR>,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    create_render_pass_2_khr: DeviceResolvedFn<PFN_vkCreateRenderPass2KHR>,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    cmd_begin_render_pass_2_khr: DeviceResolvedFn<PFN_vkCmdBeginRenderPass2KHR>,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    cmd_end_render_pass_2_khr: DeviceResolvedFn<PFN_vkCmdEndRenderPass2KHR>,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    cmd_next_subpass_2_khr: DeviceResolvedFn<PFN_vkCmdNextSubpass2KHR>,
    #[cfg(feature = "VK_KHR_synchronization2")]
    queue_submit2_khr: DeviceResolvedFn<PFN_vkQueueSubmit2KHR>,
    #[cfg(feature = "VK_KHR_synchronization2")]
    cmd_pipeline_barrier_2_khr: DeviceResolvedFn<PFN_vkCmdPipelineBarrier2KHR>,
    #[cfg(feature = "VK_KHR_push_descriptor")]
    cmd_push_descriptor_set_khr: DeviceResolvedFn<PFN_vkCmdPushDescriptorSetKHR>,
    #[cfg(feature = "VK_EXT_sample_locations")]
    cmd_set_sample_locations_ext: DeviceResolvedFn<PFN_vkCmdSetSampleLocationsEXT>,
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    get_semaphore_counter_value_ext: DeviceResolvedFn<PFN_vkGetSemaphoreCounterValueKHR>,
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    signal_semaphore_ext: DeviceResolvedFn<PFN_vkSignalSemaphoreKHR>,
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    wait_semaphores_ext: DeviceResolvedFn<PFN_vkWaitSemaphoresKHR>,
}
#[implements]
impl DeviceExtFunctions {
    const fn new(handle: VkDevice) -> Self {
        Self {
            #[cfg(feature = "VK_KHR_maintenance1")]
            trim_command_pool_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_descriptor_update_template")]
            create_descriptor_update_template_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_descriptor_update_template")]
            destroy_descriptor_update_template_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_descriptor_update_template")]
            update_descriptor_set_with_template_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_bind_memory2")]
            bind_buffer_memory2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_bind_memory2")]
            bind_image_memory2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
            get_image_drm_format_modifier_properties_ext: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_external_fence_fd")]
            get_fence_fd_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_external_fence_fd")]
            import_fence_fd_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            acquire_full_screen_exclusive_mode_ext: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            release_full_screen_exclusive_mode_ext: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_external_memory_fd")]
            get_memory_fd_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_external_memory_fd")]
            get_memory_fd_properties_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_EXT_external_memory_host")]
            get_memory_host_pointer_properties_ext: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_external_semaphore_win32")]
            import_semaphore_win32_handle_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_external_semaphore_win32")]
            get_semaphore_win32_handle_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_external_memory_win32")]
            get_memory_win32_handle_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_external_memory_win32")]
            get_memory_win32_handle_properties_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_get_memory_requirements2")]
            get_buffer_memory_requirements_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_get_memory_requirements2")]
            get_image_memory_requirements_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_get_memory_requirements2")]
            get_image_sparse_memory_requirements_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_create_renderpass2")]
            create_render_pass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_create_renderpass2")]
            cmd_begin_render_pass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_create_renderpass2")]
            cmd_end_render_pass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_create_renderpass2")]
            cmd_next_subpass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_synchronization2")]
            queue_submit2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_synchronization2")]
            cmd_pipeline_barrier_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_push_descriptor")]
            cmd_push_descriptor_set_khr: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_EXT_sample_locations")]
            cmd_set_sample_locations_ext: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_timeline_semaphore")]
            get_semaphore_counter_value_ext: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_timeline_semaphore")]
            signal_semaphore_ext: DeviceResolvedFn::new(handle),
            #[cfg(feature = "VK_KHR_timeline_semaphore")]
            wait_semaphores_ext: DeviceResolvedFn::new(handle),
        }
    }
}

#[cfg(feature = "VK_KHR_maintenance1")]
pub trait DeviceMaintenance1Extension: Device {
    #[implements]
    fn trim_command_pool_khr_fn(&self) -> PFN_vkTrimCommandPoolKHR;

    /// Trim a command pool.
    #[implements]
    #[inline(always)]
    fn trim_command_pool_khr(
        &self,
        command_pool: &mut (impl VkHandleMut<Handle = VkCommandPool> + ?Sized),
        flags: CommandPoolTrimFlags,
    ) {
        unsafe { self.trim_command_pool_khr_fn().0(self.native_ptr(), command_pool.native_ptr_mut(), flags.bits()) }
    }
}
#[cfg(feature = "VK_KHR_maintenance1")]
DerefContainerWithGuardsBracketImpl!(for DeviceMaintenance1Extension {
    #[implements]
    ForwardFnPtr!(deref trim_command_pool_khr_fn -> PFN_vkTrimCommandPoolKHR);
});
#[cfg(feature = "VK_KHR_maintenance1")]
impl<Instance: crate::Instance> DeviceMaintenance1Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn trim_command_pool_khr_fn(&self) -> PFN_vkTrimCommandPoolKHR {
        *self.ext.trim_command_pool_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub trait DeviceDescriptorUpdateTemplateExtension: Device {
    #[implements]
    fn create_descriptor_update_template_khr_fn(&self) -> PFN_vkCreateDescriptorUpdateTemplateKHR;
    #[implements]
    fn destroy_descriptor_update_template_khr_fn(&self) -> PFN_vkDestroyDescriptorUpdateTemplateKHR;
    #[implements]
    fn update_descriptor_set_with_template_khr_fn(&self) -> PFN_vkUpdateDescriptorSetWithTemplateKHR;

    /// Create a new descriptor update template
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    #[inline]
    unsafe fn new_descriptor_update_template_raw_khr(
        &self,
        create_info: &VkDescriptorUpdateTemplateCreateInfoKHR,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkDescriptorUpdateTemplateKHR> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            self.create_descriptor_update_template_khr_fn().0(
                self.native_ptr(),
                create_info,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }
        Ok(unsafe { h.assume_init() })
    }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
DerefContainerWithGuardsBracketImpl!(for DeviceDescriptorUpdateTemplateExtension {
    #[implements]
    ForwardFnPtr!(deref create_descriptor_update_template_khr_fn -> PFN_vkCreateDescriptorUpdateTemplateKHR);
    #[implements]
    ForwardFnPtr!(deref destroy_descriptor_update_template_khr_fn -> PFN_vkDestroyDescriptorUpdateTemplateKHR);
    #[implements]
    ForwardFnPtr!(deref update_descriptor_set_with_template_khr_fn -> PFN_vkUpdateDescriptorSetWithTemplateKHR);
});
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl<Instance: crate::Instance> DeviceDescriptorUpdateTemplateExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn create_descriptor_update_template_khr_fn(&self) -> PFN_vkCreateDescriptorUpdateTemplateKHR {
        *self.ext.create_descriptor_update_template_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn destroy_descriptor_update_template_khr_fn(&self) -> PFN_vkDestroyDescriptorUpdateTemplateKHR {
        *self.ext.destroy_descriptor_update_template_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn update_descriptor_set_with_template_khr_fn(&self) -> PFN_vkUpdateDescriptorSetWithTemplateKHR {
        *self.ext.update_descriptor_set_with_template_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub trait DeviceGetMemoryRequirements2Extension: Device {
    #[implements]
    fn get_buffer_memory_requirements_2_khr_fn(&self) -> PFN_vkGetBufferMemoryRequirements2KHR;
    #[implements]
    fn get_image_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageMemoryRequirements2KHR;
    #[implements]
    fn get_image_sparse_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageSparseMemoryRequirements2KHR;

    /// Returns the memory requirements for specified Vulkan object.
    #[implements]
    #[inline]
    fn get_buffer_memory_requirements2_khr(
        &self,
        info: &crate::BufferMemoryRequirementsInfo2<'_, impl crate::VkHandle<Handle = VkBuffer>>,
        sink: &mut core::mem::MaybeUninit<VkMemoryRequirements2KHR>,
    ) {
        unsafe {
            self.get_buffer_memory_requirements_2_khr_fn().0(
                self.native_ptr(),
                core::ptr::from_ref(info).cast(),
                sink.as_mut_ptr(),
            )
        }
    }

    /// Returns the memory requirements for specified Vulkan object.
    #[implements]
    #[inline]
    fn get_image_memory_requirements2_khr(
        &self,
        info: &crate::ImageMemoryRequirementsInfo2<'_, impl crate::VkHandle<Handle = VkImage>>,
        sink: &mut core::mem::MaybeUninit<VkMemoryRequirements2KHR>,
    ) {
        unsafe {
            self.get_image_memory_requirements_2_khr_fn().0(
                self.native_ptr(),
                core::ptr::from_ref(info).cast(),
                sink.as_mut_ptr(),
            )
        }
    }

    /// Query a number of memory requirements for a sparse image.
    #[implements]
    fn get_image_sparse_memory_requirements2_count_khr(&self, info: &ImageSparseMemoryRequirementsInfo2) -> u32 {
        let mut sink = core::mem::MaybeUninit::uninit();
        unsafe {
            self.get_image_sparse_memory_requirements_2_khr_fn().0(
                self.native_ptr(),
                core::ptr::from_ref(info).cast(),
                sink.as_mut_ptr(),
                core::ptr::null_mut(),
            );

            sink.assume_init()
        }
    }

    /// Query the memory requirements for a sparse image.
    ///
    /// # Returns
    ///
    /// The number of sparse image memory requirements queried.
    #[implements]
    fn get_image_sparse_memory_requirements2_khr(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2,
        sink: &mut [core::mem::MaybeUninit<VkSparseImageMemoryRequirements2KHR>],
    ) -> u32 {
        let mut count = sink.len() as u32;
        unsafe {
            self.get_image_sparse_memory_requirements_2_khr_fn().0(
                self.native_ptr(),
                core::ptr::from_ref(info).cast(),
                &mut count,
                sink.as_mut_ptr().cast(),
            );

            count
        }
    }

    /// Query the memory requirements for a sparse image.
    #[implements("alloc")]
    fn get_image_sparse_memory_requirements2_khr_alloc(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2,
    ) -> Vec<VkSparseImageMemoryRequirements2KHR> {
        let n = self.get_image_sparse_memory_requirements2_count_khr(info);
        if n == 0 {
            return crate::alloc::empty_sink_buffer();
        }

        let mut buf = crate::alloc::empty_reserved_buffer(n as _);
        let written = self.get_image_sparse_memory_requirements2_khr(info, buf.spare_capacity_mut());
        unsafe {
            buf.set_len(written as _);
        }
        buf
    }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
DerefContainerWithGuardsBracketImpl!(for DeviceGetMemoryRequirements2Extension {
    #[implements]
    ForwardFnPtr!(deref get_buffer_memory_requirements_2_khr_fn -> PFN_vkGetBufferMemoryRequirements2KHR);
    #[implements]
    ForwardFnPtr!(deref get_image_memory_requirements_2_khr_fn -> PFN_vkGetImageMemoryRequirements2KHR);
    #[implements]
    ForwardFnPtr!(deref get_image_sparse_memory_requirements_2_khr_fn -> PFN_vkGetImageSparseMemoryRequirements2KHR);
});
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<Instance: crate::Instance> DeviceGetMemoryRequirements2Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_buffer_memory_requirements_2_khr_fn(&self) -> PFN_vkGetBufferMemoryRequirements2KHR {
        *self.ext.get_buffer_memory_requirements_2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn get_image_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageMemoryRequirements2KHR {
        *self.ext.get_image_memory_requirements_2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn get_image_sparse_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageSparseMemoryRequirements2KHR {
        *self.ext.get_image_sparse_memory_requirements_2_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_bind_memory2")]
pub trait DeviceBindMemory2Extension: Device {
    #[implements]
    fn bind_buffer_memory2_khr_fn(&self) -> PFN_vkBindBufferMemory2KHR;
    #[implements]
    fn bind_image_memory2_khr_fn(&self) -> PFN_vkBindImageMemory2KHR;

    /// Multiple Binding for Buffers
    #[implements]
    #[inline]
    fn bind_buffer_memory2_khr(&self, bounds: &[BindBufferMemoryInfo]) -> crate::Result<()> {
        unsafe {
            self.bind_buffer_memory2_khr_fn().0(
                self.native_ptr(),
                bounds.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(bounds).cast(),
            )
            .into_result()
            .map(drop)
        }
    }

    /// Multiple Binding for Images
    #[implements]
    #[inline]
    fn bind_image_memory2_khr(&self, bounds: &[BindImageMemoryInfo]) -> crate::Result<()> {
        unsafe {
            self.bind_image_memory2_khr_fn().0(
                self.native_ptr(),
                bounds.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(bounds).cast(),
            )
            .into_result()
            .map(drop)
        }
    }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
DerefContainerWithGuardsBracketImpl!(for DeviceBindMemory2Extension {
    #[implements]
    ForwardFnPtr!(deref bind_buffer_memory2_khr_fn -> PFN_vkBindBufferMemory2KHR);
    #[implements]
    ForwardFnPtr!(deref bind_image_memory2_khr_fn -> PFN_vkBindImageMemory2KHR);
});
#[cfg(feature = "VK_KHR_bind_memory2")]
impl<Instance: crate::Instance> DeviceBindMemory2Extension for DeviceObject<Instance> {
    #[implements]
    fn bind_buffer_memory2_khr_fn(&self) -> PFN_vkBindBufferMemory2KHR {
        *self.ext.bind_buffer_memory2_khr.resolve()
    }
    #[implements]
    fn bind_image_memory2_khr_fn(&self) -> PFN_vkBindImageMemory2KHR {
        *self.ext.bind_image_memory2_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_external_fence_fd")]
pub trait DeviceExternalFenceFdExtension: Device {
    #[implements]
    fn get_fence_fd_khr_fn(&self) -> PFN_vkGetFenceFdKHR;
    #[implements]
    fn import_fence_fd_khr_fn(&self) -> PFN_vkImportFenceFdKHR;

    /// Get a POSIX file descriptor handle for a type
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements]
    #[inline]
    fn get_fence_fd(&self, info: &crate::FenceFdGetInfo) -> crate::Result<std::os::unix::io::RawFd> {
        let mut fd = core::mem::MaybeUninit::uninit();

        unsafe {
            self.get_fence_fd_khr_fn().0(self.native_ptr(), &info.0, fd.as_mut_ptr()).into_result()?;

            Ok(fd.assume_init())
        }
    }

    /// Import a fence from a POSIX file descriptor
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements]
    #[inline]
    fn import_fence_fd(&self, info: &crate::ImportFenceFdInfo) -> crate::Result<()> {
        unsafe {
            self.import_fence_fd_khr_fn().0(self.native_ptr(), &info.0)
                .into_result()
                .map(drop)
        }
    }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
DerefContainerBracketImpl!(for DeviceExternalFenceFdExtension {
    #[implements]
    ForwardFnPtr!(deref get_fence_fd_khr_fn -> PFN_vkGetFenceFdKHR);
    #[implements]
    ForwardFnPtr!(deref import_fence_fd_khr_fn -> PFN_vkImportFenceFdKHR);
});
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl<Instance: crate::Instance> DeviceExternalFenceFdExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_fence_fd_khr_fn(&self) -> PFN_vkGetFenceFdKHR {
        *self.ext.get_fence_fd_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn import_fence_fd_khr_fn(&self) -> PFN_vkImportFenceFdKHR {
        *self.ext.import_fence_fd_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub trait DeviceExternalSemaphoreWin32Extension: Device {
    #[implements]
    fn import_semaphore_win32_handle_khr_fn(&self) -> PFN_vkImportSemaphoreWin32HandleKHR;
    #[implements]
    fn get_semaphore_win32_handle_khr_fn(&self) -> PFN_vkGetSemaphoreWin32HandleKHR;

    /// Import a semaphore from a Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_INVALID_EXTERNAL_HANDLE
    #[implements]
    #[inline]
    fn import_semaphore_win32_handle(&self, info: &crate::ImportSemaphoreWin32HandleInfo) -> crate::Result<()> {
        unsafe {
            self.import_semaphore_win32_handle_khr_fn().0(self.native_ptr(), &info.0)
                .into_result()
                .map(drop)
        }
    }

    /// Get a Windows HANDLE for a semaphore
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_TOO_MANY_OBJECTS
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    #[implements]
    #[inline]
    fn get_semaphore_win32_handle(
        &self,
        info: &crate::SemaphoreGetWin32HandleInfo,
    ) -> crate::Result<windows::Win32::Foundation::HANDLE> {
        let mut handle = core::mem::MaybeUninit::uninit();

        unsafe {
            self.get_semaphore_win32_handle_khr_fn().0(self.native_ptr(), &info.0, handle.as_mut_ptr())
                .into_result()?;

            Ok(handle.assume_init())
        }
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
DerefContainerWithGuardsBracketImpl!(for DeviceExternalSemaphoreWin32Extension {
    #[implements]
    ForwardFnPtr!(deref import_semaphore_win32_handle_khr_fn -> PFN_vkImportSemaphoreWin32HandleKHR);
    #[implements]
    ForwardFnPtr!(deref get_semaphore_win32_handle_khr_fn -> PFN_vkGetSemaphoreWin32HandleKHR);
});
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<Instance: crate::Instance> DeviceExternalSemaphoreWin32Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn import_semaphore_win32_handle_khr_fn(&self) -> PFN_vkImportSemaphoreWin32HandleKHR {
        *self.ext.import_semaphore_win32_handle_khr.resolve()
    }

    #[implements]
    #[inline(always)]
    fn get_semaphore_win32_handle_khr_fn(&self) -> PFN_vkGetSemaphoreWin32HandleKHR {
        *self.ext.get_semaphore_win32_handle_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_external_memory_fd")]
pub trait DeviceExternalMemoryFdExtension: Device {
    #[implements]
    fn get_memory_fd_khr_fn(&self) -> PFN_vkGetMemoryFdKHR;
    #[implements]
    fn get_memory_fd_properties_khr_fn(&self) -> PFN_vkGetMemoryFdPropertiesKHR;

    /// Get a POSIX file descriptor for a memory object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements]
    #[inline]
    fn get_memory_fd(&self, info: &crate::MemoryGetFdInfo) -> crate::Result<std::os::unix::io::RawFd> {
        let mut fd = core::mem::MaybeUninit::uninit();

        unsafe {
            self.get_memory_fd_khr_fn().0(self.native_ptr(), &info.0, fd.as_mut_ptr()).into_result()?;

            Ok(fd.assume_init())
        }
    }

    /// Get Properties of External Memory File Descriptors
    /// # Safety
    /// sink must be constructed correctly
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements]
    #[inline]
    unsafe fn memory_fd_properties(
        &self,
        handle_type: crate::ExternalMemoryHandleTypeFd,
        handle: std::os::unix::io::RawFd,
        sink: &mut core::mem::MaybeUninit<VkMemoryFdPropertiesKHR>,
    ) -> crate::Result<()> {
        unsafe {
            self.get_memory_fd_properties_khr_fn().0(self.native_ptr(), handle_type as _, handle, sink.as_mut_ptr())
                .into_result()
                .map(drop)
        }
    }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
DerefContainerWithGuardsBracketImpl!(for DeviceExternalMemoryFdExtension {
    #[implements]
    ForwardFnPtr!(deref get_memory_fd_khr_fn -> PFN_vkGetMemoryFdKHR);
    #[implements]
    ForwardFnPtr!(deref get_memory_fd_properties_khr_fn -> PFN_vkGetMemoryFdPropertiesKHR);
});
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl<Instance: crate::Instance> DeviceExternalMemoryFdExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_memory_fd_khr_fn(&self) -> PFN_vkGetMemoryFdKHR {
        *self.ext.get_memory_fd_khr.resolve()
    }

    #[implements]
    #[inline(always)]
    fn get_memory_fd_properties_khr_fn(&self) -> PFN_vkGetMemoryFdPropertiesKHR {
        *self.ext.get_memory_fd_properties_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_external_memory_host")]
pub trait DeviceExternalMemoryHostExtension: Device {
    #[implements]
    fn get_memory_host_pointer_properties_ext_fn(&self) -> PFN_vkGetMemoryHostPointerPropertiesEXT;

    /// Get Properties of external memory host pointer
    /// # Safety
    /// sink must be constructed correctly
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements]
    #[inline]
    unsafe fn memory_host_pointer_properties(
        &self,
        handle_type: crate::ExternalMemoryHandleTypeHost,
        ptr: *mut core::ffi::c_void,
        sink: &mut core::mem::MaybeUninit<VkMemoryHostPointerPropertiesEXT>,
    ) -> crate::Result<()> {
        unsafe {
            self.get_memory_host_pointer_properties_ext_fn().0(
                self.native_ptr(),
                handle_type as _,
                ptr,
                sink.as_mut_ptr(),
            )
            .into_result()
            .map(drop)
        }
    }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
DerefContainerWithGuardsBracketImpl!(for DeviceExternalMemoryHostExtension {
    #[implements]
    ForwardFnPtr!(deref get_memory_host_pointer_properties_ext_fn -> PFN_vkGetMemoryHostPointerPropertiesEXT);
});
#[cfg(feature = "VK_EXT_external_memory_host")]
impl<Instance: crate::Instance> DeviceExternalMemoryHostExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_memory_host_pointer_properties_ext_fn(&self) -> PFN_vkGetMemoryHostPointerPropertiesEXT {
        *self.ext.get_memory_host_pointer_properties_ext.resolve()
    }
}

#[cfg(feature = "VK_KHR_external_memory_win32")]
impl<Instance: crate::Instance> DeviceExternalMemoryWin32Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_memory_win32_handle_khr_fn(&self) -> PFN_vkGetMemoryWin32HandleKHR {
        *self.ext.get_memory_win32_handle_khr.resolve()
    }

    #[implements]
    #[inline(always)]
    fn get_memory_win32_handle_properties_khr_fn(&self) -> PFN_vkGetMemoryWin32HandlePropertiesKHR {
        *self.ext.get_memory_win32_handle_properties_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub trait DeviceFullScreenExclusiveExtension: Device {
    #[implements]
    fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkAcquireFullScreenExclusiveModeEXT;
    #[implements]
    fn release_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkReleaseFullScreenExclusiveModeEXT;
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
DerefContainerWithGuardsBracketImpl!(for DeviceFullScreenExclusiveExtension {
    #[implements]
    ForwardFnPtr!(deref acquire_full_screen_exclusive_mode_ext_fn -> PFN_vkAcquireFullScreenExclusiveModeEXT);
    #[implements]
    ForwardFnPtr!(deref release_full_screen_exclusive_mode_ext_fn -> PFN_vkReleaseFullScreenExclusiveModeEXT);
});
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl<Instance: crate::Instance> DeviceFullScreenExclusiveExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkAcquireFullScreenExclusiveModeEXT {
        *self.ext.acquire_full_screen_exclusive_mode_ext.resolve()
    }

    #[implements]
    #[inline(always)]
    fn release_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkReleaseFullScreenExclusiveModeEXT {
        *self.ext.release_full_screen_exclusive_mode_ext.resolve()
    }
}

#[cfg(feature = "VK_KHR_create_renderpass2")]
pub trait DeviceCreateRenderPass2Extension: Device {
    #[implements]
    fn create_render_pass_2_khr_fn(&self) -> PFN_vkCreateRenderPass2KHR;
    #[implements]
    fn cmd_begin_render_pass_2_khr_fn(&self) -> PFN_vkCmdBeginRenderPass2KHR;
    #[implements]
    fn cmd_end_render_pass_2_khr_fn(&self) -> PFN_vkCmdEndRenderPass2KHR;
    #[implements]
    fn cmd_next_subpass_2_khr_fn(&self) -> PFN_vkCmdNextSubpass2KHR;

    /// Create a new render pass object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_render_pass2_khr(
        &self,
        info: &RenderPassCreateInfo2,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkRenderPass> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            (self.create_render_pass_2_khr_fn().0)(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }
}
#[cfg(feature = "VK_KHR_create_renderpass2")]
DerefContainerWithGuardsBracketImpl!(for DeviceCreateRenderPass2Extension {
    #[implements]
    ForwardFnPtr!(deref create_render_pass_2_khr_fn -> PFN_vkCreateRenderPass2KHR);
    #[implements]
    ForwardFnPtr!(deref cmd_begin_render_pass_2_khr_fn -> PFN_vkCmdBeginRenderPass2KHR);
    #[implements]
    ForwardFnPtr!(deref cmd_end_render_pass_2_khr_fn -> PFN_vkCmdEndRenderPass2KHR);
    #[implements]
    ForwardFnPtr!(deref cmd_next_subpass_2_khr_fn -> PFN_vkCmdNextSubpass2KHR);
});
#[cfg(feature = "VK_KHR_create_renderpass2")]
impl<Instance: crate::Instance> DeviceCreateRenderPass2Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn create_render_pass_2_khr_fn(&self) -> PFN_vkCreateRenderPass2KHR {
        *self.ext.create_render_pass_2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn cmd_begin_render_pass_2_khr_fn(&self) -> PFN_vkCmdBeginRenderPass2KHR {
        *self.ext.cmd_begin_render_pass_2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn cmd_end_render_pass_2_khr_fn(&self) -> PFN_vkCmdEndRenderPass2KHR {
        *self.ext.cmd_end_render_pass_2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn cmd_next_subpass_2_khr_fn(&self) -> PFN_vkCmdNextSubpass2KHR {
        *self.ext.cmd_next_subpass_2_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
pub trait DeviceSynchronization2Extension: Device {
    #[implements]
    fn queue_submit2_khr_fn(&self) -> PFN_vkQueueSubmit2KHR;
    #[implements]
    fn cmd_pipeline_barrier_2_khr_fn(&self) -> PFN_vkCmdPipelineBarrier2KHR;
}
#[cfg(feature = "VK_KHR_synchronization2")]
DerefContainerWithGuardsBracketImpl!(for DeviceSynchronization2Extension {
    #[implements]
    ForwardFnPtr!(deref queue_submit2_khr_fn -> PFN_vkQueueSubmit2KHR);
    #[implements]
    ForwardFnPtr!(deref cmd_pipeline_barrier_2_khr_fn -> PFN_vkCmdPipelineBarrier2KHR);
});
#[cfg(feature = "VK_KHR_synchronization2")]
impl<Instance: crate::Instance> DeviceSynchronization2Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn queue_submit2_khr_fn(&self) -> PFN_vkQueueSubmit2KHR {
        *self.ext.queue_submit2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn cmd_pipeline_barrier_2_khr_fn(&self) -> PFN_vkCmdPipelineBarrier2KHR {
        *self.ext.cmd_pipeline_barrier_2_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
pub trait DeviceImageDrmFormatModifierExtension: Device {
    #[implements]
    fn get_image_drm_format_modifier_properties_ext_fn(&self) -> PFN_vkGetImageDrmFormatModifierPropertiesEXT;
}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
DerefContainerWithGuardsBracketImpl!(for DeviceImageDrmFormatModifierExtension {
    #[implements]
    ForwardFnPtr!(deref get_image_drm_format_modifier_properties_ext_fn -> PFN_vkGetImageDrmFormatModifierPropertiesEXT);
});
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
impl<Instance: crate::Instance> DeviceImageDrmFormatModifierExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_image_drm_format_modifier_properties_ext_fn(&self) -> PFN_vkGetImageDrmFormatModifierPropertiesEXT {
        *self.ext.get_image_drm_format_modifier_properties_ext.resolve()
    }
}

#[cfg(feature = "VK_KHR_push_descriptor")]
pub trait DevicePushDescriptorExtension: Device {
    #[implements]
    fn cmd_push_descriptor_set_khr_fn(&self) -> PFN_vkCmdPushDescriptorSetKHR;
}
#[cfg(feature = "VK_KHR_push_descriptor")]
DerefContainerWithGuardsBracketImpl!(for DevicePushDescriptorExtension {
    #[implements("VK_KHR_push_descriptor")]
    ForwardFnPtr!(deref cmd_push_descriptor_set_khr_fn -> PFN_vkCmdPushDescriptorSetKHR);
});
#[cfg(feature = "VK_KHR_push_descriptor")]
impl<Instance: crate::Instance> DevicePushDescriptorExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn cmd_push_descriptor_set_khr_fn(&self) -> PFN_vkCmdPushDescriptorSetKHR {
        *self.ext.cmd_push_descriptor_set_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_sample_locations")]
pub trait DeviceSampleLocationsExtension: Device {
    #[implements]
    fn cmd_set_sample_locations_ext_fn(&self) -> PFN_vkCmdSetSampleLocationsEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
DerefContainerWithGuardsBracketImpl!(for DeviceSampleLocationsExtension {
    #[implements]
    ForwardFnPtr!(deref cmd_set_sample_locations_ext_fn -> PFN_vkCmdSetSampleLocationsEXT);
});
#[cfg(feature = "VK_EXT_sample_locations")]
impl<Instance: crate::Instance> DeviceSampleLocationsExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn cmd_set_sample_locations_ext_fn(&self) -> PFN_vkCmdSetSampleLocationsEXT {
        *self.ext.cmd_set_sample_locations_ext.resolve()
    }
}

#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub trait DeviceTimelineSemaphoreExtension: Device {
    #[implements]
    fn get_semaphore_counter_value_ext_fn(&self) -> PFN_vkGetSemaphoreCounterValueKHR;
    #[implements]
    fn signal_semaphore_ext_fn(&self) -> PFN_vkSignalSemaphoreKHR;
    #[implements]
    fn wait_semaphores_ext_fn(&self) -> PFN_vkWaitSemaphoresKHR;

    /// Query the current state of a timeline semaphore
    /// # Failure
    ///
    /// * [`VK_ERROR_DEVICE_LOST`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_UNKNOWN`]
    /// * [`VK_ERROR_VALIDATION_FAILED`]
    #[implements]
    fn get_semaphore_counter_value_khr(
        &self,
        semaphore: &(impl VkHandle<Handle = VkSemaphore> + ?Sized),
    ) -> crate::Result<u64> {
        let mut sink = core::mem::MaybeUninit::uninit();

        unsafe {
            (self.get_semaphore_counter_value_ext_fn().0)(self.native_ptr(), semaphore.native_ptr(), sink.as_mut_ptr())
                .into_result()?;
        }

        Ok(unsafe { sink.assume_init() })
    }

    /// Signal a timeline semaphore on the host
    /// # Failure
    ///
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_UNKNOWN`]
    /// * [`VK_ERROR_VALIDATION_FAILED`]
    #[implements]
    fn signal_semaphore_khr(&self, info: &SemaphoreSignalInfo) -> crate::Result<()> {
        unsafe {
            (self.signal_semaphore_ext_fn().0)(self.native_ptr(), info as *const _ as _)
                .into_result()
                .map(drop)
        }
    }

    /// Wait for timeline semaphores on the host
    ///
    /// Returns `false` if timed out
    /// # Failure
    ///
    /// * [`VK_ERROR_DEVICE_LOST`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_UNKNOWN`]
    /// * [`VK_ERROR_VALIDATION_FAILED`]
    #[implements]
    fn wait_semaphores_khr(&self, info: &SemaphoreWaitInfo, timeout: u64) -> crate::Result<bool> {
        match unsafe { (self.wait_semaphores_ext_fn().0)(self.native_ptr(), info as *const _ as _, timeout) } {
            r if r == VK_SUCCESS => Ok(true),
            r if r == VK_TIMEOUT => Ok(false),
            r => Err(r),
        }
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
DerefContainerWithGuardsBracketImpl!(for DeviceTimelineSemaphoreExtension {
    #[implements]
    ForwardFnPtr!(deref get_semaphore_counter_value_ext_fn -> PFN_vkGetSemaphoreCounterValueKHR);
    #[implements]
    ForwardFnPtr!(deref signal_semaphore_ext_fn -> PFN_vkSignalSemaphoreKHR);
    #[implements]
    ForwardFnPtr!(deref wait_semaphores_ext_fn -> PFN_vkWaitSemaphoresKHR);
});
#[cfg(feature = "VK_KHR_timeline_semaphore")]
impl<Instance: crate::Instance> DeviceTimelineSemaphoreExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_semaphore_counter_value_ext_fn(&self) -> PFN_vkGetSemaphoreCounterValueKHR {
        *self.ext.get_semaphore_counter_value_ext.resolve()
    }

    #[implements]
    #[inline(always)]
    fn signal_semaphore_ext_fn(&self) -> PFN_vkSignalSemaphoreKHR {
        *self.ext.signal_semaphore_ext.resolve()
    }

    #[implements]
    #[inline(always)]
    fn wait_semaphores_ext_fn(&self) -> PFN_vkWaitSemaphoresKHR {
        *self.ext.wait_semaphores_ext.resolve()
    }
}

/// Child of a device object(raw handle)
pub trait DeviceChildHandle {
    /// Retrieve a reference to a device handle that creates this objecs
    fn device_handle(&self) -> VkDevice;
}
DerefContainerBracketImpl!(for DeviceChildHandle {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice { T::device_handle(self) }
});
GuardsImpl!(for DeviceChildHandle {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice { T::device_handle(self) }
});

/// Child of a device object
pub trait DeviceChild: DeviceChildHandle {
    /// A concrete type of the parent device object.
    type ConcreteDevice: Device;

    /// Retrieve a reference to a device object that creates this object
    fn device(&self) -> &Self::ConcreteDevice;
}
DerefContainerBracketImpl!(for DeviceChild {
    type ConcreteDevice = T::ConcreteDevice;

    fn device(&self) -> &Self::ConcreteDevice { T::device(self) }
});
GuardsImpl!(for DeviceChild {
    type ConcreteDevice = T::ConcreteDevice;

    fn device(&self) -> &Self::ConcreteDevice { T::device(self) }
});

pub trait DeviceChildTransferrable: DeviceChild {
    fn transfer_device(self) -> Self::ConcreteDevice;
}
impl<T> DeviceChildTransferrable for &'_ T
where
    T: DeviceChild + ?Sized,
    T::ConcreteDevice: Clone,
{
    fn transfer_device(self) -> Self::ConcreteDevice {
        self.device().clone()
    }
}

/// Opaque handle to a queue object
#[derive(Clone, VkHandle, VkObject)]
#[VkObject(type = VK_OBJECT_TYPE_QUEUE)]
pub struct QueueObject<Device>(VkQueue, Device);
unsafe impl<Device: Sync> Sync for QueueObject<Device> {}
unsafe impl<Device: Send> Send for QueueObject<Device> {}
impl<Device: crate::Device> Queue for QueueObject<Device> {}
impl<Device: crate::Device> QueueMut for QueueObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for QueueObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for QueueObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: Clone> QueueObject<&'_ Device> {
    #[inline(always)]
    pub fn clone_parent(self) -> QueueObject<Device> {
        QueueObject(self.0, self.1.clone())
    }
}
impl<Device> QueueObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkQueue, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkQueue, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}

pub trait Queue: VkHandle<Handle = VkQueue> + DeviceChild {}
DerefContainerBracketImpl!(for Queue {});
GuardsImpl!(for Queue {});

pub trait QueueMut: Queue + VkHandleMut {
    /// Wait for a object to become idle
    #[implements]
    #[inline(always)]
    fn wait(&mut self) -> crate::Result<()> {
        unsafe {
            crate::vkfn::queue_wait_idle(self.native_ptr_mut())
                .into_result()
                .map(drop)
        }
    }

    /// Bind device memory to a sparse resource object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[implements("alloc")]
    fn bind_sparse(
        &mut self,
        batches: &[impl SparseBindingOpBatch],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        let batches: Vec<_> = crate::alloc::collect_vec(batches.iter().map(SparseBindingOpBatch::make_info_struct));

        unsafe { self.bind_sparse_raw(&batches, fence) }
    }

    /// Bind device memory to a sparse resource object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[implements]
    #[inline(always)]
    fn bind_sparse2(
        &mut self,
        batches: &[BindSparseInfo],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::queue_bind_sparse(
                self.native_ptr_mut(),
                batches.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(batches).cast(),
                fence.map(|x| x.0),
            )
            .into_result()
            .map(drop)
        }
    }

    /// Bind device memory to a sparse resource object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    unsafe fn bind_sparse_raw(
        &mut self,
        batches: &[VkBindSparseInfo],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::queue_bind_sparse(
                self.native_ptr_mut(),
                batches.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(batches),
                fence.map(|x| x.0),
            )
            .into_result()
            .map(drop)
        }
    }

    /// Submits a sequence of semaphores or command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[implements("alloc")]
    #[allow(deprecated)]
    fn submit(
        &mut self,
        batches: &[impl SubmissionBatch],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        let batch_resources: Vec<_> = crate::alloc::collect_vec(batches.iter().map(|b| {
            let mut resources = TemporalSubmissionBatchResources::new();
            b.collect_resources(&mut resources);
            resources
        }));
        let batches: Vec<_> = crate::alloc::collect_vec(
            batch_resources
                .iter()
                .map(TemporalSubmissionBatchResources::make_info_struct),
        );

        unsafe { self.submit_raw(&batches, fence) }
    }

    /// Submits a sequence of semaphores or command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    unsafe fn submit_raw(
        &mut self,
        batches: &[SubmitInfo],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::queue_submit(
                self.native_ptr_mut(),
                batches.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(batches) as _,
                fence.map(|x| x.0),
            )
            .into_result()
            .map(drop)
        }
    }

    /// Submits command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[implements("VK_KHR_synchronization2")]
    fn submit2_khr(
        &mut self,
        device: &(impl DeviceSynchronization2Extension + ?Sized),
        batches: &[SubmitInfo2],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        unsafe {
            (device.queue_submit2_khr_fn().0)(
                self.native_ptr_mut(),
                batches.len() as _,
                slice_as_ptr_empty_null(batches) as _,
                fence.map(|x| x.0),
            )
            .into_result()
            .map(drop)
        }
    }

    /// Submits command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[implements("Allow1_3APIs")]
    fn submit2(&mut self, batches: &[SubmitInfo2], fence: Option<VkHandleRefMut<VkFence>>) -> crate::Result<()> {
        unsafe {
            crate::vkfn::queue_submit2(
                self.native_ptr_mut(),
                batches.len() as _,
                slice_as_ptr_empty_null(batches) as _,
                fence.map(|x| x.0),
            )
            .into_result()
            .map(drop)
        }
    }

    /// Queue images for presentation
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    /// * [`VK_ERROR_OUT_OF_DATE_KHR`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    #[implements("VK_KHR_swapchain")]
    fn present<'r>(&mut self, info: &PresentInfo<'r>) -> crate::Result<()> {
        unsafe {
            crate::vkfn::queue_present_khr(self.native_ptr_mut(), info as *const _ as _)
                .into_result()
                .map(drop)
        }
    }
}
DerefContainerBracketImpl!(for mut QueueMut {});
GuardsImpl!(for mut QueueMut {});

#[cfg(feature = "VK_KHR_swapchain")]
#[repr(transparent)]
pub struct PresentInfo<'r>(
    VkPresentInfoKHR,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(&'r [VkSwapchainKHR], &'r [VkSemaphore], &'r [u32], &'r mut [VkResult])>,
);
#[cfg(feature = "VK_KHR_swapchain")]
impl<'r> PresentInfo<'r> {
    #[inline(always)]
    pub fn new(
        wait_semaphores: &'r [VkHandleRef<VkSemaphore>],
        swapchains: &'r [VkHandleRef<VkSwapchainKHR>],
        image_indices: &'r [u32],
        results: &'r mut [VkResult],
    ) -> Self {
        assert_eq!(swapchains.len(), image_indices.len());
        assert_eq!(swapchains.len(), results.len());

        Self(
            VkPresentInfoKHR {
                sType: VkPresentInfoKHR::TYPE,
                pNext: core::ptr::null(),
                waitSemaphoreCount: wait_semaphores.len() as _,
                pWaitSemaphores: crate::ffi_helper::slice_as_ptr_empty_null(wait_semaphores) as _,
                swapchainCount: swapchains.len() as _,
                pSwapchains: crate::ffi_helper::slice_as_ptr_empty_null(swapchains) as _,
                pImageIndices: crate::ffi_helper::slice_as_ptr_empty_null(image_indices),
                pResults: crate::ffi_helper::slice_as_mut_ptr_empty_null(results),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkPresentInfoKHR`] struct.
    pub const unsafe fn from_raw(raw: VkPresentInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkPresentInfoKHR {
        self.0
    }

    #[implements]
    pub fn submit(&self, queue: &mut (impl QueueMut + ?Sized)) -> crate::Result<()> {
        queue.present(self)
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CommandBufferSubmitInfo<'r>(
    VkCommandBufferSubmitInfoKHR,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = VkCommandBuffer>>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'r> CommandBufferSubmitInfo<'r> {
    #[inline(always)]
    pub fn new(command_buffer: &'r (impl VkHandle<Handle = VkCommandBuffer> + ?Sized)) -> Self {
        Self(
            VkCommandBufferSubmitInfoKHR {
                sType: VkCommandBufferSubmitInfoKHR::TYPE,
                pNext: core::ptr::null(),
                commandBuffer: command_buffer.native_ptr(),
                deviceMask: 0,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkCommandBufferSubmitInfoKHR`] struct.
    pub const unsafe fn from_raw(raw: VkCommandBufferSubmitInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkCommandBufferSubmitInfoKHR {
        self.0
    }

    pub const fn on_device(mut self, mask: u32) -> Self {
        self.0.deviceMask = mask;
        self
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct SubmitInfo2<'b, 'r>(
    VkSubmitInfo2KHR,
    core::marker::PhantomData<(
        &'b [SemaphoreSubmitInfo<'r>],
        &'b [SemaphoreSubmitInfo<'r>],
        &'b [CommandBufferSubmitInfo<'r>],
    )>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'b, 'r> SubmitInfo2<'b, 'r> {
    pub const fn new(
        wait_semaphores: &'b [SemaphoreSubmitInfo<'r>],
        command_buffers: &'b [CommandBufferSubmitInfo<'r>],
        signal_semaphores: &'b [SemaphoreSubmitInfo<'r>],
    ) -> Self {
        Self(
            VkSubmitInfo2KHR {
                sType: VkSubmitInfo2KHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                waitSemaphoreInfoCount: wait_semaphores.len() as _,
                pWaitSemaphoreInfos: crate::ffi_helper::slice_as_ptr_empty_null(wait_semaphores) as _,
                commandBufferInfoCount: command_buffers.len() as _,
                pCommandBufferInfos: crate::ffi_helper::slice_as_ptr_empty_null(command_buffers) as _,
                signalSemaphoreInfoCount: signal_semaphores.len() as _,
                pSignalSemaphoreInfos: crate::ffi_helper::slice_as_ptr_empty_null(signal_semaphores) as _,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkSubmitInfo2KHR`] struct.
    pub const unsafe fn from_raw(raw: VkSubmitInfo2KHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }
    pub const fn into_raw(self) -> VkSubmitInfo2KHR {
        self.0
    }

    pub const fn protected(mut self) -> Self {
        self.0.flags |= VK_SUBMIT_PROTECTED_BIT_KHR;
        self
    }
}
