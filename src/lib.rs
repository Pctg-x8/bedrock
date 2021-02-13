//! Glue library between Vulkan and Rust
//!
//! # Copyright
//! Some documentation comments are from Vulkan Manual Page.  
//! Copyright (c) 2014-2017 Khronos Group.
//!
//! # Compile Options
//! - `Implements`: Enable Vulkan implementations(functions)
//! - `Multithreaded`: Enables to use objects from some threads(experimental)
//! - `Presentation`: Enable rendering features to Window/Display(`VK_KHR_surface`/`VK_KHR_swapchain`/`VK_KHR_display`)
//! - `VK_***`: Enable Vulkan extensions(same name as each extensions)
//!   - Pseudo Extension: `VK_EXT_full_screen_exclusive_win32` for using `VK_EXT_full_screen_exclusive` on Win32 platform
#![warn(clippy::all)]

extern crate libc;
// Platform Extras
#[cfg(feature = "VK_KHR_android_surface")]
extern crate android;
#[cfg(feature = "DynamicLoaded")]
extern crate libloading;
#[cfg(feature = "VK_KHR_wayland_surface")]
extern crate wayland_client;
#[cfg(any(
    feature = "VK_KHR_win32_surface",
    feature = "VK_KHR_external_memory_win32",
    feature = "VK_KHR_external_semaphore_win32",
    feature = "VK_KHR_external_fence_win32",
    feature = "VK_NV_external_memory_win32"
))]
extern crate winapi;
#[cfg(any(feature = "VK_KHR_xlib_surface", feature = "VK_EXT_acquire_xlib_display"))]
extern crate x11;
#[cfg(feature = "VK_KHR_xcb_surface")]
extern crate xcb;

#[macro_use]
pub mod vk;
use vk::*;
pub mod error;
pub use self::error::*;
#[cfg(feature = "Implements")]
mod vkresolve;
#[cfg(feature = "Implements")]
pub use vkresolve::{Resolver, ResolverInterface};

#[cfg(feature = "Multithreaded")]
pub(crate) type RefCounter<T> = std::sync::Arc<T>;
#[cfg(not(feature = "Multithreaded"))]
pub(crate) type RefCounter<T> = std::rc::Rc<T>;

#[cfg(feature = "Implements")]
mod fnconv;

pub type Result<T> = std::result::Result<T, VkResultBox>;
pub trait VkResultHandler {
    fn into_result(self) -> Result<()>;
}
impl VkResultHandler for VkResult {
    fn into_result(self) -> Result<()> {
        if self == VK_SUCCESS {
            Ok(())
        } else {
            Err(VkResultBox(self))
        }
    }
}

/// Wrapping a Vulkan Dispatchable/Nondispatchable Handler
pub trait VkHandle {
    type Handle;
    /// Retrieve an underlying handle
    fn native_ptr(&self) -> Self::Handle;
}
/// Child of a device object
pub trait DeviceChild {
    /// Retrieve a reference to a device object that creates this object
    fn device(&self) -> &Device;
}

/// Unwrapping Option-ed Reference to VkHandles.  
/// Returns "Empty Handle" when the value is `None`.
impl<'h, H: VkHandle + ?Sized + 'h> VkHandle for Option<&'h H> {
    type Handle = <H as VkHandle>::Handle;
    fn native_ptr(&self) -> Self::Handle {
        self.map_or(unsafe { std::mem::zeroed() }, |x| x.native_ptr())
    }
}

#[cfg(feature = "Implements")]
macro_rules! DeviceChildCommonDrop
{
	{ for $($t: ty [$d: ident]),* } =>
	{
		$(
			impl Drop for $t { fn drop(&mut self) { unsafe { Resolver::get().$d(self.1.native_ptr(), self.0, ::std::ptr::null()) }; } }
		)*
	}
}

// A single Number or a Range
pub trait AnalogNumRange<T> {
    fn begin(&self) -> T;
    fn end(&self) -> T;
    fn count(&self) -> T
    where
        T: ::std::ops::Sub<T, Output = T> + Copy,
    {
        self.end() - self.begin()
    }
}
impl<T> AnalogNumRange<T> for T
where
    T: std::ops::Add<u32, Output = T> + Copy,
{
    fn begin(&self) -> T {
        *self
    }
    fn end(&self) -> T {
        *self + 1
    }
}
impl<T> AnalogNumRange<T> for std::ops::Range<T>
where
    T: Copy,
{
    fn begin(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
}

/// FFI Structure with Lifetime bounded(e.g. internal pointer-ed data origins)
pub struct LifetimeBound<'d, T>(T, std::marker::PhantomData<&'d ()>);
impl<'d, T> LifetimeBound<'d, T> {
    /// Converts a value bound with a lifetime
    pub fn new(v: T) -> Self {
        LifetimeBound(v, std::marker::PhantomData)
    }
    /// Unwrap an inner value
    pub unsafe fn unbound(self) -> T {
        self.0
    }
}
impl<'d, T> AsRef<T> for LifetimeBound<'d, T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
impl<'d, T> std::ops::Deref for LifetimeBound<'d, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

/// All of traits
pub mod traits {
    pub use super::{AnalogNumRange, ClearColorValue, DeviceChild, ImageSize, VkHandle, VkResultBox};
    #[cfg(feature = "Implements")]
    pub use super::{MemoryBound, Status, Waitable};
}

