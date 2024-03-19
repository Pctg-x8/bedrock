//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

use crate::chain;
use crate::vk::*;
use crate::DeviceChild;
use crate::VkHandle;
#[implements]
use crate::{Device, VkHandleMut};
use crate::{GenericVulkanStructure, VulkanStructure};
use derives::bitflags_newtype;
use derives::implements;

pub trait Fence: VkHandle<Handle = VkFence> + DeviceChild + Status {
    /// Wait for a fence to become signaled, returns `Ok(true)` if operation is timed out
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[implements]
    #[inline(always)]
    fn wait_timeout(&self, timeout: u64) -> crate::Result<bool> {
        let vr = unsafe {
            crate::vkresolve::wait_for_fences(self.device().native_ptr(), 1, &self.native_ptr(), false as _, timeout)
        };
        match vr {
            VK_SUCCESS => Ok(false),
            VK_TIMEOUT => Ok(true),
            _ => Err(vr),
        }
    }

    /// Wait for a fence to become signaled
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[implements]
    #[inline(always)]
    fn wait(&self) -> crate::Result<()> {
        self.wait_timeout(std::u64::MAX).map(drop)
    }

    /// Resets a fence object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline(always)]
    fn reset(&mut self) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkresolve::reset_fences(self.device().native_ptr(), 1, &self.native_ptr_mut())
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
    #[cfg(unix)]
    fn get_external_handle(&self, ty: crate::ExternalFenceFdType) -> crate::Result<std::os::unix::io::RawFd> {
        let info = VkFenceGetFdInfoKHR {
            sType: VkFenceGetFdInfoKHR::TYPE,
            pNext: std::ptr::null(),
            fence: self.native_ptr(),
            handleType: ty as _,
        };

        let mut fd = 0;
        unsafe {
            self.device().get_fence_fd_khr_fn().0(self.device().native_ptr(), &info, &mut fd)
                .into_result()
                .map(move |_| fd)
        }
    }

    /// Import a fence from a POSIX file descriptor
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements("VK_KHR_external_fence_fd")]
    #[cfg(unix)]
    fn import(&self, handle: crate::ExternalFenceFd, temporary: bool) -> crate::Result<()> {
        let info = VkImportFenceFdInfoKHR {
            sType: VkImportFenceFdInfoKHR::TYPE,
            pNext: std::ptr::null(),
            fence: self.native_ptr(),
            flags: if temporary {
                VK_FENCE_IMPORT_TEMPORARY_BIT_KHR
            } else {
                0
            },
            handleType: handle.0 as _,
            fd: handle.1,
        };

        unsafe {
            self.device().import_fence_fd_khr_fn().0(self.device().native_ptr(), &info)
                .into_result()
                .map(drop)
        }
    }
}
DerefContainerBracketImpl!(for Fence {});
GuardsImpl!(for Fence {});

pub trait Semaphore: VkHandle<Handle = VkSemaphore> + DeviceChild {
    /// Get a Windows HANDLE for a semaphore
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_TOO_MANY_OBJECTS
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    #[implements("VK_KHR_external_semaphore_win32")]
    fn request_external_handle(
        &self,
        handle_type: crate::ExternalSemaphoreHandleTypeWin32,
    ) -> crate::Result<windows::Win32::Foundation::HANDLE> {
        let info = VkSemaphoreGetWin32HandleInfoKHR {
            sType: VkSemaphoreGetWin32HandleInfoKHR::TYPE,
            pNext: core::ptr::null(),
            semaphore: self.native_ptr(),
            handleType: handle_type as _,
        };

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            self.device().get_semaphore_win32_handle_khr_fn().0(self.device().native_ptr(), &info, h.as_mut_ptr())
                .into_result()
                .map(move |_| h.assume_init())
        }
    }

    /// Import a semaphore from a Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_INVALID_EXTERNAL_HANDLE
    #[implements("VK_KHR_external_semaphore_win32")]
    fn import(&self, handle: crate::ExternalSemaphoreHandleWin32, name: &widestring::WideCString) -> crate::Result<()> {
        let info = VkImportSemaphoreWin32HandleInfoKHR {
            sType: VkImportSemaphoreWin32HandleInfoKHR::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            semaphore: self.native_ptr(),
            handleType: handle.0 as _,
            handle: handle.1,
            name: windows::core::PCWSTR(name.as_ptr()),
        };

        unsafe {
            self.device().import_semaphore_win32_handle_khr_fn().0(self.device().native_ptr(), &info)
                .into_result()
                .map(drop)
        }
    }

    /// Creates a submit info structure for this semaphore.
    #[cfg(feature = "VK_KHR_synchronization2")]
    #[inline(always)]
    fn submit_info(&self) -> SemaphoreSubmitInfo {
        SemaphoreSubmitInfo::new(self)
    }
}
DerefContainerBracketImpl!(for Semaphore {});
GuardsImpl!(for Semaphore {});

