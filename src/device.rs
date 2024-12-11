//! Vulkan Device and Queues

use cfg_if::cfg_if;
use derives::implements;
use ffi_helper::{opt_pointer, slice_as_ptr_empty_null, CStrFFIRef};

use crate::ffi_helper::ArrayFFIExtensions;
use crate::*;
#[cfg(feature = "Implements")]
use crate::{fnconv::FnTransmute, DescriptorSetCopyInfo, DescriptorSetWriteInfo, VkHandleMut, VkRawHandle};

#[implements]
#[allow(dead_code)]
type DeviceResolvedFn<F> = crate::resolver::ResolvedFnCell<F, VkDevice>;
#[implements]
impl crate::resolver::ResolverInterface for VkDevice {
    #[inline(always)]
    unsafe fn load_symbol_unconstrainted<T: crate::resolver::FromPtr>(&self, name: &core::ffi::CStr) -> T {
        T::from_ptr(core::mem::transmute(crate::vkfn::get_device_proc_addr(
            *self,
            name.as_ptr() as _,
        )))
    }

    #[inline(always)]
    unsafe fn load_function_unconstrainted<F: crate::resolver::PFN>(&self, name: &core::ffi::CStr) -> F {
        F::from_void_fn(
            crate::vkfn::get_device_proc_addr(*self, name.as_ptr() as _)
                .unwrap_or_else(|| panic!("function {name:?} not found")),
        )
    }
}

