use ffi_helper::opt_pointer;

use crate::*;
use core::mem::MaybeUninit;

#[inline]
pub unsafe fn get_device_queue(device: VkDevice, family_index: u32, index: u32) -> VkQueue {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_device_queue(device, family_index, index, h.as_mut_ptr());
    }

    unsafe { h.assume_init() }
}

#[inline]
pub unsafe fn queue_wait_idle(queue: VkQueue) -> crate::Result<()> {
    unsafe { crate::vkfn::queue_wait_idle(queue).into_result().map(drop) }
}

#[inline]
pub unsafe fn device_wait_idle(device: VkDevice) -> crate::Result<()> {
    unsafe { crate::vkfn::device_wait_idle(device).into_result().map(drop) }
}

#[inline]
pub unsafe fn queue_submit(queue: VkQueue, submit_info: &[SubmitInfo], fence: Option<VkFence>) -> crate::Result<()> {
    unsafe {
        crate::vkfn::queue_submit(
            queue,
            submit_info.len() as _,
            submit_info.as_ptr() as _,
            fence.unwrap_or(VkFence::NULL),
        )
        .into_result()
        .map(drop)
    }
}

#[cfg(feature = "Allow1_3APIs")]
#[inline]
pub unsafe fn queue_submit2(queue: VkQueue, submit_info: &[SubmitInfo2], fence: Option<VkFence>) -> crate::Result<()> {
    unsafe {
        crate::vkfn::queue_submit2(
            queue,
            submit_info.len() as _,
            submit_info.as_ptr() as _,
            fence.unwrap_or(VkFence::NULL),
        )
        .into_result()
        .map(drop)
    }
}

#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn queue_present(queue: VkQueue, present_info: &PresentInfo) -> crate::Result<VkResult> {
    unsafe { crate::vkfn::queue_present_khr(queue, present_info as *const _ as _).into_result() }
}

#[inline]
pub unsafe fn queue_bind_sparse(
    queue: VkQueue,
    infos: &[VkBindSparseInfo],
    fence: Option<VkFence>,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::queue_bind_sparse(queue, infos.len() as _, infos.as_ptr(), fence.unwrap_or(VkFence::NULL))
            .into_result()
            .map(drop)
    }
}

#[inline]
pub unsafe fn create_command_pool(
    device: VkDevice,
    create_info: &CommandPoolCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkCommandPool> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_command_pool(
            device,
            create_info as *const _ as _,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

#[inline]
pub unsafe fn destroy_command_pool(
    device: VkDevice,
    command_pool: VkCommandPool,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_command_pool(device, command_pool, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub unsafe fn allocate_command_buffers(
    device: VkDevice,
    allocation_info: &CommandBufferAllocateInfo,
    sink: &mut [VkCommandBuffer],
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::allocate_command_buffers(device, allocation_info as *const _ as _, sink.as_mut_ptr())
            .into_result()
            .map(drop)
    }
}

#[inline]
pub unsafe fn free_command_buffers(device: VkDevice, command_pool: VkCommandPool, buffers: &[VkCommandBuffer]) {
    unsafe { crate::vkfn::free_command_buffers(device, command_pool, buffers.len() as _, buffers.as_ptr()) }
}

#[inline]
pub unsafe fn reset_command_pool(
    device: VkDevice,
    command_pool: VkCommandPool,
    flags: CommandPoolResetFlags,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::reset_command_pool(device, command_pool, flags.bits())
            .into_result()
            .map(drop)
    }
}

#[inline]
pub unsafe fn begin_command_buffer(
    command_buffer: VkCommandBuffer,
    begin_info: &CommandBufferBeginInfo,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::begin_command_buffer(command_buffer, begin_info as *const _ as _)
            .into_result()
            .map(drop)
    }
}

#[inline]
pub unsafe fn end_command_buffer(command_buffer: VkCommandBuffer) -> crate::Result<()> {
    unsafe { crate::vkfn::end_command_buffer(command_buffer).into_result().map(drop) }
}
