use crate::{vk::*, DeviceChild, VkHandle, VkRawHandle};
use derives::{implements, transparent_marked};

use super::CompletionHandlerMut;

pub trait Swapchain: VkHandle<Handle = VkSwapchainKHR> + DeviceChild {
    fn format(&self) -> VkFormat;
    fn size(&self) -> &VkExtent2D;

    #[inline(always)]
    fn as_transparent_ref(&self) -> SwapchainRef<Self> {
        SwapchainRef(self.native_ptr(), core::marker::PhantomData)
    }

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
    fn acquire_next(&mut self, timeout: Option<u64>, completion: CompletionHandlerMut) -> crate::Result<u32> {
        let (semaphore, fence) = match completion {
            CompletionHandlerMut::Host(f) => (VkSemaphore::NULL, f.0),
            CompletionHandlerMut::Queue(s) => (s.0, VkFence::NULL),
        };

        let mut n = 0;
        unsafe {
            crate::vkfn::acquire_next_image_khr(
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
            crate::vkfn::get_swapchain_images_khr(
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
            crate::vkfn::get_swapchain_images_khr(self.device().native_ptr(), self.native_ptr(), &mut n, v.as_mut_ptr())
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
    #[inline(always)]
    fn format(&self) -> VkFormat {
        T::format(self)
    }

    #[inline(always)]
    fn size(&self) -> &VkExtent2D {
        T::size(self)
    }
});

#[transparent_marked]
pub struct SwapchainRef<'r, R: crate::Swapchain + ?Sized>(
    pub(crate) VkSwapchainKHR,
    pub(crate) core::marker::PhantomData<&'r R>,
);
impl<'r, R: crate::Swapchain + ?Sized> VkHandle for SwapchainRef<'r, R> {
    type Handle = VkSwapchainKHR;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle {
        self.0
    }
}
