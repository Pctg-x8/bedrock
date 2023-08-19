//! Vulkan Device and Queues

use cfg_if::cfg_if;

#[cfg(feature = "Implements")]
use crate::{
    fnconv::FnTransmute, DescriptorSetCopyInfo, DescriptorSetWriteInfo, VkHandleMut, VkRawHandle, VulkanStructure,
    VulkanStructureProvider,
};
use crate::{
    vk::*, InstanceChild, SparseBindingOpBatch, SubmissionBatch, TemporalSubmissionBatchResources, VkHandle, VkObject,
    VkResultBox,
};

cfg_if! {
    if #[cfg(feature = "Implements")] {
        type DeviceResolvedFn<F> = crate::vkresolve::ResolvedFnCell<F, VkDevice>;
        impl crate::vkresolve::ResolverInterface for VkDevice {
            unsafe fn load_symbol_unconstrainted<T: crate::vkresolve::FromPtr>(&self, name: &[u8]) -> T {
                T::from_ptr(core::mem::transmute(crate::vkresolve::get_device_proc_addr(
                    *self,
                    name.as_ptr() as _,
                )))
            }

            unsafe fn load_function_unconstrainted<F: crate::vkresolve::PFN>(&self, name: &[u8]) -> F {
                F::from_void_fn(
                    crate::vkresolve::get_device_proc_addr(*self, name.as_ptr() as _)
                        .unwrap_or_else(|| panic!("function {:?} not found", name))
                )
            }
        }
    }
}

/// Opaque handle to a device object
#[derive(VkHandle, VkObject, InstanceChild)]
#[VkObject(type = VK_OBJECT_TYPE_DEVICE)]
pub struct DeviceObject<Instance: crate::Instance> {
    #[handle]
    handle: VkDevice,
    #[parent]
    parent: Instance,
    #[cfg(all(feature = "VK_KHR_maintenance1", feature = "Implements"))]
    trim_command_pool_khr: DeviceResolvedFn<PFN_vkTrimCommandPoolKHR>,
    #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))]
    create_descriptor_update_template_khr: DeviceResolvedFn<PFN_vkCreateDescriptorUpdateTemplateKHR>,
    #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))]
    destroy_descriptor_update_template_khr: DeviceResolvedFn<PFN_vkDestroyDescriptorUpdateTemplateKHR>,
    #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))]
    update_descriptor_set_with_template_khr: DeviceResolvedFn<PFN_vkUpdateDescriptorSetWithTemplateKHR>,
    #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements"))]
    bind_buffer_memory2_khr: DeviceResolvedFn<PFN_vkBindBufferMemory2KHR>,
    #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements"))]
    bind_image_memory2_khr: DeviceResolvedFn<PFN_vkBindImageMemory2KHR>,
    #[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "Implements"))]
    get_image_drm_format_modifier_properties_ext: DeviceResolvedFn<PFN_vkGetImageDrmFormatModifierPropertiesEXT>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
    get_fence_fd_khr: DeviceResolvedFn<PFN_vkGetFenceFdKHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
    import_fence_fd_khr: DeviceResolvedFn<PFN_vkImportFenceFdKHR>,
    #[cfg(all(feature = "Implements", feature = "VK_EXT_full_screen_exclusive"))]
    acquire_full_screen_exclusive_mode_ext: DeviceResolvedFn<PFN_vkAcquireFullScreenExclusiveModeEXT>,
    #[cfg(all(feature = "Implements", feature = "VK_EXT_full_screen_exclusive"))]
    release_full_screen_exclusive_mode_ext: DeviceResolvedFn<PFN_vkReleaseFullScreenExclusiveModeEXT>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))]
    get_memory_fd_khr: DeviceResolvedFn<PFN_vkGetMemoryFdKHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))]
    get_memory_fd_properties_khr: DeviceResolvedFn<PFN_vkGetMemoryFdPropertiesKHR>,
    #[cfg(all(feature = "Implements", feature = "VK_EXT_external_memory_host"))]
    get_memory_host_pointer_properties_ext: DeviceResolvedFn<PFN_vkGetMemoryHostPointerPropertiesEXT>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))]
    import_semaphore_win32_handle_khr: DeviceResolvedFn<PFN_vkImportSemaphoreWin32HandleKHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))]
    get_semaphore_win32_handle_khr: DeviceResolvedFn<PFN_vkGetSemaphoreWin32HandleKHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))]
    get_memory_win32_handle_khr: DeviceResolvedFn<PFN_vkGetMemoryWin32HandleKHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))]
    get_memory_win32_handle_properties_khr: DeviceResolvedFn<PFN_vkGetMemoryWin32HandlePropertiesKHR>,
}
impl<Instance: crate::Instance> DeviceObject<Instance> {
    pub fn wrap_handle(handle: VkDevice, parent: Instance) -> Self {
        Self {
            handle,
            parent,
            #[cfg(all(feature = "VK_KHR_maintenance1", feature = "Implements"))]
            trim_command_pool_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))]
            create_descriptor_update_template_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))]
            destroy_descriptor_update_template_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))]
            update_descriptor_set_with_template_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements"))]
            bind_buffer_memory2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements"))]
            bind_image_memory2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "Implements"))]
            get_image_drm_format_modifier_properties_ext: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
            get_fence_fd_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
            import_fence_fd_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_EXT_full_screen_exclusive"))]
            acquire_full_screen_exclusive_mode_ext: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_EXT_full_screen_exclusive"))]
            release_full_screen_exclusive_mode_ext: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))]
            get_memory_fd_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))]
            get_memory_fd_properties_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_EXT_external_memory_host"))]
            get_memory_host_pointer_properties_ext: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))]
            import_semaphore_win32_handle_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))]
            get_semaphore_win32_handle_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))]
            get_memory_win32_handle_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))]
            get_memory_win32_handle_properties_khr: DeviceResolvedFn::new(handle),
        }
    }
}
unsafe impl<Instance: crate::Instance + Sync> Sync for DeviceObject<Instance> {}
unsafe impl<Instance: crate::Instance + Send> Send for DeviceObject<Instance> {}
#[cfg(feature = "Implements")]
impl<Instance: crate::Instance> Drop for DeviceObject<Instance> {
    fn drop(&mut self) {
        unsafe {
            crate::vkresolve::destroy_device(self.handle, std::ptr::null());
        }
    }
}
impl<Instance: crate::Instance> Device for DeviceObject<Instance> {
    #[cfg(all(feature = "VK_KHR_maintenance1", feature = "Implements"))]
    fn get_trim_command_pool_khr_fn(&self) -> PFN_vkTrimCommandPoolKHR {
        *self.trim_command_pool_khr.resolve()
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))] {
            fn create_descriptor_update_template_khr_fn(&self) -> PFN_vkCreateDescriptorUpdateTemplateKHR {
                *self.create_descriptor_update_template_khr.resolve()
            }
            fn destroy_descriptor_update_template_khr_fn(&self) -> PFN_vkDestroyDescriptorUpdateTemplateKHR {
                *self.destroy_descriptor_update_template_khr.resolve()
            }
            fn update_descriptor_set_with_template_khr_fn(&self) -> PFN_vkUpdateDescriptorSetWithTemplateKHR {
                *self.update_descriptor_set_with_template_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements"))] {
            fn bind_buffer_memory2_khr_fn(&self) -> PFN_vkBindBufferMemory2KHR {
                *self.bind_buffer_memory2_khr.resolve()
            }
            fn bind_image_memory2_khr_fn(&self) -> PFN_vkBindImageMemory2KHR {
                *self.bind_image_memory2_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "Implements"))] {
            fn get_image_drm_format_modifier_properties_ext_fn(&self) -> PFN_vkGetImageDrmFormatModifierPropertiesEXT {
                *self.get_image_drm_format_modifier_properties_ext.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))] {
            fn get_fence_fd_khr_fn(&self) -> PFN_vkGetFenceFdKHR {
                *self.get_fence_fd_khr.resolve()
            }
            fn import_fence_fd_khr_fn(&self) -> PFN_vkImportFenceFdKHR {
                *self.import_fence_fd_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_EXT_full_screen_exclusive"))] {
            fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkAcquireFullScreenExclusiveModeEXT {
                *self.acquire_full_screen_exclusive_mode_ext.resolve()
            }
            fn release_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkReleaseFullScreenExclusiveModeEXT {
                *self.release_full_screen_exclusive_mode_ext.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))] {
            fn get_memory_fd_khr_fn(&self) -> PFN_vkGetMemoryFdKHR {
                *self.get_memory_fd_khr.resolve()
            }
            fn get_memory_fd_properties_khr_fn(&self) -> PFN_vkGetMemoryFdPropertiesKHR {
                *self.get_memory_fd_properties_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_EXT_external_memory_host"))] {
            fn get_memory_host_pointer_properties_ext_fn(&self) -> PFN_vkGetMemoryHostPointerPropertiesEXT {
                *self.get_memory_host_pointer_properties_ext.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))] {
            fn import_semaphore_win32_handle_khr_fn(&self) -> PFN_vkImportSemaphoreWin32HandleKHR {
                *self.import_semaphore_win32_handle_khr.resolve()
            }
            fn get_semaphore_win32_handle_khr_fn(&self) -> PFN_vkGetSemaphoreWin32HandleKHR {
                *self.get_semaphore_win32_handle_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))] {
            fn get_memory_win32_handle_khr_fn(&self) -> PFN_vkGetMemoryWin32HandleKHR {
                *self.get_memory_win32_handle_khr.resolve()
            }
            fn get_memory_win32_handle_properties_khr_fn(&self) -> PFN_vkGetMemoryWin32HandlePropertiesKHR {
                *self.get_memory_win32_handle_properties_khr.resolve()
            }
        }
    }
}
impl<Instance: crate::Instance + Clone> DeviceObject<&'_ Instance> {
    /// Clones parent reference
    #[inline]
    pub fn clone_parent(self) -> DeviceObject<Instance> {
        let r = DeviceObject {
            handle: self.handle,
            parent: self.parent.clone(),
            #[cfg(all(feature = "VK_KHR_maintenance1", feature = "Implements"))]
            trim_command_pool_khr: unsafe { core::ptr::read(&self.trim_command_pool_khr) },
            #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))]
            create_descriptor_update_template_khr: unsafe {
                core::ptr::read(&self.create_descriptor_update_template_khr)
            },
            #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))]
            destroy_descriptor_update_template_khr: unsafe {
                core::ptr::read(&self.destroy_descriptor_update_template_khr)
            },
            #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))]
            update_descriptor_set_with_template_khr: unsafe {
                core::ptr::read(&self.update_descriptor_set_with_template_khr)
            },
            #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements"))]
            bind_buffer_memory2_khr: unsafe { core::ptr::read(&self.bind_buffer_memory2_khr) },
            #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements"))]
            bind_image_memory2_khr: unsafe { core::ptr::read(&self.bind_image_memory2_khr) },
            #[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "Implements"))]
            get_image_drm_format_modifier_properties_ext: unsafe {
                core::ptr::read(&self.get_image_drm_format_modifier_properties_ext)
            },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
            get_fence_fd_khr: unsafe { core::ptr::read(&self.get_fence_fd_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
            import_fence_fd_khr: unsafe { core::ptr::read(&self.import_fence_fd_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_EXT_full_screen_exclusive"))]
            acquire_full_screen_exclusive_mode_ext: unsafe {
                core::ptr::read(&self.acquire_full_screen_exclusive_mode_ext)
            },
            #[cfg(all(feature = "Implements", feature = "VK_EXT_full_screen_exclusive"))]
            release_full_screen_exclusive_mode_ext: unsafe {
                core::ptr::read(&self.release_full_screen_exclusive_mode_ext)
            },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))]
            get_memory_fd_khr: unsafe { core::ptr::read(&self.get_memory_fd_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))]
            get_memory_fd_properties_khr: unsafe { core::ptr::read(&self.get_memory_fd_properties_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_EXT_external_memory_host"))]
            get_memory_host_pointer_properties_ext: unsafe {
                core::ptr::read(&self.get_memory_host_pointer_properties_ext)
            },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))]
            import_semaphore_win32_handle_khr: unsafe { core::ptr::read(&self.import_semaphore_win32_handle_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))]
            get_semaphore_win32_handle_khr: unsafe { core::ptr::read(&self.get_semaphore_win32_handle_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))]
            get_memory_win32_handle_khr: unsafe { core::ptr::read(&self.get_memory_win32_handle_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))]
            get_memory_win32_handle_properties_khr: unsafe {
                core::ptr::read(&self.get_memory_win32_handle_properties_khr)
            },
        };
        // disable running VkDevice destruction
        std::mem::forget(self);

        r
    }
}