// size elements //
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct Extent1D(pub u32);
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extent2D(pub u32, pub u32);
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extent3D(pub u32, pub u32, pub u32);
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extent4D(pub u32, pub u32, pub u32, pub u32);
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct Offset1D(pub i32);
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset2D(pub i32, pub i32);
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset3D(pub i32, pub i32, pub i32);
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset4D(pub i32, pub i32, pub i32, pub i32);
// Native conversion //
impl From<Extent2D> for VkExtent2D {
    fn from(v: Extent2D) -> Self {
        VkExtent2D {
            width: v.0,
            height: v.1,
        }
    }
}
impl From<Extent3D> for VkExtent3D {
    fn from(v: Extent3D) -> Self {
        VkExtent3D {
            width: v.0,
            height: v.1,
            depth: v.2,
        }
    }
}
impl From<Offset2D> for VkOffset2D {
    fn from(v: Offset2D) -> Self {
        VkOffset2D { x: v.0, y: v.1 }
    }
}
impl From<Offset3D> for VkOffset3D {
    fn from(v: Offset3D) -> Self {
        VkOffset3D { x: v.0, y: v.1, z: v.2 }
    }
}
impl From<VkExtent2D> for Extent2D {
    fn from(v: VkExtent2D) -> Self {
        Extent2D(v.width, v.height)
    }
}
impl From<VkExtent3D> for Extent3D {
    fn from(v: VkExtent3D) -> Self {
        Extent3D(v.width, v.height, v.depth)
    }
}
impl From<VkOffset2D> for Offset2D {
    fn from(v: VkOffset2D) -> Self {
        Offset2D(v.x, v.y)
    }
}
impl From<VkOffset3D> for Offset3D {
    fn from(v: VkOffset3D) -> Self {
        Offset3D(v.x, v.y, v.z)
    }
}
// into conversion to larger dimension //
impl From<Extent1D> for Extent2D {
    fn from(v: Extent1D) -> Self {
        Extent2D(v.0, 1)
    }
}
impl From<Extent1D> for Extent3D {
    fn from(v: Extent1D) -> Self {
        Extent3D(v.0, 1, 1)
    }
}
impl From<Extent2D> for Extent3D {
    fn from(v: Extent2D) -> Self {
        Extent3D(v.0, v.1, 1)
    }
}
impl From<Offset1D> for Offset2D {
    fn from(v: Offset1D) -> Self {
        Offset2D(v.0, 0)
    }
}
impl From<Offset1D> for Offset3D {
    fn from(v: Offset1D) -> Self {
        Offset3D(v.0, 0, 0)
    }
}
impl From<Offset2D> for Offset3D {
    fn from(v: Offset2D) -> Self {
        Offset3D(v.0, v.1, 0)
    }
}
// cheap conversion by transmuting //
impl AsRef<u32> for Extent1D {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
impl AsRef<i32> for Offset1D {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}
macro_rules! CoordinateAsRefUnsafe {
    ($a: ty, $b: ty) => {
        impl AsRef<$b> for $a {
            fn as_ref(&self) -> &$b {
                unsafe { &*(self as *const $a as *const $b) }
            }
        }
    };
}
CoordinateAsRefUnsafe!(Extent2D, VkExtent2D);
CoordinateAsRefUnsafe!(Extent3D, VkExtent3D);
CoordinateAsRefUnsafe!(Offset2D, VkOffset2D);
CoordinateAsRefUnsafe!(Offset3D, VkOffset3D);
CoordinateAsRefUnsafe!(Extent1D, [u32; 1]);
CoordinateAsRefUnsafe!(Extent2D, [u32; 2]);
CoordinateAsRefUnsafe!(Extent3D, [u32; 3]);
CoordinateAsRefUnsafe!(Extent4D, [u32; 4]);
CoordinateAsRefUnsafe!(Offset1D, [i32; 1]);
CoordinateAsRefUnsafe!(Offset2D, [i32; 2]);
CoordinateAsRefUnsafe!(Offset3D, [i32; 3]);
CoordinateAsRefUnsafe!(Offset4D, [i32; 4]);
// shrinking by cheap conversion //
impl AsRef<Extent3D> for Extent4D {
    fn as_ref(&self) -> &Extent3D {
        unsafe { &*(self.as_ref() as *const [u32; 4] as *const Extent3D) }
    }
}
impl AsRef<Extent2D> for Extent4D {
    fn as_ref(&self) -> &Extent2D {
        unsafe { &*(self.as_ref() as *const [u32; 4] as *const Extent2D) }
    }
}
impl AsRef<Extent1D> for Extent4D {
    fn as_ref(&self) -> &Extent1D {
        unsafe { &*(self.as_ref() as *const [u32; 4] as *const Extent1D) }
    }
}
impl AsRef<Extent2D> for Extent3D {
    fn as_ref(&self) -> &Extent2D {
        unsafe { &*(self.as_ref() as *const [u32; 3] as *const Extent2D) }
    }
}
impl AsRef<Extent1D> for Extent3D {
    fn as_ref(&self) -> &Extent1D {
        unsafe { &*(self.as_ref() as *const [u32; 3] as *const Extent1D) }
    }
}
impl AsRef<Extent1D> for Extent2D {
    fn as_ref(&self) -> &Extent1D {
        unsafe { &*(self.as_ref() as *const [u32; 2] as *const Extent1D) }
    }
}
impl AsRef<Offset3D> for Offset4D {
    fn as_ref(&self) -> &Offset3D {
        unsafe { &*(self.as_ref() as *const [i32; 4] as *const Offset3D) }
    }
}
impl AsRef<Offset2D> for Offset4D {
    fn as_ref(&self) -> &Offset2D {
        unsafe { &*(self.as_ref() as *const [i32; 4] as *const Offset2D) }
    }
}
impl AsRef<Offset1D> for Offset4D {
    fn as_ref(&self) -> &Offset1D {
        unsafe { &*(self.as_ref() as *const [i32; 4] as *const Offset1D) }
    }
}
impl AsRef<Offset2D> for Offset3D {
    fn as_ref(&self) -> &Offset2D {
        unsafe { &*(self.as_ref() as *const [i32; 3] as *const Offset2D) }
    }
}
impl AsRef<Offset1D> for Offset3D {
    fn as_ref(&self) -> &Offset1D {
        unsafe { &*(self.as_ref() as *const [i32; 3] as *const Offset1D) }
    }
}
impl AsRef<Offset1D> for Offset2D {
    fn as_ref(&self) -> &Offset1D {
        unsafe { &*(self.as_ref() as *const [i32; 2] as *const Offset1D) }
    }
}
// AsRef for self //
impl AsRef<Extent4D> for Extent4D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<Extent3D> for Extent3D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<Extent2D> for Extent2D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<Extent1D> for Extent1D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<Offset4D> for Offset4D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<Offset3D> for Offset3D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<Offset2D> for Offset2D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<Offset1D> for Offset1D {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl From<Extent2D> for VkRect2D {
    fn from(e: Extent2D) -> Self {
        VkRect2D {
            offset: VkOffset2D { x: 0, y: 0 },
            extent: VkExtent2D {
                width: e.0,
                height: e.1,
            },
        }
    }
}
impl From<VkViewport> for VkRect2D {
    fn from(vp: VkViewport) -> Self {
        VkRect2D {
            offset: VkOffset2D {
                x: vp.x as _,
                y: vp.y as _,
            },
            extent: VkExtent2D {
                width: vp.width as _,
                height: vp.height as _,
            },
        }
    }
}

/// Viewport Util Functions
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq)]
pub struct Viewport(VkViewport);
impl From<VkViewport> for Viewport {
    fn from(v: VkViewport) -> Self {
        Viewport(v)
    }
}
impl From<Viewport> for VkViewport {
    fn from(v: Viewport) -> Self {
        v.0
    }
}
impl Viewport {
    pub fn into_inner(self) -> VkViewport {
        self.0
    }

    pub fn from_rect_with_depth_range(rect: &VkRect2D, depth_range: std::ops::Range<f32>) -> Self {
        VkViewport {
            x: rect.offset.x as _,
            y: rect.offset.y as _,
            width: rect.extent.width as _,
            height: rect.extent.height as _,
            minDepth: depth_range.start,
            maxDepth: depth_range.end,
        }
        .into()
    }
    pub fn set_offset(&mut self, offset: &VkOffset2D) -> &mut Self {
        self.0.x = offset.x as _;
        self.0.y = offset.y as _;
        self
    }
    pub fn set_extent(&mut self, extent: &VkExtent2D) -> &mut Self {
        self.0.width = extent.width as _;
        self.0.height = extent.height as _;
        self
    }
    pub fn set_depth_range(&mut self, range: std::ops::Range<f32>) -> &mut Self {
        self.0.minDepth = range.start;
        self.0.maxDepth = range.end;
        self
    }
}

