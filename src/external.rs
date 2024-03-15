//! External Memory Import/Export Operations

use cfg_if::cfg_if;
use derives::implements;

use crate::vk::*;
#[allow(unused_imports)]
use crate::VulkanStructure;
#[implements]
#[allow(unused_imports)]
use crate::{ffi_helper::ArrayFFIExtensions, DeviceChild, VkHandle};

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
impl From<ExternalMemoryHandleTypeWin32> for ExternalMemoryHandleTypes {
    fn from(value: ExternalMemoryHandleTypeWin32) -> Self {
        Self(value as _)
    }
}

#[cfg(feature = "VK_KHR_external_memory_fd")]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExternalMemoryHandleTypeFd {
    Opaque = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR as _,
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    DMABuf = VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT as _,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl ExternalMemoryHandleTypeFd {
    pub const fn with_fd(self, fd: std::os::unix::io::RawFd) -> ExternalMemoryHandleFd {
        ExternalMemoryHandleFd(self, fd)
    }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl From<ExternalMemoryHandleTypeFd> for ExternalMemoryHandleTypes {
    fn from(value: ExternalMemoryHandleTypeFd) -> Self {
        Self(value as _)
    }
}

#[cfg(feature = "VK_EXT_external_memory_host")]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExternalMemoryHandleType {
    HostAllocation = VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT as _,
    HostMappedForeignMemory = VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT as _,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl ExternalMemoryHandleType {
    pub const fn with_pointer(self, p: *const std::os::raw::c_void) -> ExternalMemoryHostPointer {
        ExternalMemoryHostPointer(self, p)
    }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl From<ExternalMemoryHandleType> for ExternalMemoryHandleTypes {
    fn from(value: ExternalMemoryHandleType) -> Self {
        Self(value as _)
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
        name: &widestring::WideCString,
    ) -> crate::DeviceMemoryRequest {
        crate::DeviceMemoryRequest::import(memory_type_index, self, name)
    }
}

#[cfg(feature = "VK_KHR_external_memory_fd")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ExternalMemoryHandleFd(pub ExternalMemoryHandleTypeFd, pub std::os::unix::io::RawFd);
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl ExternalMemoryHandleFd {
    /// Get Properties of External Memory File Descriptors
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
        mut sink: core::mem::MaybeUninit<VkMemoryFdPropertiesKHR>,
    ) -> crate::Result<VkMemoryFdPropertiesKHR> {
        device.get_memory_fd_properties_khr_fn().0(device.native_ptr(), self.0 as _, self.1, sink.as_mut_ptr())
            .into_result()
            .map(move |_| sink.assume_init())
    }

    pub fn into_import_request(self, memory_type_index: u32) -> crate::DeviceMemoryRequest {
        crate::DeviceMemoryRequest::import(memory_type_index, self)
    }
}

#[cfg(feature = "VK_EXT_external_memory_host")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ExternalMemoryHostPointer(pub ExternalMemoryHandleType, pub *const std::os::raw::c_void);
#[cfg(feature = "VK_EXT_external_memory_host")]
impl ExternalMemoryHostPointer {
    /// Get Properties of external memory host pointer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements]
    pub fn properties(
        &self,
        device: &(impl crate::Device + ?Sized),
        mut sink: core::mem::MaybeUninit<VkMemoryHostPointerPropertiesEXT>,
    ) -> crate::Result<VkMemoryHostPointerPropertiesEXT> {
        unsafe {
            device.get_memory_host_pointer_properties_ext_fn().0(
                device.native_ptr(),
                self.0 as _,
                self.1,
                sink.as_mut_ptr(),
            )
            .into_result()
            .map(move |_| sink.assume_init())
        }
    }
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
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub enum ExternalFenceFdType {
    Opaque = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR as _,
    Sync = VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR as _,
}

#[cfg(feature = "VK_KHR_external_fence_fd")]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalFenceFd(pub ExternalFenceFdType, pub std::os::unix::io::RawFd);
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl ExternalFenceFdType {
    pub const fn with_fd(self, fd: std::os::unix::io::RawFd) -> ExternalFenceFd {
        ExternalFenceFd(self, fd)
    }
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
