//! External Memory Import/Export Operations

use crate::vk::*;
#[cfg(feature = "Implements")]
#[allow(unused_imports)]
use crate::{DeviceChild, VkHandle, VkResultHandler};

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[repr(C)]
pub enum ExternalSemaphoreHandleTypeWin32 {
    OpaqueWin32 = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT as _,
    OpaqueWin32KMT = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT as _,
    D3DFence = VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT as _,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExternalSemaphoreHandleTypes(pub VkExternalSemaphoreHandleTypeFlags);
impl From<ExternalSemaphoreHandleTypes> for VkExternalSemaphoreHandleTypeFlags {
    fn from(v: ExternalSemaphoreHandleTypes) -> Self {
        v.0
    }
}
impl ExternalSemaphoreHandleTypes {
    pub const EMPTY: Self = Self(0);
    pub const OPAQUE_WIN32: Self = Self(VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT);
    pub const OPAQUE_WIN32_KMT: Self = Self(VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT);
    pub const D3D_FENCE: Self = Self(VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT);

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

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub enum ExternalSemaphoreHandleWin32 {
    OpaqueWin32(winapi::shared::ntdef::HANDLE),
    OpaqueWin32KMT(winapi::shared::ntdef::HANDLE),
    D3DFence(winapi::shared::ntdef::HANDLE),
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl ExternalSemaphoreHandleWin32 {
    fn as_type_bits(&self) -> VkExternalSemaphoreHandleTypeFlags {
        match self {
            Self::OpaqueWin32(_) => VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT,
            Self::OpaqueWin32KMT(_) => VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
            // note: same value: VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT
            Self::D3DFence(_) => VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT,
        }
    }
    fn handle(&self) -> winapi::shared::ntdef::HANDLE {
        match self {
            &Self::OpaqueWin32(h) | &Self::OpaqueWin32KMT(h) | &Self::D3DFence(h) => h,
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
    pub fn import_semaphore_win32_handle(
        &self,
        target: &crate::Semaphore,
        handle: ExternalSemaphoreHandleWin32,
        name: &widestring::WideCString,
    ) -> crate::Result<()> {
        let info = VkImportSemaphoreWin32HandleInfoKHR {
            semaphore: target.native_ptr(),
            handleType: handle.as_type_bits(),
            handle: handle.handle(),
            name: name.as_ptr(),
            ..Default::default()
        };

        let f = self
            .extra_procedure::<PFN_vkImportSemaphoreWin32HandleKHR>("vkImportSemaphoreWin32HandleKHR")
            .expect("No vkImportSemaphoreWin32HandleKHR exported");
        (f)(self.native_ptr(), &info).into_result()
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
    pub fn get_semaphore_win32_handle(
        &self,
        target: &crate::Semaphore,
        handle_type: ExternalSemaphoreHandleTypeWin32,
    ) -> crate::Result<winapi::shared::ntdef::HANDLE> {
        let info = VkSemaphoreGetWin32HandleInfoKHR {
            semaphore: target.native_ptr(),
            handleType: handle_type as _,
            ..Default::default()
        };
        let mut h = std::ptr::null_mut();

        let f = self
            .extra_procedure::<PFN_vkGetSemaphoreWin32HandleKHR>("vkGetSemaphoreWin32HandleKHR")
            .expect("No vkGetSemaphoreWin32HandleKHR exported");
        (f)(self.native_ptr(), &info, &mut h).into_result().map(move |_| h)
    }
}

#[repr(transparent)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub struct D3D12FenceSubmitInfo<'t>(VkD3D12FenceSubmitInfoKHR, std::marker::PhantomData<&'t [u64]>);
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'t> From<D3D12FenceSubmitInfo<'t>> for VkD3D12FenceSubmitInfoKHR {
    fn from(v: D3D12FenceSubmitInfo) -> Self {
        v.0
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'t> AsRef<VkD3D12FenceSubmitInfoKHR> for D3D12FenceSubmitInfo<'t> {
    fn as_ref(&self) -> &VkD3D12FenceSubmitInfoKHR {
        &self.0
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'t> std::ops::Deref for D3D12FenceSubmitInfo<'t> {
    type Target = VkD3D12FenceSubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'t> D3D12FenceSubmitInfo<'t> {
    pub fn new(wait_semaphore_values: &'t [u64], signal_semaphore_values: &'t [u64]) -> Self {
        Self(
            VkD3D12FenceSubmitInfoKHR {
                waitSemaphoreValuesCount: wait_semaphore_values.len() as _,
                pWaitSemaphoreValues: wait_semaphore_values.as_ptr(),
                signalSemaphoreValuesCount: signal_semaphore_values.len() as _,
                pSignalSemaphoreValues: signal_semaphore_values.as_ptr(),
                ..Default::default()
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
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'d> crate::Chainable<'d, D3D12FenceSubmitInfo<'d>> for crate::SubmissionBatch<'d> {
    fn chain(&mut self, next: &'d D3D12FenceSubmitInfo<'d>) -> &mut Self {
        self.chained = Some(&next.0 as _);
        self
    }
}

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[repr(transparent)]
pub struct ExportSemaphoreWin32HandleInfo<'d>(
    VkExportSemaphoreWin32HandleInfoKHR,
    std::marker::PhantomData<(
        Option<&'d winapi::um::minwinbase::SECURITY_ATTRIBUTES>,
        &'d widestring::WideCString,
    )>,
);
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'d> From<ExportSemaphoreWin32HandleInfo<'d>> for VkExportSemaphoreWin32HandleInfoKHR {
    fn from(v: ExportSemaphoreWin32HandleInfo<'d>) -> Self {
        v.0
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl AsRef<VkExportSemaphoreWin32HandleInfoKHR> for ExportSemaphoreWin32HandleInfo<'_> {
    fn as_ref(&self) -> &VkExportSemaphoreWin32HandleInfoKHR {
        &self.0
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl std::ops::Deref for ExportSemaphoreWin32HandleInfo<'_> {
    type Target = VkExportSemaphoreWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
impl<'d> ExportSemaphoreWin32HandleInfo<'d> {
    pub fn new(
        security_attributes: Option<&'d winapi::um::minwinbase::SECURITY_ATTRIBUTES>,
        access: winapi::shared::minwindef::DWORD,
        name: &'d widestring::WideCString,
    ) -> Self {
        Self(
            VkExportSemaphoreWin32HandleInfoKHR {
                pAttributes: security_attributes.map_or_else(std::ptr::null, |x| x as *const _),
                dwAccess: access,
                name: name.as_ptr(),
                ..Default::default()
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

#[cfg(feature = "VK_KHR_external_memory_win32")]
#[repr(C)]
pub enum ExternalMemoryHandleTypeWin32 {
    OpaqueWin32 = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT as _,
    OpaqueWin32KMT = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT as _,
    D3D11Texture = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT as _,
    D3D11TextureKMT = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT as _,
    D3D12Heap = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT as _,
    D3D12Resource = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT as _,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[repr(C)]
pub enum ExternalMemoryHandleTypeFd {
    Opaque = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT as _,
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    DMABuf = VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT as _,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[repr(C)]
pub enum ExternalMemoryHandleType {
    HostAllocation = VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT as _,
    HostMappedForeignMemory = VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT as _,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExternalMemoryHandleTypes(pub VkExternalMemoryHandleTypeFlags);
impl From<ExternalMemoryHandleTypes> for VkExternalMemoryHandleTypeFlags {
    fn from(v: ExternalMemoryHandleTypes) -> Self {
        v.0
    }
}
impl ExternalMemoryHandleTypes {
    pub const EMPTY: Self = Self(0);
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    pub const OPAQUE_FD: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT);
    pub const OPAQUE_WIN32: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT);
    pub const OPAQUE_WIN32_KMT: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT);
    pub const D3D11_TEXTURE: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT);
    pub const D3D11_TEXTURE_KMT: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT);
    pub const D3D12_HEAP: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT);
    pub const D3D12_RESOURCE: Self = Self(VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT);
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

impl crate::Device {
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))]
    /// [Implements][VK_KHR_external_memory_win32] Get a Windows HANDLE for a memory object
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    pub fn get_memory_win32_handle(
        &self,
        memory: &crate::DeviceMemory,
        handle_type: ExternalMemoryHandleTypeWin32,
    ) -> crate::Result<winapi::shared::ntdef::HANDLE> {
        let info = VkMemoryGetWin32HandleInfoKHR {
            memory: memory.native_ptr(),
            handleType: handle_type as _,
            ..Default::default()
        };
        let mut h = std::ptr::null_mut();

        let f = self
            .extra_procedure::<PFN_vkGetMemoryWin32HandleKHR>("vkGetMemoryWin32HandleKHR")
            .expect("No vkGetMemoryWin32HandleKHR exported");
        (f)(self.native_ptr(), &info, &mut h).into_result().map(move |_| h)
    }
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))]
    /// [Implements][VK_KHR_external_memory_fd] Get a POSIX file descriptor for a memory object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    pub fn get_memory_fd(
        &self,
        memory: &crate::DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFd,
    ) -> crate::Result<libc::c_int> {
        let info = VkMemoryGetFdInfoKHR {
            memory: memory.native_ptr(),
            handleType: handle_type as _,
            ..Default::default()
        };
        let mut fd = 0;

        let f = self
            .extra_procedure::<PFN_vkGetMemoryFdKHR>("vkGetMemoryFdKHR")
            .expect("No vkGetMemoryFdKHR exported");
        (f)(self.native_ptr(), &info, &mut fd).into_result().map(move |_| fd)
    }

    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_win32"))]
    /// [Implements][VK_KHR_external_memory_win32] Get Properties of External Memory Win32 Handles
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    pub fn get_memory_win32_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeWin32,
        handle: winapi::shared::ntdef::HANDLE,
    ) -> crate::Result<VkMemoryWin32HandlePropertiesKHR> {
        let mut info = Default::default();

        let f = self
            .extra_procedure::<PFN_vkGetMemoryWin32HandlePropertiesKHR>("vkGetMemoryWin32HandlePropertiesKHR")
            .expect("No vkGetMemoryWin32HandlePropertiesKHR exported");
        (f)(self.native_ptr(), handle_type as _, handle, &mut info)
            .into_result()
            .map(move |_| info)
    }
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_memory_fd"))]
    /// [Implements][VK_KHR_external_memory_fd] Get Properties of External Memory File Descriptors
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    pub fn get_memory_fd_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFd,
        fd: libc::c_int,
    ) -> crate::Result<VkMemoryFdPropertiesKHR> {
        let mut info = Default::default();

        let f = self
            .extra_procedure::<PFN_vkGetMemoryFdPropertiesKHR>("vkGetMemoryFdPropertiesKHR")
            .expect("No vkGetMemoryFdPropertiesKHR exported");
        (f)(self.native_ptr(), handle_type as _, fd, &mut info)
            .into_result()
            .map(move |_| info)
    }
    #[cfg(all(feature = "Implements", feature = "VK_EXT_external_memory_host"))]
    /// [Implements][VK_EXT_external_memory_host] Get Properties of external memory host pointer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    pub fn get_memory_host_pointer_properties(
        &self,
        handle_type: ExternalMemoryHandleType,
        host_pointer: *const (),
    ) -> crate::Result<VkMemoryHostPointerPropertiesEXT> {
        let mut info = Default::default();

        let f = self
            .extra_procedure::<PFN_vkGetMemoryHostPointerPropertiesEXT>("vkGetMemoryHostPointerPropertiesEXT")
            .expect("No vkGetMemoryHostPointerPropertiesEXT exported");
        (f)(self.native_ptr(), handle_type as _, host_pointer as _, &mut info)
            .into_result()
            .map(move |_| info)
    }
}

#[repr(transparent)]
pub struct ExternalMemoryImageCreateInfo(VkExternalMemoryImageCreateInfo);
impl ExternalMemoryImageCreateInfo {
    pub fn new(handle_types: ExternalMemoryHandleTypes) -> Self {
        Self(VkExternalMemoryImageCreateInfo {
            handleTypes: handle_types.into(),
            ..Default::default()
        })
    }
}
impl<'d> crate::Chainable<'d, ExternalMemoryImageCreateInfo> for crate::ImageDesc<'d> {
    fn chain(&mut self, next: &'d ExternalMemoryImageCreateInfo) -> &mut Self {
        self.0.pNext = next as *const _ as _;
        self
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub enum ExternalFenceFdType {
    Opaque = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT as _,
    Sync = VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT as _,
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ExternalFenceHandleTypes(pub VkExternalFenceHandleTypeFlags);
impl From<ExternalFenceHandleTypes> for VkExternalFenceHandleTypeFlags {
    fn from(v: ExternalFenceHandleTypes) -> Self {
        v.0
    }
}
impl ExternalFenceHandleTypes {
    pub const EMPTY: Self = Self(0);
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    pub const OPAQUE_FD: Self = Self(VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT as _);
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    pub const SYNC_FD: Self = Self(VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT as _);

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

impl<Device: VkHandle<Handle = VkDevice>> crate::Fence<Device> {
    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
    /// [Implements][VK_KHR_external_fence_fd] Get a POSIX file descriptor handle for a type
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    pub fn get_fd(&self, ty: ExternalFenceFdType) -> crate::Result<std::os::unix::io::RawFd> {
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

    #[cfg(all(feature = "Implements", feature = "VK_KHR_external_fence_fd"))]
    /// [Implements][VK_KHR_external_fence_fd] Import a fence from a POSIX file descriptor
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    pub fn import(&self, ty: ExternalFenceFdType, fd: std::os::unix::io::RawFd, temporary: bool) -> crate::Result<()> {
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
