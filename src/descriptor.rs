//! Vulkan Descriptors

use vk::*;
use {VkHandle, DeviceChild};
#[cfg(feature = "Implements")] use VkResultHandler;
use ShaderStage;
#[cfg(feature = "Implements")] use std::ptr::null;
#[cfg(feature = "Implements")] use std::mem::zeroed;
#[cfg(feature = "Implements")] use vkresolve::Resolver;

/// Opaque handle to a descriptor set layout object
pub struct DescriptorSetLayout(VkDescriptorSetLayout, ::Device);
/// Opaque handle to a descriptor pool object
pub struct DescriptorPool(VkDescriptorPool, ::Device);

#[cfg(feature = "Implements")] DeviceChildCommonDrop!{ for DescriptorSetLayout[destroy_descriptor_set_layout], DescriptorPool[destroy_descriptor_pool] }

impl VkHandle for DescriptorSetLayout { type Handle = VkDescriptorSetLayout; fn native_ptr(&self) -> VkDescriptorSetLayout { self.0 } }
impl VkHandle for DescriptorPool { type Handle = VkDescriptorPool; fn native_ptr(&self) -> VkDescriptorPool { self.0 } }
impl DeviceChild for DescriptorSetLayout { fn device(&self) -> &::Device { &self.1 } }
impl DeviceChild for DescriptorPool { fn device(&self) -> &::Device { &self.1 } }

/// Structure specifying a descriptor set layout binding  
/// Tuple Element: (binding index, descriptor count, shader visibility, immutable samplers(if needed))
pub struct DSLBindings
{
    /// Specifies a [sampler descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-sampler)
    pub sampler: Option<(u32, u32, ShaderStage, Vec<VkSampler>)>,
    /// Specifies a [combined image sampler descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-combinedimagesampler)
    pub combined_image_sampler: Option<(u32, u32, ShaderStage, Vec<VkSampler>)>,
    /// Specifies a [storage image descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-storageimage)
    pub sampled_image: Option<(u32, u32, ShaderStage)>,
    /// Specifies a [sampled image descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-sampledimage)
    pub storage_image: Option<(u32, u32, ShaderStage)>,
    /// Specifies a [uniform texel buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-uniformtexelbuffer)
    pub uniform_texel_buffer: Option<(u32, u32, ShaderStage)>,
    /// Specifies a [storage texel buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-storagetexelbuffer)
    pub storage_texel_buffer: Option<(u32, u32, ShaderStage)>,
    /// Specifies a [uniform buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-uniformbuffer)
    pub uniform_buffer: Option<(u32, u32, ShaderStage)>,
    /// Specifies a [storage buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-storagebuffer)
    pub storage_buffer: Option<(u32, u32, ShaderStage)>,
    /// Specifies a [dynamic uniform buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-uniformbufferdynamic)
    pub uniform_buffer_dynamic: Option<(u32, u32, ShaderStage)>,
    /// Specifies a [dynamic storage buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-storagebufferdynamic)
    pub storage_buffer_dynamic: Option<(u32, u32, ShaderStage)>,
    /// Specifies a [input attachment descriptor](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-inputattachment)
    pub input_attachment: Option<(u32, u32, ShaderStage)>
}
impl DSLBindings
{
    /// An empty binding
    pub fn empty() -> Self
    {
        DSLBindings
        {
            sampler: None, combined_image_sampler: None, sampled_image: None, storage_image: None,
            uniform_texel_buffer: None, storage_texel_buffer: None, uniform_buffer: None, storage_buffer: None,
            uniform_buffer_dynamic: None, storage_buffer_dynamic: None, input_attachment: None
        }
    }
}

