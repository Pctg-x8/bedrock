use crate::{vk::*, DeviceChild, MemoryBound, VkHandle, VkObject, VkRawHandle, VulkanStructure};
#[implements]
use crate::{DeviceMemory, VkHandleMut};
use derives::implements;
#[implements]
use std::ops::Range;
use std::ops::{BitOr, BitOrAssign, Deref};

pub trait Buffer: VkHandle<Handle = VkBuffer> + DeviceChild {
    /// Create a buffer view
    #[implements]
    fn create_view(self, format: VkFormat, range: Range<u64>) -> crate::Result<BufferViewObject<Self>>
    where
        Self: Sized,
    {
        let cinfo = VkBufferViewCreateInfo {
            sType: VkBufferViewCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            buffer: self.native_ptr(),
            format,
            offset: range.start,
            range: range.end - range.start,
        };
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_buffer_view(self.device().native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| BufferViewObject(h.assume_init(), self))
        }
    }
}
DerefContainerBracketImpl!(for Buffer {});
GuardsImpl!(for Buffer {});

pub trait BufferView: VkHandle<Handle = VkBufferView> + DeviceChild {}
DerefContainerBracketImpl!(for BufferView {});
GuardsImpl!(for BufferView {});

DefineStdDeviceChildObject! {
    /// Opaque handle to a buffer object(constructed via [`BufferDesc`])
    BufferObject(VkBuffer): Buffer { drop destroy_buffer }
}
impl<Device: crate::Device> MemoryBound for BufferObject<Device>
where
    Self: VkHandle<Handle = VkBuffer>,
{
    #[implements]
    fn requirements(&self) -> VkMemoryRequirements {
        let mut p = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::get_buffer_memory_requirements(
                self.device().native_ptr(),
                self.native_ptr(),
                p.as_mut_ptr(),
            );

            p.assume_init()
        }
    }

    #[implements]
    fn bind(&mut self, memory: &(impl DeviceMemory + ?Sized), offset: usize) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkresolve::bind_buffer_memory(
                self.device().native_ptr(),
                self.native_ptr_mut(),
                memory.native_ptr(),
                offset as _,
            )
            .into_result()
            .map(drop)
        }
    }
}

#[derive(VkHandle, VkObject)]
#[VkObject(type = VkBufferView::OBJECT_TYPE)]
/// Opaque handle to a buffer view object
pub struct BufferViewObject<Buffer: crate::Buffer>(VkBufferView, Buffer);
unsafe impl<Buffer: crate::Buffer + Sync> Sync for BufferViewObject<Buffer> {}
unsafe impl<Buffer: crate::Buffer + Send> Send for BufferViewObject<Buffer> {}
impl<Buffer: crate::Buffer> DeviceChild for BufferViewObject<Buffer> {
    type ConcreteDevice = Buffer::ConcreteDevice;

    fn device(&self) -> &Self::ConcreteDevice {
        self.1.device()
    }
}
#[implements]
impl<Buffer: crate::Buffer> Drop for BufferViewObject<Buffer> {
    fn drop(&mut self) {
        unsafe {
            crate::vkresolve::destroy_buffer_view(self.1.device().native_ptr(), self.0, std::ptr::null());
        }
    }
}
impl<Buffer: crate::Buffer> BufferView for BufferViewObject<Buffer> {}
impl<Buffer: crate::Buffer> Deref for BufferViewObject<Buffer> {
    type Target = Buffer;

    fn deref(&self) -> &Buffer {
        &self.1
    }
}

/// Builder structure specifying the parameters of a newly created buffer object
#[repr(transparent)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BufferDesc<'s>(VkBufferCreateInfo, core::marker::PhantomData<Option<&'s [u32]>>);
impl<'s> BufferDesc<'s> {
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
    pub fn sharing_queue_families(mut self, indices: &'s [u32]) -> Self {
        self.0.sharingMode = if indices.is_empty() {
            VK_SHARING_MODE_EXCLUSIVE
        } else {
            VK_SHARING_MODE_CONCURRENT
        };
        self.0.queueFamilyIndexCount = indices.len() as _;
        self.0.pQueueFamilyIndices = indices.as_ptr();

        self
    }

    /// A bitmask of `BufferSparseBinding` specifying additional parameters of the buffer
    pub const fn sparse_binding_opt(mut self, opt: BufferSparseBinding) -> Self {
        self.0.flags = opt as _;
        self
    }

    /// Create a new buffer object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    pub fn create<Device: crate::Device>(self, device: Device) -> crate::Result<BufferObject<Device>> {
        device.new_buffer(self)
    }
}
impl crate::VulkanStructureProvider for BufferDesc<'_> {
    type RootStructure = VkBufferCreateInfo;

    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut Self::RootStructure) -> &'r mut crate::GenericVulkanStructure {
        *root = self.0.clone();
        root.as_generic_mut()
    }
}

/// Bitmask specifying allowed usage of a buffer
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
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

    /// Specifies that the buffer can be used as the source of a transfer command
    pub const fn transfer_src(self) -> Self {
        Self(self.0 | Self::TRANSFER_SRC.0)
    }
    /// Specifies that the buffer can be used as the destination of a transfer command
    pub const fn transfer_dest(self) -> Self {
        Self(self.0 | Self::TRANSFER_DEST.0)
    }
    /// Specifies that the buffer can be used to create a `BufferView` suitable for
    /// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`
    pub const fn uniform_texel_buffer(self) -> Self {
        Self(self.0 | Self::UNIFORM_TEXEL_BUFFER.0)
    }
    /// Specifies that the buffer can be used to create a `BufferView` suitable for
    /// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`
    pub const fn storage_texel_buffer(self) -> Self {
        Self(self.0 | Self::STORAGE_TEXEL_BUFFER.0)
    }
    /// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
    /// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`
    pub const fn uniform_buffer(self) -> Self {
        Self(self.0 | Self::UNIFORM_BUFFER.0)
    }
    /// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
    /// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
    pub const fn storage_buffer(self) -> Self {
        Self(self.0 | Self::STORAGE_BUFFER.0)
    }
    /// Specifies that the buffer is suitable for passing as the `buffer` parameter to `DrawCommandBuffer::bind_index_buffer`
    pub const fn index_buffer(self) -> Self {
        Self(self.0 | Self::INDEX_BUFFER.0)
    }
    /// Specifies that the buffer is suitable for passing as an element of the `buffers` array to `DrawCommandBuffer::bind_vertex_buffers`
    pub const fn vertex_buffer(self) -> Self {
        Self(self.0 | Self::VERTEX_BUFFER.0)
    }
    /// Specifies that the buffer is suitable for passing as the `buffer` parameter to
    /// `DrawCommandBuffer::draw_indirect`, `DrawCommandBuffer::draw_indexed_indirect`, or `ComputeCommandBuffer::dispatch_indirect`
    pub const fn indirect_buffer(self) -> Self {
        Self(self.0 | Self::INDIRECT_BUFFER.0)
    }

    /// merge two flags (const alias of BitOr)
    pub const fn merge(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

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
impl BitOr for BufferUsage {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        BufferUsage(self.0 | other.0)
    }
}
impl BitOrAssign for BufferUsage {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}
impl From<BufferUsage> for VkBufferUsageFlags {
    fn from(value: BufferUsage) -> Self {
        value.0
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
