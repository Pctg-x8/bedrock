//! Vulkan Descriptors

use derives::{bitflags_newtype, implements};

use crate::ffi_helper::{ArrayFFIExtensions, opt_pointer, slice_as_ptr_empty_null};
use crate::{
    Buffer, BufferView, Device, DeviceSize, ImageLayout, ImageView, Sampler, ShaderStageFlags, VkObject, VkRawHandle,
    VulkanStructure, vk::*,
};
use core::{marker::PhantomData, ops::Range};

/// Opaque handle to a descriptor set layout object.
#[repr(transparent)]
pub struct DescriptorSetLayout(VkDescriptorSetLayout_T);
#[implements]
impl DescriptorSetLayout {
    /// Destroy a descriptor set layout object.
    pub unsafe fn destroy(&mut self, device: &Device, allocation_callbacks: Option<&VkAllocationCallbacks>) {
        unsafe {
            crate::vkfn::destroy_descriptor_set_layout(
                device as *const _ as _,
                self as *mut _ as _,
                opt_pointer(allocation_callbacks),
            )
        }
    }
}

/// Opaque handle to a descriptor pool object.
#[repr(transparent)]
pub struct DescriptorPool(VkDescriptorPool_T);
#[implements]
impl DescriptorPool {
    /// Destroy a descriptor pool object.
    #[inline]
    pub unsafe fn destroy(&mut self, device: &Device, allocation_callbacks: Option<&VkAllocationCallbacks>) {
        unsafe {
            crate::vkfn::destroy_descriptor_pool(
                device as *const _ as _,
                self as *mut _ as _,
                opt_pointer(allocation_callbacks),
            )
        }
    }

    /// Resets a descriptor pool object
    /// # Safety
    /// Application must not use descriptor sets after this call
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[inline]
    pub unsafe fn reset(&mut self, device: &Device, flags: DescriptorPoolResetFlags) -> crate::Result<()> {
        unsafe {
            crate::vkfn::reset_descriptor_pool(device as *const _ as _, self as *mut _ as _, flags)
                .into_result()
                .map(drop)
        }
    }

    /// Free one or more descriptor sets
    /// # Safety
    /// Host access to each member of pDescriptorSets must be externally synchronized
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[inline]
    pub unsafe fn free(&mut self, device: &Device, sets: &[&mut DescriptorSet]) -> crate::Result<()> {
        unsafe {
            crate::vkfn::free_descriptor_sets(
                device as *const _ as _,
                self as *mut _ as _,
                sets.len() as _,
                slice_as_ptr_empty_null(sets) as _,
            )
            .into_result()
            .map(drop)
        }
    }
}

/// Reserved for future use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct DescriptorPoolResetFlags(VkDescriptorPoolResetFlags);
impl DescriptorPoolResetFlags {
    /// Empty bits.
    pub const EMPTY: Self = Self(0);
}

/// Opaque handle to a descriptor set object.
#[repr(transparent)]
pub struct DescriptorSet(VkDescriptorSet_T);
impl DescriptorSet {
    #[inline]
    pub const fn binding_at(&self, b: u32) -> DescriptorPointer {
        DescriptorPointer::new(self.0, b)
    }
}

pub type DescriptorPoolSize = VkDescriptorPoolSize;

/// Specified the type of a descriptor in a descriptor set
#[repr(i32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Hash)]
pub enum DescriptorType {
    Sampler = VK_DESCRIPTOR_TYPE_SAMPLER,
    CombinedImageSampler = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
    SampledImage = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
    StorageImage = VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
    UniformTexelBuffer = VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER,
    StorageTexelBuffer = VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER,
    UniformBuffer = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
    StorageBuffer = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
    UniformBufferDynamic = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC,
    StorageBufferDynamic = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC,
    InputAttachment = VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT,
}
impl DescriptorType {
    pub const fn make_size(self, count: u32) -> DescriptorPoolSize {
        DescriptorPoolSize {
            _type: self as _,
            descriptorCount: count,
        }
    }

    pub const fn make_binding<'a, 'a2>(self, binding: u32, count: u32) -> DescriptorSetLayoutBinding<'a, 'a2> {
        DescriptorSetLayoutBinding::new(binding, self, count, ShaderStageFlags::ALL)
    }
}

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DescriptorSetLayoutBinding<'s, 'ss> {
    raw: VkDescriptorSetLayoutBinding,
    immutable_samplers: PhantomData<&'ss [&'s Sampler]>,
}
impl<'s, 'ss> DescriptorSetLayoutBinding<'s, 'ss> {
    pub const fn new(binding: u32, r#type: DescriptorType, count: u32, shader_stage: ShaderStageFlags) -> Self {
        Self {
            raw: VkDescriptorSetLayoutBinding {
                binding,
                descriptorType: r#type as _,
                descriptorCount: count,
                stageFlags: shader_stage.bits(),
                pImmutableSamplers: core::ptr::null(),
            },
            immutable_samplers: PhantomData,
        }
    }

