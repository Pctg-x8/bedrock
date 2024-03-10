//! Vulkan Shading(Shader/Pipeline)

use crate::ffi_helper::ArrayFFIExtensions;
#[cfg(feature = "Implements")]
use crate::VkHandleMut;
use crate::{vk::*, DeviceChild, VkHandle, VulkanStructure};
use crate::{LifetimeBound, VkRawHandle};
use std::borrow::Cow;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ops::*;

/// Bitmask specifying a pipeline stage
#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ShaderStage(pub VkShaderStageFlags);
impl ShaderStage {
    /// Empty bits
    pub const EMPTY: Self = ShaderStage(0);
    /// The vertex stage
    pub const VERTEX: Self = ShaderStage(VK_SHADER_STAGE_VERTEX_BIT);
    /// The tessellation control stage
    pub const TESSELLATION_CONTROL: Self = ShaderStage(VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT);
    /// The tessellation evaluation stage
    pub const TESSELLATION_EVALUATION: Self = ShaderStage(VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT);
    /// The geometry stage
    pub const GEOMETRY: Self = ShaderStage(VK_SHADER_STAGE_GEOMETRY_BIT);
    /// The fragment stage
    pub const FRAGMENT: Self = ShaderStage(VK_SHADER_STAGE_FRAGMENT_BIT);
    /// The compute stage
    pub const COMPUTE: Self = ShaderStage(VK_SHADER_STAGE_COMPUTE_BIT);
    /// A combination of bits used as shorthand to specify all graphics stages defined above (excluding the compute stage)
    pub const ALL_GRAPHICS: Self = ShaderStage(VK_SHADER_STAGE_ALL_GRAPHICS);
    /// A combination of bits used as shorthand to specify all shader stages supported by the device,
    /// including all additional stages which are introduced by extensions
    pub const ALL: Self = ShaderStage(VK_SHADER_STAGE_ALL);
    /// A combination of tessellation control stage and tessellation evaluation stage
    pub const TESSELLATION: Self = ShaderStage(Self::TESSELLATION_CONTROL.0 | Self::TESSELLATION_EVALUATION.0);

    /// The vertex stage
    pub fn vertex(self) -> Self {
        ShaderStage(self.0 | Self::VERTEX.0)
    }
    /// The tessellation control stage
    pub fn tessellation_control(self) -> Self {
        ShaderStage(self.0 | Self::TESSELLATION_CONTROL.0)
    }
    /// The tessellation evaluation stage
    pub fn tessellation_evaluation(self) -> Self {
        ShaderStage(self.0 | Self::TESSELLATION_EVALUATION.0)
    }
    /// The geometry stage
    pub fn geometry(self) -> Self {
        ShaderStage(self.0 | Self::GEOMETRY.0)
    }
    /// The fragment stage
    pub fn fragment(self) -> Self {
        ShaderStage(self.0 | Self::FRAGMENT.0)
    }
    /// The compute stage
    pub fn compute(self) -> Self {
        ShaderStage(self.0 | Self::COMPUTE.0)
    }
    /// A combination of bits used as shorthand to specify all graphics stages defined above (excluding the compute stage)
    pub fn all_graphics(self) -> Self {
        ShaderStage(self.0 | Self::ALL_GRAPHICS.0)
    }
    /// A combination of tessellation control stage and tessellation evaluation stage
    pub fn tessellation(self) -> Self {
        ShaderStage(self.0 | Self::TESSELLATION.0)
    }
}
impl BitOr for ShaderStage {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        ShaderStage(self.0 | other.0)
    }
}
impl BitOrAssign for ShaderStage {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

/// Stencil comparison function
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

DefineStdDeviceChildObject! {
    /// Opaque handle to a shader module object
    ShaderModuleObject(VkShaderModule): ShaderModule
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a pipeline cache object
    PipelineCacheObject(VkPipelineCache): PipelineCache
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a pipeline layout object
    PipelineLayoutObject(VkPipelineLayout): PipelineLayout
}

DefineStdDeviceChildObject! {
    /// Opaque handle to a pipeline object
    PipelineObject(VkPipeline): Pipeline
}

pub trait ShaderModule: VkHandle<Handle = VkShaderModule> {}
DerefContainerBracketImpl!(for ShaderModule {});
GuardsImpl!(for ShaderModule {});

pub trait PipelineCache: VkHandle<Handle = VkPipelineCache> + DeviceChild {
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
            crate::vkresolve::get_pipeline_cache_data(
                self.device().native_ptr(),
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
            crate::vkresolve::get_pipeline_cache_data(
                self.device().native_ptr(),
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
        let mut b = Vec::with_capacity(len);
        unsafe {
            b.set_len(len);
        }
        self.data_into(&mut b[..])?;

        Ok(b)
    }

    /// Combine the data stores of pipeline caches into `self`
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn merge_into(&mut self, src: &[impl PipelineCache]) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        let srcs = src.iter().map(VkHandle::native_ptr).collect::<Vec<_>>();
        unsafe {
            crate::vkresolve::merge_pipeline_caches(
                self.device().native_ptr(),
                self.native_ptr_mut(),
                srcs.len() as _,
                srcs.as_ptr_empty_null(),
            )
            .into_result()
            .map(drop)
        }
    }
}
DerefContainerBracketImpl!(for PipelineCache {});
GuardsImpl!(for PipelineCache {});

/// Builder struct for PipelineLayout object
pub struct PipelineLayoutBuilder<'l, D: crate::Device> {
    descriptor_set_layouts: Vec<&'l dyn crate::DescriptorSetLayout<ConcreteDevice = D>>,
    push_constant_ranges: Vec<(ShaderStage, std::ops::Range<u32>)>,
}
impl<'l, D: crate::Device> PipelineLayoutBuilder<'l, D> {
    /// Creates a new builder struct and initialize it with given parameters
    pub const fn new(
        descriptor_set_layouts: Vec<&'l dyn crate::DescriptorSetLayout<ConcreteDevice = D>>,
        push_constant_ranges: Vec<(ShaderStage, std::ops::Range<u32>)>,
    ) -> Self {
        Self {
            descriptor_set_layouts,
            push_constant_ranges,
        }
    }

    /// Creates a new empty builder struct
    pub const fn empty() -> Self {
        Self::new(Vec::new(), Vec::new())
    }

    /// Appends new descriptor set layout
    pub fn append_descriptor_set_layout(
        mut self,
        layout: &'l impl crate::DescriptorSetLayout<ConcreteDevice = D>,
    ) -> Self {
        self.descriptor_set_layouts.push(layout as _);
        self
    }

    /// Appends new push constant range data
    pub fn append_push_constant_range(mut self, shader_stage: ShaderStage, range: std::ops::Range<u32>) -> Self {
        self.push_constant_ranges.push((shader_stage, range));
        self
    }

    /// Creates a new pipeline layout object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    pub fn create(&self, device: D) -> crate::Result<PipelineLayoutObject<D>> {
        let layout_handles = self
            .descriptor_set_layouts
            .iter()
            .map(VkHandle::native_ptr)
            .collect::<Vec<_>>();
        let push_constant_ranges = self
            .push_constant_ranges
            .iter()
            .map(|(s, r)| VkPushConstantRange {
                stageFlags: s.0,
                offset: r.start,
                size: r.end - r.start,
            })
            .collect::<Vec<_>>();

        let create_info = VkPipelineLayoutCreateInfo {
            sType: VkPipelineLayoutCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            setLayoutCount: layout_handles.len() as _,
            pSetLayouts: layout_handles.as_ptr_empty_null(),
            pushConstantRangeCount: push_constant_ranges.len() as _,
            pPushConstantRanges: push_constant_ranges.as_ptr_empty_null(),
        };

        let mut handle = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_pipeline_layout(
                device.native_ptr(),
                &create_info,
                core::ptr::null(),
                handle.as_mut_ptr(),
            )
            .into_result()
            .map(move |_| PipelineLayoutObject(handle.assume_init(), device))
        }
    }
}

