//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

use crate::*;
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

    /// Query the current state of a timeline semaphore
    #[implements("Allow1_2APIs")]
    #[inline(always)]
    fn counter_value(&self) -> crate::Result<u64> {
        let mut sink = core::mem::MaybeUninit::uninit();

        unsafe {
            crate::vkfn::get_semaphore_counter_value(self.device().native_ptr(), self.native_ptr(), sink.as_mut_ptr())
                .into_result()?;
        }

        Ok(unsafe { sink.assume_init() })
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
#[derive(Clone)]
pub struct FenceCreateInfo<'d>(
    VkFenceCreateInfo,
    core::marker::PhantomData<Option<&'d dyn VulkanStructure>>,
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
    pub fn with_next(mut self, next: &'d (impl TypedVulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}

#[derive(VkHandle, VkObject)]
#[VkObject(type = VkFence::OBJECT_TYPE)]
pub struct FenceObject<Device: VkHandle<Handle = VkDevice>>(VkFence, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for FenceObject<Device> {
    fn drop(&mut self) {
        unsafe {
            crate::vkfn::destroy_fence(self.1.native_ptr(), self.0, core::ptr::null());
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
#[derive(Clone)]
pub struct SemaphoreCreateInfo<'d>(
    VkSemaphoreCreateInfo,
    core::marker::PhantomData<Option<&'d dyn VulkanStructure>>,
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

#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub type PhysicalDeviceTimelineSemaphoreFeatures = VkPhysicalDeviceTimelineSemaphoreFeaturesKHR;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
impl PhysicalDeviceTimelineSemaphoreFeatures {
    #[inline(always)]
    pub const fn new(active: bool) -> Self {
        Self {
            sType: <VkPhysicalDeviceTimelineSemaphoreFeaturesKHR as TypedVulkanStructure>::TYPE,
            pNext: core::ptr::null_mut(),
            timelineSemaphore: active as _,
        }
    }

    #[inline(always)]
    pub const fn uninit_sink() -> core::mem::MaybeUninit<Self> {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            core::ptr::addr_of_mut!((*p.as_mut_ptr()).sType)
                .write(<VkPhysicalDeviceTimelineSemaphoreFeaturesKHR as TypedVulkanSinkStructure>::TYPE);
            core::ptr::addr_of_mut!((*p.as_mut_ptr()).pNext).write(core::ptr::null_mut());
        }

        p
    }
}

#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub struct SemaphoreTypeCreateInfo<'d>(
    VkSemaphoreTypeCreateInfoKHR,
    core::marker::PhantomData<Option<&'d dyn VulkanStructure>>,
);
#[cfg(feature = "VK_KHR_timeline_semaphore")]
impl<'d> SemaphoreTypeCreateInfo<'d> {
    pub const fn binary() -> Self {
        Self(
            VkSemaphoreTypeCreateInfoKHR {
                sType: VkSemaphoreTypeCreateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                semaphoreType: VK_SEMAPHORE_TYPE_BINARY_KHR,
                initialValue: 0,
            },
            core::marker::PhantomData,
        )
    }

    pub const fn timeline(init: u64) -> Self {
        Self(
            VkSemaphoreTypeCreateInfoKHR {
                sType: VkSemaphoreTypeCreateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                semaphoreType: VK_SEMAPHORE_TYPE_TIMELINE_KHR,
                initialValue: init,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkSemaphoreTypeCreateInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkSemaphoreTypeCreateInfoKHR {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
unsafe impl VulkanStructure for SemaphoreTypeCreateInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        self.0.as_generic()
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        self.0.as_generic_mut()
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
            crate::vkfn::destroy_semaphore(self.1.native_ptr(), self.0, core::ptr::null());
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

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[repr(C)]
pub enum SemaphoreType {
    Binary = VK_SEMAPHORE_TYPE_BINARY_KHR as _,
    Timeline = VK_SEMAPHORE_TYPE_TIMELINE_KHR as _,
}

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[repr(transparent)]
pub struct SemaphoreSignalInfo<'s, 'n>(
    VkSemaphoreSignalInfoKHR,
    core::marker::PhantomData<(&'s dyn VkHandle<Handle = VkSemaphore>, Option<&'n dyn VulkanStructure>)>,
);
#[cfg(feature = "VK_KHR_timeline_semaphore")]
impl<'s, 'n> SemaphoreSignalInfo<'s, 'n> {
    #[inline(always)]
    pub fn new(semaphore: &'s (impl VkHandle<Handle = VkSemaphore> + ?Sized), value: u64) -> Self {
        Self(
            VkSemaphoreSignalInfoKHR {
                sType: VkSemaphoreSignalInfoKHR::TYPE,
                pNext: core::ptr::null(),
                semaphore: semaphore.native_ptr(),
                value,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn new_raw(semaphore: VkSemaphore, value: u64) -> Self {
        Self(
            VkSemaphoreSignalInfoKHR {
                sType: VkSemaphoreSignalInfoKHR::TYPE,
                pNext: core::ptr::null(),
                semaphore,
                value,
            },
            core::marker::PhantomData,
        )
    }

    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn from_raw(raw: VkSemaphoreSignalInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkSemaphoreSignalInfoKHR {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'n (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
unsafe impl VulkanStructure for SemaphoreSignalInfo<'_, '_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        self.0.as_generic()
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        self.0.as_generic_mut()
    }
}

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[repr(transparent)]
pub struct SemaphoreWaitInfo<'s, 'n, 'xs>(
    VkSemaphoreWaitInfoKHR,
    core::marker::PhantomData<(
        &'xs [&'s dyn VkHandle<Handle = VkSemaphore>],
        &'xs [u64],
        Option<&'n dyn VulkanStructure>,
    )>,
);
#[cfg(feature = "VK_KHR_timeline_semaphore")]
impl<'s, 'n, 'xs> SemaphoreWaitInfo<'s, 'n, 'xs> {
    #[inline(always)]
    pub fn new(semaphores: &'xs [VkHandleRef<'s, VkSemaphore>], values: &'xs [u64]) -> Self {
        use crate::ffi_helper::slice_as_ptr_empty_null;

        debug_assert_eq!(semaphores.len(), values.len());

        Self(
            VkSemaphoreWaitInfoKHR {
                sType: VkSemaphoreWaitInfoKHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                semaphoreCount: semaphores.len() as _,
                pSemaphores: slice_as_ptr_empty_null(semaphores) as _,
                pValues: slice_as_ptr_empty_null(values),
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn new_array<const N: usize>(
        semaphores: &'xs [VkHandleRef<'s, VkSemaphore>; N],
        values: &'xs [u64; N],
    ) -> Self {
        use crate::ffi_helper::slice_as_ptr_empty_null;

        Self(
            VkSemaphoreWaitInfoKHR {
                sType: VkSemaphoreWaitInfoKHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                semaphoreCount: N as _,
                pSemaphores: slice_as_ptr_empty_null(semaphores) as _,
                pValues: slice_as_ptr_empty_null(values),
            },
            core::marker::PhantomData,
        )
    }

    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn from_raw(raw: VkSemaphoreWaitInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkSemaphoreWaitInfoKHR {
        self.0
    }

    #[inline(always)]
    pub const fn for_any(mut self) -> Self {
        self.0.flags |= VK_SEMAPHORE_WAIT_ANY_BIT_KHR;
        self
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'n (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
unsafe impl VulkanStructure for SemaphoreWaitInfo<'_, '_, '_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        self.0.as_generic()
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        self.0.as_generic_mut()
    }
}

#[repr(transparent)]
#[derive(Clone)]
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
            crate::vkfn::destroy_event(self.1.native_ptr(), self.0, core::ptr::null());
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
