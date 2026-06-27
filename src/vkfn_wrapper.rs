use ffi_helper::{opt_cstr_ptr, opt_pointer};

use crate::*;
use core::{ffi::CStr, mem::MaybeUninit, ptr::null_mut};

#[inline]
pub fn instance_layer_property_count() -> crate::Result<u32> {
    let mut n = 0;
    unsafe {
        crate::vkfn::enumerate_instance_layer_properties(&mut n, null_mut()).into_result()?;
    }

    Ok(n)
}

#[inline]
pub fn instance_layer_properties(sink: &mut [MaybeUninit<VkLayerProperties>]) -> crate::Result<u32> {
    let mut n = sink.len() as _;
    unsafe {
        crate::vkfn::enumerate_instance_layer_properties(&mut n, sink.as_mut_ptr() as _).into_result()?;
    }

    Ok(n)
}

#[inline]
pub fn instance_extension_property_count(layer_name: Option<&CStr>) -> crate::Result<u32> {
    let mut n = 0;
    unsafe {
        crate::vkfn::enumerate_instance_extension_properties(opt_cstr_ptr(layer_name), &mut n, null_mut())
            .into_result()?;
    }

    Ok(n)
}

#[inline]
pub fn instance_extension_properties(
    layer_name: Option<&CStr>,
    sink: &mut [MaybeUninit<VkExtensionProperties>],
) -> crate::Result<u32> {
    let mut n = sink.len() as _;
    unsafe {
        crate::vkfn::enumerate_instance_extension_properties(opt_cstr_ptr(layer_name), &mut n, sink.as_mut_ptr() as _)
            .into_result()?;
    }

    Ok(n)
}

