use bedrock_vk::{self as brvk, TypedVulkanStructure};

use crate::*;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExternalMemoryHandleTypeHost {
    Allocation = brvk::VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT,
    MappedForeignMemory = brvk::VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportMemoryHostPointerInfo<'d>(
    brvk::VkImportMemoryHostPointerInfoEXT,
    core::marker::PhantomData<Option<&'d dyn brvk::VulkanStructure>>,
);
unsafe impl brvk::VulkanStructure for ImportMemoryHostPointerInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
impl<'d> ImportMemoryHostPointerInfo<'d> {
    pub const fn new(ty: ExternalMemoryHandleTypeHost, ptr: *mut core::ffi::c_void) -> Self {
        Self(
            brvk::VkImportMemoryHostPointerInfoEXT {
                sType: brvk::VkImportMemoryHostPointerInfoEXT::TYPE,
                pNext: core::ptr::null(),
                handleType: ty as _,
                pHostPointer: ptr,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: brvk::VkImportMemoryHostPointerInfoEXT) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkImportMemoryHostPointerInfoEXT {
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
