use crate::{ffi_helper::slice_as_ptr_empty_null, *};
use derives::implements;

pub trait Swapchain: VkHandle<Handle = VkSwapchainKHR> + DeviceChild {
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
    fn acquire_full_screen_exclusive_mode(&self) -> crate::Result<()>
    where
        Self::ConcreteDevice: DeviceFullScreenExclusiveExtension,
    {
        unsafe {
            self.device().acquire_full_screen_exclusive_mode_ext_fn().0(self.device().native_ptr(), self.native_ptr())
                .into_result()
                .map(drop)
        }
    }

    /// Release full-screen exclusive mode from a swapchain.
    #[implements("VK_EXT_full_screen_exclusive")]
    fn release_full_screen_exclusive_mode(&self) -> crate::Result<()>
    where
        Self::ConcreteDevice: DeviceFullScreenExclusiveExtension,
    {
        unsafe {
            self.device().release_full_screen_exclusive_mode_ext_fn().0(self.device().native_ptr(), self.native_ptr())
                .into_result()
                .map(drop)
        }
    }

    /// Obtain a count of the array of presentable images associated with a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn image_count(&self) -> crate::Result<u32> {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_swapchain_images_khr(
                self.device_handle(),
                self.native_ptr(),
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
    #[implements]
    #[inline]
    fn images(&self, sink: &mut [core::mem::MaybeUninit<VkImage>]) -> crate::Result<(u32, crate::ArrayQueryResult)> {
        let mut n = sink.len() as _;
        let r = unsafe {
            crate::vkfn::get_swapchain_images_khr(
                self.device_handle(),
                self.native_ptr(),
                &mut n,
                sink.as_mut_ptr() as _,
            )
        };

        match r {
            VK_SUCCESS => Ok((n, crate::ArrayQueryResult::Complete)),
            VK_INCOMPLETE => Ok((n, crate::ArrayQueryResult::Incomplete)),
            e => Err(e),
        }
    }
}
DerefContainerBracketImpl!(for Swapchain {});

pub trait SwapchainImageExt: Swapchain {
    fn format(&self) -> VkFormat;
    fn extent(&self) -> VkExtent2D;

    /// Obtain the array of presentable images associated with a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("alloc")]
    fn images_alloc(&self) -> crate::Result<Vec<crate::SwapchainImage<&Self>>>
    where
        Self: Sized,
    {
        let n = self.image_count()? as usize;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = Vec::with_capacity(n);
        let (n, _) = self.images(xs.spare_capacity_mut())?;
        unsafe {
            xs.set_len(n as _);
        }

        Ok(crate::alloc::collect_vec(xs.into_iter().map(move |r| {
            crate::SwapchainImage(r, self, self.format(), self.extent().with_depth(1))
        })))
    }
}
DerefContainerBracketImpl!(for SwapchainImageExt {
    #[inline(always)]
    fn format(&self) -> VkFormat {
        T::format(self)
    }

    #[inline(always)]
    fn extent(&self) -> VkExtent2D {
        T::extent(self)
    }
});

#[repr(transparent)]
#[derive(Clone)]
pub struct SwapchainCreateInfo<'r, 'n, 'sw>(
    VkSwapchainCreateInfoKHR,
    core::marker::PhantomData<(
        &'r dyn VkHandle<Handle = VkSurfaceKHR>,
        Option<&'n dyn VulkanStructure>,
        Option<&'r [u32]>,
        Option<&'sw dyn VkHandle<Handle = VkSwapchainKHR>>,
    )>,
);
impl<'r, 'n, 'sw> SwapchainCreateInfo<'r, 'n, 'sw> {
    #[inline]
    pub fn new(
        surface: &'r (impl VkHandle<Handle = VkSurfaceKHR> + ?Sized),
        min_image_count: u32,
        format: VkSurfaceFormatKHR,
        extent: VkExtent2D,
        usage: ImageUsageFlags,
    ) -> Self {
        Self(
            VkSwapchainCreateInfoKHR {
                sType: VkSwapchainCreateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                surface: surface.native_ptr(),
                minImageCount: min_image_count,
                imageFormat: format.format,
                imageColorSpace: format.colorSpace,
                imageExtent: extent,
                imageArrayLayers: 1,
                imageUsage: usage.bits(),
                imageSharingMode: VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: core::ptr::null(),
                preTransform: VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR,
                compositeAlpha: VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR,
                presentMode: VK_PRESENT_MODE_IMMEDIATE_KHR,
                clipped: false as _,
                oldSwapchain: VkSwapchainKHR::NULL,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkSwapchainCreateInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkSwapchainCreateInfoKHR {
        self.0
    }

    pub const fn with_next(mut self, next: &'n (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    pub const fn array_layers(mut self, layers: u32) -> Self {
        self.0.imageArrayLayers = layers;
        self
    }

    pub const fn shared(mut self, queue_families: &'r [u32]) -> Self {
        assert!(queue_families.len() > 0, "empty families not allowed");

        self.0.imageSharingMode = VK_SHARING_MODE_CONCURRENT;
        self.0.queueFamilyIndexCount = queue_families.len() as _;
        self.0.pQueueFamilyIndices = slice_as_ptr_empty_null(queue_families);
        self
    }

    pub const fn exclusive(mut self) -> Self {
        self.0.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
        self.0.queueFamilyIndexCount = 0;
        self.0.pQueueFamilyIndices = core::ptr::null();

        self
    }

    /// Default: Inherit
    pub const fn pre_transform(mut self, tf: SurfaceTransformFlags) -> Self {
        self.0.preTransform = tf.bits();
        self
    }

    /// Default: Inherit
    pub const fn composite_alpha(mut self, a: CompositeAlphaFlags) -> Self {
        self.0.compositeAlpha = a.bits();
        self
    }

    /// Default: FIFO
    pub const fn present_mode(mut self, mode: PresentMode) -> Self {
        self.0.presentMode = mode as _;
        self
    }

    /// Enables whether the Vulkan implementation is allowed to discard rendering operations
    /// that affect regions of the surface which aren't visible
    pub const fn enable_clip(mut self) -> Self {
        self.0.clipped = true as _;
        self
    }

    #[inline(always)]
    pub fn old_swapchain(mut self, old_swapchain: &'sw (impl VkHandle<Handle = VkSwapchainKHR> + ?Sized)) -> Self {
        self.0.oldSwapchain = old_swapchain.native_ptr();
        self
    }
}
