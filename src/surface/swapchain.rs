use crate::{
    ffi_helper::ArrayFFIExtensions, vk::*, CompletionHandler, DeviceChild, VkHandle, VkRawHandle, VulkanStructure,
};
use derives::implements;

pub trait Swapchain: VkHandle<Handle = VkSwapchainKHR> + DeviceChild {
    fn format(&self) -> VkFormat;
    fn size(&self) -> &VkExtent2D;

    /// Retrieve the index of the next available presentation image
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    /// * `VK_ERROR_OUT_OF_DATE_KHR`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements]
    fn acquire_next(
        &mut self,
        timeout: Option<u64>,
        completion: CompletionHandler<impl crate::Fence, impl crate::Semaphore>,
    ) -> crate::Result<u32> {
        let (semaphore, fence) = match completion {
            CompletionHandler::Host(f) => (VkSemaphore::NULL, f.native_ptr()),
            CompletionHandler::Queue(s) => (s.native_ptr(), VkFence::NULL),
        };
        let mut n = 0;
        unsafe {
            crate::vkresolve::acquire_next_image_khr(
                self.device().native_ptr(),
                self.native_ptr(),
                timeout.unwrap_or(std::u64::MAX),
                semaphore,
                fence,
                &mut n,
            )
            .into_result()
            .map(|_| n)
        }
    }

    /// Queue an image for presentation
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    /// * `VK_ERROR_OUT_OF_DATE_KHR`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements]
    fn queue_present(
        &mut self,
        queue: &mut impl VkHandle<Handle = VkQueue>,
        index: u32,
        wait_semaphores: &[impl VkHandle<Handle = VkSemaphore>],
    ) -> crate::Result<()> {
        let mut res = VkResult(0);
        let wait_semaphores = wait_semaphores.iter().map(VkHandle::native_ptr).collect::<Vec<_>>();
        let pinfo = VkPresentInfoKHR {
            sType: VkPresentInfoKHR::TYPE,
            pNext: std::ptr::null(),
            waitSemaphoreCount: wait_semaphores.len() as _,
            pWaitSemaphores: wait_semaphores.as_ptr_empty_null(),
            swapchainCount: 1,
            pSwapchains: &self.native_ptr(),
            pImageIndices: &index,
            pResults: &mut res,
        };
        unsafe {
            crate::vkresolve::queue_present_khr(queue.native_ptr(), &pinfo)
                .into_result()
                .and_then(|x| if x == VK_SUCCESS { Ok(()) } else { Err(x) })
        }
    }

    /// Acquire full-screen exclusive mode for a swapchain.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_INITIALIZATION_FAILED`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_EXT_full_screen_exclusive")]
    fn acquire_full_screen_exclusive_mode(&self) -> crate::Result<()> {
        use crate::Device;

        unsafe {
            self.device().acquire_full_screen_exclusive_mode_ext_fn().0(self.device().native_ptr(), self.native_ptr())
                .into_result()
                .map(drop)
        }
    }

    /// Release full-screen exclusive mode from a swapchain.
    #[implements("VK_EXT_full_screen_exclusive")]
    fn release_full_screen_exclusive_mode(&self) -> crate::Result<()> {
        use crate::Device;

        unsafe {
            self.device().release_full_screen_exclusive_mode_ext_fn().0(self.device().native_ptr(), self.native_ptr())
                .into_result()
                .map(drop)
        }
    }

    /// Obtain the array of presentable images associated with a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    fn get_images(&self) -> crate::Result<Vec<crate::SwapchainImage<&Self>>>
    where
        Self: Sized,
    {
        let mut n = 0;
        unsafe {
            crate::vkresolve::get_swapchain_images_khr(
                self.device().native_ptr(),
                self.native_ptr(),
                &mut n,
                std::ptr::null_mut(),
            )
            .into_result()?;
        }
        let mut v = Vec::with_capacity(n as _);
        unsafe {
            v.set_len(n as _);
            crate::vkresolve::get_swapchain_images_khr(
                self.device().native_ptr(),
                self.native_ptr(),
                &mut n,
                v.as_mut_ptr(),
            )
            .into_result()
            .map(|_| {
                v.into_iter()
                    .map(|r| crate::SwapchainImage(r, self, self.size().clone().with_depth(1)))
                    .collect()
            })
        }
    }
}
DerefContainerBracketImpl!(for Swapchain {
    fn format(&self) -> VkFormat {
        T::format(self)
    }

    fn size(&self) -> &VkExtent2D {
        T::size(self)
    }
});
