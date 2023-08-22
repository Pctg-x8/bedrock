//! Vulkan Descriptors

use cfg_if::cfg_if;
use derives::implements;

#[cfg(feature = "Implements")]
use crate::VkHandleMut;
use crate::{vk::*, DeviceChild, VkObject, VkRawHandle};
use crate::{ImageLayout, ShaderStage, VkHandle, VulkanStructure};

#[derive(VkHandle, VkObject, DeviceChild)]
#[VkObject(type = VkDescriptorSetLayout::OBJECT_TYPE)]
pub struct DescriptorSetLayoutObject<Device: crate::Device>(
    pub(crate) VkDescriptorSetLayout,
    #[parent] pub(crate) Device,
);
unsafe impl<Device: crate::Device + Send> Send for DescriptorSetLayoutObject<Device> {}
unsafe impl<Device: crate::Device + Sync> Sync for DescriptorSetLayoutObject<Device> {}
#[implements]
impl<Device: crate::Device> Drop for DescriptorSetLayoutObject<Device> {
    fn drop(&mut self) {
        unsafe {
            crate::vkresolve::destroy_descriptor_set_layout(self.1.native_ptr(), self.0, core::ptr::null());
        }
    }
}
impl<Device: crate::Device> DescriptorSetLayout for DescriptorSetLayoutObject<Device> {}
impl<Device: crate::Device> std::cmp::PartialEq for DescriptorSetLayoutObject<Device> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<Device: crate::Device> std::cmp::Eq for DescriptorSetLayoutObject<Device> {}
impl<Device: crate::Device> std::hash::Hash for DescriptorSetLayoutObject<Device> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

