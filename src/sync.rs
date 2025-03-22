//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

use crate::{
    DeviceChild, VkHandle, VkHandleMut, VkObject, VulkanStructure, VulkanStructureAsRef, ffi_helper::opt_pointer, vk::*,
};
use derives::implements;

/// Result Status for time-limited wait operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitResult {
    Success,
    Timeout,
}

/// Opaque handle to a fence object
#[repr(transparent)]
pub struct Fence(VkFence_T);
#[implements]
impl Fence {
    /// Destroy an fence object.
    /// # Safety
    /// * the `device` parameter must be owner of the event object.
    /// * After calling this function, accessing to the object causes undefined behavior.
    #[inline]
    pub unsafe fn destroy(
        &mut self,
        device: &(impl crate::DeviceT + ?Sized),
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { crate::vkfn::destroy_fence(device.native_ptr(), self as *mut _ as _, opt_pointer(allocator)) }
    }

    /// Wait for a fence to become signaled, returns `Ok(true)` if operation is timed out
    /// # Safety
    /// the `device` parameter must be owner of the event object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    pub unsafe fn wait_timeout(
        &self,
        device: &(impl crate::DeviceT + ?Sized),
        timeout: u64,
    ) -> crate::Result<WaitResult> {
        let r =
            unsafe { crate::vkfn::wait_for_fences(device.native_ptr(), 1, self as *const _ as _, false as _, timeout) };
        if r == VK_SUCCESS {
            return Ok(WaitResult::Success);
        }
        if r == VK_TIMEOUT {
            return Ok(WaitResult::Timeout);
        }

        Err(r)
    }

    /// Wait for a fence to become signaled
    /// # Safety
    /// the `device` parameter must be owner of the event object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[inline]
    pub unsafe fn wait(&self, device: &(impl crate::DeviceT + ?Sized)) -> crate::Result<()> {
        // timeoutはしない
        unsafe { self.wait_timeout(device, std::u64::MAX).map(drop) }
    }

    /// Resets a fence object
    /// # Safety
    /// the `device` parameter must be owner of the event object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline]
    pub unsafe fn reset(&mut self, device: &(impl crate::DeviceT + ?Sized)) -> crate::Result<()> {
        unsafe {
            crate::vkfn::reset_fences(device.native_ptr(), 1, self as *mut _ as _)
                .into_result()
                .map(drop)
        }
    }
}
impl Status for Fence {
    #[implements]
    #[inline]
    unsafe fn status(&self, device: &(impl crate::DeviceT + ?Sized)) -> crate::Result<SyncStatus> {
        let r = unsafe { crate::vkfn::get_fence_status(device.native_ptr(), self as *const _ as _) };
        if r == VK_SUCCESS {
            return Ok(SyncStatus::Done);
        }
        if r == VK_NOT_READY {
            return Ok(SyncStatus::Pending);
        }

        Err(r)
    }
}

/// Opaque handle to a semaphore object.
#[repr(transparent)]
pub struct Semaphore(VkSemaphore_T);
#[implements]
impl Semaphore {
    /// Destroy an semaphore object.
    /// # Safety
    /// * the `device` parameter must be owner of the event object.
    /// * After calling this function, accessing to the object causes undefined behavior.
    #[inline]
    pub unsafe fn destroy(
        &mut self,
        device: &(impl crate::DeviceT + ?Sized),
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { crate::vkfn::destroy_semaphore(device.native_ptr(), self as *mut _ as _, opt_pointer(allocator)) }
    }

    /// Creates a submit info structure for this semaphore.
    #[cfg(feature = "VK_KHR_synchronization2")]
    #[inline(always)]
    fn submit_info(&self) -> SemaphoreSubmitInfo {
        SemaphoreSubmitInfo::new(self)
    }
}

