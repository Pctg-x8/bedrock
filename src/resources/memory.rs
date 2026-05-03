use crate::*;
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

        unsafe {
            crate::vkfn::map_memory(
                self.device_handle(),
                self.native_ptr_mut(),
                range.start,
                range.end - range.start,
                0,
                p.as_mut_ptr(),
            )
            .into_result()?;
        }

        Ok(unsafe { p.assume_init() })
    }

    /// Map a memory object into application address space
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_MEMORY_MAP_FAILED`
    #[implements]
    fn map(&mut self, range: core::ops::Range<usize>) -> crate::Result<MappedMemory<Self>> {
        let p = unsafe { self.map_raw(range.start as _..range.end as _)? };

        Ok(MappedMemory(p, self))
    }

    /// Unmap a previously mapped memory object
    /// # Safety
    /// Caller must guarantee that there is no `MappedMemoryRange` alives.
    /// Accessing the mapped memory after this call has undefined behavior
    #[implements]
    #[inline]
    unsafe fn unmap(&mut self) {
        unsafe { crate::vkfn::unmap_memory(self.device_handle(), self.native_ptr_mut()) }
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
            crate::vkfn::free_memory(self.1.native_ptr(), self.0, core::ptr::null());
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
        let r = DeviceMemoryObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}
impl<Device: crate::Device> DeviceMemoryObject<Device> {
    /// Create DeviceMemory object with allocating device memory
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INVALID_EXTERNAL_HANDLE`]
    /// * [`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
    #[implements]
    #[inline]
    pub fn new(device: Device, info: &MemoryAllocateInfo) -> crate::Result<Self> {
        Ok(unsafe { Self::manage(device.allocate_memory(info, None)?, device) })
    }
}

/// Structure containing parameters of a memory allocation
#[repr(transparent)]
#[derive(Clone)]
pub struct MemoryAllocateInfo<'d>(
    VkMemoryAllocateInfo,
    core::marker::PhantomData<Option<&'d dyn VulkanStructure>>,
);
impl<'d> MemoryAllocateInfo<'d> {
    pub const fn new(size: VkDeviceSize, memory_type_index: u32) -> Self {
        Self(
            VkMemoryAllocateInfo {
                sType: VkMemoryAllocateInfo::TYPE,
                pNext: core::ptr::null(),
                allocationSize: size,
                memoryTypeIndex: memory_type_index,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkMemoryAllocateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkMemoryAllocateInfo {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    pub const unsafe fn next_sink<T: VulkanStructure>(&mut self) -> &mut *const T {
        unsafe { core::mem::transmute(&mut self.0.pNext) }
    }
}
unsafe impl VulkanStructure for MemoryAllocateInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        self.0.as_generic()
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        self.0.as_generic_mut()
    }
}

/// Specify a dedicated memory allocation resource
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[repr(transparent)]
#[derive(Clone)]
pub struct MemoryDedicatedAllocateInfo<'d>(
    VkMemoryDedicatedAllocateInfoKHR,
    core::marker::PhantomData<(
        Option<&'d dyn VulkanStructure>,
        Option<&'d dyn VkHandle<Handle = VkImage>>,
        Option<&'d dyn VkHandle<Handle = VkBuffer>>,
    )>,
);
#[cfg(feature = "VK_KHR_dedicated_allocation")]
unsafe impl VulkanStructure for MemoryDedicatedAllocateInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl<'d> MemoryDedicatedAllocateInfo<'d> {
    #[inline]
    pub fn for_buffer(buffer: &'d (impl VkHandle<Handle = VkBuffer> + ?Sized)) -> Self {
        Self(
            VkMemoryDedicatedAllocateInfoKHR {
                sType: VkMemoryDedicatedAllocateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                image: None,
                buffer: Some(buffer.native_ptr()),
            },
            core::marker::PhantomData,
        )
    }

    #[inline]
    pub const unsafe fn for_buffer_raw(buffer: VkBuffer) -> Self {
        Self(
            VkMemoryDedicatedAllocateInfoKHR {
                sType: VkMemoryDedicatedAllocateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                image: None,
                buffer: Some(buffer),
            },
            core::marker::PhantomData,
        )
    }