#[derive(VkHandle, VkObject, DeviceChild)]
#[VkObject(type = VkDescriptorPool::OBJECT_TYPE)]
pub struct DescriptorPoolObject<Device: crate::Device>(pub(crate) VkDescriptorPool, #[parent] pub(crate) Device);
unsafe impl<Device: crate::Device + Send> Send for DescriptorPoolObject<Device> {}
unsafe impl<Device: crate::Device + Sync> Sync for DescriptorPoolObject<Device> {}
#[implements]
impl<Device: crate::Device> Drop for DescriptorPoolObject<Device> {
    fn drop(&mut self) {
        unsafe {
            crate::vkresolve::destroy_descriptor_pool(self.1.native_ptr(), self.0, core::ptr::null());
        }
    }
}
impl<Device: crate::Device> DescriptorPool for DescriptorPoolObject<Device> {}

#[repr(transparent)]
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
    pub const fn with_count(self, count: u32) -> VkDescriptorPoolSize {
        VkDescriptorPoolSize {
            _type: self as _,
            descriptorCount: count,
        }
    }

    pub const fn make_binding<'a>(self, count: u32) -> DescriptorSetLayoutBinding<'a> {
        DescriptorSetLayoutBinding {
            ty: self,
            count,
            shader_stage_mask: ShaderStage::ALL,
            immutable_samplers: Vec::new(),
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct SamplerObjectRef<'s>(
    VkSampler,
    core::marker::PhantomData<&'s dyn VkHandle<Handle = VkSampler>>,
);
impl<'s> SamplerObjectRef<'s> {
    pub fn new(x: &'s (impl VkHandle<Handle = VkSampler> + ?Sized)) -> Self {
        Self(x.native_ptr(), core::marker::PhantomData)
    }
}

#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct ImageViewObjectRef<'s>(
    VkImageView,
    core::marker::PhantomData<&'s dyn VkHandle<Handle = VkImageView>>,
);
impl<'s> ImageViewObjectRef<'s> {
    pub fn new(r: &'s (impl VkHandle<Handle = VkImageView> + ?Sized)) -> Self {
        Self(r.native_ptr(), core::marker::PhantomData)
    }
}

#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct BufferObjectRef<'s>(VkBuffer, core::marker::PhantomData<&'s dyn VkHandle<Handle = VkBuffer>>);
impl<'s> BufferObjectRef<'s> {
    pub fn new(r: &'s (impl VkHandle<Handle = VkBuffer> + ?Sized)) -> Self {
        Self(r.native_ptr(), core::marker::PhantomData)
    }
}

#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct VkHandleRef<'r, H>(H, core::marker::PhantomData<&'r dyn VkHandle<Handle = H>>);
impl<'r, H> VkHandleRef<'r, H> {
    pub fn new(r: &'r (impl VkHandle<Handle = H> + ?Sized)) -> Self {
        Self(r.native_ptr(), core::marker::PhantomData)
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct DescriptorSetLayoutBinding<'s> {
    ty: DescriptorType,
    count: u32,
    shader_stage_mask: ShaderStage,
    immutable_samplers: Vec<SamplerObjectRef<'s>>,
}
impl<'s> DescriptorSetLayoutBinding<'s> {
    pub fn with_immutable_samplers(self, samplers: Vec<SamplerObjectRef<'s>>) -> Self {
        Self {
            immutable_samplers: samplers,
            ..self
        }
    }

    pub fn for_shader_stage(self, mask: ShaderStage) -> Self {
        Self {
            shader_stage_mask: mask,
            ..self
        }
    }

    pub fn only_for_vertex(self) -> Self {
        self.for_shader_stage(ShaderStage::VERTEX)
    }

    pub fn only_for_tess_control(self) -> Self {
        self.for_shader_stage(ShaderStage::TESSELLATION_CONTROL)
    }

    pub fn only_for_tess_evaluation(self) -> Self {
        self.for_shader_stage(ShaderStage::TESSELLATION_EVALUATION)
    }

    pub fn only_for_tessellation(self) -> Self {
        self.for_shader_stage(ShaderStage::TESSELLATION)
    }

    pub fn only_for_geometry(self) -> Self {
        self.for_shader_stage(ShaderStage::GEOMETRY)
    }

    pub fn only_for_fragment(self) -> Self {
        self.for_shader_stage(ShaderStage::FRAGMENT)
    }

    pub fn only_for_compute(self) -> Self {
        self.for_shader_stage(ShaderStage::COMPUTE)
    }

    fn make_structure_with_binding_index(&self, binding: u32) -> VkDescriptorSetLayoutBinding {
        VkDescriptorSetLayoutBinding {
            binding,
            descriptorType: self.ty as _,
            descriptorCount: self.count,
            stageFlags: self.shader_stage_mask.0,
            pImmutableSamplers: self.immutable_samplers.as_ptr() as *const _,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DescriptorSetLayoutBuilder<'s>(VkDescriptorSetLayoutCreateInfo, Vec<DescriptorSetLayoutBinding<'s>>);
impl<'s> DescriptorSetLayoutBuilder<'s> {
    pub const fn new() -> Self {
        Self::with_bindings(Vec::new())
    }

    pub const fn with_bindings(bindings: Vec<DescriptorSetLayoutBinding<'s>>) -> Self {
        Self(
            VkDescriptorSetLayoutCreateInfo {
                sType: VkDescriptorSetLayoutCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                bindingCount: 0,
                pBindings: core::ptr::null(),
            },
            bindings,
        )
    }

    pub fn bind(mut self, binding: DescriptorSetLayoutBinding<'s>) -> Self {
        self.1.push(binding);
        self
    }

    /// Create a new descriptor set layout
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    pub fn create<Device: crate::Device>(mut self, device: Device) -> crate::Result<DescriptorSetLayoutObject<Device>>
    where
        Self: Sized,
    {
        let bindings = self
            .1
            .iter()
            .enumerate()
            .map(|(n, b)| b.make_structure_with_binding_index(n as _))
            .collect::<Vec<_>>();
        self.0.bindingCount = bindings.len() as _;
        self.0.pBindings = bindings.as_ptr();

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_descriptor_set_layout(
                device.native_ptr(),
                &self.0,
                std::ptr::null(),
                h.as_mut_ptr(),
            )
            .into_result()
            .map(move |_| DescriptorSetLayoutObject(h.assume_init(), device))
        }
    }
}

pub trait DescriptorSetLayout: VkHandle<Handle = VkDescriptorSetLayout> + DeviceChild {}
DerefContainerBracketImpl!(for DescriptorSetLayout {});
GuardsImpl!(for DescriptorSetLayout {});

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

#[derive(Clone, Debug)]
pub struct DescriptorPoolBuilder(VkDescriptorPoolCreateInfo, Vec<VkDescriptorPoolSize>);
impl DescriptorPoolBuilder {
    pub const fn new(max_sets: u32) -> Self {
        Self(
            VkDescriptorPoolCreateInfo {
                sType: VkDescriptorPoolCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                maxSets: max_sets,
                poolSizeCount: 0,
                pPoolSizes: std::ptr::null(),
            },
            Vec::new(),
        )
    }

    pub const fn allow_individual_free(mut self) -> Self {
        self.0.flags |= VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT;
        self
    }

    pub fn with_reservations(mut self, new_sizes: Vec<VkDescriptorPoolSize>) -> Self {
        self.1 = new_sizes;
        self
    }

    pub fn reserve(mut self, size: VkDescriptorPoolSize) -> Self {
        self.1.push(size);
        self
    }

    pub fn reserve_all(mut self, size: impl IntoIterator<Item = VkDescriptorPoolSize>) -> Self {
        self.1.extend(size);
        self
    }

    /// Creates a descriptor pool object
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    #[implements]
    pub fn create<Device: crate::Device>(mut self, device: Device) -> crate::Result<DescriptorPoolObject<Device>>
    where
        Self: Sized,
    {
        self.0.poolSizeCount = self.1.len() as _;
        self.0.pPoolSizes = self.1.as_ptr() as *const _;

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_descriptor_pool(device.native_ptr(), &self.0, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| DescriptorPoolObject(h.assume_init(), device))
        }
    }
}

pub trait DescriptorPool: VkHandle<Handle = VkDescriptorPool> + DeviceChild {
    /// Allocate one or more descriptor sets
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    /// - VK_ERROR_FRAGMENTED_POOL
    #[implements]
    fn alloc(&mut self, layouts: &[&(impl DescriptorSetLayout + ?Sized)]) -> crate::Result<Vec<DescriptorSet>>
    where
        Self: VkHandleMut,
    {
        use crate::VkRawHandle;

        let layout_ptrs = layouts.iter().map(VkHandle::native_ptr).collect::<Vec<_>>();
        let ainfo = VkDescriptorSetAllocateInfo {
            sType: VkDescriptorSetAllocateInfo::TYPE,
            pNext: std::ptr::null(),
            descriptorPool: self.native_ptr_mut(),
            descriptorSetCount: layout_ptrs.len() as _,
            pSetLayouts: layout_ptrs.as_ptr(),
        };
        let mut hs = vec![VkDescriptorSet::NULL; layout_ptrs.len()];
        unsafe {
            crate::vkresolve::allocate_descriptor_sets(self.device().native_ptr(), &ainfo, hs.as_mut_ptr())
                .into_result()
                .map(|_| std::mem::transmute(hs))
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
    unsafe fn reset(&mut self) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        crate::vkresolve::reset_descriptor_pool(self.device().native_ptr(), self.native_ptr_mut(), 0)
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
    unsafe fn free(&mut self, sets: &[VkDescriptorSet]) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        crate::vkresolve::free_descriptor_sets(
            self.device().native_ptr(),
            self.native_ptr(),
            sets.len() as _,
            sets.as_ptr(),
        )
        .into_result()
        .map(drop)
    }
}
DerefContainerBracketImpl!(for DescriptorPool {});
GuardsImpl!(for DescriptorPool {});

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

    pub fn write_multiple<'r>(
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
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct OptionalSamplerObjectRef<'s>(
    VkSampler,
    core::marker::PhantomData<Option<&'s dyn VkHandle<Handle = VkSampler>>>,
);
impl<'s> OptionalSamplerObjectRef<'s> {
    pub const NONE: Self = Self(VkSampler::NULL, core::marker::PhantomData);

    pub fn new(r: &'s (impl VkHandle<Handle = VkSampler> + ?Sized)) -> Self {
        Self(r.native_ptr(), core::marker::PhantomData)
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptorBufferRef<'r>(
    VkDescriptorBufferInfo,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = VkBuffer>>,
);
impl<'r> DescriptorBufferRef<'r> {
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

    pub fn with_sampler(mut self, sampler: &'r (impl VkHandle<Handle = VkSampler> + ?Sized)) -> Self {
        self.0.sampler = sampler.native_ptr();
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
impl DescriptorContents<'_> {
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
            | DescriptorContents::InputAttachment(ref res) => (core::ptr::null(), res.as_ptr(), core::ptr::null()),
            DescriptorContents::UniformBuffer(ref res)
            | DescriptorContents::StorageBuffer(ref res)
            | DescriptorContents::UniformBufferDynamic(ref res)
            | DescriptorContents::StorageBufferDynamic(ref res) => (res.as_ptr(), core::ptr::null(), core::ptr::null()),
            DescriptorContents::UniformTexelBuffer(ref res) | DescriptorContents::StorageTexelBuffer(ref res) => {
                (core::ptr::null(), core::ptr::null(), res.as_ptr())
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

cfg_if! {
    if #[cfg(feature = "VK_KHR_descriptor_update_template")] {
        #[derive(VkHandle, VkObject, DeviceChild)]
        #[VkObject(type = VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR)]
        pub struct DescriptorUpdateTemplateObject<Device: crate::Device>(
            pub(crate) VkDescriptorUpdateTemplateKHR,
            #[parent] pub(crate) Device,
        );
        unsafe impl<Device: crate::Device + Sync> Sync for DescriptorUpdateTemplateObject<Device> {}
        unsafe impl<Device: crate::Device + Send> Send for DescriptorUpdateTemplateObject<Device> {}
        #[cfg(feature = "Implements")]
        impl<Device: crate::Device> Drop for DescriptorUpdateTemplateObject<Device> {
            fn drop(&mut self) {
                unsafe { self.1.destroy_descriptor_update_template_khr_fn().0(self.1.native_ptr(), self.0, std::ptr::null()); }
            }
        }
        impl<Device: crate::Device> DescriptorUpdateTemplate for DescriptorUpdateTemplateObject<Device> {}

        pub trait DescriptorUpdateTemplate: VkHandle<Handle = VkDescriptorUpdateTemplateKHR> + DeviceChild {
            #[cfg(feature = "Implements")]
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
