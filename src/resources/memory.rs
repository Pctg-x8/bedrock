use crate::{
    ffi_helper::opt_pointer, vk::*, DeviceChild, DeviceChildHandle, GenericVulkanStructure,
    VkDeviceChildNonExtDestroyable, VkHandle, VkHandleMut, VkObject, VkRawHandle, VulkanStructure,
    VulkanStructureAsRef,
};
use derives::implements;

pub trait DeviceMemory: VkHandle<Handle = VkDeviceMemory> + DeviceChildHandle {
    /// Query the current commitment for a `DeviceMemory`
    #[implements]
    #[inline]
    fn commitment_bytes(&self) -> VkDeviceSize {
        let mut b = 0;
        unsafe {
            crate::vkfn::get_device_memory_commitment(self.device_handle(), self.native_ptr(), &mut b);
        }

        b
    }
}
DerefContainerBracketImpl!(for DeviceMemory {});
GuardsImpl!(for DeviceMemory {});

pub trait DeviceMemoryMut: DeviceMemory + VkHandleMut {
    /// Map a memory object into application address space
    /// # Safety
    /// mapped memory must be unmapped once
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_MEMORY_MAP_FAILED`
    #[implements]
    #[inline]
    unsafe fn map_raw(&mut self, range: core::ops::Range<VkDeviceSize>) -> crate::Result<*mut core::ffi::c_void> {
        let mut p = core::mem::MaybeUninit::uninit();

        crate::vkfn::map_memory(
            self.device_handle(),
            self.native_ptr_mut(),
            range.start,
            range.end - range.start,
            0,
            p.as_mut_ptr(),
        )
        .into_result()?;

        Ok(p.assume_init())
    }

    /// Map a memory object into application address space
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_MEMORY_MAP_FAILED`
    #[implements]
    fn map(&mut self, range: core::ops::Range<usize>) -> crate::Result<MappedMemoryRange<Self>> {
        let p = unsafe { self.map_raw(range.start as _..range.end as _)? };

        Ok(MappedMemoryRange(p, self))
    }

    /// Unmap a previously mapped memory object
    /// # Safety
    /// Caller must guarantee that there is no `MappedMemoryRange` alives.
    /// Accessing the mapped memory after this call has undefined behavior
    #[implements]
    #[inline]
    unsafe fn unmap(&mut self) {
        crate::vkfn::unmap_memory(self.device_handle(), self.native_ptr_mut());
    }
}
DerefContainerBracketImpl!(for mut DeviceMemoryMut {});
GuardsImpl!(for mut DeviceMemoryMut {});

#[derive(VkHandle, VkObject)]
#[VkObject(type = VkDeviceMemory::OBJECT_TYPE)]
pub struct DeviceMemoryObject<Device: VkHandle<Handle = VkDevice>>(VkDeviceMemory, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for DeviceMemoryObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for DeviceMemoryObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for DeviceMemoryObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for DeviceMemoryObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for DeviceMemoryObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: VkHandle<Handle = VkDevice>> DeviceMemory for DeviceMemoryObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceMemoryMut for DeviceMemoryObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceMemoryObject<Device> {
    /// Execute requests for Device Memory Acquisition
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    ///
    /// # Safety
    /// no guarantee will be provided (simply calls the under api)
    #[implements]
    #[inline]
    pub unsafe fn new_raw(device: Device, info: &VkMemoryAllocateInfo) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::allocate_memory(device.native_ptr(), info, core::ptr::null(), h.as_mut_ptr()).into_result()?;

        Ok(Self(h.assume_init(), device))
    }

    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the parent
    pub const unsafe fn manage(handle: VkDeviceMemory, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkDeviceMemory, Device) {
        let v = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (v, p)
    }
}
impl<Device: VkHandle<Handle = VkDevice> + Clone> DeviceMemoryObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> DeviceMemoryObject<Device> {
        DeviceMemoryObject(self.0, self.1.clone())
    }
}

pub struct DeviceMemoryRequest(VkMemoryAllocateInfo, Vec<Box<GenericVulkanStructure>>);
impl DeviceMemoryRequest {
    pub const fn allocate(size: usize, memory_type_index: u32) -> Self {
        Self(
            VkMemoryAllocateInfo {
                sType: VkMemoryAllocateInfo::TYPE,
                pNext: core::ptr::null(),
                allocationSize: size as _,
                memoryTypeIndex: memory_type_index,
            },
            Vec::new(),
        )
    }

    pub unsafe fn with_extension(mut self, x: impl VulkanStructureAsRef) -> Self {
        self.1.push(core::mem::transmute(Box::new(x)));
        self
    }

    #[cfg(feature = "VK_KHR_external_memory_win32")]
    pub fn import(
        memory_type_index: u32,
        handle: crate::ExternalMemoryWin32Handle,
        name: Option<&widestring::WideCStr>,
    ) -> Self {
        unsafe {
            // Note: size is ignored by specification(but 0 is not allowed by validation layer...)
            Self::allocate(1, memory_type_index).with_extension(handle.import_info(name))
        }
    }

