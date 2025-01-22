use crate::vk::*;
use crate::VkHandle;
use crate::VulkanStructure;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExternalFenceFdType {
    Opaque = VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR as _,
    Sync = VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR as _,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportFenceFdInfo<'d>(
    pub(crate) VkImportFenceFdInfoKHR,
    core::marker::PhantomData<&'d dyn VkHandle<Handle = VkFence>>,
);
impl<'d> ImportFenceFdInfo<'d> {
    pub fn new(
        fence: &'d (impl VkHandle<Handle = VkFence> + ?Sized),
        handle_type: ExternalFenceFdType,
        fd: std::os::unix::io::RawFd,
    ) -> Self {
        Self(
            VkImportFenceFdInfoKHR {
                sType: VkImportFenceFdInfoKHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                handleType: handle_type as _,
                fd,
                fence: fence.native_ptr(),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkImportFenceFdInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkImportFenceFdInfoKHR {
        self.0
    }

    pub const fn with_flags(mut self, flags: VkFenceImportFlagsKHR) -> Self {
        self.0.flags = flags;
        self
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenceFdGetInfo<'d>(
    pub(crate) VkFenceGetFdInfoKHR,
    core::marker::PhantomData<&'d dyn VkHandle<Handle = VkFence>>,
);
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl<'d> FenceFdGetInfo<'d> {
    pub fn new(fence: &'d (impl VkHandle<Handle = VkFence> + ?Sized), handle_type: ExternalFenceFdType) -> Self {
        Self(
            VkFenceGetFdInfoKHR {
                sType: VkFenceGetFdInfoKHR::TYPE,
                pNext: core::ptr::null(),
                fence: fence.native_ptr(),
                handleType: handle_type as _,
            },
            core::marker::PhantomData,
        )
    }
}
