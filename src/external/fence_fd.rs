use bedrock_vk::{self as brvk, TypedVulkanStructure};

use crate::*;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExternalFenceFdType {
    Opaque = brvk::VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR,
    Sync = brvk::VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ImportFenceFdInfo<'d>(
    pub(crate) brvk::VkImportFenceFdInfoKHR,
    core::marker::PhantomData<&'d dyn VkHandle<Handle = brvk::VkFence>>,
);
impl<'d> ImportFenceFdInfo<'d> {
    pub fn new(
        fence: &'d (impl brvk::VkHandle<Handle = brvk::VkFence> + ?Sized),
        handle_type: ExternalFenceFdType,
        fd: std::os::unix::io::RawFd,
    ) -> Self {
        Self(
            brvk::VkImportFenceFdInfoKHR {
                sType: brvk::VkImportFenceFdInfoKHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                handleType: handle_type as _,
                fd,
                fence: fence.native_ptr(),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: brvk::VkImportFenceFdInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkImportFenceFdInfoKHR {
        self.0
    }

    pub const fn with_flags(mut self, flags: brvk::VkFenceImportFlagsKHR) -> Self {
        self.0.flags = flags;
        self
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct FenceFdGetInfo<'d>(
    pub(crate) brvk::VkFenceGetFdInfoKHR,
    core::marker::PhantomData<&'d dyn VkHandle<Handle = brvk::VkFence>>,
);
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl<'d> FenceFdGetInfo<'d> {
    pub fn new(fence: &'d (impl VkHandle<Handle = brvk::VkFence> + ?Sized), handle_type: ExternalFenceFdType) -> Self {
        Self(
            brvk::VkFenceGetFdInfoKHR {
                sType: brvk::VkFenceGetFdInfoKHR::TYPE,
                pNext: core::ptr::null(),
                fence: fence.native_ptr(),
                handleType: handle_type as _,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: brvk::VkFenceGetFdInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkFenceGetFdInfoKHR {
        self.0
    }
}
