//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

#![cfg_attr(not(feature = "Implements"), allow(dead_code))]

use crate::vk::*;
use crate::DeviceChild;
use crate::VkHandle;
#[cfg(feature = "Implements")]
use crate::{
    vkresolve::{Resolver, ResolverInterface},
    VkResultBox, VkResultHandler,
};

DefineStdDeviceChildObject! {
    /// Opaque Handle to a fence object
    #[object_type = "VK_OBJECT_TYPE_FENCE"]
    FenceObject(VkFence): Fence { drop destroy_fence }
}
impl<Device: crate::Device> Status for FenceObject<Device> {
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        let vr = unsafe { Resolver::get().get_fence_status(self.device().native_ptr(), self.native_ptr()) };
        match vr {
            VK_SUCCESS => Ok(true),
            VK_NOT_READY => Ok(false),
            _ => Err(VkResultBox(vr)),
        }
    }
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a semaphore object
    #[object_type = "VK_OBJECT_TYPE_SEMAPHORE"]
    SemaphoreObject(VkSemaphore): Semaphore { drop destroy_semaphore }
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a event object
    #[object_type = "VK_OBJECT_TYPE_EVENT"]
    EventObject(VkEvent): Event { drop destroy_event }
}
impl<Device: crate::Device> Status for EventObject<Device> {
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        let vr = unsafe { Resolver::get().get_event_status(self.device().native_ptr(), self.native_ptr()) };
        match vr {
            VK_EVENT_SET => Ok(true),
            VK_EVENT_RESET => Ok(false),
            _ => Err(VkResultBox(vr)),
        }
    }
}

pub trait Fence: VkHandle<Handle = VkFence> + DeviceChild + Status {
    /// Wait for a fence to become signaled, returns `Ok(true)` if operation is timed out
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[cfg(feature = "Implements")]
    fn wait_timeout(&self, timeout: u64) -> crate::Result<bool> {
        let vr = unsafe {
            Resolver::get().wait_for_fences(self.device().native_ptr(), 1, &self.native_ptr(), false as _, timeout)
        };
        match vr {
            VK_SUCCESS => Ok(false),
            VK_TIMEOUT => Ok(true),
            _ => Err(VkResultBox(vr)),
        }
    }

    /// Wait for a fence to become signaled
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[cfg(feature = "Implements")]
    fn wait(&mut self) -> crate::Result<()> {
        self.wait_timeout(std::u64::MAX).map(drop)
    }

    /// Resets a fence object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn reset(&mut self) -> crate::Result<()> {
        unsafe {
            Resolver::get()
                .reset_fences(self.device().native_ptr(), 1, &self.native_ptr())
                .into_result()
        }
    }

    /// Get a POSIX file descriptor handle for a type
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
    fn get_fd(&self, ty: crate::ExternalFenceFdType) -> crate::Result<std::os::unix::io::RawFd> {
        let info = VkFenceGetFdInfoKHR {
            fence: self.native_ptr(),
            handleType: ty as _,
            ..Default::default()
        };
        let mut fd = 0;
        let f = self
            .device()
            .extra_procedure::<PFN_vkGetFenceFdKHR>("vkGetFenceFdKHR")
            .expect("No vkGetFenceFdKHR exported");
        (f)(self.device().native_ptr(), &info, &mut fd)
            .into_result()
            .map(move |_| fd)
    }

    /// Import a fence from a POSIX file descriptor
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
    fn import(
        &self,
        ty: crate::ExternalFenceFdType,
        fd: std::os::unix::io::RawFd,
        temporary: bool,
    ) -> crate::Result<()> {
        let info = VkImportFenceFdInfoKHR {
            fence: self.native_ptr(),
            flags: if temporary { VK_FENCE_IMPORT_TEMPORARY_BIT } else { 0 },
            handleType: ty as _,
            fd,
            ..Default::default()
        };
        let f = self
            .device()
            .extra_procedure::<PFN_vkImportFenceFdKHR>("vkImportFenceFdKHR")
            .expect("No vkImportFenceFdKHR exported");
        (f)(self.device().native_ptr(), &info).into_result()
    }
}
impl<T> Fence for &'_ T where T: Fence + ?Sized {}
impl<T> Fence for std::rc::Rc<T> where T: Fence + ?Sized {}
impl<T> Fence for std::sync::Arc<T> where T: Fence + ?Sized {}

pub trait Semaphore: VkHandle<Handle = VkSemaphore> {}
impl<T> Semaphore for &'_ T where T: Semaphore + ?Sized {}
impl<T> Semaphore for std::rc::Rc<T> where T: Semaphore + ?Sized {}
impl<T> Semaphore for std::sync::Arc<T> where T: Semaphore + ?Sized {}

pub trait Event: VkHandle<Handle = VkEvent> + DeviceChild + Status {
    /// Set an event to signaled state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn set(&mut self) -> crate::Result<()> {
        unsafe {
            Resolver::get()
                .set_event(self.device().native_ptr(), self.native_ptr())
                .into_result()
        }
    }

    /// Reset an event to non-signaled state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn reset(&mut self) -> crate::Result<()> {
        unsafe {
            Resolver::get()
                .reset_event(self.device().native_ptr(), self.native_ptr())
                .into_result()
        }
    }
}
impl<T> Event for &'_ T where T: Event + ?Sized {}
impl<T> Event for std::rc::Rc<T> where T: Event + ?Sized {}
impl<T> Event for std::sync::Arc<T> where T: Event + ?Sized {}

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
impl<T> Status for &'_ T
where
    T: Status + ?Sized,
{
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        T::status(self)
    }
}
impl<T> Status for std::rc::Rc<T>
where
    T: Status + ?Sized,
{
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        T::status(self)
    }
}
impl<T> Status for std::sync::Arc<T>
where
    T: Status + ?Sized,
{
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        T::status(self)
    }
}
