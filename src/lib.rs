//! Glue library between Vulkan and Rust
//!
//! # Compile Options
//! - `FeImplements`: Enable Vulkan implementations(functions)
//! - `FeMultithreaded`: Enables to use objects from some threads(experimental)
//! - `VK_***`: Enable Vulkan extensions(same name as each extensions)

extern crate libc;
// Platform Extras
#[cfg(any(
    feature = "VK_KHR_win32_surface", feature = "VK_KHR_external_memory_win32",
    feature = "VK_KHR_external_semaphore_win32", feature = "VK_KHR_external_fence_win32",
    feature = "VK_NV_external_memory_win32"
))]
extern crate winapi;
#[cfg(any(feature = "VK_KHR_xlib_surface", feature = "VK_EXT_acquire_xlib_display"))]
extern crate x11;

#[macro_use]
mod vk;
use vk::*;
// use std::ffi::CString;

#[cfg(feature = "FeImplements")] mod fnconv;

pub type Result<T> = std::result::Result<T, VkResult>;
pub trait VkResultHandler
{
	fn into_result(self) -> Result<()>;
}
impl VkResultHandler for VkResult
{
	fn into_result(self) -> Result<()> { if self == VK_SUCCESS { Ok(()) } else { Err(self) } }
}

/// Construction from Pointer that is not checked
pub trait DeviceChild<P>
{
    /// Construct a object from unchecked handle pointer
    /// # Safety
    /// Caller and callee do not guarantee that the passed pointer is valid 
    unsafe fn from_unchecked(p: P, parent: &Device) -> Self;
}
#[cfg(feature = "FeImplements")]
macro_rules! DeviceChildCommonDrop
{
	{ for $($t: ty [$d: expr]),* } =>
	{
		$(
			impl Drop for $t { fn drop(&mut self) { unsafe { $d(self.1.native_ptr(), self.0, ::std::ptr::null()) }; } }
		)*
	}
}

/// size elements
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct Extent1D(pub u32);
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extent2D(pub u32, pub u32);
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extent3D(pub u32, pub u32, pub u32);
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct Offset1D(pub i32);
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset2D(pub i32, pub i32);
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset3D(pub i32, pub i32, pub i32);
// into conversion to larger dimension //
impl Into<Extent2D> for Extent1D { fn into(self) -> Extent2D { Extent2D(self.0, 1) } }
impl Into<Extent3D> for Extent1D { fn into(self) -> Extent3D { Extent3D(self.0, 1, 1) } }
impl Into<Extent3D> for Extent2D { fn into(self) -> Extent3D { Extent3D(self.0, self.1, 1) } }
impl Into<Offset2D> for Offset1D { fn into(self) -> Offset2D { Offset2D(self.0, 0) } }
impl Into<Offset3D> for Offset1D { fn into(self) -> Offset3D { Offset3D(self.0, 0, 0) } }
impl Into<Offset3D> for Offset2D { fn into(self) -> Offset3D { Offset3D(self.0, self.1, 0) } }
// cheap conversion by transmuting //
impl AsRef<u32> for Extent1D { fn as_ref(&self) -> &u32 { &self.0 } }
impl AsRef<VkExtent2D> for Extent2D { fn as_ref(&self) -> &VkExtent2D { unsafe { std::mem::transmute(self) } } }
impl AsRef<VkExtent3D> for Extent3D { fn as_ref(&self) -> &VkExtent3D { unsafe { std::mem::transmute(self) } } }
impl AsRef<i32> for Offset1D { fn as_ref(&self) -> &i32 { &self.0 } }
impl AsRef<VkOffset2D> for Offset2D { fn as_ref(&self) -> &VkOffset2D { unsafe { std::mem::transmute(self) } } }
impl AsRef<VkOffset3D> for Offset3D { fn as_ref(&self) -> &VkOffset3D { unsafe { std::mem::transmute(self) } } }

mod base; pub use base::*;
mod device; pub use device::*;
mod sync; pub use sync::*;
mod resources; pub use resources::*;

