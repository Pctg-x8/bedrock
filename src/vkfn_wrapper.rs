use bedrock_vk as brvk;

use crate::{error::translate_vk_result, *};
use core::{ffi::CStr, mem::MaybeUninit, ptr::null_mut};
use ffi_helper::{opt_cstr_ptr, opt_pointer};

#[inline]
pub fn instance_layer_property_count() -> crate::Result<u32> {
    let mut n = 0;
    translate_vk_result(unsafe { brvk::fns::enumerate_instance_layer_properties(&mut n, null_mut()) })?;

    Ok(n)
}

#[inline]
pub fn instance_layer_properties(
    sink: &mut [MaybeUninit<brvk::VkLayerProperties>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut n = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::enumerate_instance_layer_properties(&mut n, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(n))
}

#[inline]
pub fn instance_extension_property_count(layer_name: Option<&CStr>) -> crate::Result<u32> {
    let mut n = 0;
    translate_vk_result(unsafe {
        brvk::fns::enumerate_instance_extension_properties(opt_cstr_ptr(layer_name), &mut n, null_mut())
    })?;

    Ok(n)
}

#[inline]
pub fn instance_extension_properties(
    layer_name: Option<&CStr>,
    sink: &mut [MaybeUninit<brvk::VkExtensionProperties>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut n = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::enumerate_instance_extension_properties(opt_cstr_ptr(layer_name), &mut n, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(n))
}

#[inline]
pub fn create_instance(
    info: &InstanceCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkInstance> {
    let mut h = core::mem::MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_instance(
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

#[inline(always)]
pub fn destroy_instance(
    instance: VkHandleRefMut<brvk::VkInstance>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_instance(instance.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn physical_device_count(instance: VkHandleRef<brvk::VkInstance>) -> crate::Result<u32> {
    let mut n = 0;
    translate_vk_result(unsafe { brvk::fns::enumerate_physical_devices(instance.0, &mut n, null_mut()) })?;

    Ok(n)
}

#[inline]
pub fn enumerate_physical_devices(
    instance: VkHandleRef<brvk::VkInstance>,
    sink: &mut [MaybeUninit<brvk::VkPhysicalDevice>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut n = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::enumerate_physical_devices(instance.0, &mut n, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(n))
}

#[inline]
pub fn device_layer_property_count(physical_device: VkHandleRef<brvk::VkPhysicalDevice>) -> crate::Result<u32> {
    let mut sink = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::enumerate_device_layer_properties(physical_device.0, sink.as_mut_ptr(), null_mut())
    })?;

    Ok(unsafe { sink.assume_init() })
}

#[inline]
pub fn enumerate_device_layer_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    sink: &mut [MaybeUninit<brvk::VkLayerProperties>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut n = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::enumerate_device_layer_properties(physical_device.0, &mut n, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(n))
}

#[inline]
pub fn device_extension_property_count(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    layer_name: Option<&CStr>,
) -> crate::Result<u32> {
    let mut sink = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::enumerate_device_extension_properties(
            physical_device.0,
            opt_cstr_ptr(layer_name),
            sink.as_mut_ptr(),
            null_mut(),
        )
    })?;

    Ok(unsafe { sink.assume_init() })
}

#[inline]
pub fn enumerate_device_extension_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    layer_name: Option<&CStr>,
    sink: &mut [MaybeUninit<brvk::VkExtensionProperties>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut n = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::enumerate_device_extension_properties(
            physical_device.0,
            opt_cstr_ptr(layer_name),
            &mut n,
            sink.as_mut_ptr().cast(),
        )
    })?;

    Ok(r.with_result(n))
}

#[inline(always)]
pub fn get_physical_device_features(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    sink: &mut MaybeUninit<PhysicalDeviceFeatures>,
) {
    unsafe { brvk::fns::get_physical_device_features(physical_device.0, sink.as_mut_ptr()) }
}

#[cfg(feature = "Allow1_1APIs")]
#[inline(always)]
pub fn get_physical_device_features2(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    sink: &mut MaybeUninit<PhysicalDeviceFeatures2>,
) {
    unsafe { brvk::fns::get_physical_device_features2(physical_device.0, sink.as_mut_ptr().cast()) }
}

#[inline(always)]
pub fn get_physical_device_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    sink: &mut MaybeUninit<PhysicalDeviceProperties>,
) {
    unsafe { brvk::fns::get_physical_device_properties(physical_device.0, sink.as_mut_ptr()) }
}

#[cfg(feature = "Allow1_1APIs")]
#[inline(always)]
pub fn get_physical_device_properties2(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    sink: &mut MaybeUninit<brvk::VkPhysicalDeviceProperties2>,
) {
    unsafe { brvk::fns::get_physical_device_properties2(physical_device.0, sink.as_mut_ptr()) }
}

#[inline(always)]
pub fn get_physical_device_memory_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    sink: &mut MaybeUninit<PhysicalDeviceMemoryProperties>,
) {
    unsafe { brvk::fns::get_physical_device_memory_properties(physical_device.0, sink.as_mut_ptr().cast()) }
}

#[inline(always)]
pub fn get_physical_device_format_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    format: brvk::VkFormat,
    sink: &mut MaybeUninit<brvk::VkFormatProperties>,
) {
    unsafe { brvk::fns::get_physical_device_format_properties(physical_device.0, format, sink.as_mut_ptr()) }
}

#[cfg(feature = "Allow1_1APIs")]
#[inline(always)]
pub fn get_physical_device_format_properties2(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    format: brvk::VkFormat,
    sink: &mut MaybeUninit<brvk::VkFormatProperties2>,
) {
    unsafe { brvk::fns::get_physical_device_format_properties2(physical_device.0, format, sink.as_mut_ptr()) }
}

#[inline(always)]
pub fn get_physical_device_image_format_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    format: brvk::VkFormat,
    image_type: brvk::VkImageType,
    tiling: brvk::VkImageTiling,
    usage: ImageUsageFlags,
    flags: ImageFlags,
    sink: &mut MaybeUninit<brvk::VkImageFormatProperties>,
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::get_physical_device_image_format_properties(
            physical_device.0,
            format,
            image_type,
            tiling,
            usage.bits(),
            flags.bits(),
            sink.as_mut_ptr(),
        )
    })?;

    Ok(())
}

