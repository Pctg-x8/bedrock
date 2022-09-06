//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

#![cfg_attr(not(feature = "Implements"), allow(dead_code))]

use crate::vk::*;
use crate::VkHandle;
#[cfg(feature = "Implements")]
use crate::{
    vkresolve::{Resolver, ResolverInterface},
    VkResultBox, VkResultHandler,
};

/// Opaque handle to a fence object
#[derive(VkHandle)]
#[object_type = "VK_OBJECT_TYPE_FENCE"]
pub struct Fence<Device>(VkFence, Device)
where
    Device: VkHandle<Handle = VkDevice>;
impl<Device: VkHandle<Handle = VkDevice>> Drop for Fence<Device> {
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_fence(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}

/// Opaque handle to a semaphore object
#[derive(VkHandle)]
#[object_type = "VK_OBJECT_TYPE_SEMAPHORE"]
pub struct Semaphore<Device>(VkSemaphore, Device)
where
    Device: VkHandle<Handle = VkDevice>;
impl<Device: VkHandle<Handle = VkDevice>> Drop for Semaphore<Device> {
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_semaphore(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}

/// Opaque handle to a event object
#[derive(VkHandle)]
#[object_type = "VK_OBJECT_TYPE_EVENT"]
pub struct Event<Device>(VkEvent, Device)
where
    Device: VkHandle<Handle = VkDevice>;
impl<Device: VkHandle<Handle = VkDevice>> Drop for Event<Device> {
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_event(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}

/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>> Fence<Device> {
    /// Create a new fence object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn new(device: Device, signaled: bool) -> crate::Result<Self> {
        let mut h = VK_NULL_HANDLE as _;
        let flags = if signaled { VK_FENCE_CREATE_SIGNALED_BIT } else { 0 };
        unsafe {
            Resolver::get()
                .create_fence(
                    device.native_ptr(),
                    &VkFenceCreateInfo {
                        flags,
                        ..Default::default()
                    },
                    std::ptr::null(),
                    &mut h,
                )
                .into_result()
                .map(|_| Self(h, device))
        }
    }

    #[cfg(feature = "VK_KHR_external_fence_fd")]
    /// [Implements][VK_KHR_exteranl_fence_fd] Create a new fence object, with exporting as file descriptors
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn with_export_fd(
        device: Device,
        signaled: bool,
        compatible_handle_types: crate::ExternalFenceHandleTypes,
    ) -> crate::Result<Self> {
        let mut h = VK_NULL_HANDLE as _;
        let exp_info = VkExportFenceCreateInfo {
            handleTypes: compatible_handle_types.0,
            ..Default::default()
        };
        let cinfo = VkFenceCreateInfo {
            flags: if signaled { VK_FENCE_CREATE_SIGNALED_BIT } else { 0 },
            pNext: &exp_info as *const _ as _,
            ..Default::default()
        };
        unsafe {
            Resolver::get()
                .create_fence(device.native_ptr(), &cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(move |_| Self(h, device))
        }
    }
}

/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>> Semaphore<Device> {
    /// Create a new queue semaphore object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn new(device: Device) -> crate::Result<Self> {
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_semaphore(device.native_ptr(), &Default::default(), std::ptr::null(), &mut h)
                .into_result()
                .map(|_| Semaphore(h, device))
        }
    }
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    /// [Implements][VK_KHR_external_semaphore_win32] Create a new queue semaphore object, with exporting as Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn with_export_win32(
        device: Device,
        handle_types: crate::ExternalSemaphoreHandleTypes,
        export_info: &crate::ExportSemaphoreWin32HandleInfo,
    ) -> crate::Result<Self> {
        let exp_info = VkExportSemaphoreCreateInfo {
            handleTypes: handle_types.into(),
            pNext: export_info.as_ref() as *const _ as _,
            ..Default::default()
        };
        let info = VkSemaphoreCreateInfo {
            pNext: &exp_info as *const _ as _,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_semaphore(device.native_ptr(), &info, std::ptr::null(), &mut h)
                .into_result()
                .map(move |_| Semaphore(h, device))
        }
    }
}

#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>> Event<Device> {
    /// Create a new event object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn new(device: Device) -> crate::Result<Self> {
        let mut h = VK_NULL_HANDLE as _;
        unsafe {
            Resolver::get()
                .create_event(device.native_ptr(), &Default::default(), std::ptr::null(), &mut h)
                .into_result()
                .map(|_| Event(h, device))
        }
    }
}

