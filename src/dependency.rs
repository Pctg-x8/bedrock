use crate::*;

use self::ffi_helper::ArrayFFIExtensions;

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MemoryBarrier2(VkMemoryBarrier2KHR);
#[cfg(feature = "VK_KHR_synchronization2")]
impl MemoryBarrier2 {
    pub const fn new() -> Self {
        Self(VkMemoryBarrier2KHR {
            sType: VkMemoryBarrier2KHR::TYPE,
            pNext: core::ptr::null(),
            srcStageMask: VK_PIPELINE_STAGE_2_NONE_KHR,
            srcAccessMask: VK_ACCESS_2_NONE_KHR,
            dstStageMask: VK_PIPELINE_STAGE_2_NONE_KHR,
            dstAccessMask: VK_ACCESS_2_NONE_KHR,
        })
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BufferMemoryBarrier2<'b>(
    VkBufferMemoryBarrier2KHR,
    core::marker::PhantomData<&'b dyn VkHandle<Handle = VkBuffer>>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'b> BufferMemoryBarrier2<'b> {
    pub fn new(
        buffer: &'b (impl VkHandle<Handle = VkBuffer> + ?Sized),
        offset: VkDeviceSize,
        size: VkDeviceSize,
    ) -> Self {
        Self(
            VkBufferMemoryBarrier2KHR {
                sType: VkBufferMemoryBarrier2KHR::TYPE,
                pNext: core::ptr::null(),
                srcStageMask: VK_PIPELINE_STAGE_2_NONE_KHR,
                srcAccessMask: VK_ACCESS_2_NONE_KHR,
                dstStageMask: VK_PIPELINE_STAGE_2_NONE_KHR,
                dstAccessMask: VK_ACCESS_2_NONE_KHR,
                srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                buffer: buffer.native_ptr(),
                offset,
                size,
            },
            core::marker::PhantomData,
        )
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImageMemoryBarrier2<'r>(
    VkImageMemoryBarrier2KHR,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = VkImage>>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'r> ImageMemoryBarrier2<'r> {
    pub fn new(
        image: &'r (impl VkHandle<Handle = VkImage> + ?Sized),
        subresource_range: VkImageSubresourceRange,
    ) -> Self {
        Self(
            VkImageMemoryBarrier2KHR {
                sType: VkImageMemoryBarrier2KHR::TYPE,
                pNext: core::ptr::null(),
                srcStageMask: VK_PIPELINE_STAGE_2_NONE_KHR,
                srcAccessMask: VK_ACCESS_2_NONE_KHR,
                dstStageMask: VK_PIPELINE_STAGE_2_NONE_KHR,
                dstAccessMask: VK_ACCESS_2_NONE_KHR,
                oldLayout: VK_IMAGE_LAYOUT_UNDEFINED,
                newLayout: VK_IMAGE_LAYOUT_UNDEFINED,
                srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                image: image.native_ptr(),
                subresourceRange: subresource_range,
            },
            core::marker::PhantomData,
        )
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

    pub const fn transferring_queue_family(mut self, from: u32, to: u32) -> Self {
        self.0.srcQueueFamilyIndex = from;
        self.0.dstQueueFamilyIndex = to;
        self
    }
}

#[cfg(feature = "VK_KHR_synchronization2")]
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DependencyInfo<'b, 'r>(
    VkDependencyInfoKHR,
    core::marker::PhantomData<(
        &'b [MemoryBarrier2],
        &'b [BufferMemoryBarrier2<'r>],
        &'b [ImageMemoryBarrier2<'r>],
    )>,
);
#[cfg(feature = "VK_KHR_synchronization2")]
impl<'b, 'r> DependencyInfo<'b, 'r> {
    pub fn new(
        memory_barriers: &'b [MemoryBarrier2],
        buffer_memory_barriers: &'b [BufferMemoryBarrier2<'r>],
        image_memory_barriers: &'b [ImageMemoryBarrier2<'r>],
    ) -> Self {
        Self(
            VkDependencyInfoKHR {
                sType: VkDependencyInfoKHR::TYPE,
                pNext: core::ptr::null(),
                dependencyFlags: 0,
                memoryBarrierCount: memory_barriers.len() as _,
                pMemoryBarriers: memory_barriers.as_ptr_empty_null() as _,
                bufferMemoryBarrierCount: buffer_memory_barriers.len() as _,
                pBufferMemoryBarriers: buffer_memory_barriers.as_ptr_empty_null() as _,
                imageMemoryBarrierCount: image_memory_barriers.len() as _,
                pImageMemoryBarriers: image_memory_barriers.as_ptr_empty_null() as _,
            },
            core::marker::PhantomData,
        )
    }

    pub const fn by_region(mut self) -> Self {
        self.0.dependencyFlags |= VK_DEPENDENCY_BY_REGION_BIT;
        self
    }
}