pub trait Event: VkHandle<Handle = VkEvent> + DeviceChild + Status {
    /// Set an event to signaled state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn set(&mut self) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkresolve::set_event(self.device().native_ptr(), self.native_ptr_mut())
                .into_result()
                .map(drop)
        }
    }

    /// Reset an event to non-signaled state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn reset(&mut self) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkresolve::reset_event(self.device().native_ptr(), self.native_ptr_mut())
                .into_result()
                .map(drop)
        }
    }
}
DerefContainerBracketImpl!(for Event {});
GuardsImpl!(for Event {});

pub trait Status {
    /// Retrieve the status(whether is signaled or not) of a synchronize object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool>;
}
DerefContainerBracketImpl!(for Status {
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        T::status(self)
    }
});
GuardsImpl!(for Status {
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        T::status(&self)
    }
});

DefineStdDeviceChildObject! {
    /// Opaque Handle to a fence object
    FenceObject(VkFence): Fence
}
impl<Device: crate::Device> Status for FenceObject<Device> {
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        let vr = unsafe { crate::vkresolve::get_fence_status(self.device().native_ptr(), self.native_ptr()) };
        match vr {
            VK_SUCCESS => Ok(true),
            VK_NOT_READY => Ok(false),
            _ => Err(vr),
        }
    }
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a semaphore object
    SemaphoreObject(VkSemaphore): Semaphore
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a event object
    EventObject(VkEvent): Event
}
impl<Device: crate::Device> Status for EventObject<Device> {
    #[implements]
    fn status(&self) -> crate::Result<bool> {
        let vr = unsafe { crate::vkresolve::get_event_status(self.device().native_ptr(), self.native_ptr()) };
        match vr {
            VK_EVENT_SET => Ok(true),
            VK_EVENT_RESET => Ok(false),
            _ => Err(vr),
        }
    }
}

pub struct FenceBuilder(VkFenceCreateInfo, Vec<Box<GenericVulkanStructure>>);
impl FenceBuilder {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(
            VkFenceCreateInfo {
                sType: VkFenceCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
            },
            Vec::new(),
        )
    }

    #[inline(always)]
    pub unsafe fn with_additional_info(mut self, ext: impl VulkanStructure) -> Self {
        self.1.push(core::mem::transmute(Box::new(ext)));

        self
    }

    #[inline(always)]
    pub const fn signaled(mut self) -> Self {
        self.0.flags |= VK_FENCE_CREATE_SIGNALED_BIT;

        self
    }

    #[cfg(feature = "VK_KHR_external_fence")]
    #[inline(always)]
    pub fn exportable_as(self, ty: crate::ExternalFenceHandleTypes) -> Self {
        unsafe {
            self.with_additional_info(VkExportFenceCreateInfoKHR {
                sType: VkExportFenceCreateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                handleTypes: ty.0,
            })
        }
    }

    /// Create a new fence object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    pub fn create<Device: crate::Device>(mut self, device: Device) -> crate::Result<FenceObject<Device>> {
        crate::ext::chain(&mut self.0, self.1.iter_mut().map(|x| &mut **x));

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_fence(device.native_ptr(), &self.0, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| FenceObject(h.assume_init(), device))
        }
    }
}