pub trait PipelineLayout: VkHandle<Handle = VkPipelineLayout> {}
DerefContainerBracketImpl!(for PipelineLayout {});
GuardsImpl!(for PipelineLayout {});

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
    fn is_dynamic(&self) -> bool {
        match self {
            Self::Dynamic => true,
            _ => false,
        }
    }
    fn is_enabled(&self) -> bool {
        match self {
            Self::Disabled => false,
            _ => true,
        }
    }
}
use derives::implements;
use libc::c_void;
pub use SwitchOrDynamicState::*;
/// Untyped data cell
#[allow(dead_code)]
#[derive(Clone)]
pub struct DynamicDataCell<'d> {
    size: usize,
    data: *const (),
    ph: PhantomData<&'d ()>,
}
impl<'d, T> From<&'d T> for DynamicDataCell<'d> {
    fn from(d: &'d T) -> Self {
        DynamicDataCell {
            size: std::mem::size_of::<T>(),
            data: d as *const T as *const _,
            ph: PhantomData,
        }
    }
}
impl<'d> DynamicDataCell<'d> {
    /// Construct borrowing a slice
    pub fn from_slice<T>(s: &'d [T]) -> Self {
        DynamicDataCell {
            size: std::mem::size_of::<T>() * s.len(),
            data: s.as_ptr_empty_null() as *const _,
            ph: PhantomData,
        }
    }
}
/// Builder struct to construct a shader stage in a `Pipeline`
#[derive(Clone)]
pub struct OldPipelineShader<'d, Module: ShaderModule> {
    pub module: Module,
    pub entry_name: CString,
    pub specinfo: Option<(Cow<'d, [VkSpecializationMapEntry]>, DynamicDataCell<'d>)>,
}
/// Whether the state(type of array) is dynamic or static
pub enum DynamicArrayState<'d, T> {
    Dynamic(usize),
    Static(&'d [T]),
}
impl<'d, T> DynamicArrayState<'d, T> {
    fn count(&self) -> usize {
        match self {
            &DynamicArrayState::Dynamic(s) => s,
            DynamicArrayState::Static(ref v) => v.len(),
        }
    }
    fn as_ptr(&self) -> *const T {
        match self {
            DynamicArrayState::Static(v) => v.as_ptr_empty_null(),
            _ => std::ptr::null(),
        }
    }
    fn is_dynamic(&self) -> bool {
        match self {
            DynamicArrayState::Dynamic(_) => true,
            _ => false,
        }
    }
}
/// Which is pipeline state to derive from
#[derive(Clone, Copy)]
pub enum BasePipeline<Pipeline: crate::Pipeline> {
    /// Does not derive
    None,
    /// Derive from a handle to the pipeline state object
    Handle(Pipeline),
    /// Derive from a create info in the `pCreateInfos` parameter
    Index(u32),
}

/// VkPipelineDynamicStateCreateInfo builder
#[derive(Clone)]
pub struct PipelineDynamicStates(Vec<VkDynamicState>);
impl From<Vec<VkDynamicState>> for PipelineDynamicStates {
    fn from(mut v: Vec<VkDynamicState>) -> Self {
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
    pub fn new() -> Self {
        PipelineDynamicStates(Vec::new())
    }

    /// Enables using a dynamic state
    pub fn enable(&mut self, v: VkDynamicState) {
        if let Err(n) = self.0.binary_search(&v) {
            self.0.insert(n, v);
        }
    }
    /// Disables using a dynamic state
    pub fn disable(&mut self, v: VkDynamicState) {
        if let Ok(n) = self.0.binary_search(&v) {
            self.0.remove(n);
        }
    }
    /// Sets enable or disable state of a dynamic state
    pub fn set(&mut self, v: VkDynamicState, enable: bool) {
        if enable {
            self.enable(v);
        } else {
            self.disable(v);
        }
    }
}
impl<'d, Layout, RenderPass, ShaderStages> NonDerivedGraphicsPipelineBuilder<'d, Layout, RenderPass, ShaderStages>
where
    Layout: PipelineLayout,
    RenderPass: crate::RenderPass,
    ShaderStages: PipelineShaderStageProvider,
{
    /// Gets a mutable reference to the dynamic state settings
    pub fn dynamic_states_mut(&mut self) -> &mut PipelineDynamicStates {
        &mut self.dynamic_state_flags
    }
}

/// Helper structure for VkVertexInputBindingDescription
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VertexInputBindingDescription(VkVertexInputBindingDescription);
impl From<VkVertexInputBindingDescription> for VertexInputBindingDescription {
    fn from(v: VkVertexInputBindingDescription) -> Self {
        VertexInputBindingDescription(v)
    }
}
impl From<VertexInputBindingDescription> for VkVertexInputBindingDescription {
    fn from(v: VertexInputBindingDescription) -> Self {
        v.0
    }
}
impl VertexInputBindingDescription {
    /// Consumed per vertex with stride
    pub fn per_vertex(binding: u32, stride: u32) -> Self {
        VertexInputBindingDescription(VkVertexInputBindingDescription {
            binding,
            stride,
            inputRate: VK_VERTEX_INPUT_RATE_VERTEX,
        })
    }
    /// Consumed per instance with stride
    pub fn per_instance(binding: u32, stride: u32) -> Self {
        VertexInputBindingDescription(VkVertexInputBindingDescription {
            binding,
            stride,
            inputRate: VK_VERTEX_INPUT_RATE_INSTANCE,
        })
    }

    /// Consumed per vertex the structured data
    pub fn per_vertex_typed<T>(binding: u32) -> Self {
        Self::per_vertex(binding, std::mem::size_of::<T>() as _)
    }
    /// Consumed per instance the structured data
    pub fn per_instance_typed<T>(binding: u32) -> Self {
        Self::per_instance(binding, std::mem::size_of::<T>() as _)
    }
}

pub trait SpecializationConstants {
    const ENTRIES: &'static [VkSpecializationMapEntry];

    fn as_ptr(&self) -> *const c_void;
}
DerefContainerBracketImpl!(for SpecializationConstants {
    const ENTRIES: &'static [VkSpecializationMapEntry] = T::ENTRIES;

    fn as_ptr(&self) -> *const c_void {
        T::as_ptr(&**self)
    }
});

pub trait PipelineShaderProvider {
    type ExtraStorage;

    fn base_struct(&self, stage: ShaderStage, extras: &Self::ExtraStorage) -> VkPipelineShaderStageCreateInfo;
    fn make_extras(&self) -> Self::ExtraStorage;
}
DerefContainerBracketImpl!(for PipelineShaderProvider {
    type ExtraStorage = T::ExtraStorage;

    fn base_struct(&self, stage: ShaderStage, extras: &Self::ExtraStorage) -> VkPipelineShaderStageCreateInfo {
        T::base_struct(&**self, stage, extras)
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        T::make_extras(&**self)
    }
});

pub struct PipelineShader2<M: ShaderModule>(M, std::ffi::CString);
impl<M: ShaderModule> PipelineShader2<M> {
    pub const fn new(module: M, entry_point: std::ffi::CString) -> Self {
        Self(module, entry_point)
    }

    pub const fn specialize<T: SpecializationConstants>(self, value: T) -> SpecializedPipelineShader<Self, T> {
        SpecializedPipelineShader(self, value)
    }
}
impl<M: ShaderModule> PipelineShaderProvider for PipelineShader2<M> {
    type ExtraStorage = ();

    fn base_struct(&self, stage: ShaderStage, _extras: &Self::ExtraStorage) -> VkPipelineShaderStageCreateInfo {
        VkPipelineShaderStageCreateInfo {
            sType: VkPipelineShaderStageCreateInfo::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            stage: stage.0,
            module: self.0.native_ptr(),
            pName: self.1.as_ptr(),
            pSpecializationInfo: core::ptr::null(),
        }
    }
    fn make_extras(&self) -> Self::ExtraStorage {}
}

pub struct SpecializedPipelineShader<P: PipelineShaderProvider, T: SpecializationConstants>(P, T);
impl<P: PipelineShaderProvider, T: SpecializationConstants> PipelineShaderProvider for SpecializedPipelineShader<P, T> {
    type ExtraStorage = (P::ExtraStorage, VkSpecializationInfo);

