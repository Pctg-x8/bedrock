use bedrock_vk::{self as brvk, TypedVulkanStructure, VkRawHandle};

use crate::*;
use derives::implements;

pub trait DeviceMemory: VkHandle<Handle = brvk::VkDeviceMemory> + DeviceChildHandle {
    /// Query the current commitment for a `DeviceMemory`
    #[implements]
    #[inline]
    fn commitment_bytes(&self) -> brvk::VkDeviceSize {
        let mut b = 0;
        unsafe {
            brvk::fns::get_device_memory_commitment(self.device_handle(), self.native_ptr(), &mut b);
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
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_MEMORY_MAP_FAILED`
    #[implements]
    #[inline(always)]
    unsafe fn map_raw(&mut self, range: core::ops::Range<DeviceSize>) -> crate::Result<*mut core::ffi::c_void> {
        unsafe {
            crate::vkfn_wrapper::map_memory(
                self.device_transparent_ref(),
                VkHandleRefMut::dangling(self.native_ptr()),
                range,
                0,
            )
        }
    }

    /// Map a memory object into application address space
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_MEMORY_MAP_FAILED`
    #[implements]
    #[inline]
    fn map<'a>(&'a mut self, range: core::ops::Range<usize>) -> crate::Result<MappedMemory<'a, Self>> {
        Ok(MappedMemory(
            unsafe { self.map_raw(range.start as _..range.end as _)? },
            self,
        ))
    }

    /// Unmap a previously mapped memory object
    /// # Safety
    /// Caller must guarantee that there is no `MappedMemoryRange` alives.
    /// Accessing the mapped memory after this call has undefined behavior
    #[implements]
    #[inline(always)]
    unsafe fn unmap(&mut self) {
        unsafe {
            crate::vkfn_wrapper::unmap_memory(
                self.device_transparent_ref(),
                VkHandleRefMut::dangling(self.native_ptr()),
            )
        }
    }
}
DerefContainerBracketImpl!(for mut DeviceMemoryMut {});
GuardsImpl!(for mut DeviceMemoryMut {});

