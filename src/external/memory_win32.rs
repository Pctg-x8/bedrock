use derives::transparent_marked;
use widestring::WideCStr;

use crate::{vk::*, VkHandle, VulkanStructure, VulkanStructureAsRef};

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
impl ExternalMemoryHandleTypeWin32 {
    pub const fn with_handle(self, handle: windows::Win32::Foundation::HANDLE) -> ExternalMemoryWin32Handle {
        ExternalMemoryWin32Handle(self, handle)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ExternalMemoryWin32Handle(
    pub ExternalMemoryHandleTypeWin32,
    pub windows::Win32::Foundation::HANDLE,
);
impl ExternalMemoryWin32Handle {
    pub const fn import_info<'d>(&self, name: Option<&'d WideCStr>) -> ImportMemoryWin32HandleInfo<'d> {
        ImportMemoryWin32HandleInfo::new(*self, name)
    }
}

#[transparent_marked]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportMemoryWin32HandleInfo<'d>(
    VkImportMemoryWin32HandleInfoKHR,
    core::marker::PhantomData<Option<&'d WideCStr>>,
);
impl<'d> ImportMemoryWin32HandleInfo<'d> {
    pub const fn new(handle: ExternalMemoryWin32Handle, name: Option<&'d WideCStr>) -> Self {
        Self(
            VkImportMemoryWin32HandleInfoKHR {
                sType: VkImportMemoryWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                handleType: handle.0 as _,
                handle: handle.1,
                name: match name {
                    Some(s) => windows::core::PCWSTR(s.as_ptr()),
                    None => windows::core::PCWSTR::null(),
                },
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkImportMemoryWin32HandleInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkImportMemoryWin32HandleInfoKHR {
        self.0
    }
}
unsafe impl VulkanStructureAsRef for ImportMemoryWin32HandleInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &crate::GenericVulkanStructure {
        self.0.as_generic()
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut crate::GenericVulkanStructure {
        self.0.as_generic_mut()
    }
}

#[transparent_marked]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryGetWin32HandleInfo<'d>(
    pub(crate) VkMemoryGetWin32HandleInfoKHR,
    core::marker::PhantomData<&'d dyn VkHandle<Handle = VkDeviceMemory>>,
);
impl<'d> MemoryGetWin32HandleInfo<'d> {
    #[inline]
    pub fn new(
        memory: &'d (impl VkHandle<Handle = VkDeviceMemory> + ?Sized),
        handle_type: ExternalMemoryHandleTypeWin32,
    ) -> Self {
        Self(
            VkMemoryGetWin32HandleInfoKHR {
                sType: VkMemoryGetWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                memory: memory.native_ptr(),
                handleType: handle_type as _,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkMemoryGetWin32HandleInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkMemoryGetWin32HandleInfoKHR {
        self.0
    }
}
