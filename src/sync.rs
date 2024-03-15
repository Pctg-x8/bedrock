//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

use crate::chain;
use crate::vk::*;
use crate::DeviceChild;
use crate::VkHandle;
#[implements]
use crate::{Device, VkHandleMut};
use crate::{GenericVulkanStructure, VulkanStructure};
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
