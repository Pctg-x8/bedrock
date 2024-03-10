use derives::{bitflags_newtype, implements};

use crate::{vk::*, Extends, InstanceChild, VkHandle, VkObject, VkRawHandle, VulkanStructureProvider};

/// Opaque handle to a surface object
#[derive(VkHandle, VkObject, InstanceChild)]
#[VkObject(type = VkSurfaceKHR::OBJECT_TYPE)]
pub struct SurfaceObject<Instance: crate::Instance>(pub(crate) VkSurfaceKHR, #[parent] pub(crate) Instance);
unsafe impl<Instance: crate::Instance + Sync> Sync for SurfaceObject<Instance> {}
unsafe impl<Instance: crate::Instance + Send> Send for SurfaceObject<Instance> {}
#[implements]
impl<Instance: crate::Instance> Drop for SurfaceObject<Instance> {
    fn drop(&mut self) {
        unsafe {
            crate::vkresolve::destroy_surface_khr(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}
impl<Instance: crate::Instance> Surface for SurfaceObject<Instance> {}

pub trait Surface: VkHandle<Handle = VkSurfaceKHR> + InstanceChild {}
DerefContainerBracketImpl!(for Surface {});

pub trait TransferSurfaceObject {
    type ConcreteSurface: crate::Surface;

    fn transfer_surface(self) -> Self::ConcreteSurface;
}
impl<Parent: VulkanStructureProvider + TransferSurfaceObject, T> TransferSurfaceObject for Extends<Parent, T> {
    type ConcreteSurface = Parent::ConcreteSurface;

    fn transfer_surface(self) -> Self::ConcreteSurface {
        self.0.transfer_surface()
    }
}

#[cfg(feature = "VK_KHR_surface")]
/// Presentation mode supported for a surface
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PresentMode {
    /// The presentation engine does not wait for a vertical blanking period to update the current image, meaning
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

impl SurfaceTransform {
    /// Does the value contains this bits
    pub const fn contains(self, value: u32) -> bool {
        (value | self as u32) != 0
    }
}

impl CompositeAlpha {
    /// Does the value contains this bits
    pub const fn contains(self, value: u32) -> bool {
        (value | self as u32) != 0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags_newtype]
pub struct SurfaceTransformFlags(VkSurfaceTransformFlagsKHR);
impl SurfaceTransformFlags {
    /// The image content is presented without being transformed
    pub const IDENTITY: Self = Self(VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR);
    /// The image content is rotated 90 degrees clockwise
    pub const ROTATE_90: Self = Self(VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR);
    /// The image content is rotated 180 degrees clockwise
    pub const ROTATE_180: Self = Self(VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR);
    /// The image content is rotated 270 degrees clockwise
    pub const ROTATE_270: Self = Self(VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR);
    /// The image content is mirrored horizontally
    pub const HORIZONTAL_MIRROR: Self = Self(VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR);
    /// The image content is mirrored horizontally, then rotated 90 degrees clockwise
    pub const HORIZONTAL_MIRROR_ROTATE_90: Self = Self(VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR);
    /// The image content is mirrored horizontally, then rotated 180 degrees clockwise
    pub const HORIZONTAL_MIRROR_ROTATE_180: Self = Self(VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR);
    /// The image content is mirrored horizontally, then rotated 270 degrees clockwise
    pub const HORIZONTAL_MIRROR_ROTATE_270: Self = Self(VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR);
    /// The presentation transform is not specified, and is instead determined by platform-specific considerations and mechanisms outside Vulkan
    pub const INHERIT: Self = Self(VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR);
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags_newtype]
pub struct CompositeAlphaFlags(VkCompositeAlphaFlagsKHR);
impl CompositeAlphaFlags {
    /// The alpha channel, if it exists, of the image is ignored in the compositing process
    pub const OPAQUE: Self = Self(VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR);
    /// The alpha channel, if it exists, of the images is respected in the compositing process.
    /// The non-alpha channels of the image are expected to already be multiplied by the alpha channel by the application
    pub const PRE_MULTIPLIED: Self = Self(VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR);
    /// The alpha channel, if it exists, of the images is respected in the compositing process.
    /// The non-alpha channels of the image are not expected to already be multiplied by the alpha channel by the application;
    /// instead, the compositor will multiply the non-alpha channels of the image by the alpha channel during compositing
    pub const POST_MULTIPLIED: Self = Self(VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR);
    /// The way in which the presentation engine treats the alpha channel in the images is unknown to the Vulkan API.
    /// Instead, the application is responsible for setting the composite alpha blending mode using native window system commands
    pub const INHERIT: Self = Self(VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR);
}

// specification extensions
impl VkSurfaceCapabilitiesKHR {
    /// Supported transform flags by the surface.
    #[inline(always)]
    pub const fn supported_transforms(&self) -> SurfaceTransformFlags {
        SurfaceTransformFlags(self.supportedTransforms)
    }

    /// Supported composite-alpha flags by the surface.
    #[inline(always)]
    pub const fn supported_composite_alpha(&self) -> CompositeAlphaFlags {
        CompositeAlphaFlags(self.supportedCompositeAlpha)
    }

    /// returns (width, height), `None` if there is no value specified(=0xffff_ffff)
    #[inline(always)]
    pub const fn current_extent(&self) -> (Option<u32>, Option<u32>) {
        #[inline(always)]
        const fn conv(x: u32) -> Option<u32> {
            if x == 0xffff_ffff {
                None
            } else {
                Some(x)
            }
        }

        (conv(self.currentExtent.width), conv(self.currentExtent.height))
    }
}
