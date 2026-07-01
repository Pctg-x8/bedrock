//! Vulkan Device and Queues
use bedrock_vk::{self as brvk, TypedVulkanStructure};

use crate::error::translate_vk_result;
use crate::ffi_helper::{CStrFFIRef, slice_as_ptr_empty_null};
use crate::*;
use derives::implements;

#[implements]
#[allow(dead_code)]
type DeviceResolvedFn<F> = brvk::ResolvedFnCell<F, DeviceResolverImpl>;
#[implements]
#[repr(transparent)]
pub struct DeviceResolverImpl(pub brvk::VkDevice);
#[implements]
impl brvk::ResolverInterface for DeviceResolverImpl {
    #[tracing::instrument(
        name = "<brvk::VkDevice as ResolverInterface>::load_symbol_unconstrainted",
        skip(self)
    )]
    unsafe fn load_symbol_unconstrainted(&self, name: &core::ffi::CStr) -> core::ptr::NonNull<core::ffi::c_void> {
        match unsafe { brvk::fns::get_device_proc_addr(self.0, name.as_ptr().cast()) } {
            Some(x) => unsafe { core::ptr::NonNull::new_unchecked(x as _) },
            None => {
                tracing::error!("device function not found, bedrock could not continue");
                std::process::abort();
            }
        }
    }

    #[tracing::instrument(
        name = "<brvk::VkDevice as ResolverInterface>::load_function_unconstrainted",
        skip(self)
    )]
    unsafe fn load_function_unconstrainted(&self, name: &core::ffi::CStr) -> brvk::PFN_vkVoidFunction {
        match unsafe { brvk::fns::get_device_proc_addr(self.0, name.as_ptr().cast()) } {
            Some(x) => x,
            None => {
                tracing::error!("device function not found, bedrock could not continue");
                std::process::abort();
            }
        }
    }
}

/// Opaque handle to a device object
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VK_OBJECT_TYPE_DEVICE)]
pub struct DeviceObject<Instance> {
    #[handle]
    handle: brvk::VkDevice,
    parent: Instance,
    #[cfg(feature = "Implements")]
    ext: DeviceExtFunctions,
}
impl<Instance> DeviceObject<Instance> {
    pub const fn wrap_handle(handle: brvk::VkDevice, parent: Instance) -> Self {
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
            crate::vkfn_wrapper::destroy_device(self.as_transparent_ref_mut(), None);
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
impl<Instance: crate::Instance> DeviceMut for DeviceObject<Instance> {}
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
        // disable running brvk::VkDevice destruction
        std::mem::forget(self);

        r
    }
}
impl<Instance: crate::Instance> DeviceObject<Instance> {
    /// Create a new device instance
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_INITIALIZATION_FAILED`
    /// * `brvk::VK_ERROR_EXTENSION_NOT_PRESENT`
    /// * `brvk::VK_ERROR_FEATURE_NOT_PRESENT`
    /// * `brvk::VK_ERROR_TOO_MANY_OBJECTS`
    /// * `brvk::VK_ERROR_DEVICE_LOST`
    #[implements]
    pub fn new<
        PhysicalDevice: crate::PhysicalDevice + crate::InstanceChildTransferrable<ConcreteInstance = Instance>,
    >(
        physical_device: PhysicalDevice,
        info: &DeviceCreateInfo,
    ) -> crate::Result<Self> {
        Ok(Self::wrap_handle(
            crate::vkfn_wrapper::create_device(physical_device.as_transparent_ref(), info, None)?,
            physical_device.transfer_instance(),
        ))
    }

    /// Constructs from raw handle
    /// # Safety
    /// the handle must be valid and not freed
    pub const unsafe fn manage(handle: brvk::VkDevice, parent: Instance) -> Self {
        Self::wrap_handle(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub fn unmanage(mut self) -> (brvk::VkDevice, Instance) {
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

/// Family Index, Queue Priorities
#[repr(transparent)]
pub struct DeviceQueueCreateInfo<'d>(brvk::VkDeviceQueueCreateInfo, core::marker::PhantomData<&'d [f32]>);
impl<'d> DeviceQueueCreateInfo<'d> {
    pub const fn new(family_index: u32, priorities: &'d [f32]) -> Self {
        Self(
            brvk::VkDeviceQueueCreateInfo {
                sType: brvk::VkDeviceQueueCreateInfo::TYPE,
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
    brvk::VkDeviceCreateInfo,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        Option<&'d dyn brvk::VulkanStructure>,
        &'d [DeviceQueueCreateInfo<'d>],
        &'d [CStrFFIRef<'d>],
        &'d [CStrFFIRef<'d>],
        Option<&'d brvk::VkPhysicalDeviceFeatures>,
    )>,
);
impl<'d> DeviceCreateInfo<'d> {
    pub const fn new(
        queue_infos: &'d [DeviceQueueCreateInfo<'d>],
        layers: &'d [CStrFFIRef<'d>],
        extensions: &'d [CStrFFIRef<'d>],
    ) -> Self {
        Self(
            brvk::VkDeviceCreateInfo {
                sType: brvk::VkDeviceCreateInfo::TYPE,
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
    /// `raw` must be a valid [`brvk::VkDeviceCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkDeviceCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkDeviceCreateInfo {
        self.0
    }

    pub const fn with_features(mut self, features: &'d brvk::VkPhysicalDeviceFeatures) -> Self {
        self.0.pEnabledFeatures = features as *const _ as _;
        self
    }

    pub fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Clone)]
