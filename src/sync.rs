//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

use crate::{
    chain, vk::*, DeviceChild, DeviceChildHandle, GenericVulkanStructure, VkDeviceChildNonExtDestroyable, VkHandle,
    VkHandleMut, VkObject, VkRawHandle, VulkanStructure,
};
use derives::{implements, transparent_marked};

///
/// ```
/// // Fence is object-safe
/// let _fo: Option<Box<dyn bedrock::Fence>> = None;
/// ```
pub trait Fence: VkHandle<Handle = VkFence> + DeviceChildHandle + Status {
    #[inline(always)]
    fn as_transparent_ref(&self) -> FenceRef {
        FenceRef(self.native_ptr(), core::marker::PhantomData)
    }

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
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[implements]
    #[inline(always)]
    fn wait(&self) -> crate::Result<()> {
        self.wait_timeout(std::u64::MAX).map(drop)
    }

    /// Get a POSIX file descriptor handle for a type
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements("VK_KHR_external_fence_fd")]
    unsafe fn get_external_handle_raw(&self, info: &VkFenceGetFdInfoKHR) -> crate::Result<std::os::unix::io::RawFd>
    where
        Self: DeviceChild,
    {
        use crate::Device;

        let mut fd = 0;
        self.device().get_fence_fd_khr_fn().0(self.device_handle(), info, &mut fd).into_result()?;
        Ok(fd)
    }

    /// Get a POSIX file descriptor handle for a type
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements("VK_KHR_external_fence_fd")]
    #[inline]
    fn get_external_handle(&self, ty: crate::ExternalFenceFdType) -> crate::Result<std::os::unix::io::RawFd>
    where
        Self: DeviceChild,
    {
        unsafe {
            self.get_external_handle_raw(&VkFenceGetFdInfoKHR {
                sType: VkFenceGetFdInfoKHR::TYPE,
                pNext: std::ptr::null(),
                fence: self.native_ptr(),
                handleType: ty as _,
            })
        }
    }

    /// Import a fence from a POSIX file descriptor
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements("VK_KHR_external_fence_fd")]
    unsafe fn import_fd_raw(&self, info: &VkImportFenceFdInfoKHR) -> crate::Result<()>
    where
        Self: DeviceChild,
    {
        use crate::Device;

        self.device().import_fence_fd_khr_fn().0(self.device_handle(), info)
            .into_result()
            .map(drop)
    }

    /// Import a fence from a POSIX file descriptor
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements("VK_KHR_external_fence_fd")]
    #[inline]
    fn import_fd(&self, handle: crate::ExternalFenceFd, temporary: bool) -> crate::Result<()>
    where
        Self: DeviceChild,
    {
        unsafe {
            self.import_fd_raw(&VkImportFenceFdInfoKHR {
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
            })
        }
    }
}
DerefContainerBracketImpl!(for Fence {});
GuardsImpl!(for Fence {});

///
/// ```
/// // FenceMut is object-safe
/// let _fo: Option<Box<dyn bedrock::FenceMut>> = None;
/// ```
pub trait FenceMut: Fence + VkHandleMut {
    #[inline(always)]
    fn as_transparent_mut_ref(&mut self) -> FenceMutRef {
        FenceMutRef(self.native_ptr_mut(), core::marker::PhantomData)
    }

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
    #[inline(always)]
    fn as_transparent_ref(&self) -> SemaphoreRef {
        SemaphoreRef(self.native_ptr(), core::marker::PhantomData)
    }