#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>> Fence<Device> {
    /// Wait for one or more fences to become signaled, returns `Ok(true)` if operation is timed out
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    pub fn wait_multiple(objects: &[&Self], wait_all: bool, timeout: Option<u64>) -> crate::Result<bool> {
        let objects_ptr = objects.iter().map(|x| x.0).collect::<Vec<_>>();
        let vr = unsafe {
            Resolver::get().wait_for_fences(
                objects[0].1.native_ptr(),
                objects_ptr.len() as _,
                objects_ptr.as_ptr(),
                wait_all as _,
                timeout.unwrap_or(std::u64::MAX),
            )
        };
        match vr {
            VK_SUCCESS => Ok(false),
            VK_TIMEOUT => Ok(true),
            _ => Err(VkResultBox(vr)),
        }
    }
    /// Wait for a fence to become signaled, returns `Ok(true)` if operation is timed out
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    pub fn wait_timeout(&self, timeout: u64) -> crate::Result<bool> {
        let vr = unsafe { Resolver::get().wait_for_fences(self.1.native_ptr(), 1, &self.0, false as _, timeout) };
        match vr {
            VK_SUCCESS => Ok(false),
            VK_TIMEOUT => Ok(true),
            _ => Err(VkResultBox(vr)),
        }
    }
    /// [feature = "Implements"] Wait for a fence to become signaled
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    pub fn wait(&mut self) -> crate::Result<()> {
        self.wait_timeout(std::u64::MAX).map(drop)
    }

    /// Resets one or more fence objects
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn reset_multiple(objects: &[&mut Self]) -> crate::Result<()> {
        let objects_ptr = objects.iter().map(|x| x.0).collect::<Vec<_>>();
        unsafe {
            Resolver::get()
                .reset_fences(objects[0].1.native_ptr(), objects_ptr.len() as _, objects_ptr.as_ptr())
                .into_result()
        }
    }
    /// Resets a fence object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn reset(&mut self) -> crate::Result<()> {
        unsafe {
            Resolver::get()
                .reset_fences(self.1.native_ptr(), 1, &self.0)
                .into_result()
        }
    }
}

/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>> Event<Device> {
    /// Set an event to signaled state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn set(&mut self) -> crate::Result<()> {
        unsafe { Resolver::get().set_event(self.1.native_ptr(), self.0).into_result() }
    }
    /// Reset an event to non-signaled state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn reset(&mut self) -> crate::Result<()> {
        unsafe { Resolver::get().reset_event(self.1.native_ptr(), self.0).into_result() }
    }
}

#[cfg(feature = "Implements")]
pub trait Status {
    /// [feature = "Implements"] Retrieve the status(whether is signaled or not) of a synchronize object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    fn status(&self) -> crate::Result<bool>;
}
#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>> Status for Fence<Device> {
    fn status(&self) -> crate::Result<bool> {
        let vr = unsafe { Resolver::get().get_fence_status(self.1.native_ptr(), self.0) };
        match vr {
            VK_SUCCESS => Ok(true),
            VK_NOT_READY => Ok(false),
            _ => Err(VkResultBox(vr)),
        }
    }
}
#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>> Status for Event<Device> {
    fn status(&self) -> crate::Result<bool> {
        let vr = unsafe { Resolver::get().get_event_status(self.1.native_ptr(), self.0) };
        match vr {
            VK_EVENT_SET => Ok(true),
            VK_EVENT_RESET => Ok(false),
            _ => Err(VkResultBox(vr)),
        }
    }
}

#[cfg(feature = "Multithreaded")]
unsafe impl<Device: VkHandle<Handle = VkDevice>> Send for Fence<Device> {}
#[cfg(feature = "Multithreaded")]
unsafe impl<Device: VkHandle<Handle = VkDevice>> Sync for Fence<Device> {}

#[cfg(feature = "Multithreaded")]
unsafe impl<Device: VkHandle<Handle = VkDevice>> Send for Event<Device> {}
#[cfg(feature = "Multithreaded")]
unsafe impl<Device: VkHandle<Handle = VkDevice>> Sync for Event<Device> {}

#[cfg(feature = "Multithreaded")]
unsafe impl<Device: VkHandle<Handle = VkDevice>> Send for Semaphore<Device> {}
#[cfg(feature = "Multithreaded")]
unsafe impl<Device: VkHandle<Handle = VkDevice>> Sync for Semaphore<Device> {}
