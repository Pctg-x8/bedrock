use derives::transparent_marked;

use crate::vk::*;
use crate::VkHandle;
use crate::VulkanStructure;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExternalMemoryHandleTypeFd {
    Opaque = VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR as _,
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    DMABuf = VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT as _,
}
impl ExternalMemoryHandleTypeFd {
    pub const fn with_fd(self, fd: std::os::unix::io::RawFd) -> ExternalMemoryHandleFd {
        ExternalMemoryHandleFd(self, fd)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ExternalMemoryHandleFd(pub ExternalMemoryHandleTypeFd, pub std::os::unix::io::RawFd);
impl ExternalMemoryHandleFd {
    pub const fn import_info(&self) -> VkImportMemoryFdInfoKHR {
        VkImportMemoryFdInfoKHR {
            sType: VkImportMemoryFdInfoKHR::TYPE,
            pNext: core::ptr::null(),
            handleType: self.0 as _,
            fd: self.1,
        }
    }
}

#[transparent_marked]
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
}
