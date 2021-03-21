//! Vulkan Descriptors

use crate::vk::*;
#[cfg(feature = "Implements")]
use crate::{
    vkresolve::{Resolver, ResolverInterface},
    VkResultHandler,
};
use crate::{Device, DeviceChild, ImageLayout, ShaderStage, VkHandle};

/// Opaque handle to a descriptor set layout object
#[derive(VkHandle, DeviceChild)]
#[drop_function_name = "destroy_descriptor_set_layout"]
#[object_type = "VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT"]
pub struct DescriptorSetLayout(VkDescriptorSetLayout, Device);
/// Opaque handle to a descriptor pool object
#[derive(VkHandle, DeviceChild)]
#[drop_function_name = "destroy_descriptor_pool"]
#[object_type = "VK_OBJECT_TYPE_DESCRIPTOR_POOL"]
pub struct DescriptorPool(VkDescriptorPool, Device);

impl std::cmp::PartialEq for DescriptorSetLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for DescriptorSetLayout {}
impl std::hash::Hash for DescriptorSetLayout {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum DescriptorSetLayoutBinding<'s> {
    Sampler(u32, ShaderStage, &'s [VkSampler]),
    CombinedImageSampler(u32, ShaderStage, &'s [VkSampler]),
    SampledImage(u32, ShaderStage),
    StorageImage(u32, ShaderStage),
    UniformTexelBuffer(u32, ShaderStage),
    StorageTexelBuffer(u32, ShaderStage),
    UniformBuffer(u32, ShaderStage),
    StorageBuffer(u32, ShaderStage),
    UniformBufferDynamic(u32, ShaderStage),
    StorageBufferDynamic(u32, ShaderStage),
    InputAttachment(u32, ShaderStage),
}
impl<'s> DescriptorSetLayoutBinding<'s> {
    fn descriptor_type(&self) -> VkDescriptorType {
        match self {
            Self::Sampler(_, _, _) => VK_DESCRIPTOR_TYPE_SAMPLER,
            Self::CombinedImageSampler(_, _, _) => VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            Self::SampledImage(_, _) => VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
            Self::StorageImage(_, _) => VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
            Self::UniformTexelBuffer(_, _) => VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER,
            Self::StorageTexelBuffer(_, _) => VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER,
            Self::UniformBuffer(_, _) => VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            Self::StorageBuffer(_, _) => VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            Self::UniformBufferDynamic(_, _) => VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC,
            Self::StorageBufferDynamic(_, _) => VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC,
            Self::InputAttachment(_, _) => VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT,
        }
    }
    fn immutable_samplers(&self) -> &'s [VkSampler] {
        match self {
            &Self::Sampler(_, _, s) => s,
            &Self::CombinedImageSampler(_, _, s) => s,
            _ => &[],
        }
    }
    fn common_part(&self) -> (u32, ShaderStage) {
        match self {
            &Self::Sampler(c, s, _)
            | &Self::CombinedImageSampler(c, s, _)
            | &Self::SampledImage(c, s)
            | &Self::StorageImage(c, s)
            | &Self::UniformTexelBuffer(c, s)
            | &Self::StorageTexelBuffer(c, s)
            | &Self::UniformBuffer(c, s)
            | &Self::StorageBuffer(c, s)
            | &Self::UniformBufferDynamic(c, s)
            | &Self::StorageBufferDynamic(c, s)
            | &Self::InputAttachment(c, s) => (c, s),
        }
    }
}
impl<'s> DescriptorSetLayoutBinding<'s> {
    pub fn make_structure_with_binding_index(&self, binding_index: u32) -> VkDescriptorSetLayoutBinding {
        let (c, s) = self.common_part();
        let iss = self.immutable_samplers();

        VkDescriptorSetLayoutBinding {
            binding: binding_index,
            descriptorType: self.descriptor_type(),
            descriptorCount: c,
            stageFlags: s.0,
            pImmutableSamplers: if iss.is_empty() { std::ptr::null() } else { iss.as_ptr() },
        }
    }
}

/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl DescriptorSetLayout {
    /// Create a new descriptor set layout
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    pub fn new(device: &Device, bindings: &[DescriptorSetLayoutBinding]) -> crate::Result<Self> {
        let binding_structures: Vec<_> = bindings
            .into_iter()
            .enumerate()
            .map(|(n, b)| b.make_structure_with_binding_index(n as _))
            .collect();
        let cinfo = VkDescriptorSetLayoutCreateInfo {
            bindingCount: binding_structures.len() as _,
            pBindings: binding_structures.as_ptr(),
            ..Default::default()
        };

        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get()
                .create_descriptor_set_layout(device.native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| DescriptorSetLayout(h.assume_init(), device.clone()))
        }
    }
}

