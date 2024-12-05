//! Vulkan Descriptors

use cfg_if::cfg_if;
use derives::{implements, transparent_marked};

use crate::ffi_helper::{slice_as_ptr_empty_null, ArrayFFIExtensions};
use crate::{
    vk::*, DeviceChild, DeviceChildHandle, ImageLayout, SamplerObjectRef, ShaderStage, VkDeviceChildNonExtDestroyable,
    VkHandle, VkHandleMut, VkHandleRef, VkObject, VkRawHandle, VulkanStructure,
};

/// Opaque handle to a descriptor set layout object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkDescriptorSetLayout::OBJECT_TYPE)]
pub struct DescriptorSetLayoutObject<Device: VkHandle<Handle = VkDevice>>(
    pub(crate) VkDescriptorSetLayout,
    pub(crate) Device,
);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for DescriptorSetLayoutObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for DescriptorSetLayoutObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for DescriptorSetLayoutObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for DescriptorSetLayoutObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for DescriptorSetLayoutObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: crate::Device> DescriptorSetLayout for DescriptorSetLayoutObject<Device> {}

#[derive(VkHandle, VkObject)]
#[VkObject(type = VkDescriptorPool::OBJECT_TYPE)]
pub struct DescriptorPoolObject<Device: VkHandle<Handle = VkDevice>>(pub(crate) VkDescriptorPool, pub(crate) Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for DescriptorPoolObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for DescriptorPoolObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for DescriptorPoolObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for DescriptorPoolObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for DescriptorPoolObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: VkHandle<Handle = VkDevice>> DescriptorPool for DescriptorPoolObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DescriptorPoolMut for DescriptorPoolObject<Device> {}

#[transparent_marked]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DescriptorSet(pub VkDescriptorSet);
impl From<DescriptorSet> for VkDescriptorSet {
    fn from(v: DescriptorSet) -> Self {
        v.0
    }
}
impl AsRef<VkDescriptorSet> for DescriptorSet {
    fn as_ref(&self) -> &VkDescriptorSet {
        &self.0
    }
}
impl std::ops::Deref for DescriptorSet {
    type Target = VkDescriptorSet;

    fn deref(&self) -> &VkDescriptorSet {
        &self.0
    }
}
unsafe impl Sync for DescriptorSet {}
unsafe impl Send for DescriptorSet {}
impl DescriptorSet {
    #[inline]
    pub const fn binding_at(&self, b: u32) -> DescriptorPointer {
        DescriptorPointer::new(self.0, b)
    }
}

/// Specified the type of a descriptor in a descriptor set
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Hash)]
pub enum DescriptorType {
    Sampler = VK_DESCRIPTOR_TYPE_SAMPLER as _,
    CombinedImageSampler = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER as _,
    SampledImage = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE as _,
    StorageImage = VK_DESCRIPTOR_TYPE_STORAGE_IMAGE as _,
    UniformTexelBuffer = VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER as _,
    StorageTexelBuffer = VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER as _,
    UniformBuffer = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER as _,
    StorageBuffer = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER as _,
    UniformBufferDynamic = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC as _,
    StorageBufferDynamic = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC as _,
    InputAttachment = VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT as _,
}
impl DescriptorType {
    pub const fn make_size(self, count: u32) -> VkDescriptorPoolSize {
        VkDescriptorPoolSize {
            _type: self as _,
            descriptorCount: count,
        }
    }

    pub const fn make_binding<'a>(self, binding: u32, count: u32) -> DescriptorSetLayoutBinding<'a> {
        DescriptorSetLayoutBinding::new(binding, self, count, ShaderStage::ALL)
    }
}

#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, Debug, VkHandle)]
pub struct DescriptorSetLayoutObjectRef<'s>(
    VkDescriptorSetLayout,
    core::marker::PhantomData<&'s dyn VkHandle<Handle = VkDescriptorSetLayout>>,
);
impl<'s> DescriptorSetLayoutObjectRef<'s> {
    #[inline]
    pub fn new(x: &'s (impl VkHandle<Handle = VkDescriptorSetLayout> + ?Sized)) -> Self {
        Self(x.native_ptr(), core::marker::PhantomData)
    }

