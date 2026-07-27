use crate::{ffi_helper::slice_as_ptr_empty_null, *};
use bedrock_vk::{self as brvk, TypedVulkanStructure};
use derives::implements;

pub trait Swapchain: VkHandle<Handle = brvk::VkSwapchainKHR> + DeviceChild {
    /// Obtain a count of the array of presentable images associated with a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    fn image_count(&self) -> crate::Result<u32> {
        unsafe {
            crate::vkfn_wrapper::get_swapchain_image_count(self.device_transparent_ref(), self.as_transparent_ref())
        }
    }

    /// Obtain the array of presentable images associated with a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline(always)]
    fn images(&self, sink: &mut [core::mem::MaybeUninit<brvk::VkImage>]) -> crate::Result<ArrayQueryResult<u32>> {
        unsafe {
            crate::vkfn_wrapper::get_swapchain_images(self.device_transparent_ref(), self.as_transparent_ref(), sink)
        }
    }

    /// Acquire full-screen exclusive mode for a swapchain.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_INITIALIZATION_FAILED`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    #[implements("VK_EXT_full_screen_exclusive")]
    #[inline(always)]
    fn acquire_full_screen_exclusive_mode(&self) -> crate::Result<()>
    where
        Self::ConcreteDevice: DeviceFullScreenExclusiveExtension,
    {
        unsafe {
            self.device()
                .acquire_full_screen_exclusive_mode(self.device_transparent_ref(), self.as_transparent_ref())
        }
    }

    /// Release full-screen exclusive mode from a swapchain.
    #[implements("VK_EXT_full_screen_exclusive")]
    #[inline(always)]
    fn release_full_screen_exclusive_mode(&self) -> crate::Result<()>
    where
        Self::ConcreteDevice: DeviceFullScreenExclusiveExtension,
    {
        unsafe {
            self.device()
                .release_full_screen_exclusive_mode(self.device_transparent_ref(), self.as_transparent_ref())
        }
    }
}
DerefContainerBracketImpl!(for Swapchain {});

pub trait SwapchainMut: Swapchain + VkHandleMut {
    /// Retrieve the index of the next available presentation image
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_DEVICE_LOST`
    /// * `brvk::VK_ERROR_OUT_OF_DATE_KHR`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    #[implements]
    fn acquire_next(&mut self, timeout: Option<u64>, completion: CompletionHandlerMut) -> crate::Result<u32> {
        let (semaphore, fence) = match completion {
            CompletionHandlerMut::Host(f) => (None, Some(f)),
            CompletionHandlerMut::Queue(s) => (Some(s), None),
        };

        unsafe {
            crate::vkfn_wrapper::acquire_next_image(
                self.device_transparent_ref(),
                VkHandleRefMut::dangling(self.native_ptr()),
                timeout.unwrap_or(u64::MAX),
                semaphore,
                fence,
            )
        }
    }
}
DerefContainerBracketImpl!(for mut SwapchainMut {});

pub trait SwapchainImageExt: Swapchain {
    fn format(&self) -> brvk::VkFormat;
    fn extent(&self) -> brvk::VkExtent2D;

    /// Obtain the array of presentable images associated with a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("alloc")]
    fn images_alloc(&self) -> crate::Result<Vec<crate::SwapchainImage<&Self>>> {
        let n = self.image_count()? as usize;
        if n == 0 {
            // no items
            return Ok(crate::alloc::empty_sink_buffer());
        }

        let mut xs = crate::alloc::reserve(n);
        let res = self.images(xs.spare_capacity_mut())?.assert_complete();
        unsafe {
            xs.set_len(res as _);
        }

        Ok(crate::alloc::collect_vec(xs.into_iter().map(move |r| {
            let ext = self.extent();
            crate::SwapchainImage(
                r,
                self,
                self.format(),
                brvk::VkExtent3D {
                    width: ext.width,
                    height: ext.height,
                    depth: 1,
                },
            )
        })))
    }
}
DerefContainerBracketImpl!(for SwapchainImageExt {
    #[inline(always)]
    fn format(&self) -> brvk::VkFormat {
        T::format(self)
    }

    #[inline(always)]
    fn extent(&self) -> brvk::VkExtent2D {
        T::extent(self)
    }
});

#[repr(transparent)]
#[derive(Clone)]
pub struct SwapchainCreateInfo<'r, 'n, 'sw>(
    brvk::VkSwapchainCreateInfoKHR,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        &'r dyn VkHandle<Handle = brvk::VkSurfaceKHR>,
        Option<&'n dyn brvk::VulkanStructure>,
        Option<&'r [u32]>,
        Option<&'sw dyn VkHandle<Handle = brvk::VkSwapchainKHR>>,
    )>,
);
impl<'r, 'n, 'sw> SwapchainCreateInfo<'r, 'n, 'sw> {
    #[inline]
    pub fn new(
        surface: &'r (impl VkHandle<Handle = brvk::VkSurfaceKHR> + ?Sized),
        min_image_count: u32,
        format: brvk::VkSurfaceFormatKHR,
        extent: brvk::VkExtent2D,
        usage: ImageUsageFlags,
    ) -> Self {
        Self(
            brvk::VkSwapchainCreateInfoKHR {
                sType: brvk::VkSwapchainCreateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                surface: surface.native_ptr(),
                minImageCount: min_image_count,
                imageFormat: format.format,
                imageColorSpace: format.colorSpace,
                imageExtent: extent,
                imageArrayLayers: 1,
                imageUsage: usage.bits(),
                imageSharingMode: brvk::VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: core::ptr::null(),
                preTransform: brvk::VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR,
                compositeAlpha: brvk::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR,
                presentMode: brvk::VK_PRESENT_MODE_IMMEDIATE_KHR,
                clipped: false as _,
                oldSwapchain: None,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid `VkSwapchainCreateInfoKHR` struct.
    pub const unsafe fn from_raw(raw: brvk::VkSwapchainCreateInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkSwapchainCreateInfoKHR {
        self.0
    }

    pub const fn with_next(mut self, next: &'n (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    pub const fn array_layers(mut self, layers: u32) -> Self {
        self.0.imageArrayLayers = layers;
        self
    }

    pub const fn shared(mut self, queue_families: &'r [u32]) -> Self {
        assert!(!queue_families.is_empty(), "empty families not allowed");

        self.0.imageSharingMode = brvk::VK_SHARING_MODE_CONCURRENT;
        self.0.queueFamilyIndexCount = queue_families.len() as _;
        self.0.pQueueFamilyIndices = slice_as_ptr_empty_null(queue_families);
        self
    }

    pub const fn exclusive(mut self) -> Self {
        self.0.imageSharingMode = brvk::VK_SHARING_MODE_EXCLUSIVE;
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
    pub fn old_swapchain(
        mut self,
        old_swapchain: &'sw (impl VkHandle<Handle = brvk::VkSwapchainKHR> + ?Sized),
    ) -> Self {
        self.0.oldSwapchain = Some(old_swapchain.native_ptr());
        self
    }
}