mod base;
pub use base::*;
mod device;
pub use device::*;
mod sync;
pub use sync::*;
pub mod resources;
pub use resources::*;
#[macro_use]
mod descriptor;
pub use descriptor::*;
mod framebuffer;
pub use framebuffer::*;
mod shading;
pub use shading::*;
mod command;
pub use command::*;
mod surface;
pub use surface::*;
#[cfg(feature = "VK_EXT_debug_report")]
mod debug;
#[cfg(feature = "VK_EXT_debug_report")]
pub use debug::*;
mod ext;
pub use ext::*;
mod external;
pub use external::*;

mod fmt;
pub use self::fmt::AsFormat;

/// Opaque handle to a query pool object
pub struct QueryPool(VkQueryPool, Device);
impl VkHandle for QueryPool {
    type Handle = VkQueryPool;
    fn native_ptr(&self) -> VkQueryPool {
        self.0
    }
}
#[cfg(feature = "Implements")]
impl QueryPool {
    /// Create a new query pool object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn new(device: &Device, qtype: QueryType, count: u32) -> Result<Self> {
        let (qtype, stats) = match qtype {
            QueryType::Occlusion => (VK_QUERY_TYPE_OCCLUSION, 0),
            QueryType::PipelineStatistics(f) => (VK_QUERY_TYPE_PIPELINE_STATISTICS, f.0),
            QueryType::Timestamp => (VK_QUERY_TYPE_TIMESTAMP, 0),
        };
        let cinfo = VkQueryPoolCreateInfo {
            queryType: qtype,
            queryCount: count,
            pipelineStatistics: stats,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe { Resolver::get().create_query_pool(device.native_ptr(), &cinfo, std::ptr::null(), &mut h) }
            .into_result()
            .map(|_| QueryPool(h, device.clone()))
    }
    /// Copy results of queries in a query pool to a host memory region
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    pub fn results64(&self, query_range: std::ops::Range<u32>, flags: QueryResultFlags) -> Result<Vec<u64>> {
        let mut v = Vec::with_capacity(query_range.len());
        unsafe { v.set_len(query_range.len()) };
        unsafe {
            Resolver::get().get_query_pool_results(
                self.1.native_ptr(),
                self.0,
                query_range.start,
                query_range.len() as _,
                8 * query_range.len(),
                v.as_mut_ptr() as *mut _,
                8,
                flags.0 | VK_QUERY_RESULT_64_BIT,
            )
        }
        .into_result()
        .map(|_| v)
    }
    /// Copy results of queries in a query pool to a host memory region
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    pub fn results32(&self, query_range: std::ops::Range<u32>, flags: QueryResultFlags) -> Result<Vec<u32>> {
        let mut v = Vec::with_capacity(query_range.len());
        unsafe { v.set_len(query_range.len()) };
        unsafe {
            Resolver::get().get_query_pool_results(
                self.1.native_ptr(),
                self.0,
                query_range.start,
                query_range.len() as _,
                4 * query_range.len(),
                v.as_mut_ptr() as *mut _,
                4,
                flags.0,
            )
        }
        .into_result()
        .map(|_| v)
    }
}
#[cfg(feature = "Implements")]
DeviceChildCommonDrop! { for QueryPool[destroy_query_pool] }
/// Specify the type of queries managed by a query pool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    /// An occlusion query
    Occlusion,
    /// A pipeline statistics query
    PipelineStatistics(QueryPipelineStatisticFlags),
    /// A timestamp query
    Timestamp,
}
/// Bitmask specifying queried pipeline statistics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryPipelineStatisticFlags(pub VkQueryPipelineStatisticFlags);
impl QueryPipelineStatisticFlags {
    /// Queries managed by the pool will count the number of vertices processed by the input assembly stage
    pub const INPUT_ASSEMBLY_VERTICES: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT);
    /// Queries managed by the pool will count the number of primitives processed by the input assembly state
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT);
    /// Queries managed by the pool will count the number of vertex shader invocations
    pub const VERTEX_SHADER_INVOCATIONS: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT);
    /// Queries managed by the pool will count the number of geometry shader invocations
    pub const GEOMETRY_SHADER_INVOCATIONS: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT);
    /// Queries managed by the pool will count the number of primitives generated by geometry shader invocations
    pub const GEOMETRY_SHADER_PRIMITIVES: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT);
    /// Queries managed by the pool will count the number of primitives processed by the Primitive Clipping stage of the pipeline
    pub const CLIPPING_INVOCATIONS: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT);
    /// Queries managed by the pool will count the number of primitives output by the Primitive Clipping stage of the pipeline
    pub const CLIPPING_PRIMITIVES: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT);
    /// Queries managed by the pool will count the number of fragment shader invocations
    pub const FRAGMENT_SHADER_INVOCATIONS: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT);
    /// Queries managed by the pool will count the number of patches processed by the tessellation control shader
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT);
    /// Queries managed by the pool will count the number of invocations of the tessellation evaluation shader
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT);
    /// Queries managed by the pool will count the number of compute shader invocations
    pub const COMPUTE_SHADER_INVOCATIONS: Self =
        QueryPipelineStatisticFlags(VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT);

    /// Queries managed by the pool will count the number of vertices processed by the input assembly stage
    pub fn input_assembly_vertices(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::INPUT_ASSEMBLY_VERTICES.0)
    }
    /// Queries managed by the pool will count the number of primitives processed by the input assembly state
    pub fn input_assembly_primitives(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::INPUT_ASSEMBLY_PRIMITIVES.0)
    }
    /// Queries managed by the pool will count the number of vertex shader invocations
    pub fn vertex_shader_invocations(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::VERTEX_SHADER_INVOCATIONS.0)
    }
    /// Queries managed by the pool will count the number of geometry shader invocations
    pub fn geometry_shader_invocations(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::GEOMETRY_SHADER_INVOCATIONS.0)
    }
    /// Queries managed by the pool will count the number of primitives generated by geometry shader invocations
    pub fn geometry_shader_primitives(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::GEOMETRY_SHADER_PRIMITIVES.0)
    }
    /// Queries managed by the pool will count the number of primitives processed by the Primitive Clipping stage of the pipeline
    pub fn clipping_invocations(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::CLIPPING_INVOCATIONS.0)
    }
    /// Queries managed by the pool will count the number of primitives output by the Primitive Clipping stage of the pipeline
    pub fn clipping_primitives(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::CLIPPING_PRIMITIVES.0)
    }
    /// Queries managed by the pool will count the number of fragment shader invocations
    pub fn fragment_shader_invocations(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::FRAGMENT_SHADER_INVOCATIONS.0)
    }
    /// Queries managed by the pool will count the number of patches processed by the tessellation control shader
    pub fn tessellation_control_shader_patches(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::TESSELLATION_CONTROL_SHADER_PATCHES.0)
    }
    /// Queries managed by the pool will count the number of invocations of the tessellation evaluation shader
    pub fn tessellation_evaluation_shader_invocations(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::TESSELLATION_EVALUATION_SHADER_INVOCATIONS.0)
    }
    /// Queries managed by the pool will count the number of compute shader invocations
    pub fn compute_shader_invocations(self) -> Self {
        QueryPipelineStatisticFlags(self.0 | Self::COMPUTE_SHADER_INVOCATIONS.0)
    }
}
/// Bitmask specifying how and when query results are returned
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryResultFlags(pub VkQueryResultFlags);
impl QueryResultFlags {
    /// Empty bits
    pub const EMPTY: Self = QueryResultFlags(0);
    /// Vulkan will wait for each query's status to become available before retrieving its results
    pub const WAIT: Self = QueryResultFlags(VK_QUERY_RESULT_WAIT_BIT);
    /// The availability status accompanies the results
    pub const WITH_AVAILABILITY: Self = QueryResultFlags(VK_QUERY_RESULT_WITH_AVAILABILITY_BIT);
    /// Returning partial results is acceptable
    pub const PARTIAL: Self = QueryResultFlags(VK_QUERY_RESULT_PARTIAL_BIT);

