//! External Memory Import/Export Operations

use cfg_if::cfg_if;
use derives::implements;

use crate::vk::*;
#[implements]
#[allow(unused_imports)]
use crate::{ffi_helper::ArrayFFIExtensions, DeviceChild, VkHandle};
#[allow(unused_imports)]
use crate::{VulkanStructure, VulkanStructureAsRef};

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[repr(C)]
pub enum ExternalSemaphoreHandleTypeWin32 {
    OpaqueWin32 = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR as _,
    OpaqueWin32KMT = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR as _,
    D3DFence = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR as _,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl ExternalSemaphoreHandleTypeWin32 {
    pub const fn with_handle(self, handle: windows::Win32::Foundation::HANDLE) -> ExternalSemaphoreHandleWin32 {
        ExternalSemaphoreHandleWin32(self, handle)
    }
}

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub struct ExternalSemaphoreHandleWin32(
    pub ExternalSemaphoreHandleTypeWin32,
    pub windows::Win32::Foundation::HANDLE,
);

cfg_if! {
    if #[cfg(feature = "VK_KHR_external_semaphore_win32")] {
        #[repr(transparent)]
        pub struct D3D12FenceSubmitInfo<'t>(VkD3D12FenceSubmitInfoKHR, std::marker::PhantomData<&'t [u64]>);
        impl<'t> From<D3D12FenceSubmitInfo<'t>> for VkD3D12FenceSubmitInfoKHR {
            fn from(v: D3D12FenceSubmitInfo) -> Self {
                v.0
            }
        }
        impl<'t> AsRef<VkD3D12FenceSubmitInfoKHR> for D3D12FenceSubmitInfo<'t> {
            fn as_ref(&self) -> &VkD3D12FenceSubmitInfoKHR {
                &self.0
            }
        }
        impl<'t> std::ops::Deref for D3D12FenceSubmitInfo<'t> {
            type Target = VkD3D12FenceSubmitInfoKHR;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<'t> D3D12FenceSubmitInfo<'t> {
            pub fn new(wait_semaphore_values: &'t [u64], signal_semaphore_values: &'t [u64]) -> Self {
                Self(
                    VkD3D12FenceSubmitInfoKHR {
                        sType: VkD3D12FenceSubmitInfoKHR::TYPE,
                        pNext: std::ptr::null(),
                        waitSemaphoreValuesCount: wait_semaphore_values.len() as _,
                        pWaitSemaphoreValues: wait_semaphore_values.as_ptr_empty_null(),
                        signalSemaphoreValuesCount: signal_semaphore_values.len() as _,
                        pSignalSemaphoreValues: signal_semaphore_values.as_ptr_empty_null(),
                    },
                    std::marker::PhantomData,
                )
            }

            /// # Safety
            /// `pWaitSemaphoreValues` and `pSignalSemaphoreValues` must live in lifetime `'t`
            pub unsafe fn from_raw_structure(v: VkD3D12FenceSubmitInfoKHR) -> Self {
                Self(v, std::marker::PhantomData)
            }
        }
        /* TODO: 連結リスト作るのなんかうまい方法考えないとねぇ
        impl<'d, Semaphore, CommandBuffer> crate::Chainable<'d, D3D12FenceSubmitInfo<'d>>
            for crate::SubmissionBatch<'d, Semaphore, CommandBuffer>
        where
            Semaphore: crate::Semaphore + Clone,
            CommandBuffer: crate::CommandBuffer + Clone,
        {
            fn chain(&mut self, next: &'d D3D12FenceSubmitInfo<'d>) -> &mut Self {
                self.chained = Some(&next.0 as _);
                self
            }
        }
        */

