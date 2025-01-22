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
#![warn(clippy::all)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "alloc"), no_std)]

// Platform Extras
#[cfg(feature = "VK_KHR_android_surface")]
extern crate android;
#[cfg(feature = "DynamicLoaded")]
extern crate libloading;
#[cfg(any(feature = "VK_KHR_xlib_surface", feature = "VK_EXT_acquire_xlib_display"))]
extern crate x11;
#[cfg(feature = "VK_KHR_xcb_surface")]
extern crate xcb;

use cfg_if::cfg_if;
use derives::*;

#[macro_use]
pub mod vk;
use vk::*;
pub mod error;
mod resolver;
#[cfg(feature = "Implements")]
pub use resolver::ResolverInterface;
pub use resolver::{StaticCallable, PFN};

#[cfg(feature = "Implements")]
#[allow(dead_code)]
mod vkfn;

#[cfg(feature = "Implements")]
mod fnconv;

macro_rules! DerefContainerBracketImpl {
    (for mut $t: path { $($required: item)* }) => {
        impl<'s, T> $t for &'s mut T where T: $t + ?Sized { $($required)* }
        impl<T> $t for Box<T> where T: $t + ?Sized { $($required)* }
    };
    (for $t: path { $($required: item)* }) => {
        impl<'s, T> $t for &'s T where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::rc::Rc<T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::sync::Arc<T> where T: $t + ?Sized { $($required)* }

        DerefContainerBracketImpl!(for mut $t { $($required)* });
    }
}
macro_rules! GuardsImpl {
    (for mut $t: path { $($required: item)* }) => {
        impl<T> $t for std::cell::RefMut<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::sync::RwLockWriteGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::sync::MutexGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for parking_lot::MutexGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for parking_lot::RwLockWriteGuard<'_, T> where T: $t + ?Sized { $($required)* }
    };
    (for $t: path { $($required: item)* }) => {
        impl<T> $t for std::cell::Ref<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::sync::RwLockReadGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for parking_lot::RwLockReadGuard<'_, T> where T: $t + ?Sized { $($required)* }

        GuardsImpl!(for mut $t { $($required)* });
    };
}
macro_rules! ForwardFnPtr {
    (deref $name: ident -> $t: ty) => {
        #[inline(always)]
        fn $name(&self) -> $t {
            (**self).$name()
        }
    };
}

pub type Result<T> = core::result::Result<T, VkResult>;

#[cfg(feature = "alloc")]
#[inline(always)]
pub(crate) const fn empty_sink_buffer<T>() -> Vec<T> {
    Vec::new()
}

#[cfg(feature = "alloc")]
#[inline(always)]
pub(crate) unsafe fn alloc_sink_buffer<T>(count: usize) -> Vec<T> {
    let mut xs = Vec::with_capacity(count);
    xs.set_len(count);

    xs
}

#[cfg(feature = "alloc")]
#[inline(always)]
pub(crate) fn collect_vec_alloc<T>(iter: impl IntoIterator<Item = T>) -> Vec<T> {
    Vec::from_iter(iter)
}

#[cfg(feature = "alloc")]
#[inline(always)]
pub(crate) fn str_to_cstr_alloc(s: &str) -> core::result::Result<std::ffi::CString, std::ffi::NulError> {
    std::ffi::CString::new(s)
}

mod handle;
pub use self::handle::*;

/// An object in Vulkan
pub trait VkObject: VkHandle {
    const TYPE: VkObjectType;

    /// Give a user-friendly name to this object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_EXT_debug_utils")]
    fn set_name(&self, name: Option<&core::ffi::CStr>) -> crate::Result<()>
    where
        Self: DeviceChild,
        Self::ConcreteDevice: InstanceChild,
        Self::Handle: VkRawHandle,
    {
        self.device()
            .set_object_name(&DebugUtilsObjectNameInfo::new(self, name))
    }
}
impl<T> VkObject for &'_ T
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for &'_ mut T
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for std::rc::Rc<T>
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for std::sync::Arc<T>
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for std::cell::Ref<'_, T>
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for std::cell::RefMut<'_, T>
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for std::sync::MutexGuard<'_, T>
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
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

// Spreading single value to all dimensions
impl VkExtent2D {
    pub const fn spread1(value: u32) -> Self {
        Self {
            width: value,
            height: value,
        }
    }
}
impl VkExtent3D {
    pub const fn spread1(value: u32) -> Self {
        Self {
            width: value,
            height: value,
            depth: value,
        }
    }
}
impl VkOffset2D {
    pub const fn spread1(value: i32) -> Self {
        Self { x: value, y: value }
    }
}
impl VkOffset3D {
    pub const fn spread1(value: i32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}

// into conversion to larger dimension //
impl VkExtent2D {
    pub const fn with_depth(self, depth: u32) -> VkExtent3D {
        VkExtent3D {
            width: self.width,
            height: self.height,
            depth,
        }
    }
}
impl VkOffset2D {
    pub const fn with_z(self, z: i32) -> VkOffset3D {
        VkOffset3D {
            x: self.x,
            y: self.y,
            z,
        }
    }
}
// AsRef for self //
impl AsRef<VkExtent3D> for VkExtent3D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<VkExtent2D> for VkExtent2D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<VkOffset3D> for VkOffset3D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<VkOffset2D> for VkOffset2D {
    fn as_ref(&self) -> &Self {
        self
    }
}

// AsRef Conversion to smaller-dimension
impl AsRef<VkExtent2D> for VkExtent3D {
    fn as_ref(&self) -> &VkExtent2D {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsRef<VkOffset2D> for VkOffset3D {
    fn as_ref(&self) -> &VkOffset2D {
        unsafe { std::mem::transmute(self) }
    }
}

// Swizzling
impl VkExtent3D {
    #[inline]
    pub const fn wh(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.width,
            height: self.height,
        }
    }

    #[inline]
    pub const fn wd(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.width,
            height: self.depth,
        }
    }