    /// Vulkan will wait for each query's status to become available before retrieving its results
    pub fn wait(self) -> Self {
        QueryResultFlags(self.0 | Self::WAIT.0)
    }
    /// The availability status accompanies the results
    pub fn with_availability(self) -> Self {
        QueryResultFlags(self.0 | Self::WITH_AVAILABILITY.0)
    }
    /// Returning partial results is acceptable
    pub fn partial(self) -> Self {
        QueryResultFlags(self.0 | Self::PARTIAL.0)
    }
}

/// For testing format traits
pub trait PixelFormat {
    fn bit_width(self) -> usize;
    fn components(self) -> FormatComponents;
    fn element_type(self) -> ElementType;
}
impl PixelFormat for vk::VkFormat {
    fn bit_width(self) -> usize {
        match self {
            VK_FORMAT_R4G4_UNORM_PACK8
            | VK_FORMAT_R8_UNORM
            | VK_FORMAT_R8_SNORM
            | VK_FORMAT_R8_USCALED
            | VK_FORMAT_R8_SSCALED
            | VK_FORMAT_R8_UINT
            | VK_FORMAT_R8_SINT
            | VK_FORMAT_R8_SRGB
            | VK_FORMAT_S8_UINT => 8,
            VK_FORMAT_R4G4B4A4_UNORM_PACK16
            | VK_FORMAT_B4G4R4A4_UNORM_PACK16
            | VK_FORMAT_R5G6B5_UNORM_PACK16
            | VK_FORMAT_B5G6R5_UNORM_PACK16
            | VK_FORMAT_R5G5B5A1_UNORM_PACK16
            | VK_FORMAT_B5G5R5A1_UNORM_PACK16
            | VK_FORMAT_A1R5G5B5_UNORM_PACK16
            | VK_FORMAT_R8G8_UNORM
            | VK_FORMAT_R8G8_SNORM
            | VK_FORMAT_R8G8_USCALED
            | VK_FORMAT_R8G8_SSCALED
            | VK_FORMAT_R8G8_UINT
            | VK_FORMAT_R8G8_SINT
            | VK_FORMAT_R8G8_SRGB
            | VK_FORMAT_R16_UNORM
            | VK_FORMAT_R16_SNORM
            | VK_FORMAT_R16_USCALED
            | VK_FORMAT_R16_SSCALED
            | VK_FORMAT_R16_UINT
            | VK_FORMAT_R16_SINT
            | VK_FORMAT_R16_SFLOAT
            | VK_FORMAT_D16_UNORM => 16,
            VK_FORMAT_R8G8B8_UNORM
            | VK_FORMAT_R8G8B8_SNORM
            | VK_FORMAT_R8G8B8_USCALED
            | VK_FORMAT_R8G8B8_SSCALED
            | VK_FORMAT_R8G8B8_UINT
            | VK_FORMAT_R8G8B8_SINT
            | VK_FORMAT_R8G8B8_SRGB
            | VK_FORMAT_B8G8R8_UNORM
            | VK_FORMAT_B8G8R8_SNORM
            | VK_FORMAT_B8G8R8_USCALED
            | VK_FORMAT_B8G8R8_SSCALED
            | VK_FORMAT_B8G8R8_UINT
            | VK_FORMAT_B8G8R8_SINT
            | VK_FORMAT_B8G8R8_SRGB
            | VK_FORMAT_D16_UNORM_S8_UINT => 24,
            VK_FORMAT_R8G8B8A8_UNORM
            | VK_FORMAT_R8G8B8A8_SNORM
            | VK_FORMAT_R8G8B8A8_USCALED
            | VK_FORMAT_R8G8B8A8_SSCALED
            | VK_FORMAT_R8G8B8A8_UINT
            | VK_FORMAT_R8G8B8A8_SINT
            | VK_FORMAT_R8G8B8A8_SRGB
            | VK_FORMAT_B8G8R8A8_UNORM
            | VK_FORMAT_B8G8R8A8_SNORM
            | VK_FORMAT_B8G8R8A8_USCALED
            | VK_FORMAT_B8G8R8A8_SSCALED
            | VK_FORMAT_B8G8R8A8_UINT
            | VK_FORMAT_B8G8R8A8_SINT
            | VK_FORMAT_B8G8R8A8_SRGB
            | VK_FORMAT_A8B8G8R8_UNORM_PACK32
            | VK_FORMAT_A8B8G8R8_SNORM_PACK32
            | VK_FORMAT_A8B8G8R8_USCALED_PACK32
            | VK_FORMAT_A8B8G8R8_SSCALED_PACK32
            | VK_FORMAT_A8B8G8R8_UINT_PACK32
            | VK_FORMAT_A8B8G8R8_SINT_PACK32
            | VK_FORMAT_A8B8G8R8_SRGB_PACK32
            | VK_FORMAT_A2R10G10B10_UNORM_PACK32
            | VK_FORMAT_A2R10G10B10_SNORM_PACK32
            | VK_FORMAT_A2R10G10B10_USCALED_PACK32
            | VK_FORMAT_A2R10G10B10_SSCALED_PACK32
            | VK_FORMAT_A2R10G10B10_UINT_PACK32
            | VK_FORMAT_A2R10G10B10_SINT_PACK32
            | VK_FORMAT_R16G16_UNORM
            | VK_FORMAT_R16G16_SNORM
            | VK_FORMAT_R16G16_USCALED
            | VK_FORMAT_R16G16_SSCALED
            | VK_FORMAT_R16G16_UINT
            | VK_FORMAT_R16G16_SINT
            | VK_FORMAT_R16G16_SFLOAT
            | VK_FORMAT_R32_UINT
            | VK_FORMAT_R32_SINT
            | VK_FORMAT_R32_SFLOAT
            | VK_FORMAT_B10G11R11_UFLOAT_PACK32
            | VK_FORMAT_E5B9G9R9_UFLOAT_PACK32
            | VK_FORMAT_X8_D24_UNORM_PACK32
            | VK_FORMAT_D32_SFLOAT
            | VK_FORMAT_D24_UNORM_S8_UINT => 32,
            VK_FORMAT_D32_SFLOAT_S8_UINT => 40,
            VK_FORMAT_R16G16B16_UNORM
            | VK_FORMAT_R16G16B16_SNORM
            | VK_FORMAT_R16G16B16_USCALED
            | VK_FORMAT_R16G16B16_SSCALED
            | VK_FORMAT_R16G16B16_UINT
            | VK_FORMAT_R16G16B16_SINT
            | VK_FORMAT_R16G16B16_SFLOAT => 48,
            VK_FORMAT_R16G16B16A16_UNORM
            | VK_FORMAT_R16G16B16A16_SNORM
            | VK_FORMAT_R16G16B16A16_USCALED
            | VK_FORMAT_R16G16B16A16_SSCALED
            | VK_FORMAT_R16G16B16A16_UINT
            | VK_FORMAT_R16G16B16A16_SINT
            | VK_FORMAT_R16G16B16A16_SFLOAT
            | VK_FORMAT_R32G32_UINT
            | VK_FORMAT_R32G32_SINT
            | VK_FORMAT_R32G32_SFLOAT
            | VK_FORMAT_R64_UINT
            | VK_FORMAT_R64_SINT
            | VK_FORMAT_R64_SFLOAT => 64,
            VK_FORMAT_R32G32B32_UINT | VK_FORMAT_R32G32B32_SINT | VK_FORMAT_R32G32B32_SFLOAT => 96,
            VK_FORMAT_R32G32B32A32_UINT
            | VK_FORMAT_R32G32B32A32_SINT
            | VK_FORMAT_R32G32B32A32_SFLOAT
            | VK_FORMAT_R64G64_UINT
            | VK_FORMAT_R64G64_SINT
            | VK_FORMAT_R64G64_SFLOAT => 128,
            VK_FORMAT_R64G64B64_UINT | VK_FORMAT_R64G64B64_SINT | VK_FORMAT_R64G64B64_SFLOAT => 192,
            VK_FORMAT_R64G64B64A64_UINT | VK_FORMAT_R64G64B64A64_SINT | VK_FORMAT_R64G64B64A64_SFLOAT => 256,
            _ => 0,
        }
    }
    fn components(self) -> FormatComponents {
        match self {
            VK_FORMAT_UNDEFINED => FormatComponents::Undefined,
            VK_FORMAT_R8_UNORM
            | VK_FORMAT_R8_SNORM
            | VK_FORMAT_R8_USCALED
            | VK_FORMAT_R8_SSCALED
            | VK_FORMAT_R8_UINT
            | VK_FORMAT_R8_SINT
            | VK_FORMAT_R8_SRGB
            | VK_FORMAT_R16_UNORM
            | VK_FORMAT_R16_SNORM
            | VK_FORMAT_R16_USCALED
            | VK_FORMAT_R16_SSCALED
            | VK_FORMAT_R16_UINT
            | VK_FORMAT_R16_SINT
            | VK_FORMAT_R16_SFLOAT
            | VK_FORMAT_R32_UINT
            | VK_FORMAT_R32_SINT
            | VK_FORMAT_R32_SFLOAT
            | VK_FORMAT_R64_UINT
            | VK_FORMAT_R64_SINT
            | VK_FORMAT_R64_SFLOAT => FormatComponents::R,
            VK_FORMAT_R4G4_UNORM_PACK8
            | VK_FORMAT_R8G8_UNORM
            | VK_FORMAT_R8G8_SNORM
            | VK_FORMAT_R8G8_USCALED
            | VK_FORMAT_R8G8_SSCALED
            | VK_FORMAT_R8G8_UINT
            | VK_FORMAT_R8G8_SINT
            | VK_FORMAT_R8G8_SRGB
            | VK_FORMAT_R16G16_UNORM
            | VK_FORMAT_R16G16_SNORM
            | VK_FORMAT_R16G16_USCALED
            | VK_FORMAT_R16G16_SSCALED
            | VK_FORMAT_R16G16_UINT
            | VK_FORMAT_R16G16_SINT
            | VK_FORMAT_R16G16_SFLOAT
            | VK_FORMAT_R32G32_UINT
            | VK_FORMAT_R32G32_SINT
            | VK_FORMAT_R32G32_SFLOAT
            | VK_FORMAT_R64G64_UINT
            | VK_FORMAT_R64G64_SINT
            | VK_FORMAT_R64G64_SFLOAT => FormatComponents::RG,
            VK_FORMAT_R5G6B5_UNORM_PACK16
            | VK_FORMAT_B5G6R5_UNORM_PACK16
            | VK_FORMAT_R8G8B8_UNORM
            | VK_FORMAT_R8G8B8_SNORM
            | VK_FORMAT_R8G8B8_USCALED
            | VK_FORMAT_R8G8B8_SSCALED
            | VK_FORMAT_R8G8B8_UINT
            | VK_FORMAT_R8G8B8_SINT
            | VK_FORMAT_R8G8B8_SRGB
            | VK_FORMAT_B8G8R8_UNORM
            | VK_FORMAT_B8G8R8_SNORM
            | VK_FORMAT_B8G8R8_USCALED
            | VK_FORMAT_B8G8R8_SSCALED
            | VK_FORMAT_B8G8R8_UINT
            | VK_FORMAT_B8G8R8_SINT
            | VK_FORMAT_B8G8R8_SRGB
            | VK_FORMAT_R16G16B16_UNORM
            | VK_FORMAT_R16G16B16_SNORM
            | VK_FORMAT_R16G16B16_USCALED
            | VK_FORMAT_R16G16B16_SSCALED
            | VK_FORMAT_R16G16B16_UINT
            | VK_FORMAT_R16G16B16_SINT
            | VK_FORMAT_R16G16B16_SFLOAT
            | VK_FORMAT_R32G32B32_UINT
            | VK_FORMAT_R32G32B32_SINT
            | VK_FORMAT_R32G32B32_SFLOAT
            | VK_FORMAT_R64G64B64_UINT
            | VK_FORMAT_R64G64B64_SINT
            | VK_FORMAT_R64G64B64_SFLOAT
            | VK_FORMAT_B10G11R11_UFLOAT_PACK32 => FormatComponents::RGB,
            VK_FORMAT_R4G4B4A4_UNORM_PACK16
            | VK_FORMAT_B4G4R4A4_UNORM_PACK16
            | VK_FORMAT_R5G5B5A1_UNORM_PACK16
            | VK_FORMAT_B5G5R5A1_UNORM_PACK16
            | VK_FORMAT_A1R5G5B5_UNORM_PACK16
            | VK_FORMAT_R8G8B8A8_UNORM
            | VK_FORMAT_R8G8B8A8_SNORM
            | VK_FORMAT_R8G8B8A8_USCALED
            | VK_FORMAT_R8G8B8A8_SSCALED
            | VK_FORMAT_R8G8B8A8_UINT
            | VK_FORMAT_R8G8B8A8_SINT
            | VK_FORMAT_R8G8B8A8_SRGB
            | VK_FORMAT_B8G8R8A8_UNORM
            | VK_FORMAT_B8G8R8A8_SNORM
            | VK_FORMAT_B8G8R8A8_USCALED
            | VK_FORMAT_B8G8R8A8_SSCALED
            | VK_FORMAT_B8G8R8A8_UINT
            | VK_FORMAT_B8G8R8A8_SINT
            | VK_FORMAT_B8G8R8A8_SRGB
            | VK_FORMAT_A8B8G8R8_UNORM_PACK32
            | VK_FORMAT_A8B8G8R8_SNORM_PACK32
            | VK_FORMAT_A8B8G8R8_USCALED_PACK32
            | VK_FORMAT_A8B8G8R8_SSCALED_PACK32
            | VK_FORMAT_A8B8G8R8_UINT_PACK32
            | VK_FORMAT_A8B8G8R8_SINT_PACK32
            | VK_FORMAT_A8B8G8R8_SRGB_PACK32
            | VK_FORMAT_A2R10G10B10_UNORM_PACK32
            | VK_FORMAT_A2R10G10B10_SNORM_PACK32
            | VK_FORMAT_A2R10G10B10_USCALED_PACK32
            | VK_FORMAT_A2R10G10B10_SSCALED_PACK32
            | VK_FORMAT_A2R10G10B10_UINT_PACK32
            | VK_FORMAT_A2R10G10B10_SINT_PACK32
            | VK_FORMAT_A2B10G10R10_UNORM_PACK32
            | VK_FORMAT_A2B10G10R10_SNORM_PACK32
            | VK_FORMAT_A2B10G10R10_USCALED_PACK32
            | VK_FORMAT_A2B10G10R10_SSCALED_PACK32
            | VK_FORMAT_A2B10G10R10_UINT_PACK32
            | VK_FORMAT_A2B10G10R10_SINT_PACK32
            | VK_FORMAT_R16G16B16A16_UNORM
            | VK_FORMAT_R16G16B16A16_SNORM
            | VK_FORMAT_R16G16B16A16_USCALED
            | VK_FORMAT_R16G16B16A16_SSCALED
            | VK_FORMAT_R16G16B16A16_UINT
            | VK_FORMAT_R16G16B16A16_SINT
            | VK_FORMAT_R16G16B16A16_SFLOAT
            | VK_FORMAT_R32G32B32A32_UINT
            | VK_FORMAT_R32G32B32A32_SINT
            | VK_FORMAT_R32G32B32A32_SFLOAT
            | VK_FORMAT_R64G64B64A64_SINT
            | VK_FORMAT_R64G64B64A64_UINT
            | VK_FORMAT_R64G64B64A64_SFLOAT => FormatComponents::RGBA,
            VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 => FormatComponents::EBGR,
            VK_FORMAT_D16_UNORM | VK_FORMAT_X8_D24_UNORM_PACK32 | VK_FORMAT_D32_SFLOAT => FormatComponents::D,
            VK_FORMAT_S8_UINT => FormatComponents::S,
            VK_FORMAT_D16_UNORM_S8_UINT | VK_FORMAT_D24_UNORM_S8_UINT | VK_FORMAT_D32_SFLOAT_S8_UINT => {
                FormatComponents::DS
            }
            _ => FormatComponents::Compressed,
        }
    }
    #[allow(non_upper_case_globals)]
    fn element_type(self) -> ElementType {
        match self {
            VK_FORMAT_UNDEFINED => ElementType::Undefined,
            VK_FORMAT_R4G4_UNORM_PACK8
            | VK_FORMAT_R4G4B4A4_UNORM_PACK16
            | VK_FORMAT_B4G4R4A4_UNORM_PACK16
            | VK_FORMAT_R5G6B5_UNORM_PACK16
            | VK_FORMAT_B5G6R5_UNORM_PACK16
            | VK_FORMAT_R5G5B5A1_UNORM_PACK16
            | VK_FORMAT_B5G5R5A1_UNORM_PACK16
            | VK_FORMAT_A1R5G5B5_UNORM_PACK16
            | VK_FORMAT_R8_UNORM
            | VK_FORMAT_R8G8_UNORM
            | VK_FORMAT_R8G8B8_UNORM
            | VK_FORMAT_B8G8R8_UNORM
            | VK_FORMAT_R8G8B8A8_UNORM
            | VK_FORMAT_B8G8R8A8_UNORM
            | VK_FORMAT_A8B8G8R8_UNORM_PACK32
            | VK_FORMAT_A2R10G10B10_UNORM_PACK32
            | VK_FORMAT_A2B10G10R10_UNORM_PACK32
            | VK_FORMAT_R16G16B16_UNORM
            | VK_FORMAT_D16_UNORM
            | VK_FORMAT_X8_D24_UNORM_PACK32
            | VK_FORMAT_BC1_RGB_UNORM_BLOCK
            | VK_FORMAT_BC1_RGBA_UNORM_BLOCK
            | VK_FORMAT_BC2_UNORM_BLOCK
            | VK_FORMAT_BC3_UNORM_BLOCK
            | VK_FORMAT_BC4_UNORM_BLOCK
            | VK_FORMAT_BC5_UNORM_BLOCK
            | VK_FORMAT_BC7_UNORM_BLOCK
            | VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK
            | VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK
            | VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK
            | VK_FORMAT_EAC_R11_UNORM_BLOCK
            | VK_FORMAT_EAC_R11G11_UNORM_BLOCK
            | VK_FORMAT_ASTC_4x4_UNORM_BLOCK
            | VK_FORMAT_ASTC_5x4_UNORM_BLOCK
            | VK_FORMAT_ASTC_5x5_UNORM_BLOCK
            | VK_FORMAT_ASTC_6x5_UNORM_BLOCK
            | VK_FORMAT_ASTC_6x6_UNORM_BLOCK
            | VK_FORMAT_ASTC_8x5_UNORM_BLOCK
            | VK_FORMAT_ASTC_8x6_UNORM_BLOCK
            | VK_FORMAT_ASTC_8x8_UNORM_BLOCK
            | VK_FORMAT_ASTC_10x5_UNORM_BLOCK
            | VK_FORMAT_ASTC_10x6_UNORM_BLOCK
            | VK_FORMAT_ASTC_10x8_UNORM_BLOCK
            | VK_FORMAT_ASTC_10x10_UNORM_BLOCK
            | VK_FORMAT_ASTC_12x10_UNORM_BLOCK
            | VK_FORMAT_ASTC_12x12_UNORM_BLOCK
            | VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG
            | VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG
            | VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG
            | VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG => ElementType::UNORM,
            VK_FORMAT_R8_SNORM
            | VK_FORMAT_R8G8_SNORM
            | VK_FORMAT_R8G8B8_SNORM
            | VK_FORMAT_B8G8R8_SNORM
            | VK_FORMAT_R8G8B8A8_SNORM
            | VK_FORMAT_B8G8R8A8_SNORM
            | VK_FORMAT_A8B8G8R8_SNORM_PACK32
            | VK_FORMAT_A2R10G10B10_SNORM_PACK32
            | VK_FORMAT_A2B10G10R10_SNORM_PACK32
            | VK_FORMAT_R16_SNORM
            | VK_FORMAT_R16G16_SNORM
            | VK_FORMAT_R16G16B16_SNORM
            | VK_FORMAT_R16G16B16A16_SNORM
            | VK_FORMAT_BC4_SNORM_BLOCK
            | VK_FORMAT_BC5_SNORM_BLOCK
            | VK_FORMAT_EAC_R11_SNORM_BLOCK
            | VK_FORMAT_EAC_R11G11_SNORM_BLOCK => ElementType::SNORM,
            VK_FORMAT_R8_USCALED
            | VK_FORMAT_R8G8_USCALED
            | VK_FORMAT_R8G8B8_USCALED
            | VK_FORMAT_B8G8R8_USCALED
            | VK_FORMAT_R8G8B8A8_USCALED
            | VK_FORMAT_B8G8R8A8_USCALED
            | VK_FORMAT_A8B8G8R8_USCALED_PACK32
            | VK_FORMAT_A2R10G10B10_USCALED_PACK32
            | VK_FORMAT_A2B10G10R10_USCALED_PACK32
            | VK_FORMAT_R16_USCALED
            | VK_FORMAT_R16G16_USCALED
            | VK_FORMAT_R16G16B16_USCALED
            | VK_FORMAT_R16G16B16A16_USCALED => ElementType::USCALED,
            VK_FORMAT_R8_SSCALED
            | VK_FORMAT_R8G8_SSCALED
            | VK_FORMAT_R8G8B8_SSCALED
            | VK_FORMAT_B8G8R8_SSCALED
            | VK_FORMAT_R8G8B8A8_SSCALED
            | VK_FORMAT_B8G8R8A8_SSCALED
            | VK_FORMAT_A8B8G8R8_SSCALED_PACK32
            | VK_FORMAT_A2R10G10B10_SSCALED_PACK32
            | VK_FORMAT_A2B10G10R10_SSCALED_PACK32
            | VK_FORMAT_R16_SSCALED
            | VK_FORMAT_R16G16_SSCALED
            | VK_FORMAT_R16G16B16_SSCALED
            | VK_FORMAT_R16G16B16A16_SSCALED => ElementType::SSCALED,
            VK_FORMAT_R8_UINT
            | VK_FORMAT_R8G8_UINT
            | VK_FORMAT_R8G8B8_UINT
            | VK_FORMAT_B8G8R8_UINT
            | VK_FORMAT_R8G8B8A8_UINT
            | VK_FORMAT_B8G8R8A8_UINT
            | VK_FORMAT_A8B8G8R8_UINT_PACK32
            | VK_FORMAT_A2R10G10B10_UINT_PACK32
            | VK_FORMAT_A2B10G10R10_UINT_PACK32
            | VK_FORMAT_R16_UINT
            | VK_FORMAT_R16G16_UINT
            | VK_FORMAT_R16G16B16_UINT
            | VK_FORMAT_R16G16B16A16_UINT
            | VK_FORMAT_R32_UINT
            | VK_FORMAT_R32G32_UINT
            | VK_FORMAT_R32G32B32_UINT
            | VK_FORMAT_R32G32B32A32_UINT
            | VK_FORMAT_R64_UINT
            | VK_FORMAT_R64G64_UINT
            | VK_FORMAT_R64G64B64_UINT
            | VK_FORMAT_R64G64B64A64_UINT
            | VK_FORMAT_S8_UINT => ElementType::UINT,
            VK_FORMAT_R8_SINT
            | VK_FORMAT_R8G8_SINT
            | VK_FORMAT_R8G8B8_SINT
            | VK_FORMAT_B8G8R8_SINT
            | VK_FORMAT_R8G8B8A8_SINT
            | VK_FORMAT_B8G8R8A8_SINT
            | VK_FORMAT_A8B8G8R8_SINT_PACK32
            | VK_FORMAT_A2R10G10B10_SINT_PACK32
            | VK_FORMAT_A2B10G10R10_SINT_PACK32
            | VK_FORMAT_R16_SINT
            | VK_FORMAT_R16G16_SINT
            | VK_FORMAT_R16G16B16_SINT
            | VK_FORMAT_R16G16B16A16_SINT
            | VK_FORMAT_R32_SINT
            | VK_FORMAT_R32G32_SINT
            | VK_FORMAT_R32G32B32_SINT
            | VK_FORMAT_R32G32B32A32_SINT
            | VK_FORMAT_R64_SINT
            | VK_FORMAT_R64G64_SINT
            | VK_FORMAT_R64G64B64_SINT
            | VK_FORMAT_R64G64B64A64_SINT => ElementType::SINT,
            VK_FORMAT_R8_SRGB
            | VK_FORMAT_R8G8_SRGB
            | VK_FORMAT_R8G8B8_SRGB
            | VK_FORMAT_B8G8R8_SRGB
            | VK_FORMAT_R8G8B8A8_SRGB
            | VK_FORMAT_B8G8R8A8_SRGB
            | VK_FORMAT_A8B8G8R8_SRGB_PACK32
            | VK_FORMAT_BC1_RGB_SRGB_BLOCK
            | VK_FORMAT_BC1_RGBA_SRGB_BLOCK
            | VK_FORMAT_BC2_SRGB_BLOCK
            | VK_FORMAT_BC3_SRGB_BLOCK
            | VK_FORMAT_BC7_SRGB_BLOCK
            | VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK
            | VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK
            | VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK
            | VK_FORMAT_ASTC_4x4_SRGB_BLOCK
            | VK_FORMAT_ASTC_5x4_SRGB_BLOCK
            | VK_FORMAT_ASTC_5x5_SRGB_BLOCK
            | VK_FORMAT_ASTC_6x5_SRGB_BLOCK
            | VK_FORMAT_ASTC_6x6_SRGB_BLOCK
            | VK_FORMAT_ASTC_8x5_SRGB_BLOCK
            | VK_FORMAT_ASTC_8x6_SRGB_BLOCK
            | VK_FORMAT_ASTC_8x8_SRGB_BLOCK
            | VK_FORMAT_ASTC_10x5_SRGB_BLOCK
            | VK_FORMAT_ASTC_10x6_SRGB_BLOCK
            | VK_FORMAT_ASTC_10x8_SRGB_BLOCK
            | VK_FORMAT_ASTC_10x10_SRGB_BLOCK
            | VK_FORMAT_ASTC_12x10_SRGB_BLOCK
            | VK_FORMAT_ASTC_12x12_SRGB_BLOCK
            | VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG
            | VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG
            | VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG
            | VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG => ElementType::SRGB,
            VK_FORMAT_R16_SFLOAT
            | VK_FORMAT_R16G16_SFLOAT
            | VK_FORMAT_R16G16B16_SFLOAT
            | VK_FORMAT_R16G16B16A16_SFLOAT
            | VK_FORMAT_R32_SFLOAT
            | VK_FORMAT_R32G32_SFLOAT
            | VK_FORMAT_R32G32B32_SFLOAT
            | VK_FORMAT_R32G32B32A32_SFLOAT
            | VK_FORMAT_R64_SFLOAT
            | VK_FORMAT_R64G64_SFLOAT
            | VK_FORMAT_R64G64B64_SFLOAT
            | VK_FORMAT_R64G64B64A64_SFLOAT
            | VK_FORMAT_D32_SFLOAT
            | VK_FORMAT_BC6H_SFLOAT_BLOCK => ElementType::SFLOAT,
            VK_FORMAT_B10G11R11_UFLOAT_PACK32 | VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 | VK_FORMAT_BC6H_UFLOAT_BLOCK => {
                ElementType::UFLOAT
            }
            _ => ElementType::Compound,
        }
    }
}

