//! Vulkan Device and Queues

use crate::ffi_helper::{
    ArrayFFIExtensions, CStrFFIRef, opt_pointer, slice_as_mut_ptr_empty_null, slice_as_ptr_empty_null,
};
use crate::*;
use cfg_if::cfg_if;
use derives::implements;

#[implements]
#[allow(dead_code)]
type DeviceResolvedFn<F> = crate::resolver::ResolvedFnCell<F, VkDevice>;
#[implements]
impl crate::resolver::ResolverInterface for VkDevice {
    #[inline(always)]
    unsafe fn load_symbol_unconstrainted<T: crate::resolver::FromPtr>(&self, name: &core::ffi::CStr) -> T {
        unsafe {
            T::from_ptr(core::mem::transmute(crate::vkfn::get_device_proc_addr(
                *self,
                name.as_ptr() as _,
            )))
        }
    }

    #[inline(always)]
    unsafe fn load_function_unconstrainted<F: crate::resolver::PFN>(&self) -> F {
        unsafe {
            F::from_void_fn(
                crate::vkfn::get_device_proc_addr(*self, F::NAME_CSTR.as_ptr() as _)
                    .unwrap_or_else(|| panic!("function {:?} not found", F::NAME_CSTR)),
            )
        }
    }
}

#[implements]
struct DeviceExtFunctions {
    #[cfg(all(feature = "VK_KHR_maintenance1", not(feature = "Allow1_1APIs")))]
    trim_command_pool_khr: DeviceResolvedFn<PFN_vkTrimCommandPoolKHR>,
    #[cfg(all(feature = "VK_KHR_descriptor_update_template", not(feature = "Allow1_1APIs")))]
    create_descriptor_update_template_khr: DeviceResolvedFn<PFN_vkCreateDescriptorUpdateTemplateKHR>,
    #[cfg(all(feature = "VK_KHR_descriptor_update_template", not(feature = "Allow1_1APIs")))]
    destroy_descriptor_update_template_khr: DeviceResolvedFn<PFN_vkDestroyDescriptorUpdateTemplateKHR>,
    #[cfg(all(feature = "VK_KHR_descriptor_update_template", not(feature = "Allow1_1APIs")))]
    update_descriptor_set_with_template_khr: DeviceResolvedFn<PFN_vkUpdateDescriptorSetWithTemplateKHR>,
    #[cfg(all(feature = "VK_KHR_bind_memory2", not(feature = "Allow1_1APIs")))]
    bind_buffer_memory2_khr: DeviceResolvedFn<PFN_vkBindBufferMemory2KHR>,
    #[cfg(all(feature = "VK_KHR_bind_memory2", not(feature = "Allow1_1APIs")))]
    bind_image_memory2_khr: DeviceResolvedFn<PFN_vkBindImageMemory2KHR>,
    #[cfg(all(feature = "VK_EXT_image_drm_format_modifier"))]
    get_image_drm_format_modifier_properties_ext: DeviceResolvedFn<PFN_vkGetImageDrmFormatModifierPropertiesEXT>,
    #[cfg(all(feature = "VK_KHR_external_fence_fd"))]
    get_fence_fd_khr: DeviceResolvedFn<PFN_vkGetFenceFdKHR>,
    #[cfg(all(feature = "VK_KHR_external_fence_fd"))]
    import_fence_fd_khr: DeviceResolvedFn<PFN_vkImportFenceFdKHR>,
    #[cfg(all(feature = "VK_EXT_full_screen_exclusive"))]
    acquire_full_screen_exclusive_mode_ext: DeviceResolvedFn<PFN_vkAcquireFullScreenExclusiveModeEXT>,
    #[cfg(all(feature = "VK_EXT_full_screen_exclusive"))]
    release_full_screen_exclusive_mode_ext: DeviceResolvedFn<PFN_vkReleaseFullScreenExclusiveModeEXT>,
    #[cfg(all(feature = "VK_KHR_external_memory_fd"))]
    get_memory_fd_khr: DeviceResolvedFn<PFN_vkGetMemoryFdKHR>,
    #[cfg(all(feature = "VK_KHR_external_memory_fd"))]
    get_memory_fd_properties_khr: DeviceResolvedFn<PFN_vkGetMemoryFdPropertiesKHR>,
    #[cfg(all(feature = "VK_EXT_external_memory_host"))]
    get_memory_host_pointer_properties_ext: DeviceResolvedFn<PFN_vkGetMemoryHostPointerPropertiesEXT>,
    #[cfg(all(feature = "VK_KHR_external_semaphore_win32"))]
    import_semaphore_win32_handle_khr: DeviceResolvedFn<PFN_vkImportSemaphoreWin32HandleKHR>,
    #[cfg(all(feature = "VK_KHR_external_semaphore_win32"))]
    get_semaphore_win32_handle_khr: DeviceResolvedFn<PFN_vkGetSemaphoreWin32HandleKHR>,
    #[cfg(all(feature = "VK_KHR_external_memory_win32"))]
    get_memory_win32_handle_khr: DeviceResolvedFn<PFN_vkGetMemoryWin32HandleKHR>,
    #[cfg(all(feature = "VK_KHR_external_memory_win32"))]
    get_memory_win32_handle_properties_khr: DeviceResolvedFn<PFN_vkGetMemoryWin32HandlePropertiesKHR>,
    #[cfg(all(feature = "VK_KHR_get_memory_requirements2", not(feature = "Allow1_1APIs")))]
    get_buffer_memory_requirements_2_khr: DeviceResolvedFn<PFN_vkGetBufferMemoryRequirements2KHR>,
    #[cfg(all(feature = "VK_KHR_get_memory_requirements2", not(feature = "Allow1_1APIs")))]
    get_image_memory_requirements_2_khr: DeviceResolvedFn<PFN_vkGetImageMemoryRequirements2KHR>,
    #[cfg(all(feature = "VK_KHR_get_memory_requirements2", not(feature = "Allow1_1APIs")))]
    get_image_sparse_memory_requirements_2_khr: DeviceResolvedFn<PFN_vkGetImageSparseMemoryRequirements2KHR>,
    #[cfg(all(feature = "VK_KHR_create_renderpass2", not(feature = "Allow1_2APIs")))]
    create_render_pass_2_khr: DeviceResolvedFn<PFN_vkCreateRenderPass2KHR>,
    #[cfg(all(feature = "VK_KHR_create_renderpass2", not(feature = "Allow1_2APIs")))]
    cmd_begin_render_pass_2_khr: DeviceResolvedFn<PFN_vkCmdBeginRenderPass2KHR>,
    #[cfg(all(feature = "VK_KHR_create_renderpass2", not(feature = "Allow1_2APIs")))]
    cmd_end_render_pass_2_khr: DeviceResolvedFn<PFN_vkCmdEndRenderPass2KHR>,
    #[cfg(all(feature = "VK_KHR_create_renderpass2", not(feature = "Allow1_2APIs")))]
    cmd_next_subpass_2_khr: DeviceResolvedFn<PFN_vkCmdNextSubpass2KHR>,
    #[cfg(all(feature = "VK_KHR_synchronization2", not(feature = "Allow1_3APIs")))]
    cmd_pipeline_barrier_2_khr: DeviceResolvedFn<PFN_vkCmdPipelineBarrier2KHR>,
    #[cfg(all(feature = "VK_KHR_push_descriptor", not(feature = "Allow1_4APIs")))]
    cmd_push_descriptor_set_khr: DeviceResolvedFn<PFN_vkCmdPushDescriptorSetKHR>,
    #[cfg(all(feature = "VK_EXT_sample_locations"))]
    cmd_set_sample_locations_ext: DeviceResolvedFn<PFN_vkCmdSetSampleLocationsEXT>,
}
#[implements]
impl DeviceExtFunctions {
    const fn new(handle: VkDevice) -> Self {
        Self {
            #[cfg(all(feature = "VK_KHR_maintenance1", not(feature = "Allow1_1APIs")))]
            trim_command_pool_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_descriptor_update_template", not(feature = "Allow1_1APIs")))]
            create_descriptor_update_template_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_descriptor_update_template", not(feature = "Allow1_1APIs")))]
            destroy_descriptor_update_template_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_descriptor_update_template", not(feature = "Allow1_1APIs")))]
            update_descriptor_set_with_template_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_bind_memory2", not(feature = "Allow1_1APIs")))]
            bind_buffer_memory2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_bind_memory2", not(feature = "Allow1_1APIs")))]
            bind_image_memory2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_EXT_image_drm_format_modifier"))]
            get_image_drm_format_modifier_properties_ext: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_external_fence_fd"))]
            get_fence_fd_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_external_fence_fd"))]
            import_fence_fd_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_EXT_full_screen_exclusive"))]
            acquire_full_screen_exclusive_mode_ext: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_EXT_full_screen_exclusive"))]
            release_full_screen_exclusive_mode_ext: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_external_memory_fd"))]
            get_memory_fd_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_external_memory_fd"))]
            get_memory_fd_properties_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_EXT_external_memory_host"))]
            get_memory_host_pointer_properties_ext: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_external_semaphore_win32"))]
            import_semaphore_win32_handle_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_external_semaphore_win32"))]
            get_semaphore_win32_handle_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_external_memory_win32"))]
            get_memory_win32_handle_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_external_memory_win32"))]
            get_memory_win32_handle_properties_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_get_memory_requirements2", not(feature = "Allow1_1APIs")))]
            get_buffer_memory_requirements_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_get_memory_requirements2", not(feature = "Allow1_1APIs")))]
            get_image_memory_requirements_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_get_memory_requirements2", not(feature = "Allow1_1APIs")))]
            get_image_sparse_memory_requirements_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_create_renderpass2", not(feature = "Allow1_2APIs")))]
            create_render_pass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_create_renderpass2", not(feature = "Allow1_2APIs")))]
            cmd_begin_render_pass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_create_renderpass2", not(feature = "Allow1_2APIs")))]
            cmd_end_render_pass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_create_renderpass2", not(feature = "Allow1_2APIs")))]
            cmd_next_subpass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_synchronization2", not(feature = "Allow1_3APIs")))]
            cmd_pipeline_barrier_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_KHR_push_descriptor", not(feature = "Allow1_4APIs")))]
            cmd_push_descriptor_set_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "VK_EXT_sample_locations"))]
            cmd_set_sample_locations_ext: DeviceResolvedFn::new(handle),
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
impl<Instance: crate::Instance> Device for DeviceObject<Instance> {
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_maintenance1")]
    fn get_trim_command_pool_khr_fn(&self) -> PFN_vkTrimCommandPoolKHR {
        *self.ext.trim_command_pool_khr.resolve()
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_descriptor_update_template", feature = "Implements", not(feature = "Allow1_1APIs")))] {
            fn create_descriptor_update_template_khr_fn(&self) -> PFN_vkCreateDescriptorUpdateTemplateKHR {
                *self.ext.create_descriptor_update_template_khr.resolve()
            }
            fn destroy_descriptor_update_template_khr_fn(&self) -> PFN_vkDestroyDescriptorUpdateTemplateKHR {
                *self.ext.destroy_descriptor_update_template_khr.resolve()
            }
            fn update_descriptor_set_with_template_khr_fn(&self) -> PFN_vkUpdateDescriptorSetWithTemplateKHR {
                *self.ext.update_descriptor_set_with_template_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_bind_memory2", feature = "Implements", not(feature = "Allow1_1APIs")))] {
            fn bind_buffer_memory2_khr_fn(&self) -> PFN_vkBindBufferMemory2KHR {
                *self.ext.bind_buffer_memory2_khr.resolve()
            }
            fn bind_image_memory2_khr_fn(&self) -> PFN_vkBindImageMemory2KHR {
                *self.ext.bind_image_memory2_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "VK_EXT_image_drm_format_modifier", feature = "Implements"))] {
            fn get_image_drm_format_modifier_properties_ext_fn(&self) -> PFN_vkGetImageDrmFormatModifierPropertiesEXT {
                *self.ext.get_image_drm_format_modifier_properties_ext.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))] {
            fn get_fence_fd_khr_fn(&self) -> PFN_vkGetFenceFdKHR {
                *self.ext.get_fence_fd_khr.resolve()
            }
            fn import_fence_fd_khr_fn(&self) -> PFN_vkImportFenceFdKHR {
                *self.ext.import_fence_fd_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))] {
            fn get_memory_fd_khr_fn(&self) -> PFN_vkGetMemoryFdKHR {
                *self.ext.get_memory_fd_khr.resolve()
            }
            fn get_memory_fd_properties_khr_fn(&self) -> PFN_vkGetMemoryFdPropertiesKHR {
                *self.ext.get_memory_fd_properties_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_EXT_external_memory_host"))] {
            fn get_memory_host_pointer_properties_ext_fn(&self) -> PFN_vkGetMemoryHostPointerPropertiesEXT {
                *self.ext.get_memory_host_pointer_properties_ext.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2", not(feature = "Allow1_1APIs")))] {
            fn get_buffer_memory_requirements_2_khr_fn(&self) -> PFN_vkGetBufferMemoryRequirements2KHR {
                *self.ext.get_buffer_memory_requirements_2_khr.resolve()
            }

            fn get_image_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageMemoryRequirements2KHR {
                *self.ext.get_image_memory_requirements_2_khr.resolve()
            }

            fn get_image_sparse_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageSparseMemoryRequirements2KHR {
                *self.ext.get_image_sparse_memory_requirements_2_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2", not(feature = "Allow1_2APIs")))] {
            fn create_render_pass_2_khr_fn(&self) -> PFN_vkCreateRenderPass2KHR {
                *self.ext.create_render_pass_2_khr.resolve()
            }
        }
    }
}
#[implements("VK_KHR_external_semaphore_win32")]
impl<Instance: crate::Instance> DeviceExternalSemaphoreWin32Extension for DeviceObject<Instance> {
    #[inline(always)]
    fn import_semaphore_win32_handle_khr_fn(&self) -> PFN_vkImportSemaphoreWin32HandleKHR {
        *self.ext.import_semaphore_win32_handle_khr.resolve()
    }

    #[inline(always)]
    fn get_semaphore_win32_handle_khr_fn(&self) -> PFN_vkGetSemaphoreWin32HandleKHR {
        *self.ext.get_semaphore_win32_handle_khr.resolve()
    }
}
#[implements("VK_KHR_external_memory_win32")]
impl<Instance: crate::Instance> DeviceExternalMemoryWin32Extension for DeviceObject<Instance> {
    #[inline(always)]
    fn get_memory_win32_handle_khr_fn(&self) -> PFN_vkGetMemoryWin32HandleKHR {
        *self.ext.get_memory_win32_handle_khr.resolve()
    }

    #[inline(always)]
    fn get_memory_win32_handle_properties_khr_fn(&self) -> PFN_vkGetMemoryWin32HandlePropertiesKHR {
        *self.ext.get_memory_win32_handle_properties_khr.resolve()
    }
}
#[implements("VK_EXT_full_screen_exclusive")]
impl<Instance: crate::Instance> DeviceFullScreenExclusiveExtension for DeviceObject<Instance> {
    #[inline(always)]
    fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkAcquireFullScreenExclusiveModeEXT {
        *self.ext.acquire_full_screen_exclusive_mode_ext.resolve()
    }

    #[inline(always)]
    fn release_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkReleaseFullScreenExclusiveModeEXT {
        *self.ext.release_full_screen_exclusive_mode_ext.resolve()
    }
}
#[implements]
impl<Instance: crate::Instance> DeviceExtCommandFunctionProvider for DeviceObject<Instance> {
    cfg_if! {
        if #[cfg(all(feature = "VK_KHR_create_renderpass2", not(feature = "Allow1_2APIs")))] {
            fn cmd_begin_render_pass_2_khr_fn(&self) -> PFN_vkCmdBeginRenderPass2KHR {
                *self.ext.cmd_begin_render_pass_2_khr.resolve()
            }

            fn cmd_end_render_pass_2_khr_fn(&self) -> PFN_vkCmdEndRenderPass2KHR {
                *self.ext.cmd_end_render_pass_2_khr.resolve()
            }

            fn cmd_next_subpass_2_khr_fn(&self) -> PFN_vkCmdNextSubpass2KHR {
                *self.ext.cmd_next_subpass_2_khr.resolve()
            }
        }
    }

    #[cfg(feature = "VK_KHR_synchronization2")]
    #[cfg(not(feature = "Allow1_3APIs"))]
    fn cmd_pipeline_barrier_2_khr_fn(&self) -> PFN_vkCmdPipelineBarrier2KHR {
        *self.ext.cmd_pipeline_barrier_2_khr.resolve()
    }

    #[cfg(feature = "VK_KHR_push_descriptor")]
    #[cfg(not(feature = "Allow1_4APIs"))]
    fn cmd_push_descriptor_set_khr_fn(&self) -> PFN_vkCmdPushDescriptorSetKHR {
        *self.ext.cmd_push_descriptor_set_khr.resolve()
    }

    #[cfg(feature = "VK_EXT_sample_locations")]
    fn cmd_set_sample_locations_ext_fn(&self) -> PFN_vkCmdSetSampleLocationsEXT {
        *self.ext.cmd_set_sample_locations_ext.resolve()
    }
}
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
        let r = QueueObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceCreateInfo<'d>(
    VkDeviceCreateInfo,
    core::marker::PhantomData<(
        Option<&'d dyn VulkanStructureAsRef>,
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

    pub fn with_next(mut self, next: &'d (impl VulkanStructureAsRef + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalDeviceFeatures2<'r>(
    VkPhysicalDeviceFeatures2KHR,
    core::marker::PhantomData<Option<&'r mut dyn VulkanStructureAsRef>>,
);
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl<'r> PhysicalDeviceFeatures2<'r> {
    pub const fn new(old_features: VkPhysicalDeviceFeatures) -> Self {
        Self(
            VkPhysicalDeviceFeatures2KHR {
                sType: <VkPhysicalDeviceFeatures2KHR as VulkanStructure>::TYPE,
                pNext: core::ptr::null_mut(),
                features: old_features,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'r mut (impl VulkanStructureAsRef + ?Sized)) -> Self {
        self.0.pNext = next.as_generic_mut() as *mut _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl VulkanStructureAsRef for PhysicalDeviceFeatures2<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        VulkanStructureAsRef::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        VulkanStructureAsRef::as_generic_mut(&mut self.0)
    }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl VulkanSinkStructureAsRef for PhysicalDeviceFeatures2<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanSinkStructure {
        VulkanSinkStructureAsRef::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanSinkStructure {
        VulkanSinkStructureAsRef::as_generic_mut(&mut self.0)
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalDeviceSynchronization2Features<'r>(
    VkPhysicalDeviceSynchronization2FeaturesKHR,
    core::marker::PhantomData<Option<&'r mut dyn VulkanStructureAsRef>>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'r> PhysicalDeviceSynchronization2Features<'r> {
    pub const fn new(enabled: bool) -> Self {
        Self(
            VkPhysicalDeviceSynchronization2FeaturesKHR {
                sType: <VkPhysicalDeviceSynchronization2FeaturesKHR as VulkanStructure>::TYPE,
                pNext: core::ptr::null_mut(),
                synchronization2: enabled as _,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'r mut (impl VulkanStructureAsRef + ?Sized)) -> Self {
        self.0.pNext = next.as_generic_mut() as *mut _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
unsafe impl VulkanStructureAsRef for PhysicalDeviceSynchronization2Features<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        VulkanStructureAsRef::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        VulkanStructureAsRef::as_generic_mut(&mut self.0)
    }
}
#[cfg(feature = "VK_KHR_synchronization2")]
unsafe impl VulkanSinkStructureAsRef for PhysicalDeviceSynchronization2Features<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanSinkStructure {
        VulkanSinkStructureAsRef::as_generic(&self.0)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanSinkStructure {
        VulkanSinkStructureAsRef::as_generic_mut(&mut self.0)
    }
}

pub trait Device: VkHandle<Handle = VkDevice> + InstanceChild {
    /// Get a queue handle from a device
    #[implements]
    fn queue<'s>(&'s self, family_index: u32, queue_index: u32) -> QueueObject<&'s Self> {
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
    #[implements("VK_KHR_create_renderpass2")]
    #[inline]
    fn new_render_pass2(
        &self,
        info: &RenderPassCreateInfo2,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkRenderPass> {
        let mut h = core::mem::MaybeUninit::uninit();

        #[cfg(feature = "Allow1_2APIs")]
        unsafe {
            crate::vkfn::create_render_pass2(
                self.native_ptr(),
                info as *const _ as _,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }
        #[cfg(not(feature = "Allow1_2APIs"))]
        unsafe {
            (self.create_render_pass_2_khr_fn().0)(
                self.native_ptr(),
                info as *const _ as _,
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
        objects: &mut [VkPipeline],
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::create_graphics_pipelines(
                self.native_ptr(),
                cache.unwrap_or(VkPipelineCache::NULL),
                infos.len() as _,
                infos.as_ptr_empty_null() as _,
                opt_pointer(allocation_callbacks),
                objects.as_mut_ptr(),
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
        cache: Option<&impl crate::PipelineCache>,
    ) -> crate::Result<Vec<crate::PipelineObject<&'s Self>>> {
        let mut hs = vec![VkPipeline::NULL; infos.len()];

        unsafe {
            self.new_graphics_pipelines_raw(infos, cache.map(VkHandle::native_ptr), None, &mut hs)?;
        }

        Ok(crate::alloc::collect_vec(
            hs.into_iter()
                .map(move |h| unsafe { crate::PipelineObject::manage(h, self) }),
        ))
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
        cache: Option<&impl crate::PipelineCache>,
    ) -> crate::Result<[crate::PipelineObject<&'s Self>; N]> {
        let mut hs = [VkPipeline::NULL; N];

        unsafe {
            self.new_graphics_pipelines_raw(infos, cache.map(VkHandle::native_ptr), None, &mut hs)?;
        }

        Ok(core::array::from_fn(move |n| unsafe {
            crate::PipelineObject::manage(hs[n], self)
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
        objects: &mut [VkPipeline],
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::create_compute_pipelines(
                self.native_ptr(),
                cache.unwrap_or(VkPipelineCache::NULL),
                infos.len() as _,
                infos.as_ptr_empty_null() as _,
                opt_pointer(allocation_callbacks),
                objects.as_mut_ptr(),
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
        cache: Option<&(impl crate::PipelineCache + ?Sized)>,
    ) -> crate::Result<Vec<crate::PipelineObject<&'s Self>>> {
        let mut pipelines = vec![VkPipeline::NULL; infos.len()];

        unsafe {
            self.new_compute_pipelines_raw(infos, cache.map(VkHandle::native_ptr), None, &mut pipelines)?;
        }

        Ok(crate::alloc::collect_vec(
            pipelines
                .into_iter()
                .map(move |h| unsafe { crate::PipelineObject::manage(h, self) }),
        ))
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
        cache: Option<&(impl crate::PipelineCache + ?Sized)>,
    ) -> crate::Result<[crate::PipelineObject<&'s Self>; N]> {
        let mut pipelines = [VkPipeline::NULL; N];

        unsafe {
            self.new_compute_pipelines_raw(infos, cache.map(VkHandle::native_ptr), None, &mut pipelines)?;
        }

        Ok(core::array::from_fn(move |n| unsafe {
            crate::PipelineObject::manage(pipelines[n], self)
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
                opt_pointer(allocation_callbacks),
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
                opt_pointer(allocation_callbacks),
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
        sink: &mut [CommandBufferObject<&'s Self>],
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
        let mut sink = unsafe { crate::alloc::alloc_sink_buffer(info.0.commandBufferCount as _) };
        unsafe {
            self.allocate_command_buffers(info, &mut sink)?;
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
        let mut sink =
            [const { unsafe { core::mem::MaybeUninit::<CommandBufferObject<&'s Self>>::zeroed().assume_init() } }; N];
        unsafe {
            crate::vkfn::allocate_command_buffers(self.native_ptr(), info as *const _ as _, sink.as_mut_ptr() as _)
                .into_result()?;
        }

        Ok(sink)
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
                ranges.as_ptr_empty_null() as _,
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
                ranges.as_ptr_empty_null() as _,
            )
            .into_result()
            .map(drop)
        }
    }

    /// Update the contents of descriptor set objects
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
    #[implements("VK_KHR_get_memory_requirements2")]
    #[inline]
    unsafe fn get_image_sparse_memory_requirements2_count(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2,
        count_sink: &mut core::mem::MaybeUninit<u32>,
        sink_head_ptr: *mut VkSparseImageMemoryRequirements2KHR,
    ) {
        #[cfg(feature = "Allow1_1APIs")]
        unsafe {
            crate::vkfn::get_image_sparse_memory_requirements2(
                self.native_ptr(),
                &info.0,
                count_sink.as_mut_ptr(),
                sink_head_ptr,
            )
        }

        #[cfg(not(feature = "Allow1_1APIs"))]
        unsafe {
            self.get_image_sparse_memory_requirements_2_khr_fn().0(
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
    #[inline]
    unsafe fn bind_buffer_raw(
        &self,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::bind_buffer_memory(self.native_ptr(), buffer, memory, offset)
                .into_result()
                .map(drop)
        }
    }

    /// Multiple Binding for Buffers
    #[implements("VK_KHR_bind_memory2")]
    #[inline]
    unsafe fn bind_buffers_raw(&self, bounds: &[VkBindBufferMemoryInfoKHR]) -> crate::Result<()> {
        #[cfg(feature = "Allow1_1APIs")]
        unsafe {
            crate::vkfn::bind_buffer_memory2(self.native_ptr(), bounds.len() as _, bounds.as_ptr_empty_null())
                .into_result()
                .map(drop)
        }

        #[cfg(not(feature = "Allow1_1APIs"))]
        unsafe {
            self.bind_buffer_memory2_khr_fn().0(self.native_ptr(), bounds.len() as _, bounds.as_ptr_empty_null())
                .into_result()
                .map(drop)
        }
    }

    /// Single binding for an image
    /// # Safety
    /// `VkImage` and `VkDeviceMemory` must be valid and created from this device object
    #[implements]
    #[inline]
    unsafe fn bind_image_raw(&self, image: VkImage, memory: VkDeviceMemory, offset: VkDeviceSize) -> crate::Result<()> {
        unsafe {
            crate::vkfn::bind_image_memory(self.native_ptr(), image, memory, offset)
                .into_result()
                .map(drop)
        }
    }

    /// Multiple Binding for Images
    #[implements("VK_KHR_bind_memory2")]
    #[inline]
    unsafe fn bind_images_raw(&self, bounds: &[VkBindImageMemoryInfoKHR]) -> crate::Result<()> {
        #[cfg(feature = "Allow1_1APIs")]
        unsafe {
            crate::vkfn::bind_image_memory2(self.native_ptr(), bounds.len() as _, bounds.as_ptr_empty_null())
                .into_result()
                .map(drop)
        }

        #[cfg(not(feature = "Allow1_1APIs"))]
        unsafe {
            self.bind_image_memory2_khr_fn().0(self.native_ptr(), bounds.len() as _, bounds.as_ptr_empty_null())
                .into_result()
                .map(drop)
        }
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
                objects.as_ptr_empty_null() as _,
                wait_all as _,
                timeout.unwrap_or(std::u64::MAX),
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
            crate::vkfn::reset_fences(self.native_ptr(), objects.len() as _, objects.as_ptr_empty_null() as _)
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
                opt_pointer(allocation_callbacks),
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
        Self::ConcreteInstance: InstanceExtensions,
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
    #[implements("VK_KHR_descriptor_update_template")]
    unsafe fn new_descriptor_update_template_raw(
        &self,
        info: &VkDescriptorUpdateTemplateCreateInfoKHR,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkDescriptorUpdateTemplateKHR> {
        let mut h = core::mem::MaybeUninit::uninit();

        #[cfg(feature = "Allow1_1APIs")]
        unsafe {
            crate::vkfn::create_descriptor_update_template(
                self.native_ptr(),
                info,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }
        #[cfg(not(feature = "Allow1_1APIs"))]
        unsafe {
            self.create_descriptor_update_template_khr_fn().0(
                self.native_ptr(),
                info,
                opt_pointer(allocation_callbacks),
                h.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Create a new descriptor update template
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    #[cfg(not(feature = "VK_KHR_push_descriptor"))]
    fn new_descriptor_update_template<'s>(
        &'s self,
        entries: &[VkDescriptorUpdateTemplateEntryKHR],
        dsl: &impl crate::DescriptorSetLayout,
    ) -> crate::Result<crate::DescriptorUpdateTemplateObject<&'s Self>> {
        let cinfo = VkDescriptorUpdateTemplateCreateInfoKHR {
            sType: VkDescriptorUpdateTemplateCreateInfoKHR::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
            set: 0,
            pipelineLayout: VkPipelineLayout::NULL,
            descriptorUpdateEntryCount: entries.len() as _,
            pDescriptorUpdateEntries: entries.as_ptr_empty_null(),
            templateType: VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET,
            descriptorSetLayout: dsl.native_ptr(),
        };

        Ok(crate::DescriptorUpdateTemplateObject(
            unsafe { self.new_descriptor_update_template_raw(&cinfo, None)? },
            self,
        ))
    }

    /// Create a new descriptor update template
    /// # Bedrock extension
    /// dsl: NoneにするとPushDescriptors向けのテンプレートを作成できる
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    #[cfg(feature = "VK_KHR_push_descriptor")]
    fn new_descriptor_update_template<'s>(
        &'s self,
        entries: &[VkDescriptorUpdateTemplateEntryKHR],
        dsl: Option<&impl crate::DescriptorSetLayout>,
    ) -> crate::Result<crate::DescriptorUpdateTemplateObject<&'s Self>> {
        use crate::VkRawHandle;

        let cinfo = VkDescriptorUpdateTemplateCreateInfoKHR {
            sType: VkDescriptorUpdateTemplateCreateInfoKHR::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
            set: 0,
            pipelineLayout: VkPipelineLayout::NULL,
            descriptorUpdateEntryCount: entries.len() as _,
            pDescriptorUpdateEntries: entries.as_ptr_empty_null(),
            templateType: if dsl.is_none() {
                VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR
            } else {
                VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET
            },
            descriptorSetLayout: dsl.map_or(VkDescriptorSetLayout::NULL, VkHandle::native_ptr),
        };

        Ok(crate::DescriptorUpdateTemplateObject(
            unsafe { self.new_descriptor_update_template_raw(&cinfo, None)? },
            self,
        ))
    }

    /// Get Properties of External Memory File Descriptors
    /// # Safety
    /// sink must be constructed correctly
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements("VK_KHR_external_memory_fd")]
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

    /// Get a POSIX file descriptor for a memory object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements("VK_KHR_external_memory_fd")]
    #[inline]
    fn get_memory_fd(&self, info: &crate::MemoryGetFdInfo) -> crate::Result<std::os::unix::io::RawFd> {
        let mut fd = core::mem::MaybeUninit::uninit();

        unsafe {
            self.get_memory_fd_khr_fn().0(self.native_ptr(), &info.0, fd.as_mut_ptr()).into_result()?;

            Ok(fd.assume_init())
        }
    }

    /// Get Properties of external memory host pointer
    /// # Safety
    /// sink must be constructed correctly
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements("VK_EXT_external_memory_host")]
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

    /// Import a fence from a POSIX file descriptor
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements("VK_KHR_external_fence_fd")]
    #[inline]
    fn import_fence_fd(&self, info: &crate::ImportFenceFdInfo) -> crate::Result<()> {
        unsafe {
            self.import_fence_fd_khr_fn().0(self.native_ptr(), &info.0)
                .into_result()
                .map(drop)
        }
    }

    /// Get a POSIX file descriptor handle for a type
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements("VK_KHR_external_fence_fd")]
    #[inline]
    fn get_fence_fd(&self, info: &crate::FenceFdGetInfo) -> crate::Result<std::os::unix::io::RawFd> {
        let mut fd = core::mem::MaybeUninit::uninit();

        unsafe {
            self.get_fence_fd_khr_fn().0(self.native_ptr(), &info.0, fd.as_mut_ptr()).into_result()?;

            Ok(fd.assume_init())
        }
    }

    // Extension Function Providers

    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_maintenance1")]
    fn get_trim_command_pool_khr_fn(&self) -> PFN_vkTrimCommandPoolKHR;

    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_descriptor_update_template")]
    fn create_descriptor_update_template_khr_fn(&self) -> PFN_vkCreateDescriptorUpdateTemplateKHR;
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_descriptor_update_template")]
    fn destroy_descriptor_update_template_khr_fn(&self) -> PFN_vkDestroyDescriptorUpdateTemplateKHR;
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_descriptor_update_template")]
    fn update_descriptor_set_with_template_khr_fn(&self) -> PFN_vkUpdateDescriptorSetWithTemplateKHR;

    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_bind_memory2")]
    fn bind_buffer_memory2_khr_fn(&self) -> PFN_vkBindBufferMemory2KHR;
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_bind_memory2")]
    fn bind_image_memory2_khr_fn(&self) -> PFN_vkBindImageMemory2KHR;

    #[implements("VK_EXT_image_drm_format_modifier")]
    fn get_image_drm_format_modifier_properties_ext_fn(&self) -> PFN_vkGetImageDrmFormatModifierPropertiesEXT;

    #[implements("VK_KHR_external_fence_fd")]
    fn get_fence_fd_khr_fn(&self) -> PFN_vkGetFenceFdKHR;
    #[implements("VK_KHR_external_fence_fd")]
    fn import_fence_fd_khr_fn(&self) -> PFN_vkImportFenceFdKHR;

    #[implements("VK_KHR_external_memory_fd")]
    fn get_memory_fd_khr_fn(&self) -> PFN_vkGetMemoryFdKHR;
    #[implements("VK_KHR_external_memory_fd")]
    fn get_memory_fd_properties_khr_fn(&self) -> PFN_vkGetMemoryFdPropertiesKHR;

    #[implements("VK_EXT_external_memory_host")]
    fn get_memory_host_pointer_properties_ext_fn(&self) -> PFN_vkGetMemoryHostPointerPropertiesEXT;

    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_memory_requirements2")]
    fn get_buffer_memory_requirements_2_khr_fn(&self) -> PFN_vkGetBufferMemoryRequirements2KHR;
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_memory_requirements2")]
    fn get_image_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageMemoryRequirements2KHR;
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_memory_requirements2")]
    fn get_image_sparse_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageSparseMemoryRequirements2KHR;

    #[cfg(not(feature = "Allow1_2APIs"))]
    #[implements("VK_KHR_create_renderpass2")]
    fn create_render_pass_2_khr_fn(&self) -> PFN_vkCreateRenderPass2KHR;
}
DerefContainerWithGuardsBracketImpl!(for Device {
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_maintenance1")]
    ForwardFnPtr!(deref get_trim_command_pool_khr_fn -> PFN_vkTrimCommandPoolKHR);

    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_descriptor_update_template")]
    ForwardFnPtr!(deref create_descriptor_update_template_khr_fn -> PFN_vkCreateDescriptorUpdateTemplateKHR);
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_descriptor_update_template")]
    ForwardFnPtr!(deref destroy_descriptor_update_template_khr_fn -> PFN_vkDestroyDescriptorUpdateTemplateKHR);
        #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_descriptor_update_template")]
    ForwardFnPtr!(deref update_descriptor_set_with_template_khr_fn -> PFN_vkUpdateDescriptorSetWithTemplateKHR);

    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_bind_memory2")]
    ForwardFnPtr!(deref bind_buffer_memory2_khr_fn -> PFN_vkBindBufferMemory2KHR);
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_bind_memory2")]
    ForwardFnPtr!(deref bind_image_memory2_khr_fn -> PFN_vkBindImageMemory2KHR);

    #[implements("VK_EXT_image_drm_format_modifier")]
    ForwardFnPtr!(deref get_image_drm_format_modifier_properties_ext_fn -> PFN_vkGetImageDrmFormatModifierPropertiesEXT);

    #[implements("VK_KHR_external_fence_fd")]
    ForwardFnPtr!(deref get_fence_fd_khr_fn -> PFN_vkGetFenceFdKHR);
    #[implements("VK_KHR_external_fence_fd")]
    ForwardFnPtr!(deref import_fence_fd_khr_fn -> PFN_vkImportFenceFdKHR);

    #[implements("VK_KHR_external_memory_fd")]
    ForwardFnPtr!(deref get_memory_fd_khr_fn -> PFN_vkGetMemoryFdKHR);
    #[implements("VK_KHR_external_memory_fd")]
    ForwardFnPtr!(deref get_memory_fd_properties_khr_fn -> PFN_vkGetMemoryFdPropertiesKHR);

    #[implements("VK_EXT_external_memory_host")]
    ForwardFnPtr!(deref get_memory_host_pointer_properties_ext_fn -> PFN_vkGetMemoryHostPointerPropertiesEXT);

    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_memory_requirements2")]
    ForwardFnPtr!(deref get_buffer_memory_requirements_2_khr_fn -> PFN_vkGetBufferMemoryRequirements2KHR);
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_memory_requirements2")]
    ForwardFnPtr!(deref get_image_memory_requirements_2_khr_fn -> PFN_vkGetImageMemoryRequirements2KHR);
    #[cfg(not(feature = "Allow1_1APIs"))]
    #[implements("VK_KHR_get_memory_requirements2")]
    ForwardFnPtr!(deref get_image_sparse_memory_requirements_2_khr_fn -> PFN_vkGetImageSparseMemoryRequirements2KHR);

    #[cfg(not(feature = "Allow1_2APIs"))]
    #[implements("VK_KHR_create_renderpass2")]
    ForwardFnPtr!(deref create_render_pass_2_khr_fn -> PFN_vkCreateRenderPass2KHR);
});

#[implements("VK_KHR_external_semaphore_win32")]
pub trait DeviceExternalSemaphoreWin32Extension: Device {
    fn import_semaphore_win32_handle_khr_fn(&self) -> PFN_vkImportSemaphoreWin32HandleKHR;
    fn get_semaphore_win32_handle_khr_fn(&self) -> PFN_vkGetSemaphoreWin32HandleKHR;

    /// Import a semaphore from a Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_INVALID_EXTERNAL_HANDLE
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
#[implements("VK_KHR_external_semaphore_win32")]
DerefContainerWithGuardsBracketImpl!(for DeviceExternalSemaphoreWin32Extension {
    ForwardFnPtr!(deref import_semaphore_win32_handle_khr_fn -> PFN_vkImportSemaphoreWin32HandleKHR);
    ForwardFnPtr!(deref get_semaphore_win32_handle_khr_fn -> PFN_vkGetSemaphoreWin32HandleKHR);
});

#[implements("VK_KHR_external_memory_win32")]
pub trait DeviceExternalMemoryWin32Extension: Device {
    fn get_memory_win32_handle_khr_fn(&self) -> PFN_vkGetMemoryWin32HandleKHR;
    fn get_memory_win32_handle_properties_khr_fn(&self) -> PFN_vkGetMemoryWin32HandlePropertiesKHR;

    /// Get Properties of External Memory Win32 Handles
    /// # Safety
    /// sink must be constructed correctly
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[inline]
    unsafe fn memory_win32_handle_properties(
        &self,
        handle_type: crate::ExternalMemoryHandleTypeWin32,
        handle: windows::Win32::Foundation::HANDLE,
        sink: &mut core::mem::MaybeUninit<VkMemoryWin32HandlePropertiesKHR>,
    ) -> crate::Result<()> {
        unsafe {
            self.get_memory_win32_handle_properties_khr_fn().0(
                self.native_ptr(),
                handle_type as _,
                handle,
                sink.as_mut_ptr(),
            )
            .into_result()
            .map(drop)
        }
    }

    /// Get a Windows HANDLE for a memory object
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[inline]
    fn get_memory_win32_handle(
        &self,
        info: &crate::MemoryGetWin32HandleInfo,
    ) -> crate::Result<windows::Win32::Foundation::HANDLE> {
        let mut handle = core::mem::MaybeUninit::uninit();

        unsafe {
            self.get_memory_win32_handle_khr_fn().0(self.native_ptr(), &info.0, handle.as_mut_ptr()).into_result()?;

            Ok(handle.assume_init())
        }
    }
}
#[implements("VK_KHR_external_memory_win32")]
DerefContainerWithGuardsBracketImpl!(for DeviceExternalMemoryWin32Extension {
    ForwardFnPtr!(deref get_memory_win32_handle_khr_fn -> PFN_vkGetMemoryWin32HandleKHR);
    ForwardFnPtr!(deref get_memory_win32_handle_properties_khr_fn -> PFN_vkGetMemoryWin32HandlePropertiesKHR);
});

#[implements("VK_EXT_full_screen_exclusive")]
pub trait DeviceFullScreenExclusiveExtension: Device {
    fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkAcquireFullScreenExclusiveModeEXT;
    fn release_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkReleaseFullScreenExclusiveModeEXT;
}
#[implements("VK_EXT_full_screen_exclusive")]
DerefContainerWithGuardsBracketImpl!(for DeviceFullScreenExclusiveExtension {
    ForwardFnPtr!(deref acquire_full_screen_exclusive_mode_ext_fn -> PFN_vkAcquireFullScreenExclusiveModeEXT);
    ForwardFnPtr!(deref release_full_screen_exclusive_mode_ext_fn -> PFN_vkReleaseFullScreenExclusiveModeEXT);
});

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
    fn device_handle(&self) -> VkDevice { T::device_handle(&self) }
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
                batches.as_ptr_empty_null(),
                fence.map_or(VkFence::NULL, |x| x.0),
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

    #[implements]
    fn submit_alt<'r>(
        &mut self,
        batches: impl IntoIterator<Item = SubmissionBatch2<'r>>,
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        let batches = batches.into_iter().map(|x| x.0).collect::<Vec<_>>();

        unsafe { self.submit_raw(&batches, fence) }
    }

    #[implements]
    #[inline]
    fn submit_alt3<'r>(
        &mut self,
        batches: &'r [SubmissionBatch3<'r>],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::queue_submit(
                self.native_ptr_mut(),
                batches.len() as _,
                slice_as_ptr_empty_null(batches) as _,
                fence.map_or(VkFence::NULL, |x| x.0),
            )
            .into_result()
            .map(drop)
        }
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
        batches: &[VkSubmitInfo],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::queue_submit(
                self.native_ptr_mut(),
                batches.len() as _,
                batches.as_ptr_empty_null(),
                fence.map_or(VkFence::NULL, |x| x.0),
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
    fn submit2(&mut self, batches: &[SubmitInfo2], fence: Option<VkHandleRefMut<VkFence>>) -> crate::Result<()> {
        #[cfg(feature = "Allow1_3APIs")]
        unsafe {
            crate::vkfn::queue_submit2(
                self.native_ptr_mut(),
                batches.len() as _,
                slice_as_ptr_empty_null(batches) as _,
                fence.map_or(VkFence::NULL, |x| x.0),
            )
            .into_result()
            .map(drop)
        }
        #[cfg(not(feature = "Allow1_3APIs"))]
        todo!("cache loaded function in device object")
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
                pWaitSemaphores: wait_semaphores.as_ptr_empty_null() as _,
                swapchainCount: swapchains.len() as _,
                pSwapchains: slice_as_ptr_empty_null(swapchains) as _,
                pImageIndices: slice_as_ptr_empty_null(image_indices),
                pResults: slice_as_mut_ptr_empty_null(results),
            },
            core::marker::PhantomData,
        )
    }

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
                pWaitSemaphoreInfos: slice_as_ptr_empty_null(wait_semaphores) as _,
                commandBufferInfoCount: command_buffers.len() as _,
                pCommandBufferInfos: slice_as_ptr_empty_null(command_buffers) as _,
                signalSemaphoreInfoCount: signal_semaphores.len() as _,
                pSignalSemaphoreInfos: slice_as_ptr_empty_null(signal_semaphores) as _,
            },
            core::marker::PhantomData,
        )
    }

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