#[cfg(feature = "Allow1_1APIs")]
#[inline(always)]
pub fn get_physical_device_image_format_properties2(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    info: &brvk::VkPhysicalDeviceImageFormatInfo2,
    sink: &mut MaybeUninit<brvk::VkImageFormatProperties2>,
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::get_physical_device_image_format_properties2(
            physical_device.0,
            core::ptr::from_ref(info),
            sink.as_mut_ptr(),
        )
    })?;

    Ok(())
}

#[inline]
pub fn get_physical_device_sparse_image_format_property_count(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    format: brvk::VkFormat,
    image_type: brvk::VkImageType,
    samples: brvk::VkSampleCountFlags,
    usage: ImageUsageFlags,
    tiling: brvk::VkImageTiling,
) -> u32 {
    let mut sink = MaybeUninit::uninit();
    unsafe {
        brvk::fns::get_physical_device_sparse_image_format_properties(
            physical_device.0,
            format,
            image_type,
            samples,
            usage.bits(),
            tiling,
            sink.as_mut_ptr(),
            null_mut(),
        );
    }

    unsafe { sink.assume_init() }
}

#[inline]
pub fn get_physical_device_sparse_image_format_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    format: brvk::VkFormat,
    image_type: brvk::VkImageType,
    samples: brvk::VkSampleCountFlags,
    usage: ImageUsageFlags,
    tiling: brvk::VkImageTiling,
    sink: &mut [MaybeUninit<brvk::VkSparseImageFormatProperties>],
) -> u32 {
    let mut count = sink.len() as _;
    unsafe {
        brvk::fns::get_physical_device_sparse_image_format_properties(
            physical_device.0,
            format,
            image_type,
            samples,
            usage.bits(),
            tiling,
            &mut count,
            sink.as_mut_ptr().cast(),
        );
    }

    count
}

#[inline]
pub fn get_physical_device_queue_family_property_count(physical_device: VkHandleRef<brvk::VkPhysicalDevice>) -> u32 {
    let mut sink = MaybeUninit::uninit();
    unsafe {
        brvk::fns::get_physical_device_queue_family_properties(physical_device.0, sink.as_mut_ptr(), null_mut());
    }

    unsafe { sink.assume_init() }
}

#[inline(always)]
pub fn get_physical_device_queue_family_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    sink: &mut [MaybeUninit<QueueFamilyProperties>],
) -> u32 {
    let mut count = sink.len() as _;
    unsafe {
        brvk::fns::get_physical_device_queue_family_properties(physical_device.0, &mut count, sink.as_mut_ptr().cast());
    }

    count
}

/// # Safety
///
/// `dpy` must be a valid Xlib display pointer.
#[cfg(feature = "VK_KHR_xlib_surface")]
#[inline]
pub unsafe fn get_physical_device_xlib_presentation_support(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    queue_family_index: u32,
    dpy: *mut x11::xlib::Display,
    visual_id: x11::xlib::VisualID,
) -> bool {
    unsafe {
        brvk::fns::get_physical_device_xlib_presentation_support_khr(
            physical_device.0,
            queue_family_index,
            dpy,
            visual_id,
        ) == brvk::VK_TRUE
    }
}

/// # Safety
///
/// `connection` must be a valid XCB connection pointer.
#[cfg(feature = "VK_KHR_xcb_surface")]
#[inline]
pub unsafe fn get_physical_device_xcb_presentation_support(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    queue_family_index: u32,
    connection: *mut xcb::ffi::xcb_connection_t,
    visual_id: xcb::x::Visualid,
) -> bool {
    unsafe {
        brvk::fns::get_physical_device_xcb_presentation_support_khr(
            physical_device.0,
            queue_family_index,
            connection,
            visual_id,
        ) == brvk::VK_TRUE
    }
}

/// # Safety
///
/// `display` must be a valid Wayland display pointer.
#[cfg(feature = "VK_KHR_wayland_surface")]
#[inline]
pub unsafe fn get_physical_device_wayland_presentation_support(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    queue_family_index: u32,
    display: *mut core::ffi::c_void,
) -> bool {
    unsafe {
        brvk::fns::get_physical_device_wayland_presentation_support_khr(physical_device.0, queue_family_index, display)
            == brvk::VK_TRUE
    }
}

#[cfg(feature = "VK_KHR_win32_surface")]
#[inline(always)]
pub fn get_physical_device_win32_presentation_support(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    queue_family_index: u32,
) -> bool {
    unsafe {
        brvk::fns::get_physical_device_win32_presentation_support_khr(physical_device.0, queue_family_index)
            == brvk::VK_TRUE
    }
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `brvk::VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_support(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    queue_family_index: u32,
    surface: VkHandleRef<brvk::VkSurfaceKHR>,
) -> crate::Result<bool> {
    let mut sink = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::get_physical_device_surface_support_khr(
            physical_device.0,
            queue_family_index,
            surface.0,
            sink.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { sink.assume_init() == brvk::VK_TRUE })
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `brvk::VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_capabilities(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    surface: VkHandleRef<brvk::VkSurfaceKHR>,
    sink: &mut MaybeUninit<SurfaceCapabilities>,
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::get_physical_device_surface_capabilities_khr(physical_device.0, surface.0, sink.as_mut_ptr().cast())
    })?;

    Ok(())
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `brvk::VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_format_count(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    surface: VkHandleRef<brvk::VkSurfaceKHR>,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::get_physical_device_surface_formats_khr(
            physical_device.0,
            surface.0,
            v.as_mut_ptr(),
            core::ptr::null_mut(),
        )
    })?;

    Ok(unsafe { v.assume_init() })
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `brvk::VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_formats(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    surface: VkHandleRef<brvk::VkSurfaceKHR>,
    sink: &mut [MaybeUninit<SurfaceFormat>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::get_physical_device_surface_formats_khr(
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
/// `surface` and `physical_device` must be created from the same `brvk::VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_present_mode_count(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    surface: VkHandleRef<brvk::VkSurfaceKHR>,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::get_physical_device_surface_present_modes_khr(
            physical_device.0,
            surface.0,
            v.as_mut_ptr(),
            core::ptr::null_mut(),
        )
    })?;

    Ok(unsafe { v.assume_init() })
}