/// Opaque handle to a query pool object
pub struct QueryPool(VkQueryPool, Device);
#[cfg(feature = "FeImplements")]
impl QueryPool
{
    /// Create a new query pool object
    /// # Failure
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    pub fn new(device: &Device, qtype: QueryType, count: u32) -> Result<Self>
    {
        let (qtype, stats) = match qtype
        {
            QueryType::Occlusion => (VK_QUERY_TYPE_OCCLUSION, 0),
            QueryType::PipelineStatistics(f) => (VK_QUERY_TYPE_PIPELINE_STATISTICS, f.0),
            QueryType::Timestamp => (VK_QUERY_TYPE_TIMESTAMP, 0)
        };
        let cinfo = VkQueryPoolCreateInfo { queryType: qtype, queryCount: count, pipelineStatistics: stats, .. Default::default() };
        let mut h = VK_NULL_HANDLE as _;
        unsafe { vkCreateQueryPool(device.native_ptr(), &cinfo, std::ptr::null(), &mut h) }
            .into_result().map(|_| QueryPool(h, device.clone()))
    }
    /// Copy results of queries in a query pool to a host memory region
    pub fn results64(&self, query_range: std::ops::Range<u32>, flags: QueryResultFlags) -> Result<Vec<u64>>
    {
        let mut v = Vec::with_capacity(query_range.len()); unsafe { v.set_len(query_range.len()) };
        unsafe { vkGetQueryPoolResults(self.1.native_ptr(), self.0, query_range.start, query_range.len() as _,
            8 * query_range.len(), v.as_mut_ptr() as *mut _, 8, flags.0 | VK_QUERY_RESULT_64_BIT) }
            .into_result().map(|_| v)
    }
    /// Copy results of queries in a query pool to a host memory region
    pub fn results32(&self, query_range: std::ops::Range<u32>, flags: QueryResultFlags) -> Result<Vec<u32>>
    {
        let mut v = Vec::with_capacity(query_range.len()); unsafe { v.set_len(query_range.len()) };
        unsafe { vkGetQueryPoolResults(self.1.native_ptr(), self.0, query_range.start, query_range.len() as _,
            4 * query_range.len(), v.as_mut_ptr() as *mut _, 4, flags.0) }.into_result().map(|_| v)
    }
}
#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{ for QueryPool[vkDestroyQueryPool] }
/// Specify the type of queries managed by a query pool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType
{
    /// An occlusion query
    Occlusion,
    /// A pipeline statistics query
    PipelineStatistics(QueryPipelineStatisticFlags),
    /// A timestamp query
    Timestamp
}
/// Bitmask specifying queried pipeline statistics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryPipelineStatisticFlags(pub VkQueryPipelineStatisticFlags);
impl QueryPipelineStatisticFlags
{
    /// Queries managed by the pool will count the number of vertices processed by the input assembly stage
    pub const INPUT_ASSEMBLY_VERTICES: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT);
    /// Queries managed by the pool will count the number of primitives processed by the input assembly state
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT);
    /// Queries managed by the pool will count the number of vertex shader invocations
    pub const VERTEX_SHADER_INVOCATIONS: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT);
    /// Queries managed by the pool will count the number of geometry shader invocations
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT);
    /// Queries managed by the pool will count the number of primitives generated by geometry shader invocations
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT);
    /// Queries managed by the pool will count the number of primitives processed by the Primitive Clipping stage of the pipeline
    pub const CLIPPING_INVOCATIONS: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT);
    /// Queries managed by the pool will count the number of primitives output by the Primitive Clipping stage of the pipeline
    pub const CLIPPING_PRIMITIVES: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT);
    /// Queries managed by the pool will count the number of fragment shader invocations
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT);
    /// Queries managed by the pool will count the number of patches processed by the tessellation control shader
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT);
    /// Queries managed by the pool will count the number of invocations of the tessellation evaluation shader
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT);
    /// Queries managed by the pool will count the number of compute shader invocations
    pub const COMPUTE_SHADER_INVOCATIONS: Self = QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT);

    /// Queries managed by the pool will count the number of vertices processed by the input assembly stage
    pub fn input_assembly_vertices(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::INPUT_ASSEMBLY_VERTICES.0) }
    /// Queries managed by the pool will count the number of primitives processed by the input assembly state
    pub fn input_assembly_primitives(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::INPUT_ASSEMBLY_PRIMITIVES.0) }
    /// Queries managed by the pool will count the number of vertex shader invocations
    pub fn vertex_shader_invocations(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::VERTEX_SHADER_INVOCATIONS.0) }
    /// Queries managed by the pool will count the number of geometry shader invocations
    pub fn geometry_shader_invocations(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::GEOMETRY_SHADER_INVOCATIONS.0) }
    /// Queries managed by the pool will count the number of primitives generated by geometry shader invocations
    pub fn geometry_shader_primitives(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::GEOMETRY_SHADER_PRIMITIVES.0) }
    /// Queries managed by the pool will count the number of primitives processed by the Primitive Clipping stage of the pipeline
    pub fn clipping_invocations(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::CLIPPING_INVOCATIONS.0) }
    /// Queries managed by the pool will count the number of primitives output by the Primitive Clipping stage of the pipeline
    pub fn clipping_primitives(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::CLIPPING_PRIMITIVES.0) }
    /// Queries managed by the pool will count the number of fragment shader invocations
    pub fn fragment_shader_invocations(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::FRAGMENT_SHADER_INVOCATIONS.0) }
    /// Queries managed by the pool will count the number of patches processed by the tessellation control shader
    pub fn tessellation_control_shader_patches(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::TESSELLATION_CONTROL_SHADER_PATCHES.0) }
    /// Queries managed by the pool will count the number of invocations of the tessellation evaluation shader
    pub fn tessellation_evaluation_shader_invocations(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::TESSELLATION_EVALUATION_SHADER_INVOCATIONS.0) }
    /// Queries managed by the pool will count the number of compute shader invocations
    pub fn compute_shader_invocations(&self) -> Self { QueryPipelineStatisticFlags(self.0 | Self::COMPUTE_SHADER_INVOCATIONS.0) }
}
/// Bitmask specifying how and when query results are returned
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryResultFlags(pub VkQueryResultFlags);
impl QueryResultFlags
{
    /// Empty bits
    pub const EMPTY: Self = QueryResultFlags(0);
    /// Vulkan will wait for each query's status to become available before retrieving its results
    pub const WAIT: Self = QueryResultFlags(VK_QUERY_RESULT_WAIT_BIT);
    /// The availability status accompanies the results
    pub const WITH_AVAILABILITY: Self = QueryResultFlags(VK_QUERY_RESULT_WITH_AVAILABILITY_BIT);
    /// Returning partial results is acceptable
    pub const PARTIAL: Self = QueryResultFlags(VK_QUERY_RESULT_PARTIAL_BIT);