/// # Safety
///
/// allocation_callbacks must be valid for the lifetime of the instance.
#[inline]
pub unsafe fn create_instance(
    info: &InstanceCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkInstance> {
    let mut h = core::mem::MaybeUninit::uninit();

    unsafe {
        crate::vkfn::create_instance(
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }
    Ok(unsafe { h.assume_init() })
}

#[inline]
pub fn physical_device_count(instance: VkHandleRef<VkInstance>) -> crate::Result<u32> {
    let mut n = 0;
    unsafe {
        crate::vkfn::enumerate_physical_devices(instance.0, &mut n, null_mut()).into_result()?;
    }

    Ok(n)
}

/// # Safety
///
/// instance must be valid for the lifetime of the returned physical devices.
#[inline]
pub unsafe fn enumerate_physical_devices(
    instance: VkHandleRef<VkInstance>,
    sink: &mut [MaybeUninit<VkPhysicalDevice>],
) -> crate::Result<u32> {
    let mut n = sink.len() as _;
    unsafe {
        crate::vkfn::enumerate_physical_devices(instance.0, &mut n, sink.as_mut_ptr().cast()).into_result()?;
    }

    Ok(n)
}

#[inline]
pub fn get_physical_device_features(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    sink: &mut MaybeUninit<PhysicalDeviceFeatures>,
) {
    unsafe { crate::vkfn::get_physical_device_features(physical_device.0, sink.as_mut_ptr()) }
}

#[inline]
pub fn get_physical_device_properties(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    sink: &mut MaybeUninit<PhysicalDeviceProperties>,
) {
    unsafe { crate::vkfn::get_physical_device_properties(physical_device.0, sink.as_mut_ptr()) }
}

#[inline]
pub fn get_physical_device_memory_properties(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    sink: &mut MaybeUninit<PhysicalDeviceMemoryProperties>,
) {
    unsafe { crate::vkfn::get_physical_device_memory_properties(physical_device.0, sink.as_mut_ptr()) }
}

#[cfg(feature = "VK_KHR_xlib_surface")]
#[inline]
pub unsafe fn get_physical_device_xlib_presentation_support(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    queue_family_index: u32,
    dpy: *mut x11::xlib::Display,
    visual_id: x11::xlib::VisualID,
) -> bool {
    unsafe {
        crate::vkfn::get_physical_device_xlib_presentation_support_khr(
            physical_device.0,
            queue_family_index,
            dpy,
            visual_id,
        ) == VK_TRUE
    }
}

#[cfg(feature = "VK_KHR_xcb_surface")]
pub unsafe fn get_physical_device_xcb_presentation_support(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    queue_family_index: u32,
    connection: *mut xcb::ffi::xcb_connection_t,
    visual_id: xcb::x::Visualid,
) -> bool {
    unsafe {
        crate::vkfn::get_physical_device_xcb_presentation_support_khr(
            physical_device.0,
            queue_family_index,
            connection,
            visual_id,
        ) == VK_TRUE
    }
}

#[cfg(feature = "VK_KHR_wayland_surface")]
#[inline]
pub unsafe fn get_physical_device_wayland_presentation_support(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    queue_family_index: u32,
    display: *mut core::ffi::c_void,
) -> bool {
    unsafe {
        crate::vkfn::get_physical_device_wayland_presentation_support_khr(
            physical_device.0,
            queue_family_index,
            display,
        ) == VK_TRUE
    }
}

#[cfg(feature = "VK_KHR_win32_surface")]
pub unsafe fn get_physical_device_win32_presentation_support(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    queue_family_index: u32,
) -> bool {
    unsafe {
        crate::vkfn::get_physical_device_win32_presentation_support_khr(physical_device.0, queue_family_index)
            == VK_TRUE
    }
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_support(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    queue_family_index: u32,
    surface: VkHandleRef<VkSurfaceKHR>,
) -> crate::Result<bool> {
    let mut sink = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_physical_device_surface_support_khr(
            physical_device.0,
            queue_family_index,
            surface.0,
            sink.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { sink.assume_init() == VK_TRUE })
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_capabilities(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    surface: VkHandleRef<VkSurfaceKHR>,
    sink: &mut MaybeUninit<SurfaceCapabilities>,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::get_physical_device_surface_capabilities_khr(physical_device.0, surface.0, sink.as_mut_ptr())
            .into_result()
            .map(drop)
    }
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_format_count(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    surface: VkHandleRef<VkSurfaceKHR>,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_physical_device_surface_formats_khr(
            physical_device.0,
            surface.0,
            v.as_mut_ptr(),
            core::ptr::null_mut(),
        )
        .into_result()?;
    }

    Ok(unsafe { v.assume_init() })
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_formats(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    surface: VkHandleRef<VkSurfaceKHR>,
    sink: &mut [MaybeUninit<SurfaceFormat>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        crate::vkfn::get_physical_device_surface_formats_khr(
            physical_device.0,
            surface.0,
            &mut v,
            sink.as_mut_ptr().cast(),
        )
    })?;

    Ok(r.with_result(v))
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_present_mode_count(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    surface: VkHandleRef<VkSurfaceKHR>,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_physical_device_surface_present_modes_khr(
            physical_device.0,
            surface.0,
            v.as_mut_ptr(),
            core::ptr::null_mut(),
        )
        .into_result()?;
    }

    Ok(unsafe { v.assume_init() })
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_present_modes(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    surface: VkHandleRef<VkSurfaceKHR>,
    sink: &mut [MaybeUninit<PresentMode>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        crate::vkfn::get_physical_device_surface_present_modes_khr(
            physical_device.0,
            surface.0,
            &mut v,
            sink.as_mut_ptr().cast(),
        )
    })?;

    Ok(r.with_result(v))
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_physical_device_display_property_count(
    physical_device: VkHandleRef<VkPhysicalDevice>,
) -> crate::Result<u32> {
    let mut count = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_physical_device_display_properties_khr(physical_device.0, count.as_mut_ptr(), null_mut())
            .into_result()?;
    }

    Ok(unsafe { count.assume_init() })
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_physical_device_display_properties(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    sink: &mut [MaybeUninit<DisplayProperties>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        crate::vkfn::get_physical_device_display_properties_khr(physical_device.0, &mut v, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(v))
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_physical_device_display_plane_property_count(
    physical_device: VkHandleRef<VkPhysicalDevice>,
) -> crate::Result<u32> {
    let mut count = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_physical_device_display_plane_properties_khr(
            physical_device.0,
            count.as_mut_ptr(),
            null_mut(),
        )
        .into_result()?;
    }

    Ok(unsafe { count.assume_init() })
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_physical_device_display_plane_properties(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    sink: &mut [MaybeUninit<DisplayPlaneProperties>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        crate::vkfn::get_physical_device_display_plane_properties_khr(
            physical_device.0,
            &mut v,
            sink.as_mut_ptr().cast(),
        )
    })?;

    Ok(r.with_result(v))
}

/// # Safety
///
/// `display` and `physical_device` must be created from the same `VkInstance`.
#[cfg(feature = "VK_KHR_display")]
#[inline]
pub unsafe fn get_display_mode_property_count(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    display: VkHandleRef<VkDisplayKHR>,
) -> crate::Result<u32> {
    let mut count = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_display_mode_properties_khr(physical_device.0, display.0, count.as_mut_ptr(), null_mut())
            .into_result()?;
    }

    Ok(unsafe { count.assume_init() })
}

/// # Safety
///
/// `display` and `physical_device` must be created from the same `VkInstance`.
#[cfg(feature = "VK_KHR_display")]
#[inline]
pub unsafe fn get_display_mode_properties(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    display: VkHandleRef<VkDisplayKHR>,
    sink: &mut [MaybeUninit<DisplayModeProperties>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        crate::vkfn::get_display_mode_properties_khr(physical_device.0, display.0, &mut v, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(v))
}

/// # Safety
///
/// `mode` and `physical_device` must be created from the same `VkInstance`.
#[cfg(feature = "VK_KHR_display")]
#[inline]
pub unsafe fn get_display_plane_capabilities(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    mode: VkHandleRefMut<VkDisplayModeKHR>,
    plane_index: u32,
    sink: &mut MaybeUninit<VkDisplayPlaneCapabilitiesKHR>,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::get_display_plane_capabilities_khr(physical_device.0, mode.0, plane_index, sink.as_mut_ptr())
            .into_result()
            .map(drop)
    }
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_display_plane_supported_display_count(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    plane_index: u32,
) -> crate::Result<u32> {
    let mut count = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_display_plane_supported_displays_khr(
            physical_device.0,
            plane_index,
            count.as_mut_ptr(),
            null_mut(),
        )
        .into_result()?;
    }

    Ok(unsafe { count.assume_init() })
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_display_plane_supported_displays(
    physical_device: VkHandleRef<VkPhysicalDevice>,
    plane_index: u32,
    sink: &mut [MaybeUninit<VkDisplayKHR>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        crate::vkfn::get_display_plane_supported_displays_khr(
            physical_device.0,
            plane_index,
            &mut v,
            sink.as_mut_ptr().cast(),
        )
    })?;

    Ok(r.with_result(v))
}

/// # Safety
///
/// `surface` must be created from `instance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn destroy_surface(
    instance: VkHandleRef<VkInstance>,
    surface: VkHandleRefMut<VkSurfaceKHR>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_surface_khr(instance.0, surface.0, opt_pointer(allocation_callbacks)) }
}

#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub fn create_swapchain(
    device: VkHandleRef<VkDevice>,
    create_info: &SwapchainCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkSwapchainKHR> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_swapchain_khr(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `swapchain` must be created from `device`.
#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn destroy_swapchain(
    device: VkHandleRef<VkDevice>,
    swapchain: VkHandleRefMut<VkSwapchainKHR>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_swapchain_khr(device.0, swapchain.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// `swapchain` must be created from `device`.
#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn get_swapchain_image_count(
    device: VkHandleRef<VkDevice>,
    swapchain: VkHandleRef<VkSwapchainKHR>,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_swapchain_images_khr(device.0, swapchain.0, v.as_mut_ptr(), null_mut()).into_result()?;
    }

    Ok(unsafe { v.assume_init() })
}

/// # Safety
///
/// `swapchain` must be created from `device`.
#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn get_swapchain_images(
    device: VkHandleRef<VkDevice>,
    swapchain: VkHandleRef<VkSwapchainKHR>,
    sink: &mut [MaybeUninit<VkImage>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        crate::vkfn::get_swapchain_images_khr(device.0, swapchain.0, &mut v, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(v))
}

/// # Safety
///
/// `swapchain`, `semaphore` and `fence` must be created from the same `device`.
#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn acquire_next_image(
    device: VkHandleRef<VkDevice>,
    swapchain: VkHandleRefMut<VkSwapchainKHR>,
    timeout: u64,
    semaphore: Option<VkHandleRefMut<VkSemaphore>>,
    fence: Option<VkHandleRefMut<VkFence>>,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::acquire_next_image_khr(
            device.0,
            swapchain.0,
            timeout,
            semaphore.map(|x| x.0),
            fence.map(|x| x.0),
            v.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { v.assume_init() })
}

#[inline]
pub fn get_device_queue(device: VkHandleRef<VkDevice>, family_index: u32, index: u32) -> VkQueue {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_device_queue(device.0, family_index, index, h.as_mut_ptr());
    }

    unsafe { h.assume_init() }
}