/// Opaque handle to a queue object
#[derive(Clone, VkHandle, VkObject, crate::DeviceChild)]
#[VkObject(type = VK_OBJECT_TYPE_QUEUE)]
pub struct QueueObject<Device: crate::Device>(VkQueue, #[parent] Device);
unsafe impl<Device: crate::Device + Sync> Sync for QueueObject<Device> {}
unsafe impl<Device: crate::Device + Send> Send for QueueObject<Device> {}
impl<Device: crate::Device> Queue for QueueObject<Device> {}

/// Family Index, Queue Priorities
pub struct DeviceQueueCreateInfo(pub u32, pub Vec<f32>);

/// Builder object for constructing a `Device`
pub struct DeviceBuilder<PhysicalDevice: crate::PhysicalDevice + InstanceChild> {
    pdev_ref: PhysicalDevice,
    queue_infos: Vec<DeviceQueueCreateInfo>,
    layers: Vec<std::ffi::CString>,
    extensions: Vec<std::ffi::CString>,
    features: VkPhysicalDeviceFeatures,
}
impl<'p, PhysicalDevice: crate::PhysicalDevice + InstanceChild> DeviceBuilder<PhysicalDevice> {
    pub fn new(pdev: PhysicalDevice) -> Self {
        Self {
            pdev_ref: pdev,
            queue_infos: Vec::new(),
            layers: Vec::new(),
            extensions: Vec::new(),
            features: Default::default(),
        }
    }
    pub fn add_layer(&mut self, name: &str) -> &mut Self {
        self.layers.push(std::ffi::CString::new(name).unwrap());
        self
    }
    pub fn add_extension(&mut self, name: &str) -> &mut Self {
        self.extensions.push(std::ffi::CString::new(name).unwrap());
        self
    }
    pub fn add_layers<'s, Layers: IntoIterator<Item = &'s str>>(&mut self, layers: Layers) -> &mut Self {
        for l in layers {
            self.add_layer(l);
        }
        self
    }
    pub fn add_extensions<'s, Extensions: IntoIterator<Item = &'s str>>(
        &mut self,
        extensions: Extensions,
    ) -> &mut Self {
        for e in extensions {
            self.add_extension(e);
        }
        self
    }
    pub fn add_queue(&mut self, info: DeviceQueueCreateInfo) -> &mut Self {
        self.queue_infos.push(info);
        self
    }
    pub fn add_queues<Queues: IntoIterator<Item = DeviceQueueCreateInfo>>(&mut self, queues: Queues) -> &mut Self {
        for q in queues {
            self.add_queue(q);
        }
        self
    }
    pub fn mod_features(&mut self) -> &mut VkPhysicalDeviceFeatures {
        &mut self.features
    }

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
    #[cfg(feature = "Implements")]
    pub fn create(self) -> crate::Result<DeviceObject<PhysicalDevice::ConcreteInstance>>
    where
        PhysicalDevice: crate::InstanceChildTransferrable,
    {
        let qinfos = self
            .queue_infos
            .iter()
            .map(|&DeviceQueueCreateInfo(fi, ref ps)| VkDeviceQueueCreateInfo {
                sType: VkDeviceQueueCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                queueFamilyIndex: fi,
                queueCount: ps.len() as _,
                pQueuePriorities: ps.as_ptr(),
            })
            .collect::<Vec<_>>();
        let layers = self.layers.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
        let extensions = self.extensions.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
        let cinfo = VkDeviceCreateInfo {
            sType: VkDeviceCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            queueCreateInfoCount: qinfos.len() as _,
            pQueueCreateInfos: qinfos.as_ptr(),
            enabledLayerCount: layers.len() as _,
            ppEnabledLayerNames: layers.as_ptr(),
            enabledExtensionCount: extensions.len() as _,
            ppEnabledExtensionNames: extensions.as_ptr(),
            pEnabledFeatures: &self.features,
        };
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_device(self.pdev_ref.native_ptr(), &cinfo, ::std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| DeviceObject::wrap_handle(h.assume_init(), self.pdev_ref.transfer_instance()))
        }
    }
}

