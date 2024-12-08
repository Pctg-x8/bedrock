//! Vulkan Shading(Shader/Pipeline)

use derives::{bitflags_newtype, implements, transparent_marked};

use crate::ffi_helper::{opt_pointer, slice_as_ptr_empty_null, ArrayFFIExtensions};
use crate::{
    vk::*, DescriptorSetLayoutObjectRef, DeviceChild, DeviceChildHandle, GenericVulkanStructure, LifetimeBound,
    SubpassRef, VkDeviceChildNonExtDestroyable, VkHandle, VkHandleMut, VkHandleRef, VkObject, VkRawHandle,
    VulkanStructure, VulkanStructureAsRef,
};
use std::ffi::{c_void, CStr};
use std::marker::PhantomData;
use std::ops::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Hash)]
pub enum ShaderStage {
    Vertex = VK_SHADER_STAGE_VERTEX_BIT as _,
    TessellationControl = VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT as _,
    TessellationEvaluation = VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT as _,
    Geometry = VK_SHADER_STAGE_GEOMETRY_BIT as _,
    Fragment = VK_SHADER_STAGE_FRAGMENT_BIT as _,
    Compute = VK_SHADER_STAGE_COMPUTE_BIT as _,
}

/// Stencil comparison function
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompareOp {
    /// The test never passes
    Never = VK_COMPARE_OP_NEVER as _,
    /// The test passes when `Ref < Stencil`
    Less = VK_COMPARE_OP_LESS as _,
    /// The test passes when `Ref == Stencil`
    Equal = VK_COMPARE_OP_EQUAL as _,
    /// The test passes when `Ref <= Stencil`
    LessOrEqual = VK_COMPARE_OP_LESS_OR_EQUAL as _,
    /// The test passes when `Ref > Stencil`
    Greater = VK_COMPARE_OP_GREATER as _,
    /// The test passes when `Ref != Stencil`
    NotEqual = VK_COMPARE_OP_NOT_EQUAL as _,
    /// The test passes when `Ref >= Stencil`
    GreaterOrEqual = VK_COMPARE_OP_GREATER_OR_EQUAL as _,
    /// The test always passes
    Always = VK_COMPARE_OP_ALWAYS as _,
}
/// Stencil action function
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StencilOp {
    /// Keeps the current value
    Keep = VK_STENCIL_OP_KEEP as _,
    /// Sets the value to 0
    Zero = VK_STENCIL_OP_ZERO as _,
    /// Sets the value to `reference`
    Replace = VK_STENCIL_OP_REPLACE as _,
    /// Increments the current value and clamps to the maximum representable unsigned value
    IncrementClamp = VK_STENCIL_OP_INCREMENT_AND_CLAMP as _,
    /// Decrements the current value and clamps to 0
    DecrementClamp = VK_STENCIL_OP_DECREMENT_AND_CLAMP as _,
    /// Bitwise-inverts the current value
    Invert = VK_STENCIL_OP_INVERT as _,
    /// Increments the current value and wraps to 0 when the maximum value would have been exceeded
    IncrementWrap = VK_STENCIL_OP_INCREMENT_AND_WRAP as _,
    /// Decrements the current value and wraps to the maximum possible value when the value would go below 0
    DecrementWrap = VK_STENCIL_OP_DECREMENT_AND_WRAP as _,
}
/// Framebuffer logical operations
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOp {
    /// 0
    Clear = VK_LOGIC_OP_CLEAR as _,
    /// source & dest
    And = VK_LOGIC_OP_AND as _,
    /// source & ~dest
    AndReverse = VK_LOGIC_OP_AND_REVERSE as _,
    /// source
    Copy = VK_LOGIC_OP_COPY as _,
    /// ~source & dest
    AndInverted = VK_LOGIC_OP_AND_INVERTED as _,
    /// dest
    NoOp = VK_LOGIC_OP_NO_OP as _,
    /// source ^ dest
    Xor = VK_LOGIC_OP_XOR as _,
    /// source | dest
    Or = VK_LOGIC_OP_OR as _,
    /// ~(source | dest)
    Nor = VK_LOGIC_OP_NOR as _,
    /// ~(source ^ dest)
    Equivalent = VK_LOGIC_OP_EQUIVALENT as _,
    /// ~dest
    Invert = VK_LOGIC_OP_INVERT as _,
    /// source | ~dest
    OrReverse = VK_LOGIC_OP_OR_REVERSE as _,
    /// ~source
    CopyInverted = VK_LOGIC_OP_COPY_INVERTED as _,
    /// ~source | dest
    OrInverted = VK_LOGIC_OP_OR_INVERTED as _,
    /// ~(source & dest)
    Nand = VK_LOGIC_OP_NAND as _,
    /// 1
    Set = VK_LOGIC_OP_SET as _,
}
/// Bitmask specifying sets of stencil state for which to update the compare mask
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum StencilFaceMask {
    /// Only the front set of stencil state
    Front = VK_STENCIL_FACE_FRONT_BIT as _,
    /// Only the back set of stencil state
    Back = VK_STENCIL_FACE_BACK_BIT as _,
    /// Both sets of stencil state
    Both = VK_STENCIL_FRONT_AND_BACK as _,
}

/// Opaque handle to a shader module object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkShaderModule::OBJECT_TYPE)]
pub struct ShaderModuleObject<Device: VkHandle<Handle = VkDevice>>(VkShaderModule, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for ShaderModuleObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for ShaderModuleObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for ShaderModuleObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> ShaderModule for ShaderModuleObject<Device> {}

impl<Device: VkHandle<Handle = VkDevice>> ShaderModuleObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkShaderModule, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkShaderModule, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}

pub trait ShaderModule: VkHandle<Handle = VkShaderModule> {
    #[inline(always)]
    fn with_entry_point<'m>(&'m self, entry_point: &'m CStr) -> PipelineShader<'m, Self> {
        PipelineShader::new(self, entry_point)
    }
}
DerefContainerBracketImpl!(for ShaderModule {});
GuardsImpl!(for ShaderModule {});

/// Opaque handle to a pipeline cache object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkPipelineCache::OBJECT_TYPE)]
pub struct PipelineCacheObject<Device: VkHandle<Handle = VkDevice>>(VkPipelineCache, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for PipelineCacheObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for PipelineCacheObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for PipelineCacheObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for PipelineCacheObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: VkHandle<Handle = VkDevice>> PipelineCache for PipelineCacheObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> PipelineCacheMut for PipelineCacheObject<Device> {}

impl<Device: VkHandle<Handle = VkDevice>> PipelineCacheObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkPipelineCache, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkPipelineCache, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}

pub trait PipelineCache: VkHandle<Handle = VkPipelineCache> + DeviceChildHandle {
    #[inline(always)]
    fn as_transparent_ref(&self) -> VkHandleRef<VkPipelineCache> {
        VkHandleRef::new(self)
    }

    /// Get the size of the data store from a pipeline cache
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline]
    fn data_len(&self) -> crate::Result<usize> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_pipeline_cache_data(
                self.device_handle(),
                self.native_ptr(),
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Get the content of the data store from a pipeline cache
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline]
    fn data_into(&self, store: &mut [u8]) -> crate::Result<()> {
        let mut dl = store.len();
        unsafe {
            crate::vkfn::get_pipeline_cache_data(
                self.device_handle(),
                self.native_ptr(),
                &mut dl,
                store.as_mut_ptr() as _,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Get the data store from a pipeline cache
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline]
    fn data(&self) -> crate::Result<Vec<u8>> {
        let len = self.data_len()?;
        if len == 0 {
            // no data
            return Ok(Vec::new());
        }

        let mut b = Vec::with_capacity(len);
        unsafe {
            b.set_len(len);
        }
        self.data_into(&mut b[..])?;

        Ok(b)
    }
}
DerefContainerBracketImpl!(for PipelineCache {});
GuardsImpl!(for PipelineCache {});

pub trait PipelineCacheMut: PipelineCache + VkHandleMut {
    /// Combine the data stores of pipeline caches into `self`
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline]
    fn merge(&mut self, srcs: &[VkHandleRef<VkPipelineCache>]) -> crate::Result<()> {
        unsafe {
            crate::vkfn::merge_pipeline_caches(
                self.device_handle(),
                self.native_ptr_mut(),
                srcs.len() as _,
                srcs.as_ptr_empty_null() as _,
            )
            .into_result()
            .map(drop)
        }
    }
}
DerefContainerBracketImpl!(for mut PipelineCacheMut {});
GuardsImpl!(for mut PipelineCacheMut {});