    /// Lifetime unbound constructor
    #[inline]
    pub const unsafe fn unbound(x: VkDescriptorSetLayout) -> Self {
        Self(x, core::marker::PhantomData)
    }
}

#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, Debug, VkHandle)]
pub struct ImageViewObjectRef<'s>(
    VkImageView,
    core::marker::PhantomData<&'s dyn VkHandle<Handle = VkImageView>>,
);
impl<'s> ImageViewObjectRef<'s> {
    #[inline]
    pub fn new(r: &'s (impl VkHandle<Handle = VkImageView> + ?Sized)) -> Self {
        Self(r.native_ptr(), core::marker::PhantomData)
    }
}

#[transparent_marked]
#[derive(Clone, Hash, PartialEq, Eq, Debug, VkHandle)]
pub struct BufferObjectRef<'s>(VkBuffer, core::marker::PhantomData<&'s dyn VkHandle<Handle = VkBuffer>>);
impl<'s> BufferObjectRef<'s> {
    #[inline]
    pub fn new(r: &'s (impl VkHandle<Handle = VkBuffer> + ?Sized)) -> Self {
        Self(r.native_ptr(), core::marker::PhantomData)
    }
}

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DescriptorSetLayoutBinding<'s> {
    raw: VkDescriptorSetLayoutBinding,
    immutable_samplers: core::marker::PhantomData<&'s [SamplerObjectRef<'s>]>,
}
impl<'s> DescriptorSetLayoutBinding<'s> {
    #[inline(always)]
    pub const fn new(binding: u32, r#type: DescriptorType, count: u32, shader_stage: ShaderStage) -> Self {
        Self {
            raw: VkDescriptorSetLayoutBinding {
                binding,
                descriptorType: r#type as _,
                descriptorCount: count,
                stageFlags: shader_stage.0,
                pImmutableSamplers: core::ptr::null(),
            },
            immutable_samplers: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn with_immutable_samplers(self, samplers: &'s [SamplerObjectRef<'s>]) -> Self {
        assert_eq!(samplers.len(), self.raw.descriptorCount as usize);
        unsafe { self.with_immutable_samplers_unchecked(samplers) }
    }

    #[inline(always)]
    pub const unsafe fn with_immutable_samplers_unchecked(mut self, samplers: &'s [SamplerObjectRef<'s>]) -> Self {
        self.raw.pImmutableSamplers = slice_as_ptr_empty_null(samplers) as _;
        self
    }

    #[inline(always)]
    pub const fn for_shader_stage(mut self, mask: ShaderStage) -> Self {
        self.raw.stageFlags = mask.0;
        self
    }

    #[inline(always)]
    pub const fn only_for_vertex(self) -> Self {
        self.for_shader_stage(ShaderStage::VERTEX)
    }

    #[inline(always)]
    pub const fn only_for_tess_control(self) -> Self {
        self.for_shader_stage(ShaderStage::TESSELLATION_CONTROL)
    }

    #[inline(always)]
    pub const fn only_for_tess_evaluation(self) -> Self {
        self.for_shader_stage(ShaderStage::TESSELLATION_EVALUATION)
    }

    #[inline(always)]
    pub const fn only_for_tessellation(self) -> Self {
        self.for_shader_stage(ShaderStage::TESSELLATION)
    }

    #[inline(always)]
    pub const fn only_for_geometry(self) -> Self {
        self.for_shader_stage(ShaderStage::GEOMETRY)
    }

    #[inline(always)]
    pub const fn only_for_fragment(self) -> Self {
        self.for_shader_stage(ShaderStage::FRAGMENT)
    }

