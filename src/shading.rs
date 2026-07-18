//! Vulkan Shading(Shader/Pipeline)
use bedrock_vk::{self as brvk, TypedVulkanStructure, VkRawHandle};

use crate::*;
use crate::{error::translate_vk_result, ffi_helper::slice_as_ptr_empty_null};
use core::{
    ffi::{CStr, c_void},
    marker::PhantomData,
    ops::*,
};
use derives::{bitflags_newtype, implements};

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Hash)]
pub enum ShaderStage {
    Vertex = brvk::VK_SHADER_STAGE_VERTEX_BIT,
    TessellationControl = brvk::VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT,
    TessellationEvaluation = brvk::VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
    Geometry = brvk::VK_SHADER_STAGE_GEOMETRY_BIT,
    Fragment = brvk::VK_SHADER_STAGE_FRAGMENT_BIT,
    Compute = brvk::VK_SHADER_STAGE_COMPUTE_BIT,
}

/// Stencil comparison function
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompareOp {
    /// The test never passes
    Never = brvk::VK_COMPARE_OP_NEVER,
    /// The test passes when `Ref < Stencil`
    Less = brvk::VK_COMPARE_OP_LESS,
    /// The test passes when `Ref == Stencil`
    Equal = brvk::VK_COMPARE_OP_EQUAL,
    /// The test passes when `Ref <= Stencil`
    LessOrEqual = brvk::VK_COMPARE_OP_LESS_OR_EQUAL,
    /// The test passes when `Ref > Stencil`
    Greater = brvk::VK_COMPARE_OP_GREATER,
    /// The test passes when `Ref != Stencil`
    NotEqual = brvk::VK_COMPARE_OP_NOT_EQUAL,
    /// The test passes when `Ref >= Stencil`
    GreaterOrEqual = brvk::VK_COMPARE_OP_GREATER_OR_EQUAL,
    /// The test always passes
    Always = brvk::VK_COMPARE_OP_ALWAYS,
}

/// Stencil action function
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StencilOp {
    /// Keeps the current value
    Keep = brvk::VK_STENCIL_OP_KEEP,
    /// Sets the value to 0
    Zero = brvk::VK_STENCIL_OP_ZERO,
    /// Sets the value to `reference`
    Replace = brvk::VK_STENCIL_OP_REPLACE,
    /// Increments the current value and clamps to the maximum representable unsigned value
    IncrementClamp = brvk::VK_STENCIL_OP_INCREMENT_AND_CLAMP,
    /// Decrements the current value and clamps to 0
    DecrementClamp = brvk::VK_STENCIL_OP_DECREMENT_AND_CLAMP,
    /// Bitwise-inverts the current value
    Invert = brvk::VK_STENCIL_OP_INVERT,
    /// Increments the current value and wraps to 0 when the maximum value would have been exceeded
    IncrementWrap = brvk::VK_STENCIL_OP_INCREMENT_AND_WRAP,
    /// Decrements the current value and wraps to the maximum possible value when the value would go below 0
    DecrementWrap = brvk::VK_STENCIL_OP_DECREMENT_AND_WRAP,
}

/// Framebuffer logical operations
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOp {
    /// 0
    Clear = brvk::VK_LOGIC_OP_CLEAR,
    /// source & dest
    And = brvk::VK_LOGIC_OP_AND,
    /// source & ~dest
    AndReverse = brvk::VK_LOGIC_OP_AND_REVERSE,
    /// source
    Copy = brvk::VK_LOGIC_OP_COPY,
    /// ~source & dest
    AndInverted = brvk::VK_LOGIC_OP_AND_INVERTED,
    /// dest
    NoOp = brvk::VK_LOGIC_OP_NO_OP,
    /// source ^ dest
    Xor = brvk::VK_LOGIC_OP_XOR,
    /// source | dest
    Or = brvk::VK_LOGIC_OP_OR,
    /// ~(source | dest)
    Nor = brvk::VK_LOGIC_OP_NOR,
    /// ~(source ^ dest)
    Equivalent = brvk::VK_LOGIC_OP_EQUIVALENT,
    /// ~dest
    Invert = brvk::VK_LOGIC_OP_INVERT,
    /// source | ~dest
    OrReverse = brvk::VK_LOGIC_OP_OR_REVERSE,
    /// ~source
    CopyInverted = brvk::VK_LOGIC_OP_COPY_INVERTED,
    /// ~source | dest
    OrInverted = brvk::VK_LOGIC_OP_OR_INVERTED,
    /// ~(source & dest)
    Nand = brvk::VK_LOGIC_OP_NAND,
    /// 1
    Set = brvk::VK_LOGIC_OP_SET,
}

/// Bitmask specifying sets of stencil state for which to update the compare mask
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum StencilFaceMask {
    /// Only the front set of stencil state
    Front = brvk::VK_STENCIL_FACE_FRONT_BIT,
    /// Only the back set of stencil state
    Back = brvk::VK_STENCIL_FACE_BACK_BIT,
    /// Both sets of stencil state
    Both = brvk::VK_STENCIL_FACE_FRONT_AND_BACK,
}

#[repr(transparent)]
pub struct StencilOpState(pub brvk::VkStencilOpState);
impl StencilOpState {
    pub const NOP: Self = Self::always_forall(StencilOp::Keep);

    pub const fn always(pass: StencilOp, fail: StencilOp, depth_fail: StencilOp) -> Self {
        Self(brvk::VkStencilOpState {
            passOp: pass as _,
            failOp: fail as _,
            depthFailOp: depth_fail as _,
            compareOp: CompareOp::Always as _,
            compareMask: 0,
            writeMask: 0,
            reference: 0,
        })
    }

    pub const fn always_forall(op: StencilOp) -> Self {
        Self::always(op, op, op)
    }

    pub const fn write_mask(mut self, mask: u32) -> Self {
        self.0.writeMask = mask;
        self
    }

    pub const fn set_compare(mut self, op: CompareOp, reference: u32, mask: u32) -> Self {
        self.0.compareOp = op as _;
        self.0.reference = reference;
        self.0.compareMask = mask;
        self
    }