#[inline]
pub fn queue_wait_idle(queue: VkHandleRefMut<VkQueue>) -> crate::Result<()> {
    unsafe { crate::vkfn::queue_wait_idle(queue.0).into_result().map(drop) }
}

/// # Safety
///
/// Host access to all `VkQueue` objects created from `device` must be externally synchronized.
#[inline]
pub unsafe fn device_wait_idle(device: VkHandleRefMut<VkDevice>) -> crate::Result<()> {
    unsafe { crate::vkfn::device_wait_idle(device.0).into_result().map(drop) }
}

#[inline]
pub fn get_instance_proc_addr_pfn<F: crate::PFN>(instance: VkHandleRef<VkInstance>) -> Option<F> {
    Some(unsafe { F::from_void_fn(crate::vkfn::get_instance_proc_addr(instance.0, F::NAME_CSTR.as_ptr())?) })
}

#[inline]
pub fn get_device_proc_addr_pfn<F: crate::PFN>(device: VkHandleRef<VkDevice>) -> Option<F> {
    Some(unsafe { F::from_void_fn(crate::vkfn::get_device_proc_addr(device.0, F::NAME_CSTR.as_ptr())?) })
}

#[inline]
pub fn queue_submit(
    queue: VkHandleRefMut<VkQueue>,
    submit_info: &[SubmitInfo],
    fence: Option<VkHandleRefMut<VkFence>>,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::queue_submit(
            queue.0,
            submit_info.len() as _,
            submit_info.as_ptr().cast(),
            fence.map(|x| x.0),
        )
        .into_result()
        .map(drop)
    }
}