    #[inline(always)]
    pub fn with_immutable_samplers(self, samplers: &'ss [&'s Sampler]) -> Self {
        assert_eq!(samplers.len(), self.raw.descriptorCount as usize);
        unsafe { self.with_immutable_samplers_unchecked(samplers) }
    }

    pub const unsafe fn with_immutable_samplers_unchecked(mut self, samplers: &'ss [&'s Sampler]) -> Self {
        self.raw.pImmutableSamplers = slice_as_ptr_empty_null(samplers) as _;
        self
    }

    pub const fn for_shader_stage(mut self, mask: ShaderStageFlags) -> Self {
        self.raw.stageFlags = mask.bits();
        self
    }

    pub const fn only_for_vertex(self) -> Self {
        self.for_shader_stage(ShaderStageFlags::VERTEX)
    }

    pub const fn only_for_tess_control(self) -> Self {
        self.for_shader_stage(ShaderStageFlags::TESSELLATION_CONTROL)
    }

    pub const fn only_for_tess_evaluation(self) -> Self {
        self.for_shader_stage(ShaderStageFlags::TESSELLATION_EVALUATION)
    }

    pub const fn only_for_tessellation(self) -> Self {
        self.for_shader_stage(ShaderStageFlags::TESSELLATION_CONTROL | ShaderStageFlags::TESSELLATION_EVALUATION)
    }

    pub const fn only_for_geometry(self) -> Self {
        self.for_shader_stage(ShaderStageFlags::GEOMETRY)
    }

    pub const fn only_for_fragment(self) -> Self {
        self.for_shader_stage(ShaderStageFlags::FRAGMENT)
    }

    pub const fn only_for_compute(self) -> Self {
        self.for_shader_stage(ShaderStageFlags::COMPUTE)
    }
}

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct DescriptorSetLayoutCreateInfo<'d, 's, 'ss>(
    VkDescriptorSetLayoutCreateInfo,
    PhantomData<&'d [DescriptorSetLayoutBinding<'s, 'ss>]>,
);
impl<'d, 's, 'ss> DescriptorSetLayoutCreateInfo<'d, 's, 'ss> {
    #[inline(always)]
    pub const fn new(bindings: &'d [DescriptorSetLayoutBinding<'s, 'ss>]) -> Self {
        Self(
            VkDescriptorSetLayoutCreateInfo {
                sType: VkDescriptorSetLayoutCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                bindingCount: bindings.len() as _,
                pBindings: slice_as_ptr_empty_null(bindings) as _,
            },
            PhantomData,
        )
    }
}

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
pub struct DescriptorPoolCreateInfo<'d>(VkDescriptorPoolCreateInfo, PhantomData<&'d [DescriptorPoolSize]>);
impl<'d> DescriptorPoolCreateInfo<'d> {
    pub const fn new(max_sets: u32, sizes: &'d [DescriptorPoolSize]) -> Self {
        Self(
            VkDescriptorPoolCreateInfo {
                sType: VkDescriptorPoolCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                maxSets: max_sets,
                poolSizeCount: sizes.len() as _,
                pPoolSizes: slice_as_ptr_empty_null(sizes),
            },
            PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkDescriptorPoolCreateInfo) -> Self {
        Self(raw, PhantomData)
    }

    pub const fn into_raw(self) -> VkDescriptorPoolCreateInfo {
        self.0
    }

    pub const fn allow_individual_free(mut self) -> Self {
        self.0.flags |= VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT;
        self
    }
}

/// Pointer for descriptor array in set
#[derive(Clone)]
pub struct DescriptorPointer<'a> {
    pub set: &'a DescriptorSet,
    pub binding: u32,
    pub array_offset: u32,
}
impl<'a> DescriptorPointer<'a> {
    pub const fn new(set: &'a DescriptorSet, binding: u32) -> Self {
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

    pub const fn write<'r>(self, contents: DescriptorContents<'r>) -> DescriptorSetWriteInfo<'a, 'r> {
        DescriptorSetWriteInfo(self, contents)
    }

    pub const fn copy(self, count: u32, dest: DescriptorPointer) -> DescriptorSetCopyInfo {
        DescriptorSetCopyInfo(self, dest, count)
    }

    pub fn write_continuous_bindings<'r>(
        self,
        contents: impl IntoIterator<Item = DescriptorContents<'r>>,
    ) -> impl Iterator<Item = DescriptorSetWriteInfo<'a, 'r>> {
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
pub struct DescriptorBufferInfo<'r>(VkDescriptorBufferInfo, PhantomData<&'r Buffer>);
impl<'r> DescriptorBufferInfo<'r> {
    pub const fn new(r: &'r Buffer, range: Range<DeviceSize>) -> Self {
        Self(
            VkDescriptorBufferInfo {
                buffer: r as *const _ as _,
                offset: range.start,
                range: range.end - range.start,
            },
            PhantomData,
        )
    }

