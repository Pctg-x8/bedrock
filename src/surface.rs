//! Vulkan Surface/Swapchain Extensions

use super::*;

/// Opaque handle to a surface object
#[cfg(feature = "VK_KHR_surface")]
#[derive(VkHandle)]
pub struct SurfaceObject<Instance: crate::Instance>(pub(crate) VkSurfaceKHR, pub(crate) Instance);
#[cfg(feature = "VK_KHR_surface")]
impl<Instance: crate::Instance> VkObject for SurfaceObject<Instance> {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_SURFACE_KHR;
}
#[cfg(feature = "VK_KHR_surface")]
unsafe impl<Instance: crate::Instance + Sync> Sync for SurfaceObject<Instance> {}
#[cfg(feature = "VK_KHR_surface")]
unsafe impl<Instance: crate::Instance + Send> Send for SurfaceObject<Instance> {}
#[cfg(feature = "VK_KHR_surface")]
impl<Instance: crate::Instance> InstanceChild for SurfaceObject<Instance> {
    type ConcreteInstance = Instance;

    fn instance(&self) -> &Instance {
        &self.1
    }
}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(feature = "Implements")]
impl<Instance: crate::Instance> Drop for SurfaceObject<Instance> {
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_surface_khr(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}
#[cfg(feature = "VK_KHR_surface")]
impl<Instance: crate::Instance> Surface for SurfaceObject<Instance> {}

/// Opaque handle to as swapchain object
#[cfg(feature = "VK_KHR_swapchain")]
#[derive(VkHandle)]
pub struct SwapchainObject<Device: crate::Device, Surface: crate::Surface>(
    pub(crate) VkSwapchainKHR,
    pub(crate) Device,
    pub(crate) Surface,
    pub(crate) VkFormat,
    pub(crate) VkExtent3D,
);
#[cfg(feature = "VK_KHR_swapchain")]
impl<Device: crate::Device, Surface: crate::Surface> VkObject for SwapchainObject<Device, Surface> {
    const TYPE: VkObjectType = VK_OBJECT_TYPE_SWAPCHAIN_KHR;
}
#[cfg(feature = "VK_KHR_swapchain")]
unsafe impl<Device, Surface> Sync for SwapchainObject<Device, Surface>
where
    Device: crate::Device + Sync,
    Surface: crate::Surface + Sync,
{
}
#[cfg(feature = "VK_KHR_swapchain")]
unsafe impl<Device, Surface> Send for SwapchainObject<Device, Surface>
where
    Device: crate::Device + Send,
    Surface: crate::Surface + Send,
{
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<Device, Surface> DeviceChild for SwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: crate::Surface,
{
    type ConcreteDevice = Device;

    fn device(&self) -> &Device {
        &self.1
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(feature = "Implements")]
impl<Device, Surface> Drop for SwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: crate::Surface,
{
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_swapchain_khr(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(feature = "VK_KHR_surface")]
impl<Device, Surface> Swapchain for SwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: crate::Surface,
{
    fn format(&self) -> VkFormat {
        self.3
    }

    fn size(&self) -> &VkExtent3D {
        &self.4
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(feature = "VK_KHR_surface")]
impl<Device, Surface> SwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: crate::Surface,
{
    /// Deconstructs the swapchain and retrieves its parents
    #[cfg(feature = "Implements")]
    pub fn deconstruct(self) -> (Device, Surface) {
        let d = unsafe { std::ptr::read(&self.1) };
        let s = unsafe { std::ptr::read(&self.2) };
        unsafe {
            Resolver::get().destroy_swapchain_khr(self.1.native_ptr(), self.0, std::ptr::null());
        }
        std::mem::forget(self);

        (d, s)
    }
}

#[cfg(feature = "VK_KHR_surface")]
pub trait Surface: VkHandle<Handle = VkSurfaceKHR> + InstanceChild {}
#[cfg(feature = "VK_KHR_surface")]
DerefContainerBracketImpl!(for Surface {});

/// Builder object to construct a `Swapchain`
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(feature = "VK_KHR_surface")]
pub struct SwapchainBuilder<Surface: crate::Surface>(VkSwapchainCreateInfoKHR, Surface);
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(feature = "VK_KHR_surface")]
impl<Surface: crate::Surface> SwapchainBuilder<Surface> {
    pub fn new(
        surface: Surface,
        min_image_count: u32,
        format: &VkSurfaceFormatKHR,
        extent: &VkExtent2D,
        usage: ImageUsage,
    ) -> Self {
        SwapchainBuilder(
            VkSwapchainCreateInfoKHR {
                surface: surface.native_ptr(),
                minImageCount: min_image_count,
                imageFormat: format.format,
                imageColorSpace: format.colorSpace,
                imageExtent: extent.clone().into(),
                imageArrayLayers: 1,
                imageUsage: usage.0,
                imageSharingMode: VK_SHARING_MODE_EXCLUSIVE,
                preTransform: SurfaceTransform::Inherit as _,
                compositeAlpha: CompositeAlpha::Inherit as _,
                presentMode: PresentMode::FIFO as _,
                ..Default::default()
            },
            surface,
        )
    }

    pub fn array_layers(&mut self, layers: u32) -> &mut Self {
        self.0.imageArrayLayers = layers;
        self
    }

    pub fn share(&mut self, queue_families: &[u32]) -> &mut Self {
        self.0.imageSharingMode = if queue_families.is_empty() {
            VK_SHARING_MODE_EXCLUSIVE
        } else {
            VK_SHARING_MODE_CONCURRENT
        };
        self.0.queueFamilyIndexCount = queue_families.len() as _;
        self.0.pQueueFamilyIndices = queue_families.as_ptr();
        self
    }

    pub fn pre_transform(&mut self, tf: SurfaceTransform) -> &mut Self {
        self.0.preTransform = tf as _;
        self
    }

    pub fn composite_alpha(&mut self, a: CompositeAlpha) -> &mut Self {
        self.0.compositeAlpha = a as _;
        self
    }

    pub fn present_mode(&mut self, mode: PresentMode) -> &mut Self {
        self.0.presentMode = mode as _;
        self
    }

    /// Enables whether the Vulkan implementation is allowed to discard rendering operations
    /// that affect regions of the surface which aren't visible
    pub fn enable_clip(&mut self) -> &mut Self {
        self.0.clipped = true as _;
        self
    }
}
impl<Surface: crate::Surface> VulkanStructureProvider for SwapchainBuilder<Surface> {
    type RootStructure = VkSwapchainCreateInfoKHR;

    fn build(&mut self, root: &mut VkSwapchainCreateInfoKHR) -> &mut GenericVulkanStructure {
        *root = self.0.clone();
        unsafe { std::mem::transmute(root) }
    }
}

pub trait TransferSurfaceObject {
    type ConcreteSurface: crate::Surface;

    fn transfer_surface(self) -> Self::ConcreteSurface;
}
impl<Surface: crate::Surface> TransferSurfaceObject for SwapchainBuilder<Surface> {
    type ConcreteSurface = Surface;

    fn transfer_surface(self) -> Self::ConcreteSurface {
        self.1
    }
}
impl<Parent: VulkanStructureProvider + TransferSurfaceObject, T> TransferSurfaceObject for Extends<Parent, T> {
    type ConcreteSurface = Parent::ConcreteSurface;

    fn transfer_surface(self) -> Self::ConcreteSurface {
        self.0.transfer_surface()
    }
}

/// A semaphore or a fence
pub enum CompletionHandler<Fence: crate::Fence, Semaphore: crate::Semaphore> {
    /// A Host synchronizer(aka Fence)
    Host(Fence),
    /// A Queue synchronizer(aka Semaphore)
    Queue(Semaphore),
}

#[cfg(feature = "VK_KHR_swapchain")]
pub trait Swapchain: VkHandle<Handle = VkSwapchainKHR> + DeviceChild {
    fn format(&self) -> VkFormat;
    fn size(&self) -> &VkExtent3D;

    /// Retrieve the index of the next available presentation image
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    /// * `VK_ERROR_OUT_OF_DATE_KHR`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[cfg(feature = "Implements")]
    fn acquire_next(
        &mut self,
        timeout: Option<u64>,
        completion: CompletionHandler<impl crate::Fence, impl crate::Semaphore>,
    ) -> crate::Result<u32> {
        let (semaphore, fence) = match completion {
            CompletionHandler::Host(f) => (VK_NULL_HANDLE as _, f.native_ptr()),
            CompletionHandler::Queue(s) => (s.native_ptr(), VK_NULL_HANDLE as _),
        };
        let mut n = 0;
        unsafe {
            Resolver::get()
                .acquire_next_image_khr(
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
    #[cfg(feature = "Implements")]
    fn queue_present(
        &mut self,
        queue: &mut impl VkHandle<Handle = VkQueue>,
        index: u32,
        wait_semaphores: &[impl VkHandle<Handle = VkSemaphore>],
    ) -> crate::Result<()> {
        let mut res = 0;
        let wait_semaphores = wait_semaphores.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();
        let pinfo = VkPresentInfoKHR {
            waitSemaphoreCount: wait_semaphores.len() as _,
            pWaitSemaphores: wait_semaphores.as_ptr(),
            swapchainCount: 1,
            pSwapchains: &self.native_ptr(),
            pImageIndices: &index,
            pResults: &mut res,
            ..Default::default()
        };
        unsafe {
            Resolver::get()
                .queue_present_khr(queue.native_ptr(), &pinfo)
                .into_result()
                .and_then(|_| res.into_result())
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
    #[cfg(all(feature = "VK_EXT_full_screen_exclusive", feature = "Implements"))]
    fn acquire_full_screen_exclusive_mode(&self) -> crate::Result<()> {
        let fp: PFN_vkAcquireFullScreenExclusiveModeEXT = self
            .device()
            .extra_procedure("vkAcquireFullScreenExclusiveModeEXT")
            .expect("No full screen exclusive extension procedure found");
        (fp)(self.device().native_ptr(), self.native_ptr()).into_result()
    }

    /// Release full-screen exclusive mode from a swapchain.
    #[cfg(all(feature = "VK_EXT_full_screen_exclusive", feature = "Implements"))]
    fn release_full_screen_exclusive_mode(&self) -> crate::Result<()> {
        let fp: PFN_vkReleaseFullScreenExclusiveModeEXT = self
            .device()
            .extra_procedure("vkReleaseFullScreenExclusiveModeEXT")
            .expect("No full screen exclusive extension procedure found");
        (fp)(self.device().native_ptr(), self.native_ptr()).into_result()
    }

    /// Obtain the array of presentable images associated with a swapchain
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[cfg(feature = "Implements")]
    fn get_images(&self) -> crate::Result<Vec<crate::SwapchainImage<&Self>>>
    where
        Self: Sized,
    {
        let mut n = 0;
        unsafe {
            Resolver::get()
                .get_swapchain_images_khr(
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
            Resolver::get()
                .get_swapchain_images_khr(self.device().native_ptr(), self.native_ptr(), &mut n, v.as_mut_ptr())
                .into_result()
                .map(|_| {
                    v.into_iter()
                        .map(|r| crate::SwapchainImage(r, self, self.format()))
                        .collect()
                })
        }
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
DerefContainerBracketImpl!(for Swapchain {
    fn format(&self) -> VkFormat {
        T::format(self)
    }

    fn size(&self) -> &VkExtent3D {
        T::size(self)
    }
});

#[cfg(feature = "Implements")]
impl<Device: crate::Device> Queue<Device> {
    /// Queue images for presentation
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_DEVICE_LOST`
    /// * `VK_ERROR_OUT_OF_DATE_KHR`
    /// * `VK_ERROR_SURFACE_LOST_KHR`
    #[cfg(feature = "VK_KHR_swapchain")]
    pub fn present(
        &mut self,
        swapchains: &[(&mut impl Swapchain, u32)],
        wait_semaphores: &[impl VkHandle<Handle = VkSemaphore>],
    ) -> crate::Result<Vec<VkResult>> {
        let mut res = vec![0; swapchains.len()];
        let wait_semaphores = wait_semaphores.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();
        let (swapchains, indices): (Vec<_>, Vec<_>) = swapchains.iter().map(|&(ref x, n)| (x.native_ptr(), n)).unzip();
        let pinfo = VkPresentInfoKHR {
            waitSemaphoreCount: wait_semaphores.len() as _,
            pWaitSemaphores: wait_semaphores.as_ptr(),
            swapchainCount: swapchains.len() as _,
            pSwapchains: swapchains.as_ptr(),
            pImageIndices: indices.as_ptr(),
            pResults: res.as_mut_ptr(),
            ..Default::default()
        };
        unsafe {
            Resolver::get()
                .queue_present_khr(self.native_ptr(), &pinfo)
                .into_result()
                .map(|_| res)
        }
    }
}

#[cfg(feature = "VK_KHR_surface")]
/// Presentation mode supported for a surface
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PresentMode {
    /// The presentatino engine does not wait for a vertical blanking period to update the current image, meaning
    /// this mode may result in visible tearing
    Immediate = VK_PRESENT_MODE_IMMEDIATE_KHR as _,
    /// The presentation engine waits for the next vertical blanking period to update the current image.
    /// Tearing cannot be observed. An internal single-entry queue is used to hold pending presentation requests.
    /// If the queue is full when a new presentation request is received, the new request replaces the existing entry, and any images
    /// associated with the prior entry become available for re-use by the application
    Mailbox = VK_PRESENT_MODE_MAILBOX_KHR as _,
    /// The presentation engine waits for the next vertical blanking period to update the current image.
    /// Tearing cannot be observed. An internal queue is used to hold pending presentation requests.
    /// New requests are appended to the end of the queue, and one request is removed from the beginning of the queue
    /// and processed during each vertical blanking period in which the queue is non-empty.
    FIFO = VK_PRESENT_MODE_FIFO_KHR as _,
    /// The presentation engine generally waits for the next vertical blanking period to update the currnt image.
    /// If a vertical blanking period has already passed since the last update of the current image then the presentation engine
    /// does not wait for another vertical blanking period for the update, meaning this mode may result in visible tearing in this case
    FIFORelaxed = VK_PRESENT_MODE_FIFO_RELAXED_KHR as _,
}

#[cfg(feature = "VK_KHR_surface")]
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SurfaceTransform {
    /// The image content is presented without being transformed
    Identity = VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR as _,
    /// The image content is rotated 90 degrees clockwise
    Rotate90 = VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR as _,
    /// The image content is rotated 180 degrees clockwise
    Rotate180 = VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR as _,
    /// The image content is rotated 270 degrees clockwise
    Rotate270 = VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR as _,
    /// The image content is mirrored horizontally
    HorizontalMirror = VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR as _,
    /// The image content is mirrored horizontally, then rotated 90 degrees clockwise
    HorizontalMirrorRotate90 = VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR as _,
    /// The image content is mirrored horizontally, then rotated 180 degrees clockwise
    HorizontalMirrorRotate180 = VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR as _,
    /// The image content is mirrored horizontally, then rotated 270 degrees clockwise
    HorizontalMirrorRotate270 = VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR as _,
    /// The presentation transform is not specified, and is instead determined by platform-specific considerations and mechanisms outside Vulkan
    Inherit = VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR as _,
}
#[cfg(feature = "VK_KHR_surface")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeAlpha {
    /// The alpha channel, if it exists, of the image is ignored in the compositing process
    Opaque = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR as _,
    /// The alpha channel, if it exists, of the images is respected in the compositing process.
    /// The non-alpha channels of the image are expected to already be multiplied by the alpha channel by the application
    PreMultiplied = VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR as _,
    /// The alpha channel, if it exists, of the images is respected in the compositing process.
    /// The non-alpha channels of the image are not expected to already be multiplied by the alpha channel by the application;
    /// instead, the compositor will multiply the non-alpha channels of the image by the alpha channel during compositing
    PostMultiplied = VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR as _,
    /// The way in which the presentation engine treats the alpha channel in the images is unknown to the Vulkan API.
    /// Instead, the application is responsible for setting the composite alpha blending mode using native window system commands
    Inherit = VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR as _,
}

#[cfg(feature = "VK_KHR_surface")]
impl SurfaceTransform {
    /// Does the value contains this bits
    pub const fn contains(self, value: u32) -> bool {
        (value | self as u32) != 0
    }
}
#[cfg(feature = "VK_KHR_surface")]
impl CompositeAlpha {
    /// Does the value contains this bits
    pub const fn contains(self, value: u32) -> bool {
        (value | self as u32) != 0
    }
}

// VK_EXT_full_screen_exclusive //

/// Wraps VkSurfaceFullScreenExclusiveInfoEXT structure: Specifying the preferred full-screen transition behavior
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct FullScreenExclusiveInfoEXT(VkSurfaceFullScreenExclusiveInfoEXT);
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl FullScreenExclusiveInfoEXT {
    /// Constructs the structure, specifying the preferred full-screen transition behavior.
    pub fn new(flags: FullScreenExclusiveEXT) -> Self {
        FullScreenExclusiveInfoEXT(VkSurfaceFullScreenExclusiveInfoEXT {
            fullScreenExclusive: flags as _,
            ..Default::default()
        })
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl<T> Extendable<FullScreenExclusiveInfoEXT> for T where
    T: VulkanStructureProvider<RootStructure = VkSwapchainCreateInfoKHR>
{
}

/// Hint values an application can specify affecting full-screen transition behavior
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FullScreenExclusiveEXT {
    /// The implmentation should determine the appropriate full-screen method by whatever means it deems appropriate.
    Default = VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT as _,
    /// The implementation may use full-screen exclusive mechanisms when available.
    /// Such mechanisms may result in better performance and/or the availability of different presentation capabilities,
    /// but may require a more disruptive transition during swapchain initialization,
    /// first presentation and/or destruction.
    Allowed = VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT as _,
    /// The implementation should avoid using full-screen mechanisms which rely on disruptive transitions.
    Disallowed = VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT as _,
    /// The application will manage full-screen exclusive mode by using the `vkAcquireFullScreenExclusiveModeEXT` and
    /// `vkReleaseFullScreenExclusiveEXT` commands.
    ApplicationControlled = VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT as _,
}

/// Wraps VkSurfaceFullScreenExclusiveWin32InfoEXT structure: Specifying additional creation parameters specific to Win32 fullscreen exclusive mode.
#[cfg(feature = "VK_EXT_full_screen_exclusive_win32")]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct FullScreenExclusiveWin32InfoEXT(VkSurfaceFullScreenExclusiveWin32InfoEXT);
#[cfg(feature = "VK_EXT_full_screen_exclusive_win32")]
impl FullScreenExclusiveWin32InfoEXT {
    /// Constructs the structure, with a handle identifying the display to create the surface with.
    pub fn new(handle: winapi::shared::windef::HMONITOR) -> Self {
        FullScreenExclusiveWin32InfoEXT(VkSurfaceFullScreenExclusiveWin32InfoEXT {
            hmonitor: handle,
            ..Default::default()
        })
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive_win32")]
impl<T> Extendable<FullScreenExclusiveWin32InfoEXT> for T where
    T: VulkanStructureProvider<RootStructure = VkSwapchainCreateInfoKHR>
{
}

#[cfg(feature = "VK_EXT_full_screen_exclusive_win32")]
impl<'d> crate::ext::Chainable<'d, FullScreenExclusiveWin32InfoEXT> for FullScreenExclusiveInfoEXT {
    fn chain(&mut self, next: &'d FullScreenExclusiveWin32InfoEXT) -> &mut Self {
        self.0.pNext = next as *const _ as _;
        self
    }
}

#[cfg(feature = "VK_EXT_full_screen_exclusive")]
unsafe impl crate::ext::VulkanStructure for VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT;
}
#[cfg(all(feature = "VK_EXT_full_screen_exclusive", feature = "VK_KHR_win32_surface"))]
unsafe impl crate::ext::VulkanStructure for VkSurfaceFullScreenExclusiveWin32InfoEXT {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT;
}
