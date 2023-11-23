use crate::{ffi_helper::ArrayFFIExtensions, vk::*, PipelineStageFlags, VkHandle, VulkanStructure};

pub struct TemporalSubmissionBatchResources {
    command_buffers: Vec<VkCommandBuffer>,
    wait_semaphores: Vec<VkSemaphore>,
    wait_stages: Vec<VkPipelineStageFlags>,
    signal_semaphores: Vec<VkSemaphore>,
}
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

pub trait SubmissionBatch {
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources);

    #[inline]
    fn make_info_struct(&self) -> VkSubmitInfo {
        let mut res = TemporalSubmissionBatchResources::new();
        self.collect_resources(&mut res);
        res.make_info_struct()
    }

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
impl<T: SubmissionBatch + ?Sized> SubmissionBatch for Box<T> {
    #[inline]
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources) {
        T::collect_resources(self, target)
    }
}

pub struct EmptySubmissionBatch;
impl SubmissionBatch for EmptySubmissionBatch {
    fn collect_resources(&self, _: &mut TemporalSubmissionBatchResources) {}

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
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources) {
        self.0.collect_resources(target);
        target.command_buffers.extend(self.1.iter().copied());
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
    fn collect_resources(&self, target: &mut TemporalSubmissionBatchResources) {
        self.0.collect_resources(target);
        target.wait_semaphores.extend(self.1.iter().copied());
        target.wait_stages.extend(self.2.iter().copied());
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