    #[inline(always)]
    pub const fn only_for_compute(self) -> Self {
        self.for_shader_stage(ShaderStage::COMPUTE)
    }
}

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct DescriptorSetLayoutBuilder<'d, 's>(
    VkDescriptorSetLayoutCreateInfo,
    core::marker::PhantomData<&'d [DescriptorSetLayoutBinding<'s>]>,
);
impl<'d, 's> DescriptorSetLayoutBuilder<'d, 's> {
    #[inline(always)]
    pub const fn new(bindings: &'d [DescriptorSetLayoutBinding<'s>]) -> Self {
        Self(
            VkDescriptorSetLayoutCreateInfo {
                sType: VkDescriptorSetLayoutCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                bindingCount: bindings.len() as _,
                pBindings: slice_as_ptr_empty_null(bindings) as _,
            },
            core::marker::PhantomData,
        )
    }

    /// Create a new descriptor set layout
    /// # Failures
    /// On failure, this command returns
    ///
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    #[inline]
    pub fn create<Device: VkHandle<Handle = VkDevice>>(
        self,
        device: Device,
    ) -> crate::Result<DescriptorSetLayoutObject<Device>>
    where
        Self: Sized,
    {
        unsafe { DescriptorSetLayoutObject::new_raw(device, &self.0) }
    }
}

impl<Device: VkHandle<Handle = VkDevice>> DescriptorSetLayoutObject<Device> {
    /// Create a new descriptor set layout
    /// # Failures
    /// On failure, this command returns
    ///
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    pub unsafe fn new_raw(device: Device, info: &VkDescriptorSetLayoutCreateInfo) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_descriptor_set_layout(device.native_ptr(), info, core::ptr::null(), h.as_mut_ptr())
            .into_result()?;

        Ok(Self(h.assume_init(), device))
    }

    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkDescriptorSetLayout, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkDescriptorSetLayout, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}

pub trait DescriptorSetLayout: VkHandle<Handle = VkDescriptorSetLayout> + DeviceChild {
    #[inline(always)]
    fn as_transparent_ref(&self) -> DescriptorSetLayoutObjectRef {
        DescriptorSetLayoutObjectRef::new(self)
    }
}
DerefContainerBracketImpl!(for DescriptorSetLayout {});
GuardsImpl!(for DescriptorSetLayout {});

/*
# DescriptorPoolのフラグメンテーションについてメモ(from `VkDescriptorPoolCreateInfo` Manual)

`VkDescriptorPoolSize`構造体が`pPoolSizes`配列内に複数ある場合、プールはそれぞれのタイプの合計分のデスクリプタが十分入るように確保されます。

DescriptorPoolはフラグメンテーションを起こすことがあり、DescriptorSetの確保に失敗することがあります。
フラグメンテーションに起因する失敗は、確保したDescriptorSetの数+確保を要求したDescriptorSetの数が`maxSets`に満たない場合でも
"DescriptorSetの確保の失敗"と定義されます。(たぶんあってるはず)
実装は、以下に記述されるような"フラグメンテーションが確保の失敗を引き起こさない場合"について確固たる保証を提供します。
(言い換えると、「以下に示す場合はフラグメンテーション状態でも確保に成功する」)

DescriptorPoolが、生成されてから/間近にリセットされてから今までに開放されたDescriptorSetがない場合、
フラグメンテーションは確保の失敗を引き起こしません。(`VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT`を伴わずに生成されたプールに対しては常に満たすものとします？)
また、
- プールが生成されてから/間近にリセットされてから確保された、すべてのDescriptorSetが各タイプ同じ数のDescriptorを使う場合、そして
- 要求した確保も各タイプ同じ数のDescriptorを使う場合、
フラグメンテーションは確保の失敗を引き起こしません。

もしフラグメンテーションによって確保が失敗した場合、アプリケーションは続けてDescriptorSetの確保を行うために追加のDescriptorPoolを生成することができます
*/

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct DescriptorPoolBuilder<'d>(
    VkDescriptorPoolCreateInfo,
    core::marker::PhantomData<&'d [VkDescriptorPoolSize]>,
);
impl<'d> DescriptorPoolBuilder<'d> {
    pub const fn new(max_sets: u32, sizes: &'d [VkDescriptorPoolSize]) -> Self {
        Self(
            VkDescriptorPoolCreateInfo {
                sType: VkDescriptorPoolCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                maxSets: max_sets,
                poolSizeCount: sizes.len() as _,
                pPoolSizes: slice_as_ptr_empty_null(sizes),
            },
            core::marker::PhantomData,
        )
    }

