use crate::vk::*;
use crate::GenericVulkanStructure;
use crate::VkHandle;
use crate::VulkanStructure;
use crate::VulkanStructureAsRef;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ExternalMemoryHandleTypeFd {
    Opaque = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR,
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    DMABuf = VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportMemoryFdInfo<'d>(
    VkImportMemoryFdInfoKHR,
    core::marker::PhantomData<Option<&'d dyn VulkanStructureAsRef>>,
);
unsafe impl VulkanStructureAsRef for ImportMemoryFdInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
impl<'d> ImportMemoryFdInfo<'d> {
    pub const fn new(ty: ExternalMemoryHandleTypeFd, fd: std::os::unix::io::RawFd) -> Self {
        Self(
            VkImportMemoryFdInfoKHR {
                sType: VkImportMemoryFdInfoKHR::TYPE,
                pNext: core::ptr::null(),
                handleType: ty as _,
                fd,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkImportMemoryFdInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkImportMemoryFdInfoKHR {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl VulkanStructureAsRef + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    pub const unsafe fn next_sink<T: VulkanStructureAsRef>(&mut self) -> &mut *const T {
        core::mem::transmute(&mut self.0.pNext)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryGetFdInfo<'d>(
    pub(crate) VkMemoryGetFdInfoKHR,
    core::marker::PhantomData<&'d dyn VkHandle<Handle = VkDeviceMemory>>,
);
impl<'d> MemoryGetFdInfo<'d> {
    #[inline]
    pub fn new(
        memory: &'d (impl VkHandle<Handle = VkDeviceMemory> + ?Sized),
        handle_type: ExternalMemoryHandleTypeFd,
    ) -> Self {
        Self(
            VkMemoryGetFdInfoKHR {
                sType: VkMemoryGetFdInfoKHR::TYPE,
                pNext: core::ptr::null(),
                memory: memory.native_ptr(),
                handleType: handle_type as _,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkMemoryGetFdInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkMemoryGetFdInfoKHR {
        self.0
    }

    pub const unsafe fn next_sink<T: VulkanStructureAsRef>(&mut self) -> &mut *const T {
        core::mem::transmute(&mut self.0.pNext)
    }
}