/// Tweaking features
impl<PhysicalDevice: crate::PhysicalDevice + InstanceChild> DeviceBuilder<PhysicalDevice> {
    pub fn enable_fill_mode_nonsolid(&mut self) -> &mut Self {
        self.features.fillModeNonSolid = true as _;
        self
    }
    pub fn enable_sample_rate_shading(&mut self) -> &mut Self {
        self.features.sampleRateShading = true as _;
        self
    }
    pub fn enable_geometry_shader(&mut self) -> &mut Self {
        self.features.geometryShader = true as _;
        self
    }
    pub fn enable_tessellation_shader(&mut self) -> &mut Self {
        self.features.tessellationShader = true as _;
        self
    }
    pub fn enable_vertex_pipeline_stores_and_atomics(&mut self) -> &mut Self {
        self.features.vertexPipelineStoresAndAtomics = true as _;
        self
    }
}

pub trait Device: VkHandle<Handle = VkDevice> + InstanceChild {
    /// Return a function pointer for a command
    /// # Failures
    /// If function is not provided by instance or `name` is empty, returns `None`
    #[deprecated = "do not use this directly(this does not provide caching)"]
    #[cfg(feature = "Implements")]
    fn extra_procedure<F: FnTransmute>(&self, name: &str) -> Option<F> {
        if name.is_empty() {
            return None;
        }

        unsafe {
            let fn_cstr = std::ffi::CString::new(name).unwrap();
            crate::vkresolve::get_device_proc_addr(self.native_ptr(), fn_cstr.as_ptr()).map(|f| FnTransmute::from_fn(f))
        }
    }