    pub const fn allow_individual_free(mut self) -> Self {
        self.0.flags |= VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT;
        self
    }

    /// Creates a descriptor pool object
    /// # Failures
    /// On failure, this command returns
    ///
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    #[inline]
    pub fn create<Device: crate::Device>(self, device: Device) -> crate::Result<DescriptorPoolObject<Device>> {
        unsafe { DescriptorPoolObject::new_raw(device, &self.0) }
    }
}

impl<Device: VkHandle<Handle = VkDevice>> DescriptorPoolObject<Device> {
    /// Creates a descriptor pool object
    /// # Failures
    /// On failure, this command returns
    ///
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    pub unsafe fn new_raw(device: Device, info: &VkDescriptorPoolCreateInfo) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_descriptor_pool(device.native_ptr(), info, core::ptr::null(), h.as_mut_ptr())
            .into_result()?;

        Ok(Self(h.assume_init(), device))
    }

    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkDescriptorPool, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkDescriptorPool, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}

pub trait DescriptorPool: VkHandle<Handle = VkDescriptorPool> {}
DerefContainerBracketImpl!(for DescriptorPool {});
GuardsImpl!(for DescriptorPool {});

pub trait DescriptorPoolMut: DescriptorPool + VkHandleMut + DeviceChildHandle {
    /// Allocate one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    /// - VK_ERROR_FRAGMENTED_POOL
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    #[inline]
    unsafe fn alloc_raw(
        &mut self,
        info: &VkDescriptorSetAllocateInfo,
        objects: &mut [VkDescriptorSet],
    ) -> crate::Result<()> {
        crate::vkfn::allocate_descriptor_sets(self.device_handle(), info, objects.as_mut_ptr())
            .into_result()
            .map(drop)
    }

    /// Allocate one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    /// - VK_ERROR_FRAGMENTED_POOL
    #[implements]
    fn alloc(&mut self, layouts: &[DescriptorSetLayoutObjectRef]) -> crate::Result<Vec<DescriptorSet>> {
        let ainfo = VkDescriptorSetAllocateInfo {
            sType: VkDescriptorSetAllocateInfo::TYPE,
            pNext: core::ptr::null(),
            descriptorPool: self.native_ptr_mut(),
            descriptorSetCount: layouts.len() as _,
            pSetLayouts: layouts.as_ptr_empty_null() as _,
        };
        let mut hs = vec![VkDescriptorSet::NULL; layouts.len()];

        unsafe {
            self.alloc_raw(&ainfo, &mut hs)?;

            Ok(core::mem::transmute(hs))
        }
    }

    /// Allocate one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    /// - VK_ERROR_FRAGMENTED_POOL
    #[implements]
    fn alloc_array<const N: usize>(
        &mut self,
        layouts: &[DescriptorSetLayoutObjectRef; N],
    ) -> crate::Result<[DescriptorSet; N]> {
        let ainfo = VkDescriptorSetAllocateInfo {
            sType: VkDescriptorSetAllocateInfo::TYPE,
            pNext: core::ptr::null(),
            descriptorPool: self.native_ptr_mut(),
            descriptorSetCount: N as _,
            pSetLayouts: layouts.as_ptr_empty_null() as _,
        };
        let mut hs = [VkDescriptorSet::NULL; N];

        unsafe {
            self.alloc_raw(&ainfo, &mut hs)?;

            // Note: transmuteだと変換できない（要素数がジェネリックだとダメっぽい？）
            Ok(*(&hs as *const _ as *const [DescriptorSet; N]))
        }
    }

