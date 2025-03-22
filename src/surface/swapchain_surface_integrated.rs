use crate::{
    GenericVulkanStructure, ImageUsageFlags, PresentMode, VkObject, VkRawHandle, VulkanStructure, VulkanStructureAsRef,
    VulkanStructureProvider, ffi_helper::slice_as_ptr_empty_null, vk::*,
};

use super::{Extent2D, Surface, SurfaceFormat};

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwapchainCreateInfo<'r>(
    VkSwapchainCreateInfoKHR,
    core::marker::PhantomData<(&'r mut Surface, Option<&'r [u32]>)>,
);
impl<'r> SwapchainCreateInfo<'r> {
    #[inline]
    pub fn new(
        surface: &'r mut Surface,
        min_image_count: u32,
        format: SurfaceFormat,
        extent: Extent2D,
        usage: ImageUsageFlags,
    ) -> Self {
        Self(
            VkSwapchainCreateInfoKHR {
                sType: VkSwapchainCreateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                surface: surface as *mut _ as _,
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
    pub const fn pre_transform(mut self, tf: VkSurfaceTransformFlagsKHR) -> Self {
        self.0.preTransform = tf as _;
        self
    }

    /// Default: Inherit
    pub const fn composite_alpha(mut self, a: VkCompositeAlphaFlagsKHR) -> Self {
        self.0.compositeAlpha = a as _;
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
}
impl VulkanStructureProvider for SwapchainCreateInfo<'_> {
    type RootStructure = VkSwapchainCreateInfoKHR;

    #[inline(always)]
    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut VkSwapchainCreateInfoKHR) -> &'r mut GenericVulkanStructure {
        *root = self.0.clone();
        root.as_generic_mut()
    }
}
