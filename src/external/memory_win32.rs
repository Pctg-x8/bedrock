use widestring::WideCStr;

use crate::{vk::*, VkHandle, VulkanStructure, VulkanStructureAsRef};

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExternalMemoryHandleTypeWin32 {
    OpaqueWin32 = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR,
    OpaqueWin32KMT = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR,
    D3D11Texture = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR,
    D3D11TextureKMT = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR,
    D3D12Heap = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR,
    D3D12Resource = VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportMemoryWin32HandleInfo<'d>(
    VkImportMemoryWin32HandleInfoKHR,
    core::marker::PhantomData<(Option<&'d dyn VulkanStructureAsRef>, Option<&'d WideCStr>)>,
);
unsafe impl VulkanStructureAsRef for ImportMemoryWin32HandleInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
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
            VkImportMemoryWin32HandleInfoKHR {
                sType: VkImportMemoryWin32HandleInfoKHR::TYPE,
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

    pub const unsafe fn from_raw(raw: VkImportMemoryWin32HandleInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkImportMemoryWin32HandleInfoKHR {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl VulkanStructureAsRef + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportMemoryWin32HandleInfo<'d>(
    VkExportMemoryWin32HandleInfoKHR,
    core::marker::PhantomData<(Option<&'d dyn VulkanStructureAsRef>, &'d WideCStr)>,
);
impl VulkanStructureAsRef for ExportMemoryWin32HandleInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
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
            VkExportMemoryWin32HandleInfoKHR {
                sType: VkExportMemoryWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                pAttributes: opt_pointer(security_attributes),
                dwAccess: access,
                name: windows::core::PCWSTR(name.as_ptr()),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkExportMemoryWin32HandleInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkExportMemoryWin32HandleInfoKHR {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl VulkanStructureAsRef + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }
}

#[repr(transparent)]
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