    fn base_struct(&self, stage: ShaderStage, extras: &Self::ExtraStorage) -> VkPipelineShaderStageCreateInfo {
        VkPipelineShaderStageCreateInfo {
            pSpecializationInfo: &extras.1 as _,
            ..self.0.base_struct(stage, &extras.0)
        }
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        (
            self.0.make_extras(),
            VkSpecializationInfo {
                mapEntryCount: T::ENTRIES.len() as _,
                pMapEntries: T::ENTRIES.as_ptr_empty_null(),
                dataSize: core::mem::size_of::<T>(),
                pData: self.1.as_ptr(),
            },
        )
    }
}

pub trait PipelineShaderStageProvider {
    type ExtraStorage;

    fn base_struct(&self, extra_storage: &Self::ExtraStorage) -> Vec<VkPipelineShaderStageCreateInfo>;
    fn make_extras(&self) -> Self::ExtraStorage;

    #[inline]
    fn with_fragment_shader_stage<P: PipelineShaderProvider>(self, shader: P) -> WithFragmentShaderStage<Self, P>
    where
        Self: Sized,
    {
        WithFragmentShaderStage(self, shader)
    }

    #[inline]
    fn with_geometry_shader_stage<P: PipelineShaderProvider>(self, shader: P) -> WithGeometryShaderStage<Self, P>
    where
        Self: Sized,
    {
        WithGeometryShaderStage(self, shader)
    }

    #[inline]
    fn with_tessellation_shader_stage<C: PipelineShaderProvider, E: PipelineShaderProvider>(
        self,
        control_shader: C,
        eval_shader: E,
    ) -> WithTessellationShaderStage<Self, C, E>
    where
        Self: Sized,
    {
        WithTessellationShaderStage(self, control_shader, eval_shader)
    }
}
DerefContainerBracketImpl!(for PipelineShaderStageProvider {
    type ExtraStorage = T::ExtraStorage;

    fn base_struct(&self, extras: &Self::ExtraStorage) -> Vec<VkPipelineShaderStageCreateInfo> {
        T::base_struct(&**self, extras)
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        T::make_extras(&**self)
    }
});

#[repr(transparent)]
pub struct VertexShaderStage<P: PipelineShaderProvider>(P);
impl<P: PipelineShaderProvider> VertexShaderStage<P> {
    pub const fn new(shader: P) -> Self {
        Self(shader)
    }
}
impl<P: PipelineShaderProvider> PipelineShaderStageProvider for VertexShaderStage<P> {
    type ExtraStorage = P::ExtraStorage;

    fn base_struct(&self, extra_storage: &Self::ExtraStorage) -> Vec<VkPipelineShaderStageCreateInfo> {
        vec![self.0.base_struct(ShaderStage::VERTEX, extra_storage)]
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        self.0.make_extras()
    }
}

pub struct WithFragmentShaderStage<S: PipelineShaderStageProvider, P: PipelineShaderProvider>(S, P);
impl<S: PipelineShaderStageProvider, P: PipelineShaderProvider> PipelineShaderStageProvider
    for WithFragmentShaderStage<S, P>
{
    type ExtraStorage = (S::ExtraStorage, P::ExtraStorage);

    fn base_struct(&self, extra_storage: &Self::ExtraStorage) -> Vec<VkPipelineShaderStageCreateInfo> {
        let mut vs = self.0.base_struct(&extra_storage.0);
        vs.push(self.1.base_struct(ShaderStage::FRAGMENT, &extra_storage.1));
        vs
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        (self.0.make_extras(), self.1.make_extras())
    }
}

pub struct WithGeometryShaderStage<S: PipelineShaderStageProvider, P: PipelineShaderProvider>(S, P);
impl<S: PipelineShaderStageProvider, P: PipelineShaderProvider> PipelineShaderStageProvider
    for WithGeometryShaderStage<S, P>
{
    type ExtraStorage = (S::ExtraStorage, P::ExtraStorage);

    fn base_struct(&self, extra_storage: &Self::ExtraStorage) -> Vec<VkPipelineShaderStageCreateInfo> {
        let mut vs = self.0.base_struct(&extra_storage.0);
        vs.push(self.1.base_struct(ShaderStage::FRAGMENT, &extra_storage.1));
        vs
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        (self.0.make_extras(), self.1.make_extras())
    }
}

pub struct WithTessellationShaderStage<
    S: PipelineShaderStageProvider,
    C: PipelineShaderProvider,
    E: PipelineShaderProvider,
>(S, C, E);
impl<S: PipelineShaderStageProvider, C: PipelineShaderProvider, E: PipelineShaderProvider> PipelineShaderStageProvider
    for WithTessellationShaderStage<S, C, E>
{
    type ExtraStorage = (S::ExtraStorage, C::ExtraStorage, E::ExtraStorage);

    fn base_struct(&self, extra_storage: &Self::ExtraStorage) -> Vec<VkPipelineShaderStageCreateInfo> {
        let mut vs = self.0.base_struct(&extra_storage.0);
        vs.push(self.1.base_struct(ShaderStage::FRAGMENT, &extra_storage.1));
        vs.push(self.2.base_struct(ShaderStage::FRAGMENT, &extra_storage.2));
        vs
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        (self.0.make_extras(), self.1.make_extras(), self.2.make_extras())
    }
}

/// PipelineStateDesc: Shader Stages and Input descriptions
#[derive(Clone)]
pub struct VertexProcessingStages<'d, ShaderStages: PipelineShaderStageProvider> {
    shader_stages: ShaderStages,
    vi: VkPipelineVertexInputStateCreateInfo,
    ia: VkPipelineInputAssemblyStateCreateInfo,
    _holder: PhantomData<(
        &'d [VertexInputBindingDescription],
        &'d [VkVertexInputAttributeDescription],
    )>,
}
impl<'d, ShaderStages: PipelineShaderStageProvider> VertexProcessingStages<'d, ShaderStages> {
    pub fn new(
        shader_stages: ShaderStages,
        vbind: &'d [VertexInputBindingDescription],
        vattr: &'d [VkVertexInputAttributeDescription],
        primitive_topo: VkPrimitiveTopology,
    ) -> Self {
        VertexProcessingStages {
            shader_stages,
            vi: VkPipelineVertexInputStateCreateInfo {
                sType: VkPipelineVertexInputStateCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                vertexBindingDescriptionCount: vbind.len() as _,
                pVertexBindingDescriptions: vbind.as_ptr_empty_null() as _,
                vertexAttributeDescriptionCount: vattr.len() as _,
                pVertexAttributeDescriptions: vattr.as_ptr_empty_null(),
            },
            ia: VkPipelineInputAssemblyStateCreateInfo {
                sType: VkPipelineInputAssemblyStateCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                topology: primitive_topo,
                primitiveRestartEnable: VK_FALSE,
            },
            _holder: PhantomData,
        }
    }

    /// Update the vertex binding description
    pub fn vertex_binding(&mut self, vbind: &'d [VkVertexInputBindingDescription]) -> &mut Self {
        self.vi.vertexBindingDescriptionCount = vbind.len() as _;
        self.vi.pVertexBindingDescriptions = vbind.as_ptr_empty_null();
        self
    }
    /// Update the vertex attribute description
    pub fn vertex_attributes(&mut self, vattr: &'d [VkVertexInputAttributeDescription]) -> &mut Self {
        self.vi.vertexAttributeDescriptionCount = vattr.len() as _;
        self.vi.pVertexAttributeDescriptions = vattr.as_ptr_empty_null();
        self
    }
    /// Update the vertex input description
    pub fn vertex_input(
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
    pub fn enable_primitive_restart(&mut self, w: bool) -> &mut Self {
        self.ia.primitiveRestartEnable = w as _;
        self
    }
    /// Update the input primitive topology
    pub fn primitive_topology(&mut self, topo: VkPrimitiveTopology) -> &mut Self {
        self.ia.topology = topo;
        self
    }

    pub fn shader_stages_mut(&mut self) -> &mut ShaderStages {
        &mut self.shader_stages
    }
}

#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConservativeRasterizationMode {
    Overestimate = VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT as _,
    Underestimate = VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT as _,
}

