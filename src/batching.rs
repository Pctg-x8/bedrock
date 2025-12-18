use crate::ffi_helper::{ArrayFFIExtensions, slice_as_ptr_empty_null};
use crate::*;
use core::marker::PhantomData;

#[cfg(feature = "alloc")]
#[deprecated = "old batching library"]
pub struct TemporalSubmissionBatchResources {
    command_buffers: Vec<VkCommandBuffer>,
    wait_semaphores: Vec<VkSemaphore>,
    wait_stages: Vec<VkPipelineStageFlags>,
    signal_semaphores: Vec<VkSemaphore>,
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

    pub fn make_info_struct(&self) -> SubmitInfo {
        unsafe {
            SubmitInfo::from_raw(VkSubmitInfo {
                sType: VkSubmitInfo::TYPE,
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
    VkSubmitInfo,
    PhantomData<(
        Option<&'n dyn VulkanStructure>,
        &'rs [VkHandleRef<'r, VkSemaphore>],
        &'rs [PipelineStageFlags],
        &'rs [VkHandleRef<'r, VkCommandBuffer>],
        &'rs [VkHandleRef<'r, VkSemaphore>],
    )>,
);
impl<'r, 'rs, 'n> SubmitInfo<'r, 'rs, 'n> {
    pub unsafe fn new_unchecked(
        wait_semaphores: &'rs [VkHandleRef<'r, VkSemaphore>],
        wait_semaphore_dst_stages: &'rs [PipelineStageFlags],
        command_buffers: &'rs [VkHandleRef<'r, VkCommandBuffer>],
        signal_semaphores: &'rs [VkHandleRef<'r, VkSemaphore>],
    ) -> Self {
        Self(
            VkSubmitInfo {
                sType: VkSubmitInfo::TYPE,
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
        wait_semaphores: &'rs [VkHandleRef<'r, VkSemaphore>],
        wait_semaphore_dst_stages: &'rs [PipelineStageFlags],
        command_buffers: &'rs [VkHandleRef<'r, VkCommandBuffer>],
        signal_semaphores: &'rs [VkHandleRef<'r, VkSemaphore>],
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
        wait_semaphores: &'rs [VkHandleRef<'r, VkSemaphore>; NW],
        wait_semaphore_dst_stages: &'rs [PipelineStageFlags; NW],
        command_buffers: &'rs [VkHandleRef<'r, VkCommandBuffer>; NC],
        signal_semaphores: &'rs [VkHandleRef<'r, VkSemaphore>; NS],
    ) -> Self {
        Self(
            VkSubmitInfo {
                sType: VkSubmitInfo::TYPE,
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

    pub const unsafe fn from_raw(raw: VkSubmitInfo) -> Self {
        Self(raw, PhantomData)
    }

    pub const fn into_raw(self) -> VkSubmitInfo {
        self.0
    }

    pub const fn with_next(mut self, next: &'n (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }
}

#[cfg(feature = "VK_KHR_timeline_semaphore")]
#[repr(transparent)]
pub struct TimelineSemaphoreSubmitInfo<'d, 'xs>(
    VkTimelineSemaphoreSubmitInfoKHR,
    core::marker::PhantomData<(Option<&'d dyn VulkanStructure>, &'xs [u64])>,
);
#[cfg(feature = "VK_KHR_timeline_semaphore")]
impl<'d, 'xs> TimelineSemaphoreSubmitInfo<'d, 'xs> {
    pub const fn new(wait_semaphore_values: &'xs [u64], signal_semaphore_values: &'xs [u64]) -> Self {
        Self(
            VkTimelineSemaphoreSubmitInfoKHR {
                sType: VkTimelineSemaphoreSubmitInfoKHR::TYPE,
                pNext: core::ptr::null(),
                waitSemaphoreValueCount: wait_semaphore_values.len() as _,
                pWaitSemaphoreValues: slice_as_ptr_empty_null(wait_semaphore_values) as _,
                signalSemaphoreValueCount: signal_semaphore_values.len() as _,
                pSignalSemaphoreValues: slice_as_ptr_empty_null(signal_semaphore_values) as _,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkTimelineSemaphoreSubmitInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkTimelineSemaphoreSubmitInfoKHR {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}
#[cfg(feature = "VK_KHR_timeline_semaphore")]
unsafe impl VulkanStructure for TimelineSemaphoreSubmitInfo<'_, '_> {
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanStructure {
        self.0.as_generic()
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
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
    Vec<VkCommandBuffer>,
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
    Vec<VkSemaphore>,
    Vec<VkPipelineStageFlags>,
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
    Vec<VkSemaphore>,
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
    fn make_info_struct(&self) -> VkBindSparseInfo;

    #[inline]
    fn with_buffer_binds<'d>(
        self,
        buffer_binds: &'d [VkSparseBufferMemoryBindInfo],
    ) -> SparseBindingOpBatchWithBufferBinds<'d, Self>
    where
        Self: Sized,
    {
        SparseBindingOpBatchWithBufferBinds(self, buffer_binds)
    }

    #[inline]
    fn with_image_binds<'d>(
        self,
        buffer_binds: &'d [VkSparseImageMemoryBindInfo],
    ) -> SparseBindingOpBatchWithImageBinds<'d, Self>
    where
        Self: Sized,
    {
        SparseBindingOpBatchWithImageBinds(self, buffer_binds)
    }

    #[inline]
    fn with_image_opaque_binds<'d>(
        self,
        buffer_binds: &'d [VkSparseImageOpaqueMemoryBindInfo],
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
impl SparseBindingOpBatch for VkBindSparseInfo {
    #[inline]
    fn make_info_struct(&self) -> VkBindSparseInfo {
        self.clone()
    }
}

pub struct EmptyBindingOpBatch;
impl SparseBindingOpBatch for EmptyBindingOpBatch {
    #[inline]
    fn make_info_struct(&self) -> VkBindSparseInfo {
        VkBindSparseInfo {
            sType: VkBindSparseInfo::TYPE,
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
        }
    }
}
pub struct SparseBindingOpBatchWithBufferBinds<'d, Parent: SparseBindingOpBatch>(
    Parent,
    &'d [VkSparseBufferMemoryBindInfo],
);
impl<'d, Parent: SparseBindingOpBatch> SparseBindingOpBatch for SparseBindingOpBatchWithBufferBinds<'d, Parent> {
    #[inline]
    fn make_info_struct(&self) -> VkBindSparseInfo {
        VkBindSparseInfo {
            bufferBindCount: self.1.len() as _,
            pBufferBinds: self.1.as_ptr_empty_null(),
            ..self.0.make_info_struct()
        }
    }
}
pub struct SparseBindingOpBatchWithImageBinds<'d, Parent: SparseBindingOpBatch>(
    Parent,
    &'d [VkSparseImageMemoryBindInfo],
);
impl<'d, Parent: SparseBindingOpBatch> SparseBindingOpBatch for SparseBindingOpBatchWithImageBinds<'d, Parent> {
    #[inline]
    fn make_info_struct(&self) -> VkBindSparseInfo {
        VkBindSparseInfo {
            imageBindCount: self.1.len() as _,
            pImageBinds: self.1.as_ptr_empty_null(),
            ..self.0.make_info_struct()
        }
    }
}
pub struct SparseBindingOpBatchWithImageOpaqueBinds<'d, Parent: SparseBindingOpBatch>(
    Parent,
    &'d [VkSparseImageOpaqueMemoryBindInfo],
);
impl<'d, Parent: SparseBindingOpBatch> SparseBindingOpBatch for SparseBindingOpBatchWithImageOpaqueBinds<'d, Parent> {
    #[inline]
    fn make_info_struct(&self) -> VkBindSparseInfo {
        VkBindSparseInfo {
            imageOpaqueBindCount: self.1.len() as _,
            pImageOpaqueBinds: self.1.as_ptr_empty_null(),
            ..self.0.make_info_struct()
        }
    }
}
pub struct SparseBindingOpBatchWithWaitSemaphores<'d, Parent: SparseBindingOpBatch, Semaphore: crate::Semaphore + 'd>(
    Parent,
    Vec<VkSemaphore>,
    std::marker::PhantomData<&'d [Semaphore]>,
);
impl<'d, Parent: SparseBindingOpBatch, Semaphore: crate::Semaphore + 'd> SparseBindingOpBatch
    for SparseBindingOpBatchWithWaitSemaphores<'d, Parent, Semaphore>
{
    #[inline]
    fn make_info_struct(&self) -> VkBindSparseInfo {
        VkBindSparseInfo {
            waitSemaphoreCount: self.1.len() as _,
            pWaitSemaphores: self.1.as_ptr_empty_null(),
            ..self.0.make_info_struct()
        }
    }
}
pub struct SparseBindingOpBatchWithSignalSemaphores<'d, Parent: SparseBindingOpBatch, Semaphore: crate::Semaphore + 'd>(
    Parent,
    Vec<VkSemaphore>,
    std::marker::PhantomData<&'d [Semaphore]>,
);
impl<'d, Parent: SparseBindingOpBatch, Semaphore: crate::Semaphore + 'd> SparseBindingOpBatch
    for SparseBindingOpBatchWithSignalSemaphores<'d, Parent, Semaphore>
{
    #[inline]
    fn make_info_struct(&self) -> VkBindSparseInfo {
        VkBindSparseInfo {
            signalSemaphoreCount: self.1.len() as _,
            pSignalSemaphores: self.1.as_ptr_empty_null(),
            ..self.0.make_info_struct()
        }
    }
}