    /// Get a Windows HANDLE for a semaphore
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_TOO_MANY_OBJECTS
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements("VK_KHR_external_semaphore_win32")]
    unsafe fn request_external_handle_raw(
        &self,
        info: &VkSemaphoreGetWin32HandleInfoKHR,
    ) -> crate::Result<windows::Win32::Foundation::HANDLE> {
        use crate::Device;

        let mut h = core::mem::MaybeUninit::uninit();

        self.device().get_semaphore_win32_handle_khr_fn().0(self.device().native_ptr(), info, h.as_mut_ptr())
            .into_result()?;

        Ok(h.assume_init())
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
    fn request_external_handle(
        &self,
        handle_type: crate::ExternalSemaphoreHandleTypeWin32,
    ) -> crate::Result<windows::Win32::Foundation::HANDLE> {
        unsafe {
            self.request_external_handle_raw(&VkSemaphoreGetWin32HandleInfoKHR {
                sType: VkSemaphoreGetWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                semaphore: self.native_ptr(),
                handleType: handle_type as _,
            })
        }
    }

    /// Import a semaphore from a Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_INVALID_EXTERNAL_HANDLE
    #[implements("VK_KHR_external_semaphore_win32")]
    unsafe fn import_raw(&self, info: &VkImportSemaphoreWin32HandleInfoKHR) -> crate::Result<()> {
        use crate::Device;

        self.device().import_semaphore_win32_handle_khr_fn().0(self.device().native_ptr(), info)
            .into_result()
            .map(drop)
    }

    /// Import a semaphore from a Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_INVALID_EXTERNAL_HANDLE
    #[implements("VK_KHR_external_semaphore_win32")]
    #[inline]
    fn import(&self, handle: crate::ExternalSemaphoreHandleWin32, name: &widestring::WideCString) -> crate::Result<()> {
        unsafe {
            self.import_raw(&VkImportSemaphoreWin32HandleInfoKHR {
                sType: VkImportSemaphoreWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                semaphore: self.native_ptr(),
                handleType: handle.0 as _,
                handle: handle.1,
                name: windows::core::PCWSTR(name.as_ptr()),
            })
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

pub trait SemaphoreMut: Semaphore + VkHandleMut {
    #[inline(always)]
    fn as_transparent_mut_ref(&mut self) -> SemaphoreMutRef {
        SemaphoreMutRef(self.native_ptr_mut(), core::marker::PhantomData)
    }
}
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
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
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
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
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
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    #[implements]
    fn status(&self) -> crate::Result<bool>;
}
DerefContainerBracketImpl!(for Status {
    #[implements]
    fn status(&self) -> crate::Result<bool> {
        T::status(self)
    }
});
GuardsImpl!(for Status {
    #[implements]
    fn status(&self) -> crate::Result<bool> {
        T::status(&self)
    }
});

/// A handle reference to a fence
#[transparent_marked]
pub struct FenceRef<'r>(
    pub(crate) VkFence,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = VkFence>>,
);
impl<'r> FenceRef<'r> {
    #[inline(always)]
    pub const fn unbounded(h: VkFence) -> Self {
        Self(h, core::marker::PhantomData)
    }
}
impl<'r> VkHandle for FenceRef<'r> {
    type Handle = VkFence;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle {
        self.0
    }
}

/// A mutable handle reference to a fence
#[transparent_marked]
pub struct FenceMutRef<'r>(
    pub(crate) VkFence,
    core::marker::PhantomData<&'r mut dyn VkHandle<Handle = VkFence>>,
);
impl<'r> FenceMutRef<'r> {
    pub const NULL: Self = Self(VkFence::NULL, core::marker::PhantomData);

    #[inline(always)]
    pub fn unbounded(h: VkFence) -> Self {
        Self(h, core::marker::PhantomData)
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

/// A handle reference to a semaphore
#[transparent_marked]
pub struct SemaphoreRef<'r>(
    pub(crate) VkSemaphore,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = VkSemaphore>>,
);
impl<'r> SemaphoreRef<'r> {
    pub const fn unbounded(h: VkSemaphore) -> Self {
        Self(h, core::marker::PhantomData)
    }
}
impl<'r> VkHandle for SemaphoreRef<'r> {
    type Handle = VkSemaphore;

    fn native_ptr(&self) -> Self::Handle {
        self.0
    }
}

/// A mutable handle reference to a semaphore
#[transparent_marked]
pub struct SemaphoreMutRef<'r>(
    pub(crate) VkSemaphore,
    core::marker::PhantomData<&'r mut dyn VkHandleMut<Handle = VkSemaphore>>,
);
impl<'r> SemaphoreMutRef<'r> {
    pub fn unbounded(h: VkSemaphore) -> Self {
        Self(h, core::marker::PhantomData)
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
    #[inline(always)]
    pub const fn as_transparent_ref(&self) -> SemaphoreRef {
        SemaphoreRef(self.0, core::marker::PhantomData)
    }
}

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

#[transparent_marked]
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
    /// Creates a new object from info structure
    pub fn new(device: Device, create_info: &EventCreateInfo) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_event(device.native_ptr(), &create_info.0, core::ptr::null(), h.as_mut_ptr())
                .into_result()?;

            Ok(Self::manage(h.assume_init(), device))
        }
    }

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

        unsafe { FenceObject::new_raw(device, &self.0) }
    }
}

impl<Device: VkHandle<Handle = VkDevice>> FenceObject<Device> {
    /// Create a new fence object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    #[inline]
    pub unsafe fn new_raw(device: Device, info: &VkFenceCreateInfo) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_fence(device.native_ptr(), info, core::ptr::null(), h.as_mut_ptr()).into_result()?;

        Ok(Self::manage(h.assume_init(), device))
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

        unsafe { SemaphoreObject::new_raw(device, &self.0) }
    }
}

impl<Device: VkHandle<Handle = VkDevice>> SemaphoreObject<Device> {
    /// Create a new queue semaphore object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    #[inline]
    pub unsafe fn new_raw(device: Device, info: &VkSemaphoreCreateInfo) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_semaphore(device.native_ptr(), info, core::ptr::null(), h.as_mut_ptr()).into_result()?;

        Ok(Self::manage(h.assume_init(), device))
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
