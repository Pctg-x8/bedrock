//! Vulkan Descriptors

use cfg_if::cfg_if;
use derives::implements;

#[cfg(feature = "Implements")]
use crate::VkHandleMut;
use crate::{vk::*, DeviceChild, LifetimeBound, VkObject};
use crate::{ImageLayout, ShaderStage, VkHandle, VulkanStructure};

DefineStdDeviceChildObject! {
    /// Opaque handle to a descriptor set layout object
    DescriptorSetLayoutObject(VkDescriptorSetLayout, VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT): DescriptorSetLayout { drop destroy_descriptor_set_layout }
}
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

DefineStdDeviceChildObject! {
    /// Opaque handle to a descriptor pool object
    DescriptorPoolObject(VkDescriptorPool, VK_OBJECT_TYPE_DESCRIPTOR_POOL): DescriptorPool { drop destroy_descriptor_pool }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
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
    std::marker::PhantomData<&'s dyn VkHandle<Handle = VkSampler>>,
);
impl<'s> SamplerObjectRef<'s> {
    pub fn new(x: &'s (impl VkHandle<Handle = VkSampler> + ?Sized)) -> Self {
        Self(x.native_ptr(), std::marker::PhantomData)
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
            .into_iter()
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
            .map(move |_| crate::DescriptorSetLayoutObject(h.assume_init(), device))
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum DescriptorSetLayoutBinding1<'s> {
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
impl<'s> DescriptorSetLayoutBinding1<'s> {
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
impl<'s> DescriptorSetLayoutBinding1<'s> {
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

/// Structure specifying descriptor pool size
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct DescriptorPoolSize(pub DescriptorType, pub u32);

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
    #[cfg(feature = "Implements")]
    fn alloc(&mut self, layouts: &[impl DescriptorSetLayout]) -> crate::Result<Vec<DescriptorSet>>
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
    #[cfg(feature = "Implements")]
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
    #[cfg(feature = "Implements")]
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