#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VkDeviceMemory::OBJECT_TYPE)]
pub struct DeviceMemoryObject<Device: VkHandle<Handle = brvk::VkDevice>>(brvk::VkDeviceMemory, Device);
#[implements]
impl<Device: VkHandle<Handle = brvk::VkDevice>> Drop for DeviceMemoryObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn_wrapper::free_memory(self.device_transparent_ref(), VkHandleRefMut::dangling(self.0), None);
        }
    }
}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Sync> Sync for DeviceMemoryObject<Device> {}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Send> Send for DeviceMemoryObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceChildHandle for DeviceMemoryObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice {
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
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceMemory for DeviceMemoryObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceMemoryMut for DeviceMemoryObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceMemoryObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the parent
    pub const unsafe fn manage(handle: brvk::VkDeviceMemory, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (brvk::VkDeviceMemory, Device) {
        let v = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (v, p)
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice> + Clone> DeviceMemoryObject<&'_ Device> {
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
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_INVALID_EXTERNAL_HANDLE`]
    /// * [`brvk::VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
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
    brvk::VkMemoryAllocateInfo,
    core::marker::PhantomData<Option<&'d dyn brvk::VulkanStructure>>,
);
impl<'d> MemoryAllocateInfo<'d> {
    pub const fn new(size: brvk::VkDeviceSize, memory_type_index: u32) -> Self {
        Self(
            brvk::VkMemoryAllocateInfo {
                sType: brvk::VkMemoryAllocateInfo::TYPE,
                pNext: core::ptr::null(),
                allocationSize: size,
                memoryTypeIndex: memory_type_index,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkMemoryAllocateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkMemoryAllocateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkMemoryAllocateInfo {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    /// # Safety
    ///
    /// next data must be a value for type of T.
    pub const unsafe fn next_sink<T: brvk::VulkanStructure>(&mut self) -> &mut *const T {
        unsafe { core::mem::transmute(&mut self.0.pNext) }
    }
}
unsafe impl brvk::VulkanStructure for MemoryAllocateInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        self.0.as_generic()
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        self.0.as_generic_mut()
    }
}

/// Specify a dedicated memory allocation resource
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[repr(transparent)]
#[derive(Clone)]
pub struct MemoryDedicatedAllocateInfo<'d>(
    brvk::VkMemoryDedicatedAllocateInfoKHR,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        Option<&'d dyn brvk::VulkanStructure>,
        Option<&'d dyn VkHandle<Handle = brvk::VkImage>>,
        Option<&'d dyn VkHandle<Handle = brvk::VkBuffer>>,
    )>,
);
#[cfg(feature = "VK_KHR_dedicated_allocation")]
unsafe impl brvk::VulkanStructure for MemoryDedicatedAllocateInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl<'d> MemoryDedicatedAllocateInfo<'d> {
    #[inline]
    pub fn for_buffer(buffer: &'d (impl VkHandle<Handle = brvk::VkBuffer> + ?Sized)) -> Self {
        Self(
            brvk::VkMemoryDedicatedAllocateInfoKHR {
                sType: brvk::VkMemoryDedicatedAllocateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                image: None,
                buffer: Some(buffer.native_ptr()),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `buffer` must be a valid object.
    #[inline]
    pub const unsafe fn for_buffer_raw(buffer: brvk::VkBuffer) -> Self {
        Self(
            brvk::VkMemoryDedicatedAllocateInfoKHR {
                sType: brvk::VkMemoryDedicatedAllocateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                image: None,
                buffer: Some(buffer),
            },
            core::marker::PhantomData,
        )
    }

    #[inline]
    pub fn for_image(image: &'d (impl VkHandle<Handle = brvk::VkImage> + ?Sized)) -> Self {
        Self(
            brvk::VkMemoryDedicatedAllocateInfoKHR {
                sType: brvk::VkMemoryDedicatedAllocateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                image: Some(image.native_ptr()),
                buffer: None,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `image` must be a valid object.
    #[inline]
    pub const unsafe fn for_image_raw(image: brvk::VkImage) -> Self {
        Self(
            brvk::VkMemoryDedicatedAllocateInfoKHR {
                sType: brvk::VkMemoryDedicatedAllocateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                image: Some(image),
                buffer: None,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkMemoryDedicatedAllocateInfoKHR`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkMemoryDedicatedAllocateInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkMemoryDedicatedAllocateInfoKHR {
        self.0
    }

    pub const fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    /// # Safety
    ///
    /// `next` must be a value for type of T.
    pub const unsafe fn next_sink<T: brvk::VulkanStructure>(&mut self) -> &mut *const T {
        unsafe { core::mem::transmute(&mut self.0.pNext) }
    }
}

/// Structure specifying a mapped memory range.
pub struct MappedMemoryRange<'a>(
    brvk::VkMappedMemoryRange,
    core::marker::PhantomData<&'a dyn VkHandle<Handle = brvk::VkDeviceMemory>>,
);
impl<'a> MappedMemoryRange<'a> {
    pub fn new(
        memory: &'a (impl VkHandle<Handle = brvk::VkDeviceMemory> + ?Sized),
        range: core::ops::Range<brvk::VkDeviceSize>,
    ) -> Self {
        Self(
            brvk::VkMappedMemoryRange {
                sType: brvk::VkMappedMemoryRange::TYPE,
                pNext: core::ptr::null(),
                memory: memory.native_ptr(),
                offset: range.start,
                size: range.end - range.start,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `memory` must be a valid object.
    pub const unsafe fn new_raw(
        memory: brvk::VkDeviceMemory,
        offset: brvk::VkDeviceSize,
        size: brvk::VkDeviceSize,
    ) -> Self {
        Self(
            brvk::VkMappedMemoryRange {
                sType: brvk::VkMappedMemoryRange::TYPE,
                pNext: core::ptr::null(),
                memory,
                offset,
                size,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkMappedMemoryRange`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkMappedMemoryRange) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkMappedMemoryRange {
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
