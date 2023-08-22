#[implements]
use crate::VkHandleMut;
use crate::{vk::*, DeviceChild, DeviceMemory, Image, MemoryBound, VkHandle, VkObject, VkRawHandle};
use derives::implements;

/// Opaque handle to a image object, backed by Swapchain.
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkImage::OBJECT_TYPE)]
pub struct SwapchainImage<Swapchain: crate::Swapchain>(pub(crate) VkImage, pub(crate) Swapchain, pub(crate) VkExtent3D);
unsafe impl<Swapchain: crate::Swapchain + Sync> Sync for SwapchainImage<Swapchain> {}
unsafe impl<Swapchain: crate::Swapchain + Send> Send for SwapchainImage<Swapchain> {}
impl<Swapchain: crate::Swapchain> DeviceChild for SwapchainImage<Swapchain> {
    type ConcreteDevice = Swapchain::ConcreteDevice;

    fn device(&self) -> &Self::ConcreteDevice {
        self.1.device()
    }
}
impl<Swapchain: crate::Swapchain> Image for SwapchainImage<Swapchain> {
    fn format(&self) -> VkFormat {
        self.1.format()
    }

    fn size(&self) -> &VkExtent3D {
        &self.2
    }

    fn dimension(&self) -> VkImageViewType {
        VK_IMAGE_VIEW_TYPE_2D
    }
}
impl<Swapchain: crate::Swapchain> MemoryBound for SwapchainImage<Swapchain>
where
    Self: VkHandle<Handle = VkImage>,
{
    #[implements]
    fn requirements(&self) -> VkMemoryRequirements {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::get_image_memory_requirements(
                self.device().native_ptr(),
                self.native_ptr(),
                p.as_mut_ptr(),
            );

            p.assume_init()
        }
    }

    #[implements]
    fn bind(&mut self, memory: &(impl DeviceMemory + ?Sized), offset: usize) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkresolve::bind_image_memory(
                self.device().native_ptr(),
                self.native_ptr_mut(),
                memory.native_ptr(),
                offset as _,
            )
            .into_result()
            .map(drop)
        }
    }
}
impl<Swapchain: crate::Swapchain + Clone> SwapchainImage<&'_ Swapchain> {
    /// Clones parent reference
    pub fn clone_parent(self) -> SwapchainImage<Swapchain> {
        let r = SwapchainImage(self.0, self.1.clone(), self.2.clone());
        // disable dropping self.0
        std::mem::forget(self);
        r
    }
}