pub struct PhysicalDeviceFeatures2<'r>(
    brvk::VkPhysicalDeviceFeatures2KHR,
    core::marker::PhantomData<Option<&'r mut dyn brvk::VulkanStructure>>,
);
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl<'r> PhysicalDeviceFeatures2<'r> {
    pub const fn new(old_features: brvk::VkPhysicalDeviceFeatures) -> Self {
        Self(
            brvk::VkPhysicalDeviceFeatures2KHR {
                sType: <brvk::VkPhysicalDeviceFeatures2KHR as brvk::TypedVulkanStructure>::TYPE,
                pNext: core::ptr::null_mut(),
                features: old_features,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'r mut (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic_mut() as *mut _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl brvk::VulkanStructure for PhysicalDeviceFeatures2<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        brvk::VulkanStructure::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        brvk::VulkanStructure::as_generic_mut(&mut self.0)
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl brvk::VulkanSinkStructure for PhysicalDeviceFeatures2<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanSinkStructure {
        brvk::VulkanSinkStructure::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanSinkStructure {
        brvk::VulkanSinkStructure::as_generic_mut(&mut self.0)
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PhysicalDeviceSynchronization2Features<'r>(
    brvk::VkPhysicalDeviceSynchronization2FeaturesKHR,
    core::marker::PhantomData<Option<&'r mut dyn brvk::VulkanStructure>>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'r> PhysicalDeviceSynchronization2Features<'r> {
    pub const fn new(enabled: bool) -> Self {
        Self(
            brvk::VkPhysicalDeviceSynchronization2FeaturesKHR {
                sType: <brvk::VkPhysicalDeviceSynchronization2FeaturesKHR as brvk::TypedVulkanStructure>::TYPE,
                pNext: core::ptr::null_mut(),
                synchronization2: enabled as _,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'r mut (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic_mut() as *mut _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
unsafe impl brvk::VulkanStructure for PhysicalDeviceSynchronization2Features<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        brvk::VulkanStructure::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        brvk::VulkanStructure::as_generic_mut(&mut self.0)
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
unsafe impl brvk::VulkanSinkStructure for PhysicalDeviceSynchronization2Features<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanSinkStructure {
        brvk::VulkanSinkStructure::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanSinkStructure {
        brvk::VulkanSinkStructure::as_generic_mut(&mut self.0)
    }
}

pub trait Device: VkHandle<Handle = brvk::VkDevice> + InstanceChild {
    /// Get a queue handle from a device
    #[implements]
    fn queue(&self, family_index: u32, queue_index: u32) -> QueueObject<&Self> {
        QueueObject(
            crate::vkfn_wrapper::get_device_queue(self.as_transparent_ref(), family_index, queue_index),
            self,
        )
    }

    /// Create a new fence object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn new_fence_raw(
        &self,
        info: &FenceCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkFence> {
        crate::vkfn_wrapper::create_fence(self.as_transparent_ref(), info, allocation_callbacks)
    }

    /// Create a new queue semaphore object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn new_semaphore_raw(
        &self,
        info: &SemaphoreCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkSemaphore> {
        crate::vkfn_wrapper::create_semaphore(self.as_transparent_ref(), info, allocation_callbacks)
    }

    /// Create a new event object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_event_raw(
        &self,
        info: &EventCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkEvent> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            brvk::fns::create_event(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Allocate device memory
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_INVALID_EXTERNAL_HANDLE`]
    /// * [`brvk::VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
    #[implements]
    #[inline(always)]
    fn allocate_memory(
        &self,
        info: &MemoryAllocateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkDeviceMemory> {
        crate::vkfn_wrapper::allocate_memory(self.as_transparent_ref(), info, allocation_callbacks)
    }

    /// Create a new buffer object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn new_buffer_raw(
        &self,
        info: &BufferCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkBuffer> {
        crate::vkfn_wrapper::create_buffer(self.as_transparent_ref(), info, allocation_callbacks)
    }

    /// Create a new buffer view object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn new_buffer_view_raw(
        &self,
        info: &BufferViewCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkBufferView> {
        unsafe { crate::vkfn_wrapper::create_buffer_view(self.as_transparent_ref(), info, allocation_callbacks) }
    }

    /// Create a new sampler object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_TOO_MANY_OBJECTS`]
    #[implements]
    #[inline(always)]
    fn new_sampler_raw(
        &self,
        info: &SamplerCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkSampler> {
        crate::vkfn_wrapper::create_sampler(self.as_transparent_ref(), info, allocation_callbacks)
    }

    /// Create a new image object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_COMPRESSION_EXHAUSTED_EXT`]
    /// * [`brvk::VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
    #[implements]
    #[inline(always)]
    fn new_image_raw(
        &self,
        info: &ImageCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkImage> {
        crate::vkfn_wrapper::create_image(self.as_transparent_ref(), info, allocation_callbacks)
    }

    /// Create a new image view from an existing image
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
    #[implements]
    #[inline(always)]
    fn new_image_view_raw(
        &self,
        info: &ImageViewCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkImageView> {
        unsafe { crate::vkfn_wrapper::create_image_view(self.as_transparent_ref(), info, allocation_callbacks) }
    }

    /// Create a new render pass object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_render_pass(
        &self,
        info: &RenderPassCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkRenderPass> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            brvk::fns::create_render_pass(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new render pass object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements("Allow1_2APIs")]
    #[inline]
    fn new_render_pass2(
        &self,
        info: &RenderPassCreateInfo2,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkRenderPass> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            brvk::fns::create_render_pass2(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new framebuffer object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn new_framebuffer_raw(
        &self,
        info: &FramebufferCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkFramebuffer> {
        unsafe { crate::vkfn_wrapper::create_framebuffer(self.as_transparent_ref(), info, allocation_callbacks) }
    }

    /// Creates a new shader module object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_INVALID_SHADER_NV`]
    #[implements]
    #[inline]
    fn new_shader_module_raw(
        &self,
        info: &ShaderModuleCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkShaderModule> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            brvk::fns::create_shader_module(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new pipeline cache
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_pipeline_cache_raw(
        &self,
        info: &PipelineCacheCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkPipelineCache> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            brvk::fns::create_pipeline_cache(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new pipeline layout object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn new_pipeline_layout_raw(
        &self,
        info: &PipelineLayoutCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkPipelineLayout> {
        crate::vkfn_wrapper::create_pipeline_layout(self.as_transparent_ref(), info, allocation_callbacks)
    }

    /// Create graphics pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    #[inline(always)]
    unsafe fn new_graphics_pipelines_raw(
        &self,
        infos: &[GraphicsPipelineCreateInfo],
        cache: Option<VkHandleRef<brvk::VkPipelineCache>>,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
        objects: &mut [core::mem::MaybeUninit<brvk::VkPipeline>],
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn_wrapper::create_graphics_pipelines(
                self.as_transparent_ref(),
                cache,
                infos,
                allocation_callbacks,
                objects,
            )
        }
    }

    /// Create graphics pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("alloc")]
    fn new_graphics_pipelines<'s>(
        &'s self,
        infos: &[GraphicsPipelineCreateInfo],
        cache: Option<&(impl crate::VkHandle<Handle = brvk::VkPipelineCache> + ?Sized)>,
    ) -> crate::Result<Vec<crate::PipelineObject<&'s Self>>> {
        let mut hs = vec![core::mem::MaybeUninit::uninit(); infos.len()];

        unsafe {
            self.new_graphics_pipelines_raw(infos, cache.map(VkHandle::as_transparent_ref), None, &mut hs)?;
        }

        Ok(crate::alloc::collect_vec(hs.into_iter().map(move |h| unsafe {
            crate::PipelineObject::manage(h.assume_init(), self)
        })))
    }

    /// Create graphics pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn new_graphics_pipeline_array<'s, const N: usize>(
        &'s self,
        infos: &[GraphicsPipelineCreateInfo; N],
        cache: Option<&(impl crate::VkHandle<Handle = brvk::VkPipelineCache> + ?Sized)>,
    ) -> crate::Result<[crate::PipelineObject<&'s Self>; N]> {
        let mut hs = [core::mem::MaybeUninit::<brvk::VkPipeline>::uninit(); N];

        unsafe {
            self.new_graphics_pipelines_raw(infos, cache.map(VkHandle::as_transparent_ref), None, &mut hs)?;
        }

        Ok(core::array::from_fn(move |n| unsafe {
            crate::PipelineObject::manage(hs[n].assume_init(), self)
        }))
    }

    /// Create compute pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    unsafe fn new_compute_pipelines_raw(
        &self,
        infos: &[ComputePipelineCreateInfo],
        cache: Option<brvk::VkPipelineCache>,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
        objects: &mut [core::mem::MaybeUninit<brvk::VkPipeline>],
    ) -> crate::Result<()> {
        translate_vk_result(unsafe {
            brvk::fns::create_compute_pipelines(
                self.native_ptr(),
                cache,
                infos.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(infos) as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                objects.as_mut_ptr() as _,
            )
        })
        .map(drop)
    }

    /// Create compute pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("alloc")]
    fn new_compute_pipelines<'s>(
        &'s self,
        infos: &[ComputePipelineCreateInfo],
        cache: Option<&(impl crate::VkHandle<Handle = brvk::VkPipelineCache> + ?Sized)>,
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
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn new_compute_pipeline_array<'s, const N: usize>(
        &'s self,
        infos: &[ComputePipelineCreateInfo; N],
        cache: Option<&(impl crate::VkHandle<Handle = brvk::VkPipelineCache> + ?Sized)>,
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
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_command_pool_raw(
        &self,
        info: &CommandPoolCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkCommandPool> {
        crate::vkfn_wrapper::create_command_pool(self.as_transparent_ref(), info, allocation_callbacks)
    }

    /// Create a new query pool object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_query_pool_raw(
        &self,
        info: &QueryPoolCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkQueryPool> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            brvk::fns::create_query_pool(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Invalidate `MappedMemoryRange`s
    /// Invalidating the memory range allows that device writes to the memory ranges
    /// which have been made visible to the `brvk::VK_ACCESS_HOST_WRITE_BIT` and `brvk::VK_ACCESS_HOST_READ_BIT`
    /// are made visible to the host
    /// # Safety
    /// Memory object in `ranges` must be currently host mapped
    #[implements]
    #[inline(always)]
    unsafe fn invalidate_memory_range(&self, ranges: &[MappedMemoryRange]) -> crate::Result<()> {
        crate::vkfn_wrapper::invalidate_mapped_memory_ranges(self.as_transparent_ref(), ranges)
    }

    /// Flush `MappedMemoryRange`s
    /// Flushing the memory range allows that host writes to the memory ranges can
    /// be made available to device access
    /// # Safety
    /// Memory object in `ranges` must be currently host mapped
    #[implements]
    #[inline(always)]
    unsafe fn flush_mapped_memory_ranges(&self, ranges: &[MappedMemoryRange]) -> crate::Result<()> {
        crate::vkfn_wrapper::flush_mapped_memory_ranges(self.as_transparent_ref(), ranges)
    }

    /// Update the contents of descriptor set objects
    ///
    /// # Safety
    ///
    /// `writes` and `copies` must be a valid array of [`VkWriteDescriptorSet`] and [`VkCopyDescriptorSet`] structs, respectively.
    #[implements]
    #[inline]
    unsafe fn update_descriptor_sets_raw(
        &self,
        writes: &[brvk::VkWriteDescriptorSet],
        copies: &[brvk::VkCopyDescriptorSet],
    ) {
        unsafe {
            brvk::fns::update_descriptor_sets(
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

    /// Query the memory requirements for a sparse image
    /// # Safety
    /// `sink_head_ptr` must be a valid pointer for read/write operations.
    #[implements("Allow1_1APIs")]
    #[inline]
    unsafe fn get_image_sparse_memory_requirements2_count(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2,
        count_sink: &mut core::mem::MaybeUninit<u32>,
        sink_head_ptr: *mut brvk::VkSparseImageMemoryRequirements2KHR,
    ) {
        unsafe {
            brvk::fns::get_image_sparse_memory_requirements2(
                self.native_ptr(),
                &info.0,
                count_sink.as_mut_ptr(),
                sink_head_ptr,
            )
        }
    }

    /// Single binding for a buffer
    /// # Safety
    /// `brvk::VkBuffer` and `brvk::VkDeviceMemory` must be valid and created from this device object
    #[implements]
    #[inline(always)]
    unsafe fn bind_buffer_raw(
        &self,
        buffer: VkHandleRefMut<brvk::VkBuffer>,
        memory: VkHandleRef<brvk::VkDeviceMemory>,
        offset: brvk::VkDeviceSize,
    ) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::bind_buffer_memory(self.as_transparent_ref(), buffer, memory, offset) }
    }

    /// Multiple Binding for Buffers
    ///
    /// # Safety
    ///
    /// buffers in `bounds` must be created from this device.
    #[implements("Allow1_1APIs")]
    #[inline(always)]
    unsafe fn bind_buffers(&self, bounds: &[BindBufferMemoryInfo]) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::bind_buffer_memory2(self.as_transparent_ref(), bounds) }
    }

    /// Single binding for an image
    /// # Safety
    /// `brvk::VkImage` and `brvk::VkDeviceMemory` must be valid and created from this device object
    #[implements]
    #[inline(always)]
    unsafe fn bind_image_raw(
        &self,
        image: VkHandleRefMut<brvk::VkImage>,
        memory: VkHandleRef<brvk::VkDeviceMemory>,
        offset: brvk::VkDeviceSize,
    ) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::bind_image_memory(self.as_transparent_ref(), image, memory, offset) }
    }

    /// Multiple Binding for Images
    ///
    /// # Safety
    ///
    /// images in `bounds` must be created from this device.
    #[implements("Allow1_1APIs")]
    #[inline(always)]
    unsafe fn bind_images(&self, bounds: &[BindImageMemoryInfo]) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::bind_image_memory2(self.as_transparent_ref(), bounds) }
    }

    /// Wait for one or more fences to become signaled
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_DEVICE_LOST`
    #[implements]
    #[inline(always)]
    fn wait_multiple_fences(
        &self,
        objects: &[VkHandleRef<brvk::VkFence>],
        wait_all: bool,
        timeout: Option<u64>,
    ) -> crate::Result<TimeoutableWaitResult> {
        unsafe {
            crate::vkfn_wrapper::wait_for_fences(
                self.as_transparent_ref(),
                objects,
                wait_all,
                timeout.unwrap_or(u64::MAX),
            )
        }
    }

    /// Resets one or more fence objects
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline(always)]
    fn reset_multiple_fences(&self, objects: &[VkHandleRefMut<brvk::VkFence>]) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::reset_fences(self.as_transparent_ref(), objects) }
    }

    /// Create a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_DEVICE_LOST`]
    /// * [`brvk::VK_ERROR_SURFACE_LOST_KHR`]
    /// * [`brvk::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`]
    /// * [`brvk::VK_ERROR_INITIALIZATION_FAILED`]
    /// * [`brvk::VK_ERROR_COMPRESSION_EXHAUSTED_EXT`]
    #[implements("VK_KHR_swapchain")]
    #[inline(always)]
    fn new_swapchain_raw(
        &self,
        info: &SwapchainCreateInfo,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkSwapchainKHR> {
        crate::vkfn_wrapper::create_swapchain(self.as_transparent_ref(), info, allocation_callbacks)
    }

    /// Give a user-friendly name to an object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_EXT_debug_utils")]
    fn set_object_name(&self, info: &crate::DebugUtilsObjectNameInfo) -> crate::Result<()>
    where
        Self::ConcreteInstance: InstanceDebugUtilsExtension,
    {
        translate_vk_result(unsafe {
            self.instance().set_debug_utils_object_name_ext_fn().0(self.native_ptr(), &info.0)
        })?;

        Ok(())
    }

    /// Create a new descriptor update template
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements("Allow1_1APIs")]
    unsafe fn new_descriptor_update_template_raw(
        &self,
        info: &brvk::VkDescriptorUpdateTemplateCreateInfoKHR,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkDescriptorUpdateTemplateKHR> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            brvk::fns::create_descriptor_update_template(
                self.native_ptr(),
                info,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Query the current state of a timeline semaphore
    /// # Failure
    ///
    /// * [`brvk::VK_ERROR_DEVICE_LOST`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_UNKNOWN`]
    /// * [`brvk::VK_ERROR_VALIDATION_FAILED`]
    #[implements("Allow1_2APIs")]
    #[inline(always)]
    fn get_semaphore_counter_value(
        &self,
        semaphore: &(impl VkHandle<Handle = brvk::VkSemaphore> + ?Sized),
    ) -> crate::Result<u64> {
        unsafe {
            crate::vkfn_wrapper::get_semaphore_counter_value(self.as_transparent_ref(), semaphore.as_transparent_ref())
        }
    }

    /// Signal a timeline semaphore on the host
    /// # Failure
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_UNKNOWN`]
    /// * [`brvk::VK_ERROR_VALIDATION_FAILED`]
    #[implements("Allow1_2APIs")]
    #[inline(always)]
    fn signal_semaphore(&self, info: &SemaphoreSignalInfo) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::signal_semaphore(self.as_transparent_ref(), info) }
    }

    /// Wait for timeline semaphores on the host
    /// # Failure
    ///
    /// * [`brvk::VK_ERROR_DEVICE_LOST`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_UNKNOWN`]
    /// * [`brvk::VK_ERROR_VALIDATION_FAILED`]
    #[implements("Allow1_2APIs")]
    #[inline(always)]
    fn wait_semaphores(&self, info: &SemaphoreWaitInfo, timeout: u64) -> crate::Result<TimeoutableWaitResult> {
        unsafe { crate::vkfn_wrapper::wait_semaphores(self.as_transparent_ref(), info, timeout) }
    }
}
DerefContainerWithGuardsBracketImpl!(for Device {});

pub trait DeviceMut: Device + VkHandleMut {
    /// Wait for a object to become idle
    /// # Safety
    /// All brvk::VkQueue objects created from this device must be externally synchronized.
    #[implements]
    #[inline(always)]
    unsafe fn wait(&mut self) -> crate::Result<()> {
        unsafe { crate::vkfn_wrapper::device_wait_idle(self.as_transparent_ref_mut()) }
    }
}
DerefContainerWithGuardsBracketImpl!(for mut DeviceMut {});

/// Extension function caches
#[implements]
struct DeviceExtFunctions {
    #[cfg(feature = "VK_KHR_maintenance1")]
    trim_command_pool_khr: DeviceResolvedFn<brvk::PFN_vkTrimCommandPoolKHR>,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    create_descriptor_update_template_khr: DeviceResolvedFn<brvk::PFN_vkCreateDescriptorUpdateTemplateKHR>,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    destroy_descriptor_update_template_khr: DeviceResolvedFn<brvk::PFN_vkDestroyDescriptorUpdateTemplateKHR>,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    update_descriptor_set_with_template_khr: DeviceResolvedFn<brvk::PFN_vkUpdateDescriptorSetWithTemplateKHR>,
    #[cfg(feature = "VK_KHR_bind_memory2")]
    bind_buffer_memory2_khr: DeviceResolvedFn<brvk::PFN_vkBindBufferMemory2KHR>,
    #[cfg(feature = "VK_KHR_bind_memory2")]
    bind_image_memory2_khr: DeviceResolvedFn<brvk::PFN_vkBindImageMemory2KHR>,
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    get_image_drm_format_modifier_properties_ext: DeviceResolvedFn<brvk::PFN_vkGetImageDrmFormatModifierPropertiesEXT>,
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    get_fence_fd_khr: DeviceResolvedFn<brvk::PFN_vkGetFenceFdKHR>,
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    import_fence_fd_khr: DeviceResolvedFn<brvk::PFN_vkImportFenceFdKHR>,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    acquire_full_screen_exclusive_mode_ext: DeviceResolvedFn<brvk::PFN_vkAcquireFullScreenExclusiveModeEXT>,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    release_full_screen_exclusive_mode_ext: DeviceResolvedFn<brvk::PFN_vkReleaseFullScreenExclusiveModeEXT>,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    get_memory_fd_khr: DeviceResolvedFn<brvk::PFN_vkGetMemoryFdKHR>,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    get_memory_fd_properties_khr: DeviceResolvedFn<brvk::PFN_vkGetMemoryFdPropertiesKHR>,
    #[cfg(feature = "VK_EXT_external_memory_host")]
    get_memory_host_pointer_properties_ext: DeviceResolvedFn<brvk::PFN_vkGetMemoryHostPointerPropertiesEXT>,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    import_semaphore_win32_handle_khr: DeviceResolvedFn<brvk::PFN_vkImportSemaphoreWin32HandleKHR>,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    get_semaphore_win32_handle_khr: DeviceResolvedFn<brvk::PFN_vkGetSemaphoreWin32HandleKHR>,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    get_memory_win32_handle_khr: DeviceResolvedFn<brvk::PFN_vkGetMemoryWin32HandleKHR>,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    get_memory_win32_handle_properties_khr: DeviceResolvedFn<brvk::PFN_vkGetMemoryWin32HandlePropertiesKHR>,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    get_buffer_memory_requirements_2_khr: DeviceResolvedFn<brvk::PFN_vkGetBufferMemoryRequirements2KHR>,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    get_image_memory_requirements_2_khr: DeviceResolvedFn<brvk::PFN_vkGetImageMemoryRequirements2KHR>,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    get_image_sparse_memory_requirements_2_khr: DeviceResolvedFn<brvk::PFN_vkGetImageSparseMemoryRequirements2KHR>,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    create_render_pass_2_khr: DeviceResolvedFn<brvk::PFN_vkCreateRenderPass2KHR>,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    cmd_begin_render_pass_2_khr: DeviceResolvedFn<brvk::PFN_vkCmdBeginRenderPass2KHR>,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    cmd_end_render_pass_2_khr: DeviceResolvedFn<brvk::PFN_vkCmdEndRenderPass2KHR>,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    cmd_next_subpass_2_khr: DeviceResolvedFn<brvk::PFN_vkCmdNextSubpass2KHR>,
    #[cfg(feature = "VK_KHR_synchronization2")]
    queue_submit2_khr: DeviceResolvedFn<brvk::PFN_vkQueueSubmit2KHR>,
    #[cfg(feature = "VK_KHR_synchronization2")]
    cmd_pipeline_barrier_2_khr: DeviceResolvedFn<brvk::PFN_vkCmdPipelineBarrier2KHR>,
    #[cfg(feature = "VK_KHR_push_descriptor")]
    cmd_push_descriptor_set_khr: DeviceResolvedFn<brvk::PFN_vkCmdPushDescriptorSetKHR>,
    #[cfg(feature = "VK_EXT_sample_locations")]
    cmd_set_sample_locations_ext: DeviceResolvedFn<brvk::PFN_vkCmdSetSampleLocationsEXT>,
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    get_semaphore_counter_value_ext: DeviceResolvedFn<brvk::PFN_vkGetSemaphoreCounterValueKHR>,
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    signal_semaphore_ext: DeviceResolvedFn<brvk::PFN_vkSignalSemaphoreKHR>,
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    wait_semaphores_ext: DeviceResolvedFn<brvk::PFN_vkWaitSemaphoresKHR>,
}
#[implements]
impl DeviceExtFunctions {
    const fn new(handle: brvk::VkDevice) -> Self {
        Self {
            #[cfg(feature = "VK_KHR_maintenance1")]
            trim_command_pool_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_descriptor_update_template")]
            create_descriptor_update_template_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_descriptor_update_template")]
            destroy_descriptor_update_template_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_descriptor_update_template")]
            update_descriptor_set_with_template_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_bind_memory2")]
            bind_buffer_memory2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_bind_memory2")]
            bind_image_memory2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
            get_image_drm_format_modifier_properties_ext: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_external_fence_fd")]
            get_fence_fd_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_external_fence_fd")]
            import_fence_fd_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            acquire_full_screen_exclusive_mode_ext: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            release_full_screen_exclusive_mode_ext: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_external_memory_fd")]
            get_memory_fd_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_external_memory_fd")]
            get_memory_fd_properties_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_EXT_external_memory_host")]
            get_memory_host_pointer_properties_ext: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_external_semaphore_win32")]
            import_semaphore_win32_handle_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_external_semaphore_win32")]
            get_semaphore_win32_handle_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_external_memory_win32")]
            get_memory_win32_handle_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_external_memory_win32")]
            get_memory_win32_handle_properties_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_get_memory_requirements2")]
            get_buffer_memory_requirements_2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_get_memory_requirements2")]
            get_image_memory_requirements_2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_get_memory_requirements2")]
            get_image_sparse_memory_requirements_2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_create_renderpass2")]
            create_render_pass_2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_create_renderpass2")]
            cmd_begin_render_pass_2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_create_renderpass2")]
            cmd_end_render_pass_2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_create_renderpass2")]
            cmd_next_subpass_2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            queue_submit2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            cmd_pipeline_barrier_2_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_push_descriptor")]
            cmd_push_descriptor_set_khr: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_EXT_sample_locations")]
            cmd_set_sample_locations_ext: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_timeline_semaphore")]
            get_semaphore_counter_value_ext: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_timeline_semaphore")]
            signal_semaphore_ext: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
            #[cfg(feature = "VK_KHR_timeline_semaphore")]
            wait_semaphores_ext: DeviceResolvedFn::new(DeviceResolverImpl(handle)),
        }
    }
}

#[cfg(feature = "VK_KHR_maintenance1")]
pub trait DeviceMaintenance1Extension: Device {
    #[implements]
    fn trim_command_pool_khr_fn(&self) -> brvk::PFN_vkTrimCommandPoolKHR;

    /// Trim a command pool.
    #[implements]
    #[inline(always)]
    fn trim_command_pool_khr(
        &self,
        command_pool: &mut (impl VkHandleMut<Handle = brvk::VkCommandPool> + ?Sized),
        flags: CommandPoolTrimFlags,
    ) {
        unsafe { self.trim_command_pool_khr_fn().0(self.native_ptr(), command_pool.native_ptr_mut(), flags.bits()) }
    }
}
#[cfg(feature = "VK_KHR_maintenance1")]
DerefContainerWithGuardsBracketImpl!(for DeviceMaintenance1Extension {
    #[implements]
    ForwardFnPtr!(deref trim_command_pool_khr_fn -> brvk::PFN_vkTrimCommandPoolKHR);
});
#[cfg(feature = "VK_KHR_maintenance1")]
impl<Instance: crate::Instance> DeviceMaintenance1Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn trim_command_pool_khr_fn(&self) -> brvk::PFN_vkTrimCommandPoolKHR {
        *self.ext.trim_command_pool_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub trait DeviceDescriptorUpdateTemplateExtension: Device {
    #[implements]
    fn create_descriptor_update_template_khr_fn(&self) -> brvk::PFN_vkCreateDescriptorUpdateTemplateKHR;
    #[implements]
    fn destroy_descriptor_update_template_khr_fn(&self) -> brvk::PFN_vkDestroyDescriptorUpdateTemplateKHR;
    #[implements]
    fn update_descriptor_set_with_template_khr_fn(&self) -> brvk::PFN_vkUpdateDescriptorSetWithTemplateKHR;

    /// Create a new descriptor update template
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    #[inline]
    unsafe fn new_descriptor_update_template_raw_khr(
        &self,
        create_info: &brvk::VkDescriptorUpdateTemplateCreateInfoKHR,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkDescriptorUpdateTemplateKHR> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            self.create_descriptor_update_template_khr_fn().0(
                self.native_ptr(),
                create_info,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
DerefContainerWithGuardsBracketImpl!(for DeviceDescriptorUpdateTemplateExtension {
    #[implements]
    ForwardFnPtr!(deref create_descriptor_update_template_khr_fn -> brvk::PFN_vkCreateDescriptorUpdateTemplateKHR);
    #[implements]
    ForwardFnPtr!(deref destroy_descriptor_update_template_khr_fn -> brvk::PFN_vkDestroyDescriptorUpdateTemplateKHR);
    #[implements]
    ForwardFnPtr!(deref update_descriptor_set_with_template_khr_fn -> brvk::PFN_vkUpdateDescriptorSetWithTemplateKHR);
});
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl<Instance: crate::Instance> DeviceDescriptorUpdateTemplateExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn create_descriptor_update_template_khr_fn(&self) -> brvk::PFN_vkCreateDescriptorUpdateTemplateKHR {
        *self.ext.create_descriptor_update_template_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn destroy_descriptor_update_template_khr_fn(&self) -> brvk::PFN_vkDestroyDescriptorUpdateTemplateKHR {
        *self.ext.destroy_descriptor_update_template_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn update_descriptor_set_with_template_khr_fn(&self) -> brvk::PFN_vkUpdateDescriptorSetWithTemplateKHR {
        *self.ext.update_descriptor_set_with_template_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub trait DeviceGetMemoryRequirements2Extension: Device {
    #[implements]
    fn get_buffer_memory_requirements_2_khr_fn(&self) -> brvk::PFN_vkGetBufferMemoryRequirements2KHR;
    #[implements]
    fn get_image_memory_requirements_2_khr_fn(&self) -> brvk::PFN_vkGetImageMemoryRequirements2KHR;
    #[implements]
    fn get_image_sparse_memory_requirements_2_khr_fn(&self) -> brvk::PFN_vkGetImageSparseMemoryRequirements2KHR;

    /// Returns the memory requirements for specified Vulkan object.
    #[implements]
    #[inline]
    fn get_buffer_memory_requirements2_khr(
        &self,
        info: &crate::BufferMemoryRequirementsInfo2<'_, impl crate::VkHandle<Handle = brvk::VkBuffer>>,
        sink: &mut core::mem::MaybeUninit<brvk::VkMemoryRequirements2KHR>,
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
        info: &crate::ImageMemoryRequirementsInfo2<'_, impl crate::VkHandle<Handle = brvk::VkImage>>,
        sink: &mut core::mem::MaybeUninit<brvk::VkMemoryRequirements2KHR>,
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
        sink: &mut [core::mem::MaybeUninit<brvk::VkSparseImageMemoryRequirements2KHR>],
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
    ) -> Vec<brvk::VkSparseImageMemoryRequirements2KHR> {
        let n = self.get_image_sparse_memory_requirements2_count_khr(info);
        if n == 0 {
            return crate::alloc::empty_sink_buffer();
        }

        let mut buf = crate::alloc::reserve(n as _);
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
    ForwardFnPtr!(deref get_buffer_memory_requirements_2_khr_fn -> brvk::PFN_vkGetBufferMemoryRequirements2KHR);
    #[implements]
    ForwardFnPtr!(deref get_image_memory_requirements_2_khr_fn -> brvk::PFN_vkGetImageMemoryRequirements2KHR);
    #[implements]
    ForwardFnPtr!(deref get_image_sparse_memory_requirements_2_khr_fn -> brvk::PFN_vkGetImageSparseMemoryRequirements2KHR);
});
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<Instance: crate::Instance> DeviceGetMemoryRequirements2Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_buffer_memory_requirements_2_khr_fn(&self) -> brvk::PFN_vkGetBufferMemoryRequirements2KHR {
        *self.ext.get_buffer_memory_requirements_2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn get_image_memory_requirements_2_khr_fn(&self) -> brvk::PFN_vkGetImageMemoryRequirements2KHR {
        *self.ext.get_image_memory_requirements_2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn get_image_sparse_memory_requirements_2_khr_fn(&self) -> brvk::PFN_vkGetImageSparseMemoryRequirements2KHR {
        *self.ext.get_image_sparse_memory_requirements_2_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_bind_memory2")]
pub trait DeviceBindMemory2Extension: Device {
    #[implements]
    fn bind_buffer_memory2_khr_fn(&self) -> brvk::PFN_vkBindBufferMemory2KHR;
    #[implements]
    fn bind_image_memory2_khr_fn(&self) -> brvk::PFN_vkBindImageMemory2KHR;

    /// Multiple Binding for Buffers
    #[implements]
    #[inline]
    fn bind_buffer_memory2_khr(&self, bounds: &[BindBufferMemoryInfo]) -> crate::Result<()> {
        translate_vk_result(unsafe {
            self.bind_buffer_memory2_khr_fn().0(
                self.native_ptr(),
                bounds.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(bounds).cast(),
            )
        })?;

        Ok(())
    }

    /// Multiple Binding for Images
    #[implements]
    #[inline]
    fn bind_image_memory2_khr(&self, bounds: &[BindImageMemoryInfo]) -> crate::Result<()> {
        translate_vk_result(unsafe {
            self.bind_image_memory2_khr_fn().0(
                self.native_ptr(),
                bounds.len() as _,
                crate::ffi_helper::slice_as_ptr_empty_null(bounds).cast(),
            )
        })?;

        Ok(())
    }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
DerefContainerWithGuardsBracketImpl!(for DeviceBindMemory2Extension {
    #[implements]
    ForwardFnPtr!(deref bind_buffer_memory2_khr_fn -> brvk::PFN_vkBindBufferMemory2KHR);
    #[implements]
    ForwardFnPtr!(deref bind_image_memory2_khr_fn -> brvk::PFN_vkBindImageMemory2KHR);
});
#[cfg(feature = "VK_KHR_bind_memory2")]
impl<Instance: crate::Instance> DeviceBindMemory2Extension for DeviceObject<Instance> {
    #[implements]
    fn bind_buffer_memory2_khr_fn(&self) -> brvk::PFN_vkBindBufferMemory2KHR {
        *self.ext.bind_buffer_memory2_khr.resolve()
    }
    #[implements]
    fn bind_image_memory2_khr_fn(&self) -> brvk::PFN_vkBindImageMemory2KHR {
        *self.ext.bind_image_memory2_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_external_fence_fd")]
pub trait DeviceExternalFenceFdExtension: Device {
    #[implements]
    fn get_fence_fd_khr_fn(&self) -> brvk::PFN_vkGetFenceFdKHR;
    #[implements]
    fn import_fence_fd_khr_fn(&self) -> brvk::PFN_vkImportFenceFdKHR;

    /// Get a POSIX file descriptor handle for a type
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_TOO_MANY_OBJECTS`
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements]
    #[inline]
    fn get_fence_fd(&self, info: &crate::FenceFdGetInfo) -> crate::Result<std::os::unix::io::RawFd> {
        let mut fd = core::mem::MaybeUninit::uninit();
        crate::error::translate_vk_result(unsafe {
            self.get_fence_fd_khr_fn().0(self.native_ptr(), &info.0, fd.as_mut_ptr())
        })?;

        Ok(unsafe { fd.assume_init() })
    }

    /// Import a fence from a POSIX file descriptor
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements]
    #[inline]
    fn import_fence_fd(&self, info: &crate::ImportFenceFdInfo) -> crate::Result<()> {
        crate::error::translate_vk_result(unsafe { self.import_fence_fd_khr_fn().0(self.native_ptr(), &info.0) })?;

        Ok(())
    }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
DerefContainerBracketImpl!(for DeviceExternalFenceFdExtension {
    #[implements]
    ForwardFnPtr!(deref get_fence_fd_khr_fn -> brvk::PFN_vkGetFenceFdKHR);
    #[implements]
    ForwardFnPtr!(deref import_fence_fd_khr_fn -> brvk::PFN_vkImportFenceFdKHR);
});
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl<Instance: crate::Instance> DeviceExternalFenceFdExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_fence_fd_khr_fn(&self) -> brvk::PFN_vkGetFenceFdKHR {
        *self.ext.get_fence_fd_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn import_fence_fd_khr_fn(&self) -> brvk::PFN_vkImportFenceFdKHR {
        *self.ext.import_fence_fd_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub trait DeviceExternalSemaphoreWin32Extension: Device {
    #[implements]
    fn import_semaphore_win32_handle_khr_fn(&self) -> brvk::PFN_vkImportSemaphoreWin32HandleKHR;
    #[implements]
    fn get_semaphore_win32_handle_khr_fn(&self) -> brvk::PFN_vkGetSemaphoreWin32HandleKHR;

    /// Import a semaphore from a Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// * brvk::VK_ERROR_INVALID_EXTERNAL_HANDLE
    #[implements]
    #[inline]
    fn import_semaphore_win32_handle(&self, info: &crate::ImportSemaphoreWin32HandleInfo) -> crate::Result<()> {
        translate_vk_result(unsafe { self.import_semaphore_win32_handle_khr_fn().0(self.native_ptr(), &info.0) })?;

        Ok(())
    }

    /// Get a Windows HANDLE for a semaphore
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * brvk::VK_ERROR_TOO_MANY_OBJECTS
    /// * brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    #[implements]
    #[inline]
    fn get_semaphore_win32_handle(
        &self,
        info: &crate::SemaphoreGetWin32HandleInfo,
    ) -> crate::Result<windows::Win32::Foundation::HANDLE> {
        let mut handle = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            self.get_semaphore_win32_handle_khr_fn().0(self.native_ptr(), &info.0, handle.as_mut_ptr())
        })?;

        Ok(unsafe { handle.assume_init() })
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
DerefContainerWithGuardsBracketImpl!(for DeviceExternalSemaphoreWin32Extension {
    #[implements]
    ForwardFnPtr!(deref import_semaphore_win32_handle_khr_fn -> brvk::PFN_vkImportSemaphoreWin32HandleKHR);
    #[implements]
    ForwardFnPtr!(deref get_semaphore_win32_handle_khr_fn -> brvk::PFN_vkGetSemaphoreWin32HandleKHR);
});
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<Instance: crate::Instance> DeviceExternalSemaphoreWin32Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn import_semaphore_win32_handle_khr_fn(&self) -> brvk::PFN_vkImportSemaphoreWin32HandleKHR {
        *self.ext.import_semaphore_win32_handle_khr.resolve()
    }

    #[implements]
    #[inline(always)]
    fn get_semaphore_win32_handle_khr_fn(&self) -> brvk::PFN_vkGetSemaphoreWin32HandleKHR {
        *self.ext.get_semaphore_win32_handle_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_external_memory_fd")]
pub trait DeviceExternalMemoryFdExtension: Device {
    #[implements]
    fn get_memory_fd_khr_fn(&self) -> brvk::PFN_vkGetMemoryFdKHR;
    #[implements]
    fn get_memory_fd_properties_khr_fn(&self) -> brvk::PFN_vkGetMemoryFdPropertiesKHR;

    /// Get a POSIX file descriptor for a memory object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_TOO_MANY_OBJECTS`
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements]
    #[inline]
    fn get_memory_fd(&self, info: &crate::MemoryGetFdInfo) -> crate::Result<std::os::unix::io::RawFd> {
        let mut fd = core::mem::MaybeUninit::uninit();
        crate::error::translate_vk_result(unsafe {
            self.get_memory_fd_khr_fn().0(self.native_ptr(), &info.0, fd.as_mut_ptr())
        })?;

        Ok(unsafe { fd.assume_init() })
    }

    /// Get Properties of External Memory File Descriptors
    /// # Safety
    /// sink must be constructed correctly
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements]
    #[inline]
    unsafe fn memory_fd_properties(
        &self,
        handle_type: crate::ExternalMemoryHandleTypeFd,
        handle: std::os::unix::io::RawFd,
        sink: &mut core::mem::MaybeUninit<brvk::VkMemoryFdPropertiesKHR>,
    ) -> crate::Result<()> {
        crate::error::translate_vk_result(unsafe {
            self.get_memory_fd_properties_khr_fn().0(self.native_ptr(), handle_type as _, handle, sink.as_mut_ptr())
        })?;

        Ok(())
    }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
DerefContainerWithGuardsBracketImpl!(for DeviceExternalMemoryFdExtension {
    #[implements]
    ForwardFnPtr!(deref get_memory_fd_khr_fn -> brvk::PFN_vkGetMemoryFdKHR);
    #[implements]
    ForwardFnPtr!(deref get_memory_fd_properties_khr_fn -> brvk::PFN_vkGetMemoryFdPropertiesKHR);
});
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl<Instance: crate::Instance> DeviceExternalMemoryFdExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_memory_fd_khr_fn(&self) -> brvk::PFN_vkGetMemoryFdKHR {
        *self.ext.get_memory_fd_khr.resolve()
    }

    #[implements]
    #[inline(always)]
    fn get_memory_fd_properties_khr_fn(&self) -> brvk::PFN_vkGetMemoryFdPropertiesKHR {
        *self.ext.get_memory_fd_properties_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_external_memory_host")]
pub trait DeviceExternalMemoryHostExtension: Device {
    #[implements]
    fn get_memory_host_pointer_properties_ext_fn(&self) -> brvk::PFN_vkGetMemoryHostPointerPropertiesEXT;

    /// Get Properties of external memory host pointer
    /// # Safety
    /// sink must be constructed correctly
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements]
    #[inline]
    unsafe fn memory_host_pointer_properties(
        &self,
        handle_type: crate::ExternalMemoryHandleTypeHost,
        ptr: *mut core::ffi::c_void,
        sink: &mut core::mem::MaybeUninit<brvk::VkMemoryHostPointerPropertiesEXT>,
    ) -> crate::Result<()> {
        crate::error::translate_vk_result(unsafe {
            self.get_memory_host_pointer_properties_ext_fn().0(
                self.native_ptr(),
                handle_type as _,
                ptr,
                sink.as_mut_ptr(),
            )
        })?;

        Ok(())
    }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
DerefContainerWithGuardsBracketImpl!(for DeviceExternalMemoryHostExtension {
    #[implements]
    ForwardFnPtr!(deref get_memory_host_pointer_properties_ext_fn -> brvk::PFN_vkGetMemoryHostPointerPropertiesEXT);
});
#[cfg(feature = "VK_EXT_external_memory_host")]
impl<Instance: crate::Instance> DeviceExternalMemoryHostExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_memory_host_pointer_properties_ext_fn(&self) -> brvk::PFN_vkGetMemoryHostPointerPropertiesEXT {
        *self.ext.get_memory_host_pointer_properties_ext.resolve()
    }
}

#[cfg(feature = "VK_KHR_external_memory_win32")]
impl<Instance: crate::Instance> DeviceExternalMemoryWin32Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_memory_win32_handle_khr_fn(&self) -> brvk::PFN_vkGetMemoryWin32HandleKHR {
        *self.ext.get_memory_win32_handle_khr.resolve()
    }

    #[implements]
    #[inline(always)]
    fn get_memory_win32_handle_properties_khr_fn(&self) -> brvk::PFN_vkGetMemoryWin32HandlePropertiesKHR {
        *self.ext.get_memory_win32_handle_properties_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub trait DeviceFullScreenExclusiveExtension: Device {
    #[implements]
    fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> brvk::PFN_vkAcquireFullScreenExclusiveModeEXT;
    #[implements]
    fn release_full_screen_exclusive_mode_ext_fn(&self) -> brvk::PFN_vkReleaseFullScreenExclusiveModeEXT;

    /// # Safety
    ///
    /// `swapchain` must be created from `device`.
    #[implements]
    #[inline(always)]
    unsafe fn acquire_full_screen_exclusive_mode(
        &self,
        device: VkHandleRef<brvk::VkDevice>,
        swapchain: VkHandleRef<brvk::VkSwapchainKHR>,
    ) -> crate::Result<()> {
        translate_vk_result(unsafe { self.acquire_full_screen_exclusive_mode_ext_fn().0(device.0, swapchain.0) })?;

        Ok(())
    }

    /// # Safety
    ///
    /// `swapchain` must be created from `device`.
    #[implements]
    #[inline(always)]
    unsafe fn release_full_screen_exclusive_mode(
        &self,
        device: VkHandleRef<brvk::VkDevice>,
        swapchain: VkHandleRef<brvk::VkSwapchainKHR>,
    ) -> crate::Result<()> {
        translate_vk_result(unsafe { self.release_full_screen_exclusive_mode_ext_fn().0(device.0, swapchain.0) })?;

        Ok(())
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
DerefContainerWithGuardsBracketImpl!(for DeviceFullScreenExclusiveExtension {
    #[implements]
    ForwardFnPtr!(deref acquire_full_screen_exclusive_mode_ext_fn -> brvk::PFN_vkAcquireFullScreenExclusiveModeEXT);
    #[implements]
    ForwardFnPtr!(deref release_full_screen_exclusive_mode_ext_fn -> brvk::PFN_vkReleaseFullScreenExclusiveModeEXT);
});
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl<Instance: crate::Instance> DeviceFullScreenExclusiveExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> brvk::PFN_vkAcquireFullScreenExclusiveModeEXT {
        *self.ext.acquire_full_screen_exclusive_mode_ext.resolve()
    }

    #[implements]
    #[inline(always)]
    fn release_full_screen_exclusive_mode_ext_fn(&self) -> brvk::PFN_vkReleaseFullScreenExclusiveModeEXT {
        *self.ext.release_full_screen_exclusive_mode_ext.resolve()
    }
}

#[cfg(feature = "VK_KHR_create_renderpass2")]
pub trait DeviceCreateRenderPass2Extension: Device {
    #[implements]
    fn create_render_pass_2_khr_fn(&self) -> brvk::PFN_vkCreateRenderPass2KHR;
    #[implements]
    fn cmd_begin_render_pass_2_khr_fn(&self) -> brvk::PFN_vkCmdBeginRenderPass2KHR;
    #[implements]
    fn cmd_end_render_pass_2_khr_fn(&self) -> brvk::PFN_vkCmdEndRenderPass2KHR;
    #[implements]
    fn cmd_next_subpass_2_khr_fn(&self) -> brvk::PFN_vkCmdNextSubpass2KHR;

    /// Create a new render pass object
    ///
    /// # Failures
    ///
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn new_render_pass2_khr(
        &self,
        info: &RenderPassCreateInfo2,
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkRenderPass> {
        let mut h = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            (self.create_render_pass_2_khr_fn().0)(
                self.native_ptr(),
                info as *const _ as _,
                crate::ffi_helper::opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
        })?;

        Ok(unsafe { h.assume_init() })
    }

    /// Begin a new render pass
    ///
    /// # Safety
    ///
    /// Host access to the `VkCommandPool` that `command_buffer` was allocated from must be externally synchronized.
    #[implements]
    #[inline(always)]
    unsafe fn cmd_begin_render_pass2_khr(
        &self,
        command_buffer: &mut (impl VkHandleMut<Handle = brvk::VkCommandBuffer> + ?Sized),
        info: &RenderPassBeginInfo,
        subpass_info: &SubpassBeginInfo,
    ) {
        unsafe {
            self.cmd_begin_render_pass_2_khr_fn().0(
                command_buffer.native_ptr_mut(),
                core::ptr::from_ref(info).cast(),
                core::ptr::from_ref(subpass_info).cast(),
            )
        }
    }

    /// End the current render pass
    ///
    /// # Safety
    ///
    /// Host access to the `VkCommandPool` that `command_buffer` was allocated from must be externally synchronized.
    #[implements]
    #[inline(always)]
    unsafe fn cmd_end_render_pass2_khr(
        &self,
        command_buffer: &mut (impl VkHandleMut<Handle = brvk::VkCommandBuffer> + ?Sized),
        subpass_info: &SubpassEndInfo,
    ) {
        unsafe {
            self.cmd_end_render_pass_2_khr_fn().0(command_buffer.native_ptr(), core::ptr::from_ref(subpass_info).cast())
        }
    }

    /// Transition to the next subpass of a render pass
    ///
    /// # Safety
    ///
    /// Host access to the `VkCommandPool` that `command_buffer` was allocated from must be externally synchronized.
    #[implements]
    #[inline(always)]
    unsafe fn cmd_next_subpass2_khr(
        &self,
        command_buffer: &mut (impl VkHandleMut<Handle = brvk::VkCommandBuffer> + ?Sized),
        begin_info: &SubpassBeginInfo,
        end_info: &SubpassEndInfo,
    ) {
        unsafe {
            self.cmd_next_subpass_2_khr_fn().0(
                command_buffer.native_ptr(),
                core::ptr::from_ref(begin_info).cast(),
                core::ptr::from_ref(end_info).cast(),
            )
        }
    }
}
#[cfg(feature = "VK_KHR_create_renderpass2")]
DerefContainerWithGuardsBracketImpl!(for DeviceCreateRenderPass2Extension {
    #[implements]
    ForwardFnPtr!(deref create_render_pass_2_khr_fn -> brvk::PFN_vkCreateRenderPass2KHR);
    #[implements]
    ForwardFnPtr!(deref cmd_begin_render_pass_2_khr_fn -> brvk::PFN_vkCmdBeginRenderPass2KHR);
    #[implements]
    ForwardFnPtr!(deref cmd_end_render_pass_2_khr_fn -> brvk::PFN_vkCmdEndRenderPass2KHR);
    #[implements]
    ForwardFnPtr!(deref cmd_next_subpass_2_khr_fn -> brvk::PFN_vkCmdNextSubpass2KHR);
});
#[cfg(feature = "VK_KHR_create_renderpass2")]
impl<Instance: crate::Instance> DeviceCreateRenderPass2Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn create_render_pass_2_khr_fn(&self) -> brvk::PFN_vkCreateRenderPass2KHR {
        *self.ext.create_render_pass_2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn cmd_begin_render_pass_2_khr_fn(&self) -> brvk::PFN_vkCmdBeginRenderPass2KHR {
        *self.ext.cmd_begin_render_pass_2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn cmd_end_render_pass_2_khr_fn(&self) -> brvk::PFN_vkCmdEndRenderPass2KHR {
        *self.ext.cmd_end_render_pass_2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn cmd_next_subpass_2_khr_fn(&self) -> brvk::PFN_vkCmdNextSubpass2KHR {
        *self.ext.cmd_next_subpass_2_khr.resolve()
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
pub trait DeviceSynchronization2Extension: Device {
    #[implements]
    fn queue_submit2_khr_fn(&self) -> brvk::PFN_vkQueueSubmit2KHR;
    #[implements]
    fn cmd_pipeline_barrier_2_khr_fn(&self) -> brvk::PFN_vkCmdPipelineBarrier2KHR;
}
#[cfg(feature = "VK_KHR_synchronization2")]
DerefContainerWithGuardsBracketImpl!(for DeviceSynchronization2Extension {
    #[implements]
    ForwardFnPtr!(deref queue_submit2_khr_fn -> brvk::PFN_vkQueueSubmit2KHR);
    #[implements]
    ForwardFnPtr!(deref cmd_pipeline_barrier_2_khr_fn -> brvk::PFN_vkCmdPipelineBarrier2KHR);
});
#[cfg(feature = "VK_KHR_synchronization2")]
impl<Instance: crate::Instance> DeviceSynchronization2Extension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn queue_submit2_khr_fn(&self) -> brvk::PFN_vkQueueSubmit2KHR {
        *self.ext.queue_submit2_khr.resolve()
    }
    #[implements]
    #[inline(always)]
    fn cmd_pipeline_barrier_2_khr_fn(&self) -> brvk::PFN_vkCmdPipelineBarrier2KHR {
        *self.ext.cmd_pipeline_barrier_2_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
pub trait DeviceImageDrmFormatModifierExtension: Device {
    #[implements]
    fn get_image_drm_format_modifier_properties_ext_fn(&self) -> brvk::PFN_vkGetImageDrmFormatModifierPropertiesEXT;
}
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
DerefContainerWithGuardsBracketImpl!(for DeviceImageDrmFormatModifierExtension {
    #[implements]
    ForwardFnPtr!(deref get_image_drm_format_modifier_properties_ext_fn -> brvk::PFN_vkGetImageDrmFormatModifierPropertiesEXT);
});
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
impl<Instance: crate::Instance> DeviceImageDrmFormatModifierExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_image_drm_format_modifier_properties_ext_fn(&self) -> brvk::PFN_vkGetImageDrmFormatModifierPropertiesEXT {
        *self.ext.get_image_drm_format_modifier_properties_ext.resolve()
    }
}

#[cfg(feature = "VK_KHR_push_descriptor")]
pub trait DevicePushDescriptorExtension: Device {
    #[implements]
    fn cmd_push_descriptor_set_khr_fn(&self) -> brvk::PFN_vkCmdPushDescriptorSetKHR;
}
#[cfg(feature = "VK_KHR_push_descriptor")]
DerefContainerWithGuardsBracketImpl!(for DevicePushDescriptorExtension {
    #[implements("VK_KHR_push_descriptor")]
    ForwardFnPtr!(deref cmd_push_descriptor_set_khr_fn -> brvk::PFN_vkCmdPushDescriptorSetKHR);
});
#[cfg(feature = "VK_KHR_push_descriptor")]
impl<Instance: crate::Instance> DevicePushDescriptorExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn cmd_push_descriptor_set_khr_fn(&self) -> brvk::PFN_vkCmdPushDescriptorSetKHR {
        *self.ext.cmd_push_descriptor_set_khr.resolve()
    }
}

#[cfg(feature = "VK_EXT_sample_locations")]
pub trait DeviceSampleLocationsExtension: Device {
    #[implements]
    fn cmd_set_sample_locations_ext_fn(&self) -> brvk::PFN_vkCmdSetSampleLocationsEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
DerefContainerWithGuardsBracketImpl!(for DeviceSampleLocationsExtension {
    #[implements]
    ForwardFnPtr!(deref cmd_set_sample_locations_ext_fn -> brvk::PFN_vkCmdSetSampleLocationsEXT);
});
#[cfg(feature = "VK_EXT_sample_locations")]
impl<Instance: crate::Instance> DeviceSampleLocationsExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn cmd_set_sample_locations_ext_fn(&self) -> brvk::PFN_vkCmdSetSampleLocationsEXT {
        *self.ext.cmd_set_sample_locations_ext.resolve()
    }
}

#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub trait DeviceTimelineSemaphoreExtension: Device {
    #[implements]
    fn get_semaphore_counter_value_ext_fn(&self) -> brvk::PFN_vkGetSemaphoreCounterValueKHR;
    #[implements]
    fn signal_semaphore_ext_fn(&self) -> brvk::PFN_vkSignalSemaphoreKHR;
    #[implements]
    fn wait_semaphores_ext_fn(&self) -> brvk::PFN_vkWaitSemaphoresKHR;

    /// Query the current state of a timeline semaphore
    /// # Failure
    ///
    /// * [`brvk::VK_ERROR_DEVICE_LOST`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_UNKNOWN`]
    /// * [`brvk::VK_ERROR_VALIDATION_FAILED`]
    #[implements]
    fn get_semaphore_counter_value_khr(
        &self,
        semaphore: &(impl VkHandle<Handle = brvk::VkSemaphore> + ?Sized),
    ) -> crate::Result<u64> {
        let mut sink = core::mem::MaybeUninit::uninit();
        translate_vk_result(unsafe {
            (self.get_semaphore_counter_value_ext_fn().0)(self.native_ptr(), semaphore.native_ptr(), sink.as_mut_ptr())
        })?;

        Ok(unsafe { sink.assume_init() })
    }

    /// Signal a timeline semaphore on the host
    /// # Failure
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_UNKNOWN`]
    /// * [`brvk::VK_ERROR_VALIDATION_FAILED`]
    #[implements]
    fn signal_semaphore_khr(&self, info: &SemaphoreSignalInfo) -> crate::Result<()> {
        translate_vk_result(unsafe { (self.signal_semaphore_ext_fn().0)(self.native_ptr(), info as *const _ as _) })?;

        Ok(())
    }

    /// Wait for timeline semaphores on the host
    ///
    /// Returns `false` if timed out
    /// # Failure
    ///
    /// * [`brvk::VK_ERROR_DEVICE_LOST`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_UNKNOWN`]
    /// * [`brvk::VK_ERROR_VALIDATION_FAILED`]
    #[implements]
    fn wait_semaphores_khr(&self, info: &SemaphoreWaitInfo, timeout: u64) -> crate::Result<TimeoutableWaitResult> {
        translate_vk_result(unsafe {
            (self.wait_semaphores_ext_fn().0)(self.native_ptr(), info as *const _ as _, timeout)
        })
        .map(TimeoutableWaitResult::from_vk_result)
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
DerefContainerWithGuardsBracketImpl!(for DeviceTimelineSemaphoreExtension {
    #[implements]
    ForwardFnPtr!(deref get_semaphore_counter_value_ext_fn -> brvk::PFN_vkGetSemaphoreCounterValueKHR);
    #[implements]
    ForwardFnPtr!(deref signal_semaphore_ext_fn -> brvk::PFN_vkSignalSemaphoreKHR);
    #[implements]
    ForwardFnPtr!(deref wait_semaphores_ext_fn -> brvk::PFN_vkWaitSemaphoresKHR);
});
#[cfg(feature = "VK_KHR_timeline_semaphore")]
impl<Instance: crate::Instance> DeviceTimelineSemaphoreExtension for DeviceObject<Instance> {
    #[implements]
    #[inline(always)]
    fn get_semaphore_counter_value_ext_fn(&self) -> brvk::PFN_vkGetSemaphoreCounterValueKHR {
        *self.ext.get_semaphore_counter_value_ext.resolve()
    }

    #[implements]
    #[inline(always)]
    fn signal_semaphore_ext_fn(&self) -> brvk::PFN_vkSignalSemaphoreKHR {
        *self.ext.signal_semaphore_ext.resolve()
    }

    #[implements]
    #[inline(always)]
    fn wait_semaphores_ext_fn(&self) -> brvk::PFN_vkWaitSemaphoresKHR {
        *self.ext.wait_semaphores_ext.resolve()
    }
}

/// Child of a device object(raw handle)
pub trait DeviceChildHandle {
    /// Retrieve a reference to a device handle that creates this objecs
    fn device_handle(&self) -> brvk::VkDevice;

    #[inline(always)]
    fn device_transparent_ref<'a>(&'a self) -> VkHandleRef<'a, brvk::VkDevice> {
        unsafe { VkHandleRef::dangling(self.device_handle()) }
    }
}
DerefContainerBracketImpl!(for DeviceChildHandle {
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice { T::device_handle(self) }
});
GuardsImpl!(for DeviceChildHandle {
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice { T::device_handle(self) }
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
#[VkObject(type = brvk::VK_OBJECT_TYPE_QUEUE)]
pub struct QueueObject<Device>(brvk::VkQueue, Device);
unsafe impl<Device: Sync> Sync for QueueObject<Device> {}
unsafe impl<Device: Send> Send for QueueObject<Device> {}
impl<Device: crate::Device> Queue for QueueObject<Device> {}
impl<Device: crate::Device> QueueMut for QueueObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceChildHandle for QueueObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice {
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
    pub const unsafe fn manage(handle: brvk::VkQueue, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (brvk::VkQueue, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}

pub trait Queue: VkHandle<Handle = brvk::VkQueue> + DeviceChild {}
DerefContainerBracketImpl!(for Queue {});
GuardsImpl!(for Queue {});

pub trait QueueMut: Queue + VkHandleMut {
    /// Wait for a object to become idle
    #[implements]
    #[inline(always)]
    fn wait(&mut self) -> crate::Result<()> {
        crate::vkfn_wrapper::queue_wait_idle(self.as_transparent_ref_mut())
    }

    /// Submits a sequence of semaphores or command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_DEVICE_LOST`
    #[implements]
    #[inline(always)]
    fn submit(&mut self, batches: &[SubmitInfo], fence: Option<VkHandleRefMut<brvk::VkFence>>) -> crate::Result<()> {
        crate::vkfn_wrapper::queue_submit(self.as_transparent_ref_mut(), batches, fence)
    }

    /// Bind device memory to a sparse resource object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_DEVICE_LOST`
    #[implements]
    #[inline(always)]
    fn bind_sparse(
        &mut self,
        batches: &[BindSparseInfo],
        fence: Option<VkHandleRefMut<brvk::VkFence>>,
    ) -> crate::Result<()> {
        crate::vkfn_wrapper::queue_bind_sparse(self.as_transparent_ref_mut(), batches, fence)
    }

    /// Queue images for presentation
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_DEVICE_LOST`]
    /// * [`brvk::VK_ERROR_OUT_OF_DATE_KHR`]
    /// * [`brvk::VK_ERROR_SURFACE_LOST_KHR`]
    #[implements("VK_KHR_swapchain")]
    #[inline(always)]
    fn present<'r>(&mut self, info: &PresentInfo<'r>) -> crate::Result<PresentResult> {
        crate::vkfn_wrapper::queue_present(self.as_transparent_ref_mut(), info)
    }

    /// Submits command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_DEVICE_LOST`]
    #[implements("VK_KHR_synchronization2")]
    fn submit2_khr(
        &mut self,
        device: &(impl DeviceSynchronization2Extension + ?Sized),
        batches: &[SubmitInfo2],
        fence: Option<VkHandleRefMut<brvk::VkFence>>,
    ) -> crate::Result<()> {
        translate_vk_result(unsafe {
            (device.queue_submit2_khr_fn().0)(
                self.native_ptr_mut(),
                batches.len() as _,
                slice_as_ptr_empty_null(batches).cast(),
                fence.map(|x| x.0),
            )
        })?;

        Ok(())
    }

    /// Submits command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_DEVICE_LOST`]
    #[implements("Allow1_3APIs")]
    #[inline(always)]
    fn submit2(&mut self, batches: &[SubmitInfo2], fence: Option<VkHandleRefMut<brvk::VkFence>>) -> crate::Result<()> {
        crate::vkfn_wrapper::queue_submit2(self.as_transparent_ref_mut(), batches, fence)
    }

    // --- DEPRECATED APIS ---

    /// Bind device memory to a sparse resource object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_DEVICE_LOST`
    #[deprecated = "use `bind_sparse` with creating `BindSparseInfo`"]
    #[implements("alloc")]
    fn bind_sparse_ops(
        &mut self,
        batches: &[impl SparseBindingOpBatch],
        fence: Option<VkHandleRefMut<brvk::VkFence>>,
    ) -> crate::Result<()> {
        self.bind_sparse(
            &crate::alloc::collect_vec(batches.iter().map(SparseBindingOpBatch::make_info_struct)),
            fence,
        )
    }

    /// Submits a sequence of semaphores or command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_DEVICE_LOST`
    #[deprecated = "use `submit` with creating `SubmitInfo"]
    #[allow(deprecated)]
    #[implements("alloc")]
    fn submit_ops(
        &mut self,
        batches: &[impl SubmissionBatch],
        fence: Option<VkHandleRefMut<brvk::VkFence>>,
    ) -> crate::Result<()> {
        let batch_resources: Vec<_> = crate::alloc::collect_vec(batches.iter().map(|b| {
            let mut resources = TemporalSubmissionBatchResources::new();
            b.collect_resources(&mut resources);
            resources
        }));

        self.submit(
            &crate::alloc::collect_vec(
                batch_resources
                    .iter()
                    .map(TemporalSubmissionBatchResources::make_info_struct),
            ),
            fence,
        )
    }
}
DerefContainerBracketImpl!(for mut QueueMut {});
GuardsImpl!(for mut QueueMut {});

#[cfg(feature = "VK_KHR_swapchain")]
#[repr(transparent)]
pub struct PresentInfo<'r>(
    brvk::VkPresentInfoKHR,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        &'r [brvk::VkSwapchainKHR],
        &'r [brvk::VkSemaphore],
        &'r [u32],
        &'r mut [brvk::VkResult],
    )>,
);
#[cfg(feature = "VK_KHR_swapchain")]
impl<'r> PresentInfo<'r> {
    #[inline(always)]
    pub fn new(
        wait_semaphores: &'r [VkHandleRef<brvk::VkSemaphore>],
        swapchains: &'r [VkHandleRef<brvk::VkSwapchainKHR>],
        image_indices: &'r [u32],
        results: &'r mut [brvk::VkResult],
    ) -> Self {
        assert_eq!(swapchains.len(), image_indices.len());
        assert_eq!(swapchains.len(), results.len());

        Self(
            brvk::VkPresentInfoKHR {
                sType: brvk::VkPresentInfoKHR::TYPE,
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
    pub const unsafe fn from_raw(raw: brvk::VkPresentInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPresentInfoKHR {
        self.0
    }

    #[implements]
    #[inline(always)]
    pub fn submit(&self, queue: &mut (impl QueueMut + ?Sized)) -> crate::Result<PresentResult> {
        queue.present(self)
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CommandBufferSubmitInfo<'r>(
    brvk::VkCommandBufferSubmitInfoKHR,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = brvk::VkCommandBuffer>>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'r> CommandBufferSubmitInfo<'r> {
    #[inline(always)]
    pub fn new(command_buffer: &'r (impl VkHandle<Handle = brvk::VkCommandBuffer> + ?Sized)) -> Self {
        Self(
            brvk::VkCommandBufferSubmitInfoKHR {
                sType: brvk::VkCommandBufferSubmitInfoKHR::TYPE,
                pNext: core::ptr::null(),
                commandBuffer: command_buffer.native_ptr(),
                deviceMask: 0,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkCommandBufferSubmitInfoKHR`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkCommandBufferSubmitInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkCommandBufferSubmitInfoKHR {
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
    brvk::VkSubmitInfo2KHR,
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
            brvk::VkSubmitInfo2KHR {
                sType: brvk::VkSubmitInfo2KHR::TYPE,
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
    /// `raw` must be a valid [`brvk::VkSubmitInfo2KHR`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkSubmitInfo2KHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }
    pub const fn into_raw(self) -> brvk::VkSubmitInfo2KHR {
        self.0
    }

    pub const fn protected(mut self) -> Self {
        self.0.flags |= brvk::VK_SUBMIT_PROTECTED_BIT_KHR;
        self
    }
}
