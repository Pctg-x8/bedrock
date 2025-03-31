use crate::ffi_helper::slice_as_ptr_empty_null;
use crate::*;
use derives::implements;

pub trait Buffer: VkHandle<Handle = VkBuffer> + DeviceChildHandle {}
DerefContainerBracketImpl!(for Buffer {});
GuardsImpl!(for Buffer {});

pub trait BufferView: VkHandle<Handle = VkBufferView> {}
DerefContainerBracketImpl!(for BufferView {});
GuardsImpl!(for BufferView {});

/// Opaque handle to a buffer object(constructed via [`BufferDesc`])
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkBuffer::OBJECT_TYPE)]
pub struct BufferObject<Device: VkHandle<Handle = VkDevice>>(pub(crate) VkBuffer, pub(crate) Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for BufferObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for BufferObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for BufferObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for BufferObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for BufferObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: VkHandle<Handle = VkDevice>> Buffer for BufferObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> MemoryBound for BufferObject<Device> {
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    type MemoryRequirementsInfo2<'b>
        = BufferMemoryRequirementsInfo2<'b, Self>
    where
        Device: 'b;

    #[implements]
    fn requirements(&self) -> VkMemoryRequirements {
        let mut p = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_buffer_memory_requirements(self.1.native_ptr(), self.0, p.as_mut_ptr());

            p.assume_init()
        }
    }

    #[implements("VK_KHR_get_memory_requirements2")]
    fn requirements2<'b>(&'b self) -> Self::MemoryRequirementsInfo2<'b> {
        BufferMemoryRequirementsInfo2::new(self)
    }

    #[implements]
    fn bind(&mut self, memory: &(impl DeviceMemory + ?Sized), offset: usize) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkfn::bind_buffer_memory(self.1.native_ptr(), self.0, memory.native_ptr(), offset as _)
                .into_result()
                .map(drop)
        }
    }
}
impl<Device: VkHandle<Handle = VkDevice>> BufferObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the parent
    pub const unsafe fn manage(handle: VkBuffer, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkBuffer, Device) {
        let v = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (v, p)
    }
}
impl<Device: VkHandle<Handle = VkDevice> + Clone> BufferObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> BufferObject<Device> {
        let r = BufferObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}
impl<Device: crate::Device> BufferObject<Device> {
    /// Create a new buffer object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline]
    pub fn new(device: Device, info: &BufferCreateInfo) -> crate::Result<Self> {
        Ok(unsafe { Self::manage(device.new_buffer_raw(info, None)?, device) })
    }
}

#[derive(VkHandle, VkObject)]
#[VkObject(type = VkBufferView::OBJECT_TYPE)]
/// Opaque handle to a buffer view object
pub struct BufferViewObject<Buffer: DeviceChildHandle>(VkBufferView, Buffer);
#[implements]
impl<Buffer: DeviceChildHandle> Drop for BufferViewObject<Buffer> {
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.device_handle(), core::ptr::null());
        }
    }
}
unsafe impl<Buffer: DeviceChildHandle + Sync> Sync for BufferViewObject<Buffer> {}
unsafe impl<Buffer: DeviceChildHandle + Send> Send for BufferViewObject<Buffer> {}
impl<Buffer: DeviceChildHandle> DeviceChildHandle for BufferViewObject<Buffer> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.device_handle()
    }
}
impl<Buffer: DeviceChild> DeviceChild for BufferViewObject<Buffer> {
    type ConcreteDevice = Buffer::ConcreteDevice;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        self.1.device()
    }
}
impl<Buffer: DeviceChildHandle> BufferView for BufferViewObject<Buffer> {}
impl<Buffer: DeviceChildHandle> core::ops::Deref for BufferViewObject<Buffer> {
    type Target = Buffer;

    #[inline(always)]
    fn deref(&self) -> &Buffer {
        &self.1
    }
}
impl<Buffer: DeviceChildHandle> BufferViewObject<Buffer> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the parent
    pub const unsafe fn manage(handle: VkBufferView, parent: Buffer) -> Self {
        Self(handle, parent)
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkBufferView, Buffer) {
        let v = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (v, p)
    }
}
impl<Buffer: DeviceChildHandle + Clone> BufferViewObject<&'_ Buffer> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> BufferViewObject<Buffer> {
        let r = BufferViewObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}