    /// Get a queue handle from a device
    #[cfg(feature = "Implements")]
    fn queue(self, family_index: u32, queue_index: u32) -> QueueObject<Self>
    where
        Self: Sized,
    {
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::get_device_queue(self.native_ptr(), family_index, queue_index, h.as_mut_ptr());
            QueueObject(h.assume_init(), self)
        }
    }

    /// Invalidate `MappedMemoryRange`s
    /// Invalidating the memory range allows that device writes to the memory ranges
    /// which have been made visible to the `VK_ACCESS_HOST_WRITE_BIT` and `VK_ACCESS_HOST_READ_BIT`
    /// are made visible to the host
    /// # Safety
    /// Memory object in `ranges` must be currently host mapped
    #[cfg(feature = "Implements")]
    unsafe fn invalidate_memory_range(&self, ranges: &[VkMappedMemoryRange]) -> crate::Result<()> {
        crate::vkresolve::invalidate_mapped_memory_ranges(self.native_ptr(), ranges.len() as _, ranges.as_ptr())
            .into_result()
            .map(drop)
    }

    /// Update the contents of a descriptor set object
    #[cfg(feature = "Implements")]
    fn update_descriptor_sets(&self, write: &[DescriptorSetWriteInfo], copy: &[DescriptorSetCopyInfo]) {
        // save flatten results
        let wt = write
            .iter()
            .map(|x| {
                let (ty, count, imgs, bufs, bufviews) = x.3.decomposite();
                (
                    x.0,
                    x.1,
                    x.2,
                    ty,
                    count,
                    imgs.iter()
                        .map(|&(s, v, l)| VkDescriptorImageInfo {
                            sampler: s.unwrap_or(VkSampler::NULL),
                            imageView: v,
                            imageLayout: l as _,
                        })
                        .collect::<Vec<_>>(),
                    bufs.iter()
                        .map(|&(b, ref r)| VkDescriptorBufferInfo {
                            buffer: b,
                            offset: r.start as _,
                            range: (r.end - r.start) as _,
                        })
                        .collect::<Vec<_>>(),
                    bufviews,
                )
            })
            .collect::<Vec<_>>();
        let w = wt
            .iter()
            .map(
                |&(set, binding, array, dty, count, ref iv, ref bv, ref bvv)| VkWriteDescriptorSet {
                    sType: VkWriteDescriptorSet::TYPE,
                    pNext: std::ptr::null(),
                    dstSet: set,
                    dstBinding: binding,
                    dstArrayElement: array,
                    descriptorType: dty as _,
                    descriptorCount: count,
                    pImageInfo: iv.as_ptr(),
                    pBufferInfo: bv.as_ptr(),
                    pTexelBufferView: bvv.as_ptr(),
                },
            )
            .collect::<Vec<_>>();
        let c = copy
            .iter()
            .map(|x| VkCopyDescriptorSet {
                sType: VkCopyDescriptorSet::TYPE,
                pNext: std::ptr::null(),
                srcSet: x.src.0,
                srcBinding: x.src.1,
                srcArrayElement: x.src.2,
                dstSet: x.dst.0,
                dstBinding: x.dst.1,
                dstArrayElement: x.dst.2,
                descriptorCount: x.count,
            })
            .collect::<Vec<_>>();
        unsafe {
            crate::vkresolve::update_descriptor_sets(
                self.native_ptr(),
                w.len() as _,
                w.as_ptr(),
                c.len() as _,
                c.as_ptr(),
            );
        }
    }

    /// Wait for a object to become idle
    /// # Safety
    /// All VkQueue objects created from this device must be externally synchronized.
    #[cfg(feature = "Implements")]
    unsafe fn wait(&self) -> crate::Result<()> {
        crate::vkresolve::device_wait_idle(self.native_ptr())
            .into_result()
            .map(drop)
    }

    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))]
    /// Import a semaphore from a Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_INVALID_EXTERNAL_HANDLE
    fn import_semaphore_win32_handle(
        &self,
        target: &impl VkHandle<Handle = VkSemaphore>,
        handle: crate::ExternalSemaphoreHandleWin32,
        name: &widestring::WideCString,
    ) -> crate::Result<()> {
        let info = VkImportSemaphoreWin32HandleInfoKHR {
            sType: VkImportSemaphoreWin32HandleInfoKHR::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            semaphore: target.native_ptr(),
            handleType: handle.as_type_bits(),
            handle: handle.handle(),
            name: windows::core::PCWSTR(name.as_ptr()),
        };

        unsafe {
            VkResultBox(self.import_semaphore_win32_handle_khr_fn().0(self.native_ptr(), &info))
                .into_result()
                .map(drop)
        }
    }

    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))]
    /// Get a Windows HANDLE for a semaphore
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_TOO_MANY_OBJECTS
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    fn get_semaphore_win32_handle(
        &self,
        target: &impl VkHandle<Handle = VkSemaphore>,
        handle_type: crate::ExternalSemaphoreHandleTypeWin32,
    ) -> crate::Result<windows::Win32::Foundation::HANDLE> {
        let info = VkSemaphoreGetWin32HandleInfoKHR {
            sType: VkSemaphoreGetWin32HandleInfoKHR::TYPE,
            pNext: std::ptr::null(),
            semaphore: target.native_ptr(),
            handleType: handle_type as _,
        };
        let mut h = windows::Win32::Foundation::HANDLE(0);

        unsafe {
            VkResultBox(self.get_semaphore_win32_handle_khr_fn().0(
                self.native_ptr(),
                &info,
                &mut h,
            ))
            .into_result()
            .map(move |_| h)
        }
    }

    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))]
    /// Get a Windows HANDLE for a memory object
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    fn get_memory_win32_handle(
        &self,
        memory: &impl VkHandle<Handle = VkDeviceMemory>,
        handle_type: crate::ExternalMemoryHandleTypeWin32,
    ) -> crate::Result<windows::Win32::Foundation::HANDLE> {
        let info = VkMemoryGetWin32HandleInfoKHR {
            sType: VkMemoryGetWin32HandleInfoKHR::TYPE,
            pNext: std::ptr::null(),
            memory: memory.native_ptr(),
            handleType: handle_type as _,
        };
        let mut h = windows::Win32::Foundation::HANDLE(0);

        unsafe {
            VkResultBox(self.get_memory_win32_handle_khr_fn().0(
                self.native_ptr(),
                &info,
                &mut h,
            ))
            .into_result()
            .map(move |_| h)
        }
    }

    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))]
    /// Get a POSIX file descriptor for a memory object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    fn get_memory_fd(
        &self,
        memory: &impl VkHandle<Handle = VkDeviceMemory>,
        handle_type: crate::ExternalMemoryHandleTypeFd,
    ) -> crate::Result<libc::c_int> {
        let info = VkMemoryGetFdInfoKHR {
            sType: VkMemoryGetFdInfoKHR::TYPE,
            pNext: std::ptr::null(),
            memory: memory.native_ptr(),
            handleType: handle_type as _,
        };
        let mut fd = 0;

        unsafe {
            VkResultBox(self.get_memory_fd_khr_fn().0(self.native_ptr(), &info, &mut fd))
                .into_result()
                .map(move |_| fd)
        }
    }

    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))]
    /// Get Properties of External Memory Win32 Handles
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    fn get_memory_win32_handle_properties(
        &self,
        handle_type: crate::ExternalMemoryHandleTypeWin32,
        handle: windows::Win32::Foundation::HANDLE,
    ) -> crate::Result<VkMemoryWin32HandlePropertiesKHR> {
        let mut info = VkMemoryWin32HandlePropertiesKHR::uninit_sink();

        unsafe {
            VkResultBox(self.get_memory_win32_handle_properties_khr_fn().0(
                self.native_ptr(),
                handle_type as _,
                handle,
                info.as_mut_ptr(),
            ))
            .into_result()
            .map(move |_| info.assume_init())
        }
    }

    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))]
    /// Get Properties of External Memory File Descriptors
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    fn get_memory_fd_properties(
        &self,
        handle_type: crate::ExternalMemoryHandleTypeFd,
        fd: libc::c_int,
    ) -> crate::Result<VkMemoryFdPropertiesKHR> {
        let mut info = VkMemoryFdPropertiesKHR::uninit_sink();

        unsafe {
            VkResultBox(self.get_memory_fd_properties_khr_fn().0(
                self.native_ptr(),
                handle_type as _,
                fd,
                info.as_mut_ptr(),
            ))
            .into_result()
            .map(move |_| info.assume_init())
        }
    }

    #[cfg(all(feature = "Implements", feature = "VK_EXT_external_memory_host"))]
    /// Get Properties of external memory host pointer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    fn get_memory_host_pointer_properties(
        &self,
        handle_type: crate::ExternalMemoryHandleType,
        host_pointer: *const (),
    ) -> crate::Result<VkMemoryHostPointerPropertiesEXT> {
        let mut info = VkMemoryHostPointerPropertiesEXT::uninit_sink();

        unsafe {
            VkResultBox(self.get_memory_host_pointer_properties_ext_fn().0(
                self.native_ptr(),
                handle_type as _,
                host_pointer as _,
                info.as_mut_ptr(),
            ))
            .into_result()
            .map(move |_| info.assume_init())
        }
    }

    /// Create a new buffer object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_buffer(
        self,
        mut create_info: impl VulkanStructureProvider<RootStructure = VkBufferCreateInfo>,
    ) -> crate::Result<crate::BufferObject<Self>>
    where
        Self: Sized,
    {
        let mut h = std::mem::MaybeUninit::uninit();
        let mut s = std::mem::MaybeUninit::uninit();
        create_info.build(unsafe { &mut *s.as_mut_ptr() });
        let s = unsafe { s.assume_init_ref() };
        unsafe {
            crate::vkresolve::create_buffer(self.native_ptr(), s, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| crate::BufferObject(h.assume_init(), self))
        }
    }

    /// Multiple Binding for Buffers
    #[cfg(feature = "Implements")]
    #[cfg(feature = "VK_KHR_bind_memory2")]
    fn bind_buffers(
        &self,
        bounds: &[(
            &impl VkHandle<Handle = VkBuffer>,
            &impl VkHandle<Handle = VkDeviceMemory>,
            VkDeviceSize,
        )],
    ) -> crate::Result<()> {
        let infos: Vec<_> = bounds
            .iter()
            .map(|&(b, m, offs)| VkBindBufferMemoryInfoKHR {
                sType: VkBindBufferMemoryInfoKHR::TYPE,
                pNext: std::ptr::null(),
                buffer: b.native_ptr(),
                memory: m.native_ptr(),
                memoryOffset: offs,
            })
            .collect();

        unsafe {
            VkResultBox(self.bind_buffer_memory2_khr_fn().0(
                self.native_ptr(),
                infos.len() as _,
                infos.as_ptr(),
            ))
            .into_result()
            .map(drop)
        }
    }

    /// Multiple Binding for Images
    #[cfg(feature = "Implements")]
    #[cfg(feature = "VK_KHR_bind_memory2")]
    fn bind_images(
        &self,
        bounds: &[(
            &impl VkHandle<Handle = VkImage>,
            &impl VkHandle<Handle = VkDeviceMemory>,
            VkDeviceSize,
        )],
    ) -> crate::Result<()> {
        let infos: Vec<_> = bounds
            .iter()
            .map(|&(i, m, offs)| VkBindImageMemoryInfoKHR {
                sType: VkBindImageMemoryInfoKHR::TYPE,
                pNext: std::ptr::null(),
                image: i.native_ptr(),
                memory: m.native_ptr(),
                memoryOffset: offs,
            })
            .collect();

        unsafe {
            VkResultBox(self.bind_image_memory2_khr_fn().0(
                self.native_ptr(),
                infos.len() as _,
                infos.as_ptr(),
            ))
            .into_result()
            .map(drop)
        }
    }

    /// Multiple Binding for both resources
    #[cfg(feature = "Implements")]
    #[cfg(feature = "VK_KHR_bind_memory2")]
    fn bind_resources(
        &self,
        buf_bounds: &[(
            &impl VkHandle<Handle = VkBuffer>,
            &impl VkHandle<Handle = VkDeviceMemory>,
            VkDeviceSize,
        )],
        img_bounds: &[(
            &impl VkHandle<Handle = VkImage>,
            &impl VkHandle<Handle = VkDeviceMemory>,
            VkDeviceSize,
        )],
    ) -> crate::Result<()> {
        // 必ず両方実行されるようにする
        self.bind_buffers(buf_bounds).and(self.bind_images(img_bounds))
    }

    /// Flush `MappedMemoryRange`s
    /// Flushing the memory range allows that host writes to the memory ranges can
    /// be made available to device access
    /// # Safety
    /// Memory object in `ranges` must be currently host mapped
    #[cfg(feature = "Implements")]
    unsafe fn flush_mapped_memory_ranges(&self, ranges: &[VkMappedMemoryRange]) -> crate::Result<()> {
        crate::vkresolve::flush_mapped_memory_ranges(self.native_ptr(), ranges.len() as _, ranges.as_ptr() as *const _)
            .into_result()
            .map(drop)
    }

    /// Creates a new shader module object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_shader_module(self, code: &(impl AsRef<[u8]> + ?Sized)) -> crate::Result<crate::ShaderModuleObject<Self>>
    where
        Self: Sized,
    {
        #[allow(clippy::cast_ptr_alignment)]
        let cinfo = VkShaderModuleCreateInfo {
            sType: VkShaderModuleCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            codeSize: code.as_ref().len() as _,
            pCode: code.as_ref().as_ptr() as *const _,
        };
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_shader_module(self.native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| crate::ShaderModuleObject(h.assume_init(), self))
        }
    }

    /// Creates a new pipeline cache
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_pipeline_cache(
        self,
        initial: &(impl AsRef<[u8]> + ?Sized),
    ) -> crate::Result<crate::PipelineCacheObject<Self>>
    where
        Self: Sized,
    {
        let cinfo = VkPipelineCacheCreateInfo {
            sType: VkPipelineCacheCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            initialDataSize: initial.as_ref().len() as _,
            pInitialData: initial.as_ref().as_ptr() as *const _,
        };
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_pipeline_cache(self.native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| crate::PipelineCacheObject(h.assume_init(), self))
        }
    }

    /// Creates a new pipeline layout object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    #[deprecated = "create object via builder struct(PipelineLayoutBuilder)"]
    fn new_pipeline_layout(
        self,
        layouts: &[impl VkHandle<Handle = VkDescriptorSetLayout>],
        push_constants: &[(crate::ShaderStage, std::ops::Range<u32>)],
    ) -> crate::Result<crate::PipelineLayoutObject<Self>>
    where
        Self: Sized,
    {
        let layouts = layouts.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();
        let push_constants = push_constants
            .iter()
            .map(|&(sh, ref r)| VkPushConstantRange {
                stageFlags: sh.0,
                offset: r.start,
                size: r.end - r.start,
            })
            .collect::<Vec<_>>();
        let cinfo = VkPipelineLayoutCreateInfo {
            sType: VkPipelineLayoutCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            setLayoutCount: layouts.len() as _,
            pSetLayouts: layouts.as_ptr(),
            pushConstantRangeCount: push_constants.len() as _,
            pPushConstantRanges: push_constants.as_ptr(),
        };
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_pipeline_layout(self.native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| crate::PipelineLayoutObject(h.assume_init(), self))
        }
    }

    /// Create graphics pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_graphics_pipelines(
        &self,
        infos: &[VkGraphicsPipelineCreateInfo],
        cache: Option<&impl crate::PipelineCache>,
    ) -> crate::Result<Vec<crate::PipelineObject<Self>>>
    where
        Self: Clone,
    {
        let mut hs = vec![VkPipeline::NULL; infos.len()];
        let r = unsafe {
            crate::vkresolve::create_graphics_pipelines(
                self.native_ptr(),
                cache.map(VkHandle::native_ptr).unwrap_or(VkPipelineCache::NULL),
                infos.len() as _,
                infos.as_ptr(),
                std::ptr::null(),
                hs.as_mut_ptr(),
            )
        };

        r.into_result()
            .map(|_| hs.into_iter().map(|h| crate::PipelineObject(h, self.clone())).collect())
    }

    /// Create compute pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_compute_pipelines(
        &self,
        builders: &[crate::ComputePipelineBuilder<impl crate::PipelineLayout, impl crate::ShaderModule>],
        cache: Option<&impl crate::PipelineCache>,
    ) -> crate::Result<Vec<crate::PipelineObject<Self>>>
    where
        Self: Clone,
    {
        let (stages, _specinfos): (Vec<_>, Vec<_>) = builders
            .iter()
            .map(|b| b.shader.createinfo_native(crate::ShaderStage::COMPUTE))
            .unzip();
        let cinfos = builders
            .iter()
            .zip(stages.into_iter())
            .map(|(b, stage)| VkComputePipelineCreateInfo {
                sType: VkComputePipelineCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                basePipelineHandle: VkPipeline::NULL,
                basePipelineIndex: -1,
                stage,
                layout: b.layout.native_ptr(),
            })
            .collect::<Vec<_>>();

        let mut pipelines = vec![VkPipeline::NULL; builders.len()];
        unsafe {
            crate::vkresolve::create_compute_pipelines(
                self.native_ptr(),
                cache.map(VkHandle::native_ptr).unwrap_or(VkPipelineCache::NULL),
                cinfos.len() as _,
                cinfos.as_ptr(),
                std::ptr::null(),
                pipelines.as_mut_ptr(),
            )
            .into_result()
            .map(move |_| {
                pipelines
                    .into_iter()
                    .map(|h| crate::PipelineObject(h, self.clone()))
                    .collect()
            })
        }
    }

    /// Allocate GPU memory
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    #[cfg(feature = "Implements")]
    fn allocate_memory(self, size: usize, type_index: u32) -> crate::Result<crate::DeviceMemoryObject<Self>>
    where
        Self: Sized,
    {
        let mut h = std::mem::MaybeUninit::uninit();
        let cinfo = VkMemoryAllocateInfo {
            sType: VkMemoryAllocateInfo::TYPE,
            pNext: std::ptr::null(),
            allocationSize: size as _,
            memoryTypeIndex: type_index,
        };
        unsafe {
            crate::vkresolve::allocate_memory(self.native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| crate::DeviceMemoryObject(h.assume_init(), self))
        }
    }

    /// Import GPU memory from external apis
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "Implements")]
    fn import_memory_win32(
        self,
        size: usize,
        type_index: u32,
        handle_type: crate::ExternalMemoryHandleTypeWin32,
        handle: windows::Win32::Foundation::HANDLE,
        name: &widestring::WideCString,
    ) -> crate::Result<crate::DeviceMemoryObject<Self>>
    where
        Self: Sized,
    {
        let import_info = VkImportMemoryWin32HandleInfoKHR {
            sType: VkImportMemoryWin32HandleInfoKHR::TYPE,
            pNext: std::ptr::null(),
            handleType: handle_type as _,
            handle,
            name: windows::core::PCWSTR(name.as_ptr()),
        };
        let ainfo = VkMemoryAllocateInfo {
            sType: VkMemoryAllocateInfo::TYPE,
            pNext: &import_info as *const _ as _,
            allocationSize: size as _,
            memoryTypeIndex: type_index,
        };

        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::allocate_memory(self.native_ptr(), &ainfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| crate::DeviceMemoryObject(h.assume_init(), self))
        }
    }

    /// Allocate GPU memory and visible to external apis
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "Implements")]
    fn allocate_memory_and_export_win32(
        self,
        size: usize,
        type_index: u32,
        security_attributes: Option<&windows::Win32::Security::SECURITY_ATTRIBUTES>,
        access: u32,
        name: &widestring::WideCString,
    ) -> crate::Result<crate::DeviceMemoryObject<Self>>
    where
        Self: Sized,
    {
        let export_info = VkExportMemoryWin32HandleInfoKHR {
            sType: VkExportMemoryWin32HandleInfoKHR::TYPE,
            pNext: std::ptr::null(),
            pAttributes: security_attributes.map_or_else(std::ptr::null, |v| v as *const _),
            dwAccess: access,
            name: windows::core::PCWSTR(name.as_ptr()),
        };
        let ainfo = VkMemoryAllocateInfo {
            sType: VkMemoryAllocateInfo::TYPE,
            pNext: &export_info as *const _ as _,
            allocationSize: size as _,
            memoryTypeIndex: type_index,
        };

        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::allocate_memory(self.native_ptr(), &ainfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| crate::DeviceMemoryObject(h.assume_init(), self))
        }
    }

    /// [Implements][VK_KHR_external_memory_fd] Import GPU memory from external apis
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    #[cfg(feature = "Implements")]
    fn import_memory_fd(
        self,
        size: usize,
        type_index: u32,
        handle_type: crate::ExternalMemoryHandleTypeFd,
        fd: libc::c_int,
    ) -> crate::Result<crate::DeviceMemoryObject<Self>>
    where
        Self: Sized,
    {
        let import_info = VkImportMemoryFdInfoKHR {
            sType: VkImportMemoryFdInfoKHR::TYPE,
            pNext: std::ptr::null(),
            handleType: handle_type as _,
            fd,
        };
        let ainfo = VkMemoryAllocateInfo {
            sType: VkMemoryAllocateInfo::TYPE,
            pNext: &import_info as *const _ as _,
            allocationSize: size as _,
            memoryTypeIndex: type_index,
        };

        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::allocate_memory(self.native_ptr(), &ainfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| crate::DeviceMemoryObject(h.assume_init(), self))
        }
    }

    /// Import GPU memory from external apis
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    #[cfg(feature = "VK_EXT_external_memory_host")]
    #[cfg(feature = "Implements")]
    fn import_memory_from_host_pointer(
        self,
        size: usize,
        type_index: u32,
        handle_type: crate::ExternalMemoryHandleType,
        host_pointer: *mut (),
    ) -> crate::Result<crate::DeviceMemoryObject<Self>>
    where
        Self: Sized,
    {
        let import_info = VkImportMemoryHostPointerInfoEXT {
            sType: VkImportMemoryHostPointerInfoEXT::TYPE,
            pNext: std::ptr::null(),
            handleType: handle_type as _,
            pHostPointer: host_pointer as _,
        };
        let ainfo = VkMemoryAllocateInfo {
            sType: VkMemoryAllocateInfo::TYPE,
            pNext: &import_info as *const _ as _,
            allocationSize: size as _,
            memoryTypeIndex: type_index,
        };

        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::allocate_memory(self.native_ptr(), &ainfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| crate::DeviceMemoryObject(h.assume_init(), self))
        }
    }

    /// Create a new fence object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_fence(self, signaled: bool) -> crate::Result<crate::FenceObject<Self>>
    where
        Self: Sized,
    {
        let mut h = std::mem::MaybeUninit::uninit();
        let flags = if signaled { VK_FENCE_CREATE_SIGNALED_BIT } else { 0 };
        unsafe {
            crate::vkresolve::create_fence(
                self.native_ptr(),
                &VkFenceCreateInfo {
                    sType: VkFenceCreateInfo::TYPE,
                    pNext: std::ptr::null(),
                    flags,
                },
                std::ptr::null(),
                h.as_mut_ptr(),
            )
            .into_result()
            .map(|_| crate::FenceObject(h.assume_init(), self))
        }
    }

    #[cfg(feature = "VK_KHR_external_fence_fd")]
    /// Create a new fence object, with exporting as file descriptors
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_fence_with_export_fd(
        self,
        signaled: bool,
        compatible_handle_types: crate::ExternalFenceHandleTypes,
    ) -> crate::Result<crate::FenceObject<Self>>
    where
        Self: Sized,
    {
        let mut h = std::mem::MaybeUninit::uninit();
        let exp_info = VkExportFenceCreateInfoKHR {
            sType: VkExportFenceCreateInfoKHR::TYPE,
            pNext: std::ptr::null(),
            handleTypes: compatible_handle_types.0,
        };
        let cinfo = VkFenceCreateInfo {
            sType: VkFenceCreateInfo::TYPE,
            flags: if signaled { VK_FENCE_CREATE_SIGNALED_BIT } else { 0 },
            pNext: &exp_info as *const _ as _,
        };
        unsafe {
            crate::vkresolve::create_fence(self.native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| crate::FenceObject(h.assume_init(), self))
        }
    }

    /// Create a new queue semaphore object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_semaphore(self) -> crate::Result<crate::SemaphoreObject<Self>>
    where
        Self: Sized,
    {
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_semaphore(
                self.native_ptr(),
                &VkSemaphoreCreateInfo {
                    sType: VkSemaphoreCreateInfo::TYPE,
                    pNext: std::ptr::null(),
                    flags: 0,
                },
                std::ptr::null(),
                h.as_mut_ptr(),
            )
            .into_result()
            .map(|_| crate::SemaphoreObject(h.assume_init(), self))
        }
    }

    /// Create a new queue semaphore object, with exporting as Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[cfg(feature = "Implements")]
    fn new_semaphore_with_export_win32(
        self,
        handle_types: crate::ExternalSemaphoreHandleTypes,
        export_info: &crate::ExportSemaphoreWin32HandleInfo,
    ) -> crate::Result<crate::SemaphoreObject<Self>>
    where
        Self: Sized,
    {
        let exp_info = VkExportSemaphoreCreateInfoKHR {
            sType: VkExportSemaphoreCreateInfoKHR::TYPE,
            pNext: export_info.as_ref() as *const _ as _,
            handleTypes: handle_types.into(),
        };
        let info = VkSemaphoreCreateInfo {
            sType: VkSemaphoreCreateInfo::TYPE,
            pNext: &exp_info as *const _ as _,
            flags: 0,
        };
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_semaphore(self.native_ptr(), &info, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| crate::SemaphoreObject(h.assume_init(), self))
        }
    }

    /// Create a new event object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_event(self) -> crate::Result<crate::EventObject<Self>>
    where
        Self: Sized,
    {
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_event(
                self.native_ptr(),
                &VkEventCreateInfo {
                    sType: VkEventCreateInfo::TYPE,
                    pNext: std::ptr::null(),
                    flags: 0,
                },
                std::ptr::null(),
                h.as_mut_ptr(),
            )
            .into_result()
            .map(|_| crate::EventObject(h.assume_init(), self))
        }
    }

    /// Wait for one or more fences to become signaled, returns `Ok(true)` if operation is timed out
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[cfg(feature = "Implements")]
    fn wait_multiple_fences(
        &self,
        objects: &[impl crate::Fence],
        wait_all: bool,
        timeout: Option<u64>,
    ) -> crate::Result<bool> {
        let objects_ptr = objects.iter().map(VkHandle::native_ptr).collect::<Vec<_>>();
        let vr = unsafe {
            crate::vkresolve::wait_for_fences(
                self.native_ptr(),
                objects_ptr.len() as _,
                objects_ptr.as_ptr(),
                wait_all as _,
                timeout.unwrap_or(std::u64::MAX),
            )
        };
        match vr.0 {
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
    #[cfg(feature = "Implements")]
    fn reset_multiple_fences(&self, objects: &[&mut impl crate::Fence]) -> crate::Result<()> {
        let objects_ptr = objects.iter().map(VkHandle::native_ptr).collect::<Vec<_>>();
        unsafe {
            crate::vkresolve::reset_fences(self.native_ptr(), objects_ptr.len() as _, objects_ptr.as_ptr())
                .into_result()
                .map(drop)
        }
    }

    /// Create a new command pool object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_command_pool(
        self,
        queue_family: u32,
        transient: bool,
        indiv_resettable: bool,
    ) -> crate::Result<crate::CommandPoolObject<Self>>
    where
        Self: Sized,
    {
        let cinfo = VkCommandPoolCreateInfo {
            sType: VkCommandPoolCreateInfo::TYPE,
            pNext: std::ptr::null(),
            queueFamilyIndex: queue_family,
            flags: if transient {
                VK_COMMAND_POOL_CREATE_TRANSIENT_BIT
            } else {
                0
            } | if indiv_resettable {
                VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT
            } else {
                0
            },
        };
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_command_pool(self.native_ptr(), &cinfo, ::std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| crate::CommandPoolObject(h.assume_init(), self))
        }
    }

    /// Create a new descriptor set layout
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[cfg(feature = "Implements")]
    fn new_descriptor_set_layout(
        self,
        bindings: &[crate::DescriptorSetLayoutBinding],
    ) -> crate::Result<crate::DescriptorSetLayoutObject<Self>>
    where
        Self: Sized,
    {
        let binding_structures: Vec<_> = bindings
            .into_iter()
            .enumerate()
            .map(|(n, b)| b.make_structure_with_binding_index(n as _))
            .collect();
        let cinfo = VkDescriptorSetLayoutCreateInfo {
            sType: VkDescriptorSetLayoutCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            bindingCount: binding_structures.len() as _,
            pBindings: binding_structures.as_ptr(),
        };

        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_descriptor_set_layout(self.native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| crate::DescriptorSetLayoutObject(h.assume_init(), self))
        }
    }

    #[cfg(feature = "Implements")]
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    #[cfg(not(feature = "VK_KHR_push_descriptor"))]
    fn new_descriptor_update_template(
        self,
        entries: &[VkDescriptorUpdateTemplateEntryKHR],
        dsl: &impl crate::DescriptorSetLayout,
    ) -> crate::Result<crate::DescriptorUpdateTemplateObject<Self>>
    where
        Self: Sized + InstanceChild,
    {
        let cinfo = VkDescriptorUpdateTemplateCreateInfoKHR {
            sType: VkDescriptorUpdateTemplateCreateInfoKHR::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
            set: 0,
            pipelineLayout: VkPipelineLayout::NULL,
            descriptorUpdateEntryCount: entries.len() as _,
            pDescriptorUpdateEntries: entries.as_ptr(),
            templateType: VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET,
            descriptorSetLayout: dsl.native_ptr(),
        };
        let mut handle = std::mem::MaybeUninit::uninit();
        unsafe {
            VkResultBox(self.create_descriptor_update_template_khr_fn().0(
                self.native_ptr(),
                &cinfo,
                std::ptr::null(),
                handle.as_mut_ptr(),
            ))
            .into_result()
            .map(|_| crate::DescriptorUpdateTemplateObject(handle.assume_init(), self))
        }
    }

    /// dsl: NoneにするとPushDescriptors向けのテンプレートを作成できる
    #[cfg(feature = "Implements")]
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    #[cfg(feature = "VK_KHR_push_descriptor")]
    fn new_descriptor_update_template(
        self,
        entries: &[VkDescriptorUpdateTemplateEntryKHR],
        dsl: Option<&impl crate::DescriptorSetLayout>,
    ) -> crate::Result<crate::DescriptorUpdateTemplateObject<Self>>
    where
        Self: Sized + InstanceChild,
    {
        use crate::{Instance, VkRawHandle};

        let cinfo = VkDescriptorUpdateTemplateCreateInfoKHR {
            sType: VkDescriptorUpdateTemplateCreateInfoKHR::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
            set: 0,
            pipelineLayout: VkPipelineLayout::NULL,
            descriptorUpdateEntryCount: entries.len() as _,
            pDescriptorUpdateEntries: entries.as_ptr(),
            templateType: if dsl.is_none() {
                VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS
            } else {
                VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET
            },
            descriptorSetLayout: dsl.map_or(VkDescriptorSetLayout::NULL, VkHandle::native_ptr),
        };
        let mut handle = std::mem::MaybeUninit::uninit();
        unsafe {
            self.instance()
                .create_descriptor_update_template(self.native_ptr(), &cinfo, std::ptr::null(), handle.as_mut_ptr())
                .into_result()
                .map(|_| crate::DescriptorUpdateTemplateObject(handle.assume_init(), self))
        }
    }

    /// Create a new framebuffer object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn new_framebuffer<ImageView: crate::ImageView>(
        self,
        mold: &impl crate::RenderPass,
        attachment_objects: Vec<ImageView>,
        size: &VkExtent2D,
        layers: u32,
    ) -> crate::Result<crate::FramebufferObject<Self, ImageView>>
    where
        Self: Sized,
    {
        let views = attachment_objects.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();
        let cinfo = VkFramebufferCreateInfo {
            sType: VkFramebufferCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            renderPass: mold.native_ptr(),
            attachmentCount: views.len() as _,
            pAttachments: views.as_ptr(),
            width: size.width,
            height: size.height,
            layers,
        };
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_framebuffer(self.native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| crate::FramebufferObject(h.assume_init(), self, attachment_objects, size.as_ref().clone()))
        }
    }

    // Extension Function Providers

    #[cfg(all(feature = "VK_KHR_maintenance1", feature = "Implements"))]
    fn get_trim_command_pool_khr_fn(&self) -> PFN_vkTrimCommandPoolKHR;

    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))] {
            fn create_descriptor_update_template_khr_fn(&self) -> PFN_vkCreateDescriptorUpdateTemplateKHR;
            fn destroy_descriptor_update_template_khr_fn(&self) -> PFN_vkDestroyDescriptorUpdateTemplateKHR;
            fn update_descriptor_set_with_template_khr_fn(&self) -> PFN_vkUpdateDescriptorSetWithTemplateKHR;
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements"))] {
            fn bind_buffer_memory2_khr_fn(&self) -> PFN_vkBindBufferMemory2KHR;
            fn bind_image_memory2_khr_fn(&self) -> PFN_vkBindImageMemory2KHR;
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "Implements"))] {
            fn get_image_drm_format_modifier_properties_ext_fn(&self) -> PFN_vkGetImageDrmFormatModifierPropertiesEXT;
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))] {
            fn get_fence_fd_khr_fn(&self) -> PFN_vkGetFenceFdKHR;
            fn import_fence_fd_khr_fn(&self) -> PFN_vkImportFenceFdKHR;
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_EXT_full_screen_exclusive"))] {
            fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkAcquireFullScreenExclusiveModeEXT;
            fn release_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkReleaseFullScreenExclusiveModeEXT;
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))] {
            fn get_memory_fd_khr_fn(&self) -> PFN_vkGetMemoryFdKHR;
            fn get_memory_fd_properties_khr_fn(&self) -> PFN_vkGetMemoryFdPropertiesKHR;
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_EXT_external_memory_host"))] {
            fn get_memory_host_pointer_properties_ext_fn(&self) -> PFN_vkGetMemoryHostPointerPropertiesEXT;
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))] {
            fn import_semaphore_win32_handle_khr_fn(&self) -> PFN_vkImportSemaphoreWin32HandleKHR;
            fn get_semaphore_win32_handle_khr_fn(&self) -> PFN_vkGetSemaphoreWin32HandleKHR;
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))] {
            fn get_memory_win32_handle_khr_fn(&self) -> PFN_vkGetMemoryWin32HandleKHR;
            fn get_memory_win32_handle_properties_khr_fn(&self) -> PFN_vkGetMemoryWin32HandlePropertiesKHR;
        }
    }
}
DerefContainerBracketImpl!(for Device {
    #[cfg(all(feature = "VK_KHR_maintenance1", feature = "Implements"))]
    fn get_trim_command_pool_khr_fn(&self) -> PFN_vkTrimCommandPoolKHR {
        (**self).get_trim_command_pool_khr_fn()
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))] {
            fn create_descriptor_update_template_khr_fn(&self) -> PFN_vkCreateDescriptorUpdateTemplateKHR {
                (**self).create_descriptor_update_template_khr_fn()
            }
            fn destroy_descriptor_update_template_khr_fn(&self) -> PFN_vkDestroyDescriptorUpdateTemplateKHR {
                (**self).destroy_descriptor_update_template_khr_fn()
            }
            fn update_descriptor_set_with_template_khr_fn(&self) -> PFN_vkUpdateDescriptorSetWithTemplateKHR {
                (**self).update_descriptor_set_with_template_khr_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements"))] {
            fn bind_buffer_memory2_khr_fn(&self) -> PFN_vkBindBufferMemory2KHR {
                (**self).bind_buffer_memory2_khr_fn()
            }
            fn bind_image_memory2_khr_fn(&self) -> PFN_vkBindImageMemory2KHR {
                (**self).bind_image_memory2_khr_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "Implements"))] {
            fn get_image_drm_format_modifier_properties_ext_fn(&self) -> PFN_vkGetImageDrmFormatModifierPropertiesEXT {
                (**self).get_image_drm_format_modifier_properties_ext_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))] {
            fn get_fence_fd_khr_fn(&self) -> PFN_vkGetFenceFdKHR {
                (**self).get_fence_fd_khr_fn()
            }
            fn import_fence_fd_khr_fn(&self) -> PFN_vkImportFenceFdKHR {
                (**self).import_fence_fd_khr_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_EXT_full_screen_exclusive"))] {
            fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkAcquireFullScreenExclusiveModeEXT {
                (**self).acquire_full_screen_exclusive_mode_ext_fn()
            }
            fn release_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkReleaseFullScreenExclusiveModeEXT {
                (**self).release_full_screen_exclusive_mode_ext_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))] {
            fn get_memory_fd_khr_fn(&self) -> PFN_vkGetMemoryFdKHR {
                (**self).get_memory_fd_khr_fn()
            }
            fn get_memory_fd_properties_khr_fn(&self) -> PFN_vkGetMemoryFdPropertiesKHR {
                (**self).get_memory_fd_properties_khr_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_EXT_external_memory_host"))] {
            fn get_memory_host_pointer_properties_ext_fn(&self) -> PFN_vkGetMemoryHostPointerPropertiesEXT {
                (**self).get_memory_host_pointer_properties_ext_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))] {
            fn import_semaphore_win32_handle_khr_fn(&self) -> PFN_vkImportSemaphoreWin32HandleKHR {
                (**self).import_semaphore_win32_handle_khr_fn()
            }
            fn get_semaphore_win32_handle_khr_fn(&self) -> PFN_vkGetSemaphoreWin32HandleKHR {
                (**self).get_semaphore_win32_handle_khr_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))] {
            fn get_memory_win32_handle_khr_fn(&self) -> PFN_vkGetMemoryWin32HandleKHR {
                (**self).get_memory_win32_handle_khr_fn()
            }
            fn get_memory_win32_handle_properties_khr_fn(&self) -> PFN_vkGetMemoryWin32HandlePropertiesKHR {
                (**self).get_memory_win32_handle_properties_khr_fn()
            }
        }
    }
});
GuardsImpl!(for Device {
    #[cfg(all(feature = "VK_KHR_maintenance1", feature = "Implements"))]
    fn get_trim_command_pool_khr_fn(&self) -> PFN_vkTrimCommandPoolKHR {
        (**self).get_trim_command_pool_khr_fn()
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements"))] {
            fn create_descriptor_update_template_khr_fn(&self) -> PFN_vkCreateDescriptorUpdateTemplateKHR {
                (**self).create_descriptor_update_template_khr_fn()
            }
            fn destroy_descriptor_update_template_khr_fn(&self) -> PFN_vkDestroyDescriptorUpdateTemplateKHR {
                (**self).destroy_descriptor_update_template_khr_fn()
            }
            fn update_descriptor_set_with_template_khr_fn(&self) -> PFN_vkUpdateDescriptorSetWithTemplateKHR {
                (**self).update_descriptor_set_with_template_khr_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements"))] {
            fn bind_buffer_memory2_khr_fn(&self) -> PFN_vkBindBufferMemory2KHR {
                (**self).bind_buffer_memory2_khr_fn()
            }
            fn bind_image_memory2_khr_fn(&self) -> PFN_vkBindImageMemory2KHR {
                (**self).bind_image_memory2_khr_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "Implements"))] {
            fn get_image_drm_format_modifier_properties_ext_fn(&self) -> PFN_vkGetImageDrmFormatModifierPropertiesEXT {
                (**self).get_image_drm_format_modifier_properties_ext_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))] {
            fn get_fence_fd_khr_fn(&self) -> PFN_vkGetFenceFdKHR {
                (**self).get_fence_fd_khr_fn()
            }
            fn import_fence_fd_khr_fn(&self) -> PFN_vkImportFenceFdKHR {
                (**self).import_fence_fd_khr_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_EXT_full_screen_exclusive"))] {
            fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkAcquireFullScreenExclusiveModeEXT {
                (**self).acquire_full_screen_exclusive_mode_ext_fn()
            }
            fn release_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkReleaseFullScreenExclusiveModeEXT {
                (**self).release_full_screen_exclusive_mode_ext_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))] {
            fn get_memory_fd_khr_fn(&self) -> PFN_vkGetMemoryFdKHR {
                (**self).get_memory_fd_khr_fn()
            }
            fn get_memory_fd_properties_khr_fn(&self) -> PFN_vkGetMemoryFdPropertiesKHR {
                (**self).get_memory_fd_properties_khr_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_EXT_external_memory_host"))] {
            fn get_memory_host_pointer_properties_ext_fn(&self) -> PFN_vkGetMemoryHostPointerPropertiesEXT {
                (**self).get_memory_host_pointer_properties_ext_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))] {
            fn import_semaphore_win32_handle_khr_fn(&self) -> PFN_vkImportSemaphoreWin32HandleKHR {
                (**self).import_semaphore_win32_handle_khr_fn()
            }
            fn get_semaphore_win32_handle_khr_fn(&self) -> PFN_vkGetSemaphoreWin32HandleKHR {
                (**self).get_semaphore_win32_handle_khr_fn()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))] {
            fn get_memory_win32_handle_khr_fn(&self) -> PFN_vkGetMemoryWin32HandleKHR {
                (**self).get_memory_win32_handle_khr_fn()
            }
            fn get_memory_win32_handle_properties_khr_fn(&self) -> PFN_vkGetMemoryWin32HandlePropertiesKHR {
                (**self).get_memory_win32_handle_properties_khr_fn()
            }
        }
    }
});