    /// Vulkan will wait for each query's status to become available before retrieving its results
    pub fn wait(&self) -> Self { QueryResultFlags(self.0 | Self::WAIT.0) }
    /// The availability status accompanies the results
    pub fn with_availability(&self) -> Self { QueryResultFlags(self.0 | Self::WITH_AVAILABILITY.0) }
    /// Returning partial results is acceptable
    pub fn partial(&self) -> Self { QueryResultFlags(self.0 | Self::PARTIAL.0) }
}
    pub fn partial(&self) -> Self { QueryResultFlags(self.0 | Self::PARTIAL.0) }
}

/// Opaque handle to a sampler object
pub struct Sampler(pub VkSampler, Device);
#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{ for Sampler }

/// Specify behavior of sampling with texture coordinates outside an image
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum AddressingMode
{
    /// The repeat wrap mode
    Repeat = VK_SAMPLER_ADDRESS_MODE_REPEAT as _,
    /// The mirrored repeat wrap mode
    MirroredRepeat = VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT as _,
    /// The clamp to edge wrap mode
    ClampToEdge = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE as _,
    /// The clamp to border wrap mode
    ClampToBorder = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER as _,
    /// The mirror clamp to edge wrap mode
    #[cfg(feature = "VK_KHR_mirror_clamp_to_edge")]
    MirrorClampToEdge = VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE as _
}
/// Specify filter used for texture lookups
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum FilterMode
{
    /// Nearest filtering
    Nearest = VK_FILTER_NEAREST as _,
    /// Linear filtering
    Linear = VK_FILTER_LINEAR as _
}
/// Specify mipmap mode used for texture lookups
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MipmapFilterMode
{
    /// Nearest filtering
    Nearest = VK_SAMPLER_MIPMAP_MODE_NEAREST as _,
    /// Linear filtering
    Linear = VK_SAMPLER_MIPMAP_MODE_LINEAR as _
}
/// Specify border color used for texture lookups
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum BorderColor
{
    /// A transparent, floating-point format, black color
    TransparentBlackF = VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK as _,
    /// A transparent, integer format, black color
    TransparentBlackI = VK_BORDER_COLOR_INT_TRANSPARENT_BLACK as _,
    /// An opaque, floating-point format, black color
    OpaqueBlackF = VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK as _,
    /// An opaque, integer format, black color
    OpaqueBlackI = VK_BORDER_COLOR_INT_OPAQUE_BLACK as _,
    /// An opaque, floating-point format, white color
    OpaqueWhiteF = VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE as _,
    /// An opaque, integer format, white color
    OpaqueWhiteI = VK_BORDER_COLOR_INT_OPAQUE_WHITE as _
}
/// Builder object for constructing the sampler object
pub struct SamplerBuilder(VkSamplerCreateInfo);
impl SamplerBuilder
{
    pub fn new() -> Self
    {
        SamplerBuilder(VkSamplerCreateInfo
        {
            magFilter: FilterMode::Linear as _, minFilter: FilterMode::Linear as _, mipmapMode: MipmapFilterMode::Linear as _,
            addressModeU: AddressingMode::Repeat as _, addressModeV: AddressingMode::Repeat as _, addressModeW: AddressingMode::Repeat as _,
            mipLodBias: 0.0, anisotropyEnable: false as _, compareEnable: false as _, compareOp: CompareOp::Always as _,
            minLod: 0.0, maxLod: 0.0, borderColor: BorderColor::TransparentBlackF as _, unnormalizedCoordinates: false as _,
            .. Default::default()
        })
    }
    /// The magnification and the minification filters to apply to lookups.
    /// Default: Magnification=`FilterMode::Linear`, Minification=`FilterMode::Linear`
    pub fn filter(&mut self, mag: FilterMode, min: FilterMode) -> &mut Self
    {
        self.0.magFilter = mag as _; self.0.minFilter = min as _; self
    }
    /// The mipmap filter to apply to lookups. Default: `MipmapFilterMode::Linear`
    pub fn mip_filter(&mut self, f: MipmapFilterMode) -> &mut Self
    {
        self.0.mipmapMode = f as _; self
    }
    /// The addressing mode for outside [0..1] range for U, V and W coordinates.
    /// Default: U=`AddressingMode::Repeat`, V=`AddressinMode::Repeat`, W=`AddressingMode::Repeat`
    pub fn addressing(&mut self, u: AddressingMode, v: AddressingMode, w: AddressingMode) -> &mut Self
    {
        self.0.addressModeU = u as _; self.0.addressModeV = v as _; self.0.addressModeW = w as _; self
    }
    /// The bias to be added to mipmap LOD calculation and bias provided by image sampling functions in SPIR-V,
    /// as described in the `Level-of-Detail Operation` section in Vulkan Specification.
    /// Default: 0.0
    pub fn lod_bias(&mut self, bias: f32) -> &mut Self { self.0.mipLodBias = bias; self }
    /// The anisotropy value clamp. Specifying `None` switches off the anisotropic filtering
    /// Default: `None`
    pub fn max_anisotropy(&mut self, level: Option<f32>) -> &mut Self
    {
        self.0.anisotropyEnable = level.is_some() as _;
        self.0.maxAnisotropy = level.unwrap_or_default(); self
    }
    /// The comparison function to apply to fetched data before filtering
    /// as described in the `Depth Compare Operation` section in Vulkan Specification.
    /// Specifying `None` switches off the comparison against a reference value during lookups.
    /// Default: `None`
    pub fn comparison(&mut self, op: Option<CompareOp>) -> &mut Self
    {
        self.0.compareEnable = op.is_some() as _;
        self.0.compareOp = op.unwrap_or(CompareOp::Always) as _; self
    }
    /// The values used to clamp the computed level-of-detail value,
    /// as described in the `Level-of-Detail Operation` section in Vulkan Specification.
    /// Default: min_lod=0.0, max_lod=0.0
    /// # Panics
    /// `max_lod` must be greater than or equal to `min_lod`
    pub fn lod_clamp(&mut self, min_lod: f32, max_lod: f32) -> &mut Self
    {
        assert!(max_lod >= min_lod);
        self.0.minLod = min_lod; self.0.maxLod = max_lod; self
    }
    /// Whether to use unnormalized or normalized texel coordinates to address texels of the image.
    /// Default: `false`
    /// # Safety
    /// User must meet the constraints as described in the "Valid Usage" section in the `VkSamplerCreateInfo` manual page
    pub unsafe fn unnormalized_coordinates(&mut self, use_unnormalized: bool) -> &mut Self
    {
        self.0.unnormalizedCoordinates = use_unnormalized as _; self
    }

    /// Create a new sampler object
    /// # Failures
    /// On failure, this command returns
    /// - VK_ERROR_OUT_OF_HOST_MEMORY
    /// - VK_ERROR_OUT_OF_DEVICE_MEMORY
    /// - VK_ERROR_TOO_MANY_OBJECTS
    #[cfg(feature = "FeImplements")]
    pub fn create(&self, device: &Device) -> Result<Sampler>
    {
        let mut h = VK_NULL_HANDLE as _;
        unsafe { vkCreateSampler(device.native_ptr(), &self.0, std::ptr::null(), &mut h) }
            .into_result().map(|_| Sampler(h, device.clone()))
    }
}
