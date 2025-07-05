use crate::ffi_helper::{opt_pointer, slice_as_ptr_empty_null};
use crate::*;

#[repr(C)]
pub enum ExternalSemaphoreHandleTypeWin32 {
    OpaqueWin32 = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR as _,
    OpaqueWin32KMT = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR as _,
    D3DFence = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR as _,
}
impl ExternalSemaphoreHandleTypeWin32 {
    pub const fn with_handle(self, handle: windows::Win32::Foundation::HANDLE) -> ExternalSemaphoreHandleWin32 {
        ExternalSemaphoreHandleWin32(self, handle)
    }
}

pub struct ExternalSemaphoreHandleWin32(
    pub ExternalSemaphoreHandleTypeWin32,
    pub windows::Win32::Foundation::HANDLE,
);

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ImportSemaphoreWin32HandleInfo<'d>(
    pub(crate) VkImportSemaphoreWin32HandleInfoKHR,
    core::marker::PhantomData<(&'d dyn VkHandle<Handle = VkSemaphore>, &'d widestring::WideCStr)>,
);
impl<'d> ImportSemaphoreWin32HandleInfo<'d> {
    #[inline]
    pub fn by_handle(
        semaphore: &'d (impl VkHandle<Handle = VkSemaphore> + ?Sized),
        handle: ExternalSemaphoreHandleWin32,
    ) -> Self {
        Self(
            VkImportSemaphoreWin32HandleInfoKHR {
                sType: VkImportSemaphoreWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                semaphore: semaphore.native_ptr(),
                flags: 0,
                handleType: handle.0 as _,
                handle: handle.1,
                name: windows::core::PCWSTR::null(),
            },
            core::marker::PhantomData,
        )
    }

    #[inline]
    pub fn by_name(
        semaphore: &'d (impl VkHandle<Handle = VkSemaphore> + ?Sized),
        handle_type: ExternalSemaphoreHandleTypeWin32,
        name: &widestring::WideCStr,
    ) -> Self {
        Self(
            VkImportSemaphoreWin32HandleInfoKHR {
                sType: VkImportSemaphoreWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                semaphore: semaphore.native_ptr(),
                flags: 0,
                handleType: handle_type as _,
                handle: windows::Win32::Foundation::HANDLE(0),
                name: windows::core::PCWSTR(name.as_ptr()),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkImportSemaphoreWin32HandleInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkImportSemaphoreWin32HandleInfoKHR {
        self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct SemaphoreGetWin32HandleInfo<'d>(
    pub(crate) VkSemaphoreGetWin32HandleInfoKHR,
    core::marker::PhantomData<&'d dyn VkHandle<Handle = VkSemaphore>>,
);
impl<'d> SemaphoreGetWin32HandleInfo<'d> {
    #[inline]
    pub fn new(
        semaphore: &'d (impl VkHandle<Handle = VkSemaphore> + ?Sized),
        handle_type: ExternalSemaphoreHandleTypeWin32,
    ) -> Self {
        Self(
            VkSemaphoreGetWin32HandleInfoKHR {
                sType: VkSemaphoreGetWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                semaphore: semaphore.native_ptr(),
                handleType: handle_type as _,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkSemaphoreGetWin32HandleInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkSemaphoreGetWin32HandleInfoKHR {
        self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct D3D12FenceSubmitInfo<'t>(
    VkD3D12FenceSubmitInfoKHR,
    core::marker::PhantomData<(Option<&'t dyn VulkanStructure>, &'t [u64])>,
);
impl<'t> D3D12FenceSubmitInfo<'t> {
    pub const fn new(wait_semaphore_values: &'t [u64], signal_semaphore_values: &'t [u64]) -> Self {
        Self(
            VkD3D12FenceSubmitInfoKHR {
                sType: VkD3D12FenceSubmitInfoKHR::TYPE,
                pNext: core::ptr::null(),
                waitSemaphoreValuesCount: wait_semaphore_values.len() as _,
                pWaitSemaphoreValues: slice_as_ptr_empty_null(wait_semaphore_values),
                signalSemaphoreValuesCount: signal_semaphore_values.len() as _,
                pSignalSemaphoreValues: slice_as_ptr_empty_null(signal_semaphore_values),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkD3D12FenceSubmitInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkD3D12FenceSubmitInfoKHR {
        self.0
    }

    pub fn with_next(mut self, next: &'t (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}
unsafe impl<'t> VulkanStructure for D3D12FenceSubmitInfo<'t> {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        self.0.as_generic()
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        self.0.as_generic_mut()
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ExportSemaphoreWin32HandleInfo<'d>(
    VkExportSemaphoreWin32HandleInfoKHR,
    core::marker::PhantomData<(
        Option<&'d dyn VulkanStructure>,
        Option<&'d windows::Win32::Security::SECURITY_ATTRIBUTES>,
        &'d widestring::WideCStr,
    )>,
);
impl<'d> ExportSemaphoreWin32HandleInfo<'d> {
    pub const fn new(
        security_attributes: Option<&'d windows::Win32::Security::SECURITY_ATTRIBUTES>,
        access: u32,
        name: &'d widestring::WideCStr,
    ) -> Self {
        Self(
            VkExportSemaphoreWin32HandleInfoKHR {
                sType: VkExportSemaphoreWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                pAttributes: opt_pointer(security_attributes),
                dwAccess: access,
                name: windows::core::PCWSTR(name.as_ptr()),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkExportSemaphoreWin32HandleInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkExportSemaphoreWin32HandleInfoKHR {
        self.0
    }

    pub fn with_next(mut self, next: &'d (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}
unsafe impl VulkanStructure for ExportSemaphoreWin32HandleInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        self.0.as_generic()
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        self.0.as_generic_mut()
    }
}