/// Child of a device object
pub trait DeviceChild {
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

    fn device(&self) -> &Self::ConcreteDevice { T::device(&self) }
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

pub trait Queue: VkHandle<Handle = VkQueue> + DeviceChild {
    /// Wait for a object to become idle
    #[cfg(feature = "Implements")]
    fn wait(&mut self) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkresolve::queue_wait_idle(self.native_ptr_mut())
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
    #[cfg(feature = "Implements")]
    fn bind_sparse(
        &mut self,
        batches: &[impl SparseBindingOpBatch],
        fence: Option<&mut (impl crate::Fence + VkHandleMut)>,
    ) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        let batches: Vec<_> = batches.iter().map(SparseBindingOpBatch::make_info_struct).collect();

        self.bind_sparse_raw(&batches, fence)
    }

    /// Bind device memory to a sparse resource object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[cfg(feature = "Implements")]
    fn bind_sparse_raw(
        &mut self,
        batches: &[VkBindSparseInfo],
        fence: Option<&mut (impl crate::Fence + VkHandleMut)>,
    ) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkresolve::queue_bind_sparse(
                self.native_ptr_mut(),
                batches.len() as _,
                batches.as_ptr(),
                fence.map_or(VkFence::NULL, VkHandleMut::native_ptr_mut),
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
    #[cfg(feature = "Implements")]
    fn submit(
        &mut self,
        batches: &[impl SubmissionBatch],
        fence: Option<&mut (impl crate::Fence + VkHandleMut)>,
    ) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        let batch_resources: Vec<_> = batches
            .iter()
            .map(|b| {
                let mut resources = TemporalSubmissionBatchResources::new();
                b.collect_resources(&mut resources);
                resources
            })
            .collect();
        let batches: Vec<_> = batch_resources
            .iter()
            .map(TemporalSubmissionBatchResources::make_info_struct)
            .collect();

        self.submit_raw(&batches, fence)
    }