/// Arbitrary queries of Format
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FormatQuery(pub vk::VkFormat);
impl FormatQuery {
    pub fn eq_bit_width(self, w: usize) -> Self {
        if self.0.bit_width() == w {
            self
        } else {
            FormatQuery(VK_FORMAT_UNDEFINED)
        }
    }
    pub fn has_components(self, c: FormatComponents) -> Self {
        if c.satisfy(self.0) {
            self
        } else {
            FormatQuery(VK_FORMAT_UNDEFINED)
        }
    }
    pub fn is_component_of(self, c: FormatComponents) -> Self {
        if c.satisfy_eq(self.0) {
            self
        } else {
            FormatQuery(VK_FORMAT_UNDEFINED)
        }
    }
    pub fn has_element_of(self, e: ElementType) -> Self {
        if self.0.element_type() == e {
            self
        } else {
            FormatQuery(VK_FORMAT_UNDEFINED)
        }
    }
    pub fn passed(self) -> bool {
        self.0 != VK_FORMAT_UNDEFINED
    }

    /// convert UNORM to SRGB if exists
    pub fn srgb(self) -> Option<VkFormat> {
        match self.0 {
            VK_FORMAT_R8_UNORM => Some(VK_FORMAT_R8_SRGB),
            VK_FORMAT_R8G8_UNORM => Some(VK_FORMAT_R8G8_SRGB),
            VK_FORMAT_R8G8B8_UNORM => Some(VK_FORMAT_R8G8B8_SRGB),
            VK_FORMAT_B8G8R8_UNORM => Some(VK_FORMAT_B8G8R8_SRGB),
            VK_FORMAT_R8G8B8A8_UNORM => Some(VK_FORMAT_R8G8B8A8_SRGB),
            VK_FORMAT_B8G8R8A8_UNORM => Some(VK_FORMAT_B8G8R8A8_SRGB),
            VK_FORMAT_A8B8G8R8_UNORM_PACK32 => Some(VK_FORMAT_A8B8G8R8_SRGB_PACK32),
            _ => None,
        }
    }
    /// convert to UNORM if exists
    pub fn unorm(self) -> Option<VkFormat> {
        match self.0 {
            VK_FORMAT_R8_SRGB | VK_FORMAT_R8_UNORM => Some(VK_FORMAT_R8_UNORM),
            VK_FORMAT_R8G8_SRGB | VK_FORMAT_R8G8_UNORM => Some(VK_FORMAT_R8G8_UNORM),
            VK_FORMAT_R8G8B8_SRGB | VK_FORMAT_R8G8B8_UNORM => Some(VK_FORMAT_R8G8B8_UNORM),
            VK_FORMAT_B8G8R8_SRGB | VK_FORMAT_B8G8R8_UNORM => Some(VK_FORMAT_B8G8R8_UNORM),
            VK_FORMAT_R8G8B8A8_SRGB | VK_FORMAT_R8G8B8A8_UNORM => Some(VK_FORMAT_R8G8B8A8_UNORM),
            VK_FORMAT_B8G8R8A8_SRGB | VK_FORMAT_B8G8R8A8_UNORM => Some(VK_FORMAT_B8G8R8A8_UNORM),
            VK_FORMAT_A8B8G8R8_SRGB_PACK32 | VK_FORMAT_A8B8G8R8_UNORM_PACK32 => Some(VK_FORMAT_A8B8G8R8_UNORM_PACK32),
            _ => None,
        }
    }
}
/// Predication style of Format Selection Query
#[derive(Clone)]
pub struct FormatQueryPred {
    bit_width: Option<usize>,
    req_components: Option<FormatComponents>,
    req_elements_of: Option<ElementType>,
}
/// Empty data
impl Default for FormatQueryPred {
    fn default() -> Self {
        FormatQueryPred {
            bit_width: None,
            req_components: None,
            req_elements_of: None,
        }
    }
}
impl FormatQueryPred {
    pub fn bit(&mut self, b: usize) -> &mut Self {
        self.bit_width = Some(b);
        self
    }
    pub fn components(&mut self, c: FormatComponents) -> &mut Self {
        self.req_components = Some(c);
        self
    }
    pub fn elements(&mut self, e: ElementType) -> &mut Self {
        self.req_elements_of = Some(e);
        self
    }

