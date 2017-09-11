//! Vulkan Descriptors

use vk::*;
#[cfg(feature = "FeImplements")] use VkResultHandler;

/// Opaque handle to a descriptor set layout object
pub struct DescriptorSetLayout(pub VkDescriptorSetLayout, ::Device);
/// Opaque handle to a descriptor pool object
pub struct DescriptorPool(pub VkDescriptorPool, ::Device);

#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{ for DescriptorSetLayout[vkDestroyDescriptorSetLayout], DescriptorPool[vkDestroyDescriptorPool] }

/// Structure specifying a descriptor set layout binding
///
/// ```
/// // Create bindings for 1 uniform buffer (at binding #0) and 2 combined image samplers (at binding #1)
/// let bindings = DSLBindings::new().uniform_buffers(1, ShaderStage::VERTEX)
///   .combined_image_samplers(2, ShaderStage::FRAGMENT);
/// ```
pub struct DSLBindings
{
    counter: usize, bindings: [VkDescriptorSetLayoutBinding; VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT as usize+1],
    imm_samplers_smp: Vec<VkSampler>, imm_samplers_cmb: Vec<VkSampler>
}
impl DSLBindings
{
    pub fn new() -> Self
    {
        DSLBindings
        {
            counter: 0, bindings: unsafe { ::std::mem::zeroed() }, imm_samplers_smp: Vec::new(), imm_samplers_cmb: Vec::new()
        }
    }
    /// The elements of the `VkWriteDescriptorSet::pBufferInfo` array of `VkDescriptorBufferInfo` structures
    /// will be used to update the descriptors, and other arrays will be ignored
    /// # Panics
    /// Calling `uniform_buffer` twice or more is invalid due to Vulkan restriction
    pub fn uniform_buffers(&mut self, count: u32, shader_visibility: ::ShaderStage) -> &mut Self
    {
        self.check_registration(VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, "uniform_buffers");
        self.append(VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, count, shader_visibility, ::std::ptr::null())
    }
    pub fn storage_buffers(&mut self, count: u32, shader_visibility: ::ShaderStage) -> &mut Self
    {
        self.check_registration(VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, "storage_buffers");
        self.append(VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, count, shader_visibility, ::std::ptr::null())
    }
    pub fn uniform_buffers_dynamic(&mut self, count: u32, shader_visibility: ::ShaderStage) -> &mut Self
    {
        self.check_registration(VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC, "uniform_buffers_dynamic");
        self.append(VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC, count, shader_visibility, ::std::ptr::null())
    }
    pub fn storage_buffers_dynamic(&mut self, count: u32, shader_visibility: ::ShaderStage) -> &mut Self
    {
        self.check_registration(VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC, "storage_buffers_dynamic");
        self.append(VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC, count, shader_visibility, ::std::ptr::null())
    }
    pub fn uniform_texel_buffers(&mut self, count: u32, shader_visibility: ::ShaderStage) -> &mut Self
    {
        self.check_registration(VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER, "uniform_texel_buffers");
        self.append(VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER, count, shader_visibility, ::std::ptr::null())
    }
    pub fn storage_texel_buffers(&mut self, count: u32, shader_visibility: ::ShaderStage) -> &mut Self
    {
        self.check_registration(VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER, "storage_texel_buffers");
        self.append(VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER, count, shader_visibility, ::std::ptr::null())
    }
    pub fn samplers(&mut self, count: u32, shader_visibility: ::ShaderStage, imm_samplers: Vec<&::Sampler>) -> &mut Self
    {
        self.check_registration(VK_DESCRIPTOR_TYPE_SAMPLER, "samplers");
        assert!(imm_samplers.is_empty() || imm_samplers.len() == count as usize);
        self.imm_samplers_smp = imm_samplers.into_iter().map(|x| x.0).collect();
        let smps = if self.imm_samplers_smp.is_empty() { ::std::ptr::null() } else { self.imm_samplers_smp.as_ptr() };
        self.append(VK_DESCRIPTOR_TYPE_SAMPLER, count, shader_visibility, smps)
    }
    pub fn combined_image_samplers(&mut self, count: u32, shader_visibility: ::ShaderStage, imm_samplers: Vec<&::Sampler>) -> &mut Self
    {
        self.check_registration(VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, "combined_image_samplers");
        assert!(imm_samplers.is_empty() || imm_samplers.len() == count as usize);
        self.imm_samplers_cmb = imm_samplers.into_iter().map(|x| x.0).collect();
        let smps = if self.imm_samplers_cmb.is_empty() { ::std::ptr::null() } else { self.imm_samplers_cmb.as_ptr() };
        self.append(VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, count, shader_visibility, smps)
    }
    pub fn sampled_images(&mut self, count: u32, shader_visibility: ::ShaderStage) -> &mut Self
    {
        self.check_registration(VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE, "sampled_images");
        self.append(VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE, count, shader_visibility, ::std::ptr::null())
    }
    pub fn storage_images(&mut self, count: u32, shader_visibility: ::ShaderStage) -> &mut Self
    {
        self.check_registration(VK_DESCRIPTOR_TYPE_STORAGE_IMAGE, "storage_images");
        self.append(VK_DESCRIPTOR_TYPE_STORAGE_IMAGE, count, shader_visibility, ::std::ptr::null())
    }