/// Opaque handle to a pipeline layout object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkPipelineLayout::OBJECT_TYPE)]
pub struct PipelineLayoutObject<Device: VkHandle<Handle = VkDevice>>(VkPipelineLayout, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for PipelineLayoutObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for PipelineLayoutObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for PipelineLayoutObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for PipelineLayoutObject<Device> {
    #[inline]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for PipelineLayoutObject<Device> {
    type ConcreteDevice = Device;

    #[inline]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: VkHandle<Handle = VkDevice>> PipelineLayout for PipelineLayoutObject<Device> {}

impl VkPushConstantRange {
    pub const fn new(shader_stage: VkShaderStageFlags, byte_range: Range<u32>) -> Self {
        Self {
            stageFlags: shader_stage,
            offset: byte_range.start,
            size: byte_range.end - byte_range.start,
        }
    }

    pub const fn for_type<T>(shader_stage: VkShaderStageFlags, offset: u32) -> Self {
        Self {
            stageFlags: shader_stage,
            offset,
            size: core::mem::size_of::<T>() as _,
        }
    }
}

/// Builder struct for PipelineLayout object
#[repr(transparent)]
pub struct PipelineLayoutBuilder<'l> {
    raw: VkPipelineLayoutCreateInfo,
    descriptor_set_layouts: core::marker::PhantomData<&'l [DescriptorSetLayoutObjectRef<'l>]>,
    push_constant_ranges: core::marker::PhantomData<&'l [VkPushConstantRange]>,
}
impl<'l> PipelineLayoutBuilder<'l> {
    /// An empty builder struct
    pub const EMPTY: Self = Self::new(&[], &[]);

    /// Creates a new builder struct and initialize it with given parameters
    #[inline(always)]
    pub const fn new(
        descriptor_set_layouts: &'l [crate::DescriptorSetLayoutObjectRef<'l>],
        push_constant_ranges: &'l [VkPushConstantRange],
    ) -> Self {
        Self {
            raw: VkPipelineLayoutCreateInfo {
                sType: VkPipelineLayoutCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                setLayoutCount: descriptor_set_layouts.len() as _,
                pSetLayouts: slice_as_ptr_empty_null(descriptor_set_layouts) as _,
                pushConstantRangeCount: push_constant_ranges.len() as _,
                pPushConstantRanges: slice_as_ptr_empty_null(push_constant_ranges) as _,
            },
            descriptor_set_layouts: core::marker::PhantomData,
            push_constant_ranges: core::marker::PhantomData,
        }
    }

    /// Creates a new pipeline layout object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline]
    pub fn create<D: crate::Device>(&self, device: D) -> crate::Result<PipelineLayoutObject<D>> {
        unsafe { PipelineLayoutObject::new_raw(device, &self.raw) }
    }
}

impl<Device: VkHandle<Handle = VkDevice>> PipelineLayoutObject<Device> {
    /// Creates a new pipeline layout object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// no guarantees will be provided (simply calls under api)
    #[implements]
    #[inline]
    pub unsafe fn new_raw(device: Device, info: &VkPipelineLayoutCreateInfo) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        crate::vkfn::create_pipeline_layout(device.native_ptr(), info, core::ptr::null(), h.as_mut_ptr())
            .into_result()?;

        Ok(Self::manage(h.assume_init(), device))
    }

    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkPipelineLayout, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkPipelineLayout, Device) {
        let r = unsafe { (self.0, core::ptr::read(&self.1)) };
        core::mem::forget(self);

        r
    }
}

pub trait PipelineLayout: VkHandle<Handle = VkPipelineLayout> {}
DerefContainerBracketImpl!(for PipelineLayout {});
GuardsImpl!(for PipelineLayout {});

/// Opaque handle to a pipeline object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkPipeline::OBJECT_TYPE)]
pub struct PipelineObject<Device: VkHandle<Handle = VkDevice>>(VkPipeline, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for PipelineObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for PipelineObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for PipelineObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for PipelineObject<Device> {
    #[inline]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for PipelineObject<Device> {
    type ConcreteDevice = Device;

    #[inline]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: VkHandle<Handle = VkDevice>> Pipeline for PipelineObject<Device> {}

impl<Device: VkHandle<Handle = VkDevice>> PipelineObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkPipeline, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkPipeline, Device) {
        let r = unsafe { (self.0, core::ptr::read(&self.1)) };
        core::mem::forget(self);

        r
    }
}

pub trait Pipeline: VkHandle<Handle = VkPipeline> {}
DerefContainerBracketImpl!(for Pipeline {});
GuardsImpl!(for Pipeline {});

/// Disabled, Specified in the command buffer or Specified in the pipeline state
pub enum SwitchOrDynamicState<T> {
    Disabled,
    Dynamic,
    Static(T),
}
impl<T> SwitchOrDynamicState<T> {
    #[inline(always)]
    const fn is_dynamic(&self) -> bool {
        matches!(self, Self::Dynamic)
    }

    #[inline(always)]
    const fn is_enabled(&self) -> bool {
        !matches!(self, Self::Disabled)
    }
}

/// VkPipelineDynamicStateCreateInfo builder
#[derive(Clone)]
pub struct PipelineDynamicStates(Vec<VkDynamicState>);
impl From<Vec<VkDynamicState>> for PipelineDynamicStates {
    fn from(mut v: Vec<VkDynamicState>) -> Self {
        // needs to be sorted for efficiency enable/disable ops
        v.sort();
        PipelineDynamicStates(v)
    }
}
impl<'d> Into<LifetimeBound<'d, VkPipelineDynamicStateCreateInfo>> for &'d PipelineDynamicStates {
    fn into(self) -> LifetimeBound<'d, VkPipelineDynamicStateCreateInfo> {
        LifetimeBound::new(VkPipelineDynamicStateCreateInfo {
            sType: VkPipelineDynamicStateCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            dynamicStateCount: self.0.len() as _,
            pDynamicStates: self.0.as_ptr_empty_null(),
        })
    }
}
impl PipelineDynamicStates {
    /// Creates an empty PipelineDynamicStates
    #[allow(clippy::new_without_default)]
    #[inline(always)]
    pub const fn new() -> Self {
        PipelineDynamicStates(Vec::new())
    }

    /// Enables using a dynamic state
    #[inline(always)]
    pub fn enable(&mut self, v: VkDynamicState) {
        if let Err(n) = self.0.binary_search(&v) {
            self.0.insert(n, v);
        }
    }

    /// Disables using a dynamic state
    #[inline(always)]
    pub fn disable(&mut self, v: VkDynamicState) {
        if let Ok(n) = self.0.binary_search(&v) {
            self.0.remove(n);
        }
    }

    /// Sets enable or disable state of a dynamic state
    #[inline(always)]
    pub fn set(&mut self, v: VkDynamicState, enable: bool) {
        if enable {
            self.enable(v);
        } else {
            self.disable(v);
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.0.clear();
    }
}
impl<'d, 's, Layout, RenderPass> NonDerivedGraphicsPipelineBuilder<'d, 's, Layout, RenderPass>
where
    Layout: PipelineLayout,
    RenderPass: crate::RenderPass,
{
    /// Gets a mutable reference to the dynamic state settings
    pub const fn dynamic_states_mut(&mut self) -> &mut PipelineDynamicStates {
        &mut self.dynamic_state_flags
    }
}

impl VkVertexInputBindingDescription {
    /// Consumed per vertex with stride
    pub const fn per_vertex(binding: u32, stride: u32) -> Self {
        Self {
            binding,
            stride,
            inputRate: VK_VERTEX_INPUT_RATE_VERTEX,
        }
    }

    /// Consumed per instance with stride
    pub const fn per_instance(binding: u32, stride: u32) -> Self {
        Self {
            binding,
            stride,
            inputRate: VK_VERTEX_INPUT_RATE_INSTANCE,
        }
    }