/*
# DescriptorPoolのフラグメンテーションについて(from `VkDescriptorPoolCreateInfo` Manual)

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

/// Structure specifying descriptor pool size
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct DescriptorPoolSize(pub DescriptorType, pub u32);
/// Specified the type of a descriptor in a descriptor set
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord)]
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

/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl DescriptorPool {
    /// Creates a descriptor pool object
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    pub fn new(
        device: &Device,
        max_sets: u32,
        pool_sizes: &[DescriptorPoolSize],
        allow_free: bool,
    ) -> crate::Result<Self> {
        let mut h = VK_NULL_HANDLE as _;
        let cinfo = VkDescriptorPoolCreateInfo {
            maxSets: max_sets,
            flags: if allow_free {
                VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT
            } else {
                0
            },
            poolSizeCount: pool_sizes.len() as _,
            pPoolSizes: pool_sizes.as_ptr() as *const _,
            ..Default::default()
        };
        unsafe {
            Resolver::get()
                .create_descriptor_pool(device.native_ptr(), &cinfo, std::ptr::null(), &mut h)
                .into_result()
                .map(|_| DescriptorPool(h, device.clone()))
        }
    }
    /// Allocate one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    /// - VK_ERROR_FRAGMENTED_POOL
    pub fn alloc(&self, layouts: &[&DescriptorSetLayout]) -> crate::Result<Vec<VkDescriptorSet>> {
        let layout_ptrs = layouts.iter().map(|x| x.0).collect::<Vec<_>>();
        let ainfo = VkDescriptorSetAllocateInfo {
            descriptorPool: self.0,
            descriptorSetCount: layout_ptrs.len() as _,
            pSetLayouts: layout_ptrs.as_ptr(),
            ..Default::default()
        };
        let mut hs = vec![VK_NULL_HANDLE as _; layout_ptrs.len()];
        unsafe {
            Resolver::get()
                .allocate_descriptor_sets(self.1.native_ptr(), &ainfo, hs.as_mut_ptr())
                .into_result()
                .map(|_| hs)
        }
    }
    /// Resets a descriptor pool object
    /// # Safety
    /// Application must not use descriptor sets after this call
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    pub unsafe fn reset(&self) -> crate::Result<()> {
        Resolver::get()
            .reset_descriptor_pool(self.1.native_ptr(), self.0, 0)
            .into_result()
    }
    /// Free one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    pub fn free(&self, sets: &[VkDescriptorSet]) -> crate::Result<()> {
        unsafe {
            Resolver::get()
                .free_descriptor_sets(self.1.native_ptr(), self.0, sets.len() as _, sets.as_ptr())
                .into_result()
        }
    }
}

/// Structure specifying the parameters of a descriptor set write operation
/// Element order: DescriptorSet, Binding, ArrayIndex, Description
#[derive(Clone)]
pub struct DescriptorSetWriteInfo(pub VkDescriptorSet, pub u32, pub u32, pub DescriptorUpdateInfo);
/// Structure specifying a copy descriptor set operation
#[derive(Clone)]
pub struct DescriptorSetCopyInfo {
    pub src: (VkDescriptorSet, u32, u32),
    pub dst: (VkDescriptorSet, u32, u32),
    pub count: u32,
}
/// Structure specifying the parameters of a descriptor set write/copy operations.
///
/// * For Sampler, CombinedImageSampler, SampledImage, StorageImage and InputAttachment: Vec of tuple(ref to Sampler(optional), ref to ImageView, ImageLayout)
/// * For UniformBuffer, StorageBuffer, UniformBufferDynamic and StorageBufferDynamic: Vec of tuple(ref to Buffer, range of bytes)
/// * For UniformTexelBuffer and StorageTexelBuffer: Vec of ref to BufferView
///
/// ## Safety
///
/// Please ensure that resources are alive while updating
#[derive(Clone)]
pub enum DescriptorUpdateInfo {
    Sampler(Vec<(Option<VkSampler>, VkImageView, ImageLayout)>),
    CombinedImageSampler(Vec<(Option<VkSampler>, VkImageView, ImageLayout)>),
    SampledImage(Vec<(Option<VkSampler>, VkImageView, ImageLayout)>),
    StorageImage(Vec<(Option<VkSampler>, VkImageView, ImageLayout)>),
    InputAttachment(Vec<(Option<VkSampler>, VkImageView, ImageLayout)>),
    UniformBuffer(Vec<(VkBuffer, std::ops::Range<usize>)>),
    StorageBuffer(Vec<(VkBuffer, std::ops::Range<usize>)>),
    UniformBufferDynamic(Vec<(VkBuffer, std::ops::Range<usize>)>),
    StorageBufferDynamic(Vec<(VkBuffer, std::ops::Range<usize>)>),
    UniformTexelBuffer(Vec<VkBufferView>),
    StorageTexelBuffer(Vec<VkBufferView>),
}
#[cfg(feature = "Implements")]
use std::ops::Range;
impl DescriptorUpdateInfo {
    #[cfg(feature = "Implements")]
    #[allow(clippy::type_complexity)]
    pub(crate) fn decomposite(
        &self,
    ) -> (
        DescriptorType,
        u32,
        &[(Option<VkSampler>, VkImageView, ImageLayout)],
        &[(VkBuffer, Range<usize>)],
        &[VkBufferView],
    ) {
        match self {
            DescriptorUpdateInfo::Sampler(ref iv) => (DescriptorType::Sampler, iv.len() as _, iv, &[], &[]),
            DescriptorUpdateInfo::CombinedImageSampler(ref iv) => {
                (DescriptorType::CombinedImageSampler, iv.len() as _, iv, &[], &[])
            }
            DescriptorUpdateInfo::SampledImage(ref iv) => (DescriptorType::SampledImage, iv.len() as _, iv, &[], &[]),
            DescriptorUpdateInfo::StorageImage(ref iv) => (DescriptorType::StorageImage, iv.len() as _, iv, &[], &[]),
            DescriptorUpdateInfo::InputAttachment(ref iv) => {
                (DescriptorType::InputAttachment, iv.len() as _, iv, &[], &[])
            }
            DescriptorUpdateInfo::UniformBuffer(ref bv) => (DescriptorType::UniformBuffer, bv.len() as _, &[], bv, &[]),
            DescriptorUpdateInfo::StorageBuffer(ref bv) => (DescriptorType::StorageBuffer, bv.len() as _, &[], bv, &[]),
            DescriptorUpdateInfo::UniformBufferDynamic(ref bv) => {
                (DescriptorType::UniformBufferDynamic, bv.len() as _, &[], bv, &[])
            }
            DescriptorUpdateInfo::StorageBufferDynamic(ref bv) => {
                (DescriptorType::StorageBufferDynamic, bv.len() as _, &[], bv, &[])
            }
            DescriptorUpdateInfo::UniformTexelBuffer(ref bvv) => {
                (DescriptorType::UniformTexelBuffer, bvv.len() as _, &[], &[], bvv)
            }
            DescriptorUpdateInfo::StorageTexelBuffer(ref bvv) => {
                (DescriptorType::StorageTexelBuffer, bvv.len() as _, &[], &[], bvv)
            }
        }
    }
}

#[macro_export]
macro_rules! DescriptorUpdateTemplateEntry
{
    { ($b: expr, $a: expr) .. : [$ty: expr; $c: expr] = $o: expr, $s: expr } =>
    {
        VkDescriptorUpdateTemplateEntry
        {
            descriptorType: $ty, descriptorCount: $c,
            dstBinding: $b, dstArrayElement: $a, offset: $o, stride: $s
        }
    };
}
#[macro_export]
macro_rules! DescriptorUpdateTemplateEntries
{
    { { $(($b: expr, $a: expr) .. : [$ty: expr; $c: expr] = $o: expr, $s: expr),* } } =>
    { {
        $(DescriptorUpdateTemplateEntry! { ($b, $a) ..: [$ty; $c] = $o, $s }),*
    } };
}

#[derive(VkHandle, DeviceChild)]
#[object_type = "VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE"]
pub struct DescriptorUpdateTemplate(VkDescriptorUpdateTemplate, Device);
#[cfg(feature = "Implements")]
impl Drop for DescriptorUpdateTemplate {
    fn drop(&mut self) {
        unsafe {
            self.1
                .instance()
                .destroy_descriptor_update_template(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}
#[cfg(feature = "Implements")]
impl DescriptorUpdateTemplate {
    /// dsl: NoneにするとPushDescriptors向けのテンプレートを作成できる
    pub fn new(
        device: &Device,
        entries: &[VkDescriptorUpdateTemplateEntry],
        dsl: Option<&DescriptorSetLayout>,
    ) -> crate::Result<Self> {
        let cinfo = VkDescriptorUpdateTemplateCreateInfo {
            descriptorUpdateEntryCount: entries.len() as _,
            pDescriptorUpdateEntries: entries.as_ptr(),
            templateType: if dsl.is_none() {
                VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS
            } else {
                VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET
            },
            descriptorSetLayout: dsl.native_ptr(),
            ..Default::default()
        };
        let mut handle = std::mem::MaybeUninit::uninit();
        unsafe {
            device
                .instance()
                .create_descriptor_update_template(device.native_ptr(), &cinfo, std::ptr::null(), handle.as_mut_ptr())
                .into_result()
                .map(|_| DescriptorUpdateTemplate(handle.assume_init(), device.clone()))
        }
    }
    pub fn update_set<T>(&self, set: VkDescriptorSet, data: &T) {
        unsafe {
            Resolver::get().update_descriptor_set_with_template(
                self.device().native_ptr(),
                set,
                self.native_ptr(),
                data as *const T as *const _,
            )
        }
    }
}
