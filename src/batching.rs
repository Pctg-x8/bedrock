use bedrock_vk::{self as brvk, TypedVulkanStructure};

use crate::ffi_helper::{ArrayFFIExtensions, slice_as_ptr_empty_null};
use crate::*;
use core::marker::PhantomData;

#[cfg(feature = "alloc")]
#[deprecated = "old batching library"]
pub struct TemporalSubmissionBatchResources {
    command_buffers: Vec<brvk::VkCommandBuffer>,
    wait_semaphores: Vec<brvk::VkSemaphore>,
    wait_stages: Vec<brvk::VkPipelineStageFlags>,
    signal_semaphores: Vec<brvk::VkSemaphore>,
}
#[cfg(feature = "alloc")]
#[allow(deprecated)]
impl TemporalSubmissionBatchResources {
    pub const fn new() -> Self {
        Self {
            command_buffers: Vec::new(),
            wait_semaphores: Vec::new(),
            wait_stages: Vec::new(),
            signal_semaphores: Vec::new(),
        }
    }

    pub fn make_info_struct<'a>(&'a self) -> SubmitInfo<'a, 'a, 'a> {
        unsafe {
            SubmitInfo::from_raw(brvk::VkSubmitInfo {
                sType: brvk::VkSubmitInfo::TYPE,
                pNext: std::ptr::null(),
                commandBufferCount: self.command_buffers.len() as _,
                pCommandBuffers: self.command_buffers.as_ptr_empty_null(),
                waitSemaphoreCount: self.wait_semaphores.len() as _,
                pWaitSemaphores: self.wait_semaphores.as_ptr_empty_null(),
                pWaitDstStageMask: self.wait_stages.as_ptr_empty_null(),
                signalSemaphoreCount: self.signal_semaphores.len() as _,
                pSignalSemaphores: self.signal_semaphores.as_ptr_empty_null(),
            })
        }
    }
}

#[deprecated = "old batching library"]
pub trait SubmissionBatch {
    #[cfg(feature = "alloc")]
    #[allow(deprecated)]
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources);

    #[cfg(feature = "alloc")]
    #[allow(deprecated)]
    #[inline]
    fn with_command_buffers<'d, CommandBuffer: crate::CommandBuffer + 'd>(
        self,
        command_buffers: &'d [CommandBuffer],
    ) -> SubmissionWithCommandBuffers<'d, Self, CommandBuffer>
    where
        Self: Sized,
    {
        SubmissionWithCommandBuffers(
            self,
            crate::alloc::collect_vec(command_buffers.iter().map(crate::VkHandle::native_ptr)),
            std::marker::PhantomData,
        )
    }

    #[cfg(feature = "alloc")]
    #[allow(deprecated)]
    #[inline]
    fn with_wait_semaphores<'d, Semaphore: crate::Semaphore + 'd>(
        self,
        wait_semaphores: &'d [(Semaphore, PipelineStageFlags)],
    ) -> SubmissionWithWaitSemaphores<'d, Self, Semaphore>
    where
        Self: Sized,
    {
        let (hs, fs) = crate::alloc::unzip_vec(
            wait_semaphores
                .iter()
                .map(|(a, b)| (crate::VkHandle::native_ptr(a), b.0)),
        );
        SubmissionWithWaitSemaphores(self, hs, fs, std::marker::PhantomData)
    }

    #[cfg(feature = "alloc")]
    #[allow(deprecated)]
    #[inline]
    fn with_signal_semaphores<'d, Semaphore: crate::Semaphore + 'd>(
        self,
        signal_semaphores: &'d [Semaphore],
    ) -> SubmissionWithSignalSemaphores<'d, Self, Semaphore>
    where
        Self: Sized,
    {
        SubmissionWithSignalSemaphores(
            self,
            crate::alloc::collect_vec(signal_semaphores.iter().map(crate::VkHandle::native_ptr)),
            std::marker::PhantomData,
        )
    }
}
#[allow(deprecated)]
impl<T: SubmissionBatch + ?Sized> SubmissionBatch for Box<T> {
    #[cfg(feature = "alloc")]
    #[inline]
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources) {
        T::collect_resources(self, target)
    }
}