/// # Safety
///
/// `surface` and `physical_device` must be created from the same `brvk::VkInstance`.
#[cfg(feature = "VK_KHR_surface")]
#[inline]
pub unsafe fn get_physical_device_surface_present_modes(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    surface: VkHandleRef<brvk::VkSurfaceKHR>,
    sink: &mut [MaybeUninit<PresentMode>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::get_physical_device_surface_present_modes_khr(
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
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
) -> crate::Result<u32> {
    let mut count = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::get_physical_device_display_properties_khr(physical_device.0, count.as_mut_ptr(), null_mut())
    })?;

    Ok(unsafe { count.assume_init() })
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_physical_device_display_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    sink: &mut [MaybeUninit<DisplayProperties>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::get_physical_device_display_properties_khr(physical_device.0, &mut v, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(v))
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_physical_device_display_plane_property_count(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
) -> crate::Result<u32> {
    let mut count = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::get_physical_device_display_plane_properties_khr(physical_device.0, count.as_mut_ptr(), null_mut())
    })?;

    Ok(unsafe { count.assume_init() })
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_physical_device_display_plane_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    sink: &mut [MaybeUninit<DisplayPlaneProperties>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::get_physical_device_display_plane_properties_khr(physical_device.0, &mut v, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(v))
}

/// # Safety
///
/// `display` and `physical_device` must be created from the same `brvk::VkInstance`.
#[cfg(feature = "VK_KHR_display")]
#[inline]
pub unsafe fn get_display_mode_property_count(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    display: VkHandleRef<brvk::VkDisplayKHR>,
) -> crate::Result<u32> {
    let mut count = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::get_display_mode_properties_khr(physical_device.0, display.0, count.as_mut_ptr(), null_mut())
    })?;

    Ok(unsafe { count.assume_init() })
}

/// # Safety
///
/// `display` and `physical_device` must be created from the same `brvk::VkInstance`.
#[cfg(feature = "VK_KHR_display")]
#[inline]
pub unsafe fn get_display_mode_properties(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    display: VkHandleRef<brvk::VkDisplayKHR>,
    sink: &mut [MaybeUninit<DisplayModeProperties>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::get_display_mode_properties_khr(physical_device.0, display.0, &mut v, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(v))
}

/// # Safety
///
/// `mode` and `physical_device` must be created from the same `brvk::VkInstance`.
#[cfg(feature = "VK_KHR_display")]
#[inline]
pub unsafe fn get_display_plane_capabilities(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    mode: VkHandleRefMut<brvk::VkDisplayModeKHR>,
    plane_index: u32,
    sink: &mut MaybeUninit<brvk::VkDisplayPlaneCapabilitiesKHR>,
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::get_display_plane_capabilities_khr(physical_device.0, mode.0, plane_index, sink.as_mut_ptr())
    })?;

    Ok(())
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_display_plane_supported_display_count(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    plane_index: u32,
) -> crate::Result<u32> {
    let mut count = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::get_display_plane_supported_displays_khr(
            physical_device.0,
            plane_index,
            count.as_mut_ptr(),
            null_mut(),
        )
    })?;

    Ok(unsafe { count.assume_init() })
}

#[cfg(feature = "VK_KHR_display")]
#[inline]
pub fn get_display_plane_supported_displays(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    plane_index: u32,
    sink: &mut [MaybeUninit<brvk::VkDisplayKHR>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::get_display_plane_supported_displays_khr(
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
    instance: VkHandleRef<brvk::VkInstance>,
    surface: VkHandleRefMut<brvk::VkSurfaceKHR>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_surface_khr(instance.0, surface.0, opt_pointer(allocation_callbacks)) }
}

#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub fn create_swapchain(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &SwapchainCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkSwapchainKHR> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_swapchain_khr(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `swapchain` must be created from `device`.
#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn destroy_swapchain(
    device: VkHandleRef<brvk::VkDevice>,
    swapchain: VkHandleRefMut<brvk::VkSwapchainKHR>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_swapchain_khr(device.0, swapchain.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// `swapchain` must be created from `device`.
#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn get_swapchain_image_count(
    device: VkHandleRef<brvk::VkDevice>,
    swapchain: VkHandleRef<brvk::VkSwapchainKHR>,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::get_swapchain_images_khr(device.0, swapchain.0, v.as_mut_ptr(), null_mut())
    })?;

    Ok(unsafe { v.assume_init() })
}

/// # Safety
///
/// `swapchain` must be created from `device`.
#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn get_swapchain_images(
    device: VkHandleRef<brvk::VkDevice>,
    swapchain: VkHandleRef<brvk::VkSwapchainKHR>,
    sink: &mut [MaybeUninit<brvk::VkImage>],
) -> crate::Result<ArrayQueryResult<u32>> {
    let mut v = sink.len() as _;
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::get_swapchain_images_khr(device.0, swapchain.0, &mut v, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(v))
}

/// # Safety
///
/// `swapchain`, `semaphore` and `fence` must be created from the same `device`.
#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub unsafe fn acquire_next_image(
    device: VkHandleRef<brvk::VkDevice>,
    swapchain: VkHandleRefMut<brvk::VkSwapchainKHR>,
    timeout: u64,
    semaphore: Option<VkHandleRefMut<brvk::VkSemaphore>>,
    fence: Option<VkHandleRefMut<brvk::VkFence>>,
) -> crate::Result<u32> {
    let mut v = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::acquire_next_image_khr(
            device.0,
            swapchain.0,
            timeout,
            semaphore.map(|x| x.0),
            fence.map(|x| x.0),
            v.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { v.assume_init() })
}