/// Opaque handle to an event object.
#[repr(transparent)]
pub struct Event(VkEvent_T);
#[implements]
impl Event {
    /// Destroy an event object.
    /// # Safety
    /// * the `device` parameter must be owner of the event object.
    /// * After calling this function, accessing to the object causes undefined behavior.
    #[inline]
    pub unsafe fn destroy(
        &mut self,
        device: &(impl crate::DeviceT + ?Sized),
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { crate::vkfn::destroy_event(device.native_ptr(), self as *mut _ as _, opt_pointer(allocator)) }
    }

    /// Set an event to signaled state.
    /// # Safety
    /// the `device` parameter must be owner of the event object.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline]
    pub unsafe fn set(&mut self, device: &(impl crate::DeviceT + ?Sized)) -> crate::Result<()> {
        unsafe {
            crate::vkfn::set_event(device.native_ptr(), self as *mut _ as _)
                .into_result()
                .map(drop)
        }
    }

    /// Reset an event to non-signaled state.
    /// # Safety
    /// the `device` parameter must be owner of the event object.
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline]
    pub unsafe fn reset(&mut self, device: &(impl crate::DeviceT + ?Sized)) -> crate::Result<()> {
        unsafe {
            crate::vkfn::reset_event(self.device().native_ptr(), self.native_ptr_mut())
                .into_result()
                .map(drop)
        }
    }
}
impl Status for Event {
    #[implements]
    #[inline]
    unsafe fn status(&self, device: &(impl crate::DeviceT + ?Sized)) -> crate::Result<SyncStatus> {
        let r = unsafe { crate::vkfn::get_event_status(device.native_ptr(), self as *const _ as _) };
        if r == VK_EVENT_SET {
            return Ok(SyncStatus::Done);
        }
        if r == VK_EVENT_RESET {
            return Ok(SyncStatus::Pending);
        }

        Err(r)
    }
}

/// Status value of the synchronizing object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncStatus {
    Done,
    Pending,
}

pub trait Status {
    /// Retrieve the status(whether is signaled or not) of a synchronize object
    /// # Safety
    /// the `device` parameter must be owner of this object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[implements]
    unsafe fn status(&self, device: &(impl crate::DeviceT + ?Sized)) -> crate::Result<SyncStatus>;
}
DerefContainerBracketImpl!(for Status {
    #[implements]
    #[inline(always)]
    unsafe fn status(&self, device: &(impl crate::DeviceT + ?Sized)) -> crate::Result<SyncStatus> {
        unsafe { T::status(self, device) }
    }
});
GuardsImpl!(for Status {
    #[implements]
    #[inline(always)]
    unsafe fn status(&self, device: &(impl crate::DeviceT + ?Sized)) -> crate::Result<SyncStatus> {
        unsafe { T::status(&self, device) }
    }
});

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenceCreateInfo<'d>(
    VkFenceCreateInfo,
    core::marker::PhantomData<Option<&'d dyn VulkanStructureAsRef>>,
);
impl<'d> FenceCreateInfo<'d> {
    pub const fn new(flags: VkFenceCreateFlags) -> Self {
        Self(
            VkFenceCreateInfo {
                sType: VkFenceCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkFenceCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkFenceCreateInfo {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemaphoreCreateInfo<'d>(
    VkSemaphoreCreateInfo,
    core::marker::PhantomData<Option<&'d dyn VulkanStructureAsRef>>,
);
impl<'d> SemaphoreCreateInfo<'d> {
    pub const fn new() -> Self {
        Self(
            VkSemaphoreCreateInfo {
                sType: VkSemaphoreCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkSemaphoreCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkSemaphoreCreateInfo {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventCreateInfo(VkEventCreateInfo);
impl EventCreateInfo {
    pub const fn new() -> Self {
        Self(VkEventCreateInfo {
            sType: VkEventCreateInfo::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
        })
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
pub struct SemaphoreSubmitInfo<'s>(VkSemaphoreSubmitInfoKHR, core::marker::PhantomData<&'s Semaphore>);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'s> SemaphoreSubmitInfo<'s> {
    #[inline]
    pub fn new(semaphore: &'s Semaphore) -> Self {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derives::bitflags_newtype]
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
