//! Vulkan Surface/Swapchain Extensions

use super::*;

/// Opaque handle to a surface object
#[cfg(feature = "VK_KHR_surface")]
#[derive(VkHandle, VkObject, InstanceChild)]
#[VkObject(type = VK_OBJECT_TYPE_SURFACE_KHR)]
pub struct SurfaceObject<Instance: crate::Instance>(pub(crate) VkSurfaceKHR, #[parent] pub(crate) Instance);
#[cfg(feature = "VK_KHR_surface")]
unsafe impl<Instance: crate::Instance + Sync> Sync for SurfaceObject<Instance> {}
#[cfg(feature = "VK_KHR_surface")]
unsafe impl<Instance: crate::Instance + Send> Send for SurfaceObject<Instance> {}
#[cfg(feature = "VK_KHR_surface")]
#[implements]
impl<Instance: crate::Instance> Drop for SurfaceObject<Instance> {
    fn drop(&mut self) {
        unsafe {
            crate::vkresolve::destroy_surface_khr(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}
#[cfg(feature = "VK_KHR_surface")]
impl<Instance: crate::Instance> Surface for SurfaceObject<Instance> {}

#[cfg(feature = "VK_KHR_surface")]
pub trait Surface: VkHandle<Handle = VkSurfaceKHR> + InstanceChild {}
#[cfg(feature = "VK_KHR_surface")]
DerefContainerBracketImpl!(for Surface {});

/// Opaque handle to as swapchain object
#[cfg(feature = "VK_KHR_swapchain")]
#[derive(VkHandle, VkObject, DeviceChild)]
#[VkObject(type = VK_OBJECT_TYPE_SWAPCHAIN_KHR)]
pub struct SwapchainObject<Device: crate::Device, Surface: crate::Surface> {
    #[handle]
    pub(crate) handle: VkSwapchainKHR,
    #[parent]
    pub(crate) device: Device,
    pub(crate) surface: Surface,
    pub(crate) format: VkFormat,
    pub(crate) extent: VkExtent2D,
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
#[implements]
impl<Device, Surface> Drop for SwapchainObject<Device, Surface>
where
    Device: crate::Device,
    Surface: crate::Surface,
{
    fn drop(&mut self) {
        unsafe {
            crate::vkresolve::destroy_swapchain_khr(self.device.native_ptr(), self.handle, std::ptr::null());
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
        self.format
    }

    fn size(&self) -> &VkExtent2D {
        &self.extent
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
    #[implements]
    pub fn deconstruct(self) -> (Device, Surface) {
        let d = unsafe { core::ptr::read(&self.device) };
        let s = unsafe { core::ptr::read(&self.surface) };
        // Note: DeviceとSurfaceをdropさせたくない
        unsafe {
            crate::vkresolve::destroy_swapchain_khr(self.device.native_ptr(), self.handle, std::ptr::null());
        }
        std::mem::forget(self);

        (d, s)
    }
}

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
        format: VkSurfaceFormatKHR,
        extent: VkExtent2D,
        usage: ImageUsage,
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
                imageUsage: usage.0,
                imageSharingMode: VK_SHARING_MODE_EXCLUSIVE,
                preTransform: SurfaceTransform::Inherit as _,
                compositeAlpha: CompositeAlpha::Inherit as _,
                presentMode: PresentMode::FIFO as _,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: std::ptr::null(),
                clipped: VK_FALSE,
                oldSwapchain: VkSwapchainKHR::NULL,
            },
            surface,
        )
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
        self.0.pQueueFamilyIndices = queue_families.as_ptr();
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
    pub fn create<Device: crate::Device>(mut self, device: Device) -> crate::Result<SwapchainObject<Device, Surface>> {
        let mut h = core::mem::MaybeUninit::uninit();
        let mut structure = core::mem::MaybeUninit::uninit();
        self.build(unsafe { &mut *structure.as_mut_ptr() });
        let structure = unsafe { structure.assume_init() };

        unsafe {
            crate::vkresolve::create_swapchain_khr(device.native_ptr(), &structure, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| SwapchainObject {
                    handle: h.assume_init(),
                    device,
                    surface: self.1,
                    format: structure.imageFormat,
                    extent: structure.imageExtent,
                })
        }
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(feature = "VK_KHR_surface")]
impl<Surface: crate::Surface> VulkanStructureProvider for SwapchainBuilder<Surface> {
    type RootStructure = VkSwapchainCreateInfoKHR;

    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut VkSwapchainCreateInfoKHR) -> &'r mut GenericVulkanStructure {
        *root = self.0.clone();
        root.as_generic_mut()
    }
}

#[cfg(feature = "VK_KHR_surface")]
pub trait TransferSurfaceObject {
    type ConcreteSurface: crate::Surface;

    fn transfer_surface(self) -> Self::ConcreteSurface;
}
#[cfg(feature = "VK_KHR_surface")]
impl<Parent: VulkanStructureProvider + TransferSurfaceObject, T> TransferSurfaceObject for Extends<Parent, T> {
    type ConcreteSurface = Parent::ConcreteSurface;

    fn transfer_surface(self) -> Self::ConcreteSurface {
        self.0.transfer_surface()
    }
}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(feature = "VK_KHR_swapchain")]
impl<Surface: crate::Surface> TransferSurfaceObject for SwapchainBuilder<Surface> {
    type ConcreteSurface = Surface;

    fn transfer_surface(self) -> Self::ConcreteSurface {
        self.1
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
    fn size(&self) -> &VkExtent2D;

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
    fn acquire_next(
        &mut self,
        timeout: Option<u64>,
        completion: CompletionHandler<impl crate::Fence, impl crate::Semaphore>,
    ) -> crate::Result<u32> {
        let (semaphore, fence) = match completion {
            CompletionHandler::Host(f) => (VkSemaphore::NULL, f.native_ptr()),
            CompletionHandler::Queue(s) => (s.native_ptr(), VkFence::NULL),
        };
        let mut n = 0;
        unsafe {
            crate::vkresolve::acquire_next_image_khr(
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
    #[implements]
    fn queue_present(
        &mut self,
        queue: &mut impl VkHandle<Handle = VkQueue>,
        index: u32,
        wait_semaphores: &[impl VkHandle<Handle = VkSemaphore>],
    ) -> crate::Result<()> {
        let mut res = 0;
        let wait_semaphores = wait_semaphores.iter().map(VkHandle::native_ptr).collect::<Vec<_>>();
        let pinfo = VkPresentInfoKHR {
            sType: VkPresentInfoKHR::TYPE,
            pNext: std::ptr::null(),
            waitSemaphoreCount: wait_semaphores.len() as _,
            pWaitSemaphores: wait_semaphores.as_ptr(),
            swapchainCount: 1,
            pSwapchains: &self.native_ptr(),
            pImageIndices: &index,
            pResults: &mut res,
        };
        unsafe {
            crate::vkresolve::queue_present_khr(queue.native_ptr(), &pinfo)
                .into_result()
                .and_then(|x| if x.0 == VK_SUCCESS { Ok(()) } else { Err(x) })
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
    #[implements]
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    fn acquire_full_screen_exclusive_mode(&self) -> crate::Result<()> {
        unsafe {
            VkResultBox(self.device().acquire_full_screen_exclusive_mode_ext_fn().0(
                self.device().native_ptr(),
                self.native_ptr(),
            ))
            .into_result()
            .map(drop)
        }
    }

    /// Release full-screen exclusive mode from a swapchain.
    #[implements]
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    fn release_full_screen_exclusive_mode(&self) -> crate::Result<()> {
        unsafe {
            VkResultBox(self.device().release_full_screen_exclusive_mode_ext_fn().0(
                self.device().native_ptr(),
                self.native_ptr(),
            ))
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
            crate::vkresolve::get_swapchain_images_khr(
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
            crate::vkresolve::get_swapchain_images_khr(
                self.device().native_ptr(),
                self.native_ptr(),
                &mut n,
                v.as_mut_ptr(),
            )
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

    fn size(&self) -> &VkExtent2D {
        T::size(self)
    }
});

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
            sType: VkSurfaceFullScreenExclusiveInfoEXT::TYPE,
            pNext: std::ptr::null_mut(),
            fullScreenExclusive: flags as _,
        })
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl From<FullScreenExclusiveInfoEXT> for VkSurfaceFullScreenExclusiveInfoEXT {
    fn from(v: FullScreenExclusiveInfoEXT) -> Self {
        v.0
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(feature = "VK_KHR_surface")]
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
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[cfg(windows)]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct FullScreenExclusiveWin32InfoEXT(VkSurfaceFullScreenExclusiveWin32InfoEXT);
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[cfg(windows)]
impl FullScreenExclusiveWin32InfoEXT {
    /// Constructs the structure, with a handle identifying the display to create the surface with.
    pub fn new(handle: windows::Win32::Graphics::Gdi::HMONITOR) -> Self {
        Self(VkSurfaceFullScreenExclusiveWin32InfoEXT {
            sType: VkSurfaceFullScreenExclusiveWin32InfoEXT::TYPE,
            pNext: std::ptr::null(),
            hmonitor: handle,
        })
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[cfg(windows)]
impl From<FullScreenExclusiveWin32InfoEXT> for VkSurfaceFullScreenExclusiveWin32InfoEXT {
    fn from(v: FullScreenExclusiveWin32InfoEXT) -> Self {
        v.0
    }
}
#[cfg(windows)]
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(feature = "VK_KHR_surface")]
impl<T> Extendable<FullScreenExclusiveWin32InfoEXT> for T where
    T: VulkanStructureProvider<RootStructure = VkSwapchainCreateInfoKHR>
{
}