#[inline]
pub fn create_device(
    physical_device: VkHandleRef<brvk::VkPhysicalDevice>,
    info: &DeviceCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkDevice> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_device(
            physical_device.0,
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// Host access to all `VkQueue` objects created from `device` must be externally synchronized.
#[inline]
pub unsafe fn destroy_device(
    device: VkHandleRefMut<brvk::VkDevice>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_device(device.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn get_device_queue(device: VkHandleRef<brvk::VkDevice>, family_index: u32, index: u32) -> brvk::VkQueue {
    let mut h = MaybeUninit::uninit();
    unsafe {
        brvk::fns::get_device_queue(device.0, family_index, index, h.as_mut_ptr());
        h.assume_init()
    }
}

#[inline]
pub fn queue_wait_idle(queue: VkHandleRefMut<brvk::VkQueue>) -> crate::Result<()> {
    translate_vk_result(unsafe { brvk::fns::queue_wait_idle(queue.0) })?;

    Ok(())
}

/// # Safety
///
/// Host access to all `brvk::VkQueue` objects created from `device` must be externally synchronized.
#[inline]
pub unsafe fn device_wait_idle(device: VkHandleRef<brvk::VkDevice>) -> crate::Result<()> {
    translate_vk_result(unsafe { brvk::fns::device_wait_idle(device.0) })?;

    Ok(())
}

#[inline]
pub fn get_instance_proc_addr_pfn<F: brvk::PFN>(instance: VkHandleRef<brvk::VkInstance>) -> Option<F> {
    Some(unsafe { F::from_void_fn(brvk::fns::get_instance_proc_addr(instance.0, F::NAME_CSTR.as_ptr())?) })
}

#[inline]
pub fn get_device_proc_addr_pfn<F: brvk::PFN>(device: VkHandleRef<brvk::VkDevice>) -> Option<F> {
    Some(unsafe { F::from_void_fn(brvk::fns::get_device_proc_addr(device.0, F::NAME_CSTR.as_ptr())?) })
}

#[inline]
pub fn queue_submit(
    queue: VkHandleRefMut<brvk::VkQueue>,
    submit_info: &[SubmitInfo],
    fence: Option<VkHandleRefMut<brvk::VkFence>>,
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::queue_submit(
            queue.0,
            submit_info.len() as _,
            submit_info.as_ptr().cast(),
            fence.map(|x| x.0),
        )
    })?;

    Ok(())
}

#[cfg(feature = "Allow1_3APIs")]
#[inline]
pub fn queue_submit2(
    queue: VkHandleRefMut<brvk::VkQueue>,
    submit_info: &[SubmitInfo2],
    fence: Option<VkHandleRefMut<brvk::VkFence>>,
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::queue_submit2(
            queue.0,
            submit_info.len() as _,
            submit_info.as_ptr().cast(),
            fence.map(|x| x.0),
        )
    })?;

    Ok(())
}

#[cfg(feature = "VK_KHR_swapchain")]
#[inline]
pub fn queue_present(queue: VkHandleRefMut<brvk::VkQueue>, present_info: &PresentInfo) -> crate::Result<PresentResult> {
    match unsafe { brvk::fns::queue_present_khr(queue.0, core::ptr::from_ref(present_info).cast()) } {
        brvk::VK_SUCCESS => Ok(PresentResult::Success),
        brvk::VK_SUBOPTIMAL_KHR => Ok(PresentResult::Suboptimal),
        e => match ResultCode(e) {
            e if e.is_err() => Err(e),
            e => unreachable!("unexpected result: {e:?}"),
        },
    }
}

#[inline]
pub fn queue_bind_sparse(
    queue: VkHandleRefMut<brvk::VkQueue>,
    infos: &[BindSparseInfo],
    fence: Option<VkHandleRefMut<brvk::VkFence>>,
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::queue_bind_sparse(queue.0, infos.len() as _, infos.as_ptr().cast(), fence.map(|x| x.0))
    })?;

    Ok(())
}

#[inline]
pub fn create_command_pool(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &CommandPoolCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkCommandPool> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_command_pool(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `command_pool` must be created from `device`.
#[inline]
pub unsafe fn destroy_command_pool(
    device: VkHandleRef<brvk::VkDevice>,
    command_pool: VkHandleRefMut<brvk::VkCommandPool>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_command_pool(device.0, command_pool.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// command pool in `allocation_info` must be created from `device`.
#[inline]
pub unsafe fn allocate_command_buffers(
    device: VkHandleRef<brvk::VkDevice>,
    allocation_info: &CommandBufferAllocateInfo,
    sink: &mut [MaybeUninit<brvk::VkCommandBuffer>],
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::allocate_command_buffers(
            device.0,
            core::ptr::from_ref(allocation_info).cast(),
            sink.as_mut_ptr().cast(),
        )
    })?;

    Ok(())
}

/// # Safety
///
/// * `buffers` must be allocated from `command_pool`.
/// * `command_pool` must be created from `device`.
#[inline]
pub unsafe fn free_command_buffers(
    device: VkHandleRef<brvk::VkDevice>,
    command_pool: VkHandleRefMut<brvk::VkCommandPool>,
    buffers: &[VkHandleRefMut<brvk::VkCommandBuffer>],
) {
    unsafe { brvk::fns::free_command_buffers(device.0, command_pool.0, buffers.len() as _, buffers.as_ptr().cast()) }
}

/// # Safety
///
/// `command_pool` must be created from `device`.
#[inline(always)]
pub unsafe fn reset_command_pool(
    device: VkHandleRef<brvk::VkDevice>,
    command_pool: VkHandleRefMut<brvk::VkCommandPool>,
    flags: CommandPoolResetFlags,
) -> crate::Result<()> {
    translate_vk_result(unsafe { brvk::fns::reset_command_pool(device.0, command_pool.0, flags.bits()) })?;

    Ok(())
}

/// # Safety
///
/// `command_pool` must be created from `device`.
#[cfg(feature = "Allow1_1APIs")]
#[inline(always)]
pub unsafe fn trim_command_pool(
    device: VkHandleRef<brvk::VkDevice>,
    command_pool: VkHandleRefMut<brvk::VkCommandPool>,
    flags: CommandPoolTrimFlags,
) {
    unsafe {
        brvk::fns::trim_command_pool(device.0, command_pool.0, flags.bits());
    }
}

/// # Safety
///
/// * The command pool that `command_buffer` was allocated must be externally synchronized.
#[inline]
pub unsafe fn begin_command_buffer(
    command_buffer: VkHandleRefMut<brvk::VkCommandBuffer>,
    begin_info: &CommandBufferBeginInfo,
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::begin_command_buffer(command_buffer.0, core::ptr::from_ref(begin_info).cast())
    })?;

    Ok(())
}

/// # Safety
///
/// * The command pool that `command_buffer` was allocated must be externally synchronized.
#[inline]
pub unsafe fn end_command_buffer(command_buffer: VkHandleRefMut<brvk::VkCommandBuffer>) -> crate::Result<()> {
    translate_vk_result(unsafe { brvk::fns::end_command_buffer(command_buffer.0) })?;

    Ok(())
}

