use bedrock_vk::{self as brvk, TypedVulkanStructure};

use crate::*;
#[cfg(feature = "VK_KHR_synchronization2")]
use ffi_helper::slice_as_ptr_empty_null;

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct MemoryBarrier2(brvk::VkMemoryBarrier2KHR);
#[cfg(feature = "VK_KHR_synchronization2")]
impl MemoryBarrier2 {
    pub const fn new() -> Self {
        Self(brvk::VkMemoryBarrier2KHR {
            sType: brvk::VkMemoryBarrier2KHR::TYPE,
            pNext: core::ptr::null(),
            srcStageMask: brvk::VK_PIPELINE_STAGE_2_NONE_KHR,
            srcAccessMask: brvk::VK_ACCESS_2_NONE_KHR,
            dstStageMask: brvk::VK_PIPELINE_STAGE_2_NONE_KHR,
            dstAccessMask: brvk::VK_ACCESS_2_NONE_KHR,
        })
    }

    pub const fn from(mut self, stage: PipelineStageFlags2, access: AccessFlags2) -> Self {
        self.0.srcStageMask = stage.0;
        self.0.srcAccessMask = access.0;
        self
    }

    pub const fn to(mut self, stage: PipelineStageFlags2, access: AccessFlags2) -> Self {
        self.0.dstStageMask = stage.0;
        self.0.dstAccessMask = access.0;
        self
    }

    pub const fn of_execution(mut self, src: PipelineStageFlags2, dst: PipelineStageFlags2) -> Self {
        self.0.srcStageMask = src.0;
        self.0.dstStageMask = dst.0;
        self
    }

