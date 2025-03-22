use derives::implements;

use crate::{
    CommandBuffer, PipelineStageFlags, Semaphore, VkHandleRef, VkHandleRefMut, VulkanStructure,
    ffi_helper::{ArrayFFIExtensions, slice_as_ptr_empty_null},
    vk::*,
};

#[cfg(feature = "alloc")]
pub struct TemporalSubmissionBatchResources {
    command_buffers: Vec<VkCommandBuffer>,
    wait_semaphores: Vec<VkSemaphore>,
    wait_stages: Vec<VkPipelineStageFlags>,
    signal_semaphores: Vec<VkSemaphore>,
}
#[cfg(feature = "alloc")]
impl TemporalSubmissionBatchResources {
    pub const fn new() -> Self {
        Self {
            command_buffers: Vec::new(),
            wait_semaphores: Vec::new(),
            wait_stages: Vec::new(),
            signal_semaphores: Vec::new(),
        }
    }

    pub fn make_info_struct(&self) -> VkSubmitInfo {
        VkSubmitInfo {
            sType: VkSubmitInfo::TYPE,
            pNext: std::ptr::null(),
            commandBufferCount: self.command_buffers.len() as _,
            pCommandBuffers: self.command_buffers.as_ptr_empty_null(),
            waitSemaphoreCount: self.wait_semaphores.len() as _,
            pWaitSemaphores: self.wait_semaphores.as_ptr_empty_null(),
            pWaitDstStageMask: self.wait_stages.as_ptr_empty_null(),
            signalSemaphoreCount: self.signal_semaphores.len() as _,
            pSignalSemaphores: self.signal_semaphores.as_ptr_empty_null(),
        }
    }
}

#[repr(transparent)]
pub struct SubmissionBatch3<'d> {
    raw: VkSubmitInfo,
    _refs: core::marker::PhantomData<(&'d [&'d CommandBuffer], &'d [&'d Semaphore], &'d [PipelineStageFlags])>,
}
impl<'d> SubmissionBatch3<'d> {
    #[inline(always)]
    pub fn new(
        wait_semaphores: &'d [&'d Semaphore],
        wait_dst_stage_masks: &'d [PipelineStageFlags],
        command_buffers: &'d [&'d CommandBuffer],
        signal_semaphores: &'d [&'d Semaphore],
    ) -> Self {
        assert_eq!(wait_semaphores.len(), wait_dst_stage_masks.len());

        unsafe {
            Self::new_unchecked(
                wait_semaphores,
                wait_dst_stage_masks,
                command_buffers,
                signal_semaphores,
            )
        }
    }

    pub const fn new_wait_semaphore_array<const N: usize>(
        wait_semaphores: &'d [&'d Semaphore; N],
        wait_dst_stage_masks: &'d [PipelineStageFlags; N],
        command_buffers: &'d [&'d CommandBuffer],
        signal_semaphores: &'d [&'d Semaphore],
    ) -> Self {
        unsafe {
            Self::new_unchecked(
                wait_semaphores,
                wait_dst_stage_masks,
                command_buffers,
                signal_semaphores,
            )
        }
    }

    pub const unsafe fn new_unchecked(
        wait_semaphores: &'d [&'d Semaphore],
        wait_dst_stage_masks: &'d [PipelineStageFlags],
        command_buffers: &'d [&'d CommandBuffer],
        signal_semaphores: &'d [&'d Semaphore],
    ) -> Self {
        Self {
            raw: VkSubmitInfo {
                sType: VkSubmitInfo::TYPE,
                pNext: core::ptr::null(),
                waitSemaphoreCount: wait_semaphores.len() as _,
                pWaitSemaphores: slice_as_ptr_empty_null(wait_semaphores) as _,
                pWaitDstStageMask: slice_as_ptr_empty_null(wait_dst_stage_masks) as _,
                commandBufferCount: command_buffers.len() as _,
                pCommandBuffers: slice_as_ptr_empty_null(command_buffers) as _,
                signalSemaphoreCount: signal_semaphores.len() as _,
                pSignalSemaphores: slice_as_ptr_empty_null(signal_semaphores) as _,
            },
            _refs: core::marker::PhantomData,
        }
    }
}

pub trait SubmissionBatch {
    #[cfg(feature = "alloc")]
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources);

    #[cfg(feature = "alloc")]
    #[inline]
    fn make_info_struct(&self) -> VkSubmitInfo {
        let mut res = TemporalSubmissionBatchResources::new();
        self.collect_resources(&mut res);
        res.make_info_struct()
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn with_command_buffers<'d>(
        self,
        command_buffers: &'d [&'d CommandBuffer],
    ) -> SubmissionWithCommandBuffers<'d, Self>
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
    #[inline]
    fn with_wait_semaphores<'d>(
        self,
        wait_semaphores: &'d [(&'d Semaphore, PipelineStageFlags)],
    ) -> SubmissionWithWaitSemaphores<'d, Self>
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
    #[inline]
    fn with_signal_semaphores<'d>(
        self,
        signal_semaphores: &'d [&'d Semaphore],
    ) -> SubmissionWithSignalSemaphores<'d, Self>
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
impl<T: SubmissionBatch + ?Sized> SubmissionBatch for Box<T> {
    #[cfg(feature = "alloc")]
    #[inline]
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources) {
        T::collect_resources(self, target)
    }
}

