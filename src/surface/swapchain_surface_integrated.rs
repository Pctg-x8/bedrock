use crate::ffi_helper::slice_as_ptr_empty_null;
use crate::*;
use bedrock_vk::{self as brvk, TypedVulkanStructure, VkRawHandle, VulkanStructure};
use derives::implements;

use super::DeviceChildHandle;

/// Opaque handle to as swapchain object, backed with specific surface
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VkSwapchainKHR::OBJECT_TYPE)]
pub struct SurfaceSwapchainObject<
    Device: VkHandle<Handle = brvk::VkDevice>,
    Surface: VkHandle<Handle = brvk::VkSurfaceKHR>,
> {
    #[handle]
    pub(crate) handle: brvk::VkSwapchainKHR,
    pub(crate) device: Device,
    pub(crate) surface: Surface,
    pub(crate) format: brvk::VkFormat,
    pub(crate) extent: brvk::VkExtent2D,
}
#[implements]
impl<Device, Surface> Drop for SurfaceSwapchainObject<Device, Surface>
where
    Device: VkHandle<Handle = brvk::VkDevice>,
    Surface: VkHandle<Handle = brvk::VkSurfaceKHR>,
{
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn_wrapper::destroy_swapchain(
                self.device_transparent_ref(),
                VkHandleRefMut::dangling(self.handle),
                None,
            );
        }
    }
}
unsafe impl<Device, Surface> Sync for SurfaceSwapchainObject<Device, Surface>
where
    Device: VkHandle<Handle = brvk::VkDevice> + Sync,
    Surface: VkHandle<Handle = brvk::VkSurfaceKHR> + Sync,
{
}
unsafe impl<Device, Surface> Send for SurfaceSwapchainObject<Device, Surface>
where
    Device: VkHandle<Handle = brvk::VkDevice> + Send,
    Surface: VkHandle<Handle = brvk::VkSurfaceKHR> + Send,
{
}
impl<Device, Surface> DeviceChildHandle for SurfaceSwapchainObject<Device, Surface>
where
    Device: VkHandle<Handle = brvk::VkDevice>,
    Surface: VkHandle<Handle = brvk::VkSurfaceKHR>,
{
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice {
        self.device.native_ptr()
    }
}
impl<Device, Surface> DeviceChild for SurfaceSwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: VkHandle<Handle = brvk::VkSurfaceKHR>,
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
    Surface: VkHandle<Handle = brvk::VkSurfaceKHR>,
{
}
impl<Device, Surface> SwapchainMut for SurfaceSwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: VkHandle<Handle = brvk::VkSurfaceKHR>,
{
}
impl<Device, Surface> SwapchainImageExt for SurfaceSwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: VkHandle<Handle = brvk::VkSurfaceKHR>,
{
    #[inline(always)]
    fn format(&self) -> brvk::VkFormat {
        self.format
    }

    #[inline(always)]
    fn extent(&self) -> brvk::VkExtent2D {
        self.extent
    }
}
impl<Device, Surface> SurfaceSwapchainObject<Device, Surface>
where
    Device: VkHandle<Handle = brvk::VkDevice>,
    Surface: VkHandle<Handle = brvk::VkSurfaceKHR>,
{
    /// Deconstructs this and take ownership of managed objects(no drop occured)
    pub const fn unmanage(self) -> (Device, Surface, brvk::VkSwapchainKHR) {
        let device = unsafe { core::ptr::read(&self.device) };
        let surface = unsafe { core::ptr::read(&self.surface) };
        let swapchain = self.handle;
        core::mem::forget(self);

        (device, surface, swapchain)
    }

    /// Deconstructs the swapchain and retrieves its parents
    #[implements]
    pub fn deconstruct(mut self) -> (Device, Surface) {
        let d = unsafe { core::ptr::read(&self.device) };
        let s = unsafe { core::ptr::read(&self.surface) };

        // Note: DeviceとSurfaceをdropさせない（Swapchainだけ消す）
        unsafe {
            crate::vkfn_wrapper::destroy_swapchain(d.as_transparent_ref(), self.as_transparent_ref_mut(), None);
        }
        core::mem::forget(self);

        (d, s)
    }
}
impl<Surface: crate::Surface> super::TransferSurfaceObject for SwapchainWithSurfaceBuilder<'_, '_, Surface> {
    type ConcreteSurface = Surface;

    #[inline(always)]
    fn transfer_surface(self) -> Self::ConcreteSurface {
        self.1
    }
}

/// Builder object to construct a `Swapchain`, backed with a surface
pub struct SwapchainWithSurfaceBuilder<'n, 'sw, Surface: crate::Surface>(
    brvk::VkSwapchainCreateInfoKHR,
    Surface,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        Option<&'n dyn brvk::VulkanStructure>,
        Option<&'sw dyn VkHandle<Handle = brvk::VkSwapchainKHR>>,
    )>,
);
impl<'n, 'sw, Surface: crate::Surface> SwapchainWithSurfaceBuilder<'n, 'sw, Surface> {
    pub fn new(
        surface: Surface,
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
                preTransform: brvk::VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR,
                compositeAlpha: brvk::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR,
                presentMode: PresentMode::FIFO as _,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: core::ptr::null(),
                clipped: false as _,
                oldSwapchain: None,
            },
            surface,
            core::marker::PhantomData,
        )
    }

    pub const fn with_next(mut self, next: &'n (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }

    pub const fn array_layers(mut self, layers: u32) -> Self {
        self.0.imageArrayLayers = layers;
        self
    }

    pub const fn shared(mut self, queue_families: &[u32]) -> Self {
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
    pub fn old_swapchain(mut self, sw: &'sw (impl VkHandle<Handle = brvk::VkSwapchainKHR> + ?Sized)) -> Self {
        self.0.oldSwapchain = Some(sw.native_ptr());
        self
    }

    /// Create a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_DEVICE_LOST`
    /// * `brvk::VK_ERROR_SURFACE_LOST_KHR`
    /// * `brvk::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
    #[implements]
    pub fn create<Device: crate::Device>(
        mut self,
        device: Device,
    ) -> crate::Result<SurfaceSwapchainObject<Device, Surface>> {
        let mut structure = core::mem::MaybeUninit::uninit();
        self.build(unsafe { &mut *structure.as_mut_ptr() });
        let structure = unsafe { structure.assume_init() };

        let handle = {
            crate::vkfn_wrapper::create_swapchain(
                device.as_transparent_ref(),
                unsafe { core::mem::transmute::<&brvk::VkSwapchainCreateInfoKHR, &SwapchainCreateInfo>(&structure) },
                None,
            )?
        };

        Ok(SurfaceSwapchainObject {
            handle,
            device,
            surface: self.1,
            format: structure.imageFormat,
            extent: structure.imageExtent,
        })
    }
}
impl<Surface: crate::Surface> VulkanStructureProvider for SwapchainWithSurfaceBuilder<'_, '_, Surface> {
    type RootStructure = brvk::VkSwapchainCreateInfoKHR;

    #[inline(always)]
    fn build<'r, 's: 'r>(
        &'s mut self,
        root: &'s mut brvk::VkSwapchainCreateInfoKHR,
    ) -> &'r mut brvk::GenericVulkanStructure {
        *root = self.0.clone();
        root.as_generic_mut()
    }
}
