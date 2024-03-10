use crate::{
    ffi_helper::ArrayFFIExtensions, vk::*, CompositeAlpha, DeviceChild, GenericVulkanStructure, ImageUsageFlags,
    PresentMode, SurfaceTransform, Swapchain, VkDeviceChildNonExtDestroyable, VkHandle, VkObject, VkRawHandle,
    VulkanStructure, VulkanStructureProvider,
};
use derives::implements;

/// Opaque handle to as swapchain object, backed with specific surface
#[derive(VkHandle, VkObject, DeviceChild)]
#[VkObject(type = VkSwapchainKHR::OBJECT_TYPE)]
pub struct SurfaceSwapchainObject<Device: crate::Device, Surface: crate::Surface> {
    #[handle]
    pub(crate) handle: VkSwapchainKHR,
    #[parent]
    pub(crate) device: Device,
    pub(crate) surface: Surface,
    pub(crate) format: VkFormat,
    pub(crate) extent: VkExtent2D,
}
unsafe impl<Device, Surface> Sync for SurfaceSwapchainObject<Device, Surface>
where
    Device: crate::Device + Sync,
    Surface: crate::Surface + Sync,
{
}
unsafe impl<Device, Surface> Send for SurfaceSwapchainObject<Device, Surface>
where
    Device: crate::Device + Send,
    Surface: crate::Surface + Send,
{
}
#[implements]
impl<Device, Surface> Drop for SurfaceSwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: crate::Surface,
{
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy(self.device.native_ptr(), core::ptr::null());
        }
    }
}
impl<Device, Surface> Swapchain for SurfaceSwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: crate::Surface,
{
    fn format(&self) -> VkFormat {
        self.format
    }

    fn size(&self) -> &VkExtent2D {
        &self.extent
    }
}
impl<Device, Surface> SurfaceSwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: crate::Surface,
{
    /// Deconstructs the swapchain and retrieves its parents
    #[implements]
    pub fn deconstruct(self) -> (Device, Surface) {
        let d = unsafe { core::ptr::read(&self.device) };
        let s = unsafe { core::ptr::read(&self.surface) };

        // Note: DeviceとSurfaceをdropさせたくない
        unsafe {
            self.handle.destroy(self.device.native_ptr(), core::ptr::null());
        }
        core::mem::forget(self);

        (d, s)
    }
}
impl<Surface: crate::Surface> super::TransferSurfaceObject for SwapchainBuilder<Surface> {
    type ConcreteSurface = Surface;

    fn transfer_surface(self) -> Self::ConcreteSurface {
        self.1
    }
}

/// Builder object to construct a `Swapchain`, backed with a surface
pub struct SwapchainBuilder<Surface: crate::Surface>(VkSwapchainCreateInfoKHR, Surface);
impl<Surface: crate::Surface> SwapchainBuilder<Surface> {
    pub fn new(
        surface: Surface,
        min_image_count: u32,
        format: VkSurfaceFormatKHR,
        extent: VkExtent2D,
        usage: ImageUsageFlags,
    ) -> Self {
        Self(
            VkSwapchainCreateInfoKHR {
                sType: VkSwapchainCreateInfoKHR::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                surface: surface.native_ptr(),
                minImageCount: min_image_count,
                imageFormat: format.format,
                imageColorSpace: format.colorSpace,
                imageExtent: extent,
                imageArrayLayers: 1,
                imageUsage: usage.into(),
                imageSharingMode: VK_SHARING_MODE_EXCLUSIVE,
                preTransform: SurfaceTransform::Inherit as _,
                compositeAlpha: CompositeAlpha::Inherit as _,
                presentMode: PresentMode::FIFO as _,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: std::ptr::null(),
                clipped: false as _,
                oldSwapchain: VkSwapchainKHR::NULL,
            },
            surface,
        )
    }

    pub const fn array_layers(mut self, layers: u32) -> Self {
        self.0.imageArrayLayers = layers;
        self
    }

    pub fn shared(mut self, queue_families: &[u32]) -> Self {
        assert!(queue_families.len() > 0, "empty families not allowed");

        self.0.imageSharingMode = if queue_families.is_empty() {
            VK_SHARING_MODE_EXCLUSIVE
        } else {
            VK_SHARING_MODE_CONCURRENT
        };
        self.0.queueFamilyIndexCount = queue_families.len() as _;
        self.0.pQueueFamilyIndices = queue_families.as_ptr_empty_null();
        self
    }

    pub const fn exclusive(mut self) -> Self {
        self.0.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
        self.0.queueFamilyIndexCount = 0;
        self.0.pQueueFamilyIndices = core::ptr::null();

        self
    }

    /// Default: Inherit
    pub const fn pre_transform(mut self, tf: SurfaceTransform) -> Self {
        self.0.preTransform = tf as _;
        self
    }

    /// Default: Inherit
    pub const fn composite_alpha(mut self, a: CompositeAlpha) -> Self {
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

    /// Create a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    /// * `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
    #[implements]
    pub fn create<Device: crate::Device>(
        mut self,
        device: Device,
    ) -> crate::Result<SurfaceSwapchainObject<Device, Surface>> {
        let mut h = core::mem::MaybeUninit::uninit();
        let mut structure = core::mem::MaybeUninit::uninit();
        self.build(unsafe { &mut *structure.as_mut_ptr() });
        let structure = unsafe { structure.assume_init() };

        unsafe {
            crate::vkresolve::create_swapchain_khr(device.native_ptr(), &structure, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| SurfaceSwapchainObject {
                    handle: h.assume_init(),
                    device,
                    surface: self.1,
                    format: structure.imageFormat,
                    extent: structure.imageExtent,
                })
        }
    }
}
impl<Surface: crate::Surface> VulkanStructureProvider for SwapchainBuilder<Surface> {
    type RootStructure = VkSwapchainCreateInfoKHR;

    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut VkSwapchainCreateInfoKHR) -> &'r mut GenericVulkanStructure {
        *root = self.0.clone();
        root.as_generic_mut()
    }
}