    /// Consumed per vertex the structured data
    pub const fn per_vertex_typed<T>(binding: u32) -> Self {
        Self::per_vertex(binding, core::mem::size_of::<T>() as _)
    }

    /// Consumed per instance the structured data
    pub const fn per_instance_typed<T>(binding: u32) -> Self {
        Self::per_instance(binding, core::mem::size_of::<T>() as _)
    }
}

pub trait SpecializationConstants {
    const ENTRIES: &'static [VkSpecializationMapEntry];

    fn as_ptr(&self) -> *const c_void;
}
DerefContainerBracketImpl!(for SpecializationConstants {
    const ENTRIES: &'static [VkSpecializationMapEntry] = T::ENTRIES;

    #[inline(always)]
    fn as_ptr(&self) -> *const c_void {
        T::as_ptr(&**self)
    }
});

pub trait PipelineShaderProvider {
    type ExtraStorage<'d>
    where
        Self: 'd;

    fn base_struct<'d, 's>(
        &'d self,
        stage: ShaderStage,
        extras: &'s Self::ExtraStorage<'d>,
    ) -> PipelineShaderStage<'d, 's>;
    fn make_extras<'d>(&'d self) -> Self::ExtraStorage<'d>;
}
impl<T: PipelineShaderProvider> PipelineShaderProvider for &'_ T {
    type ExtraStorage<'d>
        = T::ExtraStorage<'d>
    where
        Self: 'd;

    #[inline(always)]
    fn base_struct<'d, 's>(
        &'d self,
        stage: ShaderStage,
        extras: &'s Self::ExtraStorage<'d>,
    ) -> PipelineShaderStage<'d, 's> {
        T::base_struct(&self, stage, extras)
    }
    #[inline(always)]
    fn make_extras<'d>(&'d self) -> Self::ExtraStorage<'d> {
        T::make_extras(&**self)
    }
}
impl<T: PipelineShaderProvider> PipelineShaderProvider for &'_ mut T {
    type ExtraStorage<'d>
        = T::ExtraStorage<'d>
    where
        Self: 'd;

    #[inline(always)]
    fn base_struct<'d, 's>(
        &'d self,
        stage: ShaderStage,
        extras: &'s Self::ExtraStorage<'d>,
    ) -> PipelineShaderStage<'d, 's> {
        T::base_struct(&self, stage, extras)
    }
    #[inline(always)]
    fn make_extras<'d>(&'d self) -> Self::ExtraStorage<'d> {
        T::make_extras(&**self)
    }
}
impl<T: PipelineShaderProvider> PipelineShaderProvider for Box<T> {
    type ExtraStorage<'d>
        = T::ExtraStorage<'d>
    where
        Self: 'd;

    #[inline(always)]
    fn base_struct<'d, 's>(
        &'d self,
        stage: ShaderStage,
        extras: &'s Self::ExtraStorage<'d>,
    ) -> PipelineShaderStage<'d, 's> {
        T::base_struct(&self, stage, extras)
    }
    #[inline(always)]
    fn make_extras<'d>(&'d self) -> Self::ExtraStorage<'d> {
        T::make_extras(&**self)
    }
}

pub struct PipelineShader<'m, M: 'm + ShaderModule + ?Sized>(&'m M, &'m CStr);
impl<'m, M: 'm + ShaderModule + ?Sized> PipelineShader<'m, M> {
    #[inline(always)]
    pub const fn new(module: &'m M, entry_point: &'m CStr) -> Self {
        Self(module, entry_point)
    }

    #[inline(always)]
    pub fn on_stage<'s>(self, stage: ShaderStage) -> PipelineShaderStage<'m, 's> {
        PipelineShaderStage::new(stage, self.0, self.1)
    }
}
impl<M: ShaderModule + ?Sized> PipelineShaderProvider for PipelineShader<'_, M> {
    type ExtraStorage<'d>
        = ()
    where
        Self: 'd;

    fn base_struct<'d, 's>(
        &'d self,
        stage: ShaderStage,
        _extras: &'s Self::ExtraStorage<'d>,
    ) -> PipelineShaderStage<'d, 's> {
        PipelineShaderStage::new(stage, &self.0, &self.1)
    }
    fn make_extras<'d>(&'d self) -> Self::ExtraStorage<'d> {}
}

