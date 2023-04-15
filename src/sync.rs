//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

use crate::vk::*;
use crate::DeviceChild;
use crate::VkHandle;
#[cfg(feature = "Implements")]
use crate::{
    vkresolve::{Resolver, ResolverInterface},
    VkHandleMut,
};

DefineStdDeviceChildObject! {
    /// Opaque Handle to a fence object
    FenceObject(VkFence, VK_OBJECT_TYPE_FENCE): Fence { drop destroy_fence }
}
impl<Device: crate::Device> Status for FenceObject<Device> {
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        let vr = unsafe { Resolver::get().get_fence_status(self.device().native_ptr(), self.native_ptr()) };
        match vr.0 {
            VK_SUCCESS => Ok(true),
            VK_NOT_READY => Ok(false),
            _ => Err(vr),
        }
    }
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a semaphore object
    SemaphoreObject(VkSemaphore, VK_OBJECT_TYPE_SEMAPHORE): Semaphore { drop destroy_semaphore }
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a event object
    EventObject(VkEvent, VK_OBJECT_TYPE_EVENT): Event { drop destroy_event }
}
impl<Device: crate::Device> Status for EventObject<Device> {
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        let vr = unsafe { Resolver::get().get_event_status(self.device().native_ptr(), self.native_ptr()) };
        match vr.0 {
            VK_EVENT_SET => Ok(true),
            VK_EVENT_RESET => Ok(false),
            _ => Err(vr),
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
        match vr.0 {
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
    #[cfg(feature = "Implements")]
    fn wait(&self) -> crate::Result<()> {
        self.wait_timeout(std::u64::MAX).map(drop)
    }

    /// Resets a fence object
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
            Resolver::get()
                .reset_fences(self.device().native_ptr(), 1, &self.native_ptr_mut())
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
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
    #[cfg(unix)]
    fn get_fd(&self, ty: crate::ExternalFenceFdType) -> crate::Result<std::os::unix::io::RawFd> {
        use crate::VkResultBox;

        let info = VkFenceGetFdInfoKHR {
            sType: VkFenceGetFdInfoKHR::TYPE,
            pNext: std::ptr::null(),
            fence: self.native_ptr(),
            handleType: ty as _,
        };
        let mut fd = 0;
        let f = self
            .device()
            .extra_procedure::<PFN_vkGetFenceFdKHR>("vkGetFenceFdKHR")
            .expect("No vkGetFenceFdKHR exported");
        VkResultBox((f)(self.device().native_ptr(), &info, &mut fd))
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
    #[cfg(unix)]
    fn import(
        &self,
        ty: crate::ExternalFenceFdType,
        fd: std::os::unix::io::RawFd,
        temporary: bool,
    ) -> crate::Result<()> {
        use crate::VkResultBox;

        let info = VkImportFenceFdInfoKHR {
            sType: VkImportFenceFdInfoKHR::TYPE,
            pNext: std::ptr::null(),
            fence: self.native_ptr(),
            flags: if temporary { VK_FENCE_IMPORT_TEMPORARY_BIT } else { 0 },
            handleType: ty as _,
            fd,
        };
        let f = self
            .device()
            .extra_procedure::<PFN_vkImportFenceFdKHR>("vkImportFenceFdKHR")
            .expect("No vkImportFenceFdKHR exported");
        VkResultBox((f)(self.device().native_ptr(), &info))
            .into_result()
            .map(drop)
    }
}
DerefContainerBracketImpl!(for Fence {});
GuardsImpl!(for Fence {});

pub trait Semaphore: VkHandle<Handle = VkSemaphore> {}
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
            Resolver::get()
                .set_event(self.device().native_ptr(), self.native_ptr_mut())
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
            Resolver::get()
                .reset_event(self.device().native_ptr(), self.native_ptr_mut())
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
