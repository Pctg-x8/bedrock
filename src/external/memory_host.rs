use crate::vk::*;
use crate::VulkanStructure;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExternalMemoryHandleTypeHost {
    Allocation = VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT as _,
    MappedForeignMemory = VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT as _,
}
impl ExternalMemoryHandleTypeHost {
    pub const fn with_pointer(self, p: *mut core::ffi::c_void) -> ExternalMemoryHostPointer {
        ExternalMemoryHostPointer(self, p)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ExternalMemoryHostPointer(pub ExternalMemoryHandleTypeHost, pub *mut core::ffi::c_void);
impl ExternalMemoryHostPointer {
    pub const fn import_info(&self) -> VkImportMemoryHostPointerInfoEXT {
        VkImportMemoryHostPointerInfoEXT {
            sType: VkImportMemoryHostPointerInfoEXT::TYPE,
            pNext: core::ptr::null(),
            handleType: self.0 as _,
            pHostPointer: self.1,
        }
    }
}
