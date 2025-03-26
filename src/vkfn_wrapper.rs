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
