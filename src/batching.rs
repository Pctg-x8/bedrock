use crate::{vk::*, PipelineStageFlags, VkHandle};

pub trait SubmissionBatch {
    fn make_info_struct(&self) -> VkSubmitInfo;

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
            command_buffers.iter().map(VkHandle::native_ptr).collect(),
            std::marker::PhantomData,
        )
    }

    #[inline]
    fn with_wait_semaphores<'d, Semaphore: crate::Semaphore + 'd>(
        self,
        wait_semaphores: &'d [(Semaphore, PipelineStageFlags)],
    ) -> SubmissionWithWaitSemaphores<'d, Self, Semaphore>
    where
        Self: Sized,
    {
        let (hs, fs) = wait_semaphores.iter().map(|(a, b)| (a.native_ptr(), b.0)).unzip();
        SubmissionWithWaitSemaphores(self, hs, fs, std::marker::PhantomData)
    }

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
            signal_semaphores.iter().map(VkHandle::native_ptr).collect(),
            std::marker::PhantomData,
        )
    }
}
impl SubmissionBatch for VkSubmitInfo {
    #[inline]
    fn make_info_struct(&self) -> VkSubmitInfo {
        self.clone()
    }
}

pub struct EmptySubmissionBatch;
impl SubmissionBatch for EmptySubmissionBatch {
    #[inline]
    fn make_info_struct(&self) -> VkSubmitInfo {
        Default::default()
    }
}
pub struct SubmissionWithCommandBuffers<'d, Parent: SubmissionBatch, CommandBuffer: crate::CommandBuffer + 'd>(
    Parent,
    Vec<VkCommandBuffer>,
    std::marker::PhantomData<&'d [CommandBuffer]>,
);
impl<'d, Parent, CommandBuffer> SubmissionBatch for SubmissionWithCommandBuffers<'d, Parent, CommandBuffer>
where
    Parent: SubmissionBatch,
    CommandBuffer: crate::CommandBuffer + 'd,
{
    #[inline]
    fn make_info_struct(&self) -> VkSubmitInfo {
        VkSubmitInfo {
            commandBufferCount: self.1.len() as _,
            pCommandBuffers: self.1.as_ptr(),
            ..self.0.make_info_struct()
        }
    }
}
pub struct SubmissionWithWaitSemaphores<'d, Parent: SubmissionBatch, Semaphore: crate::Semaphore + 'd>(
    Parent,
    Vec<VkSemaphore>,
    Vec<VkPipelineStageFlags>,
    std::marker::PhantomData<&'d [Semaphore]>,
);
impl<'d, Parent, Semaphore> SubmissionBatch for SubmissionWithWaitSemaphores<'d, Parent, Semaphore>
where
    Parent: SubmissionBatch,
    Semaphore: crate::Semaphore + 'd,
{
    #[inline]
    fn make_info_struct(&self) -> VkSubmitInfo {
        VkSubmitInfo {
            waitSemaphoreCount: self.1.len() as _,
            pWaitSemaphores: self.1.as_ptr(),
            pWaitDstStageMask: self.2.as_ptr(),
            ..self.0.make_info_struct()
        }
    }
}
pub struct SubmissionWithSignalSemaphores<'d, Parent: SubmissionBatch, Semaphore: crate::Semaphore + 'd>(
    Parent,
    Vec<VkSemaphore>,
    std::marker::PhantomData<&'d [Semaphore]>,
);
impl<'d, Parent, Semaphore> SubmissionBatch for SubmissionWithSignalSemaphores<'d, Parent, Semaphore>
where
    Parent: SubmissionBatch,
    Semaphore: crate::Semaphore + 'd,
{
    #[inline]
    fn make_info_struct(&self) -> VkSubmitInfo {
        VkSubmitInfo {
            signalSemaphoreCount: self.1.len() as _,
            pSignalSemaphores: self.1.as_ptr() as _,
            ..self.0.make_info_struct()
        }
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
            semaphores.iter().map(VkHandle::native_ptr).collect(),
            std::marker::PhantomData,
        )
    }

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
            semaphores.iter().map(VkHandle::native_ptr).collect(),
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
        Default::default()
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
            pBufferBinds: self.1.as_ptr(),
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
            pImageBinds: self.1.as_ptr(),
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
            pImageOpaqueBinds: self.1.as_ptr(),
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
            pWaitSemaphores: self.1.as_ptr(),
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
            pSignalSemaphores: self.1.as_ptr(),
            ..self.0.make_info_struct()
        }
    }
}