    pub const fn always_pass(mut self) -> Self {
        self.0.compareOp = CompareOp::Always as _;
        self
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ShaderModuleCreateInfo<'d>(brvk::VkShaderModuleCreateInfo, core::marker::PhantomData<&'d [u32]>);
impl<'d> ShaderModuleCreateInfo<'d> {
    pub const fn new(code: &'d [u32]) -> Self {
        Self(
            brvk::VkShaderModuleCreateInfo {
                sType: brvk::VkShaderModuleCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                // Note: ここはbyte単位
                codeSize: code.len() << 2,
                pCode: slice_as_ptr_empty_null(code),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkShaderModuleCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkShaderModuleCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkShaderModuleCreateInfo {
        self.0
    }
}

/// Opaque handle to a shader module object
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VkShaderModule::OBJECT_TYPE)]
pub struct ShaderModuleObject<Device: VkHandle<Handle = brvk::VkDevice>>(brvk::VkShaderModule, Device);
#[implements]
impl<Device: VkHandle<Handle = brvk::VkDevice>> Drop for ShaderModuleObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn_wrapper::destroy_shader_module(
                self.1.as_transparent_ref(),
                VkHandleRefMut::dangling(self.0),
                None,
            );
        }
    }
}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Sync> Sync for ShaderModuleObject<Device> {}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Send> Send for ShaderModuleObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> ShaderModule for ShaderModuleObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> ShaderModuleObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: brvk::VkShaderModule, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (brvk::VkShaderModule, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice> + Clone> ShaderModuleObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> ShaderModuleObject<Device> {
        let r = ShaderModuleObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}
impl<Device: crate::Device> ShaderModuleObject<Device> {
    /// Creates a new shader module object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_INVALID_SHADER_NV`]
    #[implements]
    #[inline]
    pub fn new(device: Device, info: &ShaderModuleCreateInfo) -> crate::Result<Self> {
        Ok(unsafe {
            Self::manage(
                crate::vkfn_wrapper::create_shader_module(device.as_transparent_ref(), info, None)?,
                device,
            )
        })
    }
}

pub trait ShaderModule: VkHandle<Handle = brvk::VkShaderModule> {
    /// Constructs a new [`PipelineShaderStage`] data.
    #[inline(always)]
    fn on_stage<'m, 's>(&'m self, stage: ShaderStage, entry_point: &'m CStr) -> PipelineShaderStage<'m, 's> {
        PipelineShaderStage::new(stage, self, entry_point)
    }
}
DerefContainerBracketImpl!(for ShaderModule {});
GuardsImpl!(for ShaderModule {});

#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineCacheCreateInfo<'d>(brvk::VkPipelineCacheCreateInfo, core::marker::PhantomData<&'d [u8]>);
impl<'d> PipelineCacheCreateInfo<'d> {
    pub const fn new(initial_data: &'d [u8]) -> Self {
        Self(
            brvk::VkPipelineCacheCreateInfo {
                sType: brvk::VkPipelineCacheCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                initialDataSize: initial_data.len() as _,
                pInitialData: slice_as_ptr_empty_null(initial_data) as _,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineCacheCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineCacheCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineCacheCreateInfo {
        self.0
    }
}

/// Opaque handle to a pipeline cache object
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VkPipelineCache::OBJECT_TYPE)]
pub struct PipelineCacheObject<Device: VkHandle<Handle = brvk::VkDevice>>(brvk::VkPipelineCache, Device);
#[implements]
impl<Device: VkHandle<Handle = brvk::VkDevice>> Drop for PipelineCacheObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn_wrapper::destroy_pipeline_cache(
                self.1.as_transparent_ref(),
                VkHandleRefMut::dangling(self.0),
                None,
            );
        }
    }
}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Sync> Sync for PipelineCacheObject<Device> {}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Send> Send for PipelineCacheObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceChildHandle for PipelineCacheObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice>> PipelineCache for PipelineCacheObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> PipelineCacheMut for PipelineCacheObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> PipelineCacheObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: brvk::VkPipelineCache, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (brvk::VkPipelineCache, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice> + Clone> PipelineCacheObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> PipelineCacheObject<Device> {
        let r = PipelineCacheObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}
impl<Device: crate::Device> PipelineCacheObject<Device> {
    /// Create a new pipeline cache
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    pub fn new(device: Device, create_info: &PipelineCacheCreateInfo) -> crate::Result<Self> {
        Ok(unsafe {
            Self::manage(
                crate::vkfn_wrapper::create_pipeline_cache(device.as_transparent_ref(), create_info, None)?,
                device,
            )
        })
    }
}

pub trait PipelineCache: VkHandle<Handle = brvk::VkPipelineCache> + DeviceChildHandle {
    /// Get the size of the data store from a pipeline cache
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn data_len(&self) -> crate::Result<usize> {
        unsafe {
            crate::vkfn_wrapper::get_pipeline_cache_data_byte_length(
                self.device_transparent_ref(),
                self.as_transparent_ref(),
            )
        }
    }

    /// Get the content of the data store from a pipeline cache
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn data_into(&self, store: &mut [core::mem::MaybeUninit<u8>]) -> crate::Result<ArrayQueryResult<usize>> {
        unsafe {
            crate::vkfn_wrapper::get_pipeline_cache_data(
                self.device_transparent_ref(),
                self.as_transparent_ref(),
                store,
            )
        }
    }
}
DerefContainerBracketImpl!(for PipelineCache {});
GuardsImpl!(for PipelineCache {});

pub trait PipelineCacheMut: PipelineCache + VkHandleMut {
    /// Combine the data stores of pipeline caches into `self`
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn merge(&mut self, srcs: &[VkHandleRef<brvk::VkPipelineCache>]) -> crate::Result<()> {
        translate_vk_result(unsafe {
            brvk::fns::merge_pipeline_caches(
                self.device_handle(),
                self.native_ptr_mut(),
                srcs.len() as _,
                slice_as_ptr_empty_null(srcs).cast(),
            )
        })?;
        Ok(())
    }
}
DerefContainerBracketImpl!(for mut PipelineCacheMut {});
GuardsImpl!(for mut PipelineCacheMut {});

#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineLayoutCreateInfo<'d>(
    brvk::VkPipelineLayoutCreateInfo,
    core::marker::PhantomData<(
        &'d dyn VkHandle<Handle = brvk::VkDescriptorSetLayout>,
        &'d [PushConstantRange],
    )>,
);
impl<'d> PipelineLayoutCreateInfo<'d> {
    pub const fn new(
        descriptor_set_layouts: &'d [VkHandleRef<'d, brvk::VkDescriptorSetLayout>],
        push_constant_ranges: &'d [PushConstantRange],
    ) -> Self {
        Self(
            brvk::VkPipelineLayoutCreateInfo {
                sType: brvk::VkPipelineLayoutCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                setLayoutCount: descriptor_set_layouts.len() as _,
                pSetLayouts: slice_as_ptr_empty_null(descriptor_set_layouts) as _,
                pushConstantRangeCount: push_constant_ranges.len() as _,
                pPushConstantRanges: slice_as_ptr_empty_null(push_constant_ranges).cast(),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineLayoutCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineLayoutCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineLayoutCreateInfo {
        self.0
    }
}

/// Opaque handle to a pipeline layout object
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VkPipelineLayout::OBJECT_TYPE)]
pub struct PipelineLayoutObject<Device: VkHandle<Handle = brvk::VkDevice>>(brvk::VkPipelineLayout, Device);
#[implements]
impl<Device: VkHandle<Handle = brvk::VkDevice>> Drop for PipelineLayoutObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn_wrapper::destroy_pipeline_layout(
                self.1.as_transparent_ref(),
                VkHandleRefMut::dangling(self.0),
                None,
            );
        }
    }
}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Sync> Sync for PipelineLayoutObject<Device> {}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Send> Send for PipelineLayoutObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceChildHandle for PipelineLayoutObject<Device> {
    #[inline]
    fn device_handle(&self) -> brvk::VkDevice {
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
impl<Device: VkHandle<Handle = brvk::VkDevice>> PipelineLayout for PipelineLayoutObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> PipelineLayoutObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: brvk::VkPipelineLayout, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (brvk::VkPipelineLayout, Device) {
        let r = unsafe { (self.0, core::ptr::read(&self.1)) };
        core::mem::forget(self);

        r
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice> + Clone> PipelineLayoutObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> PipelineLayoutObject<Device> {
        let r = PipelineLayoutObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}
impl<Device: crate::Device> PipelineLayoutObject<Device> {
    /// Creates a new pipeline layout object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn new(device: Device, info: &PipelineLayoutCreateInfo) -> crate::Result<Self> {
        Ok(unsafe { Self::manage(device.new_pipeline_layout_raw(info, None)?, device) })
    }
}

#[repr(transparent)]
pub struct PushConstantRange(pub brvk::VkPushConstantRange);
impl PushConstantRange {
    pub const fn new(shader_stage: brvk::VkShaderStageFlags, byte_range: Range<u32>) -> Self {
        Self(brvk::VkPushConstantRange {
            stageFlags: shader_stage,
            offset: byte_range.start,
            size: byte_range.end - byte_range.start,
        })
    }

    pub const fn for_type<T>(shader_stage: brvk::VkShaderStageFlags, offset: u32) -> Self {
        Self(brvk::VkPushConstantRange {
            stageFlags: shader_stage,
            offset,
            size: core::mem::size_of::<T>() as _,
        })
    }
}

pub trait PipelineLayout: VkHandle<Handle = brvk::VkPipelineLayout> {}
DerefContainerBracketImpl!(for PipelineLayout {});
GuardsImpl!(for PipelineLayout {});

/// Opaque handle to a pipeline object
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VkPipeline::OBJECT_TYPE)]
pub struct PipelineObject<Device: VkHandle<Handle = brvk::VkDevice>>(brvk::VkPipeline, Device);
#[implements]
impl<Device: VkHandle<Handle = brvk::VkDevice>> Drop for PipelineObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn_wrapper::destroy_pipeline(self.1.as_transparent_ref(), VkHandleRefMut::dangling(self.0), None);
        }
    }
}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Sync> Sync for PipelineObject<Device> {}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Send> Send for PipelineObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceChildHandle for PipelineObject<Device> {
    #[inline]
    fn device_handle(&self) -> brvk::VkDevice {
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
impl<Device: VkHandle<Handle = brvk::VkDevice>> Pipeline for PipelineObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> PipelineObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: brvk::VkPipeline, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (brvk::VkPipeline, Device) {
        let r = unsafe { (self.0, core::ptr::read(&self.1)) };
        core::mem::forget(self);

        r
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice> + Clone> PipelineObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> PipelineObject<Device> {
        let r = PipelineObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}

pub trait Pipeline: VkHandle<Handle = brvk::VkPipeline> {}
DerefContainerBracketImpl!(for Pipeline {});
GuardsImpl!(for Pipeline {});

/// Structure specifying parameters of a newly created pipeline dynamic state
#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineDynamicStateCreateInfo<'d>(
    brvk::VkPipelineDynamicStateCreateInfo,
    core::marker::PhantomData<&'d [brvk::VkDynamicState]>,
);
impl<'d> PipelineDynamicStateCreateInfo<'d> {
    pub const fn new(states: &'d [brvk::VkDynamicState]) -> Self {
        Self(
            brvk::VkPipelineDynamicStateCreateInfo {
                sType: brvk::VkPipelineDynamicStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                dynamicStateCount: states.len() as _,
                pDynamicStates: slice_as_ptr_empty_null(states),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineDynamicStateCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineDynamicStateCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineDynamicStateCreateInfo {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineShaderStage<'d, 's>(
    pub(crate) brvk::VkPipelineShaderStageCreateInfo,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        Option<&'d dyn brvk::VulkanStructure>,
        &'d dyn VkHandle<Handle = brvk::VkShaderModule>,
        &'d core::ffi::CStr,
        Option<&'s SpecializationInfo<'d>>,
    )>,
);
impl<'d, 's> PipelineShaderStage<'d, 's> {
    #[inline(always)]
    pub fn new(
        stage: ShaderStage,
        shader: &'d (impl VkHandle<Handle = brvk::VkShaderModule> + ?Sized),
        entrypoint_name: &'d core::ffi::CStr,
    ) -> Self {
        Self(
            brvk::VkPipelineShaderStageCreateInfo {
                sType: brvk::VkPipelineShaderStageCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                stage: stage as _,
                module: Some(shader.native_ptr()),
                pName: entrypoint_name.as_ptr(),
                pSpecializationInfo: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineShaderStageCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineShaderStageCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineShaderStageCreateInfo {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl brvk::TypedVulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }

    pub const fn with_specialization_info(mut self, info: &'s SpecializationInfo<'d>) -> Self {
        self.0.pSpecializationInfo = info as *const _ as _;
        self
    }
}

/// A trait for types that can be used as specialization constants.
///
/// # Safety
///
/// `ENTRIES` must be represent an content of this structs.
pub unsafe trait SpecializationConstants {
    const ENTRIES: &'static [SpecializationMapEntry];

    fn as_ptr(&self) -> *const c_void;
}
DerefContainerBracketImpl!(unsafe for SpecializationConstants {
    const ENTRIES: &'static [SpecializationMapEntry] = T::ENTRIES;

    #[inline(always)]
    fn as_ptr(&self) -> *const c_void {
        T::as_ptr(self)
    }
});

#[repr(transparent)]
pub struct SpecializationMapEntry(pub brvk::VkSpecializationMapEntry);
impl SpecializationMapEntry {
    pub const fn for_byte_range(constant_id: u32, byte_range: core::ops::Range<u32>) -> Self {
        Self(brvk::VkSpecializationMapEntry {
            constantID: constant_id,
            offset: byte_range.start,
            size: (byte_range.end - byte_range.start) as _,
        })
    }

    pub const fn for_type<T>(constant_id: u32, offset: u32) -> Self {
        Self(brvk::VkSpecializationMapEntry {
            constantID: constant_id,
            offset,
            size: core::mem::size_of::<T>(),
        })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct SpecializationInfo<'d>(
    brvk::VkSpecializationInfo,
    core::marker::PhantomData<(&'d [brvk::VkSpecializationMapEntry], &'d dyn core::any::Any)>,
);
impl<'d> SpecializationInfo<'d> {
    pub fn new<T: 'd + SpecializationConstants>(data: &'d T) -> Self {
        Self(
            brvk::VkSpecializationInfo {
                mapEntryCount: T::ENTRIES.len() as _,
                pMapEntries: slice_as_ptr_empty_null(T::ENTRIES).cast(),
                dataSize: core::mem::size_of::<T>() as _,
                pData: data.as_ptr(),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    /// `data` must have enough size.
    pub const unsafe fn from_any_type<T: 'd>(entries: &'d [SpecializationMapEntry], data: &'d T) -> Self {
        Self(
            brvk::VkSpecializationInfo {
                mapEntryCount: entries.len() as _,
                pMapEntries: slice_as_ptr_empty_null(entries).cast(),
                dataSize: core::mem::size_of::<T>() as _,
                pData: data as *const _ as _,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    /// `data` must have enough size.
    pub const unsafe fn from_binary(entries: &'d [SpecializationMapEntry], data: &'d [u8]) -> Self {
        Self(
            brvk::VkSpecializationInfo {
                mapEntryCount: entries.len() as _,
                pMapEntries: slice_as_ptr_empty_null(entries).cast(),
                dataSize: data.len() as _,
                pData: data.as_ptr() as _,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkSpecializationInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkSpecializationInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkSpecializationInfo {
        self.0
    }
}

#[repr(transparent)]
pub struct VertexInputBindingDescription(pub brvk::VkVertexInputBindingDescription);
impl VertexInputBindingDescription {
    /// Consumed per vertex with stride
    pub const fn per_vertex(binding: u32, stride: u32) -> Self {
        Self(brvk::VkVertexInputBindingDescription {
            binding,
            stride,
            inputRate: brvk::VK_VERTEX_INPUT_RATE_VERTEX,
        })
    }

    /// Consumed per instance with stride
    pub const fn per_instance(binding: u32, stride: u32) -> Self {
        Self(brvk::VkVertexInputBindingDescription {
            binding,
            stride,
            inputRate: brvk::VK_VERTEX_INPUT_RATE_INSTANCE,
        })
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

#[repr(transparent)]
pub struct VertexInputAttributeDescription(pub brvk::VkVertexInputAttributeDescription);

/// Structure specifying parameters of a newly created pipeline vertex input state
#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineVertexInputStateCreateInfo<'d>(
    brvk::VkPipelineVertexInputStateCreateInfo,
    core::marker::PhantomData<(
        &'d [VertexInputBindingDescription],
        &'d [VertexInputAttributeDescription],
    )>,
);
impl<'d> PipelineVertexInputStateCreateInfo<'d> {
    pub const fn new(
        bindings: &'d [VertexInputBindingDescription],
        attributes: &'d [VertexInputAttributeDescription],
    ) -> Self {
        Self(
            brvk::VkPipelineVertexInputStateCreateInfo {
                sType: brvk::VkPipelineVertexInputStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                vertexBindingDescriptionCount: bindings.len() as _,
                pVertexBindingDescriptions: slice_as_ptr_empty_null(bindings).cast(),
                vertexAttributeDescriptionCount: attributes.len() as _,
                pVertexAttributeDescriptions: slice_as_ptr_empty_null(attributes).cast(),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineVertexInputStateCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineVertexInputStateCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineVertexInputStateCreateInfo {
        self.0
    }
}

/// Supported primitive topologies
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveTopology {
    /// A series of [separate point primitives](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-point-lists)
    PointList = brvk::VK_PRIMITIVE_TOPOLOGY_POINT_LIST,
    /// A series of [separate line primitives](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-line-lists)
    LineList = brvk::VK_PRIMITIVE_TOPOLOGY_LINE_LIST,
    /// A series of [connected line primitives](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-line-strips)
    /// with consecutive lines sharing a vertex
    LineStrip = brvk::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP,
    /// A series of [separate triangle primitives](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-triangle-lists)
    TriangleList = brvk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
    /// A series of [connected triangle primitives](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-triangle-strips)
    /// with consecutive triangles sharing an edge
    TriangleStrip = brvk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
    /// A series of [connected triangle primitives](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-triangle-strips)
    /// with all triangles sharing a common vertex
    TriangleFan = brvk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
    /// A series of [separate line primitives with adjacency](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-line-lists-with-adjacency)
    LineListWithAdjacency = brvk::VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY,
    /// A series of [connected line primitives with adjacency](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-line-strips-with-adjacency)
    /// with consecutive primitives sharing three vertices
    LineStripWithAdjacency = brvk::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY,
    /// A series of [separate triangle primitives with adjacency](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-triangle-lists-with-adjacency)
    TriangleListWithAdjacency = brvk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY,
    /// [Connected triangle primitives with adjacency](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-triangle-strips-with-adjacency)
    /// with consecutive triangles sharing an edge
    TriangleStripWithAdjacency = brvk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY,
    /// [Separate patch primitives](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#drawing-patch-lists)
    PatchList = brvk::VK_PRIMITIVE_TOPOLOGY_PATCH_LIST,
}

/// Structure specifying parameters of a newly created pipeline input assembly state
#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineInputAssemblyStateCreateInfo(brvk::VkPipelineInputAssemblyStateCreateInfo);
impl PipelineInputAssemblyStateCreateInfo {
    pub const fn new(topology: PrimitiveTopology) -> Self {
        Self(brvk::VkPipelineInputAssemblyStateCreateInfo {
            sType: brvk::VkPipelineInputAssemblyStateCreateInfo::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            topology: topology as _,
            primitiveRestartEnable: false as _,
        })
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineInputAssemblyStateCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineInputAssemblyStateCreateInfo) -> Self {
        Self(raw)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineInputAssemblyStateCreateInfo {
        self.0
    }

    pub const fn enable_primitive_restart(mut self) -> Self {
        self.0.primitiveRestartEnable = true as _;
        self
    }
}

/// Structure specifying parameters of a newly created pipeline tessellation state
#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineTessellationStateCreateInfo(brvk::VkPipelineTessellationStateCreateInfo);
impl PipelineTessellationStateCreateInfo {
    pub const fn new(patch_control_points: u32) -> Self {
        Self(brvk::VkPipelineTessellationStateCreateInfo {
            sType: brvk::VkPipelineTessellationStateCreateInfo::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            patchControlPoints: patch_control_points,
        })
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineTessellationStateCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineTessellationStateCreateInfo) -> Self {
        Self(raw)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineTessellationStateCreateInfo {
        self.0
    }
}

/// Structure specifying parameters of a newly created pipeline viewport state
#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineViewportStateCreateInfo<'d>(
    brvk::VkPipelineViewportStateCreateInfo,
    core::marker::PhantomData<(&'d [brvk::VkViewport], &'d [brvk::VkRect2D])>,
);
impl<'d> PipelineViewportStateCreateInfo<'d> {
    /// # Safety
    ///
    /// `viewports` and `scissors` must have same length.
    pub const unsafe fn new_unchecked(viewports: &'d [brvk::VkViewport], scissors: &'d [brvk::VkRect2D]) -> Self {
        Self(
            brvk::VkPipelineViewportStateCreateInfo {
                sType: brvk::VkPipelineViewportStateCreateInfo::TYPE,
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

    #[inline]
    pub fn new(viewports: &'d [brvk::VkViewport], scissors: &'d [brvk::VkRect2D]) -> Self {
        assert_eq!(viewports.len(), scissors.len());

        unsafe { Self::new_unchecked(viewports, scissors) }
    }

    pub const fn new_array<const N: usize>(
        viewports: &'d [brvk::VkViewport; N],
        scissors: &'d [brvk::VkRect2D; N],
    ) -> Self {
        // checked that both length are identitcal by const generic parameter
        unsafe { Self::new_unchecked(viewports, scissors) }
    }

    pub const fn new_dynamic(count: u32) -> Self {
        Self(
            brvk::VkPipelineViewportStateCreateInfo {
                sType: brvk::VkPipelineViewportStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                viewportCount: count,
                pViewports: core::ptr::null(),
                scissorCount: count,
                pScissors: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    pub const fn new_dynamic_with_count() -> Self {
        Self::new_dynamic(0)
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineViewportStateCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineViewportStateCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineViewportStateCreateInfo {
        self.0
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolygonMode {
    Fill = brvk::VK_POLYGON_MODE_FILL,
    Line = brvk::VK_POLYGON_MODE_LINE,
    Point = brvk::VK_POLYGON_MODE_POINT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[bitflags_newtype]
pub struct CullModeFlags(brvk::VkCullModeFlags);
impl CullModeFlags {
    pub const NONE: Self = Self(brvk::VK_CULL_MODE_NONE);
    pub const FRONT: Self = Self(brvk::VK_CULL_MODE_FRONT_BIT);
    pub const BACK: Self = Self(brvk::VK_CULL_MODE_BACK_BIT);
    pub const FRONT_AND_BACK: Self = Self(brvk::VK_CULL_MODE_FRONT_AND_BACK);
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrontFace {
    Clockwise = brvk::VK_FRONT_FACE_CLOCKWISE,
    CounterClockwise = brvk::VK_FRONT_FACE_COUNTER_CLOCKWISE,
}

/// Structure specifying parameters of a newly created pipeline rasterization state
#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineRasterizationStateCreateInfo<'d>(
    brvk::VkPipelineRasterizationStateCreateInfo,
    core::marker::PhantomData<Option<&'d dyn brvk::VulkanStructure>>,
);
impl<'d> PipelineRasterizationStateCreateInfo<'d> {
    pub const fn new(polygon_mode: PolygonMode, cull_mode: CullModeFlags, front_face: FrontFace) -> Self {
        Self(
            brvk::VkPipelineRasterizationStateCreateInfo {
                sType: brvk::VkPipelineRasterizationStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                depthClampEnable: false as _,
                rasterizerDiscardEnable: false as _,
                polygonMode: polygon_mode as _,
                cullMode: cull_mode.bits(),
                frontFace: front_face as _,
                depthBiasEnable: false as _,
                depthBiasConstantFactor: 0.0,
                depthBiasClamp: 0.0,
                depthBiasSlopeFactor: 0.0,
                lineWidth: 1.0,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineRasterizationStateCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineRasterizationStateCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineRasterizationStateCreateInfo {
        self.0
    }

    #[inline]
    pub fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    pub const fn enable_depth_clamp(mut self) -> Self {
        self.0.depthClampEnable = true as _;
        self
    }

    pub const fn enable_rasterizer_discard(mut self) -> Self {
        self.0.rasterizerDiscardEnable = true as _;
        self
    }

    pub const fn enable_depth_bias(mut self, constant_factor: f32, clamp: f32, slope_factor: f32) -> Self {
        self.0.depthBiasEnable = true as _;
        self.0.depthBiasConstantFactor = constant_factor;
        self.0.depthBiasClamp = clamp;
        self.0.depthBiasSlopeFactor = slope_factor;
        self
    }

    pub const fn line_width(mut self, w: f32) -> Self {
        self.0.lineWidth = w;
        self
    }
}

/// Specify the conservative rasterization mode
#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConservativeRasterizationMode {
    /// Conservative rasterization is disabled and rasterization proceeds as normal
    Disabled = brvk::VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT,
    /// Conservative rasterization is enabled in overestimation mode
    Overestimate = brvk::VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT,
    /// Conservative rasterization is enabled in underestimation mode
    Underestimate = brvk::VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT,
}

/// Structure specifying conservative raster state
#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PipelineRasterizationConservativeStateCreateInfo<'d>(
    brvk::VkPipelineRasterizationConservativeStateCreateInfoEXT,
    core::marker::PhantomData<Option<&'d dyn brvk::VulkanStructure>>,
);
#[cfg(feature = "VK_EXT_conservative_rasterization")]
unsafe impl brvk::VulkanStructure for PipelineRasterizationConservativeStateCreateInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl<'d> PipelineRasterizationConservativeStateCreateInfo<'d> {
    pub const fn new(mode: ConservativeRasterizationMode, extra_primitive_overestimation_size: f32) -> Self {
        Self(
            brvk::VkPipelineRasterizationConservativeStateCreateInfoEXT {
                sType: brvk::VkPipelineRasterizationConservativeStateCreateInfoEXT::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                conservativeRasterizationMode: mode as _,
                extraPrimitiveOverestimationSize: extra_primitive_overestimation_size,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: brvk::VkPipelineRasterizationConservativeStateCreateInfoEXT) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineRasterizationConservativeStateCreateInfoEXT {
        self.0
    }

    #[inline]
    pub fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }
}

/// Line rasterization modes
#[cfg(feature = "VK_KHR_line_rasterization")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineRasterizationMode {
    /// Equivalent to [`LineRasterizationMode::Rectangular`] if [`brvk::VkPhysicalDeviceLimits::strictLines`] is true,
    /// otherwise lines are drawn as non-`strictLines` parallelograms.
    /// Both of these modes are defined in [Basic Line Segment Rasterization](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#primsrast-lines-basic)
    Default = brvk::VK_LINE_RASTERIZATION_MODE_DEFAULT_KHR,
    /// Lines drawn as if they were rectangles extruded from the line
    Rectangular = brvk::VK_LINE_RASTERIZATION_MODE_RECTANGULAR_KHR,
    /// Lines drawn by determining which pixel diamonds the line intersects and exits,
    /// as defined in [Bresenham Line Segment Rasterization](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#primsrast-lines-bresenham)
    Bresenham = brvk::VK_LINE_RASTERIZATION_MODE_BRESENHAM_KHR,
    /// Lines drawn if they were rectangles extruded from the line, with alpha falloff,
    /// as defined in [Smooth Lines](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#primsrast-lines-smooth)
    RectangularSmooth = brvk::VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_KHR,
}

/// Structure specifying parameters of a newly created pipeline line rasterization state
#[cfg(feature = "VK_KHR_line_rasterization")]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PipelineRasterizationLineStateCreateInfo<'d>(
    brvk::VkPipelineRasterizationLineStateCreateInfoKHR,
    core::marker::PhantomData<Option<&'d dyn brvk::VulkanStructure>>,
);
#[cfg(feature = "VK_KHR_line_rasterization")]
unsafe impl brvk::VulkanStructure for PipelineRasterizationLineStateCreateInfo<'_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "VK_KHR_line_rasterization")]
impl<'d> PipelineRasterizationLineStateCreateInfo<'d> {
    pub const fn new(mode: LineRasterizationMode) -> Self {
        Self(
            brvk::VkPipelineRasterizationLineStateCreateInfoKHR {
                sType: brvk::VkPipelineRasterizationLineStateCreateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                lineRasterizationMode: mode as _,
                stippledLineEnable: false as _,
                lineStippleFactor: 0,
                lineStipplePattern: 0,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineRasterizationLineStateCreateInfoKHR`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineRasterizationLineStateCreateInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineRasterizationLineStateCreateInfoKHR {
        self.0
    }

    #[inline]
    pub fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    pub const fn stippled(mut self, factor: u32, pattern: u16) -> Self {
        self.0.stippledLineEnable = true as _;
        self.0.lineStippleFactor = factor;
        self.0.lineStipplePattern = pattern;
        self
    }
}

/// Structure specifying parameters of a newly created pipeline multisample state
#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineMultisampleStateCreateInfo<'d> {
    data: brvk::VkPipelineMultisampleStateCreateInfo,
    samplemask_lifetime_binder: PhantomData<&'d [brvk::VkSampleMask]>,
}
impl<'d> PipelineMultisampleStateCreateInfo<'d> {
    pub const fn new() -> Self {
        Self {
            data: brvk::VkPipelineMultisampleStateCreateInfo {
                sType: brvk::VkPipelineMultisampleStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                rasterizationSamples: 1,
                sampleShadingEnable: brvk::VK_FALSE,
                minSampleShading: 1.0,
                pSampleMask: core::ptr::null(),
                alphaToCoverageEnable: brvk::VK_FALSE,
                alphaToOneEnable: brvk::VK_FALSE,
            },
            samplemask_lifetime_binder: PhantomData,
        }
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineMultisampleStateCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineMultisampleStateCreateInfo) -> Self {
        Self {
            data: raw,
            samplemask_lifetime_binder: core::marker::PhantomData,
        }
    }

    pub const fn into_raw(self) -> brvk::VkPipelineMultisampleStateCreateInfo {
        self.data
    }

    /// Specifies the number of samples per pixel used in rasterization. default=1
    pub const fn rasterization_samples(mut self, samples: usize) -> Self {
        self.data.rasterizationSamples = samples as _;
        self
    }

    /// A bitmask of static coverage information that is ANDed with the coverage information generated
    /// during rasterization, as described in [Sample Mask](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#fragops-samplemask).
    pub fn sample_mask(mut self, mask: &'d [brvk::VkSampleMask]) -> Self {
        if mask.is_empty() {
            self.data.pSampleMask = core::ptr::null();
            return self;
        }

        assert_eq!(mask.len(), self.data.rasterizationSamples.div_ceil(32) as _);
        self.data.pSampleMask = mask.as_ptr();
        self
    }

    /// Specifies a minimum fraction of sample shading(must be in the range [0, 1]).
    /// Pass a `None` to disable [Sample Shading](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#primsrast-sampleshading).
    pub const fn sample_shading(mut self, min: f32) -> Self {
        assert!(
            0.0 <= min && min <= 1.0,
            "Invalid usage: brvk::VkPipelineMultisampleStateCreateInfo::minSampleShading must be in the range [0, 1]"
        );

        self.data.sampleShadingEnable = true as _;
        self.data.minSampleShading = min;
        self
    }

    /// Controls whether a temporary coverage value is generated based on the alpha component of the fragment's
    /// first color output as specified in the [Multisample Coverage](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#fragops-covg) section.
    pub const fn enable_alpha_to_coverage(mut self) -> Self {
        self.data.alphaToCoverageEnable = true as _;
        self
    }

    /// Controls whether the alpha component of the fragment's first color output is replaced with one as described in
    /// [Multisample Coverage](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#fragops-covg).
    pub const fn replace_alpha_to_one(mut self) -> Self {
        self.data.alphaToOneEnable = true as _;
        self
    }
}

/// Structure specifying parameters of a newly created pipeline depth stencil state
#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineDepthStencilStateCreateInfo(brvk::VkPipelineDepthStencilStateCreateInfo);
impl PipelineDepthStencilStateCreateInfo {
    pub const fn new() -> Self {
        Self(brvk::VkPipelineDepthStencilStateCreateInfo {
            sType: brvk::VkPipelineDepthStencilStateCreateInfo::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            depthTestEnable: brvk::VK_FALSE,
            depthWriteEnable: brvk::VK_FALSE,
            depthCompareOp: brvk::VK_COMPARE_OP_LESS,
            depthBoundsTestEnable: brvk::VK_FALSE,
            stencilTestEnable: brvk::VK_FALSE,
            front: brvk::VkStencilOpState {
                failOp: brvk::VK_STENCIL_OP_KEEP,
                passOp: brvk::VK_STENCIL_OP_KEEP,
                depthFailOp: brvk::VK_STENCIL_OP_KEEP,
                compareOp: brvk::VK_COMPARE_OP_ALWAYS,
                compareMask: 0,
                writeMask: 0,
                reference: 0,
            },
            back: brvk::VkStencilOpState {
                failOp: brvk::VK_STENCIL_OP_KEEP,
                passOp: brvk::VK_STENCIL_OP_KEEP,
                depthFailOp: brvk::VK_STENCIL_OP_KEEP,
                compareOp: brvk::VK_COMPARE_OP_ALWAYS,
                compareMask: 0,
                writeMask: 0,
                reference: 0,
            },
            minDepthBounds: 0.0,
            maxDepthBounds: 1.0,
        })
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineDepthStencilStateCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineDepthStencilStateCreateInfo) -> Self {
        Self(raw)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineDepthStencilStateCreateInfo {
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
    pub const fn stencil_state_front(mut self, state: StencilOpState) -> Self {
        self.0.front = state.0;
        self
    }

    /// Sets the state parameters for the back-face stencil test
    pub const fn stencil_state_back(mut self, state: StencilOpState) -> Self {
        self.0.back = state.0;
        self
    }
}

/// Structure specifying parameters of a newly created pipeline color blend state
#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineColorBlendStateCreateInfo<'d>(
    brvk::VkPipelineColorBlendStateCreateInfo,
    core::marker::PhantomData<&'d [PipelineColorBlendAttachmentState]>,
);
impl<'d> PipelineColorBlendStateCreateInfo<'d> {
    pub const fn new(attachments: &'d [PipelineColorBlendAttachmentState]) -> Self {
        Self(
            brvk::VkPipelineColorBlendStateCreateInfo {
                sType: brvk::VkPipelineColorBlendStateCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                logicOpEnable: false as _,
                logicOp: LogicOp::NoOp as _,
                attachmentCount: attachments.len() as _,
                pAttachments: slice_as_ptr_empty_null(attachments).cast(),
                blendConstants: [0.0; 4],
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`brvk::VkPipelineColorBlendStateCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkPipelineColorBlendStateCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkPipelineColorBlendStateCreateInfo {
        self.0
    }

    pub const fn logic_op(mut self, logic_op: LogicOp) -> Self {
        self.0.logicOpEnable = true as _;
        self.0.logicOp = logic_op as _;
        self
    }

    pub const fn blend_constants(mut self, constants: [f32; 4]) -> Self {
        self.0.blendConstants = constants;
        self
    }
}

#[repr(transparent)]
pub struct PipelineColorBlendAttachmentState(pub brvk::VkPipelineColorBlendAttachmentState);
impl PipelineColorBlendAttachmentState {
    pub const NOBLEND: Self = Self(brvk::VkPipelineColorBlendAttachmentState {
        colorWriteMask: brvk::VK_COLOR_COMPONENT_A_BIT
            | brvk::VK_COLOR_COMPONENT_R_BIT
            | brvk::VK_COLOR_COMPONENT_G_BIT
            | brvk::VK_COLOR_COMPONENT_B_BIT,
        blendEnable: brvk::VK_FALSE,
        ..unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    });

    /// `src * 1 + dst * (1 - src.a)` for both color and alpha.
    ///
    /// https://stackoverflow.com/questions/18918643/how-to-achieve-d3d-output-with-premultiplied-alpha-for-use-with-d3dimage-in-wpf
    pub const PREMULTIPLIED: Self = Self(brvk::VkPipelineColorBlendAttachmentState {
        colorWriteMask: brvk::VK_COLOR_COMPONENT_A_BIT
            | brvk::VK_COLOR_COMPONENT_R_BIT
            | brvk::VK_COLOR_COMPONENT_G_BIT
            | brvk::VK_COLOR_COMPONENT_B_BIT,
        blendEnable: brvk::VK_TRUE,
        srcColorBlendFactor: brvk::VK_BLEND_FACTOR_ONE,
        dstColorBlendFactor: brvk::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
        colorBlendOp: brvk::VK_BLEND_OP_ADD,
        // srcAlphaBlendFactor: BlendFactor::OneMinusDestAlpha as _,
        // dstAlphaBlendFactor: BlendFactor::One as _,
        srcAlphaBlendFactor: brvk::VK_BLEND_FACTOR_ONE,
        dstAlphaBlendFactor: brvk::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
        alphaBlendOp: brvk::VK_BLEND_OP_ADD,
    });

    pub const fn enabled(mut self) -> Self {
        self.0.blendEnable = brvk::VK_TRUE;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.0.blendEnable = brvk::VK_FALSE;
        self
    }

    pub const fn color_blend_factor_src(mut self, f: brvk::VkBlendFactor) -> Self {
        self.0.srcColorBlendFactor = f;
        self
    }

    pub const fn color_blend_factor_dst(mut self, f: brvk::VkBlendFactor) -> Self {
        self.0.dstColorBlendFactor = f;
        self
    }

    pub const fn alpha_blend_factor_src(mut self, f: brvk::VkBlendFactor) -> Self {
        self.0.srcAlphaBlendFactor = f;
        self
    }

    pub const fn alpha_blend_factor_dst(mut self, f: brvk::VkBlendFactor) -> Self {
        self.0.dstAlphaBlendFactor = f;
        self
    }

    pub const fn color_blend_op(mut self, op: brvk::VkBlendOp) -> Self {
        self.0.colorBlendOp = op;
        self
    }

    pub const fn alpha_blend_op(mut self, op: brvk::VkBlendOp) -> Self {
        self.0.alphaBlendOp = op;
        self
    }

    pub const fn color_blend(
        mut self,
        src: brvk::VkBlendFactor,
        op: brvk::VkBlendOp,
        dst: brvk::VkBlendFactor,
    ) -> Self {
        self.0.srcColorBlendFactor = src;
        self.0.dstColorBlendFactor = dst;
        self.0.colorBlendOp = op;
        self
    }

    pub const fn alpha_blend(
        mut self,
        src: brvk::VkBlendFactor,
        op: brvk::VkBlendOp,
        dst: brvk::VkBlendFactor,
    ) -> Self {
        self.0.srcAlphaBlendFactor = src;
        self.0.dstAlphaBlendFactor = dst;
        self.0.alphaBlendOp = op;
        self
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct GraphicsPipelineCreateInfo<'d>(
    brvk::VkGraphicsPipelineCreateInfo,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        &'d dyn VkHandle<Handle = brvk::VkPipelineLayout>,
        &'d dyn VkHandle<Handle = brvk::VkRenderPass>,
        &'d [PipelineShaderStage<'d, 'd>],
        &'d PipelineVertexInputStateCreateInfo<'d>,
        &'d PipelineInputAssemblyStateCreateInfo,
        Option<&'d PipelineTessellationStateCreateInfo>,
        &'d PipelineViewportStateCreateInfo<'d>,
        &'d PipelineRasterizationStateCreateInfo<'d>,
        Option<&'d PipelineMultisampleStateCreateInfo<'d>>,
        Option<&'d PipelineDepthStencilStateCreateInfo>,
        &'d PipelineColorBlendStateCreateInfo<'d>,
        Option<&'d PipelineDynamicStateCreateInfo<'d>>,
        Option<&'d dyn VkHandle<Handle = brvk::VkPipeline>>,
    )>,
);
impl<'d> GraphicsPipelineCreateInfo<'d> {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        layout: &'d (impl VkHandle<Handle = brvk::VkPipelineLayout> + ?Sized),
        pass: SubpassRef<'d, impl VkHandle<Handle = brvk::VkRenderPass> + ?Sized>,
        stages: &'d [PipelineShaderStage<'d, 'd>],
        vertex_input_state: &'d PipelineVertexInputStateCreateInfo<'d>,
        input_assembly_state: &'d PipelineInputAssemblyStateCreateInfo,
        viewport_state: &'d PipelineViewportStateCreateInfo<'d>,
        rasterization_state: &'d PipelineRasterizationStateCreateInfo<'d>,
        color_blend_state: &'d PipelineColorBlendStateCreateInfo<'d>,
    ) -> Self {
        Self(
            brvk::VkGraphicsPipelineCreateInfo {
                sType: brvk::VkGraphicsPipelineCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                stageCount: stages.len() as _,
                pStages: slice_as_ptr_empty_null(stages) as _,
                pVertexInputState: vertex_input_state as *const _ as _,
                pInputAssemblyState: input_assembly_state as *const _ as _,
                pTessellationState: core::ptr::null(),
                pViewportState: viewport_state as *const _ as _,
                pRasterizationState: rasterization_state as *const _ as _,
                pMultisampleState: core::ptr::null(),
                pDepthStencilState: core::ptr::null(),
                pColorBlendState: color_blend_state as *const _ as _,
                pDynamicState: core::ptr::null(),
                layout: Some(layout.native_ptr()),
                renderPass: Some(pass.0.native_ptr()),
                subpass: pass.1,
                basePipelineHandle: None,
                basePipelineIndex: -1,
            },
            core::marker::PhantomData,
        )
    }

    #[inline]
    pub fn derived_by_handle(parent: &'d (impl VkHandle<Handle = brvk::VkPipeline> + ?Sized)) -> Self {
        Self(
            brvk::VkGraphicsPipelineCreateInfo {
                sType: brvk::VkGraphicsPipelineCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: brvk::VK_PIPELINE_CREATE_DERIVATIVE_BIT,
                stageCount: 0,
                pStages: core::ptr::null(),
                pVertexInputState: core::ptr::null(),
                pInputAssemblyState: core::ptr::null(),
                pTessellationState: core::ptr::null(),
                pViewportState: core::ptr::null(),
                pRasterizationState: core::ptr::null(),
                pMultisampleState: core::ptr::null(),
                pDepthStencilState: core::ptr::null(),
                pColorBlendState: core::ptr::null(),
                pDynamicState: core::ptr::null(),
                layout: None,
                renderPass: None,
                subpass: 0,
                basePipelineHandle: Some(parent.native_ptr()),
                basePipelineIndex: -1,
            },
            core::marker::PhantomData,
        )
    }

    pub const fn derived_by_index(parent: i32) -> Self {
        Self(
            brvk::VkGraphicsPipelineCreateInfo {
                sType: brvk::VkGraphicsPipelineCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: brvk::VK_PIPELINE_CREATE_DERIVATIVE_BIT,
                stageCount: 0,
                pStages: core::ptr::null(),
                pVertexInputState: core::ptr::null(),
                pInputAssemblyState: core::ptr::null(),
                pTessellationState: core::ptr::null(),
                pViewportState: core::ptr::null(),
                pRasterizationState: core::ptr::null(),
                pMultisampleState: core::ptr::null(),
                pDepthStencilState: core::ptr::null(),
                pColorBlendState: core::ptr::null(),
                pDynamicState: core::ptr::null(),
                layout: None,
                renderPass: None,
                subpass: 0,
                basePipelineHandle: None,
                basePipelineIndex: parent,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`VkGraphicsPipelineCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkGraphicsPipelineCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkGraphicsPipelineCreateInfo {
        self.0
    }

    pub const fn allow_derivatives(mut self) -> Self {
        self.0.flags |= brvk::VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT;
        self
    }

    #[inline(always)]
    pub fn set_layout(mut self, layout: &'d (impl VkHandle<Handle = brvk::VkPipelineLayout> + ?Sized)) -> Self {
        self.0.layout = Some(layout.native_ptr());
        self
    }

    #[inline(always)]
    pub fn set_pass(mut self, pass: SubpassRef<'d, impl VkHandle<Handle = brvk::VkRenderPass> + ?Sized>) -> Self {
        self.0.renderPass = Some(pass.0.native_ptr());
        self.0.subpass = pass.1;
        self
    }

    pub const fn set_stages(mut self, stages: &'d [PipelineShaderStage<'d, 'd>]) -> Self {
        self.0.pStages = slice_as_ptr_empty_null(stages) as _;
        self
    }

    pub const fn set_vertex_input_state(mut self, state: &'d PipelineVertexInputStateCreateInfo<'d>) -> Self {
        self.0.pVertexInputState = state as *const _ as _;
        self
    }

    pub const fn set_input_assembly_state(mut self, state: &'d PipelineInputAssemblyStateCreateInfo) -> Self {
        self.0.pInputAssemblyState = state as *const _ as _;
        self
    }

    pub const fn set_tessellation_state(mut self, state: &'d PipelineTessellationStateCreateInfo) -> Self {
        self.0.pTessellationState = state as *const _ as _;
        self
    }

    pub const fn set_viewport_state(mut self, state: &'d PipelineViewportStateCreateInfo<'d>) -> Self {
        self.0.pViewportState = state as *const _ as _;
        self
    }

    pub const fn set_rasterization_state(mut self, state: &'d PipelineRasterizationStateCreateInfo<'d>) -> Self {
        self.0.pRasterizationState = state as *const _ as _;
        self
    }

    pub const fn set_multisample_state(mut self, state: &'d PipelineMultisampleStateCreateInfo<'d>) -> Self {
        self.0.pMultisampleState = state as *const _ as _;
        self
    }

    pub const fn set_depth_stencil_state(mut self, state: &'d PipelineDepthStencilStateCreateInfo) -> Self {
        self.0.pDepthStencilState = state as *const _ as _;
        self
    }

    pub const fn set_color_blend_state(mut self, state: &'d PipelineColorBlendStateCreateInfo<'d>) -> Self {
        self.0.pColorBlendState = state as *const _ as _;
        self
    }

    pub const fn set_dynamic_state(mut self, states: &'d PipelineDynamicStateCreateInfo) -> Self {
        self.0.pDynamicState = states as *const _ as _;
        self
    }
}

/// Structure specifying parameters of a newly created compute pipeline
#[repr(transparent)]
#[derive(Clone)]
pub struct ComputePipelineCreateInfo<'d>(
    brvk::VkComputePipelineCreateInfo,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        &'d dyn VkHandle<Handle = brvk::VkPipelineLayout>,
        PipelineShaderStage<'d, 'd>,
        Option<&'d dyn VkHandle<Handle = brvk::VkPipeline>>,
    )>,
);
impl<'d> ComputePipelineCreateInfo<'d> {
    #[inline]
    pub fn new(
        layout: &'d (impl VkHandle<Handle = brvk::VkPipelineLayout> + ?Sized),
        stage: PipelineShaderStage<'d, 'd>,
    ) -> Self {
        Self(
            brvk::VkComputePipelineCreateInfo {
                sType: brvk::VkComputePipelineCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                stage: stage.0,
                layout: Some(layout.native_ptr()),
                basePipelineHandle: None,
                basePipelineIndex: -1,
            },
            core::marker::PhantomData,
        )
    }

    #[inline]
    pub fn derived_by_handle(parent: &'d (impl VkHandle<Handle = brvk::VkPipeline> + ?Sized)) -> Self {
        Self(
            brvk::VkComputePipelineCreateInfo {
                sType: brvk::VkComputePipelineCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: brvk::VK_PIPELINE_CREATE_DERIVATIVE_BIT,
                stage: brvk::VkPipelineShaderStageCreateInfo {
                    sType: brvk::VkPipelineShaderStageCreateInfo::TYPE,
                    pNext: core::ptr::null(),
                    flags: 0,
                    stage: 0,
                    module: None,
                    pName: core::ptr::null(),
                    pSpecializationInfo: core::ptr::null(),
                },
                layout: None,
                basePipelineHandle: Some(parent.native_ptr()),
                basePipelineIndex: -1,
            },
            core::marker::PhantomData,
        )
    }

    #[inline]
    pub fn derived_by_index(parent: i32) -> Self {
        Self(
            brvk::VkComputePipelineCreateInfo {
                sType: brvk::VkComputePipelineCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: brvk::VK_PIPELINE_CREATE_DERIVATIVE_BIT,
                stage: brvk::VkPipelineShaderStageCreateInfo {
                    sType: brvk::VkPipelineShaderStageCreateInfo::TYPE,
                    pNext: core::ptr::null(),
                    flags: 0,
                    stage: 0,
                    module: None,
                    pName: core::ptr::null(),
                    pSpecializationInfo: core::ptr::null(),
                },
                layout: None,
                basePipelineHandle: None,
                basePipelineIndex: parent,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must have a valid [`VkComputePipelineCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkComputePipelineCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkComputePipelineCreateInfo {
        self.0
    }

    pub const fn allow_derivatives(mut self) -> Self {
        self.0.flags |= brvk::VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT;
        self
    }

    pub const fn set_stage(mut self, stage: PipelineShaderStage<'d, 'd>) -> Self {
        self.0.stage = stage.0;
        self
    }

    #[inline]
    pub fn set_layout(mut self, layout: &'d (impl VkHandle<Handle = brvk::VkPipelineLayout> + ?Sized)) -> Self {
        self.0.layout = Some(layout.native_ptr());
        self
    }
}

/// Bitmask specifying pipeline stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[bitflags_newtype]
pub struct PipelineStageFlags(pub brvk::VkPipelineStageFlags);
impl PipelineStageFlags {
    /// The stage of the pipeline where any commands are initially received by the queue
    pub const TOP_OF_PIPE: Self = Self(brvk::VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT);
    /// The stage of the pipeline where Draw/DispatchIndirect data structures are consumed
    pub const DRAW_INDIRECT: Self = Self(brvk::VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT);
    /// The stage of the pipeline where vertex and index buffers are consumed
    pub const VERTEX_INPUT: Self = Self(brvk::VK_PIPELINE_STAGE_VERTEX_INPUT_BIT);
    /// The vertex shader stage
    pub const VERTEX_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_VERTEX_SHADER_BIT);
    /// The tessellation control shader stage
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT);
    /// The tessellation evaluation shader stage
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT);
    /// The geometry shader stage
    pub const GEOMETRY_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT);
    /// The fragment shader stage
    pub const FRAGMENT_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
    /// The stage of the pipeline where early fragment tests (depth and stencil tests before fragment shading) are performed
    pub const EARLY_FRAGMENT_TESTS: Self = Self(brvk::VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT);
    /// The stage of the pipeline where late fragment tests (depth and stencil tests after fragment shading) are performed
    pub const LATE_FRAGMENT_TESTS: Self = Self(brvk::VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT);
    /// The stage of the pipeline after blending where the final color values are output from the pipeline
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(brvk::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT);
    /// The execution of copy commands
    pub const TRANSFER: Self = Self(brvk::VK_PIPELINE_STAGE_TRANSFER_BIT);
    /// The execution of a compute shader
    pub const COMPUTE_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT);
    /// The final stage in the pipeline where operations generated by all commands complete execution
    pub const BOTTOM_OF_PIPE: Self = Self(brvk::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT);
    /// A pseudo-stage indicating execution on the host of reads/writes of device memory
    pub const HOST: Self = Self(brvk::VK_PIPELINE_STAGE_HOST_BIT);
    /// The execution of all graphics pipeline stages
    pub const ALL_GRAPHICS: Self = Self(brvk::VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT);
    /// Equivalent to the logical OR of every other pipeline stage flag that is supported on the quue it is used with
    pub const ALL_COMMANDS: Self = Self(brvk::VK_PIPELINE_STAGE_ALL_COMMANDS_BIT);
}

/// Bitmask specifying pipeline stages (extended)
#[cfg(feature = "VK_KHR_synchronization2")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[bitflags_newtype]
pub struct PipelineStageFlags2(pub brvk::VkPipelineStageFlags2KHR);
#[cfg(feature = "VK_KHR_synchronization2")]
impl PipelineStageFlags2 {
    pub const TOP_OF_PIPE: Self = Self(brvk::VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR);
    pub const DRAW_INDIRECT: Self = Self(brvk::VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR);
    pub const VERTEX_INPUT: Self = Self(brvk::VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR);
    pub const VERTEX_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR);
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR);
    pub const TESSELLATION_EVALUATION_SHADER: Self =
        Self(brvk::VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR);
    pub const GEOMETRY_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR);
    pub const FRAGMENT_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(brvk::VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR);
    pub const LATE_FRAGMENT_TESTS: Self = Self(brvk::VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR);
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(brvk::VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR);
    pub const COMPUTE_SHADER: Self = Self(brvk::VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR);
    pub const ALL_TRANSFER: Self = Self(brvk::VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR);
    pub const BOTTOM_OF_PIPE: Self = Self(brvk::VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR);
    pub const HOST: Self = Self(brvk::VK_PIPELINE_STAGE_2_HOST_BIT_KHR);
    pub const ALL_GRAPHICS: Self = Self(brvk::VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR);
    pub const ALL_COMMANDS: Self = Self(brvk::VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR);
    pub const COPY: Self = Self(brvk::VK_PIPELINE_STAGE_2_COPY_BIT_KHR);
    pub const RESOLVE: Self = Self(brvk::VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR);
    pub const BLIT: Self = Self(brvk::VK_PIPELINE_STAGE_2_BLIT_BIT_KHR);
    pub const CLEAR: Self = Self(brvk::VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR);
    pub const INDEX_INPUT: Self = Self(brvk::VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR);
    pub const VERTEX_ATTRIBUTE_INPUT: Self = Self(brvk::VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR);
    pub const PRE_RASTERIZATION_SHADERS: Self = Self(brvk::VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR);
}
