//! Vulkan Descriptors

use vk::*;

/// Opaque handle to a descriptor set layout object
pub struct DescriptorSetLayout(pub VkDescriptorSetLayout, ::Device);
/// Opaque handle to a descriptor pool object
pub struct DescriptorPool(pub VkDescriptorPool, ::Device);

#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{ for DescriptorSetLayout, DescriptorPool }

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
            bindingCount: VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT + 1, pBindings: bindings.bindings.as_ptr(),
            .. Default::default()
        };
        unsafe { vkCreateDescriptorSetLayout(device.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }
            .into_result().map(|_| DescriptorSetLayout(h, device.clone()))
    }
}
