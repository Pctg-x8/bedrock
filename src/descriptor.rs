//! Vulkan Descriptors
use bedrock_vk::{self as brvk, TypedVulkanStructure, VkRawHandle};

use derives::implements;

use crate::error::translate_vk_result;
use crate::ffi_helper::{ArrayFFIExtensions, slice_as_ptr_empty_null};
use crate::*;

/// Opaque handle to a descriptor set layout object
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VkDescriptorSetLayout::OBJECT_TYPE)]
pub struct DescriptorSetLayoutObject<Device: VkHandle<Handle = brvk::VkDevice>>(
    pub(crate) brvk::VkDescriptorSetLayout,
    pub(crate) Device,
);
#[implements]
impl<Device: VkHandle<Handle = brvk::VkDevice>> Drop for DescriptorSetLayoutObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            brvk::fns::destroy_descriptor_set_layout(self.1.native_ptr(), self.0, core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Sync> Sync for DescriptorSetLayoutObject<Device> {}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Send> Send for DescriptorSetLayoutObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceChildHandle for DescriptorSetLayoutObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice {
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
impl<Device: VkHandle<Handle = brvk::VkDevice>> DescriptorSetLayoutObject<Device> {
    /// Create a new descriptor set layout
    /// # Failures
    /// On failure, this command returns
    ///
    /// - [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// - [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    pub fn new(device: Device, info: &DescriptorSetLayoutCreateInfo) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        translate_vk_result(unsafe {
            brvk::fns::create_descriptor_set_layout(device.native_ptr(), &info.0, core::ptr::null(), h.as_mut_ptr())
        })?;

        Ok(Self(unsafe { h.assume_init() }, device))
    }

    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: brvk::VkDescriptorSetLayout, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (brvk::VkDescriptorSetLayout, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice> + Clone> DescriptorSetLayoutObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> DescriptorSetLayoutObject<Device> {
        let r = DescriptorSetLayoutObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}

#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VkDescriptorPool::OBJECT_TYPE)]
pub struct DescriptorPoolObject<Device: VkHandle<Handle = brvk::VkDevice>>(
    pub(crate) brvk::VkDescriptorPool,
    pub(crate) Device,
);
#[implements]
impl<Device: VkHandle<Handle = brvk::VkDevice>> Drop for DescriptorPoolObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            brvk::fns::destroy_descriptor_pool(self.1.native_ptr(), self.0, core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Sync> Sync for DescriptorPoolObject<Device> {}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Send> Send for DescriptorPoolObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceChildHandle for DescriptorPoolObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice {
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
impl<Device: VkHandle<Handle = brvk::VkDevice>> DescriptorPool for DescriptorPoolObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DescriptorPoolMut for DescriptorPoolObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DescriptorPoolObject<Device> {
    /// Creates a descriptor pool object
    /// # Failures
    /// On failure, this command returns
    ///
    /// - brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// - brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    pub fn new(device: Device, info: &DescriptorPoolCreateInfo) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        translate_vk_result(unsafe {
            brvk::fns::create_descriptor_pool(device.native_ptr(), &info.0, core::ptr::null(), h.as_mut_ptr())
        })?;

        Ok(Self(unsafe { h.assume_init() }, device))
    }

    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: brvk::VkDescriptorPool, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (brvk::VkDescriptorPool, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice> + Clone> DescriptorPoolObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> DescriptorPoolObject<Device> {
        let r = DescriptorPoolObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DescriptorSet(pub(crate) brvk::VkDescriptorSet);
unsafe impl Sync for DescriptorSet {}
unsafe impl Send for DescriptorSet {}
impl DescriptorSet {
    #[inline]
    pub const fn binding_at(&self, b: u32) -> DescriptorPointer {
        DescriptorPointer::new(self.0, b)
    }
}

/// Specified the type of a descriptor in a descriptor set
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Hash)]
pub enum DescriptorType {
    Sampler = brvk::VK_DESCRIPTOR_TYPE_SAMPLER as _,
    CombinedImageSampler = brvk::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER as _,
    SampledImage = brvk::VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE as _,
    StorageImage = brvk::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE as _,
    UniformTexelBuffer = brvk::VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER as _,
    StorageTexelBuffer = brvk::VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER as _,
    UniformBuffer = brvk::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER as _,
    StorageBuffer = brvk::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER as _,
    UniformBufferDynamic = brvk::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC as _,
    StorageBufferDynamic = brvk::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC as _,
    InputAttachment = brvk::VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT as _,
}
impl DescriptorType {
    pub const fn make_size(self, count: u32) -> brvk::VkDescriptorPoolSize {
        brvk::VkDescriptorPoolSize {
            r#type: self as _,
            descriptorCount: count,
        }
    }

    pub const fn make_binding<'a>(self, binding: u32, count: u32) -> DescriptorSetLayoutBinding<'a> {
        DescriptorSetLayoutBinding::new(binding, self, count, brvk::VK_SHADER_STAGE_ALL)
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DescriptorSetLayoutBinding<'s> {
    raw: brvk::VkDescriptorSetLayoutBinding,
    immutable_samplers: core::marker::PhantomData<&'s dyn VkHandle<Handle = brvk::VkSampler>>,
}
impl<'s> DescriptorSetLayoutBinding<'s> {
    pub const fn new(binding: u32, r#type: DescriptorType, count: u32, shader_stage: brvk::VkShaderStageFlags) -> Self {
        Self {
            raw: brvk::VkDescriptorSetLayoutBinding {
                binding,
                descriptorType: r#type as _,
                descriptorCount: count,
                stageFlags: shader_stage,
                pImmutableSamplers: core::ptr::null(),
            },
            immutable_samplers: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn with_immutable_samplers(self, samplers: &'s [VkHandleRef<brvk::VkSampler>]) -> Self {
        assert_eq!(samplers.len(), self.raw.descriptorCount as usize);
        unsafe { self.with_immutable_samplers_unchecked(samplers) }
    }

    /// # Safety
    ///
    /// The caller must ensure that a number of `samplers` is equal to the descriptor's count.
    pub const unsafe fn with_immutable_samplers_unchecked(
        mut self,
        samplers: &'s [VkHandleRef<brvk::VkSampler>],
    ) -> Self {
        self.raw.pImmutableSamplers = slice_as_ptr_empty_null(samplers) as _;
        self
    }

    pub const fn for_shader_stage(mut self, mask: brvk::VkShaderStageFlags) -> Self {
        self.raw.stageFlags = mask;
        self
    }

    pub const fn only_for_vertex(self) -> Self {
        self.for_shader_stage(brvk::VK_SHADER_STAGE_VERTEX_BIT)
    }

    pub const fn only_for_tess_control(self) -> Self {
        self.for_shader_stage(brvk::VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT)
    }

    pub const fn only_for_tess_evaluation(self) -> Self {
        self.for_shader_stage(brvk::VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT)
    }

    pub const fn only_for_tessellation(self) -> Self {
        self.for_shader_stage(
            brvk::VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT | brvk::VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
        )
    }

    pub const fn only_for_geometry(self) -> Self {
        self.for_shader_stage(brvk::VK_SHADER_STAGE_GEOMETRY_BIT)
    }

    pub const fn only_for_fragment(self) -> Self {
        self.for_shader_stage(brvk::VK_SHADER_STAGE_FRAGMENT_BIT)
    }

    pub const fn only_for_compute(self) -> Self {
        self.for_shader_stage(brvk::VK_SHADER_STAGE_COMPUTE_BIT)
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DescriptorSetLayoutCreateInfo<'d, 's>(
    brvk::VkDescriptorSetLayoutCreateInfo,
    core::marker::PhantomData<&'d [DescriptorSetLayoutBinding<'s>]>,
);
impl<'d, 's> DescriptorSetLayoutCreateInfo<'d, 's> {
    #[inline(always)]
    pub const fn new(bindings: &'d [DescriptorSetLayoutBinding<'s>]) -> Self {
        Self(
            brvk::VkDescriptorSetLayoutCreateInfo {
                sType: brvk::VkDescriptorSetLayoutCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                bindingCount: bindings.len() as _,
                pBindings: slice_as_ptr_empty_null(bindings) as _,
            },
            core::marker::PhantomData,
        )
    }
}

pub trait DescriptorSetLayout: VkHandle<Handle = brvk::VkDescriptorSetLayout> + DeviceChild {}
DerefContainerBracketImpl!(for DescriptorSetLayout {});
GuardsImpl!(for DescriptorSetLayout {});

/*
# DescriptorPoolのフラグメンテーションについてメモ(from `brvk::VkDescriptorPoolCreateInfo` Manual)

`brvk::VkDescriptorPoolSize`構造体が`pPoolSizes`配列内に複数ある場合、プールはそれぞれのタイプの合計分のデスクリプタが十分入るように確保されます。

DescriptorPoolはフラグメンテーションを起こすことがあり、DescriptorSetの確保に失敗することがあります。
フラグメンテーションに起因する失敗は、確保したDescriptorSetの数+確保を要求したDescriptorSetの数が`maxSets`に満たない場合でも
"DescriptorSetの確保の失敗"と定義されます。(たぶんあってるはず)
実装は、以下に記述されるような"フラグメンテーションが確保の失敗を引き起こさない場合"について確固たる保証を提供します。
(言い換えると、「以下に示す場合はフラグメンテーション状態でも確保に成功する」)

DescriptorPoolが、生成されてから/間近にリセットされてから今までに開放されたDescriptorSetがない場合、
フラグメンテーションは確保の失敗を引き起こしません。(`brvk::VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT`を伴わずに生成されたプールに対しては常に満たすものとします？)
また、
- プールが生成されてから/間近にリセットされてから確保された、すべてのDescriptorSetが各タイプ同じ数のDescriptorを使う場合、そして
- 要求した確保も各タイプ同じ数のDescriptorを使う場合、
フラグメンテーションは確保の失敗を引き起こしません。

もしフラグメンテーションによって確保が失敗した場合、アプリケーションは続けてDescriptorSetの確保を行うために追加のDescriptorPoolを生成することができます
*/

#[repr(transparent)]
#[derive(Clone)]
pub struct DescriptorPoolCreateInfo<'d>(
    brvk::VkDescriptorPoolCreateInfo,
    core::marker::PhantomData<&'d [brvk::VkDescriptorPoolSize]>,
);
impl<'d> DescriptorPoolCreateInfo<'d> {
    pub const fn new(max_sets: u32, sizes: &'d [brvk::VkDescriptorPoolSize]) -> Self {
        Self(
            brvk::VkDescriptorPoolCreateInfo {
                sType: brvk::VkDescriptorPoolCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                maxSets: max_sets,
                poolSizeCount: sizes.len() as _,
                pPoolSizes: slice_as_ptr_empty_null(sizes),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid `brvk::VkDescriptorPoolCreateInfo` value.
    pub const unsafe fn from_raw(raw: brvk::VkDescriptorPoolCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkDescriptorPoolCreateInfo {
        self.0
    }

    pub const fn allow_individual_free(mut self) -> Self {
        self.0.flags |= brvk::VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT;
        self
    }
}

pub trait DescriptorPool: VkHandle<Handle = brvk::VkDescriptorPool> {}
DerefContainerBracketImpl!(for DescriptorPool {});
GuardsImpl!(for DescriptorPool {});

pub trait DescriptorPoolMut: DescriptorPool + VkHandleMut + DeviceChildHandle {
    /// Allocate one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// - brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    /// - brvk::VK_ERROR_FRAGMENTED_POOL
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    #[inline]
    unsafe fn alloc_raw(
        &mut self,
        info: &brvk::VkDescriptorSetAllocateInfo,
        objects: &mut [core::mem::MaybeUninit<brvk::VkDescriptorSet>],
    ) -> crate::Result<()> {
        translate_vk_result(unsafe {
            brvk::fns::allocate_descriptor_sets(self.device_handle(), info, objects.as_mut_ptr().cast())
        })?;

        Ok(())
    }

    /// Allocate one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// - brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    /// - brvk::VK_ERROR_FRAGMENTED_POOL
    #[implements]
    #[cfg(feature = "alloc")]
    fn alloc(&mut self, layouts: &[VkHandleRef<brvk::VkDescriptorSetLayout>]) -> crate::Result<Vec<DescriptorSet>> {
        let ainfo = brvk::VkDescriptorSetAllocateInfo {
            sType: brvk::VkDescriptorSetAllocateInfo::TYPE,
            pNext: core::ptr::null(),
            descriptorPool: self.native_ptr_mut(),
            descriptorSetCount: layouts.len() as _,
            pSetLayouts: layouts.as_ptr_empty_null() as _,
        };
        let mut hs = crate::alloc::empty_reserved_buffer(layouts.len());

        unsafe {
            self.alloc_raw(
                &ainfo,
                core::mem::transmute::<
                    &mut [core::mem::MaybeUninit<DescriptorSet>],
                    &mut [core::mem::MaybeUninit<brvk::VkDescriptorSet>],
                >(hs.spare_capacity_mut()),
            )?;
            hs.set_len(layouts.len())
        }

        Ok(hs)
    }

    /// Allocate one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// - brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    /// - brvk::VK_ERROR_FRAGMENTED_POOL
    #[implements]
    fn alloc_array<const N: usize>(
        &mut self,
        layouts: &[VkHandleRef<brvk::VkDescriptorSetLayout>; N],
    ) -> crate::Result<[DescriptorSet; N]> {
        let ainfo = brvk::VkDescriptorSetAllocateInfo {
            sType: brvk::VkDescriptorSetAllocateInfo::TYPE,
            pNext: core::ptr::null(),
            descriptorPool: self.native_ptr_mut(),
            descriptorSetCount: N as _,
            pSetLayouts: layouts.as_ptr_empty_null() as _,
        };

        unsafe {
            let mut hs = [core::mem::MaybeUninit::uninit().assume_init(); N];
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
    /// - brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// - brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    #[inline]
    unsafe fn reset(&mut self, flags: brvk::VkDescriptorPoolResetFlags) -> crate::Result<()> {
        translate_vk_result(unsafe {
            brvk::fns::reset_descriptor_pool(self.device_handle(), self.native_ptr_mut(), flags)
        })?;

        Ok(())
    }

    /// Free one or more descriptor sets
    /// # Safety
    /// Host access to each member of pDescriptorSets must be externally synchronized
    /// # Failures
    /// On failure, this command returns
    /// - brvk::VK_ERROR_OUT_OF_HOST_MEMORY
    /// - brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    #[inline]
    unsafe fn free(&mut self, sets: &[DescriptorSet]) -> crate::Result<()> {
        translate_vk_result(unsafe {
            brvk::fns::free_descriptor_sets(
                self.device_handle(),
                self.native_ptr(),
                sets.len() as _,
                sets.as_ptr_empty_null().cast(),
            )
        })?;

        Ok(())
    }
}
DerefContainerBracketImpl!(for mut DescriptorPoolMut {});
GuardsImpl!(for mut DescriptorPoolMut {});

/// Pointer for descriptor array in set
#[derive(Clone)]
pub struct DescriptorPointer {
    pub set: brvk::VkDescriptorSet,
    pub binding: u32,
    pub array_offset: u32,
}
impl DescriptorPointer {
    pub const fn new(set: brvk::VkDescriptorSet, binding: u32) -> Self {
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
#[derive(Clone)]
pub struct DescriptorBufferInfo<'r>(
    brvk::VkDescriptorBufferInfo,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = brvk::VkBuffer>>,
);
impl<'r> DescriptorBufferInfo<'r> {
    #[inline(always)]
    pub fn new(
        r: &'r (impl VkHandle<Handle = brvk::VkBuffer> + ?Sized),
        range: core::ops::Range<brvk::VkDeviceSize>,
    ) -> Self {
        Self(
            brvk::VkDescriptorBufferInfo {
                buffer: r.native_ptr(),
                offset: range.start,
                range: range.end - range.start,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub const fn unbounded(h: brvk::VkBuffer, range: core::ops::Range<brvk::VkDeviceSize>) -> Self {
        Self(
            brvk::VkDescriptorBufferInfo {
                buffer: h,
                offset: range.start,
                range: range.end - range.start,
            },
            core::marker::PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DescriptorImageInfo<'r>(
    brvk::VkDescriptorImageInfo,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        &'r dyn VkHandle<Handle = brvk::VkImageView>,
        Option<&'r dyn VkHandle<Handle = brvk::VkSampler>>,
    )>,
);
impl<'r> DescriptorImageInfo<'r> {
    #[inline(always)]
    pub fn new(r: &'r (impl VkHandle<Handle = brvk::VkImageView> + ?Sized), layout: ImageLayout) -> Self {
        Self(
            brvk::VkDescriptorImageInfo {
                imageView: r.native_ptr(),
                imageLayout: layout as _,
                sampler: None,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub const fn unbounded(h: brvk::VkImageView, layout: ImageLayout) -> Self {
        Self(
            brvk::VkDescriptorImageInfo {
                imageView: h,
                imageLayout: layout as _,
                sampler: None,
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn with_sampler(mut self, sampler: &'r (impl VkHandle<Handle = brvk::VkSampler> + ?Sized)) -> Self {
        self.0.sampler = Some(sampler.native_ptr());
        self
    }

    #[inline(always)]
    pub const fn with_unbounded_sampler(mut self, sampler: brvk::VkSampler) -> Self {
        self.0.sampler = Some(sampler);
        self
    }
}

#[derive(Clone)]
pub enum DescriptorContents<'r> {
    Sampler(Vec<DescriptorImageInfo<'r>>),
    CombinedImageSampler(Vec<DescriptorImageInfo<'r>>),
    SampledImage(Vec<DescriptorImageInfo<'r>>),
    StorageImage(Vec<DescriptorImageInfo<'r>>),
    InputAttachment(Vec<DescriptorImageInfo<'r>>),
    UniformBuffer(Vec<DescriptorBufferInfo<'r>>),
    StorageBuffer(Vec<DescriptorBufferInfo<'r>>),
    UniformBufferDynamic(Vec<DescriptorBufferInfo<'r>>),
    StorageBufferDynamic(Vec<DescriptorBufferInfo<'r>>),
    UniformTexelBuffer(Vec<VkHandleRef<'r, brvk::VkBufferView>>),
    StorageTexelBuffer(Vec<VkHandleRef<'r, brvk::VkBufferView>>),
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
    pub fn sampler(obj: &'d (impl VkHandle<Handle = brvk::VkImageView> + ?Sized), layout: ImageLayout) -> Self {
        Self::Sampler(vec![DescriptorImageInfo::new(obj, layout)])
    }
    #[inline(always)]
    pub fn combined_image_sampler(
        obj: &'d (impl VkHandle<Handle = brvk::VkImageView> + ?Sized),
        layout: ImageLayout,
    ) -> Self {
        Self::CombinedImageSampler(vec![DescriptorImageInfo::new(obj, layout)])
    }
    #[inline(always)]
    pub fn sampled_image(obj: &'d (impl VkHandle<Handle = brvk::VkImageView> + ?Sized), layout: ImageLayout) -> Self {
        Self::SampledImage(vec![DescriptorImageInfo::new(obj, layout)])
    }
    #[inline(always)]
    pub fn storage_image(obj: &'d (impl VkHandle<Handle = brvk::VkImageView> + ?Sized), layout: ImageLayout) -> Self {
        Self::StorageImage(vec![DescriptorImageInfo::new(obj, layout)])
    }
    #[inline(always)]
    pub fn input_attachment(
        obj: &'d (impl VkHandle<Handle = brvk::VkImageView> + ?Sized),
        layout: ImageLayout,
    ) -> Self {
        Self::InputAttachment(vec![DescriptorImageInfo::new(obj, layout)])
    }
    #[inline(always)]
    pub fn uniform_buffer(
        obj: &'d (impl VkHandle<Handle = brvk::VkBuffer> + ?Sized),
        range: core::ops::Range<brvk::VkDeviceSize>,
    ) -> Self {
        Self::UniformBuffer(vec![DescriptorBufferInfo::new(obj, range)])
    }
    #[inline(always)]
    pub fn storage_buffer(
        obj: &'d (impl VkHandle<Handle = brvk::VkBuffer> + ?Sized),
        range: core::ops::Range<brvk::VkDeviceSize>,
    ) -> Self {
        Self::StorageBuffer(vec![DescriptorBufferInfo::new(obj, range)])
    }
    #[inline(always)]
    pub fn uniform_buffer_dynamic(
        obj: &'d (impl VkHandle<Handle = brvk::VkBuffer> + ?Sized),
        range: core::ops::Range<brvk::VkDeviceSize>,
    ) -> Self {
        Self::UniformBufferDynamic(vec![DescriptorBufferInfo::new(obj, range)])
    }
    #[inline(always)]
    pub fn storage_buffer_dynamic(
        obj: &'d (impl VkHandle<Handle = brvk::VkBuffer> + ?Sized),
        range: core::ops::Range<brvk::VkDeviceSize>,
    ) -> Self {
        Self::StorageBufferDynamic(vec![DescriptorBufferInfo::new(obj, range)])
    }
    #[inline(always)]
    pub fn uniform_texel_buffer(obj: &'d (impl VkHandle<Handle = brvk::VkBufferView> + ?Sized)) -> Self {
        Self::UniformTexelBuffer(vec![VkHandleRef::new(obj)])
    }
    #[inline(always)]
    pub fn storage_texel_buffer(obj: &'d (impl VkHandle<Handle = brvk::VkBufferView> + ?Sized)) -> Self {
        Self::StorageTexelBuffer(vec![VkHandleRef::new(obj)])
    }
}

#[derive(Clone)]
pub struct DescriptorSetWriteInfo<'s>(pub DescriptorPointer, pub DescriptorContents<'s>);
impl DescriptorSetWriteInfo<'_> {
    pub fn make_structure(&self) -> brvk::VkWriteDescriptorSet {
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

        brvk::VkWriteDescriptorSet {
            sType: brvk::VkWriteDescriptorSet::TYPE,
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
    pub fn make_structure(&self) -> brvk::VkCopyDescriptorSet {
        brvk::VkCopyDescriptorSet {
            sType: brvk::VkCopyDescriptorSet::TYPE,
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
        brvk::VkDescriptorUpdateTemplateEntry
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

#[cfg(feature = "VK_KHR_descriptor_update_template")]
mod update_template;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub use self::update_template::*;
