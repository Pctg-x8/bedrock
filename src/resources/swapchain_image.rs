use crate::*;

/// Opaque handle to a image object, backed by Swapchain.
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkImage::OBJECT_TYPE)]
pub struct SwapchainImage<Swapchain>(
    pub(crate) VkImage,
    pub(crate) Swapchain,
    pub(crate) VkFormat,
    pub(crate) VkExtent3D,
);
unsafe impl<Swapchain: Sync> Sync for SwapchainImage<Swapchain> {}
unsafe impl<Swapchain: Send> Send for SwapchainImage<Swapchain> {}
impl<Swapchain: DeviceChildHandle> DeviceChildHandle for SwapchainImage<Swapchain> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.device_handle()
    }
}
impl<Swapchain: DeviceChild> DeviceChild for SwapchainImage<Swapchain> {
    type ConcreteDevice = Swapchain::ConcreteDevice;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        self.1.device()
    }
}
impl<Swapchain: crate::Swapchain> Image for SwapchainImage<Swapchain> {
    #[inline(always)]
    fn format(&self) -> VkFormat {
        self.2
    }

    #[inline(always)]
    fn size(&self) -> &VkExtent3D {
        &self.3
    }

    #[inline(always)]
    fn dimension(&self) -> VkImageViewType {
        VK_IMAGE_VIEW_TYPE_2D
    }
}
impl<Swapchain: Clone> SwapchainImage<&'_ Swapchain> {
    /// Clones parent reference
    #[inline(always)]
    pub fn clone_parent(self) -> SwapchainImage<Swapchain> {
        SwapchainImage(self.0, self.1.clone(), self.2, self.3)
    }
}
impl<Swapchain> SwapchainImage<Swapchain> {
    /// Purges the construct
    pub const fn unmanage(self) -> (VkImage, Swapchain, VkFormat, VkExtent3D) {
        let image = unsafe { core::ptr::read(&self.0) };
        let swapchain = unsafe { core::ptr::read(&self.1) };
        let format = unsafe { core::ptr::read(&self.2) };
        let extent = unsafe { core::ptr::read(&self.3) };
        core::mem::forget(self);

        (image, swapchain, format, extent)
    }
}
