//! External Memory Import/Export Operations

use crate::vk::*;
#[cfg(feature = "Implements")] use crate::{VkHandle, VkResultHandler, Resolver, ResolverInterface};

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[repr(C)]
pub enum ExternalSemaphoreHandleTypeWin32 {
    OpaqueWin32 = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT as _,
    OpaqueWin32KMT = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT as _,
    D3DFence = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT as _
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ExternalSemaphoreHandleTypes(pub VkExternalSemaphoreHandleTypeFlags);
impl From<ExternalSemaphoreHandleTypes> for VkExternalSemaphoreHandleTypeFlags { fn from(v: ExternalSemaphoreHandleTypes) -> Self { v.0 } }
impl ExternalSemaphoreHandleTypes {
    pub const EMPTY: Self = Self(0);
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    pub const OPAQUE_WIN32: Self = Self(VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT);
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    pub const OPAQUE_WIN32_KMT: Self = Self(VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT);
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    pub const D3D_FENCE: Self = Self(VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT);

    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    pub fn opaque_win32(self) -> Self { Self(self.0 | VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT) }
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    pub fn opaque_win32_kmt(self) -> Self { Self(self.0 | VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT) }
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    pub fn d3d_fence(self) -> Self { Self(self.0 | VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT) }
}

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub enum ExternalSemaphoreHandleWin32 {
    OpaqueWin32(winapi::shared::ntdef::HANDLE),
    OpaqueWin32KMT(winapi::shared::ntdef::HANDLE),
    D3DFence(winapi::shared::ntdef::HANDLE)
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl ExternalSemaphoreHandleWin32 {
    fn as_type_bits(&self) -> VkExternalSemaphoreHandleTypeFlags {
        match self {
            Self::OpaqueWin32(_) => VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT,
            Self::OpaqueWin32KMT(_) => VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
            // note: same value: VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT
            Self::D3DFence(_) => VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT
        }
    }
    fn handle(&self) -> winapi::shared::ntdef::HANDLE {
        match self {
            &Self::OpaqueWin32(h) | &Self::OpaqueWin32KMT(h) | &Self::D3DFence(h) => h
        }
    }
}

impl crate::Device {
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))]
    /// [Implements][VK_KHR_external_semaphore_win32] Import a semaphore from a Windows HANDLE
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    /// * VK_ERROR_INVALID_EXTERNAL_HANDLE
    pub fn import_semaphore_win32_handle(&self, target: &crate::Semaphore, handle: ExternalSemaphoreHandleWin32, name: &widestring::WideCString) -> crate::Result<()> {
        let info = VkImportSemaphoreWin32HandleInfoKHR {
            semaphore: target.native_ptr(),
            handleType: handle.as_type_bits(),
            handle: handle.handle(),
            name: name.as_ptr(),
            .. Default::default()
        };
        unsafe {
            Resolver::get().import_semaphore_win32_handle_khr(self.native_ptr(), &info).into_result()
        }
    }
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_semaphore_win32"))]
    /// [Implements][VK_KHR_external_semaphore_win32] Get a Windows HANDLE for a semaphore
    /// 
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * VK_ERROR_TOO_MANY_OBJECTS
    /// * VK_ERROR_OUT_OF_HOST_MEMORY
    pub fn get_semaphore_win32_handle(&self, target: &crate::Semaphore, handle_type: ExternalSemaphoreHandleTypeWin32) -> crate::Result<winapi::shared::ntdef::HANDLE> {
        let info = VkSemaphoreGetWin32HandleInfoKHR {
            semaphore: target.native_ptr(),
            handleType: handle_type as _,
            .. Default::default()
        };
        let mut h = std::ptr::null_mut();
        unsafe {
            Resolver::get().get_semaphore_win32_handle_khr(self.native_ptr(), &info, &mut h).into_result().map(move |_| h)
        }
    }
}

#[repr(transparent)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub struct D3D12FenceSubmitInfo<'t>(VkD3D12FenceSubmitInfoKHR, std::marker::PhantomData<&'t [u64]>);
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'t> From<D3D12FenceSubmitInfo<'t>> for VkD3D12FenceSubmitInfoKHR { fn from(v: D3D12FenceSubmitInfo) -> Self { v.0 } }
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'t> AsRef<VkD3D12FenceSubmitInfoKHR> for D3D12FenceSubmitInfo<'t> { fn as_ref(&self) -> &VkD3D12FenceSubmitInfoKHR { &self.0 } }
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'t> std::ops::Deref for D3D12FenceSubmitInfo<'t> {
    type Target = VkD3D12FenceSubmitInfoKHR;
    fn deref(&self) -> &Self::Target { &self.0 }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'t> D3D12FenceSubmitInfo<'t> {
    pub fn new(wait_semaphore_values: &'t [u64], signal_semaphore_values: &'t [u64]) -> Self {
        Self(VkD3D12FenceSubmitInfoKHR {
            waitSemaphoreValuesCount: wait_semaphore_values.len() as _,
            pWaitSemaphoreValues: wait_semaphore_values.as_ptr(),
            signalSemaphoreValuesCount: signal_semaphore_values.len() as _,
            pSignalSemaphoreValues: signal_semaphore_values.as_ptr(),
            .. Default::default()
        }, std::marker::PhantomData)
    }
    /// # Safety
    /// `pWaitSemaphoreValues` and `pSignalSemaphoreValues` must live in lifetime `'t`
    pub unsafe fn from_raw_structure(v: VkD3D12FenceSubmitInfoKHR) -> Self {
        Self(v, std::marker::PhantomData)
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'d> crate::Chainable<'d, D3D12FenceSubmitInfo<'d>> for crate::SubmissionBatch<'d> {
    fn chain(&mut self, next: &'d D3D12FenceSubmitInfo<'d>) { self.chained = Some(&next.0 as _); }
}

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[repr(transparent)]
pub struct ExportSemaphoreWin32HandleInfo<'d>(
    VkExportSemaphoreWin32HandleInfoKHR,
    std::marker::PhantomData<(Option<&'d winapi::um::minwinbase::SECURITY_ATTRIBUTES>, &'d widestring::WideCString)>
);
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'d> From<ExportSemaphoreWin32HandleInfo<'d>> for VkExportSemaphoreWin32HandleInfoKHR { fn from(v: ExportSemaphoreWin32HandleInfo<'d>) -> Self { v.0 } }
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl AsRef<VkExportSemaphoreWin32HandleInfoKHR> for ExportSemaphoreWin32HandleInfo<'_> { fn as_ref(&self) -> &VkExportSemaphoreWin32HandleInfoKHR { &self.0 } }
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl std::ops::Deref for ExportSemaphoreWin32HandleInfo<'_> {
    type Target = VkExportSemaphoreWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target { &self.0 }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'d> ExportSemaphoreWin32HandleInfo<'d> {
    pub fn new(security_attributes: Option<&'d winapi::um::minwinbase::SECURITY_ATTRIBUTES>, access: winapi::shared::minwindef::DWORD, name: &'d widestring::WideCString) -> Self {
        Self(VkExportSemaphoreWin32HandleInfoKHR {
            pAttributes: security_attributes.map_or_else(std::ptr::null, |x| x as *const _),
            dwAccess: access,
            name: name.as_ptr(),
            .. Default::default()
        }, std::marker::PhantomData)
    }
    /// # Safety
    /// `pAttributes` and `name` must live in lifetime `'d`
    pub unsafe fn from_raw_structure(v: VkExportSemaphoreWin32HandleInfoKHR) -> Self { Self(v, std::marker::PhantomData) }
}