#[repr(transparent)]
pub struct SubmitInfo<'r, 'rs, 'n>(
    brvk::VkSubmitInfo,
    #[allow(clippy::type_complexity)]
    PhantomData<(
        Option<&'n dyn brvk::VulkanStructure>,
        &'rs [VkHandleRef<'r, brvk::VkSemaphore>],
        &'rs [PipelineStageFlags],
        &'rs [VkHandleRef<'r, brvk::VkCommandBuffer>],
        &'rs [VkHandleRef<'r, brvk::VkSemaphore>],
    )>,
);
impl<'r, 'rs, 'n> SubmitInfo<'r, 'rs, 'n> {
    /// # Safety
    ///
    /// `wait_semaphores`, `wait_semaphore_dst_stages`, `command_buffers`, and `signal_semaphores` must have same length.
    pub unsafe fn new_unchecked(
        wait_semaphores: &'rs [VkHandleRef<'r, brvk::VkSemaphore>],
        wait_semaphore_dst_stages: &'rs [PipelineStageFlags],
        command_buffers: &'rs [VkHandleRef<'r, brvk::VkCommandBuffer>],
        signal_semaphores: &'rs [VkHandleRef<'r, brvk::VkSemaphore>],
    ) -> Self {
        Self(
            brvk::VkSubmitInfo {
                sType: brvk::VkSubmitInfo::TYPE,
                pNext: core::ptr::null(),
                waitSemaphoreCount: wait_semaphores.len() as _,
                pWaitSemaphores: slice_as_ptr_empty_null(wait_semaphores) as _,
                pWaitDstStageMask: slice_as_ptr_empty_null(wait_semaphore_dst_stages) as _,
                commandBufferCount: command_buffers.len() as _,
                pCommandBuffers: slice_as_ptr_empty_null(command_buffers) as _,
                signalSemaphoreCount: signal_semaphores.len() as _,
                pSignalSemaphores: slice_as_ptr_empty_null(signal_semaphores) as _,
            },
            PhantomData,
        )
    }

    pub fn new(
        wait_semaphores: &'rs [VkHandleRef<'r, brvk::VkSemaphore>],
        wait_semaphore_dst_stages: &'rs [PipelineStageFlags],
        command_buffers: &'rs [VkHandleRef<'r, brvk::VkCommandBuffer>],
        signal_semaphores: &'rs [VkHandleRef<'r, brvk::VkSemaphore>],
    ) -> Self {
        assert_eq!(wait_semaphores.len(), wait_semaphore_dst_stages.len());

        unsafe {
            Self::new_unchecked(
                wait_semaphores,
                wait_semaphore_dst_stages,
                command_buffers,
                signal_semaphores,
            )
        }
    }

    pub const fn new_array<const NW: usize, const NC: usize, const NS: usize>(
        wait_semaphores: &'rs [VkHandleRef<'r, brvk::VkSemaphore>; NW],
        wait_semaphore_dst_stages: &'rs [PipelineStageFlags; NW],
        command_buffers: &'rs [VkHandleRef<'r, brvk::VkCommandBuffer>; NC],
        signal_semaphores: &'rs [VkHandleRef<'r, brvk::VkSemaphore>; NS],
    ) -> Self {
        Self(
            brvk::VkSubmitInfo {
                sType: brvk::VkSubmitInfo::TYPE,
                pNext: core::ptr::null(),
                waitSemaphoreCount: NW as _,
                pWaitSemaphores: slice_as_ptr_empty_null(wait_semaphores) as _,
                pWaitDstStageMask: slice_as_ptr_empty_null(wait_semaphore_dst_stages) as _,
                commandBufferCount: NC as _,
                pCommandBuffers: slice_as_ptr_empty_null(command_buffers) as _,
                signalSemaphoreCount: NS as _,
                pSignalSemaphores: slice_as_ptr_empty_null(signal_semaphores) as _,
            },
            PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkSubmitInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkSubmitInfo) -> Self {
        Self(raw, PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkSubmitInfo {
        self.0
    }

    pub const fn with_next(mut self, next: &'n (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }
}

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[repr(transparent)]
pub struct TimelineSemaphoreSubmitInfo<'d, 'xs>(
    brvk::VkTimelineSemaphoreSubmitInfoKHR,
    core::marker::PhantomData<(Option<&'d dyn brvk::VulkanStructure>, &'xs [u64])>,
);
#[cfg(feature = "VK_KHR_timeline_semaphore")]
impl<'d, 'xs> TimelineSemaphoreSubmitInfo<'d, 'xs> {
    pub const fn new(wait_semaphore_values: &'xs [u64], signal_semaphore_values: &'xs [u64]) -> Self {
        Self(
            brvk::VkTimelineSemaphoreSubmitInfoKHR {
                sType: brvk::VkTimelineSemaphoreSubmitInfoKHR::TYPE,
                pNext: core::ptr::null(),
                waitSemaphoreValueCount: wait_semaphore_values.len() as _,
                pWaitSemaphoreValues: slice_as_ptr_empty_null(wait_semaphore_values) as _,
                signalSemaphoreValueCount: signal_semaphore_values.len() as _,
                pSignalSemaphoreValues: slice_as_ptr_empty_null(signal_semaphore_values) as _,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkTimelineSemaphoreSubmitInfoKHR`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkTimelineSemaphoreSubmitInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkTimelineSemaphoreSubmitInfoKHR {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
unsafe impl brvk::VulkanStructure for TimelineSemaphoreSubmitInfo<'_, '_> {
    #[inline(always)]
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        self.0.as_generic()
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        self.0.as_generic_mut()
    }
}

#[deprecated = "old batching library"]
pub struct EmptySubmissionBatch;
#[allow(deprecated)]
impl SubmissionBatch for EmptySubmissionBatch {
    #[cfg(feature = "alloc")]
    fn collect_resources(&self, _: &mut TemporalSubmissionBatchResources) {}
}
#[cfg(feature = "alloc")]
#[deprecated = "old batching library"]
#[allow(deprecated)]
pub struct SubmissionWithCommandBuffers<'d, Parent: SubmissionBatch, CommandBuffer: crate::CommandBuffer + 'd>(
    Parent,
    Vec<brvk::VkCommandBuffer>,
    std::marker::PhantomData<&'d [CommandBuffer]>,
);
#[cfg(feature = "alloc")]
#[allow(deprecated)]
impl<'d, Parent, CommandBuffer> SubmissionBatch for SubmissionWithCommandBuffers<'d, Parent, CommandBuffer>
where
    Parent: SubmissionBatch,
    CommandBuffer: crate::CommandBuffer + 'd,
{
    #[inline]
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources) {
        self.0.collect_resources(target);
        target.command_buffers.extend(self.1.iter().copied());
    }
}
#[cfg(feature = "alloc")]
#[deprecated = "old batching library"]
#[allow(deprecated)]
pub struct SubmissionWithWaitSemaphores<'d, Parent: SubmissionBatch, Semaphore: crate::Semaphore + 'd>(
    Parent,
    Vec<brvk::VkSemaphore>,
    Vec<brvk::VkPipelineStageFlags>,
    std::marker::PhantomData<&'d [Semaphore]>,
);
#[cfg(feature = "alloc")]
#[allow(deprecated)]
impl<'d, Parent, Semaphore> SubmissionBatch for SubmissionWithWaitSemaphores<'d, Parent, Semaphore>
where
    Parent: SubmissionBatch,
    Semaphore: crate::Semaphore + 'd,
{
    #[inline]
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources) {
        self.0.collect_resources(target);
        target.wait_semaphores.extend(self.1.iter().copied());
        target.wait_stages.extend(self.2.iter().copied());
    }
}
#[cfg(feature = "alloc")]
#[deprecated = "old batching library"]
#[allow(deprecated)]
pub struct SubmissionWithSignalSemaphores<'d, Parent: SubmissionBatch, Semaphore: crate::Semaphore + 'd>(
    Parent,
    Vec<brvk::VkSemaphore>,
    std::marker::PhantomData<&'d [Semaphore]>,
);
#[cfg(feature = "alloc")]
#[allow(deprecated)]
impl<'d, Parent, Semaphore> SubmissionBatch for SubmissionWithSignalSemaphores<'d, Parent, Semaphore>
where
    Parent: SubmissionBatch,
    Semaphore: crate::Semaphore + 'd,
{
    #[inline]
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources) {
        self.0.collect_resources(target);
        target.signal_semaphores.extend(self.1.iter().copied());
    }
}

pub trait SparseBindingOpBatch {
    fn make_info_struct<'a>(&'a self) -> BindSparseInfo<'a>;

    #[inline]
    fn with_buffer_binds<'d>(
        self,
        buffer_binds: &'d [brvk::VkSparseBufferMemoryBindInfo],
    ) -> SparseBindingOpBatchWithBufferBinds<'d, Self>
    where
        Self: Sized,
    {
        SparseBindingOpBatchWithBufferBinds(self, buffer_binds)
    }

    #[inline]
    fn with_image_binds<'d>(
        self,
        buffer_binds: &'d [brvk::VkSparseImageMemoryBindInfo],
    ) -> SparseBindingOpBatchWithImageBinds<'d, Self>
    where
        Self: Sized,
    {
        SparseBindingOpBatchWithImageBinds(self, buffer_binds)
    }

    #[inline]
    fn with_image_opaque_binds<'d>(
        self,
        buffer_binds: &'d [brvk::VkSparseImageOpaqueMemoryBindInfo],
    ) -> SparseBindingOpBatchWithImageOpaqueBinds<'d, Self>
    where
        Self: Sized,
    {
        SparseBindingOpBatchWithImageOpaqueBinds(self, buffer_binds)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn with_wait_semaphores<'d, Semaphore: crate::Semaphore + 'd>(
        self,
        semaphores: &'d [Semaphore],
    ) -> SparseBindingOpBatchWithWaitSemaphores<'d, Self, Semaphore>
    where
        Self: Sized,
    {
        SparseBindingOpBatchWithWaitSemaphores(
            self,
            crate::alloc::collect_vec(semaphores.iter().map(crate::VkHandle::native_ptr)),
            std::marker::PhantomData,
        )
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn with_signal_semaphores<'d, Semaphore: crate::Semaphore + 'd>(
        self,
        semaphores: &'d [Semaphore],
    ) -> SparseBindingOpBatchWithSignalSemaphores<'d, Self, Semaphore>
    where
        Self: Sized,
    {
        SparseBindingOpBatchWithSignalSemaphores(
            self,
            crate::alloc::collect_vec(semaphores.iter().map(crate::VkHandle::native_ptr)),
            std::marker::PhantomData,
        )
    }
}
impl SparseBindingOpBatch for BindSparseInfo<'_> {
    #[inline]
    fn make_info_struct<'a>(&'a self) -> BindSparseInfo<'a> {
        unsafe { BindSparseInfo::from_raw(self.as_raw_ref().clone()) }
    }
}

pub struct EmptyBindingOpBatch;
impl SparseBindingOpBatch for EmptyBindingOpBatch {
    #[inline]
    fn make_info_struct<'a>(&'a self) -> BindSparseInfo<'a> {
        unsafe {
            BindSparseInfo::from_raw(brvk::VkBindSparseInfo {
                sType: brvk::VkBindSparseInfo::TYPE,
                pNext: std::ptr::null(),
                waitSemaphoreCount: 0,
                pWaitSemaphores: std::ptr::null(),
                bufferBindCount: 0,
                pBufferBinds: std::ptr::null(),
                imageBindCount: 0,
                pImageBinds: std::ptr::null(),
                imageOpaqueBindCount: 0,
                pImageOpaqueBinds: std::ptr::null(),
                signalSemaphoreCount: 0,
                pSignalSemaphores: std::ptr::null(),
            })
        }
    }
}
pub struct SparseBindingOpBatchWithBufferBinds<'d, Parent: SparseBindingOpBatch>(
    Parent,
    &'d [brvk::VkSparseBufferMemoryBindInfo],
);
impl<'d, Parent: SparseBindingOpBatch> SparseBindingOpBatch for SparseBindingOpBatchWithBufferBinds<'d, Parent> {
    #[inline]
    fn make_info_struct<'a>(&'a self) -> BindSparseInfo<'a> {
        unsafe {
            BindSparseInfo::from_raw(brvk::VkBindSparseInfo {
                bufferBindCount: self.1.len() as _,
                pBufferBinds: self.1.as_ptr_empty_null(),
                ..self.0.make_info_struct().into_raw()
            })
        }
    }
}
pub struct SparseBindingOpBatchWithImageBinds<'d, Parent: SparseBindingOpBatch>(
    Parent,
    &'d [brvk::VkSparseImageMemoryBindInfo],
);
impl<'d, Parent: SparseBindingOpBatch> SparseBindingOpBatch for SparseBindingOpBatchWithImageBinds<'d, Parent> {
    #[inline]
    fn make_info_struct<'a>(&'a self) -> BindSparseInfo<'a> {
        unsafe {
            BindSparseInfo::from_raw(brvk::VkBindSparseInfo {
                imageBindCount: self.1.len() as _,
                pImageBinds: self.1.as_ptr_empty_null(),
                ..self.0.make_info_struct().into_raw()
            })
        }
    }
}
pub struct SparseBindingOpBatchWithImageOpaqueBinds<'d, Parent: SparseBindingOpBatch>(
    Parent,
    &'d [brvk::VkSparseImageOpaqueMemoryBindInfo],
);
impl<'d, Parent: SparseBindingOpBatch> SparseBindingOpBatch for SparseBindingOpBatchWithImageOpaqueBinds<'d, Parent> {
    #[inline]
    fn make_info_struct<'a>(&'a self) -> BindSparseInfo<'a> {
        unsafe {
            BindSparseInfo::from_raw(brvk::VkBindSparseInfo {
                imageOpaqueBindCount: self.1.len() as _,
                pImageOpaqueBinds: self.1.as_ptr_empty_null(),
                ..self.0.make_info_struct().into_raw()
            })
        }
    }
}
pub struct SparseBindingOpBatchWithWaitSemaphores<'d, Parent: SparseBindingOpBatch, Semaphore: crate::Semaphore + 'd>(
    Parent,
    Vec<brvk::VkSemaphore>,
    std::marker::PhantomData<&'d [Semaphore]>,
);
impl<'d, Parent: SparseBindingOpBatch, Semaphore: crate::Semaphore + 'd> SparseBindingOpBatch
    for SparseBindingOpBatchWithWaitSemaphores<'d, Parent, Semaphore>
{
    #[inline]
    fn make_info_struct<'a>(&'a self) -> BindSparseInfo<'a> {
        unsafe {
            BindSparseInfo::from_raw(brvk::VkBindSparseInfo {
                waitSemaphoreCount: self.1.len() as _,
                pWaitSemaphores: self.1.as_ptr_empty_null(),
                ..self.0.make_info_struct().into_raw()
            })
        }
    }
}
pub struct SparseBindingOpBatchWithSignalSemaphores<'d, Parent: SparseBindingOpBatch, Semaphore: crate::Semaphore + 'd>(
    Parent,
    Vec<brvk::VkSemaphore>,
    std::marker::PhantomData<&'d [Semaphore]>,
);
impl<'d, Parent: SparseBindingOpBatch, Semaphore: crate::Semaphore + 'd> SparseBindingOpBatch
    for SparseBindingOpBatchWithSignalSemaphores<'d, Parent, Semaphore>
{
    #[inline]
    fn make_info_struct<'a>(&'a self) -> BindSparseInfo<'a> {
        unsafe {
            BindSparseInfo::from_raw(brvk::VkBindSparseInfo {
                signalSemaphoreCount: self.1.len() as _,
                pSignalSemaphores: self.1.as_ptr_empty_null(),
                ..self.0.make_info_struct().into_raw()
            })
        }
    }
}

#[repr(transparent)]
pub struct BindSparseInfo<'a> {
    vk: brvk::VkBindSparseInfo,
    _wait_semaphores: core::marker::PhantomData<&'a [VkHandleRef<'a, brvk::VkSemaphore>]>,
    _signal_semaphores: core::marker::PhantomData<&'a [VkHandleRef<'a, brvk::VkSemaphore>]>,
    _buffer_binds: core::marker::PhantomData<&'a [SparseBufferMemoryBindInfo<'a>]>,
    _image_binds: core::marker::PhantomData<&'a [SparseImageMemoryBindInfo<'a>]>,
    _image_opaque_binds: core::marker::PhantomData<&'a [SparseImageOpaqueMemoryBindInfo<'a>]>,
}
impl<'a> BindSparseInfo<'a> {
    #[inline(always)]
    pub const fn new() -> Self {
        unsafe {
            Self::from_raw(brvk::VkBindSparseInfo {
                sType: brvk::VkBindSparseInfo::TYPE,
                pNext: core::ptr::null(),
                waitSemaphoreCount: 0,
                pWaitSemaphores: core::ptr::null(),
                signalSemaphoreCount: 0,
                pSignalSemaphores: core::ptr::null(),
                bufferBindCount: 0,
                pBufferBinds: core::ptr::null(),
                imageBindCount: 0,
                pImageBinds: core::ptr::null(),
                imageOpaqueBindCount: 0,
                pImageOpaqueBinds: core::ptr::null(),
            })
        }
    }

    #[inline(always)]
    pub const fn into_raw(self) -> brvk::VkBindSparseInfo {
        self.vk
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkBindSparseInfo`] struct.
    pub const unsafe fn from_raw(vk: brvk::VkBindSparseInfo) -> Self {
        Self {
            vk,
            _wait_semaphores: core::marker::PhantomData,
            _signal_semaphores: core::marker::PhantomData,
            _buffer_binds: core::marker::PhantomData,
            _image_binds: core::marker::PhantomData,
            _image_opaque_binds: core::marker::PhantomData,
        }
    }

    pub const fn as_raw_ref(&self) -> &brvk::VkBindSparseInfo {
        &self.vk
    }

    pub const unsafe fn as_raw_ref_mut(&mut self) -> &mut brvk::VkBindSparseInfo {
        &mut self.vk
    }

    #[inline(always)]
    pub const fn wait_semaphores(mut self, semaphores: &'a [VkHandleRef<'a, brvk::VkSemaphore>]) -> Self {
        self.vk.waitSemaphoreCount = semaphores.len() as _;
        self.vk.pWaitSemaphores = semaphores.as_ptr() as _;

        self
    }

    #[inline(always)]
    pub const fn signal_semaphores(mut self, semaphores: &'a [VkHandleRef<'a, brvk::VkSemaphore>]) -> Self {
        self.vk.signalSemaphoreCount = semaphores.len() as _;
        self.vk.pSignalSemaphores = semaphores.as_ptr() as _;

        self
    }

    #[inline(always)]
    pub const fn buffer_binds(mut self, binds: &'a [SparseBufferMemoryBindInfo<'a>]) -> Self {
        self.vk.bufferBindCount = binds.len() as _;
        self.vk.pBufferBinds = binds.as_ptr() as _;

        self
    }

    #[inline(always)]
    pub const fn image_binds(mut self, binds: &'a [SparseImageMemoryBindInfo<'a>]) -> Self {
        self.vk.imageBindCount = binds.len() as _;
        self.vk.pImageBinds = binds.as_ptr() as _;

        self
    }

    #[inline(always)]
    pub const fn image_opaque_binds(mut self, binds: &'a [SparseImageOpaqueMemoryBindInfo<'a>]) -> Self {
        self.vk.imageOpaqueBindCount = binds.len() as _;
        self.vk.pImageOpaqueBinds = binds.as_ptr() as _;

        self
    }
}