/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl DescriptorSetLayout
{
    /// Create a new descriptor set layout
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    pub fn new(device: &::Device, bindings: &DSLBindings) -> ::Result<Self>
    {
        let mut h = VK_NULL_HANDLE as _;
        let mut n_bindings = Vec::with_capacity(VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT as usize + 1);
        fn mapper2(&(b, n, sv, ref imm): &(u32, u32, ShaderStage, Vec<VkSampler>), dty: VkDescriptorType)
            -> VkDescriptorSetLayoutBinding
        {
            VkDescriptorSetLayoutBinding
            {
                binding: b, descriptorType: dty, descriptorCount: n, stageFlags: sv.0, pImmutableSamplers: imm.as_ptr()
            }
        }
        fn mapper(&(b, n, sv): &(u32, u32, ShaderStage), dty: VkDescriptorType) -> VkDescriptorSetLayoutBinding
        {
            VkDescriptorSetLayoutBinding
            {
                binding: b, descriptorType: dty, descriptorCount: n, stageFlags: sv.0, pImmutableSamplers: null()
            }
        }
        fn append_some<T>(v: Option<T>, a: &mut Vec<T>) { if let Some(v) = v { a.push(v); } }
        append_some(bindings.sampler.as_ref().map(|b| mapper2(b, VK_DESCRIPTOR_TYPE_SAMPLER)), &mut n_bindings);
        append_some(bindings.combined_image_sampler.as_ref()
            .map(|b| mapper2(b, VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER)), &mut n_bindings);
        append_some(bindings.sampled_image.as_ref()
            .map(|b| mapper(b, VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE)), &mut n_bindings);
        append_some(bindings.storage_image.as_ref()
            .map(|b| mapper(b, VK_DESCRIPTOR_TYPE_STORAGE_IMAGE)), &mut n_bindings);
        append_some(bindings.uniform_texel_buffer.as_ref()
            .map(|b| mapper(b, VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER)), &mut n_bindings);
        append_some(bindings.storage_texel_buffer.as_ref()
            .map(|b| mapper(b, VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER)), &mut n_bindings);
        append_some(bindings.uniform_buffer.as_ref()
            .map(|b| mapper(b, VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER)), &mut n_bindings);
        append_some(bindings.storage_buffer.as_ref()
            .map(|b| mapper(b, VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)), &mut n_bindings);
        append_some(bindings.uniform_buffer_dynamic.as_ref()
            .map(|b| mapper(b, VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC)), &mut n_bindings);
        append_some(bindings.storage_buffer_dynamic.as_ref()
            .map(|b| mapper(b, VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC)), &mut n_bindings);
        append_some(bindings.input_attachment.as_ref()
            .map(|b| mapper(b, VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT)), &mut n_bindings);
        let cinfo = VkDescriptorSetLayoutCreateInfo
        {
            bindingCount: n_bindings.len() as _, pBindings: n_bindings.as_ptr(), .. Default::default()
        };
        unsafe { Resolver::get().create_descriptor_set_layout(device.native_ptr(), &cinfo, null(), &mut h) }
            .into_result().map(|_| DescriptorSetLayout(h, device.clone()))
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
#[repr(u32)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum DescriptorType
{
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
    InputAttachment = VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT as _
}

/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl DescriptorPool
{
    /// Creates a descriptor pool object
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    pub fn new(device: &::Device, max_sets: u32, pool_sizes: &[DescriptorPoolSize], allow_free: bool) -> ::Result<Self>
    {
        let mut h = VK_NULL_HANDLE as _;
        let cinfo = VkDescriptorPoolCreateInfo
        {
            maxSets: max_sets, flags: if allow_free { VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT } else { 0 },
            poolSizeCount: pool_sizes.len() as _, pPoolSizes: pool_sizes.as_ptr() as *const _, .. Default::default()
        };
        unsafe { Resolver::get().create_descriptor_pool(device.native_ptr(), &cinfo, null(), &mut h) }
            .into_result().map(|_| DescriptorPool(h, device.clone()))
    }
    /// Allocate one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    /// - VK_ERROR_FRAGMENTED_POOL
    pub fn alloc(&self, layouts: &[&DescriptorSetLayout]) -> ::Result<Vec<VkDescriptorSet>>
    {
        let layout_ptrs = layouts.into_iter().map(|x| x.0).collect::<Vec<_>>();
        let ainfo = VkDescriptorSetAllocateInfo
        {
            descriptorPool: self.0, descriptorSetCount: layout_ptrs.len() as _, pSetLayouts: layout_ptrs.as_ptr(),
            .. Default::default()
        };
        let mut hs = vec![VK_NULL_HANDLE as _; layout_ptrs.len()];
        unsafe { Resolver::get().allocate_descriptor_sets(self.1.native_ptr(), &ainfo, hs.as_mut_ptr()) }
            .into_result().map(|_| hs)
    }
    /// Resets a descriptor pool object
    /// # Safety
    /// Application cannot use descriptor sets after this call
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    pub unsafe fn reset(&self) -> ::Result<()>
    {
        Resolver::get().reset_descriptor_pool(self.1.native_ptr(), self.0, 0).into_result()
    }
    /// Free one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    pub fn free(&self, sets: &[VkDescriptorSet]) -> ::Result<()>
    {
        unsafe { Resolver::get().free_descriptor_sets(self.1.native_ptr(), self.0, sets.len() as _, sets.as_ptr()) }.into_result()
    }
}

/// Structure specifying the parameters of a descriptor set write operation
/// Element order: DescriptorSet, Binding, ArrayIndex, Description
#[derive(Clone)]
pub struct DescriptorSetWriteInfo(pub VkDescriptorSet, pub u32, pub u32, pub DescriptorUpdateInfo);
/// Structure specifying a copy descriptor set operation
#[derive(Clone)]
pub struct DescriptorSetCopyInfo { pub src: (VkDescriptorSet, u32, u32), pub dst: (VkDescriptorSet, u32, u32), pub count: u32 }
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
pub enum DescriptorUpdateInfo
{
    Sampler(Vec<(Option<VkSampler>, VkImageView, ::ImageLayout)>),
    CombinedImageSampler(Vec<(Option<VkSampler>, VkImageView, ::ImageLayout)>),
    SampledImage(Vec<(Option<VkSampler>, VkImageView, ::ImageLayout)>),
    StorageImage(Vec<(Option<VkSampler>, VkImageView, ::ImageLayout)>),
    InputAttachment(Vec<(Option<VkSampler>, VkImageView, ::ImageLayout)>),
    UniformBuffer(Vec<(VkBuffer, ::std::ops::Range<usize>)>),
    StorageBuffer(Vec<(VkBuffer, ::std::ops::Range<usize>)>),
    UniformBufferDynamic(Vec<(VkBuffer, ::std::ops::Range<usize>)>),
    StorageBufferDynamic(Vec<(VkBuffer, ::std::ops::Range<usize>)>),
    UniformTexelBuffer(Vec<VkBufferView>),
    StorageTexelBuffer(Vec<VkBufferView>)
}
#[cfg(feature = "Implements")]
use std::ops::Range;
impl DescriptorUpdateInfo
{
	#[cfg(feature = "Implements")]
    pub(crate) fn decomposite(&self) -> (DescriptorType, u32, &[(Option<VkSampler>, VkImageView, ::ImageLayout)], &[(VkBuffer, Range<usize>)], &[VkBufferView])
    {
        match self
        {
            &DescriptorUpdateInfo::Sampler(ref iv) => (DescriptorType::Sampler, iv.len() as _, iv, &[], &[]),
            &DescriptorUpdateInfo::CombinedImageSampler(ref iv) => (DescriptorType::CombinedImageSampler, iv.len() as _, iv, &[], &[]),
            &DescriptorUpdateInfo::SampledImage(ref iv) => (DescriptorType::SampledImage, iv.len() as _, iv, &[], &[]),
            &DescriptorUpdateInfo::StorageImage(ref iv) => (DescriptorType::StorageImage, iv.len() as _, iv, &[], &[]),
            &DescriptorUpdateInfo::InputAttachment(ref iv) => (DescriptorType::InputAttachment, iv.len() as _, iv, &[], &[]),
            &DescriptorUpdateInfo::UniformBuffer(ref bv) => (DescriptorType::UniformBuffer, bv.len() as _, &[], bv, &[]),
            &DescriptorUpdateInfo::StorageBuffer(ref bv) => (DescriptorType::StorageBuffer, bv.len() as _, &[], bv, &[]),
            &DescriptorUpdateInfo::UniformBufferDynamic(ref bv) => (DescriptorType::UniformBufferDynamic, bv.len() as _, &[], bv, &[]),
            &DescriptorUpdateInfo::StorageBufferDynamic(ref bv) => (DescriptorType::StorageBufferDynamic, bv.len() as _, &[], bv, &[]),
            &DescriptorUpdateInfo::UniformTexelBuffer(ref bvv) => (DescriptorType::UniformTexelBuffer, bvv.len() as _, &[], &[], bvv),
            &DescriptorUpdateInfo::StorageTexelBuffer(ref bvv) => (DescriptorType::StorageTexelBuffer, bvv.len() as _, &[], &[], bvv)
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

pub struct DescriptorUpdateTemplate(VkDescriptorUpdateTemplate, ::Device);
#[cfg(feature = "Implements")] impl Drop for DescriptorUpdateTemplate
{
    fn drop(&mut self)
    {
        unsafe
        {
            self.1.instance().destroy_descriptor_update_template(self.1.native_ptr(), self.0, null());
        }
    }
}
#[cfg(feature = "Implements")]
impl DescriptorUpdateTemplate
{
    /// dsl: NoneにするとPushDescriptors向けのテンプレートを作成できる
    pub fn new(device: &::Device, entries: &[VkDescriptorUpdateTemplateEntry], dsl: Option<&DescriptorSetLayout>) -> ::Result<Self>
    {
        let cinfo = VkDescriptorUpdateTemplateCreateInfo
        {
            descriptorUpdateEntryCount: entries.len() as _, pDescriptorUpdateEntries: entries.as_ptr(),
            templateType: if dsl.is_none() { VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS }
                else { VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET },
            descriptorSetLayout: dsl.native_ptr(), .. Default::default()
        };
        let mut handle = unsafe { zeroed() };
        unsafe
        {
            device.instance().create_descriptor_update_template(device.native_ptr(), &cinfo, null(), &mut handle)
                .into_result().map(|_| DescriptorUpdateTemplate(handle, device.clone()))
        }
    }
    pub fn update_set<T>(&self, set: VkDescriptorSet, data: &T)
    {
        unsafe
        {
            Resolver::get().update_descriptor_set_with_template(self.device().native_ptr(), set, self.native_ptr(),
                data as *const T as *const _)
        }
    }
}
impl VkHandle for DescriptorUpdateTemplate
{
    type Handle = VkDescriptorUpdateTemplate; fn native_ptr(&self) -> VkDescriptorUpdateTemplate { self.0 }
}
impl DeviceChild for DescriptorUpdateTemplate
{
    fn device(&self) -> &::Device { &self.1 }
}
