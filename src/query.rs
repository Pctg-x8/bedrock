use crate::*;
use derives::{bitflags_newtype, implements};

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

/// Structure specifying parameters of a newly created query pool
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryPoolCreateInfo(VkQueryPoolCreateInfo);
impl QueryPoolCreateInfo {
    pub const fn new(query_type: QueryType, count: u32) -> Self {
        let (qt, ps) = match query_type {
            QueryType::Occlusion => (VK_QUERY_TYPE_OCCLUSION, 0),
            QueryType::PipelineStatistics(st) => (VK_QUERY_TYPE_PIPELINE_STATISTICS, st.bits()),
            QueryType::Timestamp => (VK_QUERY_TYPE_TIMESTAMP, 0),
        };

        Self(VkQueryPoolCreateInfo {
            sType: VkQueryPoolCreateInfo::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            queryType: qt,
            queryCount: count,
            pipelineStatistics: ps,
        })
    }

    pub const unsafe fn from_raw(raw: VkQueryPoolCreateInfo) -> Self {
        Self(raw)
    }

    pub const fn into_raw(self) -> VkQueryPoolCreateInfo {
        self.0
    }
}

/// Opaque handle to a query pool object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkQueryPool::OBJECT_TYPE)]
pub struct QueryPoolObject<Device: VkHandle<Handle = VkDevice>>(VkQueryPool, Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for QueryPoolObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for QueryPoolObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for QueryPoolObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for QueryPoolObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for QueryPoolObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: VkHandle<Handle = VkDevice>> QueryPool for QueryPoolObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> QueryPoolObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkQueryPool, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkQueryPool, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Device: VkHandle<Handle = VkDevice> + Clone> QueryPoolObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> QueryPoolObject<Device> {
        let r = QueryPoolObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}
impl<Device: crate::Device> QueryPoolObject<Device> {
    /// Create a new query pool object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    pub fn new(device: Device, info: &QueryPoolCreateInfo) -> crate::Result<Self> {
        Ok(unsafe { Self::manage(device.new_query_pool_raw(info, None)?, device) })
    }
}

pub enum QueryResult<T> {
    Err(VkResult),
    Ready(T),
    NotReady,
}
impl QueryResult<()> {
    pub(crate) const fn from_vk(r: VkResult) -> Self {
        match r {
            VK_NOT_READY => Self::NotReady,
            r if r.is_err() => Self::Err(r),
            _ => Self::Ready(()),
        }
    }
}
impl<T> QueryResult<T> {
    #[inline(always)]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> QueryResult<U> {
        match self {
            Self::Err(r) => QueryResult::Err(r),
            Self::NotReady => QueryResult::NotReady,
            Self::Ready(x) => QueryResult::Ready(f(x)),
        }
    }
}

pub trait QueryPool: VkHandle<Handle = VkQueryPool> + DeviceChildHandle {
    /// Copy results of queries in a query pool to a host memory region
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[implements]
    #[inline]
    fn results<T>(&self, offset: u32, sink: &mut [T], flags: QueryResultFlags) -> QueryResult<()> {
        unsafe {
            QueryResult::from_vk(crate::vkfn::get_query_pool_results(
                self.device_handle(),
                self.native_ptr(),
                offset,
                sink.len() as _,
                (core::mem::size_of::<T>() * sink.len()) as _,
                sink.as_mut_ptr() as _,
                core::mem::size_of::<T>() as _,
                flags.bits(),
            ))
        }
    }

    /// Copy results of queries in a query pool to a host memory region
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[implements]
    fn result_array<const N: usize, T>(&self, offset: u32, flags: QueryResultFlags) -> QueryResult<[T; N]> {
        let mut sink = [const { unsafe { core::mem::MaybeUninit::<T>::uninit().assume_init() } }; N];
        self.results(offset, &mut sink, flags).map(move |_| sink)
    }

    /// Copy results of queries in a query pool to a host memory region
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[implements("alloc")]
    fn results64_alloc(&self, query_range: core::ops::Range<u32>, flags: QueryResultFlags) -> QueryResult<Vec<u64>> {
        let mut v = unsafe { crate::alloc::alloc_sink_buffer(query_range.len()) };
        self.results(query_range.start, &mut v, flags | QueryResultFlags::WIDE)
            .map(move |_| v)
    }

    /// Copy results of queries in a query pool to a host memory region
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    #[implements("alloc")]
    fn results32_alloc(&self, query_range: core::ops::Range<u32>, flags: QueryResultFlags) -> QueryResult<Vec<u32>> {
        let mut v = unsafe { crate::alloc::alloc_sink_buffer(query_range.len()) };
        self.results(query_range.start, &mut v, flags).map(move |_| v)
    }
}

/// Bitmask specifying queried pipeline statistics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
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
}

/// Bitmask specifying how and when query results are returned
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct QueryResultFlags(VkQueryResultFlags);
impl QueryResultFlags {
    /// Empty bits
    pub const EMPTY: Self = QueryResultFlags(0);
    /// The results will be written as array of 64-bit unsigned integer values
    pub const WIDE: Self = QueryResultFlags(VK_QUERY_RESULT_64_BIT);
    /// Vulkan will wait for each query's status to become available before retrieving its results
    pub const WAIT: Self = QueryResultFlags(VK_QUERY_RESULT_WAIT_BIT);
    /// The availability status accompanies the results
    pub const WITH_AVAILABILITY: Self = QueryResultFlags(VK_QUERY_RESULT_WITH_AVAILABILITY_BIT);
    /// Returning partial results is acceptable
    pub const PARTIAL: Self = QueryResultFlags(VK_QUERY_RESULT_PARTIAL_BIT);
}
