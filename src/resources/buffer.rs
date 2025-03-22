#[implements]
use crate::DeviceMemory;
use crate::{
    Device, DeviceSize, MemoryBound, VkHandle, VkObject, VulkanStructure, VulkanStructureAsRef,
    ffi_helper::{opt_pointer, slice_as_ptr_empty_null},
    vk::*,
};
use core::{marker::PhantomData, mem::MaybeUninit, ops::Range};
use derives::{bitflags_newtype, implements};

/// Opaque handle to a buffer object.
#[repr(transparent)]
pub struct Buffer(VkBuffer_T);
#[implements]
impl Buffer {
    /// Destroy a buffer object.
    #[inline]
    pub unsafe fn destroy(
        &mut self,
        device: &(impl crate::DeviceT + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) {
        unsafe {
            crate::vkfn::destroy_buffer(
                device.native_ptr(),
                self as *mut _ as _,
                opt_pointer(allocation_callbacks),
            )
        }
    }
}
impl MemoryBound for Buffer {
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    type MemoryRequirementsInfo2<'b> = BufferMemoryRequirementsInfo2<'b>;

    #[implements]
    #[inline]
    unsafe fn get_memory_requirements(&self, device: &Device, sink: &mut MaybeUninit<super::MemoryRequirements>) {
        unsafe {
            crate::vkfn::get_buffer_memory_requirements(
                device as *const _ as _,
                self as *const _ as _,
                sink.as_mut_ptr(),
            )
        }
    }

    #[implements("VK_KHR_get_memory_requirements2")]
    #[inline]
    unsafe fn memory_requirements2<'b>(&'b self, device: &'b Device) -> Self::MemoryRequirementsInfo2<'b> {
        unsafe { BufferMemoryRequirementsInfo2::new(self, device) }
    }

    #[implements]
    #[inline]
    unsafe fn bind(
        &mut self,
        device: &Device,
        memory: &(impl DeviceMemory + ?Sized),
        offset: usize,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::bind_buffer_memory(
                device.native_ptr(),
                self as *mut _ as _,
                memory.native_ptr(),
                offset as _,
            )
            .into_result()
            .map(drop)
        }
    }
}