/// Opaque handle to a device object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VK_OBJECT_TYPE_DEVICE)]
pub struct DeviceObject<Instance> {
    #[handle]
    handle: VkDevice,
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
    #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2"))]
    get_buffer_memory_requirements_2_khr: DeviceResolvedFn<PFN_vkGetBufferMemoryRequirements2KHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2"))]
    get_image_memory_requirements_2_khr: DeviceResolvedFn<PFN_vkGetImageMemoryRequirements2KHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2"))]
    get_image_sparse_memory_requirements_2_khr: DeviceResolvedFn<PFN_vkGetImageSparseMemoryRequirements2KHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
    create_render_pass_2_khr: DeviceResolvedFn<PFN_vkCreateRenderPass2KHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
    cmd_begin_render_pass_2_khr: DeviceResolvedFn<PFN_vkCmdBeginRenderPass2KHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
    cmd_end_render_pass_2_khr: DeviceResolvedFn<PFN_vkCmdEndRenderPass2KHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
    cmd_next_subpass_2_khr: DeviceResolvedFn<PFN_vkCmdNextSubpass2KHR>,
    #[cfg(all(feature = "Implements", feature = "VK_KHR_synchronization2"))]
    cmd_pipeline_barrier_2_khr: DeviceResolvedFn<PFN_vkCmdPipelineBarrier2KHR>,
}
impl<Instance> DeviceObject<Instance> {
    pub const fn wrap_handle(handle: VkDevice, parent: Instance) -> Self {
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
            #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2"))]
            get_buffer_memory_requirements_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2"))]
            get_image_memory_requirements_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2"))]
            get_image_sparse_memory_requirements_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
            create_render_pass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
            cmd_begin_render_pass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
            cmd_end_render_pass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
            cmd_next_subpass_2_khr: DeviceResolvedFn::new(handle),
            #[cfg(all(feature = "Implements", feature = "VK_KHR_synchronization2"))]
            cmd_pipeline_barrier_2_khr: DeviceResolvedFn::new(handle),
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
    #[implements("VK_KHR_maintenance1")]
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

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2"))] {
            fn get_buffer_memory_requirements_2_khr_fn(&self) -> PFN_vkGetBufferMemoryRequirements2KHR {
                *self.get_buffer_memory_requirements_2_khr.resolve()
            }

            fn get_image_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageMemoryRequirements2KHR {
                *self.get_image_memory_requirements_2_khr.resolve()
            }

            fn get_image_sparse_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageSparseMemoryRequirements2KHR {
                *self.get_image_sparse_memory_requirements_2_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))] {
            fn create_render_pass_2_khr_fn(&self) -> PFN_vkCreateRenderPass2KHR {
                *self.create_render_pass_2_khr.resolve()
            }

            fn cmd_begin_render_pass_2_khr_fn(&self) -> PFN_vkCmdBeginRenderPass2KHR {
                *self.cmd_begin_render_pass_2_khr.resolve()
            }

            fn cmd_end_render_pass_2_khr_fn(&self) -> PFN_vkCmdEndRenderPass2KHR {
                *self.cmd_end_render_pass_2_khr.resolve()
            }

            fn cmd_next_subpass_2_khr_fn(&self) -> PFN_vkCmdNextSubpass2KHR {
                *self.cmd_next_subpass_2_khr.resolve()
            }
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "Implements", feature = "VK_KHR_synchronization2"))] {
            fn cmd_pipeline_barrier_2_khr_fn(&self) -> PFN_vkCmdPipelineBarrier2KHR {
                *self.cmd_pipeline_barrier_2_khr.resolve()
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
            #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2"))]
            get_buffer_memory_requirements_2_khr: unsafe {
                core::ptr::read(&self.get_buffer_memory_requirements_2_khr)
            },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2"))]
            get_image_memory_requirements_2_khr: unsafe { core::ptr::read(&self.get_image_memory_requirements_2_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_get_memory_requirements2"))]
            get_image_sparse_memory_requirements_2_khr: unsafe {
                core::ptr::read(&self.get_image_sparse_memory_requirements_2_khr)
            },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
            create_render_pass_2_khr: unsafe { core::ptr::read(&self.create_render_pass_2_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
            cmd_begin_render_pass_2_khr: unsafe { core::ptr::read(&self.cmd_begin_render_pass_2_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
            cmd_end_render_pass_2_khr: unsafe { core::ptr::read(&self.cmd_end_render_pass_2_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_create_renderpass2"))]
            cmd_next_subpass_2_khr: unsafe { core::ptr::read(&self.cmd_next_subpass_2_khr) },
            #[cfg(all(feature = "Implements", feature = "VK_KHR_synchronization2"))]
            cmd_pipeline_barrier_2_khr: unsafe { core::ptr::read(&self.cmd_pipeline_barrier_2_khr) },
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

/// Family Index, Queue Priorities
#[transparent_marked]
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

#[transparent_marked]
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
impl VkPhysicalDeviceFeatures2KHR {
    pub const fn new(old_features: VkPhysicalDeviceFeatures) -> Self {
        Self {
            sType: <Self as VulkanStructure>::TYPE,
            pNext: core::ptr::null_mut(),
            features: old_features,
        }
    }

    #[inline(always)]
    pub fn with_next(self, next: &mut (impl VulkanStructureAsRef + ?Sized)) -> Self {
        Self {
            pNext: next.as_generic_mut() as *mut _ as _,
            ..self
        }
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
impl VkPhysicalDeviceSynchronization2FeaturesKHR {
    pub const fn new(enabled: bool) -> Self {
        Self {
            sType: <Self as VulkanStructure>::TYPE,
            pNext: core::ptr::null_mut(),
            synchronization2: enabled as _,
        }
    }
}

pub trait ExtraProcedureProvider {
    #[implements]
    unsafe fn extra_procedure<F: FnTransmute>(&self, name: &str) -> Option<F>;
}

pub trait Device: VkHandle<Handle = VkDevice> + InstanceChild {
    /// Get a queue handle from a device
    #[implements]
    fn queue(self, family_index: u32, queue_index: u32) -> QueueObject<Self>
    where
        Self: Sized,
    {
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_device_queue(self.native_ptr(), family_index, queue_index, h.as_mut_ptr());
            QueueObject(h.assume_init(), self)
        }
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
        infos: &[VkGraphicsPipelineCreateInfo],
        cache: Option<VkPipelineCache>,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
        objects: &mut [VkPipeline],
    ) -> crate::Result<()> {
        crate::vkfn::create_graphics_pipelines(
            self.native_ptr(),
            cache.unwrap_or(VkPipelineCache::NULL),
            infos.len() as _,
            infos.as_ptr_empty_null(),
            opt_pointer(allocation_callbacks),
            objects.as_mut_ptr(),
        )
        .into_result()
        .map(drop)
    }

    /// Create graphics pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn new_graphics_pipelines(
        &self,
        infos: &[VkGraphicsPipelineCreateInfo],
        cache: Option<&impl crate::PipelineCache>,
    ) -> crate::Result<Vec<crate::PipelineObject<Self>>>
    where
        Self: Clone,
    {
        let mut hs = vec![VkPipeline::NULL; infos.len()];

        unsafe {
            self.new_graphics_pipelines_raw(infos, cache.map(VkHandle::native_ptr), None, &mut hs)?;
        }

        Ok(hs
            .into_iter()
            .map(|h| unsafe { crate::PipelineObject::manage(h, self.clone()) })
            .collect())
    }

    /// Create graphics pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn new_graphics_pipeline_array<const N: usize>(
        &self,
        infos: &[VkGraphicsPipelineCreateInfo; N],
        cache: Option<&impl crate::PipelineCache>,
    ) -> crate::Result<[crate::PipelineObject<Self>; N]>
    where
        Self: Clone,
    {
        let mut hs = [VkPipeline::NULL; N];

        unsafe {
            self.new_graphics_pipelines_raw(infos, cache.map(VkHandle::native_ptr), None, &mut hs)?;
        }

        Ok(core::array::from_fn(|n| unsafe {
            crate::PipelineObject::manage(hs[n], self.clone())
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
        infos: &[VkComputePipelineCreateInfo],
        cache: Option<VkPipelineCache>,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
        objects: &mut [VkPipeline],
    ) -> crate::Result<()> {
        crate::vkfn::create_compute_pipelines(
            self.native_ptr(),
            cache.unwrap_or(VkPipelineCache::NULL),
            infos.len() as _,
            infos.as_ptr_empty_null(),
            opt_pointer(allocation_callbacks),
            objects.as_mut_ptr(),
        )
        .into_result()
        .map(drop)
    }

    /// Create compute pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn new_compute_pipelines(
        &self,
        builders: &[crate::ComputePipelineBuilder<impl crate::PipelineLayout, impl crate::PipelineShaderProvider>],
        cache: Option<&impl crate::PipelineCache>,
    ) -> crate::Result<Vec<crate::PipelineObject<Self>>>
    where
        Self: Clone,
    {
        let (cinfos, _extras): (Vec<_>, Vec<_>) = builders
            .iter()
            .map(|b| {
                let extras = Box::pin(b.shader.make_extras());
                let stage = b.shader.base_struct(crate::ShaderStage::Compute, &extras);

                (
                    VkComputePipelineCreateInfo {
                        sType: VkComputePipelineCreateInfo::TYPE,
                        pNext: std::ptr::null(),
                        flags: 0,
                        basePipelineHandle: VkPipeline::NULL,
                        basePipelineIndex: -1,
                        stage: stage.0,
                        layout: b.layout.native_ptr(),
                    },
                    extras,
                )
            })
            .unzip();
        let mut pipelines = vec![VkPipeline::NULL; builders.len()];

        unsafe {
            self.new_compute_pipelines_raw(&cinfos, cache.map(VkHandle::native_ptr), None, &mut pipelines)?;
        }

        Ok(pipelines
            .into_iter()
            .map(|h| unsafe { crate::PipelineObject::manage(h, self.clone()) })
            .collect())
    }

    /// Create compute pipelines
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn new_compute_pipeline_array<const N: usize>(
        &self,
        info: &[crate::ComputePipelineBuilder<impl crate::PipelineLayout, impl crate::PipelineShaderProvider>; N],
        cache: Option<&impl crate::PipelineCache>,
    ) -> crate::Result<[crate::PipelineObject<Self>; N]>
    where
        Self: Clone,
    {
        let (cinfos, _extras): (Vec<_>, Vec<_>) = info
            .iter()
            .map(|b| {
                let extras = Box::pin(b.shader.make_extras());
                let stage = b.shader.base_struct(crate::ShaderStage::Compute, &extras);

                (
                    VkComputePipelineCreateInfo {
                        sType: VkComputePipelineCreateInfo::TYPE,
                        pNext: std::ptr::null(),
                        flags: 0,
                        basePipelineHandle: VkPipeline::NULL,
                        basePipelineIndex: -1,
                        stage: stage.0,
                        layout: b.layout.native_ptr(),
                    },
                    extras,
                )
            })
            .unzip();
        let mut pipelines = [VkPipeline::NULL; N];

        unsafe {
            self.new_compute_pipelines_raw(&cinfos, cache.map(VkHandle::native_ptr), None, &mut pipelines)?;
        }

        Ok(core::array::from_fn(|n| unsafe {
            crate::PipelineObject::manage(pipelines[n], self.clone())
        }))
    }

    /// Invalidate `MappedMemoryRange`s
    /// Invalidating the memory range allows that device writes to the memory ranges
    /// which have been made visible to the `VK_ACCESS_HOST_WRITE_BIT` and `VK_ACCESS_HOST_READ_BIT`
    /// are made visible to the host
    /// # Safety
    /// Memory object in `ranges` must be currently host mapped
    #[implements]
    unsafe fn invalidate_memory_range(&self, ranges: &[VkMappedMemoryRange]) -> crate::Result<()> {
        crate::vkfn::invalidate_mapped_memory_ranges(self.native_ptr(), ranges.len() as _, ranges.as_ptr_empty_null())
            .into_result()
            .map(drop)
    }

    /// Flush `MappedMemoryRange`s
    /// Flushing the memory range allows that host writes to the memory ranges can
    /// be made available to device access
    /// # Safety
    /// Memory object in `ranges` must be currently host mapped
    #[implements]
    unsafe fn flush_mapped_memory_ranges(&self, ranges: &[VkMappedMemoryRange]) -> crate::Result<()> {
        crate::vkfn::flush_mapped_memory_ranges(
            self.native_ptr(),
            ranges.len() as _,
            ranges.as_ptr_empty_null() as *const _,
        )
        .into_result()
        .map(drop)
    }

    /// Update the contents of descriptor set objects
    #[implements]
    fn update_descriptor_sets(&self, writes: &[DescriptorSetWriteInfo], copies: &[DescriptorSetCopyInfo]) {
        let writes = writes
            .iter()
            .map(DescriptorSetWriteInfo::make_structure)
            .collect::<Vec<_>>();
        let copies = copies
            .iter()
            .map(DescriptorSetCopyInfo::make_structure)
            .collect::<Vec<_>>();

        unsafe {
            crate::vkfn::update_descriptor_sets(
                self.native_ptr(),
                writes.len() as _,
                writes.as_ptr_empty_null(),
                copies.len() as _,
                copies.as_ptr_empty_null(),
            );
        }
    }

    /// Wait for a object to become idle
    /// # Safety
    /// All VkQueue objects created from this device must be externally synchronized.
    #[implements]
    unsafe fn wait(&self) -> crate::Result<()> {
        crate::vkfn::device_wait_idle(self.native_ptr()).into_result().map(drop)
    }

    /// Single binding for buffer
    #[implements]
    fn bind_buffer_raw(&self, buffer: VkBuffer, memory: VkDeviceMemory, offset: VkDeviceSize) -> crate::Result<()> {
        unsafe {
            crate::vkfn::bind_buffer_memory(self.native_ptr(), buffer, memory, offset)
                .into_result()
                .map(drop)
        }
    }

    /// Multiple Binding for Buffers
    #[implements("VK_KHR_bind_memory2")]
    fn bind_buffers(&self, bounds: &[VkBindBufferMemoryInfoKHR]) -> crate::Result<()> {
        tracing::trace!(target: "br-vkapi-call", "vkBindBufferMemory2KHR");

        unsafe {
            self.bind_buffer_memory2_khr_fn().0(self.native_ptr(), bounds.len() as _, bounds.as_ptr_empty_null())
                .into_result()
                .map(drop)
        }
    }

    /// Single binding for image
    #[implements]
    fn bind_image_raw(&self, image: VkImage, memory: VkDeviceMemory, offset: VkDeviceSize) -> crate::Result<()> {
        unsafe {
            crate::vkfn::bind_image_memory(self.native_ptr(), image, memory, offset)
                .into_result()
                .map(drop)
        }
    }

    /// Multiple Binding for Images
    #[implements("VK_KHR_bind_memory2")]
    fn bind_images(&self, bounds: &[VkBindImageMemoryInfoKHR]) -> crate::Result<()> {
        tracing::trace!(target: "br-vkapi-call", "vkBindImageMemory2KHR");

        unsafe {
            self.bind_image_memory2_khr_fn().0(self.native_ptr(), bounds.len() as _, bounds.as_ptr_empty_null())
                .into_result()
                .map(drop)
        }
    }

    /// Multiple Binding for both resources
    #[implements("VK_KHR_bind_memory2")]
    fn bind_resources(
        &self,
        buf_bounds: &[VkBindBufferMemoryInfoKHR],
        img_bounds: &[VkBindImageMemoryInfoKHR],
    ) -> crate::Result<()> {
        // 必ず両方実行されるようにする
        self.bind_buffers(buf_bounds).and(self.bind_images(img_bounds))
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
    fn reset_multiple_fences(&self, objects: &[VkHandleRefMut<VkFence>]) -> crate::Result<()> {
        unsafe {
            crate::vkfn::reset_fences(self.native_ptr(), objects.len() as _, objects.as_ptr_empty_null() as _)
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

        self.create_descriptor_update_template_khr_fn().0(
            self.native_ptr(),
            info,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;

        Ok(h.assume_init())
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
            pDescriptorUpdateEntries: entries.as_ptr_empty_null(),
            templateType: if dsl.is_none() {
                VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS
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
        handle: &crate::ExternalMemoryHandleFd,
        sink: &mut core::mem::MaybeUninit<VkMemoryFdPropertiesKHR>,
    ) -> crate::Result<()> {
        self.get_memory_fd_properties_khr_fn().0(self.native_ptr(), handle.0 as _, handle.1, sink.as_mut_ptr())
            .into_result()
            .map(drop)
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
    #[implements("VK_EXT_external_memory_host_pointer")]
    #[inline]
    unsafe fn memory_host_pointer_properties(
        &self,
        handle: &crate::ExternalMemoryHostPointer,
        sink: &mut core::mem::MaybeUninit<VkMemoryHostPointerPropertiesEXT>,
    ) -> crate::Result<()> {
        self.get_memory_host_pointer_properties_ext_fn().0(
            self.native_ptr(),
            handle.0 as _,
            handle.1,
            sink.as_mut_ptr(),
        )
        .into_result()
        .map(drop)
    }

    /// Get Properties of External Memory Win32 Handles
    /// # Safety
    /// sink must be constructed correctly
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements("VK_KHR_external_memory_win32")]
    #[inline]
    unsafe fn memory_win32_handle_properties(
        &self,
        handle: &crate::ExternalMemoryWin32Handle,
        sink: &mut core::mem::MaybeUninit<VkMemoryWin32HandlePropertiesKHR>,
    ) -> crate::Result<()> {
        self.get_memory_win32_handle_properties_khr_fn().0(
            self.native_ptr(),
            handle.0 as _,
            handle.1,
            sink.as_mut_ptr(),
        )
        .into_result()
        .map(drop)
    }

    /// Get a Windows HANDLE for a memory object
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements("VK_KHR_external_memory_win32")]
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

    /// Import a semaphore from a Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_INVALID_EXTERNAL_HANDLE
    #[implements("VK_KHR_external_semaphore_win32")]
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
    #[implements("VK_KHR_external_semaphore_win32")]
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

    // Extension Function Providers

    #[implements("VK_KHR_maintenance1")]
    fn get_trim_command_pool_khr_fn(&self) -> PFN_vkTrimCommandPoolKHR;

    #[implements("VK_KHR_descriptor_update_template")]
    fn create_descriptor_update_template_khr_fn(&self) -> PFN_vkCreateDescriptorUpdateTemplateKHR;
    #[implements("VK_KHR_descriptor_update_template")]
    fn destroy_descriptor_update_template_khr_fn(&self) -> PFN_vkDestroyDescriptorUpdateTemplateKHR;
    #[implements("VK_KHR_descriptor_update_template")]
    fn update_descriptor_set_with_template_khr_fn(&self) -> PFN_vkUpdateDescriptorSetWithTemplateKHR;

    #[implements("VK_KHR_bind_memory2")]
    fn bind_buffer_memory2_khr_fn(&self) -> PFN_vkBindBufferMemory2KHR;
    #[implements("VK_KHR_bind_memory2")]
    fn bind_image_memory2_khr_fn(&self) -> PFN_vkBindImageMemory2KHR;

    #[implements("VK_EXT_image_drm_format_modifier")]
    fn get_image_drm_format_modifier_properties_ext_fn(&self) -> PFN_vkGetImageDrmFormatModifierPropertiesEXT;

    #[implements("VK_KHR_external_fence_fd")]
    fn get_fence_fd_khr_fn(&self) -> PFN_vkGetFenceFdKHR;
    #[implements("VK_KHR_external_fence_fd")]
    fn import_fence_fd_khr_fn(&self) -> PFN_vkImportFenceFdKHR;

    #[implements("VK_EXT_full_screen_exclusive")]
    fn acquire_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkAcquireFullScreenExclusiveModeEXT;
    #[implements("VK_EXT_full_screen_exclusive")]
    fn release_full_screen_exclusive_mode_ext_fn(&self) -> PFN_vkReleaseFullScreenExclusiveModeEXT;

    #[implements("VK_KHR_external_memory_fd")]
    fn get_memory_fd_khr_fn(&self) -> PFN_vkGetMemoryFdKHR;
    #[implements("VK_KHR_external_memory_fd")]
    fn get_memory_fd_properties_khr_fn(&self) -> PFN_vkGetMemoryFdPropertiesKHR;

    #[implements("VK_EXT_external_memory_host")]
    fn get_memory_host_pointer_properties_ext_fn(&self) -> PFN_vkGetMemoryHostPointerPropertiesEXT;

    #[implements("VK_KHR_external_semaphore_win32")]
    fn import_semaphore_win32_handle_khr_fn(&self) -> PFN_vkImportSemaphoreWin32HandleKHR;
    #[implements("VK_KHR_external_semaphore_win32")]
    fn get_semaphore_win32_handle_khr_fn(&self) -> PFN_vkGetSemaphoreWin32HandleKHR;

    #[implements("VK_KHR_external_memory_win32")]
    fn get_memory_win32_handle_khr_fn(&self) -> PFN_vkGetMemoryWin32HandleKHR;
    #[implements("VK_KHR_external_memory_win32")]
    fn get_memory_win32_handle_properties_khr_fn(&self) -> PFN_vkGetMemoryWin32HandlePropertiesKHR;

    #[implements("VK_KHR_get_memory_requirements2")]
    fn get_buffer_memory_requirements_2_khr_fn(&self) -> PFN_vkGetBufferMemoryRequirements2KHR;
    #[implements("VK_KHR_get_memory_requirements2")]
    fn get_image_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageMemoryRequirements2KHR;
    #[implements("VK_KHR_get_memory_requirements2")]
    fn get_image_sparse_memory_requirements_2_khr_fn(&self) -> PFN_vkGetImageSparseMemoryRequirements2KHR;

    #[implements("VK_KHR_create_renderpass2")]
    fn create_render_pass_2_khr_fn(&self) -> PFN_vkCreateRenderPass2KHR;
    #[implements("VK_KHR_create_renderpass2")]
    fn cmd_begin_render_pass_2_khr_fn(&self) -> PFN_vkCmdBeginRenderPass2KHR;
    #[implements("VK_KHR_create_renderpass2")]
    fn cmd_end_render_pass_2_khr_fn(&self) -> PFN_vkCmdEndRenderPass2KHR;
    #[implements("VK_KHR_create_renderpass2")]
    fn cmd_next_subpass_2_khr_fn(&self) -> PFN_vkCmdNextSubpass2KHR;

    #[implements("VK_KHR_synchronization2")]
    fn cmd_pipeline_barrier_2_khr_fn(&self) -> PFN_vkCmdPipelineBarrier2KHR;
}
DerefContainerBracketImpl!(for Device {
    #[implements("VK_KHR_maintenance1")]
    ForwardFnPtr!(deref get_trim_command_pool_khr_fn -> PFN_vkTrimCommandPoolKHR);

    #[implements("VK_KHR_descriptor_update_template")]
    ForwardFnPtr!(deref create_descriptor_update_template_khr_fn -> PFN_vkCreateDescriptorUpdateTemplateKHR);
    #[implements("VK_KHR_descriptor_update_template")]
    ForwardFnPtr!(deref destroy_descriptor_update_template_khr_fn -> PFN_vkDestroyDescriptorUpdateTemplateKHR);
    #[implements("VK_KHR_descriptor_update_template")]
    ForwardFnPtr!(deref update_descriptor_set_with_template_khr_fn -> PFN_vkUpdateDescriptorSetWithTemplateKHR);

    #[implements("VK_KHR_bind_memory2")]
    ForwardFnPtr!(deref bind_buffer_memory2_khr_fn -> PFN_vkBindBufferMemory2KHR);
    #[implements("VK_KHR_bind_memory2")]
    ForwardFnPtr!(deref bind_image_memory2_khr_fn -> PFN_vkBindImageMemory2KHR);

    #[implements("VK_EXT_image_drm_format_modifier")]
    ForwardFnPtr!(deref get_image_drm_format_modifier_properties_ext_fn -> PFN_vkGetImageDrmFormatModifierPropertiesEXT);

    #[implements("VK_KHR_external_fence_fd")]
    ForwardFnPtr!(deref get_fence_fd_khr_fn -> PFN_vkGetFenceFdKHR);
    #[implements("VK_KHR_external_fence_fd")]
    ForwardFnPtr!(deref import_fence_fd_khr_fn -> PFN_vkImportFenceFdKHR);

    #[implements("VK_EXT_full_screen_exclusive")]
    ForwardFnPtr!(deref acquire_full_screen_exclusive_mode_ext_fn -> PFN_vkAcquireFullScreenExclusiveModeEXT);
    #[implements("VK_EXT_full_screen_exclusive")]
    ForwardFnPtr!(deref release_full_screen_exclusive_mode_ext_fn -> PFN_vkReleaseFullScreenExclusiveModeEXT);

    #[implements("VK_KHR_external_memory_fd")]
    ForwardFnPtr!(deref get_memory_fd_khr_fn -> PFN_vkGetMemoryFdKHR);
    #[implements("VK_KHR_external_memory_fd")]
    ForwardFnPtr!(deref get_memory_fd_properties_khr_fn -> PFN_vkGetMemoryFdPropertiesKHR);

    #[implements("VK_EXT_external_memory_host")]
    ForwardFnPtr!(deref get_memory_host_pointer_properties_ext_fn -> PFN_vkGetMemoryHostPointerPropertiesEXT);

    #[implements("VK_KHR_external_semaphore_win32")]
    ForwardFnPtr!(deref import_semaphore_win32_handle_khr_fn -> PFN_vkImportSemaphoreWin32HandleKHR);
    #[implements("VK_KHR_external_semaphore_win32")]
    ForwardFnPtr!(deref get_semaphore_win32_handle_khr_fn -> PFN_vkGetSemaphoreWin32HandleKHR);

    #[implements("VK_KHR_external_memory_win32")]
    ForwardFnPtr!(deref get_memory_win32_handle_khr_fn -> PFN_vkGetMemoryWin32HandleKHR);
    #[implements("VK_KHR_external_memory_win32")]
    ForwardFnPtr!(deref get_memory_win32_handle_properties_khr_fn -> PFN_vkGetMemoryWin32HandlePropertiesKHR);

    #[implements("VK_KHR_get_memory_requirements2")]
    ForwardFnPtr!(deref get_buffer_memory_requirements_2_khr_fn -> PFN_vkGetBufferMemoryRequirements2KHR);
    #[implements("VK_KHR_get_memory_requirements2")]
    ForwardFnPtr!(deref get_image_memory_requirements_2_khr_fn -> PFN_vkGetImageMemoryRequirements2KHR);
    #[implements("VK_KHR_get_memory_requirements2")]
    ForwardFnPtr!(deref get_image_sparse_memory_requirements_2_khr_fn -> PFN_vkGetImageSparseMemoryRequirements2KHR);

    #[implements("VK_KHR_create_renderpass2")]
    ForwardFnPtr!(deref create_render_pass_2_khr_fn -> PFN_vkCreateRenderPass2KHR);
    #[implements("VK_KHR_create_renderpass2")]
    ForwardFnPtr!(deref cmd_begin_render_pass_2_khr_fn -> PFN_vkCmdBeginRenderPass2KHR);
    #[implements("VK_KHR_create_renderpass2")]
    ForwardFnPtr!(deref cmd_end_render_pass_2_khr_fn -> PFN_vkCmdEndRenderPass2KHR);
    #[implements("VK_KHR_create_renderpass2")]
    ForwardFnPtr!(deref cmd_next_subpass_2_khr_fn -> PFN_vkCmdNextSubpass2KHR);

    #[implements("VK_KHR_synchronization2")]
    ForwardFnPtr!(deref cmd_pipeline_barrier_2_khr_fn -> PFN_vkCmdPipelineBarrier2KHR);
});
GuardsImpl!(for Device {
    #[implements("VK_KHR_maintenance1")]
    ForwardFnPtr!(deref get_trim_command_pool_khr_fn -> PFN_vkTrimCommandPoolKHR);

    #[implements("VK_KHR_descriptor_update_template")]
    ForwardFnPtr!(deref create_descriptor_update_template_khr_fn -> PFN_vkCreateDescriptorUpdateTemplateKHR);
    #[implements("VK_KHR_descriptor_update_template")]
    ForwardFnPtr!(deref destroy_descriptor_update_template_khr_fn -> PFN_vkDestroyDescriptorUpdateTemplateKHR);
    #[implements("VK_KHR_descriptor_update_template")]
    ForwardFnPtr!(deref update_descriptor_set_with_template_khr_fn -> PFN_vkUpdateDescriptorSetWithTemplateKHR);

    #[implements("VK_KHR_bind_memory2")]
    ForwardFnPtr!(deref bind_buffer_memory2_khr_fn -> PFN_vkBindBufferMemory2KHR);
    #[implements("VK_KHR_bind_memory2")]
    ForwardFnPtr!(deref bind_image_memory2_khr_fn -> PFN_vkBindImageMemory2KHR);

    #[implements("VK_EXT_image_drm_format_modifier")]
    ForwardFnPtr!(deref get_image_drm_format_modifier_properties_ext_fn -> PFN_vkGetImageDrmFormatModifierPropertiesEXT);

    #[implements("VK_KHR_external_fence_fd")]
    ForwardFnPtr!(deref get_fence_fd_khr_fn -> PFN_vkGetFenceFdKHR);
    #[implements("VK_KHR_external_fence_fd")]
    ForwardFnPtr!(deref import_fence_fd_khr_fn -> PFN_vkImportFenceFdKHR);

    #[implements("VK_EXT_full_screen_exclusive")]
    ForwardFnPtr!(deref acquire_full_screen_exclusive_mode_ext_fn -> PFN_vkAcquireFullScreenExclusiveModeEXT);
    #[implements("VK_EXT_full_screen_exclusive")]
    ForwardFnPtr!(deref release_full_screen_exclusive_mode_ext_fn -> PFN_vkReleaseFullScreenExclusiveModeEXT);

    #[implements("VK_KHR_external_memory_fd")]
    ForwardFnPtr!(deref get_memory_fd_khr_fn -> PFN_vkGetMemoryFdKHR);
    #[implements("VK_KHR_external_memory_fd")]
    ForwardFnPtr!(deref get_memory_fd_properties_khr_fn -> PFN_vkGetMemoryFdPropertiesKHR);

    #[implements("VK_EXT_external_memory_host")]
    ForwardFnPtr!(deref get_memory_host_pointer_properties_ext_fn -> PFN_vkGetMemoryHostPointerPropertiesEXT);

    #[implements("VK_KHR_external_semaphore_win32")]
    ForwardFnPtr!(deref import_semaphore_win32_handle_khr_fn -> PFN_vkImportSemaphoreWin32HandleKHR);
    #[implements("VK_KHR_external_semaphore_win32")]
    ForwardFnPtr!(deref get_semaphore_win32_handle_khr_fn -> PFN_vkGetSemaphoreWin32HandleKHR);

    #[implements("VK_KHR_external_memory_win32")]
    ForwardFnPtr!(deref get_memory_win32_handle_khr_fn -> PFN_vkGetMemoryWin32HandleKHR);
    #[implements("VK_KHR_external_memory_win32")]
    ForwardFnPtr!(deref get_memory_win32_handle_properties_khr_fn -> PFN_vkGetMemoryWin32HandlePropertiesKHR);

    #[implements("VK_KHR_get_memory_requirements2")]
    ForwardFnPtr!(deref get_buffer_memory_requirements_2_khr_fn -> PFN_vkGetBufferMemoryRequirements2KHR);
    #[implements("VK_KHR_get_memory_requirements2")]
    ForwardFnPtr!(deref get_image_memory_requirements_2_khr_fn -> PFN_vkGetImageMemoryRequirements2KHR);
    #[implements("VK_KHR_get_memory_requirements2")]
    ForwardFnPtr!(deref get_image_sparse_memory_requirements_2_khr_fn -> PFN_vkGetImageSparseMemoryRequirements2KHR);

    #[implements("VK_KHR_create_renderpass2")]
    ForwardFnPtr!(deref create_render_pass_2_khr_fn -> PFN_vkCreateRenderPass2KHR);
    #[implements("VK_KHR_create_renderpass2")]
    ForwardFnPtr!(deref cmd_begin_render_pass_2_khr_fn -> PFN_vkCmdBeginRenderPass2KHR);
    #[implements("VK_KHR_create_renderpass2")]
    ForwardFnPtr!(deref cmd_end_render_pass_2_khr_fn -> PFN_vkCmdEndRenderPass2KHR);
    #[implements("VK_KHR_create_renderpass2")]
    ForwardFnPtr!(deref cmd_next_subpass_2_khr_fn -> PFN_vkCmdNextSubpass2KHR);

    #[implements("VK_KHR_synchronization2")]
    ForwardFnPtr!(deref cmd_pipeline_barrier_2_khr_fn -> PFN_vkCmdPipelineBarrier2KHR);
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
    #[implements]
    fn bind_sparse(
        &mut self,
        batches: &[impl SparseBindingOpBatch],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        let batches: Vec<_> = batches.iter().map(SparseBindingOpBatch::make_info_struct).collect();

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
        crate::vkfn::queue_bind_sparse(
            self.native_ptr_mut(),
            batches.len() as _,
            batches.as_ptr_empty_null(),
            fence.map_or(VkFence::NULL, |x| x.0),
        )
        .into_result()
        .map(drop)
    }

    /// Submits a sequence of semaphores or command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[implements]
    fn submit(
        &mut self,
        batches: &[impl SubmissionBatch],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
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
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    unsafe fn submit_raw(
        &mut self,
        batches: &[VkSubmitInfo],
        fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        crate::vkfn::queue_submit(
            self.native_ptr_mut(),
            batches.len() as _,
            batches.as_ptr_empty_null(),
            fence.map_or(VkFence::NULL, |x| x.0),
        )
        .into_result()
        .map(drop)
    }

    /// Submits command buffers to a queue
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
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
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    /// * `VK_ERROR_OUT_OF_DATE_KHR`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_KHR_swapchain")]
    fn present<'r>(&mut self, info: PresentInfo<'r>) -> crate::Result<Vec<crate::Result<()>>> {
        info.submit(self)
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
    #[deprecated = "use PresentInfo for more extensibiility"]
    fn present1(
        &mut self,
        swapchains: &mut [(&mut (impl crate::Swapchain + VkHandleMut), u32)],
        wait_semaphores: &mut [impl VkHandleMut<Handle = VkSemaphore>],
    ) -> crate::Result<Vec<VkResult>> {
        let mut res = vec![VkResult(0); swapchains.len()];
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
            pWaitSemaphores: wait_semaphores.as_ptr_empty_null(),
            swapchainCount: swapchains.len() as _,
            pSwapchains: swapchains.as_ptr_empty_null(),
            pImageIndices: indices.as_ptr_empty_null(),
            pResults: res.as_mut_ptr(),
        };
        unsafe {
            crate::vkfn::queue_present_khr(self.native_ptr_mut(), &pinfo)
                .into_result()
                .map(|_| res)
        }
    }
}
DerefContainerBracketImpl!(for mut QueueMut {});
GuardsImpl!(for mut QueueMut {});

#[cfg(feature = "VK_KHR_swapchain")]
#[transparent_marked]
pub struct PresentInfo<'r>(
    VkPresentInfoKHR,
    core::marker::PhantomData<(&'r [VkSwapchainKHR], &'r [VkSemaphore], &'r [u32])>,
);
#[cfg(feature = "VK_KHR_swapchain")]
impl<'r> PresentInfo<'r> {
    #[inline(always)]
    pub fn new(
        wait_semaphores: &'r [impl crate::Transparent<Target = VkSemaphore>],
        swapchains: &'r [impl crate::Transparent<Target = VkSwapchainKHR>],
        image_indices: &'r [u32],
    ) -> Self {
        assert_eq!(swapchains.len(), image_indices.len());

        Self(
            VkPresentInfoKHR {
                sType: VkPresentInfoKHR::TYPE,
                pNext: core::ptr::null(),
                waitSemaphoreCount: wait_semaphores.len() as _,
                pWaitSemaphores: wait_semaphores.as_ptr_empty_null() as _,
                swapchainCount: swapchains.len() as _,
                pSwapchains: swapchains.as_ptr_empty_null() as _,
                pImageIndices: image_indices.as_ptr_empty_null(),
                pResults: core::ptr::null_mut(),
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub const unsafe fn from_raw(raw: VkPresentInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    #[implements]
    pub fn submit(mut self, queue: &mut (impl QueueMut + ?Sized)) -> crate::Result<Vec<crate::Result<()>>> {
        let mut results = vec![VK_SUCCESS; self.0.swapchainCount as usize];
        self.0.pResults = results.as_mut_ptr_empty_null();

        unsafe {
            crate::vkfn::queue_present_khr(queue.native_ptr_mut(), &self.0)
                .into_result()
                .map(|_| results.into_iter().map(|r| r.into_result().map(drop)).collect())
        }
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

    #[inline(always)]
    pub const unsafe fn from_raw(raw: VkCommandBufferSubmitInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn on_device(mut self, mask: u32) -> Self {
        self.0.deviceMask = mask;
        self
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[transparent_marked]
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

    pub const fn protected(mut self) -> Self {
        self.0.flags |= VK_SUBMIT_PROTECTED_BIT_KHR;
        self
    }
}