impl<Buffer: DeviceChild> BufferViewObject<Buffer> {
    /// Create a new buffer view object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn new(buffer: Buffer, info: &BufferViewCreateInfo) -> crate::Result<Self> {
        use crate::Device;

        Ok(unsafe { Self::manage(buffer.device().new_buffer_view_raw(info, None)?, buffer) })
    }
}

/// Builder structure specifying the parameters of a newly created buffer object
#[repr(transparent)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BufferCreateInfo<'s>(VkBufferCreateInfo, core::marker::PhantomData<Option<&'s [u32]>>);
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
pub struct BufferViewCreateInfo<'d>(
    VkBufferViewCreateInfo,
    core::marker::PhantomData<&'d dyn VkHandle<Handle = VkBuffer>>,
);
impl<'d> BufferViewCreateInfo<'d> {
    #[inline]
    pub fn new(
        buffer: &'d (impl VkHandle<Handle = VkBuffer> + ?Sized),
        format: VkFormat,
        range: core::ops::Range<VkDeviceSize>,
    ) -> Self {
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
#[repr(transparent)]
pub struct BufferMemoryRequirementsInfo2<'b, Buffer: VkHandle<Handle = VkBuffer> + 'b>(
    VkBufferMemoryRequirementsInfo2KHR,
    &'b Buffer,
);
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'b, Buffer: VkHandle<Handle = VkBuffer> + 'b> BufferMemoryRequirementsInfo2<'b, Buffer> {
    pub fn new(buffer: &'b Buffer) -> Self {
        Self(
            VkBufferMemoryRequirementsInfo2KHR {
                sType: VkBufferMemoryRequirementsInfo2KHR::TYPE,
                pNext: core::ptr::null(),
                buffer: buffer.native_ptr(),
            },
            buffer,
        )
    }

    #[implements]
    pub fn query(self, sink: &mut core::mem::MaybeUninit<VkMemoryRequirements2KHR>)
    where
        Buffer: crate::DeviceChild,
    {
        #[cfg(feature = "Allow1_1APIs")]
        unsafe {
            crate::vkfn::get_buffer_memory_requirements2(self.1.device().native_ptr(), &self.0, sink.as_mut_ptr());
        }

        #[cfg(not(feature = "Allow1_1APIs"))]
        unsafe {
            use crate::Device;

            self.1.device().get_buffer_memory_requirements_2_khr_fn().0(
                self.1.device().native_ptr(),
                &self.0,
                sink.as_mut_ptr(),
            );
        }
    }
}

#[cfg(feature = "VK_KHR_bind_memory2")]
#[repr(transparent)]
pub struct BindBufferMemoryInfo<'b>(
    VkBindBufferMemoryInfoKHR,
    core::marker::PhantomData<(VkHandleRef<'b, VkBuffer>, VkHandleRef<'b, VkDeviceMemory>)>,
);
#[cfg(feature = "VK_KHR_bind_memory2")]
impl<'b> BindBufferMemoryInfo<'b> {
    pub fn new(
        buffer: &'b (impl VkHandle<Handle = VkBuffer> + ?Sized),
        memory: &'b (impl VkHandle<Handle = VkDeviceMemory> + ?Sized),
        offset: DeviceSize,
    ) -> Self {
        Self(
            VkBindBufferMemoryInfoKHR {
                sType: VkBindBufferMemoryInfoKHR::TYPE,
                pNext: core::ptr::null(),
                buffer: buffer.native_ptr(),
                memory: memory.native_ptr(),
                memoryOffset: offset,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkBindBufferMemoryInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkBindBufferMemoryInfoKHR {
        self.0
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
impl core::ops::BitOr for BufferUsage {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, other: Self) -> Self {
        BufferUsage(self.0 | other.0)
    }
}
impl core::ops::BitOrAssign for BufferUsage {
    #[inline(always)]
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}
impl From<BufferUsage> for VkBufferUsageFlags {
    #[inline(always)]
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