#[transparent_marked]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineShaderStage<'d, 's>(
    pub(crate) VkPipelineShaderStageCreateInfo,
    core::marker::PhantomData<(
        Option<&'d dyn VulkanStructureAsRef>,
        &'d dyn VkHandle<Handle = VkShaderModule>,
        &'d core::ffi::CStr,
        Option<&'s SpecializationInfo<'d>>,
    )>,
);
impl<'d, 's> PipelineShaderStage<'d, 's> {
    #[inline(always)]
    pub fn new(
        stage: ShaderStage,
        shader: &'d (impl ShaderModule + ?Sized),
        entrypoint_name: &'d core::ffi::CStr,
    ) -> Self {
        Self(
            VkPipelineShaderStageCreateInfo {
                sType: VkPipelineShaderStageCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                stage: stage as _,
                module: shader.native_ptr(),
                pName: entrypoint_name.as_ptr(),
                pSpecializationInfo: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }

    pub const fn with_specialization_info(mut self, info: &'s SpecializationInfo<'d>) -> Self {
        self.0.pSpecializationInfo = info as *const _ as _;
        self
    }
}

#[transparent_marked]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecializationInfo<'d>(
    VkSpecializationInfo,
    core::marker::PhantomData<(&'d [VkSpecializationMapEntry], &'d dyn core::any::Any)>,
);
impl<'d> SpecializationInfo<'d> {
    pub const fn new<T: 'd>(entries: &'d [VkSpecializationMapEntry], data: &'d T) -> Self {
        Self(
            VkSpecializationInfo {
                mapEntryCount: entries.len() as _,
                pMapEntries: slice_as_ptr_empty_null(entries),
                dataSize: core::mem::size_of::<T>() as _,
                pData: data as *const _ as _,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkSpecializationInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkSpecializationInfo {
        self.0
    }
}

/// PipelineStateDesc: Shader Stages and Input descriptions
#[derive(Clone)]
pub struct VertexProcessingStages<'d, 's> {
    shader_stages: &'d [PipelineShaderStage<'d, 's>],
    vi: VkPipelineVertexInputStateCreateInfo,
    ia: VkPipelineInputAssemblyStateCreateInfo,
    _holder: PhantomData<(
        &'d [VkVertexInputBindingDescription],
        &'d [VkVertexInputAttributeDescription],
    )>,
}
impl<'d, 's> VertexProcessingStages<'d, 's> {
    pub const fn new(
        shader_stages: &'d [PipelineShaderStage<'d, 's>],
        vbind: &'d [VkVertexInputBindingDescription],
        vattr: &'d [VkVertexInputAttributeDescription],
        primitive_topo: VkPrimitiveTopology,
    ) -> Self {
        Self {
            shader_stages,
            vi: VkPipelineVertexInputStateCreateInfo {
                sType: VkPipelineVertexInputStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                vertexBindingDescriptionCount: vbind.len() as _,
                pVertexBindingDescriptions: slice_as_ptr_empty_null(vbind),
                vertexAttributeDescriptionCount: vattr.len() as _,
                pVertexAttributeDescriptions: slice_as_ptr_empty_null(vattr),
            },
            ia: VkPipelineInputAssemblyStateCreateInfo {
                sType: VkPipelineInputAssemblyStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                topology: primitive_topo,
                primitiveRestartEnable: VK_FALSE,
            },
            _holder: PhantomData,
        }
    }

    /// Update the vertex binding description
    pub const fn vertex_binding(&mut self, vbind: &'d [VkVertexInputBindingDescription]) -> &mut Self {
        self.vi.vertexBindingDescriptionCount = vbind.len() as _;
        self.vi.pVertexBindingDescriptions = slice_as_ptr_empty_null(vbind);
        self
    }

    /// Update the vertex attribute description
    pub const fn vertex_attributes(&mut self, vattr: &'d [VkVertexInputAttributeDescription]) -> &mut Self {
        self.vi.vertexAttributeDescriptionCount = vattr.len() as _;
        self.vi.pVertexAttributeDescriptions = slice_as_ptr_empty_null(vattr);
        self
    }

    /// Update the vertex input description
    pub const fn vertex_input(
        &mut self,
        vbind: &'d [VkVertexInputBindingDescription],
        vattr: &'d [VkVertexInputAttributeDescription],
    ) -> &mut Self {
        self.vertex_binding(vbind).vertex_attributes(vattr)
    }

    /// Controls whether a special vertex index value is treated as restarting the assembly of primitives.
    /// This enable only applies to indexed draws, and the special index value is either
    ///
    /// * `0xffff_ffff` when the `indexType` parameter of `vkCmdBindIndexBuffer` is equal to `VK_INDEX_TYPE_UINT32`, or
    /// * `0xffff` when `indexType` is equal to `VK_INDEX_TYPE_UINT16`.
    ///
    /// Primitive restart is not allowed for "list" topologies.
    pub const fn enable_primitive_restart(&mut self, w: bool) -> &mut Self {
        self.ia.primitiveRestartEnable = w as _;
        self
    }

    /// Update the input primitive topology
    pub const fn primitive_topology(&mut self, topo: VkPrimitiveTopology) -> &mut Self {
        self.ia.topology = topo;
        self
    }
}

/// PipelineStateDesc: Rasterization State
#[derive(Clone)]
pub struct RasterizationState {
    base: VkPipelineRasterizationStateCreateInfo,
    is_dynamic_depth_bias: bool,
    is_dynamic_line_width: bool,
    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    conservative: Option<VkPipelineRasterizationConservativeStateCreateInfoEXT>,
    #[cfg(feature = "VK_KHR_line_rasterization")]
    line_rasterization: Option<VkPipelineRasterizationLineStateCreateInfoKHR>,
}
impl Default for RasterizationState {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
impl RasterizationState {
    pub const fn new() -> Self {
        Self {
            base: VkPipelineRasterizationStateCreateInfo {
                sType: VkPipelineRasterizationStateCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                depthClampEnable: VK_FALSE,
                rasterizerDiscardEnable: VK_FALSE,
                polygonMode: VK_POLYGON_MODE_FILL,
                cullMode: VK_CULL_MODE_NONE,
                frontFace: VK_FRONT_FACE_CLOCKWISE,
                depthBiasEnable: VK_FALSE,
                depthBiasConstantFactor: 0.0,
                depthBiasClamp: 0.0,
                depthBiasSlopeFactor: 1.0,
                lineWidth: 1.0,
            },
            is_dynamic_depth_bias: false,
            is_dynamic_line_width: false,
            #[cfg(feature = "VK_EXT_conservative_rasterization")]
            conservative: None,
            #[cfg(feature = "VK_KHR_line_rasterization")]
            line_rasterization: None,
        }
    }

    fn apply_dynamic_states(&self, st: &mut PipelineDynamicStates) {
        st.set(VK_DYNAMIC_STATE_DEPTH_BIAS, self.is_dynamic_depth_bias);
        st.set(VK_DYNAMIC_STATE_LINE_WIDTH, self.is_dynamic_line_width);
    }

    #[allow(unused_assignments)]
    fn make_chained(&mut self) -> &VkPipelineRasterizationStateCreateInfo {
        #[allow(unused_variables, unused_mut)]
        let mut base: &mut GenericVulkanStructure = self.base.as_generic_mut();

        #[cfg(feature = "VK_EXT_conservative_rasterization")]
        if let Some(ref mut c) = self.conservative {
            base.pNext = c as *const _ as _;
            base = c.as_generic_mut();
        }
        #[cfg(feature = "VK_KHR_line_rasterization")]
        if let Some(ref mut c) = self.line_rasterization {
            base.pNext = &c as *const _ as _;
            base = c.as_generic_mut();
        }

        &self.base
    }

    /// Controls whether to clamp the fragment's depth values instead of clipping primitives to the z planes of the frustum,
    /// as described in `Primitive Clipping` in Vulkan Specification
    pub const fn depth_clamp_enable(&mut self, enable: bool) -> &mut Self {
        self.base.depthClampEnable = enable as _;
        self
    }

    /// Controls whether primitives are discarded immediately before the rasterization stage
    pub const fn rasterizer_discard_enable(&mut self, enable: bool) -> &mut Self {
        self.base.rasterizerDiscardEnable = enable as _;
        self
    }

    /// The triangle rendering mode
    pub const fn polygon_mode(&mut self, mode: VkPolygonMode) -> &mut Self {
        self.base.polygonMode = mode;
        self
    }

    /// The triangle facing direction used for primitive culling
    pub const fn cull_mode(&mut self, mode: VkCullModeFlags) -> &mut Self {
        self.base.cullMode = mode;
        self
    }

    /// The front-facing triangle orientation to be used for culling
    pub const fn front_face(&mut self, face: VkFrontFace) -> &mut Self {
        self.base.frontFace = face;
        self
    }

    /// Specify `None` to disable to bias fragment depth values.
    /// Tuple Member: (`ConstantFactor`, `Clamp`, `SlopeFactor`)
    ///
    /// - `ConstantFactor`: A scalar factor controlling the constant depth value added to each fragment
    /// - `Clamp`: The maximum (or minimum) depth bias of a fragment
    /// - `SlopeFactor`: A scalar factor applied to a fragment's slope in depth bias calculations
    pub const fn depth_bias(&mut self, opts: SwitchOrDynamicState<(f32, f32, f32)>) -> &mut Self {
        self.base.depthBiasEnable = opts.is_enabled() as _;
        self.is_dynamic_depth_bias = opts.is_dynamic();
        if let SwitchOrDynamicState::Static((cf, c, sf)) = opts {
            self.base.depthBiasConstantFactor = cf;
            self.base.depthBiasClamp = c;
            self.base.depthBiasSlopeFactor = sf;
        }
        self
    }

    /// The width of rasterized line segments. Specifying `None` means that the `lineWidth` parameter is a dynamic state.
    pub const fn line_width(&mut self, width: Option<f32>) -> &mut Self {
        self.is_dynamic_line_width = width.is_none();
        self.base.lineWidth = match width {
            Some(x) => x,
            None => 0.0,
        };
        self
    }

    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    /// Sets conservative rasterization mode to use.
    pub const fn conservative_rasterization_mode(
        &mut self,
        mode: VkConservativeRasterizationModeEXT,
        extra: Option<f32>,
    ) -> &mut Self {
        if self.conservative.is_none() {
            self.conservative = Some(VkPipelineRasterizationConservativeStateCreateInfoEXT {
                sType: VkPipelineRasterizationConservativeStateCreateInfoEXT::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                conservativeRasterizationMode: VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT,
                extraPrimitiveOverestimationSize: 0.0,
            });
        }

        let r = self.conservative.as_mut().unwrap();
        r.conservativeRasterizationMode = mode as _;
        if let Some(x) = extra {
            r.extraPrimitiveOverestimationSize = x;
        }

        self
    }

    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    /// [VK_EXT_conservative_rasterization] Disables conservative rasterization.
    pub const fn disable_conservative_rasterization(&mut self) -> &mut Self {
        self.conservative = None;
        self
    }

    #[cfg(feature = "VK_KHR_line_rasterization")]
    /// Sets line rasterization state
    pub const fn line_state(&mut self, state: VkPipelineRasterizationLineStateCreateInfoKHR) -> &mut Self {
        self.line_rasterization = Some(state);
        self
    }

    #[cfg(feature = "VK_KHR_line_rasterization")]
    /// Clears line rasterization state
    pub const fn clear_line_state(&mut self) -> &mut Self {
        self.line_rasterization = None;
        self
    }
}

#[cfg(feature = "VK_KHR_line_rasterization")]
impl VkPipelineRasterizationLineStateCreateInfoKHR {
    pub const fn new(mode: VkLineRasterizationModeKHR) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            lineRasterizationMode: mode,
            stippledLineEnable: false as _,
            lineStippleFactor: 0,
            lineStipplePattern: 0,
        }
    }

    pub const fn stippled(mut self, factor: u32, pattern: u16) -> Self {
        self.stippledLineEnable = true as _;
        self.lineStippleFactor = factor;
        self.lineStipplePattern = pattern;

        self
    }
}

impl VkPipelineTessellationStateCreateInfo {
    pub const fn new(patch_control_points: u32) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            patchControlPoints: patch_control_points,
        }
    }
}