/// Opaque handle to a buffer view object.
#[repr(transparent)]
pub struct BufferView(VkBufferView_T);
#[implements]
impl BufferView {
    /// Destroy a buffer view object.
    #[inline]
    pub unsafe fn destroy(
        &mut self,
        device: &(impl crate::DeviceT + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) {
        unsafe {
            crate::vkfn::destroy_buffer_view(
                device.native_ptr(),
                self as *mut _ as _,
                opt_pointer(&allocation_callbacks),
            )
        }
    }
}

/// Builder structure specifying the parameters of a newly created buffer object
#[repr(transparent)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BufferCreateInfo<'s>(VkBufferCreateInfo, PhantomData<Option<&'s [u32]>>);
impl<'s> BufferCreateInfo<'s> {
    /// Creates a new buffer description with provided byte-size and usage flags
    pub const fn new(byte_size: usize, usage: BufferUsage) -> Self {
        Self(
            VkBufferCreateInfo {
                sType: VkBufferCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                size: byte_size as _,
                usage: usage.0,
                sharingMode: VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    /// Creates a new buffer description which fits for a type
    #[inline(always)]
    pub const fn new_for_type<T>(usage: BufferUsage) -> Self {
        Self::new(core::mem::size_of::<T>(), usage)
    }

    /// Wraps raw vulkan structure
    /// # Safety
    /// This function does not check any references/constraints
    pub const unsafe fn from_raw(s: VkBufferCreateInfo) -> Self {
        Self(s, core::marker::PhantomData)
    }

    /// Unwraps raw vulkan structure
    /// # Safety
    /// Lifetime constraints are removed
    pub const unsafe fn into_raw(self) -> VkBufferCreateInfo {
        self.0
    }

    /// A list of queue families that will access this buffer
    pub const fn sharing_queue_families(mut self, indices: &'s [u32]) -> Self {
        self.0.sharingMode = if indices.is_empty() {
            VK_SHARING_MODE_EXCLUSIVE
        } else {
            VK_SHARING_MODE_CONCURRENT
        };
        self.0.queueFamilyIndexCount = indices.len() as _;
        self.0.pQueueFamilyIndices = slice_as_ptr_empty_null(indices);

        self
    }

    /// A bitmask of `BufferSparseBinding` specifying additional parameters of the buffer
    pub const fn sparse_binding_opt(mut self, opt: BufferSparseBinding) -> Self {
        self.0.flags = opt as _;
        self
    }

    pub const fn and_usage(mut self, usage: BufferUsage) -> Self {
        self.0.usage |= usage.0;
        self
    }

    pub const fn size(&self) -> VkDeviceSize {
        self.0.size
    }

    pub const fn usage(&self) -> BufferUsage {
        BufferUsage(self.0.usage)
    }
}
impl crate::VulkanStructureProvider for BufferCreateInfo<'_> {
    type RootStructure = VkBufferCreateInfo;

    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut Self::RootStructure) -> &'r mut crate::GenericVulkanStructure {
        *root = self.0.clone();
        root.as_generic_mut()
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferViewCreateInfo<'d>(VkBufferViewCreateInfo, PhantomData<&'d Buffer>);
impl<'d> BufferViewCreateInfo<'d> {
    #[inline]
    pub fn new(buffer: &'d Buffer, format: VkFormat, range: Range<DeviceSize>) -> Self {
        Self(
            VkBufferViewCreateInfo {
                sType: VkBufferViewCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                buffer: buffer.native_ptr(),
                format,
                offset: range.start,
                range: range.end - range.start,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkBufferViewCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkBufferViewCreateInfo {
        self.0
    }
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct BufferMemoryRequirementsInfo2<'b>(VkBufferMemoryRequirementsInfo2KHR, &'b Device, PhantomData<&'b Buffer>);
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'b> BufferMemoryRequirementsInfo2<'b> {
    pub const unsafe fn new(buffer: &'b Buffer, device: &'b Device) -> Self {
        Self(
            VkBufferMemoryRequirementsInfo2KHR {
                sType: VkBufferMemoryRequirementsInfo2KHR::TYPE,
                pNext: core::ptr::null(),
                buffer: buffer as *const _ as _,
            },
            device,
            PhantomData,
        )
    }

    #[implements("Allow1_1APIs")]
    pub fn query(self, sink: &mut core::mem::MaybeUninit<VkMemoryRequirements2KHR>) {
        unsafe {
            crate::vkfn::get_buffer_memory_requirements2(self.1 as *const _ as _, &self.0, sink.as_mut_ptr());
        }
    }
}

/// Bitmask specifying allowed usage of a buffer
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[bitflags_newtype]
pub struct BufferUsage(pub VkBufferUsageFlags);
impl BufferUsage {
    /// Specifies that the buffer can be used as the source of a transfer command
    pub const TRANSFER_SRC: Self = Self(VK_BUFFER_USAGE_TRANSFER_SRC_BIT);
    /// Specifies that the buffer can be used as the destination of a transfer command
    pub const TRANSFER_DEST: Self = Self(VK_BUFFER_USAGE_TRANSFER_DST_BIT);
    /// Specifies that the buffer can be used to create a `BufferView` suitable for
    /// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT);
    /// Specifies that the buffer can be used to create a `BufferView` suitable for
    /// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`
    pub const STORAGE_TEXEL_BUFFER: Self = Self(VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT);
    /// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
    /// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`
    pub const UNIFORM_BUFFER: Self = Self(VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT);
    /// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
    /// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
    pub const STORAGE_BUFFER: Self = Self(VK_BUFFER_USAGE_STORAGE_BUFFER_BIT);
    /// Specifies that the buffer is suitable for passing as the `buffer` parameter to `DrawCommandBuffer::bind_index_buffer`
    pub const INDEX_BUFFER: Self = Self(VK_BUFFER_USAGE_INDEX_BUFFER_BIT);
    /// Specifies that the buffer is suitable for passing as an element of the `buffers` array to `DrawCommandBuffer::bind_vertex_buffers`
    pub const VERTEX_BUFFER: Self = Self(VK_BUFFER_USAGE_VERTEX_BUFFER_BIT);
    /// Specifies that the buffer is suitable for passing as the `buffer` parameter to
    /// `DrawCommandBuffer::draw_indirect`, `DrawCommandBuffer::draw_indexed_indirect`, or `ComputeCommandBuffer::dispatch_indirect`
    pub const INDIRECT_BUFFER: Self = Self(VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT);

    /// Generates a default access type mask
    pub const fn default_access_mask(self) -> VkAccessFlags {
        let mut bits = 0;
        if (self.0 & Self::TRANSFER_SRC.0) != 0 {
            bits |= VK_ACCESS_TRANSFER_READ_BIT;
        }
        if (self.0 & Self::TRANSFER_DEST.0) != 0 {
            bits |= VK_ACCESS_TRANSFER_WRITE_BIT;
        }
        if (self.0 & Self::UNIFORM_TEXEL_BUFFER.0) != 0 {
            bits |= VK_ACCESS_UNIFORM_READ_BIT;
        }
        if (self.0 & Self::STORAGE_TEXEL_BUFFER.0) != 0 {
            bits |= VK_ACCESS_UNIFORM_READ_BIT;
        }
        if (self.0 & Self::UNIFORM_BUFFER.0) != 0 {
            bits |= VK_ACCESS_UNIFORM_READ_BIT;
        }
        if (self.0 & Self::STORAGE_BUFFER.0) != 0 {
            bits |= VK_ACCESS_UNIFORM_READ_BIT;
        }
        if (self.0 & Self::INDEX_BUFFER.0) != 0 {
            bits |= VK_ACCESS_INDEX_READ_BIT;
        }
        if (self.0 & Self::VERTEX_BUFFER.0) != 0 {
            bits |= VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT;
        }
        if (self.0 & Self::INDIRECT_BUFFER.0) != 0 {
            bits |= VK_ACCESS_INDIRECT_COMMAND_READ_BIT;
        }
        bits
    }

    /// Determines if flag contains usage of uniform-buffer
    pub const fn is_uniform(self) -> bool {
        (self.0 & (Self::UNIFORM_BUFFER.0 | Self::UNIFORM_TEXEL_BUFFER.0)) != 0
    }
    /// Determines if flag contains usage of storage-buffer
    pub const fn is_storage(self) -> bool {
        (self.0 & (Self::STORAGE_BUFFER.0 | Self::STORAGE_TEXEL_BUFFER.0)) != 0
    }
}

/// Bitset specifying additional parameters of a buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum BufferSparseBinding {
    /// No sparse binding features
    None = 0,
    /// the buffer will be backed using sparse memory binding
    Bound = VK_BUFFER_CREATE_SPARSE_BINDING_BIT as _,
    /// the buffer can be partially backed using sparse memory binding.
    Residency = (VK_BUFFER_CREATE_SPARSE_BINDING_BIT | VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT) as _,
    /// the buffer will be backed using sparse memory binding with memory ranges
    /// that might also simultaneously be backing another buffer (or another portion of the same buffer)
    Aliased = (VK_BUFFER_CREATE_SPARSE_BINDING_BIT | VK_BUFFER_CREATE_SPARSE_ALIASED_BIT) as _,
    /// Aliased and Residency
    Both = (VK_BUFFER_CREATE_SPARSE_BINDING_BIT
        | VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT
        | VK_BUFFER_CREATE_SPARSE_ALIASED_BIT) as _,
}