#[cfg(feature = "Allow1_3APIs")]
#[inline]
pub fn queue_submit2(
    queue: VkHandleRefMut<VkQueue>,
    submit_info: &[SubmitInfo2],
    fence: Option<VkHandleRefMut<VkFence>>,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::queue_submit2(
            queue.0,
            submit_info.len() as _,
            submit_info.as_ptr().cast(),
            fence.map(|x| x.0),
        )
        .into_result()
        .map(drop)
    }
}

#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub fn queue_present(queue: VkHandleRefMut<VkQueue>, present_info: &PresentInfo) -> crate::Result<PresentResult> {
    match unsafe { crate::vkfn::queue_present_khr(queue.0, core::ptr::from_ref(present_info).cast()) } {
        VK_SUCCESS => Ok(PresentResult::Success),
        VK_SUBOPTIMAL_KHR => Ok(PresentResult::Suboptimal),
        e if e.is_err() => Err(e),
        e => unreachable!("unexpected result: {e:?}"),
    }
}

#[inline]
pub fn queue_bind_sparse(
    queue: VkHandleRefMut<VkQueue>,
    infos: &[BindSparseInfo],
    fence: Option<VkHandleRefMut<VkFence>>,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::queue_bind_sparse(queue.0, infos.len() as _, infos.as_ptr().cast(), fence.map(|x| x.0))
            .into_result()
            .map(drop)
    }
}

