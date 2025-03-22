use crate::vk::*;
use core::ptr::NonNull;
use derives::implements;

use super::{CompletionHandlerMut, Device, EnumerationResult, Image};

/// Opaque handle to a swapchain object.
#[repr(transparent)]
pub struct Swapchain(VkSwapchainKHR_T);
#[implements]
impl Swapchain {
    /// Retrieve the index of the next available presentation image
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_DEVICE_LOST`]
    /// * [`VK_ERROR_OUT_OF_DATE_KHR`]
    /// * [`VK_ERROR_SURFACE_LOST_KHR`]
    pub unsafe fn acquire_next(
        &mut self,
        device: &Device,
        timeout: Option<u64>,
        completion: CompletionHandlerMut,
    ) -> crate::Result<u32> {
        let (semaphore, fence) = match completion {
            CompletionHandlerMut::Host(f) => (VK_NULL_HANDLE as _, f as *mut _ as _),
            CompletionHandlerMut::Queue(s) => (s as *mut _ as _, VK_NULL_HANDLE as _),
        };

        let mut n = 0;
        unsafe {
            crate::vkfn::acquire_next_image_khr(
                device as *const _ as _,
                self as *mut _ as _,
                timeout.unwrap_or(u64::MAX),
                semaphore,
                fence,
                &mut n,
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Obtain a count of the array of presentable images associated with a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline]
    pub unsafe fn image_count(&self, device: &Device) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_swapchain_images_khr(
                device as *const _ as _,
                self as *const _ as _,
                &mut n,
                core::ptr::null_mut(),
            )
            .into_result()?;
        }

        Ok(n)
    }

    /// Obtain the array of presentable images associated with a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    pub unsafe fn images(&self, device: &Device, sink: &mut [*mut Image]) -> crate::Result<(u32, EnumerationResult)> {
        let mut n = sink.len() as _;
        let r = unsafe {
            crate::vkfn::get_swapchain_images_khr(
                device as *const _ as _,
                self as *const _ as _,
                &mut n,
                sink.as_mut_ptr(),
            )
        };
        if r == VK_SUCCESS {
            return Ok((n, EnumerationResult::Complete));
        }
        if r == VK_INCOMPLETE {
            return Ok((n, EnumerationResult::Incomplete));
        }

        Err(r)
    }

    /// Obtain the array of presentable images associated with a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[cfg(feature = "alloc")]
    pub unsafe fn images_alloc(&self, device: &Device) -> crate::Result<Vec<NonNull<Image>>> {
        let n = unsafe { self.image_count(device)? };
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        // 全部取れるのでIncompleteは見ない
        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        unsafe {
            self.images(device, &mut xs)?;
        }

        Ok(crate::alloc::collect_vec(
            xs.into_iter().map(|r| unsafe { NonNull::new_unchecked(r) }),
        ))
    }
}