    #[inline]
    pub fn for_image(image: &'d (impl VkHandle<Handle = VkImage> + ?Sized)) -> Self {
        Self(
            VkMemoryDedicatedAllocateInfoKHR {
                sType: VkMemoryDedicatedAllocateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                image: Some(image.native_ptr()),
                buffer: None,
            },
            core::marker::PhantomData,
        )
    }

    #[inline]
    pub const unsafe fn for_image_raw(image: VkImage) -> Self {
        Self(
            VkMemoryDedicatedAllocateInfoKHR {
                sType: VkMemoryDedicatedAllocateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                image: Some(image),
                buffer: None,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkMemoryDedicatedAllocateInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkMemoryDedicatedAllocateInfoKHR {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    pub const unsafe fn next_sink<T: VulkanStructure>(&mut self) -> &mut *const T {
        unsafe { core::mem::transmute(&mut self.0.pNext) }
    }
}

/// Structure specifying a mapped memory range.
pub struct MappedMemoryRange<'a>(
    VkMappedMemoryRange,
    core::marker::PhantomData<&'a dyn VkHandle<Handle = VkDeviceMemory>>,
);
impl<'a> MappedMemoryRange<'a> {
    pub fn new(
        memory: &'a (impl VkHandle<Handle = VkDeviceMemory> + ?Sized),
        range: core::ops::Range<DeviceSize>,
    ) -> Self {
        Self(
            VkMappedMemoryRange {
                sType: VkMappedMemoryRange::TYPE,
                pNext: core::ptr::null(),
                memory: memory.native_ptr(),
                offset: range.start,
                size: range.end - range.start,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn new_raw(memory: VkDeviceMemory, offset: DeviceSize, size: DeviceSize) -> Self {
        Self(
            VkMappedMemoryRange {
                sType: VkMappedMemoryRange::TYPE,
                pNext: core::ptr::null(),
                memory,
                offset,
                size,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkMappedMemoryRange) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkMappedMemoryRange {
        self.0
    }
}

/// Specifies the block of mapped memory in a `DeviceMemory`
pub struct MappedMemory<'m, DeviceMemory: crate::DeviceMemoryMut + ?Sized + 'm>(
    *mut core::ffi::c_void,
    &'m mut DeviceMemory,
);
#[allow(clippy::mut_from_ref)]
impl<'m, DeviceMemory: crate::DeviceMemoryMut + ?Sized + 'm> MappedMemory<'m, DeviceMemory> {
    /// Returns a pointer to the head of the mapped region
    pub const fn ptr(&self) -> *mut core::ffi::c_void {
        self.0
    }

    /// Get a mutable pointer in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn addr_of_mut<T>(&self, offset: usize) -> *mut T {
        unsafe { self.0.byte_add(offset) as *mut T }
    }

    /// Get a reference in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn get<T>(&self, offset: usize) -> &T {
        unsafe { &*(self.0.byte_add(offset) as *const T) }
    }

    /// Get a mutable reference in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn get_mut<T>(&self, offset: usize) -> &mut T {
        unsafe { &mut *(self.0.byte_add(offset) as *mut T) }
    }

    /// Get a slice in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn slice<T>(&self, offset: usize, count: usize) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.0.byte_add(offset) as *const T, count) }
    }

    /// Get a mutable slice in mapped memory with byte offsets
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    pub const unsafe fn slice_mut<T>(&self, offset: usize, count: usize) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.0.byte_add(offset) as *mut T, count) }
    }

    /// Clone data from slice at the specified offset in mapped memory.
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    #[inline(always)]
    pub unsafe fn clone_from_slice_at<T: Clone>(&self, offset: usize, src: &[T]) {
        unsafe { self.slice_mut(offset, src.len()).clone_from_slice(src) };
    }

    /// Clone data from slice at the specified offset in mapped memory.
    /// # Safety
    /// Caller must guarantee that the pointer and its alignment are valid
    #[inline(always)]
    pub unsafe fn clone_at<T: Clone>(&self, offset: usize, src: &T) {
        unsafe { *self.get_mut(offset) = src.clone() };
    }

    #[implements]
    /// Unmap region
    pub fn end(self) {
        unsafe {
            self.1.unmap();
        }
    }
}
