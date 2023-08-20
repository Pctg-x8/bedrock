//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

use crate::chain;
use crate::vk::*;
use crate::DeviceChild;
use crate::VkHandle;
#[cfg(feature = "Implements")]
use crate::{Device, VkHandleMut, VkResultBox};
use crate::{GenericVulkanStructure, VulkanStructure};
use derives::implements;

DefineStdDeviceChildObject! {
    /// Opaque Handle to a fence object
    FenceObject(VkFence, VK_OBJECT_TYPE_FENCE): Fence { drop destroy_fence }
}
impl<Device: crate::Device> Status for FenceObject<Device> {
    #[cfg(feature = "Implements")]
    fn status(&self) -> crate::Result<bool> {
        let vr = unsafe { crate::vkresolve::get_fence_status(self.device().native_ptr(), self.native_ptr()) };
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
        let vr = unsafe { crate::vkresolve::get_event_status(self.device().native_ptr(), self.native_ptr()) };
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
            crate::vkresolve::wait_for_fences(self.device().native_ptr(), 1, &self.native_ptr(), false as _, timeout)
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
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
    #[cfg(unix)]
    fn get_external_handle(&self, ty: crate::ExternalFenceFdType) -> crate::Result<std::os::unix::io::RawFd> {
        use crate::{ext::VulkanStructure, Device, VkResultBox};

        let info = VkFenceGetFdInfoKHR {
            sType: VkFenceGetFdInfoKHR::TYPE,
            pNext: std::ptr::null(),
            fence: self.native_ptr(),
            handleType: ty as _,
        };
        let mut fd = 0;
        unsafe {
            VkResultBox(self.device().get_fence_fd_khr_fn().0(
                self.device().native_ptr(),
                &info,
                &mut fd,
            ))
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
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
    #[cfg(unix)]
    fn import(
        &self,
        ty: crate::ExternalFenceFdType,
        fd: std::os::unix::io::RawFd,
        temporary: bool,
    ) -> crate::Result<()> {
        use crate::{ext::VulkanStructure, Device, VkResultBox};

        let info = VkImportFenceFdInfoKHR {
            sType: VkImportFenceFdInfoKHR::TYPE,
            pNext: std::ptr::null(),
            fence: self.native_ptr(),
            flags: if temporary {
                VK_FENCE_IMPORT_TEMPORARY_BIT_KHR
            } else {
                0
            },
            handleType: ty as _,
            fd,
        };

        unsafe {
            VkResultBox(self.device().import_fence_fd_khr_fn().0(
                self.device().native_ptr(),
                &info,
            ))
            .into_result()
            .map(drop)
        }
    }
}
DerefContainerBracketImpl!(for Fence {});
GuardsImpl!(for Fence {});

pub struct SemaphoreBuilder(VkSemaphoreCreateInfo, Vec<Box<GenericVulkanStructure>>);
impl SemaphoreBuilder {
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

    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    pub fn with_export(
        mut self,
        handle_types: crate::ExternalSemaphoreHandleTypes,
        export_info: &crate::ExportSemaphoreWin32HandleInfo,
    ) -> Self {
        self.1.push(unsafe {
            core::mem::transmute(Box::new(VkExportSemaphoreCreateInfoKHR {
                sType: VkExportSemaphoreCreateInfoKHR::TYPE,
                pNext: export_info.as_ref() as *const _ as _,
                handleTypes: handle_types.into(),
            }))
        });

        self
    }

    /// Create a new queue semaphore object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    pub fn create<Device: crate::Device>(mut self, device: Device) -> crate::Result<SemaphoreObject<Device>> {
        chain(&mut self.0, &mut self.1);

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_semaphore(device.native_ptr(), &self.0, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| SemaphoreObject(h.assume_init(), device))
        }
    }
}

pub trait Semaphore: VkHandle<Handle = VkSemaphore> + DeviceChild {
    /// Get a Windows HANDLE for a semaphore
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_TOO_MANY_OBJECTS
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    #[implements]
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
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
            VkResultBox(self.device().get_semaphore_win32_handle_khr_fn().0(
                self.device().native_ptr(),
                &info,
                h.as_mut_ptr(),
            ))
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
    #[implements]
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    fn import(&self, handle: crate::ExternalSemaphoreHandleWin32, name: &widestring::WideCString) -> crate::Result<()> {
        let info = VkImportSemaphoreWin32HandleInfoKHR {
            sType: VkImportSemaphoreWin32HandleInfoKHR::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            semaphore: self.native_ptr(),
            handleType: handle.as_type_bits(),
            handle: handle.handle(),
            name: windows::core::PCWSTR(name.as_ptr()),
        };

        unsafe {
            VkResultBox(self.device().import_semaphore_win32_handle_khr_fn().0(
                self.device().native_ptr(),
                &info,
            ))
            .into_result()
            .map(drop)
        }
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
