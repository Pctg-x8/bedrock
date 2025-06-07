use crate::ffi_helper::slice_as_ptr_empty_null;
use crate::*;
use derives::implements;

use super::DeviceChildHandle;

/// Opaque handle to as swapchain object, backed with specific surface
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkSwapchainKHR::OBJECT_TYPE)]
pub struct SurfaceSwapchainObject<Device: VkHandle<Handle = VkDevice>, Surface: VkHandle<Handle = VkSurfaceKHR>> {
    #[handle]
    pub(crate) handle: VkSwapchainKHR,
    pub(crate) device: Device,
    pub(crate) surface: Surface,
    pub(crate) format: VkFormat,
    pub(crate) extent: VkExtent2D,
}
#[implements]
impl<Device, Surface> Drop for SurfaceSwapchainObject<Device, Surface>
where
    Device: VkHandle<Handle = VkDevice>,
    Surface: VkHandle<Handle = VkSurfaceKHR>,
{
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy(self.device.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device, Surface> Sync for SurfaceSwapchainObject<Device, Surface>
where
    Device: VkHandle<Handle = VkDevice> + Sync,
    Surface: VkHandle<Handle = VkSurfaceKHR> + Sync,
{
}
unsafe impl<Device, Surface> Send for SurfaceSwapchainObject<Device, Surface>
where
    Device: VkHandle<Handle = VkDevice> + Send,
    Surface: VkHandle<Handle = VkSurfaceKHR> + Send,
{
}
impl<Device, Surface> DeviceChildHandle for SurfaceSwapchainObject<Device, Surface>
where
    Device: VkHandle<Handle = VkDevice>,
    Surface: VkHandle<Handle = VkSurfaceKHR>,
{
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.device.native_ptr()
    }
}
impl<Device, Surface> DeviceChild for SurfaceSwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: VkHandle<Handle = VkSurfaceKHR>,
{
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.device
    }
}
impl<Device, Surface> Swapchain for SurfaceSwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: VkHandle<Handle = VkSurfaceKHR>,
{
    #[inline(always)]
    fn format(&self) -> VkFormat {
        self.format
    }

    #[inline(always)]
    fn size(&self) -> &VkExtent2D {
        &self.extent
    }
}
impl<Device, Surface> SurfaceSwapchainObject<Device, Surface>
where
    Device: VkHandle<Handle = VkDevice>,
    Surface: VkHandle<Handle = VkSurfaceKHR>,
{
    /// Deconstructs the swapchain and retrieves its parents
    #[implements]
    pub fn deconstruct(self) -> (Device, Surface) {
        let d = unsafe { core::ptr::read(&self.device) };
        let s = unsafe { core::ptr::read(&self.surface) };

        // Note: DeviceとSurfaceをdropさせない（Swapchainだけ消す）
        unsafe {
            self.handle.destroy(self.device.native_ptr(), core::ptr::null());
        }
        core::mem::forget(self);

        (d, s)
    }
}
impl<Surface: crate::Surface> super::TransferSurfaceObject for SwapchainBuilder<'_, Surface> {
    type ConcreteSurface = Surface;

    #[inline(always)]
    fn transfer_surface(self) -> Self::ConcreteSurface {
        self.1
    }
}

/// Builder object to construct a `Swapchain`, backed with a surface
pub struct SwapchainBuilder<'n, Surface: crate::Surface>(
    VkSwapchainCreateInfoKHR,
    Surface,
    core::marker::PhantomData<Option<&'n dyn VulkanStructure>>,
);
impl<'n, Surface: crate::Surface> SwapchainBuilder<'n, Surface> {
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
                preTransform: VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR,
                compositeAlpha: VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR,
                presentMode: PresentMode::FIFO as _,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: core::ptr::null(),
                clipped: false as _,
                oldSwapchain: VkSwapchainKHR::NULL,
            },
            surface,
            core::marker::PhantomData,
        )
    }

    pub const fn with_next(mut self, next: &'n (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    pub const fn array_layers(mut self, layers: u32) -> Self {
        self.0.imageArrayLayers = layers;
        self
    }

    pub const fn shared(mut self, queue_families: &[u32]) -> Self {
        assert!(queue_families.len() > 0, "empty families not allowed");

        self.0.imageSharingMode = if queue_families.is_empty() {
            VK_SHARING_MODE_EXCLUSIVE
        } else {
            VK_SHARING_MODE_CONCURRENT
        };
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
            crate::vkfn::create_swapchain_khr(device.native_ptr(), &structure, std::ptr::null(), h.as_mut_ptr())
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
impl<Surface: crate::Surface> VulkanStructureProvider for SwapchainBuilder<'_, Surface> {
    type RootStructure = VkSwapchainCreateInfoKHR;

    #[inline(always)]
    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut VkSwapchainCreateInfoKHR) -> &'r mut GenericVulkanStructure {
        *root = self.0.clone();
        root.as_generic_mut()
    }
}
