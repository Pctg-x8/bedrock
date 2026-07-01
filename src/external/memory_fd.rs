use bedrock_vk::{self as brvk, TypedVulkanStructure};

use crate::*;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ExternalMemoryHandleTypeFd {
    Opaque = brvk::VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR,
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    DMABuf = brvk::VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ImportMemoryFdInfo<'d>(
    brvk::VkImportMemoryFdInfoKHR,
    core::marker::PhantomData<Option<&'d dyn brvk::VulkanStructure>>,
);
unsafe impl brvk::VulkanStructure for ImportMemoryFdInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
impl<'d> ImportMemoryFdInfo<'d> {
    pub const fn new(ty: ExternalMemoryHandleTypeFd, fd: std::os::unix::io::RawFd) -> Self {
        Self(
            brvk::VkImportMemoryFdInfoKHR {
                sType: brvk::VkImportMemoryFdInfoKHR::TYPE,
                pNext: core::ptr::null(),
                handleType: ty as _,
                fd,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: brvk::VkImportMemoryFdInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkImportMemoryFdInfoKHR {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    pub const unsafe fn next_sink<T: brvk::VulkanStructure>(&mut self) -> &mut *const T {
        unsafe { core::mem::transmute(&mut self.0.pNext) }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct MemoryGetFdInfo<'d>(
    pub(crate) brvk::VkMemoryGetFdInfoKHR,
    core::marker::PhantomData<&'d dyn VkHandle<Handle = brvk::VkDeviceMemory>>,
);
impl<'d> MemoryGetFdInfo<'d> {
    #[inline]
    pub fn new(
        memory: &'d (impl VkHandle<Handle = brvk::VkDeviceMemory> + ?Sized),
        handle_type: ExternalMemoryHandleTypeFd,
    ) -> Self {
        Self(
            brvk::VkMemoryGetFdInfoKHR {
                sType: brvk::VkMemoryGetFdInfoKHR::TYPE,
                pNext: core::ptr::null(),
                memory: memory.native_ptr(),
                handleType: handle_type as _,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: brvk::VkMemoryGetFdInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkMemoryGetFdInfoKHR {
        self.0
    }

    pub const unsafe fn next_sink<T: brvk::VulkanStructure>(&mut self) -> &mut *const T {
        unsafe { core::mem::transmute(&mut self.0.pNext) }
    }
}
