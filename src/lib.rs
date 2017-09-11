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

pub type Result<T> = std::result::Result<T, VkResultBox>;
pub trait VkResultHandler
{
	fn into_result(self) -> Result<()>;
}
impl VkResultHandler for VkResult
{
	fn into_result(self) -> Result<()> { if self == VK_SUCCESS { Ok(()) } else { Err(VkResultBox(self)) } }
}
/// Boxed version of `VkResult`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkResultBox(pub VkResult);
impl std::error::Error for VkResultBox
{
    fn description(&self) -> &str
    {
        match self.0
        {
            // Success Codes //
            VK_SUCCESS => "Command successfully completed", VK_NOT_READY => "A fence or query has not yet completed",
            VK_TIMEOUT => "A wait operation has not completed in the specified time", VK_EVENT_SET => "An event is signaled",
            VK_EVENT_RESET => "An event is unsignaled", VK_INCOMPLETE => "A return array was too small for the result",
            #[cfg(feature = "VK_KHR_swapchain")]
            VK_KHR_SUBOPTIMAL => "Sub-optimal swapchain",
            // Error Codes //
            VK_ERROR_OUT_OF_HOST_MEMORY => "A host memory allocation has failed",
            VK_ERROR_OUT_OF_DEVICE_MEMORY => "A device memory allocation has failed",
            VK_ERROR_INITIALIZATION_FAILED => "Initialization of an object could not be completed for implementation-specific reasons",
            VK_ERROR_DEVICE_LOST => "The logical or physical device has been lost",
            VK_ERROR_MEMORY_MAP_FAILED => "Mapping of a memory object has failed",
            VK_ERROR_LAYER_NOT_PRESENT => "A requested layer is not presented or could not be loaded",
            VK_ERROR_EXTENSION_NOT_PRESENT => "A requested extension is not supported",
            VK_ERROR_FEATURE_NOT_PRESENT => "A requested feature is not supported",
            VK_ERROR_INCOMPATIBLE_DRIVER => "The requested version of Vulkan is not supported by the driver or is otherwise incompatible for implementation-specific reasons",
            VK_ERROR_TOO_MANY_OBJECTS => "Too many objects of the type have already been created",
            VK_ERROR_FORMAT_NOT_SUPPORTED => "A requested format is not supported on this device",
            VK_ERROR_FRAGMENTED_POOL => "A pool allocation has failed due to fragmentation of the pool's memory",
            #[cfg(feature = "VK_KHR_surface")]
            VK_ERROR_SURFACE_LOST_KHR => "Surface lost",
            #[cfg(feature = "VK_KHR_surface")]
            VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => "Native window is in use",
            #[cfg(feature = "VK_KHR_swapchain")]
            VK_ERROR_OUT_OF_DATE_KHR => "Out of date",
            #[cfg(feature = "VK_KHR_display_swapchain")]
            VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => "The display used by a swapchain does not use the same presentable image layout",
            #[cfg(feature = "VK_EXT_debug_report")]
            VK_ERROR_VALIDATION_FAILED_EXT => "Validation failed",
            #[cfg(feature = "VK_NV_glsl_shader")]
            VK_ERROR_INVALID_SHADER_NV => "Invalid GLSL shader",
            #[cfg(feature = "VK_KHR_maintenance1")]
            VK_ERROR_OUT_OF_POOL_MEMORY_KHR => "A pool memory allocation has failed",
            #[cfg(feature = "VK_KHR_external_memory_capabilities")]
            VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR => "An external handle is not a valid handle of ths specified type",
            _ => "Unknown or extension-specific error"
        }
    }
}
impl std::fmt::Display for VkResultBox
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(fmt, "[{:?}] {}", self.0, (self as &std::error::Error).description())
    }
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
mod descriptor; pub use descriptor::*;
mod shading; pub use shading::*;

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