/// PipelineStateDesc: Multisample State
#[repr(transparent)]
#[derive(Clone)]
pub struct MultisampleState<'d> {
    data: VkPipelineMultisampleStateCreateInfo,
    samplemask_lifetime_binder: PhantomData<&'d [VkSampleMask]>,
}
impl<'d> MultisampleState<'d> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        MultisampleState {
            data: VkPipelineMultisampleStateCreateInfo {
                sType: VkPipelineMultisampleStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                rasterizationSamples: 1,
                sampleShadingEnable: VK_FALSE,
                minSampleShading: 1.0,
                pSampleMask: core::ptr::null(),
                alphaToCoverageEnable: VK_FALSE,
                alphaToOneEnable: VK_FALSE,
            },
            samplemask_lifetime_binder: PhantomData,
        }
    }

    /// Specifies the number of samples per pixel used in rasterization. default=1
    pub const fn rasterization_samples(&mut self, samples: usize) -> &mut Self {
        self.data.rasterizationSamples = samples as _;
        self
    }

    /// A bitmask of static coverage information that is ANDed with the coverage information generated
    /// during rasterization, as described in [Sample Mask](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#fragops-samplemask).
    pub fn sample_mask(&mut self, mask: &'d [VkSampleMask]) -> &mut Self {
        if mask.is_empty() {
            self.data.pSampleMask = std::ptr::null();
        } else {
            assert_eq!(mask.len(), (self.data.rasterizationSamples as usize + 31) / 32);
            self.data.pSampleMask = mask.as_ptr_empty_null();
        }
        self
    }

    /// Specifies a minimum fraction of sample shading(must be in the range [0, 1]).
    /// Pass a `None` to disable [Sample Shading](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#primsrast-sampleshading).
    pub const fn sample_shading(&mut self, min_sample_shading: Option<f32>) -> &mut Self {
        self.data.sampleShadingEnable = min_sample_shading.is_some() as _;
        if let Some(m) = min_sample_shading {
            assert!(
                0.0 <= m && m <= 1.0,
                "Invalid usage: VkPipelineMultisampleStateCreateInfo::minSampleShading must be in the range [0, 1]"
            );
            self.data.minSampleShading = m as _;
        }
        self
    }

    /// Controls whether a temporary coverage value is generated based on the alpha component of the fragment's
    /// first color output as specified in the [Multisample Coverage](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#fragops-covg) section.
    pub const fn enable_alpha_to_coverage(&mut self, w: bool) -> &mut Self {
        self.data.alphaToCoverageEnable = w as _;
        self
    }

    /// Controls whether the alpha component of the fragment's first color output is replaced with one as described in
    /// [Multisample Coverage](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#fragops-covg).
    pub const fn replace_alpha_to_one(&mut self, w: bool) -> &mut Self {
        self.data.alphaToOneEnable = w as _;
        self
    }
}
impl<'d> Into<LifetimeBound<'d, VkPipelineMultisampleStateCreateInfo>> for MultisampleState<'d> {
    fn into(self) -> LifetimeBound<'d, VkPipelineMultisampleStateCreateInfo> {
        LifetimeBound::new(self.data)
    }
}

/// Depth/Stencil State
#[transparent_marked]
#[derive(Debug, Clone, PartialEq)]
pub struct DepthStencilState(VkPipelineDepthStencilStateCreateInfo);
impl DepthStencilState {
    /// Creates an empty DepthStencilState
    pub const fn new() -> Self {
        Self(VkPipelineDepthStencilStateCreateInfo {
            sType: VkPipelineDepthStencilStateCreateInfo::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            depthTestEnable: VK_FALSE,
            depthWriteEnable: VK_FALSE,
            depthCompareOp: VK_COMPARE_OP_LESS,
            depthBoundsTestEnable: VK_FALSE,
            stencilTestEnable: VK_FALSE,
            front: VkStencilOpState {
                failOp: VK_STENCIL_OP_KEEP,
                passOp: VK_STENCIL_OP_KEEP,
                depthFailOp: VK_STENCIL_OP_KEEP,
                compareOp: VK_COMPARE_OP_ALWAYS,
                compareMask: 0,
                writeMask: 0,
                reference: 0,
            },
            back: VkStencilOpState {
                failOp: VK_STENCIL_OP_KEEP,
                passOp: VK_STENCIL_OP_KEEP,
                depthFailOp: VK_STENCIL_OP_KEEP,
                compareOp: VK_COMPARE_OP_ALWAYS,
                compareMask: 0,
                writeMask: 0,
                reference: 0,
            },
            minDepthBounds: 0.0,
            maxDepthBounds: 1.0,
        })
    }

    /// Constructs the state structure from raw data
    pub const unsafe fn from_raw(raw: VkPipelineDepthStencilStateCreateInfo) -> Self {
        Self(raw)
    }

    /// Unpacks the raw data
    pub const fn into_raw(self) -> VkPipelineDepthStencilStateCreateInfo {
        self.0
    }

    /// Configures depth operation
    ///
    /// Controls whether depth testing is enabled, depth writes are enabled, and the comparison operator used in the depth test
    /// Specifying `None` to `compare_to` disables depth testing
    pub const fn config_depth(mut self, compare_op: Option<CompareOp>, write_enable: bool) -> Self {
        self.0.depthTestEnable = compare_op.is_some() as _;
        self.0.depthCompareOp = match compare_op {
            Some(x) => x,
            None => CompareOp::Always,
        } as _;
        self.0.depthWriteEnable = write_enable as _;

        self
    }

    /// Disables depth testing
    pub const fn disable_depth_test(mut self) -> Self {
        self.0.depthTestEnable = false as _;
        self
    }

    /// Controls whether depth writes are enabled, or always disabled
    pub const fn depth_write(mut self, enable: bool) -> Self {
        self.0.depthWriteEnable = enable as _;
        self
    }

    /// Sets depth bounds. Specifying `None` to disable depth bounds test
    pub const fn depth_bounds(mut self, bounds: Option<core::ops::Range<f32>>) -> Self {
        self.0.depthBoundsTestEnable = bounds.is_some() as _;
        self.0.minDepthBounds = match bounds {
            Some(ref r) => r.start,
            None => 0.0,
        };
        self.0.maxDepthBounds = match bounds {
            Some(ref r) => r.end,
            None => 0.0,
        };

        self
    }

    /// Controls whether stencil testing is enabled
    pub const fn stencil_test(mut self, enable: bool) -> Self {
        self.0.stencilTestEnable = enable as _;
        self
    }

    /// Sets the state parameters for the front-face stencil test
    pub const fn stencil_state_front(mut self, state: VkStencilOpState) -> Self {
        self.0.front = state;
        self
    }

    /// Sets the state parameters for the back-face stencil test
    pub const fn stencil_state_back(mut self, state: VkStencilOpState) -> Self {
        self.0.back = state;
        self
    }
}

pub trait GraphicsPipelineBuilder {
    type ExtraStorage;

    fn build(&mut self, extras: &Self::ExtraStorage) -> VkGraphicsPipelineCreateInfo;
    /// Builds extra values needed by constructing the Struct.
    ///
    /// Values live until the struct will be consumed.
    fn make_extras(&self) -> Self::ExtraStorage;

    /// Create a graphics pipeline
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn create<Device: crate::Device>(
        &mut self,
        device: Device,
        cache: Option<&impl PipelineCache>,
    ) -> crate::Result<PipelineObject<Device>> {
        let extras = self.make_extras();
        let cinfo = self.build(&extras);

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_graphics_pipelines(
                device.native_ptr(),
                cache.map_or(VkPipelineCache::NULL, VkHandle::native_ptr),
                1,
                &cinfo,
                core::ptr::null(),
                h.as_mut_ptr(),
            )
            .into_result()
            .map(|_| PipelineObject(h.assume_init(), device))
        }
    }
}