pub struct SemaphoreBuilder(VkSemaphoreCreateInfo, Vec<Box<GenericVulkanStructure>>);
impl SemaphoreBuilder {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(
            VkSemaphoreCreateInfo {
                sType: VkSemaphoreCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
            },
            Vec::new(),
        )
    }

    #[inline(always)]
    pub unsafe fn with_additional_info(mut self, ext: impl VulkanStructure) -> Self {
        self.1.push(core::mem::transmute(Box::new(ext)));

        self
    }

    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[inline]
    pub fn exportable_as(
        self,
        handle_types: crate::ExternalSemaphoreHandleTypes,
        export_info: crate::ExportSemaphoreWin32HandleInfo,
    ) -> Self {
        unsafe {
            self.with_additional_info(VkExportSemaphoreCreateInfoKHR {
                sType: VkExportSemaphoreCreateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                handleTypes: handle_types.into(),
            })
            .with_additional_info(export_info)
        }
    }

    /// Create a new queue semaphore object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    pub fn create<Device: crate::Device>(mut self, device: Device) -> crate::Result<SemaphoreObject<Device>> {
        chain(&mut self.0, self.1.iter_mut().map(|x| &mut **x));

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_semaphore(device.native_ptr(), &self.0, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| SemaphoreObject(h.assume_init(), device))
        }
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
pub struct SemaphoreSubmitInfo<'s>(
    VkSemaphoreSubmitInfoKHR,
    core::marker::PhantomData<&'s dyn VkHandle<Handle = VkSemaphore>>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'s> SemaphoreSubmitInfo<'s> {
    #[inline]
    pub fn new(semaphore: &'s (impl VkHandle<Handle = VkSemaphore> + ?Sized)) -> Self {
        Self(
            VkSemaphoreSubmitInfoKHR {
                sType: VkSemaphoreSubmitInfoKHR::TYPE,
                pNext: core::ptr::null(),
                semaphore: semaphore.native_ptr(),
                value: 0,
                stageMask: VK_PIPELINE_STAGE_2_NONE_KHR,
                deviceIndex: 0,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub const fn with_value(mut self, value: u64) -> Self {
        self.0.value = value;
        self
    }

    #[inline(always)]
    pub const fn with_on_stage(mut self, stage_mask: VkPipelineStageFlags2KHR) -> Self {
        self.0.stageMask |= stage_mask;
        self
    }

    #[inline(always)]
    pub const fn on_top_of_pipe(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_draw_indirect(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_vertex_input(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_vertex_shader(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_tessellation_control_shader(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_tessellation_evaluation_shader(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_geometry_shader(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_fragment_shader(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_early_fragment_tests(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_late_fragment_tests(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_color_attachment_output(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_compute_shader(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_any_transfer(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_bottom_of_pipe(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_host(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_HOST_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_any_graphics(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_any_command(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_copy(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_COPY_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_resolve(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_blit(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_BLIT_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_clear(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_index_input(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_vertex_attribute_input(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR)
    }

    #[inline(always)]
    pub const fn on_pre_rasterization_shaders(self) -> Self {
        self.with_on_stage(VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR)
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[bitflags_newtype]
pub struct AccessFlags2(pub VkAccessFlags2KHR);
#[cfg(feature = "VK_KHR_synchronization2")]
impl AccessFlags2 {
    pub const NONE: Self = Self(VK_ACCESS_2_NONE_KHR);

    pub const INDIRECT_COMMAND_READ: Self = Self(VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR);
    pub const INDEX_READ: Self = Self(VK_ACCESS_2_INDEX_READ_BIT_KHR);
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR);
    pub const UNIFORM_READ: Self = Self(VK_ACCESS_2_UNIFORM_READ_BIT_KHR);
    pub const INPUT_ATTACHMENT_READ: Self = Self(VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR);
    pub const SHADER: AccessFlags2ReadWriteBits = AccessFlags2ReadWriteBits {
        read: Self(VK_ACCESS_2_SHADER_READ_BIT_KHR),
        write: Self(VK_ACCESS_2_SHADER_WRITE_BIT_KHR),
    };
    pub const COLOR_ATTACHMENT: AccessFlags2ReadWriteBits = AccessFlags2ReadWriteBits {
        read: Self(VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR),
        write: Self(VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR),
    };
    pub const DEPTH_STENCIL_ATTACHMENT: AccessFlags2ReadWriteBits = AccessFlags2ReadWriteBits {
        read: Self(VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR),
        write: Self(VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR),
    };
    pub const TRANSFER: AccessFlags2ReadWriteBits = AccessFlags2ReadWriteBits {
        read: Self(VK_ACCESS_2_TRANSFER_READ_BIT_KHR),
        write: Self(VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR),
    };
    pub const HOST: AccessFlags2ReadWriteBits = AccessFlags2ReadWriteBits {
        read: Self(VK_ACCESS_2_HOST_READ_BIT_KHR),
        write: Self(VK_ACCESS_2_HOST_WRITE_BIT_KHR),
    };
    pub const MEMORY: AccessFlags2ReadWriteBits = AccessFlags2ReadWriteBits {
        read: Self(VK_ACCESS_2_MEMORY_READ_BIT_KHR),
        write: Self(VK_ACCESS_2_MEMORY_WRITE_BIT_KHR),
    };
    pub const SHADER_SAMPLED_READ: Self = Self(VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR);
    pub const SHADER_STORAGE: AccessFlags2ReadWriteBits = AccessFlags2ReadWriteBits {
        read: Self(VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR),
        write: Self(VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR),
    };
}
#[cfg(feature = "VK_KHR_synchronization2")]
impl Default for AccessFlags2 {
    fn default() -> Self {
        Self::NONE
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
pub struct AccessFlags2ReadWriteBits {
    pub read: AccessFlags2,
    pub write: AccessFlags2,
}