#[repr(transparent)]
pub struct SparseBufferMemoryBindInfo<'a> {
    vk: brvk::VkSparseBufferMemoryBindInfo,
    _buffer: core::marker::PhantomData<&'a dyn VkHandle<Handle = brvk::VkBuffer>>,
    _binds: core::marker::PhantomData<&'a [SparseMemoryBind<'a>]>,
}
impl<'a> SparseBufferMemoryBindInfo<'a> {
    #[inline(always)]
    pub fn new(
        buffer: &'a (impl VkHandle<Handle = brvk::VkBuffer> + ?Sized),
        binds: &'a [SparseMemoryBind<'a>],
    ) -> Self {
        unsafe {
            Self::from_raw(brvk::VkSparseBufferMemoryBindInfo {
                buffer: buffer.native_ptr(),
                bindCount: binds.len() as _,
                pBinds: binds.as_ptr() as _,
            })
        }
    }

    #[inline(always)]
    pub const fn into_raw(self) -> brvk::VkSparseBufferMemoryBindInfo {
        self.vk
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkSparseBufferMemoryBindInfo`] struct.
    #[inline(always)]
    pub const unsafe fn from_raw(vk: brvk::VkSparseBufferMemoryBindInfo) -> Self {
        Self {
            vk,
            _buffer: core::marker::PhantomData,
            _binds: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub const fn as_raw_ref(&self) -> &brvk::VkSparseBufferMemoryBindInfo {
        &self.vk
    }

    #[inline(always)]
    pub const unsafe fn as_raw_ref_mut(&mut self) -> &mut brvk::VkSparseBufferMemoryBindInfo {
        &mut self.vk
    }
}

#[repr(transparent)]
pub struct SparseImageMemoryBindInfo<'a> {
    vk: brvk::VkSparseImageMemoryBindInfo,
    _image: core::marker::PhantomData<&'a dyn VkHandle<Handle = brvk::VkImage>>,
    _binds: core::marker::PhantomData<&'a [SparseImageMemoryBind<'a>]>,
}
impl<'a> SparseImageMemoryBindInfo<'a> {
    #[inline(always)]
    pub fn new(
        image: &'a (impl VkHandle<Handle = brvk::VkImage> + ?Sized),
        binds: &'a [SparseImageMemoryBind<'a>],
    ) -> Self {
        unsafe {
            Self::from_raw(brvk::VkSparseImageMemoryBindInfo {
                image: image.native_ptr(),
                bindCount: binds.len() as _,
                pBinds: binds.as_ptr() as _,
            })
        }
    }

    #[inline(always)]
    pub const fn into_raw(self) -> brvk::VkSparseImageMemoryBindInfo {
        self.vk
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkSparseImageMemoryBindInfo`] struct.
    #[inline(always)]
    pub const unsafe fn from_raw(vk: brvk::VkSparseImageMemoryBindInfo) -> Self {
        Self {
            vk,
            _image: core::marker::PhantomData,
            _binds: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub const fn as_raw_ref(&self) -> &brvk::VkSparseImageMemoryBindInfo {
        &self.vk
    }

    #[inline(always)]
    pub const unsafe fn as_raw_ref_mut(&mut self) -> &mut brvk::VkSparseImageMemoryBindInfo {
        &mut self.vk
    }
}

#[repr(transparent)]
pub struct SparseImageOpaqueMemoryBindInfo<'a> {
    vk: brvk::VkSparseImageOpaqueMemoryBindInfo,
    _image: core::marker::PhantomData<&'a dyn VkHandle<Handle = brvk::VkImage>>,
    _binds: core::marker::PhantomData<&'a [SparseMemoryBind<'a>]>,
}
impl<'a> SparseImageOpaqueMemoryBindInfo<'a> {
    #[inline(always)]
    pub fn new(image: &'a (impl VkHandle<Handle = brvk::VkImage> + ?Sized), binds: &'a [SparseMemoryBind<'a>]) -> Self {
        unsafe {
            Self::from_raw(brvk::VkSparseImageOpaqueMemoryBindInfo {
                image: image.native_ptr(),
                bindCount: binds.len() as _,
                pBinds: binds.as_ptr() as _,
            })
        }
    }

    #[inline(always)]
    pub const fn into_raw(self) -> brvk::VkSparseImageOpaqueMemoryBindInfo {
        self.vk
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkSparseImageOpaqueMemoryBindInfo`] struct.
    #[inline(always)]
    pub const unsafe fn from_raw(vk: brvk::VkSparseImageOpaqueMemoryBindInfo) -> Self {
        Self {
            vk,
            _image: core::marker::PhantomData,
            _binds: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub const fn as_raw_ref(&self) -> &brvk::VkSparseImageOpaqueMemoryBindInfo {
        &self.vk
    }

    #[inline(always)]
    pub const unsafe fn as_raw_ref_mut(&mut self) -> &mut brvk::VkSparseImageOpaqueMemoryBindInfo {
        &mut self.vk
    }
}

#[repr(transparent)]
pub struct SparseMemoryBind<'a> {
    vk: brvk::VkSparseMemoryBind,
    _memory: core::marker::PhantomData<&'a dyn VkHandle<Handle = brvk::VkDeviceMemory>>,
}
impl<'a> SparseMemoryBind<'a> {
    pub fn new(
        resource_byte_range: core::ops::Range<brvk::VkDeviceSize>,
        memory: &'a (impl VkHandle<Handle = brvk::VkDeviceMemory> + ?Sized),
        memory_offset: brvk::VkDeviceSize,
    ) -> Self {
        unsafe {
            Self::from_raw(brvk::VkSparseMemoryBind {
                resourceOffset: resource_byte_range.start,
                size: resource_byte_range.end - resource_byte_range.start,
                memory: memory.native_ptr(),
                memoryOffset: memory_offset,
                flags: 0,
            })
        }
    }

    #[inline(always)]
    pub const fn into_raw(self) -> brvk::VkSparseMemoryBind {
        self.vk
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkSparseMemoryBind`] struct.
    #[inline(always)]
    pub const unsafe fn from_raw(vk: brvk::VkSparseMemoryBind) -> Self {
        Self {
            vk,
            _memory: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub const fn as_raw_ref(&self) -> &brvk::VkSparseMemoryBind {
        &self.vk
    }

    #[inline(always)]
    pub const unsafe fn as_raw_ref_mut(&mut self) -> &mut brvk::VkSparseMemoryBind {
        &mut self.vk
    }
}

#[repr(transparent)]
pub struct SparseImageMemoryBind<'a> {
    vk: brvk::VkSparseImageMemoryBind,
    _memory: core::marker::PhantomData<&'a dyn VkHandle<Handle = brvk::VkDeviceMemory>>,
}
impl<'a> SparseImageMemoryBind<'a> {
    pub fn new(
        subresource: ImageSubresource,
        offset: brvk::VkOffset3D,
        extent: brvk::VkExtent3D,
        memory: &'a (impl VkHandle<Handle = brvk::VkDeviceMemory> + ?Sized),
        memory_offset: brvk::VkDeviceSize,
    ) -> Self {
        unsafe {
            Self::from_raw(brvk::VkSparseImageMemoryBind {
                subresource: subresource.0,
                offset,
                extent,
                memory: memory.native_ptr(),
                memoryOffset: memory_offset,
                flags: 0,
            })
        }
    }

    #[inline(always)]
    pub const fn into_raw(self) -> brvk::VkSparseImageMemoryBind {
        self.vk
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkSparseImageMemoryBind`] struct.
    #[inline(always)]
    pub const unsafe fn from_raw(vk: brvk::VkSparseImageMemoryBind) -> Self {
        Self {
            vk,
            _memory: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub const fn as_raw_ref(&self) -> &brvk::VkSparseImageMemoryBind {
        &self.vk
    }

    #[inline(always)]
    pub const unsafe fn as_raw_ref_mut(&mut self) -> &mut brvk::VkSparseImageMemoryBind {
        &mut self.vk
    }
}