    #[cfg(feature = "VK_KHR_external_memory_fd")]
    pub fn import(memory_type_index: u32, handle: crate::ExternalMemoryHandleFd) -> Self {
        unsafe { Self::allocate(1, memory_type_index).with_extension(handle.import_info()) }
    }

    #[cfg(feature = "VK_EXT_external_memory_host")]
    #[implements]
    pub fn import_host_pointer(memory_type_index: u32, ptr: crate::ExternalMemoryHostPointer) -> Self {
        unsafe {
            // Note: size is ignored by specification(but 0 is not allowed by validation layer...)
            Self::allocate(1, memory_type_index).with_extension(ptr.import_info())
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
            self.with_extension(VkExportMemoryWin32HandleInfoKHR {
                sType: VkExportMemoryWin32HandleInfoKHR::TYPE,
                pNext: core::ptr::null(),
                pAttributes: security_attributes.map_or_else(core::ptr::null, |v| v as *const _),
                dwAccess: access,
                name: windows::core::PCWSTR(name.as_ptr()),
            })
        }
    }

    /// Adds dedicated allocation info for a buffer
    /// # Safety
    /// lifetime not captured
    #[cfg(feature = "VK_KHR_dedicated_allocation")]
    pub unsafe fn for_dedicated_buffer_allocation(self, buffer: &(impl VkHandle<Handle = VkBuffer> + ?Sized)) -> Self {
        use crate::VkRawHandle;

        self.with_extension(VkMemoryDedicatedAllocateInfoKHR {
            sType: VkMemoryDedicatedAllocateInfoKHR::TYPE,
            pNext: core::ptr::null(),
            image: VkImage::NULL,
            buffer: buffer.native_ptr(),
        })
    }

    /// Adds dedicated allocation info for an image
    /// # Safety
    /// lifetime not captured
    #[cfg(feature = "VK_KHR_dedicated_allocation")]
    pub unsafe fn for_dedicated_image_allocation(self, image: &(impl VkHandle<Handle = VkImage> + ?Sized)) -> Self {
        use crate::VkRawHandle;

        self.with_extension(VkMemoryDedicatedAllocateInfoKHR {
            sType: VkMemoryDedicatedAllocateInfoKHR::TYPE,
            pNext: core::ptr::null(),
            image: image.native_ptr(),
            buffer: VkBuffer::NULL,
        })
    }

    /// Execute requests for Device Memory Acquisition
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_TOO_MANY_OBJECTS`]
    #[implements]
    #[inline]
    pub fn execute_raw(
        mut self,
        device: &(impl VkHandle<Handle = VkDevice> + ?Sized),
        allocator: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkDeviceMemory> {
        crate::ext::chain(&mut self.0, self.1.iter_mut().map(|x| &mut **x));

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::allocate_memory(device.native_ptr(), &self.0, opt_pointer(allocator), h.as_mut_ptr())
                .into_result()?;
        }

        Ok(unsafe { h.assume_init() })
    }

    /// Execute requests for Device Memory Acquisition
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    #[implements]
    pub fn execute<Device: crate::Device>(mut self, device: Device) -> crate::Result<DeviceMemoryObject<Device>> {
        crate::ext::chain(&mut self.0, self.1.iter_mut().map(|x| &mut **x));

        unsafe { DeviceMemoryObject::new_raw(device, &self.0) }
    }
}

/// Specifies the block of mapped memory in a `DeviceMemory`
pub struct MappedMemoryRange<'m, DeviceMemory: crate::DeviceMemoryMut + ?Sized + 'm>(
    *mut core::ffi::c_void,
    &'m mut DeviceMemory,
);
#[allow(clippy::mut_from_ref)]
impl<'m, DeviceMemory: crate::DeviceMemoryMut + ?Sized + 'm> MappedMemoryRange<'m, DeviceMemory> {
    /// Returns a pointer to the head of the mapped region
    pub const fn ptr(&self) -> *mut core::ffi::c_void {
        self.0
    }

    /// Get a reference in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn get<T>(&self, offset: usize) -> &T {
        &*(self.0.add(offset) as *const T)
    }

    /// Get a mutable reference in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn get_mut<T>(&self, offset: usize) -> &mut T {
        &mut *(self.0.add(offset) as *mut T)
    }

    /// Get a slice in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn slice<T>(&self, offset: usize, count: usize) -> &[T] {
        core::slice::from_raw_parts(self.0.add(offset) as *const T, count)
    }

    /// Get a mutable slice in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn slice_mut<T>(&self, offset: usize, count: usize) -> &mut [T] {
        core::slice::from_raw_parts_mut(self.0.add(offset) as *mut T, count)
    }

    /// Clone data from slice at the specified offset in mapped memory.
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    #[inline(always)]
    pub unsafe fn clone_from_slice_at<T: Clone>(&self, offset: usize, src: &[T]) {
        self.slice_mut(offset, src.len()).clone_from_slice(src);
    }

    /// Clone data from slice at the specified offset in mapped memory.
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    #[inline(always)]
    pub unsafe fn clone_at<T: Clone>(&self, offset: usize, src: &T) {
        *self.get_mut(offset) = src.clone();
    }

    #[implements]
    /// Unmap region
    pub fn end(self) {
        unsafe {
            self.1.unmap();
        }
    }
}