#[repr(transparent)]
pub struct SubmissionBatch2<'r>(
    pub(crate) VkSubmitInfo,
    core::marker::PhantomData<(
        &'r [VkSemaphore],
        &'r [VkPipelineStageFlags],
        &'r [VkCommandBuffer],
        &'r [VkSemaphore],
    )>,
);
impl<'r> SubmissionBatch2<'r> {
    #[inline(always)]
    pub fn new(
        wait_semaphores: &'r [VkHandleRef<VkSemaphore>],
        wait_semaphore_dst_stages: &'r [VkPipelineStageFlags],
        command_buffers: &'r [VkHandleRef<VkCommandBuffer>],
        signal_semaphores: &'r [VkHandleRef<VkSemaphore>],
    ) -> Self {
        assert_eq!(wait_semaphores.len(), wait_semaphore_dst_stages.len());

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
            core::marker::PhantomData,
        )
    }

    #[implements]
    #[inline(always)]
    pub fn submit(
        self,
        queue: &mut (impl crate::QueueMut + ?Sized),
        wait_fence: Option<VkHandleRefMut<VkFence>>,
    ) -> crate::Result<()> {
        unsafe { queue.submit_raw(&[self.0], wait_fence) }
    }
}

pub struct EmptySubmissionBatch;
impl SubmissionBatch for EmptySubmissionBatch {
    #[cfg(feature = "alloc")]
    fn collect_resources(&self, _: &mut TemporalSubmissionBatchResources) {}

    #[cfg(feature = "alloc")]
    fn make_info_struct(&self) -> VkSubmitInfo {
        VkSubmitInfo {
            sType: VkSubmitInfo::TYPE,
            pNext: std::ptr::null(),
            waitSemaphoreCount: 0,
            pWaitSemaphores: std::ptr::null(),
            pWaitDstStageMask: std::ptr::null(),
            commandBufferCount: 0,
            pCommandBuffers: std::ptr::null(),
            signalSemaphoreCount: 0,
            pSignalSemaphores: std::ptr::null(),
        }
    }
}
#[cfg(feature = "alloc")]
pub struct SubmissionWithCommandBuffers<'d, Parent: SubmissionBatch>(
    Parent,
    Vec<VkCommandBuffer>,
    std::marker::PhantomData<&'d [&'d CommandBuffer]>,
);
#[cfg(feature = "alloc")]
impl<'d, Parent> SubmissionBatch for SubmissionWithCommandBuffers<'d, Parent>
where
    Parent: SubmissionBatch,
{
    #[inline]
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources) {
        self.0.collect_resources(target);
        target.command_buffers.extend(self.1.iter().copied());
    }
}
#[cfg(feature = "alloc")]
pub struct SubmissionWithWaitSemaphores<'d, Parent: SubmissionBatch>(
    Parent,
    Vec<VkSemaphore>,
    Vec<VkPipelineStageFlags>,
    std::marker::PhantomData<&'d [&'d Semaphore]>,
);
#[cfg(feature = "alloc")]
impl<'d, Parent> SubmissionBatch for SubmissionWithWaitSemaphores<'d, Parent>
where
    Parent: SubmissionBatch,
{
    #[inline]
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources) {
        self.0.collect_resources(target);
        target.wait_semaphores.extend(self.1.iter().copied());
        target.wait_stages.extend(self.2.iter().copied());
    }
}
#[cfg(feature = "alloc")]
pub struct SubmissionWithSignalSemaphores<'d, Parent: SubmissionBatch>(
    Parent,
    Vec<VkSemaphore>,
    std::marker::PhantomData<&'d [&'d Semaphore]>,
);
#[cfg(feature = "alloc")]
impl<'d, Parent> SubmissionBatch for SubmissionWithSignalSemaphores<'d, Parent>
where
    Parent: SubmissionBatch,
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
    fn with_wait_semaphores<'d>(
        self,
        semaphores: &'d [&'d Semaphore],
    ) -> SparseBindingOpBatchWithWaitSemaphores<'d, Self>
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
    fn with_signal_semaphores<'d>(
        self,
        semaphores: &'d [&'d Semaphore],
    ) -> SparseBindingOpBatchWithSignalSemaphores<'d, Self>
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
pub struct SparseBindingOpBatchWithWaitSemaphores<'d, Parent: SparseBindingOpBatch>(
    Parent,
    Vec<VkSemaphore>,
    std::marker::PhantomData<&'d [&'d Semaphore]>,
);
impl<'d, Parent: SparseBindingOpBatch> SparseBindingOpBatch for SparseBindingOpBatchWithWaitSemaphores<'d, Parent> {
    #[inline]
    fn make_info_struct(&self) -> VkBindSparseInfo {
        VkBindSparseInfo {
            waitSemaphoreCount: self.1.len() as _,
            pWaitSemaphores: self.1.as_ptr_empty_null(),
            ..self.0.make_info_struct()
        }
    }
}
pub struct SparseBindingOpBatchWithSignalSemaphores<'d, Parent: SparseBindingOpBatch>(
    Parent,
    Vec<VkSemaphore>,
    std::marker::PhantomData<&'d [&'d Semaphore]>,
);
impl<'d, Parent: SparseBindingOpBatch> SparseBindingOpBatch for SparseBindingOpBatchWithSignalSemaphores<'d, Parent> {
    #[inline]
    fn make_info_struct(&self) -> VkBindSparseInfo {
        VkBindSparseInfo {
            signalSemaphoreCount: self.1.len() as _,
            pSignalSemaphores: self.1.as_ptr_empty_null(),
            ..self.0.make_info_struct()
        }
    }
}