#[inline]
pub fn create_command_pool(
    device: VkHandleRef<VkDevice>,
    create_info: &CommandPoolCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkCommandPool> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_command_pool(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `command_pool` must be created from `device`.
#[inline]
pub unsafe fn destroy_command_pool(
    device: VkHandleRef<VkDevice>,
    command_pool: VkHandleRefMut<VkCommandPool>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_command_pool(device.0, command_pool.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// command pool in `allocation_info` must be created from `device`.
#[inline]
pub unsafe fn allocate_command_buffers(
    device: VkHandleRef<VkDevice>,
    allocation_info: &CommandBufferAllocateInfo,
    sink: &mut [MaybeUninit<VkCommandBuffer>],
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::allocate_command_buffers(
            device.0,
            core::ptr::from_ref(allocation_info).cast(),
            sink.as_mut_ptr().cast(),
        )
        .into_result()
        .map(drop)
    }
}

/// # Safety
///
/// * `buffers` must be allocated from `command_pool`.
/// * `command_pool` must be created from `device`.
#[inline]
pub unsafe fn free_command_buffers(
    device: VkHandleRef<VkDevice>,
    command_pool: VkHandleRefMut<VkCommandPool>,
    buffers: &[VkHandleRefMut<VkCommandBuffer>],
) {
    unsafe { crate::vkfn::free_command_buffers(device.0, command_pool.0, buffers.len() as _, buffers.as_ptr().cast()) }
}

/// # Safety
///
/// `command_pool` must be created from `device`.
#[inline(always)]
pub unsafe fn reset_command_pool(
    device: VkHandleRef<VkDevice>,
    command_pool: VkHandleRefMut<VkCommandPool>,
    flags: CommandPoolResetFlags,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::reset_command_pool(device.0, command_pool.0, flags.bits())
            .into_result()
            .map(drop)
    }
}

/// # Safety
///
/// `command_pool` must be created from `device`.
#[cfg(feature = "Allow1_1APIs")]
#[inline(always)]
pub unsafe fn trim_command_pool(
    device: VkHandleRef<VkDevice>,
    command_pool: VkHandleRefMut<VkCommandPool>,
    flags: CommandPoolTrimFlags,
) {
    unsafe {
        crate::vkfn::trim_command_pool(device.0, command_pool.0, flags.bits());
    }
}

/// # Safety
///
/// * The command pool that `command_buffer` was allocated must be externally synchronized.
#[inline]
pub unsafe fn begin_command_buffer(
    command_buffer: VkHandleRefMut<VkCommandBuffer>,
    begin_info: &CommandBufferBeginInfo,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::begin_command_buffer(command_buffer.0, core::ptr::from_ref(begin_info).cast())
            .into_result()
            .map(drop)
    }
}

/// # Safety
///
/// * The command pool that `command_buffer` was allocated must be externally synchronized.
#[inline]
pub unsafe fn end_command_buffer(command_buffer: VkHandleRefMut<VkCommandBuffer>) -> crate::Result<()> {
    unsafe {
        crate::vkfn::end_command_buffer(command_buffer.0)
            .into_result()
            .map(drop)
    }
}

#[inline]
pub fn create_fence(
    device: VkHandleRef<VkDevice>,
    info: &FenceCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkFence> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_fence(
            device.0,
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `fence` must be created from the `device`.
#[inline]
pub unsafe fn destroy_fence(
    device: VkHandleRef<VkDevice>,
    fence: VkHandleRefMut<VkFence>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_fence(device.0, fence.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// fences in `fences` must be created from the `device`.
#[inline]
pub unsafe fn wait_for_fences(
    device: VkHandleRef<VkDevice>,
    fences: &[VkHandleRef<VkFence>],
    wait_all: bool,
    timeout: u64,
) -> crate::Result<VkResult> {
    unsafe {
        crate::vkfn::wait_for_fences(
            device.0,
            fences.len() as _,
            fences.as_ptr().cast(),
            wait_all as _,
            timeout,
        )
        .into_result()
    }
}

/// # Safety
///
/// fences in `fences` must be created from the `device`.
#[inline]
pub unsafe fn reset_fences(device: VkHandleRef<VkDevice>, fences: &[VkHandleRefMut<VkFence>]) -> crate::Result<()> {
    unsafe {
        crate::vkfn::reset_fences(device.0, fences.len() as _, fences.as_ptr().cast())
            .into_result()
            .map(drop)
    }
}

/// # Safety
///
/// `fence` must be created from the `device`.
#[inline]
pub unsafe fn get_fence_status(device: VkHandleRef<VkDevice>, fence: VkHandleRef<VkFence>) -> crate::Result<VkResult> {
    unsafe { crate::vkfn::get_fence_status(device.0, fence.0).into_result() }
}

#[inline]
pub fn create_semaphore(
    device: VkHandleRef<VkDevice>,
    create_info: &SemaphoreCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkSemaphore> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_semaphore(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// * `device` and `semaphore` must be a valid device handle and semaphore handle, respectively.
/// * `semaphore` must be created from the `device`.
#[inline]
pub unsafe fn destroy_semaphore(
    device: VkHandleRef<VkDevice>,
    semaphore: VkHandleRefMut<VkSemaphore>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_semaphore(device.0, semaphore.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn create_buffer(
    device: VkHandleRef<VkDevice>,
    create_info: &BufferCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkBuffer> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_buffer(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// * `buffer` must be created from `device`.
#[inline]
pub unsafe fn destroy_buffer(
    device: VkHandleRef<VkDevice>,
    buffer: VkHandleRefMut<VkBuffer>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_buffer(device.0, buffer.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// * `buffer` must be created from `device`.
#[inline]
pub unsafe fn get_buffer_memory_requirements(
    device: VkHandleRef<VkDevice>,
    buffer: VkHandleRef<VkBuffer>,
) -> VkMemoryRequirements {
    let mut sink = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_buffer_memory_requirements(device.0, buffer.0, sink.as_mut_ptr());

        sink.assume_init()
    }
}

/// # Safety
///
/// * `buffer` must be created from `device`.
#[cfg(feature = "Allow1_1APIs")]
#[inline]
pub unsafe fn get_buffer_memory_requirements2(
    device: VkHandleRef<VkDevice>,
    info: &BufferMemoryRequirementsInfo2<'_, impl VkHandle<Handle = VkBuffer>>,
    sink: &mut MaybeUninit<VkMemoryRequirements2>,
) {
    unsafe { crate::vkfn::get_buffer_memory_requirements2(device.0, info.as_ref(), sink.as_mut_ptr()) }
}

/// # Safety
///
/// * `buffer` and `memory` must be created from `device`.
#[inline]
pub unsafe fn bind_buffer_memory(
    device: VkHandleRef<VkDevice>,
    buffer: VkHandleRef<VkBuffer>,
    memory: VkHandleRef<VkDeviceMemory>,
    offset: DeviceSize,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::bind_buffer_memory(device.0, buffer.0, memory.0, offset)
            .into_result()
            .map(drop)
    }
}

/// # Safety
///
/// * buffers in `bind_infos` must be created from `device`.
#[cfg(feature = "Allow1_1APIs")]
#[inline]
pub unsafe fn bind_buffer_memory2(
    device: VkHandleRef<VkDevice>,
    bind_infos: &[BindBufferMemoryInfo],
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::bind_buffer_memory2(device.0, bind_infos.len() as _, bind_infos.as_ptr().cast())
            .into_result()
            .map(drop)
    }
}

#[inline]
pub fn create_image(
    device: VkHandleRef<VkDevice>,
    create_info: &ImageCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkImage> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_image(
            device.0,
            create_info as *const _ as _,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `image` must be created from the `device`.
#[inline]
pub unsafe fn destroy_image(
    device: VkHandleRef<VkDevice>,
    image: VkHandleRef<VkImage>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_image(device.0, image.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// `image` must be created from the `device`.
#[inline]
pub unsafe fn get_image_memory_requirements(
    device: VkHandleRef<VkDevice>,
    image: VkHandleRef<VkImage>,
) -> VkMemoryRequirements {
    let mut sink = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_image_memory_requirements(device.0, image.0, sink.as_mut_ptr());

        sink.assume_init()
    }
}

/// # Safety
///
/// image in `info` must be created from the `device`.
#[cfg(feature = "Allow1_1APIs")]
#[inline]
pub unsafe fn get_image_memory_requirements2(
    device: VkHandleRef<VkDevice>,
    info: &ImageMemoryRequirementsInfo2<'_, impl VkHandle<Handle = VkImage>>,
    sink: &mut MaybeUninit<VkMemoryRequirements2>,
) {
    unsafe { crate::vkfn::get_image_memory_requirements2(device.0, info.as_ref(), sink.as_mut_ptr()) }
}

/// # Safety
///
/// * `image` and `memory` must be created from `device`.
#[inline]
pub unsafe fn bind_image_memory(
    device: VkHandleRef<VkDevice>,
    image: VkHandleRef<VkImage>,
    memory: VkHandleRef<VkDeviceMemory>,
    offset: DeviceSize,
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::bind_image_memory(device.0, image.0, memory.0, offset)
            .into_result()
            .map(drop)
    }
}

/// # Safety
///
/// buffers in `bind_infos` must be created from `device`.
#[cfg(feature = "Allow1_1APIs")]
#[inline]
pub unsafe fn bind_image_memory2(
    device: VkHandleRef<VkDevice>,
    bind_infos: &[BindImageMemoryInfo],
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::bind_image_memory2(device.0, bind_infos.len() as _, bind_infos.as_ptr().cast())
            .into_result()
            .map(drop)
    }
}

#[inline]
pub fn create_sampler(
    device: VkHandleRef<VkDevice>,
    create_info: &SamplerCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkSampler> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_sampler(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `sampler` must be created from `device`.
#[inline]
pub unsafe fn destroy_sampler(
    device: VkHandleRef<VkDevice>,
    sampler: VkHandleRefMut<VkSampler>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_sampler(device.0, sampler.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn allocate_memory(
    device: VkHandleRef<VkDevice>,
    allocate_info: &MemoryAllocateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkDeviceMemory> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::allocate_memory(
            device.0,
            core::ptr::from_ref(allocate_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `memory` must be allocated from `device`.
#[inline]
pub unsafe fn free_memory(
    device: VkHandleRef<VkDevice>,
    memory: VkHandleRefMut<VkDeviceMemory>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::free_memory(device.0, memory.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// `memory` must be allocated from `device`.
#[inline]
pub unsafe fn map_memory(
    device: VkHandleRef<VkDevice>,
    memory: VkHandleRefMut<VkDeviceMemory>,
    range: core::ops::Range<DeviceSize>,
    flags: VkMemoryMapFlags,
) -> crate::Result<*mut core::ffi::c_void> {
    let mut p = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::map_memory(
            device.0,
            memory.0,
            range.start,
            range.end - range.start,
            flags,
            p.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { p.assume_init() })
}

/// # Safety
///
/// `memory` must be mapped from `device`.
#[inline]
pub unsafe fn unmap_memory(device: VkHandleRef<VkDevice>, memory: VkHandleRefMut<VkDeviceMemory>) {
    unsafe {
        crate::vkfn::unmap_memory(device.0, memory.0);
    }
}

#[inline]
pub fn invalidate_mapped_memory_ranges(
    device: VkHandleRef<VkDevice>,
    memory_ranges: &[MappedMemoryRange],
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::invalidate_mapped_memory_ranges(device.0, memory_ranges.len() as _, memory_ranges.as_ptr().cast())
            .into_result()
            .map(drop)
    }
}

#[inline]
pub fn flush_mapped_memory_ranges(
    device: VkHandleRef<VkDevice>,
    memory_ranges: &[MappedMemoryRange],
) -> crate::Result<()> {
    unsafe {
        crate::vkfn::flush_mapped_memory_ranges(device.0, memory_ranges.len() as _, memory_ranges.as_ptr().cast())
            .into_result()
            .map(drop)
    }
}

#[inline]
pub fn create_image_view(
    device: VkHandleRef<VkDevice>,
    create_info: &ImageViewCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkImageView> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_image_view(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `image_view` must be created from `device`.
#[inline]
pub unsafe fn destroy_image_view(
    device: VkHandleRef<VkDevice>,
    image_view: VkHandleRefMut<VkImageView>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_image_view(device.0, image_view.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// buffer in `create_info` must be created from `device`.
#[inline]
pub unsafe fn create_buffer_view(
    device: VkHandleRef<VkDevice>,
    create_info: &BufferViewCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkBufferView> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_buffer_view(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `buffer_view` must be created from `device`.
#[inline]
pub unsafe fn destroy_buffer_view(
    device: VkHandleRef<VkDevice>,
    buffer_view: VkHandleRefMut<VkBufferView>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_buffer_view(device.0, buffer_view.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// renderPass and attachments in `create_info` must be created from `device`.
#[inline]
pub unsafe fn create_framebuffer(
    device: VkHandleRef<VkDevice>,
    create_info: &FramebufferCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkFramebuffer> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_framebuffer(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `framebuffer` must be created from `device`.
#[inline]
pub unsafe fn destroy_framebuffer(
    device: VkHandleRef<VkDevice>,
    framebuffer: VkHandleRefMut<VkFramebuffer>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_framebuffer(device.0, framebuffer.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// `render_pass` must be created from `device`.
#[inline]
pub unsafe fn destroy_render_pass(
    device: VkHandleRef<VkDevice>,
    render_pass: VkHandleRefMut<VkRenderPass>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_render_pass(device.0, render_pass.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn create_graphics_pipelines(
    device: VkHandleRef<VkDevice>,
    pipeline_cache: Option<VkHandleRef<VkPipelineCache>>,
    create_infos: &[GraphicsPipelineCreateInfo],
    allocation_callbacks: Option<&VkAllocationCallbacks>,
    results: &mut [MaybeUninit<VkPipeline>],
) -> crate::Result<()> {
    debug_assert!(results.len() >= create_infos.len());

    unsafe {
        crate::vkfn::create_graphics_pipelines(
            device.0,
            pipeline_cache.map(|x| x.0),
            create_infos.len() as _,
            create_infos.as_ptr().cast(),
            opt_pointer(allocation_callbacks),
            results.as_mut_ptr().cast(),
        )
        .into_result()
        .map(drop)
    }
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn create_graphics_pipeline_array<const N: usize>(
    device: VkHandleRef<VkDevice>,
    pipeline_cache: Option<VkHandleRef<VkPipelineCache>>,
    create_infos: &[GraphicsPipelineCreateInfo; N],
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<[VkPipeline; N]> {
    let mut results = [MaybeUninit::uninit(); N];
    unsafe {
        create_graphics_pipelines(device, pipeline_cache, create_infos, allocation_callbacks, &mut results)?;
    }

    Ok(core::array::from_fn(|n| unsafe { results[n].assume_init() }))
}

/// # Safety
///
/// `pipeline` must be created from `device`.
#[inline]
pub unsafe fn destroy_pipeline(
    device: VkHandleRef<VkDevice>,
    pipeline: VkHandleRefMut<VkPipeline>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe { crate::vkfn::destroy_pipeline(device.0, pipeline.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn create_pipeline_layout(
    device: VkHandleRef<VkDevice>,
    create_info: &PipelineLayoutCreateInfo,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) -> crate::Result<VkPipelineLayout> {
    let mut h = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::create_pipeline_layout(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
        .into_result()?;
    }

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `pipeline_layout` must be created from `device`.
#[inline]
pub unsafe fn destroy_pipeline_layout(
    device: VkHandleRef<VkDevice>,
    pipeline_layout: VkHandleRefMut<VkPipelineLayout>,
    allocation_callbacks: Option<&VkAllocationCallbacks>,
) {
    unsafe {
        crate::vkfn::destroy_pipeline_layout(device.0, pipeline_layout.0, opt_pointer(allocation_callbacks));
    }
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn get_pipeline_cache_data_byte_length(
    device: VkHandleRef<VkDevice>,
    pipeline_cache: VkHandleRef<VkPipelineCache>,
) -> crate::Result<usize> {
    let mut len = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_pipeline_cache_data(device.0, pipeline_cache.0, len.as_mut_ptr(), null_mut()).into_result()?;
    }

    Ok(unsafe { len.assume_init() })
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn get_pipeline_cache_data(
    device: VkHandleRef<VkDevice>,
    pipeline_cache: VkHandleRef<VkPipelineCache>,
    sink: &mut [MaybeUninit<u8>],
) -> crate::Result<ArrayQueryResult<usize>> {
    let mut len = sink.len();
    let r = ArrayQueryResult::from_vk_result(unsafe {
        crate::vkfn::get_pipeline_cache_data(device.0, pipeline_cache.0, &mut len, sink.as_mut_ptr().cast())
            .into_result()?
    })?;

    Ok(r.with_result(len))
}

/// # Safety
///
/// * `semaphore` must be created with a [`VkSemaphoreType`] of [`VK_SEMAPHORE_TYPE_TIMELINE`].
/// * `semaphore` must be created from `device`.
#[cfg(feature = "Allow1_2APIs")]
#[inline]
pub unsafe fn get_semaphore_counter_value(
    device: VkHandleRef<VkDevice>,
    semaphore: VkHandleRef<VkSemaphore>,
) -> crate::Result<u64> {
    let mut value = MaybeUninit::uninit();
    unsafe {
        crate::vkfn::get_semaphore_counter_value(device.0, semaphore.0, value.as_mut_ptr()).into_result()?;
    }

    Ok(unsafe { value.assume_init() })
}

/// # Safety
///
/// * semaphore in `signal_info` must be created with a [`VkSemaphoreType`] of [`VK_SEMAPHORE_TYPE_TIMELINE`].
/// * semaphore in `signal_info` must be created from `device`.
#[cfg(feature = "Allow1_2APIs")]
#[inline]
pub unsafe fn signal_semaphore(device: VkHandleRef<VkDevice>, signal_info: &SemaphoreSignalInfo) -> crate::Result<()> {
    unsafe {
        crate::vkfn::signal_semaphore(device.0, core::ptr::from_ref(signal_info).cast())
            .into_result()
            .map(drop)
    }
}

/// # Safety
///
/// * semaphore in `wait_info` must be created with a [`VkSemaphoreType`] of [`VK_SEMAPHORE_TYPE_TIMELINE`].
/// * semaphore in `wait_info` must be created from `device`.
#[cfg(feature = "Allow1_2APIs")]
#[inline]
pub unsafe fn wait_semaphores(
    device: VkHandleRef<VkDevice>,
    wait_info: &SemaphoreWaitInfo,
    timeout: u64,
) -> crate::Result<VkResult> {
    unsafe { crate::vkfn::wait_semaphores(device.0, core::ptr::from_ref(wait_info).cast(), timeout).into_result() }
}
