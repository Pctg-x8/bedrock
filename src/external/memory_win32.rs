use bedrock_vk::{self as brvk, TypedVulkanStructure};
use widestring::WideCStr;
use windows::Win32::Foundation::HANDLE;

use crate::error::translate_vk_result;
use crate::ffi_helper::opt_pointer;
use crate::*;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExternalMemoryHandleTypeWin32 {
    OpaqueWin32 = brvk::VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR,
    OpaqueWin32KMT = brvk::VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR,
    D3D11Texture = brvk::VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR,
    D3D11TextureKMT = brvk::VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR,
    D3D12Heap = brvk::VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR,
    D3D12Resource = brvk::VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR,
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ImportMemoryWin32HandleInfo<'d>(
    brvk::VkImportMemoryWin32HandleInfoKHR,
    core::marker::PhantomData<(Option<&'d dyn brvk::VulkanStructure>, Option<&'d WideCStr>)>,
);
unsafe impl brvk::VulkanStructure for ImportMemoryWin32HandleInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
impl<'d> ImportMemoryWin32HandleInfo<'d> {
    pub const fn new(
        handle_type: ExternalMemoryHandleTypeWin32,
        handle: windows::Win32::Foundation::HANDLE,
        name: Option<&'d WideCStr>,
    ) -> Self {
        Self(
            brvk::VkImportMemoryWin32HandleInfoKHR {
                sType: brvk::VkImportMemoryWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                handleType: handle_type as _,
                handle,
                name: match name {
                    Some(s) => windows::core::PCWSTR(s.as_ptr()),
                    None => windows::core::PCWSTR::null(),
                },
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkImportMemoryWin32HandleInfoKHR`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkImportMemoryWin32HandleInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkImportMemoryWin32HandleInfoKHR {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    /// # Safety
    ///
    /// `self` must be a valid value for type of T.
    pub const unsafe fn next_sink<T: brvk::VulkanStructure>(&mut self) -> &mut *const T {
        unsafe { core::mem::transmute(&mut self.0.pNext) }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ExportMemoryWin32HandleInfo<'d>(
    brvk::VkExportMemoryWin32HandleInfoKHR,
    core::marker::PhantomData<(Option<&'d dyn brvk::VulkanStructure>, &'d WideCStr)>,
);
unsafe impl brvk::VulkanStructure for ExportMemoryWin32HandleInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
impl<'d> ExportMemoryWin32HandleInfo<'d> {
    pub const fn new(
        security_attributes: Option<&windows::Win32::Security::SECURITY_ATTRIBUTES>,
        access: u32,
        name: &'d WideCStr,
    ) -> Self {
        Self(
            brvk::VkExportMemoryWin32HandleInfoKHR {
                sType: brvk::VkExportMemoryWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                pAttributes: opt_pointer(security_attributes),
                dwAccess: access,
                name: windows::core::PCWSTR(name.as_ptr()),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkExportMemoryWin32HandleInfoKHR`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkExportMemoryWin32HandleInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkExportMemoryWin32HandleInfoKHR {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    /// # Safety
    ///
    /// `self` must be a valid value for type of T.
    pub const unsafe fn next_sink<T: brvk::VulkanStructure>(&mut self) -> &mut *const T {
        unsafe { core::mem::transmute(&mut self.0.pNext) }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MemoryGetWin32HandleInfo<'d>(
    pub(crate) brvk::VkMemoryGetWin32HandleInfoKHR,
    core::marker::PhantomData<&'d dyn VkHandle<Handle = brvk::VkDeviceMemory>>,
);
unsafe impl brvk::VulkanStructure for MemoryGetWin32HandleInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
impl<'d> MemoryGetWin32HandleInfo<'d> {
    #[inline]
    pub fn new(
        memory: &'d (impl VkHandle<Handle = brvk::VkDeviceMemory> + ?Sized),
        handle_type: ExternalMemoryHandleTypeWin32,
    ) -> Self {
        Self(
            brvk::VkMemoryGetWin32HandleInfoKHR {
                sType: brvk::VkMemoryGetWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                memory: memory.native_ptr(),
                handleType: handle_type as _,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkMemoryGetWin32HandleInfoKHR`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkMemoryGetWin32HandleInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkMemoryGetWin32HandleInfoKHR {
        self.0
    }

    /// # Safety
    ///
    /// `self` must be a valid value for type of T.
    pub const unsafe fn next_sink<T: brvk::VulkanStructure>(&mut self) -> &mut *const T {
        unsafe { core::mem::transmute(&mut self.0.pNext) }
    }
}

pub trait DeviceExternalMemoryWin32Extension: Device {
    #[implements]
    fn get_memory_win32_handle_khr_fn(&self) -> brvk::PFN_vkGetMemoryWin32HandleKHR;
    #[implements]
    fn get_memory_win32_handle_properties_khr_fn(&self) -> brvk::PFN_vkGetMemoryWin32HandlePropertiesKHR;

    /// Get Properties of External Memory Win32 Handles
    /// # Safety
    /// sink must be constructed correctly
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_INVALID_EXTERNAL_HANDLE`
    #[implements]
    #[inline]
    unsafe fn memory_win32_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeWin32,
        handle: HANDLE,
        sink: &mut core::mem::MaybeUninit<brvk::VkMemoryWin32HandlePropertiesKHR>,
    ) -> crate::Result<()> {
        translate_vk_result(unsafe {
            self.get_memory_win32_handle_properties_khr_fn().0(
                self.native_ptr(),
                handle_type as _,
                handle,
                sink.as_mut_ptr(),
            )
        })?;

        Ok(())
    }

    /// Get a Windows HANDLE for a memory object
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_TOO_MANY_OBJECTS`
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements]
    #[inline]
    fn get_memory_win32_handle(&self, info: &MemoryGetWin32HandleInfo) -> crate::Result<HANDLE> {
        let mut handle = core::mem::MaybeUninit::uninit();

        translate_vk_result(unsafe {
            self.get_memory_win32_handle_khr_fn().0(self.native_ptr(), &info.0, handle.as_mut_ptr())
        })?;

        Ok(unsafe { handle.assume_init() })
    }
}
DerefContainerWithGuardsBracketImpl!(for DeviceExternalMemoryWin32Extension {
    #[implements]
    ForwardFnPtr!(deref get_memory_win32_handle_khr_fn -> brvk::PFN_vkGetMemoryWin32HandleKHR);
    #[implements]
    ForwardFnPtr!(deref get_memory_win32_handle_properties_khr_fn -> brvk::PFN_vkGetMemoryWin32HandlePropertiesKHR);
});