    pub fn satisfy(&self, f: vk::VkFormat) -> bool {
        self.bit_width.map_or(true, |b| f.bit_width() == b)
            && self.req_components.map_or(true, |c| c.satisfy(f))
            && self.req_elements_of.map_or(true, |e| f.element_type() == e)
    }
}

/// Containing Components in Format(Order is not considered)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FormatComponents {
    Undefined,
    R,
    RG,
    RGB,
    RGBA,
    EBGR,
    D,
    S,
    DS,
    Compressed,
}
/// Containing component element in format
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Undefined,
    UNORM,
    SNORM,
    UINT,
    SINT,
    SFLOAT,
    UFLOAT,
    SRGB,
    USCALED,
    SSCALED,
    Compound,
}

impl FormatComponents {
    pub fn has(self, o: Self) -> bool {
        use self::FormatComponents::*;
        match self {
            R => o == R || o == RG || o == RGB || o == RGBA,
            RG => o == RG || o == RGB || o == RGBA,
            RGB => o == RGB || o == RGBA,
            D => o == D || o == DS,
            S => o == S || o == DS,
            t => t == o,
        }
    }
    pub fn satisfy(self, f: vk::VkFormat) -> bool {
        self.has(f.components())
    }
    pub fn satisfy_eq(self, f: vk::VkFormat) -> bool {
        f.components() == self
    }
}
