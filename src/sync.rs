//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

use crate::{
    vk::*, DeviceChild, DeviceChildHandle, VkDeviceChildNonExtDestroyable, VkHandle, VkHandleMut, VkObject,
    VkRawHandle, VulkanStructure, VulkanStructureAsRef,
};
use derives::implements;

pub trait Fence: VkHandle<Handle = VkFence> + DeviceChildHandle + Status {
    /// Wait for a fence to become signaled, returns `Ok(true)` if operation is timed out
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[implements]
    #[inline(always)]
    fn wait_timeout(&self, timeout: u64) -> crate::Result<bool> {
        let vr =
            unsafe { crate::vkfn::wait_for_fences(self.device_handle(), 1, &self.native_ptr(), false as _, timeout) };
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
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[implements]
    #[inline(always)]
    fn wait(&self) -> crate::Result<()> {
        self.wait_timeout(std::u64::MAX).map(drop)
    }
}
DerefContainerBracketImpl!(for Fence {});
GuardsImpl!(for Fence {});

pub trait FenceMut: Fence + VkHandleMut {
    /// Resets a fence object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline(always)]
    fn reset(&mut self) -> crate::Result<()> {
        unsafe {
            crate::vkfn::reset_fences(self.device_handle(), 1, &self.native_ptr_mut())
                .into_result()
                .map(drop)
        }
    }
}
DerefContainerBracketImpl!(for mut FenceMut {});
GuardsImpl!(for mut FenceMut {});

pub trait DeviceChildFence: DeviceChild + Fence {}
impl<T: DeviceChild + Fence> DeviceChildFence for T {}

pub trait Semaphore: VkHandle<Handle = VkSemaphore> + DeviceChild {
    /// Creates a submit info structure for this semaphore.
    #[cfg(feature = "VK_KHR_synchronization2")]
    #[inline(always)]
    fn submit_info(&self) -> SemaphoreSubmitInfo {
        SemaphoreSubmitInfo::new(self)
    }
}
DerefContainerBracketImpl!(for Semaphore {});
GuardsImpl!(for Semaphore {});

pub trait SemaphoreMut: Semaphore + VkHandleMut {}
DerefContainerBracketImpl!(for mut SemaphoreMut {});
GuardsImpl!(for mut SemaphoreMut {});

pub trait Event: VkHandle<Handle = VkEvent> + DeviceChild + Status {}
DerefContainerBracketImpl!(for Event {});
GuardsImpl!(for Event {});

pub trait EventMut: Event + VkHandleMut {
    /// Set an event to signaled state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn set(&mut self) -> crate::Result<()> {
        unsafe {
            crate::vkfn::set_event(self.device().native_ptr(), self.native_ptr_mut())
                .into_result()
                .map(drop)
        }
    }

    /// Reset an event to non-signaled state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn reset(&mut self) -> crate::Result<()> {
        unsafe {
            crate::vkfn::reset_event(self.device().native_ptr(), self.native_ptr_mut())
                .into_result()
                .map(drop)
        }
    }
}
DerefContainerBracketImpl!(for mut EventMut {});
GuardsImpl!(for mut EventMut {});

pub trait Status {
    /// Retrieve the status(whether is signaled or not) of a synchronize object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[implements]
    fn status(&self) -> crate::Result<bool>;
}
DerefContainerBracketImpl!(for Status {
    #[implements]
    #[inline(always)]
    fn status(&self) -> crate::Result<bool> {
        T::status(self)
    }
});
GuardsImpl!(for Status {
    #[implements]
    #[inline(always)]
    fn status(&self) -> crate::Result<bool> {
        T::status(&self)
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

#[derive(VkHandle)]
pub struct FenceObject<Device: VkHandle<Handle = VkDevice>>(VkFence, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for FenceObject<Device> {
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for FenceObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for FenceObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for FenceObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for FenceObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: VkHandle<Handle = VkDevice>> Fence for FenceObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> FenceMut for FenceObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> Status for FenceObject<Device> {
    #[inline(always)]
    #[implements]
    fn status(&self) -> crate::Result<bool> {
        match unsafe { crate::vkfn::get_fence_status(self.1.native_ptr(), self.0) } {
            VK_SUCCESS => Ok(true),
            VK_NOT_READY => Ok(false),
            vr => Err(vr),
        }
    }
}
impl<Device: VkHandle<Handle = VkDevice>> FenceObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkFence, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkFence, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Device: VkHandle<Handle = VkDevice> + Clone> FenceObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> FenceObject<Device> {
        FenceObject(self.0, self.1.clone())
    }
}
impl<Device: crate::Device> FenceObject<Device> {
    /// Create a new fence object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn new(device: Device, create_info: &FenceCreateInfo) -> crate::Result<Self> {
        Ok(unsafe { Self::manage(device.new_fence_raw(create_info, None)?, device) })
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

#[derive(VkHandle, VkObject)]
#[VkObject(type = VkSemaphore::OBJECT_TYPE)]
pub struct SemaphoreObject<Device: VkHandle<Handle = VkDevice>>(VkSemaphore, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for SemaphoreObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for SemaphoreObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for SemaphoreObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for SemaphoreObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for SemaphoreObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: crate::Device> Semaphore for SemaphoreObject<Device> {}
impl<Device: crate::Device> SemaphoreMut for SemaphoreObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> SemaphoreObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkSemaphore, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkSemaphore, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Device: VkHandle<Handle = VkDevice> + Clone> SemaphoreObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> SemaphoreObject<Device> {
        SemaphoreObject(self.0, self.1.clone())
    }
}
impl<Device: crate::Device> SemaphoreObject<Device> {
    /// Create a new queue semaphore object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn new(device: Device, info: &SemaphoreCreateInfo) -> crate::Result<Self> {
        Ok(unsafe { Self::manage(device.new_semaphore_raw(info, None)?, device) })
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

#[derive(VkHandle, VkObject)]
#[VkObject(type = VkEvent::OBJECT_TYPE)]
pub struct EventObject<Device: VkHandle<Handle = VkDevice>>(VkEvent, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for EventObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for EventObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for EventObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for EventObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for EventObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: VkHandle<Handle = VkDevice>> Status for EventObject<Device> {
    #[implements]
    fn status(&self) -> crate::Result<bool> {
        match unsafe { crate::vkfn::get_event_status(self.1.native_ptr(), self.0) } {
            VK_EVENT_SET => Ok(true),
            VK_EVENT_RESET => Ok(false),
            vr => Err(vr),
        }
    }
}
impl<Device: VkHandle<Handle = VkDevice>> EventObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkEvent, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkEvent, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Device: VkHandle<Handle = VkDevice> + Clone> EventObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> EventObject<Device> {
        EventObject(self.0, self.1.clone())
    }
}
impl<Device: crate::Device> EventObject<Device> {
    /// Create a new event object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn new(device: Device, create_info: &EventCreateInfo) -> crate::Result<Self> {
        Ok(unsafe { Self::manage(device.new_event_raw(create_info, None)?, device) })
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