#[inline]
pub fn create_fence(
    device: VkHandleRef<brvk::VkDevice>,
    info: &FenceCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkFence> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_fence(
            device.0,
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `fence` must be created from the `device`.
#[inline]
pub unsafe fn destroy_fence(
    device: VkHandleRef<brvk::VkDevice>,
    fence: VkHandleRefMut<brvk::VkFence>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_fence(device.0, fence.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// fences in `fences` must be created from the `device`.
#[inline]
pub unsafe fn wait_for_fences(
    device: VkHandleRef<brvk::VkDevice>,
    fences: &[VkHandleRef<brvk::VkFence>],
    wait_all: bool,
    timeout: u64,
) -> crate::Result<TimeoutableWaitResult> {
    Ok(TimeoutableWaitResult::from_vk_result(translate_vk_result(unsafe {
        brvk::fns::wait_for_fences(
            device.0,
            fences.len() as _,
            fences.as_ptr().cast(),
            wait_all as _,
            timeout,
        )
    })?))
}

/// # Safety
///
/// fences in `fences` must be created from the `device`.
#[inline]
pub unsafe fn reset_fences(
    device: VkHandleRef<brvk::VkDevice>,
    fences: &[VkHandleRefMut<brvk::VkFence>],
) -> crate::Result<()> {
    translate_vk_result(unsafe { brvk::fns::reset_fences(device.0, fences.len() as _, fences.as_ptr().cast()) })?;

    Ok(())
}

/// # Safety
///
/// `fence` must be created from the `device`.
#[inline]
pub unsafe fn get_fence_status(
    device: VkHandleRef<brvk::VkDevice>,
    fence: VkHandleRef<brvk::VkFence>,
) -> crate::Result<brvk::VkResult> {
    translate_vk_result(unsafe { brvk::fns::get_fence_status(device.0, fence.0) })
}

#[inline]
pub fn create_semaphore(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &SemaphoreCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkSemaphore> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_semaphore(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// * `device` and `semaphore` must be a valid device handle and semaphore handle, respectively.
/// * `semaphore` must be created from the `device`.
#[inline]
pub unsafe fn destroy_semaphore(
    device: VkHandleRef<brvk::VkDevice>,
    semaphore: VkHandleRefMut<brvk::VkSemaphore>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_semaphore(device.0, semaphore.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn create_event(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &EventCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkEvent> {
    let mut h = MaybeUninit::uninit();
    crate::error::translate_vk_result(unsafe {
        brvk::fns::create_event(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// * `device` and `event` must be a valid device handle and event handle, respectively.
/// * `event` must be created from the `device`.
#[inline]
pub unsafe fn destroy_event(
    device: VkHandleRef<brvk::VkDevice>,
    event: VkHandleRefMut<brvk::VkEvent>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_event(device.0, event.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn create_buffer(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &BufferCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkBuffer> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_buffer(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// * `buffer` must be created from `device`.
#[inline]
pub unsafe fn destroy_buffer(
    device: VkHandleRef<brvk::VkDevice>,
    buffer: VkHandleRefMut<brvk::VkBuffer>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_buffer(device.0, buffer.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// * `buffer` must be created from `device`.
#[inline]
pub unsafe fn get_buffer_memory_requirements(
    device: VkHandleRef<brvk::VkDevice>,
    buffer: VkHandleRef<brvk::VkBuffer>,
) -> brvk::VkMemoryRequirements {
    let mut sink = MaybeUninit::uninit();
    unsafe {
        brvk::fns::get_buffer_memory_requirements(device.0, buffer.0, sink.as_mut_ptr());

        sink.assume_init()
    }
}

/// # Safety
///
/// * `buffer` must be created from `device`.
#[cfg(feature = "Allow1_1APIs")]
#[inline]
pub unsafe fn get_buffer_memory_requirements2(
    device: VkHandleRef<brvk::VkDevice>,
    info: &BufferMemoryRequirementsInfo2<'_, impl VkHandle<Handle = brvk::VkBuffer>>,
    sink: &mut MaybeUninit<brvk::VkMemoryRequirements2>,
) {
    unsafe { brvk::fns::get_buffer_memory_requirements2(device.0, info.as_ref(), sink.as_mut_ptr()) }
}

/// # Safety
///
/// * `buffer` and `memory` must be created from `device`.
#[inline]
pub unsafe fn bind_buffer_memory(
    device: VkHandleRef<brvk::VkDevice>,
    buffer: VkHandleRefMut<brvk::VkBuffer>,
    memory: VkHandleRef<brvk::VkDeviceMemory>,
    offset: brvk::VkDeviceSize,
) -> crate::Result<()> {
    translate_vk_result(unsafe { brvk::fns::bind_buffer_memory(device.0, buffer.0, memory.0, offset) })?;

    Ok(())
}

/// # Safety
///
/// * buffers in `bind_infos` must be created from `device`.
#[cfg(feature = "Allow1_1APIs")]
#[inline]
pub unsafe fn bind_buffer_memory2(
    device: VkHandleRef<brvk::VkDevice>,
    bind_infos: &[BindBufferMemoryInfo],
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::bind_buffer_memory2(device.0, bind_infos.len() as _, bind_infos.as_ptr().cast())
    })?;

    Ok(())
}

#[inline]
pub fn create_image(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &ImageCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkImage> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_image(
            device.0,
            create_info as *const _ as _,
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `image` must be created from the `device`.
#[inline]
pub unsafe fn destroy_image(
    device: VkHandleRef<brvk::VkDevice>,
    image: VkHandleRefMut<brvk::VkImage>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_image(device.0, image.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// `image` must be created from the `device`.
#[inline]
pub unsafe fn get_image_memory_requirements(
    device: VkHandleRef<brvk::VkDevice>,
    image: VkHandleRef<brvk::VkImage>,
) -> brvk::VkMemoryRequirements {
    let mut sink = MaybeUninit::uninit();
    unsafe {
        brvk::fns::get_image_memory_requirements(device.0, image.0, sink.as_mut_ptr());

        sink.assume_init()
    }
}

/// # Safety
///
/// image in `info` must be created from the `device`.
#[cfg(feature = "Allow1_1APIs")]
#[inline]
pub unsafe fn get_image_memory_requirements2(
    device: VkHandleRef<brvk::VkDevice>,
    info: &ImageMemoryRequirementsInfo2<'_, impl VkHandle<Handle = brvk::VkImage>>,
    sink: &mut MaybeUninit<brvk::VkMemoryRequirements2>,
) {
    unsafe { brvk::fns::get_image_memory_requirements2(device.0, info.as_ref(), sink.as_mut_ptr()) }
}

/// # Safety
///
/// * `image` and `memory` must be created from `device`.
#[inline]
pub unsafe fn bind_image_memory(
    device: VkHandleRef<brvk::VkDevice>,
    image: VkHandleRefMut<brvk::VkImage>,
    memory: VkHandleRef<brvk::VkDeviceMemory>,
    offset: brvk::VkDeviceSize,
) -> crate::Result<()> {
    translate_vk_result(unsafe { brvk::fns::bind_image_memory(device.0, image.0, memory.0, offset) })?;

    Ok(())
}

/// # Safety
///
/// buffers in `bind_infos` must be created from `device`.
#[cfg(feature = "Allow1_1APIs")]
#[inline]
pub unsafe fn bind_image_memory2(
    device: VkHandleRef<brvk::VkDevice>,
    bind_infos: &[BindImageMemoryInfo],
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::bind_image_memory2(device.0, bind_infos.len() as _, bind_infos.as_ptr().cast())
    })?;

    Ok(())
}

#[inline]
pub fn create_sampler(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &SamplerCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkSampler> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_sampler(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `sampler` must be created from `device`.
#[inline]
pub unsafe fn destroy_sampler(
    device: VkHandleRef<brvk::VkDevice>,
    sampler: VkHandleRefMut<brvk::VkSampler>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_sampler(device.0, sampler.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn allocate_memory(
    device: VkHandleRef<brvk::VkDevice>,
    allocate_info: &MemoryAllocateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkDeviceMemory> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::allocate_memory(
            device.0,
            core::ptr::from_ref(allocate_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `memory` must be allocated from `device`.
#[inline]
pub unsafe fn free_memory(
    device: VkHandleRef<brvk::VkDevice>,
    memory: VkHandleRefMut<brvk::VkDeviceMemory>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::free_memory(device.0, memory.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// `memory` must be allocated from `device`.
#[inline]
pub unsafe fn map_memory(
    device: VkHandleRef<brvk::VkDevice>,
    memory: VkHandleRefMut<brvk::VkDeviceMemory>,
    range: core::ops::Range<brvk::VkDeviceSize>,
    flags: brvk::VkMemoryMapFlags,
) -> crate::Result<*mut core::ffi::c_void> {
    let mut p = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::map_memory(
            device.0,
            memory.0,
            range.start,
            range.end - range.start,
            flags,
            p.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { p.assume_init() })
}

/// # Safety
///
/// `memory` must be mapped from `device`.
#[inline]
pub unsafe fn unmap_memory(device: VkHandleRef<brvk::VkDevice>, memory: VkHandleRefMut<brvk::VkDeviceMemory>) {
    unsafe { brvk::fns::unmap_memory(device.0, memory.0) }
}

#[inline]
pub fn invalidate_mapped_memory_ranges(
    device: VkHandleRef<brvk::VkDevice>,
    memory_ranges: &[MappedMemoryRange],
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::invalidate_mapped_memory_ranges(device.0, memory_ranges.len() as _, memory_ranges.as_ptr().cast())
    })?;

    Ok(())
}

#[inline]
pub fn flush_mapped_memory_ranges(
    device: VkHandleRef<brvk::VkDevice>,
    memory_ranges: &[MappedMemoryRange],
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::flush_mapped_memory_ranges(device.0, memory_ranges.len() as _, memory_ranges.as_ptr().cast())
    })?;

    Ok(())
}

/// # Safety
///
/// image in `create_info` must be created from `device`.
#[inline]
pub unsafe fn create_image_view(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &ImageViewCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkImageView> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_image_view(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `image_view` must be created from `device`.
#[inline]
pub unsafe fn destroy_image_view(
    device: VkHandleRef<brvk::VkDevice>,
    image_view: VkHandleRefMut<brvk::VkImageView>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_image_view(device.0, image_view.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// buffer in `create_info` must be created from `device`.
#[inline]
pub unsafe fn create_buffer_view(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &BufferViewCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkBufferView> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_buffer_view(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `buffer_view` must be created from `device`.
#[inline]
pub unsafe fn destroy_buffer_view(
    device: VkHandleRef<brvk::VkDevice>,
    buffer_view: VkHandleRefMut<brvk::VkBufferView>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_buffer_view(device.0, buffer_view.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// renderPass and attachments in `create_info` must be created from `device`.
#[inline]
pub unsafe fn create_framebuffer(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &FramebufferCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkFramebuffer> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_framebuffer(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `framebuffer` must be created from `device`.
#[inline]
pub unsafe fn destroy_framebuffer(
    device: VkHandleRef<brvk::VkDevice>,
    framebuffer: VkHandleRefMut<brvk::VkFramebuffer>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_framebuffer(device.0, framebuffer.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn create_shader_module(
    device: VkHandleRef<brvk::VkDevice>,
    info: &ShaderModuleCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkShaderModule> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_shader_module(
            device.0,
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `shader_module` must be created from `device`.
#[inline]
pub unsafe fn destroy_shader_module(
    device: VkHandleRef<brvk::VkDevice>,
    shader_module: VkHandleRefMut<brvk::VkShaderModule>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_shader_module(device.0, shader_module.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn create_graphics_pipelines(
    device: VkHandleRef<brvk::VkDevice>,
    pipeline_cache: Option<VkHandleRef<brvk::VkPipelineCache>>,
    create_infos: &[GraphicsPipelineCreateInfo],
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    results: &mut [MaybeUninit<brvk::VkPipeline>],
) -> crate::Result<()> {
    debug_assert!(results.len() >= create_infos.len());

    translate_vk_result(unsafe {
        brvk::fns::create_graphics_pipelines(
            device.0,
            pipeline_cache.map(|x| x.0),
            create_infos.len() as _,
            create_infos.as_ptr().cast(),
            opt_pointer(allocation_callbacks),
            results.as_mut_ptr().cast(),
        )
    })?;

    Ok(())
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn create_graphics_pipeline_array<const N: usize>(
    device: VkHandleRef<brvk::VkDevice>,
    pipeline_cache: Option<VkHandleRef<brvk::VkPipelineCache>>,
    create_infos: &[GraphicsPipelineCreateInfo; N],
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<[brvk::VkPipeline; N]> {
    let mut results = [MaybeUninit::uninit(); N];
    unsafe {
        create_graphics_pipelines(device, pipeline_cache, create_infos, allocation_callbacks, &mut results)?;
    }

    Ok(core::array::from_fn(|n| unsafe { results[n].assume_init() }))
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn create_compute_pipelines(
    device: VkHandleRef<brvk::VkDevice>,
    pipeline_cache: Option<VkHandleRef<brvk::VkPipelineCache>>,
    create_infos: &[ComputePipelineCreateInfo],
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    results: &mut [MaybeUninit<brvk::VkPipeline>],
) -> crate::Result<()> {
    debug_assert!(results.len() >= create_infos.len());

    translate_vk_result(unsafe {
        brvk::fns::create_compute_pipelines(
            device.0,
            pipeline_cache.map(|x| x.0),
            create_infos.len() as _,
            create_infos.as_ptr().cast(),
            opt_pointer(allocation_callbacks),
            results.as_mut_ptr().cast(),
        )
    })?;

    Ok(())
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn create_compute_pipeline_array<const N: usize>(
    device: VkHandleRef<brvk::VkDevice>,
    pipeline_cache: Option<VkHandleRef<brvk::VkPipelineCache>>,
    create_infos: &[ComputePipelineCreateInfo; N],
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<[brvk::VkPipeline; N]> {
    let mut results = [MaybeUninit::uninit(); N];
    unsafe {
        create_compute_pipelines(device, pipeline_cache, create_infos, allocation_callbacks, &mut results)?;
    }

    Ok(core::array::from_fn(|n| unsafe { results[n].assume_init() }))
}

/// # Safety
///
/// `pipeline` must be created from `device`.
#[inline]
pub unsafe fn destroy_pipeline(
    device: VkHandleRef<brvk::VkDevice>,
    pipeline: VkHandleRefMut<brvk::VkPipeline>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_pipeline(device.0, pipeline.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn create_pipeline_layout(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &PipelineLayoutCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkPipelineLayout> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_pipeline_layout(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `pipeline_layout` must be created from `device`.
#[inline]
pub unsafe fn destroy_pipeline_layout(
    device: VkHandleRef<brvk::VkDevice>,
    pipeline_layout: VkHandleRefMut<brvk::VkPipelineLayout>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_pipeline_layout(device.0, pipeline_layout.0, opt_pointer(allocation_callbacks)) }
}

#[inline]
pub fn create_pipeline_cache(
    device: VkHandleRef<brvk::VkDevice>,
    info: &PipelineCacheCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkPipelineCache> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_pipeline_cache(
            device.0,
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn destroy_pipeline_cache(
    device: VkHandleRef<brvk::VkDevice>,
    pipeline_cache: VkHandleRefMut<brvk::VkPipelineCache>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_pipeline_cache(device.0, pipeline_cache.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn get_pipeline_cache_data_byte_length(
    device: VkHandleRef<brvk::VkDevice>,
    pipeline_cache: VkHandleRef<brvk::VkPipelineCache>,
) -> crate::Result<usize> {
    let mut len = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::get_pipeline_cache_data(device.0, pipeline_cache.0, len.as_mut_ptr(), null_mut())
    })?;

    Ok(unsafe { len.assume_init() })
}

/// # Safety
///
/// `pipeline_cache` must be created from `device`.
#[inline]
pub unsafe fn get_pipeline_cache_data(
    device: VkHandleRef<brvk::VkDevice>,
    pipeline_cache: VkHandleRef<brvk::VkPipelineCache>,
    sink: &mut [MaybeUninit<u8>],
) -> crate::Result<ArrayQueryResult<usize>> {
    let mut len = sink.len();
    let r = ArrayQueryResult::from_vk_result(unsafe {
        brvk::fns::get_pipeline_cache_data(device.0, pipeline_cache.0, &mut len, sink.as_mut_ptr().cast())
    })?;

    Ok(r.with_result(len))
}

/// # Safety
///
/// * `semaphore` must be created with a [`brvk::VkSemaphoreType`] of [`brvk::VK_SEMAPHORE_TYPE_TIMELINE`].
/// * `semaphore` must be created from `device`.
#[cfg(feature = "Allow1_2APIs")]
#[inline]
pub unsafe fn get_semaphore_counter_value(
    device: VkHandleRef<brvk::VkDevice>,
    semaphore: VkHandleRef<brvk::VkSemaphore>,
) -> crate::Result<u64> {
    let mut value = MaybeUninit::uninit();
    translate_vk_result(unsafe { brvk::fns::get_semaphore_counter_value(device.0, semaphore.0, value.as_mut_ptr()) })?;

    Ok(unsafe { value.assume_init() })
}

/// # Safety
///
/// * semaphore in `signal_info` must be created with a [`brvk::VkSemaphoreType`] of [`brvk::VK_SEMAPHORE_TYPE_TIMELINE`].
/// * semaphore in `signal_info` must be created from `device`.
#[cfg(feature = "Allow1_2APIs")]
#[inline]
pub unsafe fn signal_semaphore(
    device: VkHandleRef<brvk::VkDevice>,
    signal_info: &SemaphoreSignalInfo,
) -> crate::Result<()> {
    translate_vk_result(unsafe { brvk::fns::signal_semaphore(device.0, core::ptr::from_ref(signal_info).cast()) })?;

    Ok(())
}

/// # Safety
///
/// * semaphore in `wait_info` must be created with a [`brvk::VkSemaphoreType`] of [`brvk::VK_SEMAPHORE_TYPE_TIMELINE`].
/// * semaphore in `wait_info` must be created from `device`.
#[cfg(feature = "Allow1_2APIs")]
#[inline]
pub unsafe fn wait_semaphores(
    device: VkHandleRef<brvk::VkDevice>,
    wait_info: &SemaphoreWaitInfo,
    timeout: u64,
) -> crate::Result<TimeoutableWaitResult> {
    Ok(TimeoutableWaitResult::from_vk_result(translate_vk_result(unsafe {
        brvk::fns::wait_semaphores(device.0, core::ptr::from_ref(wait_info).cast(), timeout)
    })?))
}

#[inline]
pub fn create_descriptor_pool(
    device: VkHandleRef<brvk::VkDevice>,
    info: &DescriptorPoolCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkDescriptorPool> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_descriptor_pool(
            device.0,
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `descriptor_pool` must be created from `device`.
#[inline]
pub unsafe fn destroy_descriptor_pool(
    device: VkHandleRef<brvk::VkDevice>,
    descriptor_pool: VkHandleRefMut<brvk::VkDescriptorPool>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe { brvk::fns::destroy_descriptor_pool(device.0, descriptor_pool.0, opt_pointer(allocation_callbacks)) }
}

/// # Safety
///
/// * `descriptor_pool` must be created from `device`.
/// * Host access to any `VkDescriptorSet` objects allocated from `descriptor_pool` must be externally synchronized.
#[inline]
pub unsafe fn reset_descriptor_pool(
    device: VkHandleRef<brvk::VkDevice>,
    descriptor_pool: VkHandleRefMut<brvk::VkDescriptorPool>,
    flags: brvk::VkDescriptorPoolResetFlags,
) -> crate::Result<()> {
    translate_vk_result(unsafe { brvk::fns::reset_descriptor_pool(device.0, descriptor_pool.0, flags) })?;

    Ok(())
}

/// # Safety
///
/// * `info` must be a valid `VkDescriptorSetAllocateInfo` structure.
/// * `sink` must be a slice of `MaybeUninit<VkDescriptorSet>` with enough capacity to hold `info.descriptorSetCount` elements.
/// * `VkDescriptorPool` in `info` must be valid and created from `device`.
#[inline]
pub unsafe fn allocate_descriptor_sets(
    device: VkHandleRef<brvk::VkDevice>,
    info: &brvk::VkDescriptorSetAllocateInfo,
    sink: &mut [MaybeUninit<DescriptorSet>],
) -> crate::Result<()> {
    translate_vk_result(unsafe { brvk::fns::allocate_descriptor_sets(device.0, info, sink.as_mut_ptr().cast()) })?;

    Ok(())
}

/// # Safety
///
/// * `descriptor_pool` must be created from `device`.
/// * each member of `descriptor_sets` must be allocated from `descriptor_pool`.
/// * Host access to each member of `descriptor_sets` must be externally synchronized.
#[inline]
pub unsafe fn free_descriptor_sets(
    device: VkHandleRef<brvk::VkDevice>,
    descriptor_pool: VkHandleRefMut<brvk::VkDescriptorPool>,
    descriptor_sets: &[DescriptorSet],
) -> crate::Result<()> {
    translate_vk_result(unsafe {
        brvk::fns::free_descriptor_sets(
            device.0,
            descriptor_pool.0,
            descriptor_sets.len() as _,
            descriptor_sets.as_ptr().cast(),
        )
    })?;

    Ok(())
}

/// # Safety
///
/// `writes` and `copies` must be a valid array of [`VkWriteDescriptorSet`] and [`VkCopyDescriptorSet`] structs, respectively.
#[inline]
pub unsafe fn update_descriptor_sets(
    device: VkHandleRef<brvk::VkDevice>,
    writes: &[brvk::VkWriteDescriptorSet],
    copies: &[brvk::VkCopyDescriptorSet],
) {
    unsafe {
        brvk::fns::update_descriptor_sets(
            device.0,
            writes.len() as _,
            writes.as_ptr(),
            copies.len() as _,
            copies.as_ptr(),
        );
    }
}

#[inline]
pub fn create_descriptor_set_layout(
    device: VkHandleRef<brvk::VkDevice>,
    info: &DescriptorSetLayoutCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkDescriptorSetLayout> {
    let mut h = MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_descriptor_set_layout(
            device.0,
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `descriptor_set_layout` must be created from `device`.
#[inline]
pub unsafe fn destroy_descriptor_set_layout(
    device: VkHandleRef<brvk::VkDevice>,
    descriptor_set_layout: VkHandleRefMut<brvk::VkDescriptorSetLayout>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe {
        brvk::fns::destroy_descriptor_set_layout(device.0, descriptor_set_layout.0, opt_pointer(allocation_callbacks))
    }
}

/// # Safety
///
/// `descriptorSetLayout` and `pipelineLayout` in `create_info` must be created from `device`.
#[inline]
#[cfg(feature = "Allow1_1APIs")]
pub unsafe fn create_descriptor_update_template(
    device: VkHandleRef<brvk::VkDevice>,
    create_info: &brvk::VkDescriptorUpdateTemplateCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkDescriptorUpdateTemplate> {
    let mut h = core::mem::MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_descriptor_update_template(
            device.0,
            core::ptr::from_ref(create_info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `descriptor_update_template` must be created from `device`.
#[inline]
#[cfg(feature = "Allow1_1APIs")]
pub unsafe fn destroy_descriptor_update_template(
    device: VkHandleRef<brvk::VkDevice>,
    descriptor_update_template: VkHandleRefMut<brvk::VkDescriptorUpdateTemplate>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe {
        brvk::fns::destroy_descriptor_update_template(
            device.0,
            descriptor_update_template.0,
            opt_pointer(allocation_callbacks),
        )
    }
}

#[inline]
pub fn create_render_pass(
    device: VkHandleRef<brvk::VkDevice>,
    info: &RenderPassCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkRenderPass> {
    let mut h = core::mem::MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_render_pass(
            device.0,
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

#[inline]
#[cfg(feature = "Allow1_2APIs")]
pub fn create_render_pass2(
    device: VkHandleRef<brvk::VkDevice>,
    info: &RenderPassCreateInfo2,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkRenderPass> {
    let mut h = core::mem::MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_render_pass2(
            device.0,
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `render_pass` must be created from `device`.
#[inline]
pub unsafe fn destroy_render_pass(
    device: VkHandleRef<brvk::VkDevice>,
    render_pass: VkHandleRefMut<brvk::VkRenderPass>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe {
        brvk::fns::destroy_render_pass(device.0, render_pass.0, opt_pointer(allocation_callbacks));
    }
}

#[inline]
pub fn create_query_pool(
    device: VkHandleRef<brvk::VkDevice>,
    info: &QueryPoolCreateInfo,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) -> crate::Result<brvk::VkQueryPool> {
    let mut h = core::mem::MaybeUninit::uninit();
    translate_vk_result(unsafe {
        brvk::fns::create_query_pool(
            device.0,
            core::ptr::from_ref(info).cast(),
            opt_pointer(allocation_callbacks),
            h.as_mut_ptr(),
        )
    })?;

    Ok(unsafe { h.assume_init() })
}

/// # Safety
///
/// `query_pool` must be created from `device`.
#[inline]
pub unsafe fn destroy_query_pool(
    device: VkHandleRef<brvk::VkDevice>,
    query_pool: VkHandleRefMut<brvk::VkQueryPool>,
    allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
) {
    unsafe {
        brvk::fns::destroy_query_pool(device.0, query_pool.0, opt_pointer(allocation_callbacks));
    }
}
