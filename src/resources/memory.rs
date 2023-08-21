#[implements]
use crate::VkHandleMut;
use crate::{vk::*, DeviceChild, GenericVulkanStructure, VkHandle, VulkanStructure};
use derives::implements;
#[implements]
use std::ops::Range;

pub trait DeviceMemory: VkHandle<Handle = VkDeviceMemory> + DeviceChild {
    /// Map a memory object into application address space
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_MEMORY_MAP_FAILED`
    #[implements]
    fn map(&mut self, range: Range<usize>) -> crate::Result<MappedMemoryRange<Self>>
    where
        Self: VkHandleMut,
    {
        let mut p = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::map_memory(
                self.device().native_ptr(),
                self.native_ptr_mut(),
                range.start as _,
                (range.end - range.start) as _,
                0,
                p.as_mut_ptr(),
            )
            .into_result()
            .map(move |_| MappedMemoryRange(range, p.assume_init() as *mut _, self))
        }
    }

    /// Unmap a previously mapped memory object
    /// # Safety
    /// Caller must guarantee that there is no `MappedMemoryRange` alives.  
    /// Accessing the mapped memory after this call has undefined behavior
    #[implements]
    unsafe fn unmap(&mut self)
    where
        Self: VkHandleMut,
    {
        crate::vkresolve::unmap_memory(self.device().native_ptr(), self.native_ptr_mut());
    }

    /// Query the current commitment for a `DeviceMemory`
    #[implements]
    fn commitment_bytes(&self) -> VkDeviceSize {
        let mut b = 0;
        unsafe {
            crate::vkresolve::get_device_memory_commitment(self.device().native_ptr(), self.native_ptr(), &mut b);
        }

        b
    }

    /// Get a Windows HANDLE for a memory object
    ///
    /// A returned handle needs to be closed by caller
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements]
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    fn get_win32_handle(
        &self,
        handle_type: crate::ExternalMemoryHandleTypeWin32,
    ) -> crate::Result<windows::Win32::Foundation::HANDLE> {
        use crate::Device;

        let info = VkMemoryGetWin32HandleInfoKHR {
            sType: VkMemoryGetWin32HandleInfoKHR::TYPE,
            pNext: std::ptr::null(),
            memory: self.native_ptr(),
            handleType: handle_type as _,
        };
        let mut h = windows::Win32::Foundation::HANDLE(0);

        unsafe {
            crate::VkResultBox(self.device().get_memory_win32_handle_khr_fn().0(
                self.device().native_ptr(),
                &info,
                &mut h,
            ))
            .into_result()
            .map(move |_| h)
        }
    }
}
DerefContainerBracketImpl!(for DeviceMemory {});
GuardsImpl!(for DeviceMemory {});

DefineStdDeviceChildObject! {
    /// Opaque handle to a device memory object
    DeviceMemoryObject(VkDeviceMemory): DeviceMemory { drop free_memory }
}

pub struct DeviceMemoryRequest(VkMemoryAllocateInfo, Vec<Box<GenericVulkanStructure>>);
impl DeviceMemoryRequest {
    pub const fn allocate(size: usize, memory_type_index: u32) -> Self {
        Self(
            VkMemoryAllocateInfo {
                sType: VkMemoryAllocateInfo::TYPE,
                pNext: std::ptr::null(),
                allocationSize: size as _,
                memoryTypeIndex: memory_type_index,
            },
            Vec::new(),
        )
    }

    #[cfg(feature = "VK_KHR_external_memory_win32")]
    pub fn import(
        memory_type_index: u32,
        handle: crate::ExternalMemoryHandleWin32,
        name: &widestring::WideCString,
    ) -> Self {
        unsafe {
            // Note: size is ignored by specification(but 0 is not allowed by validation layer...)
            Self::allocate(1, memory_type_index).with_additional_info(VkImportMemoryWin32HandleInfoKHR {
                sType: VkImportMemoryWin32HandleInfoKHR::TYPE,
                pNext: std::ptr::null(),
                handleType: handle.0 as _,
                handle: handle.1,
                name: windows::core::PCWSTR::from_raw(name.as_ptr()),
            })
        }
    }

    #[cfg(feature = "VK_EXT_external_memory_host")]
    #[implements]
    pub fn import_host_pointer(
        memory_type_index: u32,
        handle_type: crate::ExternalMemoryHandleType,
        host_pointer: *mut std::os::raw::c_void,
    ) -> Self {
        unsafe {
            // Note: size is ignored by specification(but 0 is not allowed by validation layer...)
            Self::allocate(1, memory_type_index).with_additional_info(VkImportMemoryHostPointerInfoEXT {
                sType: VkImportMemoryHostPointerInfoEXT::TYPE,
                pNext: std::ptr::null(),
                handleType: handle_type as _,
                pHostPointer: host_pointer,
            })
        }
    }

    #[cfg(feature = "VK_KHR_external_memory_win32")]
    pub fn and_export(
        self,
        security_attributes: Option<&windows::Win32::Security::SECURITY_ATTRIBUTES>,
        access: u32,
        name: &widestring::WideCString,
    ) -> Self {
        unsafe {
            self.with_additional_info(VkExportMemoryWin32HandleInfoKHR {
                sType: VkExportMemoryWin32HandleInfoKHR::TYPE,
                pNext: std::ptr::null(),
                pAttributes: security_attributes.map_or_else(std::ptr::null, |v| v as *const _),
                dwAccess: access,
                name: windows::core::PCWSTR(name.as_ptr()),
            })
        }
    }

    pub unsafe fn with_additional_info(mut self, x: impl VulkanStructure) -> Self {
        self.1.push(core::mem::transmute(Box::new(x)));
        self
    }

    /// Execute requests for Device Memory Acquisition
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    #[implements]
    pub fn execute<Device: crate::Device>(mut self, device: Device) -> crate::Result<DeviceMemoryObject<Device>>
    where
        Self: Sized,
    {
        crate::ext::chain(&mut self.0, self.1.iter_mut().map(|x| &mut **x));

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::allocate_memory(device.native_ptr(), &self.0, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| DeviceMemoryObject(h.assume_init(), device))
        }
    }
}

/// Specifies the block of mapped memory in a `DeviceMemory`
pub struct MappedMemoryRange<'m, DeviceMemory: crate::DeviceMemory + VkHandleMut + ?Sized + 'm>(
    std::ops::Range<usize>,
    *mut u8,
    &'m mut DeviceMemory,
);
#[allow(clippy::mut_from_ref)]
impl<'m, DeviceMemory: crate::DeviceMemory + VkHandleMut + ?Sized + 'm> MappedMemoryRange<'m, DeviceMemory> {
    /// Get a reference in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn get<T>(&self, offset: usize) -> &T {
        &*(self.1.add(offset) as *const T)
    }
    /// Get a mutable reference in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub unsafe fn get_mut<T>(&self, offset: usize) -> &mut T {
        &mut *(self.1.add(offset) as *mut T)
    }

    /// Get a slice in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn slice<T>(&self, offset: usize, count: usize) -> &[T] {
        std::slice::from_raw_parts(self.1.add(offset) as *const T, count)
    }
    /// Get a mutable slice in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub unsafe fn slice_mut<T>(&self, offset: usize, count: usize) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.1.add(offset) as *mut T, count)
    }

    /// Clone data from slice at the specified offset in mapped memory.
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub unsafe fn clone_from_slice_at<T: Clone>(&self, offset: usize, src: &[T]) {
        self.slice_mut(offset, src.len()).clone_from_slice(src);
    }
    /// Clone data from slice at the specified offset in mapped memory.
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub unsafe fn clone_at<T: Clone>(&self, offset: usize, src: &T) {
        *self.get_mut(offset) = src.clone();
    }

    #[implements]
    /// [feature = "Implements"] Unmap region
    pub fn end(self) {
        unsafe {
            self.2.unmap();
        }
    }
}