/// Builder struct to construct a `Pipeline` for graphics operations
#[derive(Clone)]
pub struct NonDerivedGraphicsPipelineBuilder<
    'd,
    's,
    Layout: PipelineLayout,
    RenderPass: 'd + crate::RenderPass + ?Sized,
> {
    flags: VkPipelineCreateFlags,
    _layout: Layout,
    rp: &'d RenderPass,
    subpass: u32,
    vp: VertexProcessingStages<'d, 's>,
    rasterizer_state: RasterizationState,
    tess_state: Option<Box<VkPipelineTessellationStateCreateInfo>>,
    viewport_state: Option<ViewportState<'d>>,
    ms_state: Option<MultisampleState<'d>>,
    ds_state: Option<DepthStencilState>,
    color_blending: Option<ColorBlendState<'d>>,
    dynamic_state_flags: PipelineDynamicStates,
}

impl<'d, 's, Layout: PipelineLayout, RenderPass: 'd + crate::RenderPass + ?Sized>
    NonDerivedGraphicsPipelineBuilder<'d, 's, Layout, RenderPass>
{
    /// Initialize the builder object
    pub const fn new(layout: Layout, subpass: SubpassRef<'d, RenderPass>, vp: VertexProcessingStages<'d, 's>) -> Self {
        Self {
            flags: 0,
            _layout: layout,
            rp: subpass.0,
            subpass: subpass.1,
            vp,
            rasterizer_state: RasterizationState::new(),
            tess_state: None,
            viewport_state: None,
            ms_state: None,
            ds_state: None,
            color_blending: None,
            dynamic_state_flags: PipelineDynamicStates::new(),
        }
    }
}
/// Shading State and Input Configuration
impl<'d, 's, Layout: PipelineLayout, RenderPass: 'd + crate::RenderPass + ?Sized>
    NonDerivedGraphicsPipelineBuilder<'d, 's, Layout, RenderPass>
{
    /// Set the vertex processing stages in this pipeline
    pub fn vertex_processing(&mut self, vp: VertexProcessingStages<'d, 's>) -> &mut Self {
        self.vp = vp;
        self
    }

    /// Get a mutable reference to the vertex processing stage configuration in this pipeline
    pub const fn vertex_processing_mut(&mut self) -> &mut VertexProcessingStages<'d, 's> {
        &mut self.vp
    }

    /// Number of control points per patch
    pub fn patch_control_point_count(&mut self, count: u32) -> &mut Self {
        if self.tess_state.is_none() {
            self.tess_state = Some(Box::new(VkPipelineTessellationStateCreateInfo {
                sType: VkPipelineTessellationStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                patchControlPoints: 0,
            }));
        }
        self.tess_state.as_mut().unwrap().patchControlPoints = count;
        self
    }
}