        #[repr(transparent)]
        pub struct ExportSemaphoreWin32HandleInfo<'d>(
            VkExportSemaphoreWin32HandleInfoKHR,
            std::marker::PhantomData<(
                Option<&'d windows::Win32::Security::SECURITY_ATTRIBUTES>,
                &'d widestring::WideCString,
            )>,
        );
        impl<'d> From<ExportSemaphoreWin32HandleInfo<'d>> for VkExportSemaphoreWin32HandleInfoKHR {
            fn from(v: ExportSemaphoreWin32HandleInfo<'d>) -> Self {
                v.0
            }
        }
        impl AsRef<VkExportSemaphoreWin32HandleInfoKHR> for ExportSemaphoreWin32HandleInfo<'_> {
            fn as_ref(&self) -> &VkExportSemaphoreWin32HandleInfoKHR {
                &self.0
            }
        }
        impl std::ops::Deref for ExportSemaphoreWin32HandleInfo<'_> {
            type Target = VkExportSemaphoreWin32HandleInfoKHR;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<'d> ExportSemaphoreWin32HandleInfo<'d> {
            pub fn new(
                security_attributes: Option<&'d windows::Win32::Security::SECURITY_ATTRIBUTES>,
                access: u32,
                name: &'d widestring::WideCString,
            ) -> Self {
                Self(
                    VkExportSemaphoreWin32HandleInfoKHR {
                        sType: VkExportSemaphoreWin32HandleInfoKHR::TYPE,
                        pNext: std::ptr::null(),
                        pAttributes: security_attributes.map_or_else(std::ptr::null, |x| x as *const _),
                        dwAccess: access,
                        name: windows::core::PCWSTR(name.as_ptr()),
                    },
                    std::marker::PhantomData,
                )
            }

            /// # Safety
            /// `pAttributes` and `name` must live in lifetime `'d`
            pub unsafe fn from_raw_structure(v: VkExportSemaphoreWin32HandleInfoKHR) -> Self {
                Self(v, std::marker::PhantomData)
            }
        }
        unsafe impl VulkanStructureAsRef for ExportSemaphoreWin32HandleInfo<'_> {
            fn as_generic(&self) -> &crate::GenericVulkanStructure {
                unsafe { core::mem::transmute(self) }
            }

            fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
                unsafe { core::mem::transmute(self) }
            }
        }
        unsafe impl VulkanStructure for ExportSemaphoreWin32HandleInfo<'_> {
            const TYPE: crate::vk::VkStructureType = <VkExportSemaphoreWin32HandleInfoKHR as VulkanStructure>::TYPE;
        }
    }
}

#[cfg(feature = "VK_KHR_external_memory_win32")]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExternalMemoryHandleTypeWin32 {
    OpaqueWin32 = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR as _,
    OpaqueWin32KMT = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR as _,
    D3D11Texture = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR as _,
    D3D11TextureKMT = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR as _,
    D3D12Heap = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR as _,
    D3D12Resource = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR as _,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
impl ExternalMemoryHandleTypeWin32 {
    pub const fn with_handle(self, handle: windows::Win32::Foundation::HANDLE) -> ExternalMemoryHandleWin32 {
        ExternalMemoryHandleWin32(self, handle)
    }
}

#[cfg(feature = "VK_KHR_external_memory_win32")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ExternalMemoryHandleWin32(
    pub ExternalMemoryHandleTypeWin32,
    pub windows::Win32::Foundation::HANDLE,
);
#[cfg(feature = "VK_KHR_external_memory_win32")]
impl ExternalMemoryHandleWin32 {
    /// Get Properties of External Memory Win32 Handles
    /// # Safety
    /// sink must be constructed correctly
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements]
    pub unsafe fn properties(
        &self,
        device: &(impl crate::Device + ?Sized),
        mut sink: core::mem::MaybeUninit<VkMemoryWin32HandlePropertiesKHR>,
    ) -> crate::Result<VkMemoryWin32HandlePropertiesKHR> {
        device.get_memory_win32_handle_properties_khr_fn().0(
            device.native_ptr(),
            self.0 as _,
            self.1,
            sink.as_mut_ptr(),
        )
        .into_result()
        .map(move |_| sink.assume_init())
    }

    pub fn into_import_request(
        self,
        memory_type_index: u32,
        name: Option<&widestring::WideCString>,
    ) -> crate::DeviceMemoryRequest {
        crate::DeviceMemoryRequest::import(memory_type_index, self, name)
    }
}

#[cfg(feature = "VK_KHR_external_memory")]
impl VkExternalMemoryImageCreateInfoKHR {
    pub const fn new(types: VkExternalMemoryHandleTypeFlagsKHR) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            handleTypes: types,
        }
    }
}

#[cfg(feature = "VK_KHR_external_memory_fd")]
mod memory_fd;
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub use self::memory_fd::*;

#[cfg(feature = "VK_EXT_external_memory_host")]
mod memory_host;
#[cfg(feature = "VK_EXT_external_memory_host")]
pub use self::memory_host::*;

#[cfg(feature = "VK_KHR_external_fence_fd")]
mod fence_fd;
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub use self::fence_fd::*;