    /// Resets a descriptor pool object
    /// # Safety
    /// Application must not use descriptor sets after this call
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    #[inline]
    unsafe fn reset(&mut self, flags: VkDescriptorPoolResetFlags) -> crate::Result<()> {
        crate::vkfn::reset_descriptor_pool(self.device_handle(), self.native_ptr_mut(), flags)
            .into_result()
            .map(drop)
    }

    /// Free one or more descriptor sets
    /// # Safety
    /// Host access to each member of pDescriptorSets must be externally synchronized
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    #[inline]
    unsafe fn free(&mut self, sets: &[DescriptorSet]) -> crate::Result<()> {
        crate::vkfn::free_descriptor_sets(
            self.device_handle(),
            self.native_ptr(),
            sets.len() as _,
            sets.as_ptr_empty_null() as _,
        )
        .into_result()
        .map(drop)
    }
}
DerefContainerBracketImpl!(for mut DescriptorPoolMut {});
GuardsImpl!(for mut DescriptorPoolMut {});

/// Pointer for descriptor array in set
#[derive(Clone)]
pub struct DescriptorPointer {
    pub set: VkDescriptorSet,
    pub binding: u32,
    pub array_offset: u32,
}
impl DescriptorPointer {
    pub const fn new(set: VkDescriptorSet, binding: u32) -> Self {
        Self {
            set,
            binding,
            array_offset: 0,
        }
    }

    pub const fn array_offset(self, offset: u32) -> Self {
        Self {
            array_offset: offset,
            ..self
        }
    }