    pub const fn of_memory(mut self, src: AccessFlags2, dst: AccessFlags2) -> Self {
        self.0.srcAccessMask = src.0;
        self.0.dstAccessMask = dst.0;
        self
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct BufferMemoryBarrier2<'b>(
    brvk::VkBufferMemoryBarrier2KHR,
    core::marker::PhantomData<&'b dyn VkHandle<Handle = brvk::VkBuffer>>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'b> BufferMemoryBarrier2<'b> {
    pub fn new(
        buffer: &'b (impl VkHandle<Handle = brvk::VkBuffer> + ?Sized),
        range: core::ops::Range<DeviceSize>,
    ) -> Self {
        Self(
            brvk::VkBufferMemoryBarrier2KHR {
                sType: brvk::VkBufferMemoryBarrier2KHR::TYPE,
                pNext: core::ptr::null(),
                srcStageMask: brvk::VK_PIPELINE_STAGE_2_NONE_KHR,
                srcAccessMask: brvk::VK_ACCESS_2_NONE_KHR,
                dstStageMask: brvk::VK_PIPELINE_STAGE_2_NONE_KHR,
                dstAccessMask: brvk::VK_ACCESS_2_NONE_KHR,
                srcQueueFamilyIndex: brvk::VK_QUEUE_FAMILY_IGNORED,
                dstQueueFamilyIndex: brvk::VK_QUEUE_FAMILY_IGNORED,
                buffer: buffer.native_ptr(),
                offset: range.start,
                size: range.end - range.start,
            },
            core::marker::PhantomData,
        )
    }

    pub const fn from(mut self, stage: PipelineStageFlags2, access: AccessFlags2) -> Self {
        self.0.srcStageMask = stage.0;
        self.0.srcAccessMask = access.0;
        self
    }

    pub const fn to(mut self, stage: PipelineStageFlags2, access: AccessFlags2) -> Self {
        self.0.dstStageMask = stage.0;
        self.0.dstAccessMask = access.0;
        self
    }

    pub const fn of_execution(mut self, src: PipelineStageFlags2, dst: PipelineStageFlags2) -> Self {
        self.0.srcStageMask = src.0;
        self.0.dstStageMask = dst.0;
        self
    }

    pub const fn of_memory(mut self, src: AccessFlags2, dst: AccessFlags2) -> Self {
        self.0.srcAccessMask = src.0;
        self.0.dstAccessMask = dst.0;
        self
    }

    pub const fn transferring_queue_family(mut self, from: u32, to: u32) -> Self {
        self.0.srcQueueFamilyIndex = from;
        self.0.dstQueueFamilyIndex = to;
        self
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Clone)]
pub struct ImageMemoryBarrier2<'r>(
    brvk::VkImageMemoryBarrier2KHR,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = brvk::VkImage>>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'r> ImageMemoryBarrier2<'r> {
    pub fn new(
        image: &'r (impl VkHandle<Handle = brvk::VkImage> + ?Sized),
        subresource_range: ImageSubresourceRange,
    ) -> Self {
        Self(
            brvk::VkImageMemoryBarrier2KHR {
                sType: brvk::VkImageMemoryBarrier2KHR::TYPE,
                pNext: core::ptr::null(),
                srcStageMask: brvk::VK_PIPELINE_STAGE_2_NONE_KHR,
                srcAccessMask: brvk::VK_ACCESS_2_NONE_KHR,
                dstStageMask: brvk::VK_PIPELINE_STAGE_2_NONE_KHR,
                dstAccessMask: brvk::VK_ACCESS_2_NONE_KHR,
                oldLayout: brvk::VK_IMAGE_LAYOUT_UNDEFINED,
                newLayout: brvk::VK_IMAGE_LAYOUT_UNDEFINED,
                srcQueueFamilyIndex: brvk::VK_QUEUE_FAMILY_IGNORED,
                dstQueueFamilyIndex: brvk::VK_QUEUE_FAMILY_IGNORED,
                image: image.native_ptr(),
                subresourceRange: subresource_range.0,
            },
            core::marker::PhantomData,
        )
    }

    pub const fn from(mut self, stage: PipelineStageFlags2, access: AccessFlags2) -> Self {
        self.0.srcStageMask = stage.0;
        self.0.srcAccessMask = access.0;
        self
    }

    pub const fn to(mut self, stage: PipelineStageFlags2, access: AccessFlags2) -> Self {
        self.0.dstStageMask = stage.0;
        self.0.dstAccessMask = access.0;
        self
    }

    pub const fn of_execution(mut self, src: PipelineStageFlags2, dst: PipelineStageFlags2) -> Self {
        self.0.srcStageMask = src.0;
        self.0.dstStageMask = dst.0;
        self
    }

    pub const fn of_memory(mut self, src: AccessFlags2, dst: AccessFlags2) -> Self {
        self.0.srcAccessMask = src.0;
        self.0.dstAccessMask = dst.0;
        self
    }

    pub const fn transferring_layout(mut self, from: ImageLayout, to: ImageLayout) -> Self {
        self.0.oldLayout = from as _;
        self.0.newLayout = to as _;
        self
    }
    pub const fn transit_from(self, trans: LayoutTransition) -> Self {
        self.transferring_layout(trans.from, trans.to)
    }
    pub const fn transit_to(self, trans: LayoutTransition) -> Self {
        self.transferring_layout(trans.from, trans.to)
    }

    pub const fn transferring_queue_family(mut self, from: u32, to: u32) -> Self {
        self.0.srcQueueFamilyIndex = from;
        self.0.dstQueueFamilyIndex = to;
        self
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct DependencyInfo<'b, 'r>(
    brvk::VkDependencyInfoKHR,
    core::marker::PhantomData<(
        &'b [MemoryBarrier2],
        &'b [BufferMemoryBarrier2<'r>],
        &'b [ImageMemoryBarrier2<'r>],
    )>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'b, 'r> DependencyInfo<'b, 'r> {
    pub const fn new(
        memory_barriers: &'b [MemoryBarrier2],
        buffer_memory_barriers: &'b [BufferMemoryBarrier2<'r>],
        image_memory_barriers: &'b [ImageMemoryBarrier2<'r>],
    ) -> Self {
        Self(
            brvk::VkDependencyInfoKHR {
                sType: brvk::VkDependencyInfoKHR::TYPE,
                pNext: core::ptr::null(),
                dependencyFlags: 0,
                memoryBarrierCount: memory_barriers.len() as _,
                pMemoryBarriers: slice_as_ptr_empty_null(memory_barriers).cast(),
                bufferMemoryBarrierCount: buffer_memory_barriers.len() as _,
                pBufferMemoryBarriers: slice_as_ptr_empty_null(buffer_memory_barriers).cast(),
                imageMemoryBarrierCount: image_memory_barriers.len() as _,
                pImageMemoryBarriers: slice_as_ptr_empty_null(image_memory_barriers).cast(),
            },
            core::marker::PhantomData,
        )
    }

    pub const fn by_region(mut self) -> Self {
        self.0.dependencyFlags |= brvk::VK_DEPENDENCY_BY_REGION_BIT;
        self
    }
}
