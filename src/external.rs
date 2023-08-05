//! External Memory Import/Export Operations

use cfg_if::cfg_if;

use crate::vk::*;
use crate::VulkanStructure;
#[cfg(feature = "Implements")]
#[allow(unused_imports)]
use crate::{DeviceChild, VkHandle};

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[repr(C)]
pub enum ExternalSemaphoreHandleTypeWin32 {
    OpaqueWin32 = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT as _,
    OpaqueWin32KMT = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT as _,
    D3DFence = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT as _,
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_external_semaphore")] {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct ExternalSemaphoreHandleTypes(pub VkExternalSemaphoreHandleTypeFlagsKHR);
        impl From<ExternalSemaphoreHandleTypes> for VkExternalSemaphoreHandleTypeFlagsKHR {
            fn from(v: ExternalSemaphoreHandleTypes) -> Self {
                v.0
            }
        }
        impl ExternalSemaphoreHandleTypes {
            pub const EMPTY: Self = Self(0);
            pub const OPAQUE_WIN32: Self = Self(VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR);
            pub const OPAQUE_WIN32_KMT: Self = Self(VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR);
            pub const D3D_FENCE: Self = Self(VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR);

            pub fn opaque_win32(self) -> Self {
                Self(self.0 | Self::OPAQUE_WIN32.0)
            }
            pub fn opaque_win32_kmt(self) -> Self {
                Self(self.0 | Self::OPAQUE_WIN32_KMT.0)
            }
            pub fn d3d_fence(self) -> Self {
                Self(self.0 | Self::D3D_FENCE.0)
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_external_semaphore_win32")] {
        pub enum ExternalSemaphoreHandleWin32 {
            OpaqueWin32(windows::Win32::Foundation::HANDLE),
            OpaqueWin32KMT(windows::Win32::Foundation::HANDLE),
            D3DFence(windows::Win32::Foundation::HANDLE),
        }
        impl ExternalSemaphoreHandleWin32 {
            pub(crate) fn as_type_bits(&self) -> VkExternalSemaphoreHandleTypeFlags {
                match self {
                    Self::OpaqueWin32(_) => VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT,
                    Self::OpaqueWin32KMT(_) => VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
                    // note: same value: VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT
                    Self::D3DFence(_) => VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT,
                }
            }

            pub(crate) fn handle(&self) -> windows::Win32::Foundation::HANDLE {
                match self {
                    &Self::OpaqueWin32(h) | &Self::OpaqueWin32KMT(h) | &Self::D3DFence(h) => h,
                }
            }
        }

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
                        pWaitSemaphoreValues: wait_semaphore_values.as_ptr(),
                        signalSemaphoreValuesCount: signal_semaphore_values.len() as _,
                        pSignalSemaphoreValues: signal_semaphore_values.as_ptr(),
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
    }
}

#[cfg(feature = "VK_KHR_external_memory_win32")]
#[repr(C)]
pub enum ExternalMemoryHandleTypeWin32 {
    OpaqueWin32 = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR as _,
    OpaqueWin32KMT = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR as _,
    D3D11Texture = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR as _,
    D3D11TextureKMT = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR as _,
    D3D12Heap = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR as _,
    D3D12Resource = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR as _,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[repr(C)]
pub enum ExternalMemoryHandleTypeFd {
    Opaque = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR as _,
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    DMABuf = VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT as _,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[repr(C)]
pub enum ExternalMemoryHandleType {
    HostAllocation = VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT as _,
    HostMappedForeignMemory = VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT as _,
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_external_memory")] {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct ExternalMemoryHandleTypes(pub VkExternalMemoryHandleTypeFlagsKHR);
        impl From<ExternalMemoryHandleTypes> for VkExternalMemoryHandleTypeFlagsKHR {
            fn from(v: ExternalMemoryHandleTypes) -> Self {
                v.0
            }
        }
        impl ExternalMemoryHandleTypes {
            pub const EMPTY: Self = Self(0);
            #[cfg(feature = "VK_KHR_external_memory_fd")]
            pub const OPAQUE_FD: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR);
            pub const OPAQUE_WIN32: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR);
            pub const OPAQUE_WIN32_KMT: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR);
            pub const D3D11_TEXTURE: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR);
            pub const D3D11_TEXTURE_KMT: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR);
            pub const D3D12_HEAP: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR);
            pub const D3D12_RESOURCE: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR);
            #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
            pub const DMA_BUF: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT);
            #[cfg(feature = "VK_EXT_external_memory_host")]
            pub const HOST_ALLOCATION: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT);
            #[cfg(feature = "VK_EXT_external_memory_host")]
            pub const HOST_MAPPED_FOREIGN_MEMORY: Self =
                Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT);

            #[cfg(feature = "VK_KHR_external_memory_fd")]
            pub fn opaque_fd(self) -> Self {
                Self(self.0 | Self::OPAQUE_FD.0)
            }
            pub fn opaque_win32(self) -> Self {
                Self(self.0 | Self::OPAQUE_WIN32.0)
            }
            pub fn opaque_win32_kmt(self) -> Self {
                Self(self.0 | Self::OPAQUE_WIN32_KMT.0)
            }
            pub fn d3d11_texture(self) -> Self {
                Self(self.0 | Self::D3D11_TEXTURE.0)
            }
            pub fn d3d11_texture_kmt(self) -> Self {
                Self(self.0 | Self::D3D11_TEXTURE_KMT.0)
            }
            pub fn d3d12_heap(self) -> Self {
                Self(self.0 | Self::D3D12_HEAP.0)
            }
            pub fn d3d12_resource(self) -> Self {
                Self(self.0 | Self::D3D12_RESOURCE.0)
            }
            #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
            pub fn dma_buf(self) -> Self {
                Self(self.0 | Self::DMA_BUF.0)
            }
            #[cfg(feature = "VK_EXT_external_memory_host")]
            pub fn host_allocation(self) -> Self {
                Self(self.0 | Self::HOST_ALLOCATION.0)
            }
            #[cfg(feature = "VK_EXT_external_memory_host")]
            pub fn host_mapped_foreign_memory(self) -> Self {
                Self(self.0 | Self::HOST_MAPPED_FOREIGN_MEMORY.0)
            }
        }

        #[repr(transparent)]
        pub struct ExternalMemoryImageCreateInfo(VkExternalMemoryImageCreateInfoKHR);
        impl ExternalMemoryImageCreateInfo {
            pub fn new(handle_types: ExternalMemoryHandleTypes) -> Self {
                Self(VkExternalMemoryImageCreateInfoKHR {
                    sType: VkExternalMemoryImageCreateInfoKHR::TYPE,
                    pNext: std::ptr::null(),
                    handleTypes: handle_types.into(),
                })
            }
        }
        impl<'d> crate::Chainable<'d, ExternalMemoryImageCreateInfo> for crate::ImageDesc<'d> {
            fn chain(&mut self, next: &'d ExternalMemoryImageCreateInfo) -> &mut Self {
                self.0.pNext = next as *const _ as _;
                self
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub enum ExternalFenceFdType {
    Opaque = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR as _,
    Sync = VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR as _,
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_external_fence")] {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct ExternalFenceHandleTypes(pub VkExternalFenceHandleTypeFlagsKHR);
        impl From<ExternalFenceHandleTypes> for VkExternalFenceHandleTypeFlagsKHR {
            fn from(v: ExternalFenceHandleTypes) -> Self {
                v.0
            }
        }
        impl ExternalFenceHandleTypes {
            pub const EMPTY: Self = Self(0);
            #[cfg(feature = "VK_KHR_external_fence_fd")]
            pub const OPAQUE_FD: Self = Self(VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR as _);
            #[cfg(feature = "VK_KHR_external_fence_fd")]
            pub const SYNC_FD: Self = Self(VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR as _);

            #[cfg(feature = "VK_KHR_external_fence_fd")]
            pub const fn opaque_fd(self) -> Self {
                Self(self.0 | Self::OPAQUE_FD.0)
            }
            #[cfg(feature = "VK_KHR_external_fence_fd")]
            pub const fn sync_fd(self) -> Self {
                Self(self.0 | Self::SYNC_FD.0)
            }

            #[cfg(feature = "VK_KHR_external_fence_fd")]
            pub const fn contains_opaque_fd(self) -> bool {
                (self.0 & Self::OPAQUE_FD.0) != 0
            }
            #[cfg(feature = "VK_KHR_external_fence_fd")]
            pub const fn contains_sync_fd(self) -> bool {
                (self.0 & Self::SYNC_FD.0) != 0
            }
        }
        impl std::fmt::Debug for ExternalFenceHandleTypes {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_mut)]
                let mut bit_strings = Vec::<&'static str>::new();
                #[cfg(feature = "VK_KHR_external_fence_fd")]
                if self.contains_opaque_fd() {
                    bit_strings.push("OPAQUE_FD");
                }
                #[cfg(feature = "VK_KHR_external_fence_fd")]
                if self.contains_sync_fd() {
                    bit_strings.push("SYNC_FD");
                }

                write!(
                    fmt,
                    "ExternalFenceHandleTypes(0x{:08x}: {})",
                    self.0,
                    bit_strings.join("/")
                )
            }
        }
    }
}
