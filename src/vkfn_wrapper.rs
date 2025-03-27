use ffi_helper::opt_pointer;

use crate::*;
use core::mem::MaybeUninit;

#[inline]
pub unsafe fn get_physical_device_features(
    physical_device: VkPhysicalDevice,
    sink: &mut MaybeUninit<PhysicalDeviceFeatures>,
) {
    unsafe { crate::vkfn::get_physical_device_features(physical_device, sink.as_mut_ptr()) }
}

#[inline]
pub unsafe fn get_physical_device_properties(
    physical_device: VkPhysicalDevice,
    sink: &mut MaybeUninit<PhysicalDeviceProperties>,
) {
    unsafe { crate::vkfn::get_physical_device_properties(physical_device, sink.as_mut_ptr()) }
}

#[inline]
pub unsafe fn get_physical_device_memory_properties(
    physical_device: VkPhysicalDevice,
    sink: &mut MaybeUninit<PhysicalDeviceMemoryProperties>,
) {
    unsafe { crate::vkfn::get_physical_device_memory_properties(physical_device, sink.as_mut_ptr()) }
}

#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_support(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
) -> crate::Result<bool> {
    let mut sink = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_physical_device_surface_support_khr(
            physical_device,
            queue_family_index,
            surface,
            sink.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { sink.assume_init() == VK_TRUE })
}

#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_capabilities(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    sink: &mut MaybeUninit<SurfaceCapabilities>,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::get_physical_device_surface_capabilities_khr(physical_device, surface, sink.as_mut_ptr())
            .into_result()
            .map(drop)
    }
}

#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_format_count(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_physical_device_surface_formats_khr(
            physical_device,
            surface,
            v.as_mut_ptr(),
            core::ptr::null_mut(),
        )
        .into_result()?;
    }

    Ok(unsafe { v.assume_init() })
}

#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_formats(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    sink: &mut [SurfaceFormat],
) -> crate::Result<(u32, VkResult)> {
    let mut v = sink.len() as _;
    let r = unsafe {
        crate::vkfn::get_physical_device_surface_formats_khr(physical_device, surface, &mut v, sink.as_mut_ptr())
            .into_result()?
    };

    Ok((v, r))
}

#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_present_mode_count(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_physical_device_surface_present_modes_khr(
            physical_device,
            surface,
            v.as_mut_ptr(),
            core::ptr::null_mut(),
        )
        .into_result()?;
    }

    Ok(unsafe { v.assume_init() })
}

#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_present_modes(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    sink: &mut [PresentMode],
) -> crate::Result<(u32, VkResult)> {
    let mut v = sink.len() as _;
    let r = unsafe {
        crate::vkfn::get_physical_device_surface_present_modes_khr(
            physical_device,
            surface,
            &mut v,
            sink.as_mut_ptr() as _,
        )
        .into_result()?
    };

    Ok((v, r))
}

#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn destroy_surface(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_surface_khr(instance, surface, opt_pointer(allocation_callbacks)) }
}

#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn create_swapchain(
    device: VkDevice,
    create_info: &SwapchainCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkSwapchainKHR> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_swapchain_khr(
            device,
            create_info as *const _ as _,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn destroy_swapchain(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_swapchain_khr(device, swapchain, opt_pointer(allocation_callbacks)) }
}

#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn get_swapchain_image_count(device: VkDevice, swapchain: VkSwapchainKHR) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_swapchain_images_khr(device, swapchain, v.as_mut_ptr(), core::ptr::null_mut())
            .into_result()?;
    }

    Ok(unsafe { v.assume_init() })
}

#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn get_swapchain_images(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    sink: &mut [VkImage],
) -> crate::Result<(u32, VkResult)> {
    let mut v = sink.len() as _;
    let r =
        unsafe { crate::vkfn::get_swapchain_images_khr(device, swapchain, &mut v, sink.as_mut_ptr()).into_result()? };

    Ok((v, r))
}

#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn acquire_next_image(
    device: VkDevice,
    mut swapchain: VkHandleRefMut<VkSwapchainKHR>,
    timeout: u64,
    mut semaphore: Option<VkHandleRefMut<VkSemaphore>>,
    mut fence: Option<VkHandleRefMut<VkFence>>,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::acquire_next_image_khr(
            device,
            swapchain.native_ptr_mut(),
            timeout,
            semaphore
                .as_mut()
                .map_or(VkSemaphore::NULL, VkHandleRefMut::native_ptr_mut),
            fence.as_mut().map_or(VkFence::NULL, VkHandleRefMut::native_ptr_mut),
            v.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { v.assume_init() })
}

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
pub unsafe fn queue_submit(
    mut queue: VkHandleRefMut<VkQueue>,
    submit_info: &[SubmitInfo],
    mut fence: Option<VkHandleRefMut<VkFence>>,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::queue_submit(
            queue.native_ptr_mut(),
            submit_info.len() as _,
            submit_info.as_ptr() as _,
            fence.as_mut().map_or(VkFence::NULL, VkHandleRefMut::native_ptr_mut),
        )
        .into_result()
        .map(drop)
    }
}

#[cfg(feature = "Allow1_3APIs")]
#[inline]
pub unsafe fn queue_submit2(
    mut queue: VkHandleRefMut<VkQueue>,
    submit_info: &[SubmitInfo2],
    mut fence: Option<VkHandleRefMut<VkFence>>,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::queue_submit2(
            queue.native_ptr_mut(),
            submit_info.len() as _,
            submit_info.as_ptr() as _,
            fence.as_mut().map_or(VkFence::NULL, VkHandleRefMut::native_ptr_mut),
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
    mut queue: VkHandleRefMut<VkQueue>,
    infos: &[VkBindSparseInfo],
    mut fence: Option<VkHandleRefMut<VkFence>>,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::queue_bind_sparse(
            queue.native_ptr_mut(),
            infos.len() as _,
            infos.as_ptr(),
            fence.as_mut().map_or(VkFence::NULL, VkHandleRefMut::native_ptr_mut),
        )
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

#[inline]
pub unsafe fn create_fence(
    device: VkDevice,
    info: &FenceCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkFence> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_fence(
            device,
            info as *const _ as _,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

#[inline]
pub unsafe fn destroy_fence(device: VkDevice, fence: VkFence, allocation_callbacks: Option<&VkAllocationCallbacks>) {
    unsafe { crate::vkfn::destroy_fence(device, fence, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub unsafe fn wait_for_fences(
    device: VkDevice,
    fences: &[VkFence],
    wait_all: bool,
    timeout: u64,
) -> crate::Result<VkResult> {
    unsafe {
        crate::vkfn::wait_for_fences(device, fences.len() as _, fences.as_ptr(), wait_all as _, timeout).into_result()
    }
}

#[inline]
pub unsafe fn reset_fences(device: VkDevice, fences: &[VkHandleRefMut<VkFence>]) -> crate::Result<()> {
    unsafe {
        crate::vkfn::reset_fences(device, fences.len() as _, fences.as_ptr() as _)
            .into_result()
            .map(drop)
    }
}

#[inline]
pub unsafe fn get_fence_status(device: VkDevice, fence: VkFence) -> crate::Result<VkResult> {
    unsafe { crate::vkfn::get_fence_status(device, fence).into_result() }
}

#[inline]
pub unsafe fn create_semaphore(
    device: VkDevice,
    create_info: &SemaphoreCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkSemaphore> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_semaphore(
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
pub unsafe fn destroy_semaphore(
    device: VkDevice,
    semaphore: VkSemaphore,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_semaphore(device, semaphore, opt_pointer(allocation_callbacks)) }
}