    pub const fn unbounded(h: VkBuffer, range: Range<DeviceSize>) -> Self {
        Self(
            VkDescriptorBufferInfo {
                buffer: h,
                offset: range.start,
                range: range.end - range.start,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptorImageInfo<'r>(VkDescriptorImageInfo, PhantomData<(&'r ImageView, Option<&'r Sampler>)>);
impl<'r> DescriptorImageInfo<'r> {
    pub const fn new(r: &'r ImageView, layout: ImageLayout) -> Self {
        Self(
            VkDescriptorImageInfo {
                imageView: r as *const _ as _,
                imageLayout: layout as _,
                sampler: VkSampler::NULL,
            },
            PhantomData,
        )
    }

    pub const fn unbounded(h: VkImageView, layout: ImageLayout) -> Self {
        Self(
            VkDescriptorImageInfo {
                imageView: h,
                imageLayout: layout as _,
                sampler: VkSampler::NULL,
            },
            PhantomData,
        )
    }

    #[inline(always)]
    pub const fn with_sampler(mut self, sampler: &'r Sampler) -> Self {
        self.0.sampler = sampler as *const _ as _;
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
    Sampler(Vec<DescriptorImageInfo<'r>>),
    CombinedImageSampler(Vec<DescriptorImageInfo<'r>>),
    SampledImage(Vec<DescriptorImageInfo<'r>>),
    StorageImage(Vec<DescriptorImageInfo<'r>>),
    InputAttachment(Vec<DescriptorImageInfo<'r>>),
    UniformBuffer(Vec<DescriptorBufferInfo<'r>>),
    StorageBuffer(Vec<DescriptorBufferInfo<'r>>),
    UniformBufferDynamic(Vec<DescriptorBufferInfo<'r>>),
    StorageBufferDynamic(Vec<DescriptorBufferInfo<'r>>),
    UniformTexelBuffer(Vec<&'r BufferView>),
    StorageTexelBuffer(Vec<&'r BufferView>),
}
impl<'d> DescriptorContents<'d> {
    pub const fn type_count(&self) -> (DescriptorType, usize) {
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
    pub fn sampler(obj: &'d ImageView, layout: ImageLayout) -> Self {
        Self::Sampler(vec![DescriptorImageInfo::new(obj, layout)])
    }
    #[inline(always)]
    pub fn combined_image_sampler(obj: &'d ImageView, layout: ImageLayout) -> Self {
        Self::CombinedImageSampler(vec![DescriptorImageInfo::new(obj, layout)])
    }
    #[inline(always)]
    pub fn sampled_image(obj: &'d ImageView, layout: ImageLayout) -> Self {
        Self::SampledImage(vec![DescriptorImageInfo::new(obj, layout)])
    }
    #[inline(always)]
    pub fn storage_image(obj: &'d ImageView, layout: ImageLayout) -> Self {
        Self::StorageImage(vec![DescriptorImageInfo::new(obj, layout)])
    }
    #[inline(always)]
    pub fn input_attachment(obj: &'d ImageView, layout: ImageLayout) -> Self {
        Self::InputAttachment(vec![DescriptorImageInfo::new(obj, layout)])
    }
    #[inline(always)]
    pub fn uniform_buffer(obj: &'d Buffer, range: Range<DeviceSize>) -> Self {
        Self::UniformBuffer(vec![DescriptorBufferInfo::new(obj, range)])
    }
    #[inline(always)]
    pub fn storage_buffer(obj: &'d Buffer, range: Range<DeviceSize>) -> Self {
        Self::StorageBuffer(vec![DescriptorBufferInfo::new(obj, range)])
    }
    #[inline(always)]
    pub fn uniform_buffer_dynamic(obj: &'d Buffer, range: Range<DeviceSize>) -> Self {
        Self::UniformBufferDynamic(vec![DescriptorBufferInfo::new(obj, range)])
    }
    #[inline(always)]
    pub fn storage_buffer_dynamic(obj: &'d Buffer, range: Range<DeviceSize>) -> Self {
        Self::StorageBufferDynamic(vec![DescriptorBufferInfo::new(obj, range)])
    }
    #[inline(always)]
    pub fn uniform_texel_buffer(obj: &'d BufferView) -> Self {
        Self::UniformTexelBuffer(vec![obj])
    }
    #[inline(always)]
    pub fn storage_texel_buffer(obj: &'d BufferView) -> Self {
        Self::StorageTexelBuffer(vec![obj])
    }
}

#[derive(Clone)]
pub struct DescriptorSetWriteInfo<'d, 's>(pub DescriptorPointer<'d>, pub DescriptorContents<'s>);
impl DescriptorSetWriteInfo<'_, '_> {
    pub const fn make_structure(&self) -> VkWriteDescriptorSet {
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
pub struct DescriptorSetCopyInfo<'a>(pub DescriptorPointer<'a>, pub DescriptorPointer<'a>, u32);
impl DescriptorSetCopyInfo<'_> {
    pub const fn make_structure(&self) -> VkCopyDescriptorSet {
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

#[cfg(feature = "VK_KHR_descriptor_update_template")]
mod update_template;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub use self::update_template::*;
