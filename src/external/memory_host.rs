use crate::vk::*;
use crate::GenericVulkanStructure;
use crate::VulkanStructure;
use crate::VulkanStructureAsRef;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExternalMemoryHandleTypeHost {
    Allocation = VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT,
    MappedForeignMemory = VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportMemoryHostPointerInfo<'d>(
    VkImportMemoryHostPointerInfoEXT,
    core::marker::PhantomData<Option<&'d dyn VulkanStructureAsRef>>,
);
unsafe impl VulkanStructureAsRef for ImportMemoryHostPointerInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
impl<'d> ImportMemoryHostPointerInfo<'d> {
    pub const fn new(ty: ExternalMemoryHandleTypeHost, ptr: *mut core::ffi::c_void) -> Self {
        Self(
            VkImportMemoryHostPointerInfoEXT {
                sType: VkImportMemoryHostPointerInfoEXT::TYPE,
                pNext: core::ptr::null(),
                handleType: ty as _,
                pHostPointer: ptr,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkImportMemoryHostPointerInfoEXT) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkImportMemoryHostPointerInfoEXT {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl VulkanStructureAsRef + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }
}