/// PipelineStateDesc: Rasterization State
#[derive(Clone)]
pub struct RasterizationState {
    base: VkPipelineRasterizationStateCreateInfo,
    is_dynamic_depth_bias: bool,
    is_dynamic_line_width: bool,
    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    conservative: Option<VkPipelineRasterizationConservativeStateCreateInfoEXT>,
}
impl Default for RasterizationState {
    fn default() -> Self {
        RasterizationState {
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
        }
    }
}
impl RasterizationState {
    fn apply_dynamic_states(&self, st: &mut PipelineDynamicStates) {
        st.set(VK_DYNAMIC_STATE_DEPTH_BIAS, self.is_dynamic_depth_bias);
        st.set(VK_DYNAMIC_STATE_LINE_WIDTH, self.is_dynamic_line_width);
    }
    fn make_chained(&mut self) -> &VkPipelineRasterizationStateCreateInfo {
        #[cfg(feature = "VK_EXT_conservative_rasterization")]
        if let Some(ref c) = self.conservative {
            self.base.pNext = c as *const _ as _;
        }

        &self.base
    }

    /// Controls whether to clamp the fragment's depth values instead of clipping primitives to the z planes of the frustum,
    /// as described in `Primitive Clipping` in Vulkan Specification
    pub fn depth_clamp_enable(&mut self, enable: bool) -> &mut Self {
        self.base.depthClampEnable = enable as _;
        self
    }
    /// Controls whether primitives are discarded immediately before the rasterization stage
    pub fn rasterizer_discard_enable(&mut self, enable: bool) -> &mut Self {
        self.base.rasterizerDiscardEnable = enable as _;
        self
    }
    /// The triangle rendering mode
    pub fn polygon_mode(&mut self, mode: VkPolygonMode) -> &mut Self {
        self.base.polygonMode = mode;
        self
    }
    /// The triangle facing direction used for primitive culling
    pub fn cull_mode(&mut self, mode: VkCullModeFlags) -> &mut Self {
        self.base.cullMode = mode;
        self
    }
    /// The front-facing triangle orientation to be used for culling
    pub fn front_face(&mut self, face: VkFrontFace) -> &mut Self {
        self.base.frontFace = face;
        self
    }
    /// Specify `None` to disable to bias fragment depth values.  
    /// Tuple Member: (`ConstantFactor`, `Clamp`, `SlopeFactor`)
    ///
    /// - `ConstantFactor`: A scalar factor controlling the constant depth value added to each fragment
    /// - `Clamp`: The maximum (or minimum) depth bias of a fragment
    /// - `SlopeFactor`: A scalar factor applied to a fragment's slope in depth bias calculations
    pub fn depth_bias(&mut self, opts: SwitchOrDynamicState<(f32, f32, f32)>) -> &mut Self {
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
    pub fn line_width(&mut self, width: Option<f32>) -> &mut Self {
        self.is_dynamic_line_width = width.is_none();
        self.base.lineWidth = width.unwrap_or(0.0);
        self
    }

    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    /// [VK_EXT_conservative_rasterization] Sets conservative rasterization mode to use.
    pub fn conservative_rasterization_mode(
        &mut self,
        mode: ConservativeRasterizationMode,
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
    pub fn disable_conservative_rasterization(&mut self) -> &mut Self {
        self.conservative = None;
        self
    }
}

/// PipelineStateDesc: Multisample State
#[derive(Clone)]
pub struct MultisampleState<'d> {
    data: VkPipelineMultisampleStateCreateInfo,
    _samplemask_lifetime_binder: PhantomData<&'d [VkSampleMask]>,
}
impl<'d> MultisampleState<'d> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        MultisampleState {
            data: VkPipelineMultisampleStateCreateInfo {
                sType: VkPipelineMultisampleStateCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                rasterizationSamples: 1,
                sampleShadingEnable: VK_FALSE,
                minSampleShading: 1.0,
                pSampleMask: std::ptr::null(),
                alphaToCoverageEnable: VK_FALSE,
                alphaToOneEnable: VK_FALSE,
            },
            _samplemask_lifetime_binder: PhantomData,
        }
    }

    /// Specifies the number of samples per pixel used in rasterization. default=1
    pub fn rasterization_samples(&mut self, samples: usize) -> &mut Self {
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
    pub fn sample_shading(&mut self, min_sample_shading: Option<f32>) -> &mut Self {
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
    pub fn enable_alpha_to_coverage(&mut self, w: bool) -> &mut Self {
        self.data.alphaToCoverageEnable = w as _;
        self
    }
    /// Controls whether the alpha component of the fragment's first color output is replaced with one as described in
    /// [Multisample Coverage](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#fragops-covg).
    pub fn replace_alpha_to_one(&mut self, w: bool) -> &mut Self {
        self.data.alphaToOneEnable = w as _;
        self
    }
}
impl<'d> Into<LifetimeBound<'d, VkPipelineMultisampleStateCreateInfo>> for MultisampleState<'d> {
    fn into(self) -> LifetimeBound<'d, VkPipelineMultisampleStateCreateInfo> {
        LifetimeBound::new(self.data)
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

        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_graphics_pipelines(
                device.native_ptr(),
                cache.map(VkHandle::native_ptr).unwrap_or(VkPipelineCache::NULL),
                1,
                &cinfo,
                std::ptr::null(),
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
    Layout: PipelineLayout,
    RenderPass: crate::RenderPass,
    ShaderStages: PipelineShaderStageProvider,
> {
    flags: VkPipelineCreateFlags,
    _layout: Layout,
    rp: &'d RenderPass,
    subpass: u32,
    vp: VertexProcessingStages<'d, ShaderStages>,
    rasterizer_state: RasterizationState,
    tess_state: Option<Box<VkPipelineTessellationStateCreateInfo>>,
    viewport_state: Option<Box<VkPipelineViewportStateCreateInfo>>,
    ms_state: Option<MultisampleState<'d>>,
    ds_state: Option<Box<VkPipelineDepthStencilStateCreateInfo>>,
    color_blending: Option<(
        Box<VkPipelineColorBlendStateCreateInfo>,
        Vec<VkPipelineColorBlendAttachmentState>,
    )>,
    dynamic_state_flags: PipelineDynamicStates,
}

impl<'d, Layout: PipelineLayout, RenderPass: crate::RenderPass, ShaderStages: PipelineShaderStageProvider>
    NonDerivedGraphicsPipelineBuilder<'d, Layout, RenderPass, ShaderStages>
{
    /// Initialize the builder object
    pub fn new(layout: Layout, rpsp: (&'d RenderPass, u32), vp: VertexProcessingStages<'d, ShaderStages>) -> Self {
        Self {
            flags: 0,
            _layout: layout,
            rp: rpsp.0,
            subpass: rpsp.1,
            vp,
            rasterizer_state: Default::default(),
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
impl<'d, Layout: PipelineLayout, RenderPass: crate::RenderPass, ShaderStages: PipelineShaderStageProvider>
    NonDerivedGraphicsPipelineBuilder<'d, Layout, RenderPass, ShaderStages>
{
    /// Set the vertex processing stages in this pipeline
    pub fn vertex_processing(&mut self, vp: VertexProcessingStages<'d, ShaderStages>) -> &mut Self {
        self.vp = vp;
        self
    }
    /// Get a mutable reference to the vertex processing stage configuration in this pipeline
    pub fn vertex_processing_mut(&mut self) -> &mut VertexProcessingStages<'d, ShaderStages> {
        &mut self.vp
    }

    /// Number of control points per patch
    pub fn patch_control_point_count(&mut self, count: u32) -> &mut Self {
        if self.tess_state.is_none() {
            self.tess_state = Some(Box::new(VkPipelineTessellationStateCreateInfo {
                sType: VkPipelineTessellationStateCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                patchControlPoints: 0,
            }));
        }
        self.tess_state.as_mut().unwrap().patchControlPoints = count;
        self
    }
}

/// Viewport / Scissor State
impl<'d, Layout: PipelineLayout, RenderPass: crate::RenderPass, ShaderStages: PipelineShaderStageProvider>
    NonDerivedGraphicsPipelineBuilder<'d, Layout, RenderPass, ShaderStages>
{
    /// # Safety
    /// Application must guarantee that the number of viewports and scissors are identical
    pub unsafe fn viewports(&mut self, vps: DynamicArrayState<VkViewport>) -> &mut Self {
        if self.viewport_state.is_none() {
            self.viewport_state = Some(Box::new(VkPipelineViewportStateCreateInfo {
                sType: VkPipelineViewportStateCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                viewportCount: 0,
                pViewports: std::ptr::null(),
                scissorCount: 0,
                pScissors: std::ptr::null(),
            }));
        }

        self.viewport_state.as_mut().unwrap().viewportCount = vps.count() as _;
        self.viewport_state.as_mut().unwrap().pViewports = vps.as_ptr();
        self.dynamic_state_flags
            .set(VK_DYNAMIC_STATE_VIEWPORT, vps.is_dynamic());

        self
    }
    /// # Safety
    /// Application must guarantee that the number of viewports and scissors are identical
    pub unsafe fn scissors(&mut self, scs: DynamicArrayState<VkRect2D>) -> &mut Self {
        if self.viewport_state.is_none() {
            self.viewport_state = Some(Box::new(VkPipelineViewportStateCreateInfo {
                sType: VkPipelineViewportStateCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                viewportCount: 0,
                pViewports: std::ptr::null(),
                scissorCount: 0,
                pScissors: std::ptr::null(),
            }));
        }

        self.viewport_state.as_mut().unwrap().scissorCount = scs.count() as _;
        self.viewport_state.as_mut().unwrap().pScissors = scs.as_ptr();
        self.dynamic_state_flags.set(VK_DYNAMIC_STATE_SCISSOR, scs.is_dynamic());

        self
    }
    /// Safety way calling `viewports` and `scissors`
    pub fn viewport_scissors(
        &mut self,
        vps: DynamicArrayState<VkViewport>,
        scissor: DynamicArrayState<VkRect2D>,
    ) -> &mut Self {
        assert_eq!(vps.count(), scissor.count());
        unsafe { self.viewports(vps).scissors(scissor) }
    }
}

impl<'d, Layout: PipelineLayout, RenderPass: crate::RenderPass, ShaderStages: PipelineShaderStageProvider>
    NonDerivedGraphicsPipelineBuilder<'d, Layout, RenderPass, ShaderStages>
{
    /// Rasterization State
    pub fn rasterization_state(&mut self, state: RasterizationState) -> &mut Self {
        self.rasterizer_state = state;
        self
    }
    /// Multisample State
    pub fn multisample_state(&mut self, state: Option<MultisampleState<'d>>) -> &mut Self {
        self.ms_state = state;
        self
    }
}

/// Depth/Stencil State
impl<'d, Layout: PipelineLayout, RenderPass: crate::RenderPass, ShaderStages: PipelineShaderStageProvider>
    NonDerivedGraphicsPipelineBuilder<'d, Layout, RenderPass, ShaderStages>
{
    /// Clear depth/stencil state
    pub fn clear_depth_stencil_state(&mut self) -> &mut Self {
        self.ds_state = None;
        self
    }
    fn dss_ref(&mut self) -> &mut VkPipelineDepthStencilStateCreateInfo {
        if self.ds_state.is_none() {
            self.ds_state = Some(Box::new(VkPipelineDepthStencilStateCreateInfo {
                sType: VkPipelineDepthStencilStateCreateInfo::TYPE,
                pNext: std::ptr::null(),
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
                    compareOp: VK_COMPARE_OP_EQUAL,
                    compareMask: 0,
                    writeMask: 0,
                    reference: 0,
                },
                back: VkStencilOpState {
                    failOp: VK_STENCIL_OP_KEEP,
                    passOp: VK_STENCIL_OP_KEEP,
                    depthFailOp: VK_STENCIL_OP_KEEP,
                    compareOp: VK_COMPARE_OP_EQUAL,
                    compareMask: 0,
                    writeMask: 0,
                    reference: 0,
                },
                minDepthBounds: 0.0,
                maxDepthBounds: 1.0,
            }));
        }
        self.ds_state.as_mut().unwrap()
    }
    /// Controls whether depth testing is enabled
    pub fn depth_test_enable(&mut self, enable: bool) -> &mut Self {
        self.dss_ref().depthTestEnable = enable as _;
        self
    }
    /// Controls whether depth writes are enabled, or always disabled
    pub fn depth_write_enable(&mut self, enable: bool) -> &mut Self {
        self.dss_ref().depthWriteEnable = enable as _;
        self
    }
    /// The comparison operator used in the depth test
    pub fn depth_compare_op(&mut self, op: CompareOp) -> &mut Self {
        self.dss_ref().depthCompareOp = op as _;
        self
    }
    /// Controls whether depth testing is enabled, depth writes are enabled, and the comparison operator used in the depth test
    /// Specifying `None` to `compare_to` disables depth testing
    pub fn depth_test_settings(&mut self, compare_op: Option<CompareOp>, write_enable: bool) -> &mut Self {
        if let Some(op) = compare_op {
            self.depth_test_enable(true).depth_compare_op(op);
        } else {
            self.depth_test_enable(false);
        }
        self.depth_write_enable(write_enable)
    }
    /// Controls whether depth bounds testing is enabled
    pub fn depth_bounds_test_enable(&mut self, enable: bool) -> &mut Self {
        self.dss_ref().depthBoundsTestEnable = enable as _;
        self
    }
    /// Controls whether stencil testing is enabled
    pub fn stencil_test_enable(&mut self, enable: bool) -> &mut Self {
        self.dss_ref().stencilTestEnable = enable as _;
        self
    }
    /// Control the parameter of the stencil test
    pub fn stencil_control_front(&mut self, state: VkStencilOpState) -> &mut Self {
        self.dynamic_state_flags.disable(VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK);
        self.dynamic_state_flags.disable(VK_DYNAMIC_STATE_STENCIL_WRITE_MASK);
        self.dynamic_state_flags.disable(VK_DYNAMIC_STATE_STENCIL_REFERENCE);
        self.dss_ref().front = state;
        self
    }
    /// Control the parameter of the stencil test
    pub fn stencil_control_back(&mut self, state: VkStencilOpState) -> &mut Self {
        self.dynamic_state_flags.disable(VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK);
        self.dynamic_state_flags.disable(VK_DYNAMIC_STATE_STENCIL_WRITE_MASK);
        self.dynamic_state_flags.disable(VK_DYNAMIC_STATE_STENCIL_REFERENCE);
        self.dss_ref().back = state;
        self
    }
    /// Convenient function for setting same stencil_control values both front and back
    pub fn stencil_control(&mut self, state: VkStencilOpState) -> &mut Self {
        self.stencil_control_front(state.clone()).stencil_control_back(state)
    }
    /// Controls the parameter of the compare mask of the stencil test. Tuple ordering: (front, back).
    /// Specifying `None` means that the parameter is a dynamic state
    pub fn stencil_compare_mask(&mut self, mask: Option<(u32, u32)>) -> &mut Self {
        let is_dynamic = if let Some((f, b)) = mask {
            self.dss_ref().front.compareMask = f;
            self.dss_ref().back.compareMask = b;
            false
        } else {
            true
        };
        self.dynamic_state_flags
            .set(VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK, is_dynamic);
        self
    }
    /// Controls the parameter of the write mask of the stencil test. Tuple ordering: (front, back)
    /// Specifying `None` means that the parameter is a dynamic state
    pub fn stencil_write_mask(&mut self, mask: Option<(u32, u32)>) -> &mut Self {
        let is_dynamic = if let Some((f, b)) = mask {
            self.dss_ref().front.writeMask = f;
            self.dss_ref().back.writeMask = b;
            false
        } else {
            true
        };
        self.dynamic_state_flags
            .set(VK_DYNAMIC_STATE_STENCIL_WRITE_MASK, is_dynamic);
        self
    }
    /// Controls the parameter of the reference of the stencil test. Tuple ordering: (front, back)
    /// Specifying `None` means that the parameter is a dynamic state
    pub fn stencil_reference(&mut self, mask: Option<(u32, u32)>) -> &mut Self {
        let is_dynamic = if let Some((f, b)) = mask {
            self.dss_ref().front.reference = f;
            self.dss_ref().back.reference = b;
            false
        } else {
            true
        };
        self.dynamic_state_flags
            .set(VK_DYNAMIC_STATE_STENCIL_REFERENCE, is_dynamic);
        self
    }
    /// The range of values used in the depth bounds test
    pub fn depth_bounds_range(&mut self, bounds: Range<f32>) -> &mut Self {
        self.dss_ref().minDepthBounds = bounds.start;
        self.dss_ref().maxDepthBounds = bounds.end;
        self
    }
    /// Control the depth bounds test
    pub fn depth_bounds(&mut self, bounds: SwitchOrDynamicState<Range<f32>>) -> &mut Self {
        self.depth_bounds_test_enable(bounds.is_enabled());
        self.dynamic_state_flags
            .set(VK_DYNAMIC_STATE_DEPTH_BOUNDS, bounds.is_dynamic());
        if let SwitchOrDynamicState::Static(r) = bounds {
            self.depth_bounds_range(r)
        } else {
            self
        }
    }
}

/// Blending Factor
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlendFactor {
    Zero = VK_BLEND_FACTOR_ZERO as _,
    One = VK_BLEND_FACTOR_ONE as _,
    SourceColor = VK_BLEND_FACTOR_SRC_COLOR as _,
    OneMinusSourceColor = VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR as _,
    DestColor = VK_BLEND_FACTOR_DST_COLOR as _,
    OneMinusDestColor = VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR as _,
    SourceAlpha = VK_BLEND_FACTOR_SRC_ALPHA as _,
    OneMinusSourceAlpha = VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA as _,
    DestAlpha = VK_BLEND_FACTOR_DST_ALPHA as _,
    OneMinusDestAlpha = VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA as _,
    ConstantColor = VK_BLEND_FACTOR_CONSTANT_COLOR as _,
    OneMinusConstantColor = VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR as _,
    ConstantAlpha = VK_BLEND_FACTOR_CONSTANT_ALPHA as _,
    OneMinusConstantAlpha = VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA as _,
    /// (f, f, f, 1) where f = min(Source Alpha, 1 - Dest Alpha)
    SrcAlphaSat = VK_BLEND_FACTOR_SRC_ALPHA_SATURATE as _,
    AltSourceColor = VK_BLEND_FACTOR_SRC1_COLOR as _,
    OneMinusAltSourceColor = VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR as _,
    AltSourceAlpha = VK_BLEND_FACTOR_SRC1_ALPHA as _,
    OneMinusAltSourceAlpha = VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA as _,
}
/// Blending Op
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlendOp {
    Add = VK_BLEND_OP_ADD as _,
    Sub = VK_BLEND_OP_SUBTRACT as _,
    /// Reverse subtraction order(destination - source)
    RevSub = VK_BLEND_OP_REVERSE_SUBTRACT as _,
    Min = VK_BLEND_OP_MIN as _,
    Max = VK_BLEND_OP_MAX as _,
}

/// Structure specifying a pipeline color blend attachment state
#[derive(Clone)]
pub struct AttachmentColorBlendState(VkPipelineColorBlendAttachmentState);
impl AttachmentColorBlendState {
    #[inline]
    pub fn noblend() -> Self {
        AttachmentColorBlendState(VkPipelineColorBlendAttachmentState {
            colorWriteMask: VK_COLOR_COMPONENT_A_BIT
                | VK_COLOR_COMPONENT_R_BIT
                | VK_COLOR_COMPONENT_G_BIT
                | VK_COLOR_COMPONENT_B_BIT,
            blendEnable: VK_FALSE,
            ..unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
        })
    }

    #[inline]
    // https://stackoverflow.com/questions/18918643/how-to-achieve-d3d-output-with-premultiplied-alpha-for-use-with-d3dimage-in-wpf
    pub const fn premultiplied() -> Self {
        AttachmentColorBlendState(VkPipelineColorBlendAttachmentState {
            colorWriteMask: VK_COLOR_COMPONENT_A_BIT
                | VK_COLOR_COMPONENT_R_BIT
                | VK_COLOR_COMPONENT_G_BIT
                | VK_COLOR_COMPONENT_B_BIT,
            blendEnable: VK_TRUE,
            srcColorBlendFactor: BlendFactor::One as _,
            dstColorBlendFactor: BlendFactor::OneMinusSourceAlpha as _,
            colorBlendOp: BlendOp::Add as _,
            // srcAlphaBlendFactor: BlendFactor::OneMinusDestAlpha as _,
            // dstAlphaBlendFactor: BlendFactor::One as _,
            srcAlphaBlendFactor: BlendFactor::One as _,
            dstAlphaBlendFactor: BlendFactor::OneMinusSourceAlpha as _,
            alphaBlendOp: BlendOp::Add as _,
        })
    }

    pub fn enable(&mut self) -> &mut Self {
        self.0.blendEnable = VK_TRUE;
        self
    }
    pub fn disable(&mut self) -> &mut Self {
        self.0.blendEnable = VK_FALSE;
        self
    }
    pub fn color_blend_factor_src(&mut self, f: BlendFactor) -> &mut Self {
        self.0.srcColorBlendFactor = f as _;
        self
    }
    pub fn color_blend_factor_dst(&mut self, f: BlendFactor) -> &mut Self {
        self.0.dstColorBlendFactor = f as _;
        self
    }
    pub fn alpha_blend_factor_src(&mut self, f: BlendFactor) -> &mut Self {
        self.0.srcAlphaBlendFactor = f as _;
        self
    }
    pub fn alpha_blend_factor_dst(&mut self, f: BlendFactor) -> &mut Self {
        self.0.dstAlphaBlendFactor = f as _;
        self
    }
    pub fn color_blend_op(&mut self, op: BlendOp) -> &mut Self {
        self.0.colorBlendOp = op as _;
        self
    }
    pub fn alpha_blend_op(&mut self, op: BlendOp) -> &mut Self {
        self.0.alphaBlendOp = op as _;
        self
    }
    pub fn color_blend(&mut self, src: BlendFactor, op: BlendOp, dst: BlendFactor) -> &mut Self {
        self.color_blend_factor_src(src)
            .color_blend_op(op)
            .color_blend_factor_dst(dst)
    }
    pub fn alpha_blend(&mut self, src: BlendFactor, op: BlendOp, dst: BlendFactor) -> &mut Self {
        self.alpha_blend_factor_src(src)
            .alpha_blend_op(op)
            .alpha_blend_factor_dst(dst)
    }
}

/// Color Blending
impl<'d, Layout: PipelineLayout, RenderPass: crate::RenderPass, ShaderStages: PipelineShaderStageProvider>
    NonDerivedGraphicsPipelineBuilder<'d, Layout, RenderPass, ShaderStages>
{
    fn cb_ref(
        &mut self,
    ) -> &mut (
        Box<VkPipelineColorBlendStateCreateInfo>,
        Vec<VkPipelineColorBlendAttachmentState>,
    ) {
        if self.color_blending.is_none() {
            self.color_blending = Some((
                Box::new(VkPipelineColorBlendStateCreateInfo {
                    sType: VkPipelineColorBlendStateCreateInfo::TYPE,
                    pNext: std::ptr::null(),
                    flags: 0,
                    logicOpEnable: VK_FALSE,
                    logicOp: VK_LOGIC_OP_NO_OP,
                    attachmentCount: 0,
                    pAttachments: std::ptr::null(),
                    blendConstants: [0.0; 4],
                }),
                Vec::new(),
            ))
        }
        self.color_blending.as_mut().unwrap()
    }

    /// Which logical operation to apply. Specifying `None` disables *Logical Operations*
    pub fn logic_op(&mut self, op: Option<LogicOp>) -> &mut Self {
        let (ref mut state, _) = self.cb_ref();
        state.logicOpEnable = op.is_some() as _;
        state.logicOp = op.unwrap_or(LogicOp::NoOp) as _;
        self
    }
    /// Per target attachment states
    pub fn add_attachment_blend(&mut self, blend: AttachmentColorBlendState) -> &mut Self {
        {
            let cb = self.cb_ref();
            cb.1.push(blend.0);
            cb.0.attachmentCount = cb.1.len() as _;
            cb.0.pAttachments = cb.1.as_ptr_empty_null();
        }
        self
    }
    /// Sets per-target attachment states
    pub fn set_attachment_blends(&mut self, blends: Vec<VkPipelineColorBlendAttachmentState>) -> &mut Self {
        let (ref mut state, ref mut blend_infos) = self.cb_ref();
        *blend_infos = blends;
        state.attachmentCount = blend_infos.len() as _;
        state.pAttachments = blend_infos.as_ptr_empty_null();
        self
    }
    /// Clears per-target attachment blending state
    pub fn clear_attachment_blends(&mut self) -> &mut Self {
        self.cb_ref().1.clear();
        self
    }
    /// Clears blending state
    pub fn clear_blending_state(&mut self) -> &mut Self {
        self.color_blending = None;
        self
    }

    /// Array of four values used as the R, G, B, and A components of the blend constant that are used in blending, depending on the blend factor.
    /// Specifying `None` means that the `blendConstants` parameter is a dynamic state
    pub fn blend_constants(&mut self, values: Option<[f32; 4]>) -> &mut Self {
        self.dynamic_state_flags
            .set(VK_DYNAMIC_STATE_BLEND_CONSTANTS, values.is_none());
        self.cb_ref()
            .0
            .blendConstants
            .copy_from_slice(&values.unwrap_or([0.0; 4]));
        self
    }
}

/// Misc Configurations
impl<'d, Layout: PipelineLayout, RenderPass: crate::RenderPass, ShaderStages: PipelineShaderStageProvider>
    NonDerivedGraphicsPipelineBuilder<'d, Layout, RenderPass, ShaderStages>
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
    pub fn render_pass(&mut self, rpo: &'d RenderPass, subpass: u32) -> &mut Self {
        self.rp = rpo;
        self.subpass = subpass;
        self
    }
    /// The created pipeline will be optimized.  
    /// Disabling optimization of the pipeline may reduce the time taken to create the pipeline
    pub fn enable_optimization(&mut self) -> &mut Self {
        self.flags &= !VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT;
        self
    }
    /// The created pipeline will not be optimized.  
    /// Disabling optimization of the pipeline may reduce the time taken to create the pipeline
    pub fn disable_optimization(&mut self) -> &mut Self {
        self.flags |= VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT;
        self
    }
    /// The pipeline to be created is allowed to be the parent of a pipeline that will be created in a subsequent creation operation
    pub fn allow_derivatives(&mut self) -> &mut Self {
        self.flags |= VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT;
        self
    }
    /// The pipeline to be created is denied to be the parent of a pipeline that will be created in a subsequent creation operation
    pub fn deny_derivatives(&mut self) -> &mut Self {
        self.flags &= !VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT;
        self
    }
}

/// Unsafe Utilities
impl<'d, Layout: PipelineLayout, RenderPass: crate::RenderPass, ShaderStages: PipelineShaderStageProvider>
    NonDerivedGraphicsPipelineBuilder<'d, Layout, RenderPass, ShaderStages>
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
    /// Set the `VkPipelineViewportStateCreateInfo` structure directly.
    /// This does not clear any dynamic states
    /// # Safety
    /// Application must guarantee these constraints:
    ///
    /// - The lifetime of the content in the structure is valid for this builder
    /// - The content in the structure is valid
    pub unsafe fn viewport_state_create_info(
        &mut self,
        state: Option<Box<VkPipelineViewportStateCreateInfo>>,
    ) -> &mut Self {
        self.viewport_state = state;
        self
    }
    /// Set the `VkPipelineDepthStencilStateCreateInfo` structure directly.
    /// This does not clear any dynamic states
    /// # Safety
    /// Application must guarantee these constraints:
    ///
    /// - The lifetime of the content in the structure is valid for this builder
    /// - The content in the structure is valid
    pub unsafe fn depth_stencil_state_create_info(
        &mut self,
        state: Option<Box<VkPipelineDepthStencilStateCreateInfo>>,
    ) -> &mut Self {
        self.ds_state = state;
        self
    }
    /// Set the `VkPipelineColorBlendStateCreateInfo` structure directly.
    /// This does not clear any dynamic states
    /// # Safety
    /// Application must guarantee these constraints:
    ///
    /// - The lifetime of the content in the structure is valid for this builder
    /// - The content in the structure is valid
    pub unsafe fn color_blend_state_info(
        &mut self,
        state: Option<Box<VkPipelineColorBlendStateCreateInfo>>,
    ) -> &mut Self {
        self.color_blending = state.map(|x| (x, Vec::new()));
        self
    }
}
pub struct NonDerivedGraphicPipelineBuilderExtraStorage<ShaderStages: PipelineShaderStageProvider> {
    pub stages: Vec<VkPipelineShaderStageCreateInfo>,
    pub shader_stage_extras: ShaderStages::ExtraStorage,
}
impl<'d, Layout: PipelineLayout, RenderPass: crate::RenderPass, ShaderStages: PipelineShaderStageProvider>
    GraphicsPipelineBuilder for NonDerivedGraphicsPipelineBuilder<'d, Layout, RenderPass, ShaderStages>
{
    type ExtraStorage = NonDerivedGraphicPipelineBuilderExtraStorage<ShaderStages>;

    fn build(&mut self, extras: &Self::ExtraStorage) -> VkGraphicsPipelineCreateInfo {
        self.rasterizer_state
            .apply_dynamic_states(&mut self.dynamic_state_flags);
        let ds = if !self.dynamic_state_flags.0.is_empty() {
            unsafe { Some(Into::<LifetimeBound<_>>::into(&self.dynamic_state_flags).unbound()) }
        } else {
            None
        };
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
            stageCount: extras.stages.len() as _,
            pStages: extras.stages.as_ptr_empty_null(),
            pVertexInputState: &self.vp.vi,
            pInputAssemblyState: &self.vp.ia,
            pTessellationState: self
                .tess_state
                .as_ref()
                .map(|x| &**x as *const _)
                .unwrap_or_else(std::ptr::null),
            pViewportState: self
                .viewport_state
                .as_ref()
                .map(|x| &**x as *const _)
                .unwrap_or_else(std::ptr::null),
            pRasterizationState: rst,
            pMultisampleState: ms.map_or_else(std::ptr::null, |x| x as *const _),
            pDepthStencilState: self
                .ds_state
                .as_ref()
                .map(|x| &**x as *const _)
                .unwrap_or_else(std::ptr::null),
            pColorBlendState: self
                .color_blending
                .as_ref()
                .map(|&(ref x, _)| &**x as *const _)
                .unwrap_or_else(std::ptr::null),
            pDynamicState: ds.as_ref().map(|x| x as *const _).unwrap_or_else(std::ptr::null),
            layout: self._layout.native_ptr(),
            renderPass: self.rp.native_ptr(),
            subpass: self.subpass,
            basePipelineHandle: VkPipeline::NULL,
            basePipelineIndex: -1,
            flags: 0,
        }
    }
    fn make_extras(&self) -> Self::ExtraStorage {
        let shader_stage_extras = self.vp.shader_stages.make_extras();
        let stages = self.vp.shader_stages.base_struct(&shader_stage_extras);

        NonDerivedGraphicPipelineBuilderExtraStorage {
            stages,
            shader_stage_extras,
        }
    }
}

pub struct DerivedGraphicsPipelineBuilder<Base: Pipeline, Diff: GraphicsPipelineBuilder>(Base, Diff);
impl<Base: Pipeline, Diff: GraphicsPipelineBuilder> DerivedGraphicsPipelineBuilder<Base, Diff> {
    pub fn diff_mut(&mut self) -> &mut Diff {
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
    pub fn diff_mut(&mut self) -> &mut Diff {
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
        let stage = self.shader.base_struct(ShaderStage::COMPUTE, &extras);

        let cinfo = VkComputePipelineCreateInfo {
            sType: VkComputePipelineCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            basePipelineHandle: VkPipeline::NULL,
            basePipelineIndex: -1,
            stage,
            layout: self.layout.native_ptr(),
        };

        let mut pipeline = ::std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_compute_pipelines(
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
#[repr(transparent)]
pub struct PipelineStageFlags(pub VkPipelineStageFlags);
impl PipelineStageFlags {
    /// The stage of the pipeline where any commands are initially received by the queue
    pub const TOP_OF_PIPE: Self = PipelineStageFlags(VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT);
    /// The stage of the pipeline where Draw/DispatchIndirect data structures are consumed
    pub const DRAW_INDIRECT: Self = PipelineStageFlags(VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT);
    /// The stage of the pipeline where vertex and index buffers are consumed
    pub const VERTEX_INPUT: Self = PipelineStageFlags(VK_PIPELINE_STAGE_VERTEX_INPUT_BIT);
    /// The vertex shader stage
    pub const VERTEX_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_VERTEX_SHADER_BIT);
    /// The tessellation control shader stage
    pub const TESSELLATION_CONTROL_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT);
    /// The tessellation evaluation shader stage
    pub const TESSELLATION_EVALUATION_SHADER: Self =
        PipelineStageFlags(VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT);
    /// The geometry shader stage
    pub const GEOMETRY_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT);
    /// The fragment shader stage
    pub const FRAGMENT_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
    /// The stage of the pipeline where early fragment tests (depth and stencil tests before fragment shading) are performed
    pub const EARLY_FRAGMENT_TESTS: Self = PipelineStageFlags(VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT);
    /// The stage of the pipeline where late fragment tests (depth and stencil tests after fragment shading) are performed
    pub const LATE_FRAGMENT_TESTS: Self = PipelineStageFlags(VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT);
    /// The stage of the pipeline after blending where the final color values are output from the pipeline
    pub const COLOR_ATTACHMENT_OUTPUT: Self = PipelineStageFlags(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT);
    /// The execution of copy commands
    pub const TRANSFER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_TRANSFER_BIT);
    /// The execution of a compute shader
    pub const COMPUTE_SHADER: Self = PipelineStageFlags(VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT);
    /// The final stage in the pipeline where operations generated by all commands complete execution
    pub const BOTTOM_OF_PIPE: Self = PipelineStageFlags(VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT);
    /// A pseudo-stage indicating execution on the host of reads/writes of device memory
    pub const HOST: Self = PipelineStageFlags(VK_PIPELINE_STAGE_HOST_BIT);
    /// The execution of all graphics pipeline stages
    pub const ALL_GRAPHICS: Self = PipelineStageFlags(VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT);
    /// Equivalent to the logical OR of every other pipeline stage flag that is supported on the quue it is used with
    pub const ALL_COMMANDS: Self = PipelineStageFlags(VK_PIPELINE_STAGE_ALL_COMMANDS_BIT);

    /// The stage of the pipeline where any commands are initially received by the queue
    pub fn top_of_pipe(self) -> Self {
        PipelineStageFlags(self.0 | Self::TOP_OF_PIPE.0)
    }
    /// The stage of the pipeline where Draw/DispatchIndirect data structures are consumed
    pub fn draw_indirect(self) -> Self {
        PipelineStageFlags(self.0 | Self::DRAW_INDIRECT.0)
    }
    /// The stage of the pipeline where vertex and index buffers are consumed
    pub fn vertex_input(self) -> Self {
        PipelineStageFlags(self.0 | Self::VERTEX_INPUT.0)
    }
    /// The vertex shader stage
    pub fn vertex_shader(self) -> Self {
        PipelineStageFlags(self.0 | Self::VERTEX_SHADER.0)
    }
    /// The tessellation control shader stage
    pub fn tessellation_control_shader(self) -> Self {
        PipelineStageFlags(self.0 | Self::TESSELLATION_CONTROL_SHADER.0)
    }
    /// The tessellation evaluation shader stage
    pub fn tessellation_evaluation_shader(self) -> Self {
        PipelineStageFlags(self.0 | Self::TESSELLATION_EVALUATION_SHADER.0)
    }
    /// The geometry shader stage
    pub fn geometry_shader(self) -> Self {
        PipelineStageFlags(self.0 | Self::GEOMETRY_SHADER.0)
    }
    /// The fragment shader stage
    pub fn fragment_shader(self) -> Self {
        PipelineStageFlags(self.0 | Self::FRAGMENT_SHADER.0)
    }
    /// The stage of the pipeline where early fragment tests (depth and stencil tests before fragment shading) are performed
    pub fn early_fragment_tests(self) -> Self {
        PipelineStageFlags(self.0 | Self::EARLY_FRAGMENT_TESTS.0)
    }
    /// The stage of the pipeline where late fragment tests (depth and stencil tests after fragment shading) are performed
    pub fn late_fragment_tests(self) -> Self {
        PipelineStageFlags(self.0 | Self::LATE_FRAGMENT_TESTS.0)
    }
    /// The stage of the pipeline after blending where the final color values are output from the pipeline
    pub fn color_attachment_output(self) -> Self {
        PipelineStageFlags(self.0 | Self::COLOR_ATTACHMENT_OUTPUT.0)
    }
    /// The execution of copy commands
    pub fn transfer(self) -> Self {
        PipelineStageFlags(self.0 | Self::TRANSFER.0)
    }
    /// The execution of a compute shader
    pub fn compute_shader(self) -> Self {
        PipelineStageFlags(self.0 | Self::COMPUTE_SHADER.0)
    }
    /// The final stage in the pipeline where operations generated by all commands complete execution
    pub fn bottom_of_pipe(self) -> Self {
        PipelineStageFlags(self.0 | Self::BOTTOM_OF_PIPE.0)
    }
    /// A pseudo-stage indicating execution on the host of reads/writes of device memory
    pub fn host(self) -> Self {
        PipelineStageFlags(self.0 | Self::HOST.0)
    }
    /// The execution of all graphics pipeline stages
    pub fn all_graphics(self) -> Self {
        PipelineStageFlags(self.0 | Self::ALL_GRAPHICS.0)
    }
    /// Equivalent to the logical OR of every other pipeline stage flag that is supported on the quue it is used with
    pub fn all_commands(self) -> Self {
        PipelineStageFlags(self.0 | Self::ALL_COMMANDS.0)
    }
}
impl BitOr for PipelineStageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        PipelineStageFlags(self.0 | rhs.0)
    }
}
impl BitOrAssign for PipelineStageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