    #[inline]
    pub const fn hw(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.height,
            height: self.width,
        }
    }

    #[inline]
    pub const fn hd(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.height,
            height: self.depth,
        }
    }

    #[inline]
    pub const fn dw(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.depth,
            height: self.width,
        }
    }

    #[inline]
    pub const fn dh(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.depth,
            height: self.height,
        }
    }
}
impl VkOffset3D {
    #[inline]
    pub const fn xy(&self) -> VkOffset2D {
        VkOffset2D { x: self.x, y: self.y }
    }

    #[inline]
    pub const fn xz(&self) -> VkOffset2D {
        VkOffset2D { x: self.x, y: self.z }
    }

    #[inline]
    pub const fn yx(&self) -> VkOffset2D {
        VkOffset2D { x: self.y, y: self.x }
    }

    #[inline]
    pub const fn yz(&self) -> VkOffset2D {
        VkOffset2D { x: self.y, y: self.z }
    }

    #[inline]
    pub const fn zx(&self) -> VkOffset2D {
        VkOffset2D { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zy(&self) -> VkOffset2D {
        VkOffset2D { x: self.z, y: self.y }
    }
}

/// Utility Constants
impl VkExtent2D {
    pub const ONE: Self = Self::spread1(1);
}
impl VkExtent3D {
    pub const ONE: Self = Self::spread1(1);
}
impl VkOffset2D {
    pub const ZERO: Self = Self::spread1(0);
}
impl VkOffset3D {
    pub const ZERO: Self = Self::spread1(0);
}

/// Viewport and Rect Util Functions
impl VkExtent2D {
    pub const fn into_rect(self, offset: VkOffset2D) -> VkRect2D {
        VkRect2D { offset, extent: self }
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
impl VkRect2D {
    pub const fn make_viewport(&self, depth_range: std::ops::Range<f32>) -> VkViewport {
        VkViewport {
            x: self.offset.x as _,
            y: self.offset.y as _,
            width: self.extent.width as _,
            height: self.extent.height as _,
            minDepth: depth_range.start,
            maxDepth: depth_range.end,
        }
    }
}
impl VkViewport {
    pub const fn from_rect_with_depth_range(rect: &VkRect2D, depth_range: std::ops::Range<f32>) -> Self {
        rect.make_viewport(depth_range)
    }

    pub fn set_offset(&mut self, offset: &VkOffset2D) -> &mut Self {
        self.x = offset.x as _;
        self.y = offset.y as _;
        self
    }
    pub fn set_extent(&mut self, extent: &VkExtent2D) -> &mut Self {
        self.width = extent.width as _;
        self.height = extent.height as _;
        self
    }
    pub fn set_depth_range(&mut self, range: std::ops::Range<f32>) -> &mut Self {
        self.minDepth = range.start;
        self.maxDepth = range.end;
        self
    }
}

mod base;
pub(crate) mod ffi_helper;
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
mod renderpass;
pub use self::renderpass::*;
mod framebuffer;
pub use framebuffer::*;
mod shading;
pub use shading::*;
mod command;
pub use command::*;
mod surface;
pub use surface::*;
mod debug;
#[allow(unused_imports)]
pub use debug::*;
mod ext;
pub use self::ext::*;
mod external;
#[allow(unused_imports)]
pub use external::*;
mod batching;
pub use self::batching::*;
mod dependency;
#[allow(unused_imports)]
pub use self::dependency::*;

mod fmt;
pub use self::fmt::*;

/// Opaque handle to a query pool object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkQueryPool::OBJECT_TYPE)]
pub struct QueryPool<Device: VkHandle<Handle = VkDevice>>(VkQueryPool, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for QueryPool<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for QueryPool<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for QueryPool<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for QueryPool<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for QueryPool<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> QueryPool<Device> {
    /// Create a new query pool object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn new(device: Device, qtype: QueryType, count: u32) -> Result<Self> {
        let (qtype, stats) = match qtype {
            QueryType::Occlusion => (VK_QUERY_TYPE_OCCLUSION, 0),
            QueryType::PipelineStatistics(f) => (VK_QUERY_TYPE_PIPELINE_STATISTICS, f.0),
            QueryType::Timestamp => (VK_QUERY_TYPE_TIMESTAMP, 0),
        };
        let cinfo = VkQueryPoolCreateInfo {
            sType: VkQueryPoolCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            queryType: qtype,
            queryCount: count,
            pipelineStatistics: stats,
        };
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            vkfn::create_query_pool(device.native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| Self(h.assume_init(), device))
        }
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
            vkfn::get_query_pool_results(
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
            vkfn::get_query_pool_results(
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
impl<Device: VkHandle<Handle = VkDevice> + Clone> QueryPool<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> QueryPool<Device> {
        QueryPool(self.0, self.1.clone())
    }
}

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