    fn check_registration(&self, dty: VkDescriptorType, dty_name: &str)
    {
        if self.bindings[dty as usize].descriptorCount > 0 { panic!("Assigning to {} has occured twice.", dty_name); }
    }
    fn append(&mut self, dty: VkDescriptorType, count: u32, shader_visibility: ::ShaderStage, imm_samplers: *const VkSampler) -> &mut Self
    {
        self.bindings[dty as usize] = VkDescriptorSetLayoutBinding
        {
            binding: self.counter as _, descriptorType: dty, descriptorCount: count,
            stageFlags: shader_visibility.0, pImmutableSamplers: imm_samplers
        };
        self.counter += 1; self
    }
}

#[cfg(feature = "FeImplements")]
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
        let cinfo = VkDescriptorSetLayoutCreateInfo
        {
            bindingCount: VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT as u32 + 1, pBindings: bindings.bindings.as_ptr(),
            .. Default::default()
        };
        unsafe { vkCreateDescriptorSetLayout(device.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }
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

#[cfg(feature = "FeImplements")]
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
        unsafe { vkCreateDescriptorPool(device.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }
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
        unsafe { vkAllocateDescriptorSets(self.1.native_ptr(), &ainfo, hs.as_mut_ptr()) }
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
        vkResetDescriptorPool(self.1.native_ptr(), self.0, 0).into_result()
    }
    /// Free one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    pub fn free(&self, sets: &[VkDescriptorSet]) -> ::Result<()>
    {
        unsafe { vkFreeDescriptorSets(self.1.native_ptr(), self.0, sets.len() as _, sets.as_ptr()) }.into_result()
    }
}

/// Structure specifying the parameters of a descriptor set write operation
/// Element order: DescriptorSet, Binding, ArrayIndex, Description
#[derive(Clone)]
pub struct DescriptorSetWriteInfo<'d>(pub VkDescriptorSet, pub u32, pub u32, pub DescriptorUpdateInfo<'d>);
/// Structure specifying a copy descriptor set operation
#[derive(Clone)]
pub struct DescriptorSetCopyInfo { pub src: (VkDescriptorSet, u32, u32), pub dst: (VkDescriptorSet, u32, u32), pub count: u32 }
/// Structure specifying the parameters of a descriptor set write/copy operations
/// For Sampler, CombinedImageSampler, SampledImage, StorageImage and InputAttachment: Vec of tuple(ref to Sampler(optional), ref to ImageView, ImageLayout)
/// For UniformBuffer, StorageBuffer, UniformBufferDynamic and StorageBufferDynamic: Vec of tuple(ref to Buffer, range of bytes)
/// For UniformTexelBuffer and StorageTexelBuffer: Vec of ref to BufferView
#[derive(Clone)]
pub enum DescriptorUpdateInfo<'d>
{
    Sampler(Vec<(Option<&'d ::Sampler>, &'d ::ImageView, ::ImageLayout)>),
    CombinedImageSampler(Vec<(Option<&'d ::Sampler>, &'d ::ImageView, ::ImageLayout)>),
    SampledImage(Vec<(Option<&'d ::Sampler>, &'d ::ImageView, ::ImageLayout)>),
    StorageImage(Vec<(Option<&'d ::Sampler>, &'d ::ImageView, ::ImageLayout)>),
    InputAttachment(Vec<(Option<&'d ::Sampler>, &'d ::ImageView, ::ImageLayout)>),
    UniformBuffer(Vec<(&'d ::Buffer, ::std::ops::Range<usize>)>),
    StorageBuffer(Vec<(&'d ::Buffer, ::std::ops::Range<usize>)>),
    UniformBufferDynamic(Vec<(&'d ::Buffer, ::std::ops::Range<usize>)>),
    StorageBufferDynamic(Vec<(&'d ::Buffer, ::std::ops::Range<usize>)>),
    UniformTexelBuffer(Vec<&'d ::BufferView>),
    StorageTexelBuffer(Vec<&'d ::BufferView>)
}
#[cfg(feature = "FeImplements")]
impl<'d> DescriptorUpdateInfo<'d>
{
    pub fn decomposite(&self) -> (DescriptorType, u32, &[(Option<&'d ::Sampler>, &'d ::ImageView, ::ImageLayout)], &[(&'d ::Buffer, ::std::ops::Range<usize>)], &[&'d ::BufferView])
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