#[transparent_marked]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewportState<'d>(
    VkPipelineViewportStateCreateInfo,
    core::marker::PhantomData<(Option<&'d dyn VulkanStructureAsRef>, &'d [VkViewport], &'d [VkRect2D])>,
);
impl<'d> ViewportState<'d> {
    pub const fn new(viewports: &'d [VkViewport], scissors: &'d [VkRect2D]) -> Self {
        assert!(
            viewports.len() == scissors.len(),
            "a number of viewports and scissor rects must be match"
        );

        Self(
            VkPipelineViewportStateCreateInfo {
                sType: VkPipelineViewportStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                viewportCount: viewports.len() as _,
                pViewports: slice_as_ptr_empty_null(viewports),
                scissorCount: scissors.len() as _,
                pScissors: slice_as_ptr_empty_null(scissors),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkPipelineViewportStateCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkPipelineViewportStateCreateInfo {
        self.0
    }

    pub fn with_next(mut self, next: &'d dyn VulkanStructureAsRef) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}

impl VkPipelineColorBlendAttachmentState {
    pub const NOBLEND: Self = Self {
        colorWriteMask: VK_COLOR_COMPONENT_A_BIT
            | VK_COLOR_COMPONENT_R_BIT
            | VK_COLOR_COMPONENT_G_BIT
            | VK_COLOR_COMPONENT_B_BIT,
        blendEnable: VK_FALSE,
        ..unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    };

    /// `src * 1 + dst * (1 - src.a)` for both color and alpha.
    ///
    /// https://stackoverflow.com/questions/18918643/how-to-achieve-d3d-output-with-premultiplied-alpha-for-use-with-d3dimage-in-wpf
    pub const PREMULTIPLIED: Self = Self {
        colorWriteMask: VK_COLOR_COMPONENT_A_BIT
            | VK_COLOR_COMPONENT_R_BIT
            | VK_COLOR_COMPONENT_G_BIT
            | VK_COLOR_COMPONENT_B_BIT,
        blendEnable: VK_TRUE,
        srcColorBlendFactor: VK_BLEND_FACTOR_ONE,
        dstColorBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
        colorBlendOp: VK_BLEND_OP_ADD,
        // srcAlphaBlendFactor: BlendFactor::OneMinusDestAlpha as _,
        // dstAlphaBlendFactor: BlendFactor::One as _,
        srcAlphaBlendFactor: VK_BLEND_FACTOR_ONE,
        dstAlphaBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
        alphaBlendOp: VK_BLEND_OP_ADD,
    };

    pub const fn enabled(self) -> Self {
        Self {
            blendEnable: VK_TRUE,
            ..self
        }
    }

    pub const fn disabled(self) -> Self {
        Self {
            blendEnable: VK_FALSE,
            ..self
        }
    }

    pub const fn color_blend_factor_src(self, f: VkBlendFactor) -> Self {
        Self {
            srcColorBlendFactor: f,
            ..self
        }
    }

    pub const fn color_blend_factor_dst(self, f: VkBlendFactor) -> Self {
        Self {
            dstColorBlendFactor: f,
            ..self
        }
    }

    pub const fn alpha_blend_factor_src(self, f: VkBlendFactor) -> Self {
        Self {
            srcAlphaBlendFactor: f,
            ..self
        }
    }

    pub const fn alpha_blend_factor_dst(self, f: VkBlendFactor) -> Self {
        Self {
            dstAlphaBlendFactor: f,
            ..self
        }
    }

    pub const fn color_blend_op(self, op: VkBlendOp) -> Self {
        Self {
            colorBlendOp: op,
            ..self
        }
    }

    pub const fn alpha_blend_op(self, op: VkBlendOp) -> Self {
        Self {
            alphaBlendOp: op,
            ..self
        }
    }

    pub const fn color_blend(self, src: VkBlendFactor, op: VkBlendOp, dst: VkBlendFactor) -> Self {
        Self {
            srcColorBlendFactor: src,
            dstColorBlendFactor: dst,
            colorBlendOp: op,
            ..self
        }
    }

    pub const fn alpha_blend(self, src: VkBlendFactor, op: VkBlendOp, dst: VkBlendFactor) -> Self {
        Self {
            srcAlphaBlendFactor: src,
            dstAlphaBlendFactor: dst,
            alphaBlendOp: op,
            ..self
        }
    }
}

/// Color Blending
#[transparent_marked]
#[derive(Clone, PartialEq)]
pub struct ColorBlendState<'d>(
    VkPipelineColorBlendStateCreateInfo,
    core::marker::PhantomData<(
        Option<&'d dyn VulkanStructureAsRef>,
        &'d [VkPipelineColorBlendAttachmentState],
    )>,
);
impl<'d> ColorBlendState<'d> {
    /// Creates with standard parameters
    pub const fn new(
        logic_op: Option<VkLogicOp>,
        attachments: &'d [VkPipelineColorBlendAttachmentState],
        blend_constants: [f32; 4],
    ) -> Self {
        Self(
            VkPipelineColorBlendStateCreateInfo {
                sType: VkPipelineColorBlendStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                logicOpEnable: logic_op.is_some() as _,
                logicOp: match logic_op {
                    Some(x) => x,
                    None => VK_LOGIC_OP_NO_OP,
                },
                attachmentCount: attachments.len() as _,
                pAttachments: slice_as_ptr_empty_null(attachments),
                blendConstants: blend_constants,
            },
            core::marker::PhantomData,
        )
    }

    /// Constructs from the raw data
    pub const unsafe fn from_raw(raw: VkPipelineColorBlendStateCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    /// Unpacks the raw data
    pub const fn into_raw(self) -> VkPipelineColorBlendStateCreateInfo {
        self.0
    }

    /// Chains an extra structure extending the `VkPipelineColorBlendStateCreateInfo`
    #[inline(always)]
    pub fn with_next(mut self, next: &'d dyn VulkanStructureAsRef) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}

impl<'d, 's, Layout: PipelineLayout, RenderPass: 'd + crate::RenderPass + ?Sized>
    NonDerivedGraphicsPipelineBuilder<'d, 's, Layout, RenderPass>
{
    #[inline(always)]
    pub const fn viewport_state(&mut self, state: ViewportState<'d>) -> &mut Self {
        self.viewport_state = Some(state);
        self
    }

    #[inline(always)]
    pub const fn clear_viewport_state(&mut self) -> &mut Self {
        self.viewport_state = None;
        self
    }

    /// Rasterization State
    #[inline(always)]
    pub const fn rasterization_state(&mut self, state: RasterizationState) -> &mut Self {
        self.rasterizer_state = state;
        self
    }

    /// Multisample State
    #[inline(always)]
    pub const fn multisample_state(&mut self, state: Option<MultisampleState<'d>>) -> &mut Self {
        self.ms_state = state;
        self
    }

    /// Sets depth/stencil state
    #[inline(always)]
    pub const fn depth_stencil_state(&mut self, state: DepthStencilState) -> &mut Self {
        self.ds_state = Some(state);
        self
    }

    /// Clear depth/stencil state
    #[inline(always)]
    pub fn clear_depth_stencil_state(&mut self) -> &mut Self {
        self.ds_state = None;
        self
    }

    #[inline(always)]
    pub const fn color_blend_state(&mut self, state: ColorBlendState<'d>) -> &mut Self {
        self.color_blending = Some(state);
        self
    }

    #[inline(always)]
    pub const fn clear_color_blend_state(&mut self) -> &mut Self {
        self.color_blending = None;
        self
    }
}

/// Dynamic States
impl<'d, 's, Layout: PipelineLayout, RenderPass: 'd + crate::RenderPass + ?Sized>
    NonDerivedGraphicsPipelineBuilder<'d, 's, Layout, RenderPass>
{
    #[inline(always)]
    pub fn dynamic(&mut self, state: VkDynamicState) -> &mut Self {
        self.dynamic_state_flags.enable(state);
        self
    }

    #[inline(always)]
    pub fn r#static(&mut self, state: VkDynamicState) -> &mut Self {
        self.dynamic_state_flags.disable(state);
        self
    }

    #[inline(always)]
    pub fn all_state_static(&mut self) -> &mut Self {
        self.dynamic_state_flags.clear();
        self
    }

    #[inline(always)]
    pub fn set_dynamic_states(&mut self, dynamic_states: Vec<VkDynamicState>) -> &mut Self {
        self.dynamic_state_flags = dynamic_states.into();
        self
    }
}

/// Misc Configurations
impl<'d, 's, Layout: PipelineLayout, RenderPass: 'd + crate::RenderPass + ?Sized>
    NonDerivedGraphicsPipelineBuilder<'d, 's, Layout, RenderPass>
{
    /// The base pipeline handle to derive from
    pub const fn derive<BP: Pipeline>(self, b: BP) -> DerivedGraphicsPipelineBuilder<BP, Self> {
        DerivedGraphicsPipelineBuilder(b, self)
    }

    //// The base pipeline index to derive from
    pub const fn derive_index(self, index: i32) -> IndexDerivedGraphicsPipelineBuilder<Self> {
        IndexDerivedGraphicsPipelineBuilder(index, self)
    }

    /// The description of binding locations used by both the pipeline and descriptor sets used with the pipeline
    pub fn layout(&mut self, l: Layout) -> &mut Self {
        self._layout = l;
        self
    }

    /// A handle to a render pass object and the index of the subpass where this pipeline will be used
    pub const fn render_pass(&mut self, rpo: &'d RenderPass, subpass: u32) -> &mut Self {
        self.rp = rpo;
        self.subpass = subpass;
        self
    }

    /// The created pipeline will be optimized.
    /// Disabling optimization of the pipeline may reduce the time taken to create the pipeline
    pub const fn enable_optimization(&mut self) -> &mut Self {
        self.flags &= !VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT;
        self
    }

    /// The created pipeline will not be optimized.
    /// Disabling optimization of the pipeline may reduce the time taken to create the pipeline
    pub const fn disable_optimization(&mut self) -> &mut Self {
        self.flags |= VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT;
        self
    }

    /// The pipeline to be created is allowed to be the parent of a pipeline that will be created in a subsequent creation operation
    pub const fn allow_derivatives(&mut self) -> &mut Self {
        self.flags |= VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT;
        self
    }

    /// The pipeline to be created is denied to be the parent of a pipeline that will be created in a subsequent creation operation
    pub const fn deny_derivatives(&mut self) -> &mut Self {
        self.flags &= !VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT;
        self
    }
}

/// Unsafe Utilities
impl<'d, 's, Layout: PipelineLayout, RenderPass: 'd + crate::RenderPass + ?Sized>
    NonDerivedGraphicsPipelineBuilder<'d, 's, Layout, RenderPass>
{
    /// Set the `VkPipelineTessellationStateCreateInfo` structure directly
    /// # Safety
    /// Application must guarantee these constraints:
    ///
    /// - The lifetime of the content in the structure is valid for this builder
    /// - The content in the structure is valid
    pub unsafe fn tessellation_state_create_info(
        &mut self,
        state: Option<Box<VkPipelineTessellationStateCreateInfo>>,
    ) -> &mut Self {
        self.tess_state = state;
        self
    }
}
pub struct NonDerivedGraphicPipelineBuilderExtraStorage {
    pub dynamic_state: Option<VkPipelineDynamicStateCreateInfo>,
}
impl<'d, 's, Layout: PipelineLayout, RenderPass: 'd + crate::RenderPass + ?Sized> GraphicsPipelineBuilder
    for NonDerivedGraphicsPipelineBuilder<'d, 's, Layout, RenderPass>
{
    type ExtraStorage = NonDerivedGraphicPipelineBuilderExtraStorage;

    fn build(&mut self, extras: &Self::ExtraStorage) -> VkGraphicsPipelineCreateInfo {
        self.rasterizer_state
            .apply_dynamic_states(&mut self.dynamic_state_flags);
        let rst = self.rasterizer_state.make_chained();
        let ms = if let Some(ref msr) = self.ms_state {
            Some(&msr.data)
        } else {
            assert!(
                rst.rasterizerDiscardEnable == VK_TRUE,
                "MultisampleState must be specified when rasterizerDiscardEnable is false"
            );
            None
        };

        VkGraphicsPipelineCreateInfo {
            sType: VkGraphicsPipelineCreateInfo::TYPE,
            pNext: std::ptr::null(),
            stageCount: self.vp.shader_stages.len() as _,
            pStages: slice_as_ptr_empty_null(self.vp.shader_stages) as _,
            pVertexInputState: &self.vp.vi,
            pInputAssemblyState: &self.vp.ia,
            pTessellationState: opt_pointer(self.tess_state.as_ref().map(|x| &**x)),
            pViewportState: opt_pointer(self.viewport_state.as_ref()) as _,
            pRasterizationState: rst,
            pMultisampleState: ms.map_or_else(std::ptr::null, |x| x as *const _),
            pDepthStencilState: opt_pointer(self.ds_state.as_ref().map(|x| &x.0)),
            pColorBlendState: opt_pointer(self.color_blending.as_ref()) as _,
            pDynamicState: opt_pointer(extras.dynamic_state.as_ref()) as _,
            layout: self._layout.native_ptr(),
            renderPass: self.rp.native_ptr(),
            subpass: self.subpass,
            basePipelineHandle: VkPipeline::NULL,
            basePipelineIndex: -1,
            flags: 0,
        }
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        let dynamic_state = if !self.dynamic_state_flags.0.is_empty() {
            unsafe { Some(Into::<LifetimeBound<_>>::into(&self.dynamic_state_flags).unbound()) }
        } else {
            None
        };

        NonDerivedGraphicPipelineBuilderExtraStorage { dynamic_state }
    }
}

pub struct DerivedGraphicsPipelineBuilder<Base: Pipeline, Diff: GraphicsPipelineBuilder>(Base, Diff);
impl<Base: Pipeline, Diff: GraphicsPipelineBuilder> DerivedGraphicsPipelineBuilder<Base, Diff> {
    pub const fn diff_mut(&mut self) -> &mut Diff {
        &mut self.1
    }
}
impl<Base: Pipeline, Diff: GraphicsPipelineBuilder> GraphicsPipelineBuilder
    for DerivedGraphicsPipelineBuilder<Base, Diff>
{
    type ExtraStorage = Diff::ExtraStorage;

    fn build(&mut self, extras: &Self::ExtraStorage) -> VkGraphicsPipelineCreateInfo {
        let base = self.1.build(extras);

        VkGraphicsPipelineCreateInfo {
            basePipelineIndex: -1,
            basePipelineHandle: self.0.native_ptr(),
            flags: base.flags | VK_PIPELINE_CREATE_DERIVATIVE_BIT,
            ..base
        }
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        self.1.make_extras()
    }
}

pub struct IndexDerivedGraphicsPipelineBuilder<Diff: GraphicsPipelineBuilder>(i32, Diff);
impl<Diff: GraphicsPipelineBuilder> IndexDerivedGraphicsPipelineBuilder<Diff> {
    pub const fn diff_mut(&mut self) -> &mut Diff {
        &mut self.1
    }
}
impl<Diff: GraphicsPipelineBuilder> GraphicsPipelineBuilder for IndexDerivedGraphicsPipelineBuilder<Diff> {
    type ExtraStorage = Diff::ExtraStorage;

    fn build(&mut self, extras: &Self::ExtraStorage) -> VkGraphicsPipelineCreateInfo {
        let base = self.1.build(extras);

        VkGraphicsPipelineCreateInfo {
            basePipelineIndex: self.0,
            basePipelineHandle: VkPipeline::NULL,
            flags: base.flags | VK_PIPELINE_CREATE_DERIVATIVE_BIT,
            ..base
        }
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        self.1.make_extras()
    }
}

#[derive(Clone)]
pub struct ComputePipelineBuilder<Layout: PipelineLayout, Shader: PipelineShaderProvider> {
    pub(crate) shader: Shader,
    pub(crate) layout: Layout,
}
impl<'d, Layout: PipelineLayout, Shader: PipelineShaderProvider> ComputePipelineBuilder<Layout, Shader> {
    pub const fn new(layout: Layout, shader: Shader) -> Self {
        ComputePipelineBuilder { shader, layout }
    }
}
#[implements]
impl<'d, Layout: PipelineLayout, Shader: PipelineShaderProvider> ComputePipelineBuilder<Layout, Shader> {
    /// Create a compute pipeline
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn create<Device: crate::Device>(
        &self,
        device: Device,
        cache: Option<&impl PipelineCache>,
    ) -> crate::Result<PipelineObject<Device>> {
        let extras = self.shader.make_extras();
        let stage = self.shader.base_struct(ShaderStage::Compute, &extras);

        let cinfo = VkComputePipelineCreateInfo {
            sType: VkComputePipelineCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            basePipelineHandle: VkPipeline::NULL,
            basePipelineIndex: -1,
            stage: stage.0,
            layout: self.layout.native_ptr(),
        };

        let mut pipeline = ::std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_compute_pipelines(
                device.native_ptr(),
                cache.map(VkHandle::native_ptr).unwrap_or(VkPipelineCache::NULL),
                1,
                &cinfo,
                std::ptr::null(),
                pipeline.as_mut_ptr(),
            )
            .into_result()
            .map(move |_| PipelineObject(pipeline.assume_init(), device))
        }
    }
}

/// Bitmask specifying pipeline stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[bitflags_newtype]
pub struct PipelineStageFlags(pub VkPipelineStageFlags);
impl PipelineStageFlags {
    /// The stage of the pipeline where any commands are initially received by the queue
    pub const TOP_OF_PIPE: Self = Self(VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT);
    /// The stage of the pipeline where Draw/DispatchIndirect data structures are consumed
    pub const DRAW_INDIRECT: Self = Self(VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT);
    /// The stage of the pipeline where vertex and index buffers are consumed
    pub const VERTEX_INPUT: Self = Self(VK_PIPELINE_STAGE_VERTEX_INPUT_BIT);
    /// The vertex shader stage
    pub const VERTEX_SHADER: Self = Self(VK_PIPELINE_STAGE_VERTEX_SHADER_BIT);
    /// The tessellation control shader stage
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT);
    /// The tessellation evaluation shader stage
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT);
    /// The geometry shader stage
    pub const GEOMETRY_SHADER: Self = Self(VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT);
    /// The fragment shader stage
    pub const FRAGMENT_SHADER: Self = Self(VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
    /// The stage of the pipeline where early fragment tests (depth and stencil tests before fragment shading) are performed
    pub const EARLY_FRAGMENT_TESTS: Self = Self(VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT);
    /// The stage of the pipeline where late fragment tests (depth and stencil tests after fragment shading) are performed
    pub const LATE_FRAGMENT_TESTS: Self = Self(VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT);
    /// The stage of the pipeline after blending where the final color values are output from the pipeline
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT);
    /// The execution of copy commands
    pub const TRANSFER: Self = Self(VK_PIPELINE_STAGE_TRANSFER_BIT);
    /// The execution of a compute shader
    pub const COMPUTE_SHADER: Self = Self(VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT);
    /// The final stage in the pipeline where operations generated by all commands complete execution
    pub const BOTTOM_OF_PIPE: Self = Self(VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT);
    /// A pseudo-stage indicating execution on the host of reads/writes of device memory
    pub const HOST: Self = Self(VK_PIPELINE_STAGE_HOST_BIT);
    /// The execution of all graphics pipeline stages
    pub const ALL_GRAPHICS: Self = Self(VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT);
    /// Equivalent to the logical OR of every other pipeline stage flag that is supported on the quue it is used with
    pub const ALL_COMMANDS: Self = Self(VK_PIPELINE_STAGE_ALL_COMMANDS_BIT);
}

/// Bitmask specifying pipeline stages (extended)
#[cfg(feature = "VK_KHR_synchronization2")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[bitflags_newtype]
pub struct PipelineStageFlags2(pub VkPipelineStageFlags2KHR);
#[cfg(feature = "VK_KHR_synchronization2")]
impl PipelineStageFlags2 {
    pub const TOP_OF_PIPE: Self = Self(VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR);
    pub const DRAW_INDIRECT: Self = Self(VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR);
    pub const VERTEX_INPUT: Self = Self(VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR);
    pub const VERTEX_SHADER: Self = Self(VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR);
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR);
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR);
    pub const GEOMETRY_SHADER: Self = Self(VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR);
    pub const FRAGMENT_SHADER: Self = Self(VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR);
    pub const LATE_FRAGMENT_TESTS: Self = Self(VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR);
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR);
    pub const COMPUTE_SHADER: Self = Self(VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR);
    pub const ALL_TRANSFER: Self = Self(VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR);
    pub const BOTTOM_OF_PIPE: Self = Self(VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR);
    pub const HOST: Self = Self(VK_PIPELINE_STAGE_2_HOST_BIT_KHR);
    pub const ALL_GRAPHICS: Self = Self(VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR);
    pub const ALL_COMMANDS: Self = Self(VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR);
    pub const COPY: Self = Self(VK_PIPELINE_STAGE_2_COPY_BIT_KHR);
    pub const RESOLVE: Self = Self(VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR);
    pub const BLIT: Self = Self(VK_PIPELINE_STAGE_2_BLIT_BIT_KHR);
    pub const CLEAR: Self = Self(VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR);
    pub const INDEX_INPUT: Self = Self(VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR);
    pub const VERTEX_ATTRIBUTE_INPUT: Self = Self(VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR);
    pub const PRE_RASTERIZATION_SHADERS: Self = Self(VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR);
}