    pub const fn write<'r>(self, contents: DescriptorContents<'r>) -> DescriptorSetWriteInfo<'r> {
        DescriptorSetWriteInfo(self, contents)
    }

    pub const fn copy(self, count: u32, dest: DescriptorPointer) -> DescriptorSetCopyInfo {
        DescriptorSetCopyInfo(self, dest, count)
    }

    pub fn write_continuous_bindings<'r>(
        self,
        contents: impl IntoIterator<Item = DescriptorContents<'r>>,
    ) -> impl Iterator<Item = DescriptorSetWriteInfo<'r>> {
        let base_binding = self.binding;

        contents.into_iter().enumerate().map(move |(n, c)| {
            Self {
                binding: base_binding + n as u32,
                ..self.clone()
            }
            .write(c)
        })
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptorBufferRef<'r>(
    VkDescriptorBufferInfo,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = VkBuffer>>,
);
impl<'r> DescriptorBufferRef<'r> {
    #[inline(always)]
    pub fn new(r: &'r (impl VkHandle<Handle = VkBuffer> + ?Sized), range: core::ops::Range<VkDeviceSize>) -> Self {
        Self(
            VkDescriptorBufferInfo {
                buffer: r.native_ptr(),
                offset: range.start,
                range: range.end - range.start,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub const fn unbounded(h: VkBuffer, range: core::ops::Range<VkDeviceSize>) -> Self {
        Self(
            VkDescriptorBufferInfo {
                buffer: h,
                offset: range.start,
                range: range.end - range.start,
            },
            core::marker::PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptorImageRef<'r>(
    VkDescriptorImageInfo,
    core::marker::PhantomData<(
        &'r dyn VkHandle<Handle = VkImageView>,
        Option<&'r dyn VkHandle<Handle = VkSampler>>,
    )>,
);
impl<'r> DescriptorImageRef<'r> {
    #[inline(always)]
    pub fn new(r: &'r (impl VkHandle<Handle = VkImageView> + ?Sized), layout: ImageLayout) -> Self {
        Self(
            VkDescriptorImageInfo {
                imageView: r.native_ptr(),
                imageLayout: layout as _,
                sampler: VkSampler::NULL,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub const fn unbounded(h: VkImageView, layout: ImageLayout) -> Self {
        Self(
            VkDescriptorImageInfo {
                imageView: h,
                imageLayout: layout as _,
                sampler: VkSampler::NULL,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn with_sampler(mut self, sampler: &'r (impl VkHandle<Handle = VkSampler> + ?Sized)) -> Self {
        self.0.sampler = sampler.native_ptr();
        self
    }

    #[inline(always)]
    pub const fn with_unbounded_sampler(mut self, sampler: VkSampler) -> Self {
        self.0.sampler = sampler;
        self
    }
}

#[derive(Clone)]
pub enum DescriptorContents<'r> {
    Sampler(Vec<DescriptorImageRef<'r>>),
    CombinedImageSampler(Vec<DescriptorImageRef<'r>>),
    SampledImage(Vec<DescriptorImageRef<'r>>),
    StorageImage(Vec<DescriptorImageRef<'r>>),
    InputAttachment(Vec<DescriptorImageRef<'r>>),
    UniformBuffer(Vec<DescriptorBufferRef<'r>>),
    StorageBuffer(Vec<DescriptorBufferRef<'r>>),
    UniformBufferDynamic(Vec<DescriptorBufferRef<'r>>),
    StorageBufferDynamic(Vec<DescriptorBufferRef<'r>>),
    UniformTexelBuffer(Vec<VkHandleRef<'r, VkBufferView>>),
    StorageTexelBuffer(Vec<VkHandleRef<'r, VkBufferView>>),
}
impl<'d> DescriptorContents<'d> {
    pub fn type_count(&self) -> (DescriptorType, usize) {
        match self {
            Self::Sampler(rs) => (DescriptorType::Sampler, rs.len()),
            Self::CombinedImageSampler(rs) => (DescriptorType::CombinedImageSampler, rs.len()),
            Self::SampledImage(rs) => (DescriptorType::SampledImage, rs.len()),
            Self::StorageImage(rs) => (DescriptorType::StorageImage, rs.len()),
            Self::InputAttachment(rs) => (DescriptorType::InputAttachment, rs.len()),
            Self::UniformBuffer(rs) => (DescriptorType::UniformBuffer, rs.len()),
            Self::StorageBuffer(rs) => (DescriptorType::StorageBuffer, rs.len()),
            Self::UniformBufferDynamic(rs) => (DescriptorType::UniformBufferDynamic, rs.len()),
            Self::StorageBufferDynamic(rs) => (DescriptorType::StorageBufferDynamic, rs.len()),
            Self::UniformTexelBuffer(rs) => (DescriptorType::UniformTexelBuffer, rs.len()),
            Self::StorageTexelBuffer(rs) => (DescriptorType::StorageTexelBuffer, rs.len()),
        }
    }

    // single content utilities

    #[inline(always)]
    pub fn sampler(obj: &'d (impl VkHandle<Handle = VkImageView> + ?Sized), layout: ImageLayout) -> Self {
        Self::Sampler(vec![DescriptorImageRef::new(obj, layout)])
    }
    #[inline(always)]
    pub fn combined_image_sampler(
        obj: &'d (impl VkHandle<Handle = VkImageView> + ?Sized),
        layout: ImageLayout,
    ) -> Self {
        Self::CombinedImageSampler(vec![DescriptorImageRef::new(obj, layout)])
    }
    #[inline(always)]
    pub fn sampled_image(obj: &'d (impl VkHandle<Handle = VkImageView> + ?Sized), layout: ImageLayout) -> Self {
        Self::SampledImage(vec![DescriptorImageRef::new(obj, layout)])
    }
    #[inline(always)]
    pub fn storage_image(obj: &'d (impl VkHandle<Handle = VkImageView> + ?Sized), layout: ImageLayout) -> Self {
        Self::StorageImage(vec![DescriptorImageRef::new(obj, layout)])
    }
    #[inline(always)]
    pub fn input_attachment(obj: &'d (impl VkHandle<Handle = VkImageView> + ?Sized), layout: ImageLayout) -> Self {
        Self::InputAttachment(vec![DescriptorImageRef::new(obj, layout)])
    }
    #[inline(always)]
    pub fn uniform_buffer(
        obj: &'d (impl VkHandle<Handle = VkBuffer> + ?Sized),
        range: core::ops::Range<VkDeviceSize>,
    ) -> Self {
        Self::UniformBuffer(vec![DescriptorBufferRef::new(obj, range)])
    }
    #[inline(always)]
    pub fn storage_buffer(
        obj: &'d (impl VkHandle<Handle = VkBuffer> + ?Sized),
        range: core::ops::Range<VkDeviceSize>,
    ) -> Self {
        Self::StorageBuffer(vec![DescriptorBufferRef::new(obj, range)])
    }
    #[inline(always)]
    pub fn uniform_buffer_dynamic(
        obj: &'d (impl VkHandle<Handle = VkBuffer> + ?Sized),
        range: core::ops::Range<VkDeviceSize>,
    ) -> Self {
        Self::UniformBufferDynamic(vec![DescriptorBufferRef::new(obj, range)])
    }
    #[inline(always)]
    pub fn storage_buffer_dynamic(
        obj: &'d (impl VkHandle<Handle = VkBuffer> + ?Sized),
        range: core::ops::Range<VkDeviceSize>,
    ) -> Self {
        Self::StorageBufferDynamic(vec![DescriptorBufferRef::new(obj, range)])
    }
    #[inline(always)]
    pub fn uniform_texel_buffer(obj: &'d (impl VkHandle<Handle = VkBufferView> + ?Sized)) -> Self {
        Self::UniformTexelBuffer(vec![VkHandleRef::new(obj)])
    }
    #[inline(always)]
    pub fn storage_texel_buffer(obj: &'d (impl VkHandle<Handle = VkBufferView> + ?Sized)) -> Self {
        Self::StorageTexelBuffer(vec![VkHandleRef::new(obj)])
    }
}

#[derive(Clone)]
pub struct DescriptorSetWriteInfo<'s>(pub DescriptorPointer, pub DescriptorContents<'s>);
impl DescriptorSetWriteInfo<'_> {
    pub fn make_structure(&self) -> VkWriteDescriptorSet {
        let (r#type, count) = self.1.type_count();
        let (buffers, images, buffer_views) = match self.1 {
            DescriptorContents::Sampler(ref res)
            | DescriptorContents::CombinedImageSampler(ref res)
            | DescriptorContents::SampledImage(ref res)
            | DescriptorContents::StorageImage(ref res)
            | DescriptorContents::InputAttachment(ref res) => {
                (core::ptr::null(), res.as_ptr_empty_null(), core::ptr::null())
            }
            DescriptorContents::UniformBuffer(ref res)
            | DescriptorContents::StorageBuffer(ref res)
            | DescriptorContents::UniformBufferDynamic(ref res)
            | DescriptorContents::StorageBufferDynamic(ref res) => {
                (res.as_ptr_empty_null(), core::ptr::null(), core::ptr::null())
            }
            DescriptorContents::UniformTexelBuffer(ref res) | DescriptorContents::StorageTexelBuffer(ref res) => {
                (core::ptr::null(), core::ptr::null(), res.as_ptr_empty_null())
            }
        };

        VkWriteDescriptorSet {
            sType: VkWriteDescriptorSet::TYPE,
            pNext: core::ptr::null(),
            dstSet: self.0.set,
            dstBinding: self.0.binding,
            dstArrayElement: self.0.array_offset,
            descriptorType: r#type as _,
            descriptorCount: count as _,
            pImageInfo: images as _,
            pBufferInfo: buffers as _,
            pTexelBufferView: buffer_views as _,
        }
    }
}

#[derive(Clone)]
pub struct DescriptorSetCopyInfo(pub DescriptorPointer, pub DescriptorPointer, u32);
impl DescriptorSetCopyInfo {
    pub fn make_structure(&self) -> VkCopyDescriptorSet {
        VkCopyDescriptorSet {
            sType: VkCopyDescriptorSet::TYPE,
            pNext: core::ptr::null(),
            srcSet: self.0.set,
            srcBinding: self.0.binding,
            srcArrayElement: self.0.array_offset,
            dstSet: self.1.set,
            dstBinding: self.1.binding,
            dstArrayElement: self.1.array_offset,
            descriptorCount: self.2,
        }
    }
}

#[macro_export]
macro_rules! DescriptorUpdateTemplateEntry {
    { ($b: expr, $a: expr) .. : [$ty: expr; $c: expr] = $o: expr, $s: expr } => {
        VkDescriptorUpdateTemplateEntry
        {
            descriptorType: $ty, descriptorCount: $c,
            dstBinding: $b, dstArrayElement: $a, offset: $o, stride: $s
        }
    };
}
#[macro_export]
macro_rules! DescriptorUpdateTemplateEntries {
    { { $(($b: expr, $a: expr) .. : [$ty: expr; $c: expr] = $o: expr, $s: expr),* } } => { {
        $(DescriptorUpdateTemplateEntry! { ($b, $a) ..: [$ty; $c] = $o, $s }),*
    } };
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_descriptor_update_template")] {
        #[derive(VkHandle, VkObject)]
        #[VkObject(type = VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR)]
        pub struct DescriptorUpdateTemplateObject<Device: crate::Device>(
            pub(crate) VkDescriptorUpdateTemplateKHR,
            pub(crate) Device,
        );
        #[implements]
        impl<Device: crate::Device> Drop for DescriptorUpdateTemplateObject<Device> {
            fn drop(&mut self) {
                unsafe { self.1.destroy_descriptor_update_template_khr_fn().0(self.1.native_ptr(), self.0, core::ptr::null()); }
            }
        }
        unsafe impl<Device: crate::Device + Sync> Sync for DescriptorUpdateTemplateObject<Device> {}
        unsafe impl<Device: crate::Device + Send> Send for DescriptorUpdateTemplateObject<Device> {}
        impl<Device: crate::Device> DeviceChildHandle for DescriptorUpdateTemplateObject<Device> {
            #[inline(always)]
            fn device_handle(&self) -> VkDevice {
                self.1.native_ptr()
            }
        }
        impl<Device: crate::Device> DeviceChild for DescriptorUpdateTemplateObject<Device> {
            type ConcreteDevice = Device;

            #[inline(always)]
            fn device(&self) -> &Self::ConcreteDevice {
                &self.1
            }
        }
        impl<Device: crate::Device> DescriptorUpdateTemplate for DescriptorUpdateTemplateObject<Device> {}

        impl<Device: crate::Device> DescriptorUpdateTemplateObject<Device> {
            /// Constructs from raw values
            /// # Safety
            /// the resource must be created from the device and not freed anywhere
            pub const unsafe fn manage(handle: VkDescriptorUpdateTemplateKHR, parent: Device) -> Self {
                Self(handle, parent)
            }

            /// Purges the construct (Drop will not be called for this resource)
            pub fn unmanage(self) -> (VkDescriptorUpdateTemplateKHR, Device) {
                let h = self.0;
                let p = unsafe { core::ptr::read(&self.1) };
                core::mem::forget(self);

                (h, p)
            }
        }

        pub trait DescriptorUpdateTemplate: VkHandle<Handle = VkDescriptorUpdateTemplateKHR> + DeviceChild {
            #[implements]
            fn update_set<T>(&self, set: VkDescriptorSet, data: &T) {
                use crate::Device;

                unsafe {
                    self.device().update_descriptor_set_with_template_khr_fn().0(
                        self.device().native_ptr(),
                        set,
                        self.native_ptr(),
                        data as *const T as *const _,
                    )
                }
            }
        }
        DerefContainerBracketImpl!(for DescriptorUpdateTemplate {});
        GuardsImpl!(for DescriptorUpdateTemplate {});
    }
}