    /// Submits a sequence of semaphores or command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[cfg(feature = "Implements")]
    fn submit_raw(
        &mut self,
        batches: &[VkSubmitInfo],
        fence: Option<&mut (impl crate::Fence + VkHandleMut)>,
    ) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkresolve::queue_submit(
                self.native_ptr_mut(),
                batches.len() as _,
                batches.as_ptr(),
                fence.map_or(VkFence::NULL, VkHandleMut::native_ptr_mut),
            )
            .into_result()
            .map(drop)
        }
    }

    /// Queue images for presentation
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    /// * `VK_ERROR_OUT_OF_DATE_KHR`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[cfg(feature = "Implements")]
    #[cfg(feature = "VK_KHR_swapchain")]
    fn present(
        &mut self,
        swapchains: &mut [(&mut (impl crate::Swapchain + VkHandleMut), u32)],
        wait_semaphores: &mut [impl VkHandleMut<Handle = VkSemaphore>],
    ) -> crate::Result<Vec<VkResult>>
    where
        Self: VkHandleMut,
    {
        let mut res = vec![0; swapchains.len()];
        let wait_semaphores = wait_semaphores
            .iter_mut()
            .map(VkHandleMut::native_ptr_mut)
            .collect::<Vec<_>>();
        let (swapchains, indices): (Vec<_>, Vec<_>) = swapchains
            .iter_mut()
            .map(|&mut (ref mut x, n)| (x.native_ptr_mut(), n))
            .unzip();
        let pinfo = VkPresentInfoKHR {
            sType: VkPresentInfoKHR::TYPE,
            pNext: std::ptr::null(),
            waitSemaphoreCount: wait_semaphores.len() as _,
            pWaitSemaphores: wait_semaphores.as_ptr(),
            swapchainCount: swapchains.len() as _,
            pSwapchains: swapchains.as_ptr(),
            pImageIndices: indices.as_ptr(),
            pResults: res.as_mut_ptr(),
        };
        unsafe {
            crate::vkresolve::queue_present_khr(self.native_ptr_mut(), &pinfo)
                .into_result()
                .map(|_| res)
        }
    }
}
