use std::ops::{BitOr, BitOrAssign, Deref, DerefMut, Range};

use crate::{
    vk::*, DeviceChild, GenericVulkanStructure, ImageMemoryBarrier, MemoryBound, VkDeviceChildNonExtDestroyable,
    VkHandle, VkObject, VkRawHandle, VulkanStructure,
};
#[implements]
use crate::{DeviceMemory, VkHandleMut};
use derives::implements;

pub trait Image: VkHandle<Handle = VkImage> + DeviceChild {
    /// The pixel format of an image
    fn format(&self) -> VkFormat;

    /// The size of an image
    fn size(&self) -> &VkExtent3D;

    fn dimension(&self) -> VkImageViewType;

    /// Create an image view
    #[implements]
    #[deprecated = "use ImageViewBuilder which can be omit some default arguments"]
    fn create_view(
        self,
        format: Option<VkFormat>,
        vtype: Option<VkImageViewType>,
        cmap: &super::ComponentMapping,
        subresource_range: &VkImageSubresourceRange,
    ) -> crate::Result<ImageViewObject<Self>>
    where
        Self: Sized,
    {
        let (format, vtype) = (
            format.unwrap_or_else(|| self.format()),
            vtype.unwrap_or_else(|| self.dimension()),
        );
        let cinfo = VkImageViewCreateInfo {
            sType: VkImageViewCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            image: self.native_ptr(),
            viewType: vtype,
            format,
            components: cmap.clone().into(),
            subresourceRange: subresource_range.clone(),
        };
        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_image_view(self.device().native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| ImageViewObject(h.assume_init(), self))
        }
    }

    /// Retrieve information about an image subresource  
    /// Subresource: (`aspect`, `mipLevel`, `arrayLayer`)
    #[implements]
    #[deprecated = "use ImageSubresource"]
    fn image_subresource_layout(
        &self,
        subres_aspect: AspectMask,
        subres_mip_level: u32,
        subres_array_layer: u32,
    ) -> VkSubresourceLayout {
        let subres = VkImageSubresource {
            aspectMask: subres_aspect.0,
            mipLevel: subres_mip_level,
            arrayLayer: subres_array_layer,
        };
        let mut s = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::get_image_subresource_layout(
                self.device().native_ptr(),
                self.native_ptr(),
                &subres,
                s.as_mut_ptr(),
            );

            s.assume_init()
        }
    }

    /// Query the memory requirements for a sparse image
    #[cfg(feature = "Implements")]
    fn sparse_requirements(&self) -> Vec<VkSparseImageMemoryRequirements> {
        let mut n = 0;
        unsafe {
            crate::vkresolve::get_image_sparse_memory_requirements(
                self.device().native_ptr(),
                self.native_ptr(),
                &mut n,
                std::ptr::null_mut(),
            );
        };
        let mut v = Vec::with_capacity(n as _);
        unsafe {
            v.set_len(n as _);
            crate::vkresolve::get_image_sparse_memory_requirements(
                self.device().native_ptr(),
                self.native_ptr(),
                &mut n,
                v.as_mut_ptr(),
            )
        };

        v
    }

    /// Returns an image's DRM format modifier
    #[cfg(all(feature = "Implements", feature = "VK_EXT_image_drm_format_modifier"))]
    fn drm_format_modifier_properties(&self) -> crate::Result<VkImageDrmFormatModifierPropertiesEXT> {
        use crate::Device;

        let mut properties = VkImageDrmFormatModifierPropertiesEXT {
            sType: VkImageDrmFormatModifierPropertiesEXT::TYPE,
            pNext: std::ptr::null_mut(),
            drmFormatModifier: 0,
        };
        unsafe {
            crate::VkResultBox(self.device().get_image_drm_format_modifier_properties_ext_fn().0(
                self.device().native_ptr(),
                self.native_ptr(),
                &mut properties,
            ))
            .into_result()
            .map(move |_| properties)
        }
    }
}
DerefContainerBracketImpl!(for Image {
    fn format(&self) -> VkFormat {
        T::format(self)
    }

    fn size(&self) -> &VkExtent3D {
        T::size(self)
    }

    fn dimension(&self) -> VkImageViewType {
        T::dimension(self)
    }
});
GuardsImpl!(for Image {
    fn format(&self) -> VkFormat {
        T::format(&self)
    }

    fn size(&self) -> &VkExtent3D {
        T::size(&self)
    }

    fn dimension(&self) -> VkImageViewType {
        T::dimension(&self)
    }
});

pub trait ImageSubresourceSlice: Image {
    /// method chaining helper
    fn by_ref(&self) -> &Self {
        self
    }

    /// Creates subresource
    #[inline]
    fn subresource(self, aspect_mask: AspectMask, mip_level: u32, array_layer: u32) -> ImageSubresource<Self>
    where
        Self: Sized,
    {
        ImageSubresource(
            self,
            VkImageSubresource {
                aspectMask: aspect_mask.0,
                mipLevel: mip_level,
                arrayLayer: array_layer,
            },
        )
    }

    /// Creates subresource-range
    #[inline]
    fn subresource_range(
        self,
        aspect_mask: AspectMask,
        mip_level: Range<u32>,
        array_layers: Range<u32>,
    ) -> ImageSubresourceRange<Self>
    where
        Self: Sized,
    {
        ImageSubresourceRange(
            self,
            VkImageSubresourceRange {
                aspectMask: aspect_mask.0,
                baseMipLevel: mip_level.start,
                levelCount: mip_level.len() as _,
                baseArrayLayer: array_layers.start,
                layerCount: array_layers.len() as _,
            },
        )
    }
}
impl<T> ImageSubresourceSlice for T where T: Image {}

pub trait ImageChild {
    type ConcreteImage: crate::Image;

    fn image(&self) -> &Self::ConcreteImage;
}
DerefContainerBracketImpl!(for ImageChild {
    type ConcreteImage = T::ConcreteImage;

    fn image(&self) -> &Self::ConcreteImage { T::image(self) }
});
GuardsImpl!(for ImageChild {
    type ConcreteImage = T::ConcreteImage;

    fn image(&self) -> &Self::ConcreteImage { T::image(&self) }
});

pub trait ImageView: VkHandle<Handle = VkImageView> + DeviceChild {}
DerefContainerBracketImpl!(for ImageView {});
GuardsImpl!(for ImageView {});

/// Image Dimension by corresponding extent type
pub trait ImageSize {
    const DIMENSION: VkImageType;

    fn conv(self) -> VkExtent3D;
}
impl ImageSize for u32 {
    const DIMENSION: VkImageType = VK_IMAGE_TYPE_1D;

    fn conv(self) -> VkExtent3D {
        VkExtent3D {
            width: self,
            height: 1,
            depth: 1,
        }
    }
}
impl ImageSize for VkExtent2D {
    const DIMENSION: VkImageType = VK_IMAGE_TYPE_2D;

    fn conv(self) -> VkExtent3D {
        self.with_depth(1)
    }
}
impl ImageSize for VkExtent3D {
    const DIMENSION: VkImageType = VK_IMAGE_TYPE_3D;

    fn conv(self) -> VkExtent3D {
        self
    }
}

/// Opaque handle to a image object(constructed via `ImageDesc`)
#[derive(VkHandle, VkObject, DeviceChild)]
#[VkObject(type = VkImage::OBJECT_TYPE)]
pub struct ImageObject<Device: crate::Device>(VkImage, #[parent] Device, VkImageType, VkFormat, VkExtent3D);
unsafe impl<Device: crate::Device + Sync> Sync for ImageObject<Device> {}
unsafe impl<Device: crate::Device + Send> Send for ImageObject<Device> {}
#[implements]
impl<Device: crate::Device> Drop for ImageObject<Device> {
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
impl<Device: crate::Device> Image for ImageObject<Device> {
    fn format(&self) -> VkFormat {
        self.3
    }

    fn size(&self) -> &VkExtent3D {
        &self.4
    }

    fn dimension(&self) -> VkImageViewType {
        match self.2 {
            VK_IMAGE_TYPE_1D => VK_IMAGE_VIEW_TYPE_1D,
            VK_IMAGE_TYPE_2D => VK_IMAGE_VIEW_TYPE_2D,
            VK_IMAGE_TYPE_3D => VK_IMAGE_VIEW_TYPE_3D,
            _ => unreachable!(),
        }
    }
}
impl<Device: crate::Device> MemoryBound for ImageObject<Device>
where
    Self: VkHandle<Handle = VkImage>,
{
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    type MemoryRequirementsInfo2<'b> = ImageMemoryRequirementsInfo2<'b, Self> where Device: 'b;

    #[implements]
    fn requirements(&self) -> VkMemoryRequirements {
        let mut p = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::get_image_memory_requirements(
                self.device().native_ptr(),
                self.native_ptr(),
                p.as_mut_ptr(),
            );

            p.assume_init()
        }
    }

    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    #[implements]
    fn requirements2<'b>(&'b self) -> Self::MemoryRequirementsInfo2<'b> {
        ImageMemoryRequirementsInfo2::new(self)
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

/// Builder structure specifying the parameters of a newly created image object
#[derive(Clone, Debug)]
pub struct ImageDesc<'d>(
    VkImageCreateInfo,
    Vec<Box<GenericVulkanStructure>>,
    core::marker::PhantomData<Option<&'d dyn std::any::Any>>,
);
impl<'d> ImageDesc<'d> {
    pub fn new<Size: ImageSize>(size: Size, format: VkFormat, usage: ImageUsage, initial_layout: ImageLayout) -> Self {
        ImageDesc(
            VkImageCreateInfo {
                sType: VkImageCreateInfo::TYPE,
                pNext: std::ptr::null(),
                flags: 0,
                imageType: Size::DIMENSION,
                extent: size.conv(),
                format,
                usage: usage.0,
                mipLevels: 1,
                arrayLayers: 1,
                samples: 1,
                initialLayout: initial_layout as _,
                tiling: VK_IMAGE_TILING_OPTIMAL,
                sharingMode: VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: core::ptr::null(),
            },
            Vec::new(),
            core::marker::PhantomData,
        )
    }

    pub unsafe fn with_extension(mut self, ext: impl VulkanStructure) -> Self {
        self.1.push(core::mem::transmute(Box::new(ext)));
        self
    }

    /// Wraps raw vulkan structure
    /// # Safety
    /// This function does not check any references/constraints
    pub const unsafe fn from_raw(s: VkImageCreateInfo) -> Self {
        Self(s, Vec::new(), core::marker::PhantomData)
    }

    /// Unwraps raw vulkan structure
    /// # Safety
    /// Lifetime constraints are removed
    pub unsafe fn into_raw(self) -> VkImageCreateInfo {
        self.0
    }

    /// A list of queue families that will access this image,
    /// or an empty list if no queue families can access this image simultaneously
    pub fn sharing_queue_families(mut self, indices: &[u32]) -> Self {
        self.0.sharingMode = if indices.is_empty() {
            VK_SHARING_MODE_EXCLUSIVE
        } else {
            VK_SHARING_MODE_CONCURRENT
        };
        self.0.queueFamilyIndexCount = indices.len() as _;
        self.0.pQueueFamilyIndices = indices.as_ptr();

        self
    }

    /// The number of sub-data element samples in the image  
    /// bitmask of 1(default), 2, 4, 8, 16, 32, 64
    pub fn sample_counts(mut self, count_bits: u32) -> Self {
        self.0.samples = count_bits;
        self
    }

    /// Sets the tiling arrangement of the data elements in memory as "linear tiling"  
    /// default: optimal tiling
    pub fn use_linear_tiling(mut self) -> Self {
        self.0.tiling = VK_IMAGE_TILING_LINEAR;
        self
    }

    /// A bitmask of `ImageFlags`describing additional parameters of the image  
    /// default: none
    pub fn flags(mut self, opt: ImageFlags) -> Self {
        self.0.flags = opt.0;
        self
    }

    /// The number of layers in the image  
    /// default: 1
    pub fn array_layers(mut self, layers: u32) -> Self {
        self.0.arrayLayers = layers;
        self
    }

    /// The number of levels of detail available for minified sampling of the image  
    /// default: 1
    pub fn mip_levels(mut self, levels: u32) -> Self {
        self.0.mipLevels = levels;
        self
    }

    #[cfg(feature = "VK_KHR_external_memory")]
    pub fn exportable_as(self, types: crate::ExternalMemoryHandleTypes) -> Self {
        unsafe {
            self.with_extension(VkExternalMemoryImageCreateInfoKHR {
                sType: VkExternalMemoryImageCreateInfoKHR::TYPE,
                pNext: core::ptr::null(),
                handleTypes: types.into(),
            })
        }
    }

    /// Create an image
    #[implements]
    pub fn create<Device: crate::Device>(mut self, device: Device) -> crate::Result<ImageObject<Device>> {
        crate::ext::chain(&mut self.0, self.1.iter_mut().map(AsMut::as_mut));

        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_image(device.native_ptr(), &self.0, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| {
                    ImageObject(
                        h.assume_init(),
                        device,
                        self.0.imageType,
                        self.0.format,
                        self.0.extent.clone(),
                    )
                })
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ImageSubresource<S: Image>(S, VkImageSubresource);
impl<S: Image> ImageSubresource<S> {
    /// Retrieve information about an image subresource
    #[implements]
    pub fn layout_info(&self) -> VkSubresourceLayout {
        let mut s = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::get_image_subresource_layout(
                self.0.device().native_ptr(),
                self.0.native_ptr(),
                &self.1,
                s.as_mut_ptr(),
            );

            s.assume_init()
        }
    }

    pub fn make_ref(&self) -> ImageSubresource<&S> {
        ImageSubresource(&self.0, self.1.clone())
    }
}
impl<S: Image> From<ImageSubresource<S>> for VkImageSubresource {
    fn from(value: ImageSubresource<S>) -> Self {
        value.1
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImageSubresourceRange<S: Image>(S, VkImageSubresourceRange);
impl<S: Image> ImageSubresourceRange<S> {
    /// Builds ImageView information
    pub fn view_builder(self) -> ImageViewBuilder<S> {
        ImageViewBuilder::new(self.0, self.1)
    }

    /// Retrieves single subresource in this range
    pub fn subresource(self, mip_level_offset: u32, array_layer_offset: u32) -> ImageSubresource<S> {
        ImageSubresource(
            self.0,
            VkImageSubresource {
                aspectMask: self.1.aspectMask,
                mipLevel: self.1.baseMipLevel + mip_level_offset,
                arrayLayer: self.1.baseArrayLayer + array_layer_offset,
            },
        )
    }

    pub fn make_ref(&self) -> ImageSubresourceRange<&S> {
        ImageSubresourceRange(&self.0, self.1.clone())
    }
}
impl<S: Image> ImageSubresourceRange<&'_ S> {
    pub fn memory_barrier(self, from_layout: ImageLayout, to_layout: ImageLayout) -> ImageMemoryBarrier {
        ImageMemoryBarrier::new(self.0, self.1, from_layout, to_layout)
    }
}
impl<S: Image> From<ImageSubresourceRange<S>> for VkImageSubresourceRange {
    fn from(value: ImageSubresourceRange<S>) -> Self {
        value.1
    }
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct ImageMemoryRequirementsInfo2<'b, Image: self::Image + 'b>(VkImageMemoryRequirementsInfo2KHR, &'b Image);
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'b, Image: self::Image + 'b> ImageMemoryRequirementsInfo2<'b, Image> {
    pub fn new(image: &'b Image) -> Self {
        Self(
            VkImageMemoryRequirementsInfo2KHR {
                sType: VkImageMemoryRequirementsInfo2KHR::TYPE,
                pNext: core::ptr::null(),
                image: image.native_ptr(),
            },
            image,
        )
    }

    #[implements]
    pub fn query(self, sink: &mut core::mem::MaybeUninit<VkMemoryRequirements2KHR>)
    where
        <Image as crate::DeviceChild>::ConcreteDevice: crate::Device,
    {
        use crate::Device;

        unsafe {
            self.1.device().get_image_memory_requirements_2_khr_fn().0(
                self.1.device().native_ptr(),
                &self.0,
                sink.as_mut_ptr(),
            );
        }
    }
}

/// Layouts of image and image subresources
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum ImageLayout {
    /// does not support device access
    Undefined = VK_IMAGE_LAYOUT_UNDEFINED as _,
    /// does not support device access. host can be written to this memory immediately
    Preinitialized = VK_IMAGE_LAYOUT_PREINITIALIZED as _,
    /// supports all types of device access
    General = VK_IMAGE_LAYOUT_GENERAL as _,
    /// must only be used as a color or resolve attachment in a `Framebuffer`
    ColorAttachmentOpt = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL as _,
    /// must only be used as a depth/stencil attachment in a `Framebuffer`
    DepthStencilAttachmentOpt = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL as _,
    /// must only be used as a read-only depth/stencil attachment in a `Framebuffer`
    /// and/or as a read-only image in a shader (which can be read as a sampled image,
    /// combined image/sampler and/or input attachment).
    DepthStencilReadOnlyOpt = VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL as _,
    /// must only be used as a read-only image in a shader (which can be read as a sampled image,
    /// combined image/sampler and/or input attachment).
    ShaderReadOnlyOpt = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL as _,
    /// must only be used as a source image of a transfer command
    TransferSrcOpt = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL as _,
    /// must only be used as a destination image of a transfer command
    TransferDestOpt = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL as _,
    /// must only be used for presenting a swapchain image for display
    #[cfg(feature = "VK_KHR_swapchain")]
    PresentSrc = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR as _,
}
impl ImageLayout {
    /// Commonly used access types with the layout
    pub fn default_access_mask(self) -> VkAccessFlags {
        match self {
            Self::Undefined | Self::Preinitialized => 0,
            Self::General => VK_ACCESS_MEMORY_READ_BIT,
            Self::ColorAttachmentOpt => VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            Self::DepthStencilAttachmentOpt => VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
            Self::DepthStencilReadOnlyOpt => VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT,
            Self::ShaderReadOnlyOpt => VK_ACCESS_SHADER_READ_BIT,
            Self::TransferSrcOpt => VK_ACCESS_TRANSFER_READ_BIT,
            Self::TransferDestOpt => VK_ACCESS_TRANSFER_WRITE_BIT,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::PresentSrc => VK_ACCESS_MEMORY_READ_BIT,
        }
    }
}

/// Bitmask specifying intended usage of an image
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ImageUsage(pub VkImageUsageFlags);
impl ImageUsage {
    /// The image can be used as the source of a transfer command
    pub const TRANSFER_SRC: Self = Self(VK_IMAGE_USAGE_TRANSFER_SRC_BIT);
    /// The image can be used as the destination of a transfer command
    pub const TRANSFER_DEST: Self = Self(VK_IMAGE_USAGE_TRANSFER_DST_BIT);
    /// The image can be used to create `ImageView` suitable for occupying a `DescriptorSet` slot
    /// either of type `DescriptorType::SampledImage` or `DescriptorType::CombinedImageSampler`, and be sampled by a shader
    pub const SAMPLED: Self = Self(VK_IMAGE_USAGE_SAMPLED_BIT);
    /// The image can be used to create a `ImageView` suitable for occupying a `DescriptorSet` slot of type `DescriptorType::StorageImage`
    pub const STORAGE: Self = Self(VK_IMAGE_USAGE_STORAGE_BIT);
    /// The image can be used to create a `ImageView` suitable for use as a color or resolve attachment in a `Framebuffer`
    pub const COLOR_ATTACHMENT: Self = Self(VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT);
    /// The image can be used to create a `ImageView` suitable for use as a depth/stencil attachment in a `Framebuffer`
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT);
    /// The memory bound to this image will have been allocated with the `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`
    /// This bit can be set for any image that can be used to create a `ImageView` suitable for use as a color, resolve, depth/stencil,
    /// or input attachment
    pub const TRANSIENT_ATTACHMENT: Self = Self(VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT);
    /// The image can be used to create a `ImageView` suitable for occupying `DescriptorSet` slot of type `DescriptorType::InputAttachment`;
    /// be read from a shader as an input attachment; and be used as an input attachment in a framebuffer
    pub const INPUT_ATTACHMENT: Self = Self(VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT);

    /// The image can be used as the source of a transfer command
    pub const fn transfer_src(self) -> Self {
        Self(self.0 | Self::TRANSFER_SRC.0)
    }
    /// The image can be used as the destination of a transfer command
    pub const fn transfer_dest(self) -> Self {
        Self(self.0 | Self::TRANSFER_DEST.0)
    }
    /// The image can be used to create `ImageView` suitable for occupying a `DescriptorSet` slot
    /// either of type `DescriptorType::SampledImage` or `DescriptorType::CombinedImageSampler`, and be sampled by a shader
    pub const fn sampled(self) -> Self {
        Self(self.0 | Self::SAMPLED.0)
    }
    /// The image can be used to create a `ImageView` suitable for occupying a `DescriptorSet` slot of type `DescriptorType::StorageImage`
    pub const fn storage(self) -> Self {
        Self(self.0 | Self::STORAGE.0)
    }
    /// The image can be used to create a `ImageView` suitable for use as a color or resolve attachment in a `Framebuffer`
    pub const fn color_attachment(self) -> Self {
        Self(self.0 | Self::COLOR_ATTACHMENT.0)
    }
    /// The image can be used to create a `ImageView` suitable for use as a depth/stencil attachment in a `Framebuffer`
    pub const fn depth_stencil_attachment(self) -> Self {
        Self(self.0 | Self::DEPTH_STENCIL_ATTACHMENT.0)
    }
    /// The memory bound to this image will have been allocated with the `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`
    /// This bit can be set for any image that can be used to create a `ImageView` suitable for use as a color, resolve, depth/stencil,
    /// or input attachment
    pub const fn transient_attachment(self) -> Self {
        Self(self.0 | Self::TRANSIENT_ATTACHMENT.0)
    }
    /// The image can be used to create a `ImageView` suitable for occupying `DescriptorSet` slot of type `DescriptorType::InputAttachment`;
    /// be read from a shader as an input attachment; and be used as an input attachment in a framebuffer
    pub const fn input_attachment(self) -> Self {
        Self(self.0 | Self::INPUT_ATTACHMENT.0)
    }

    /// merge two flags (const alias of BitOr)
    pub const fn merge(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl BitOr for ImageUsage {
    type Output = ImageUsage;
    fn bitor(self, other: Self) -> Self {
        ImageUsage(self.0 | other.0)
    }
}
impl BitOrAssign for ImageUsage {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}
impl From<ImageUsage> for VkImageUsageFlags {
    fn from(value: ImageUsage) -> Self {
        value.0
    }
}

/// Bitmask specifying additional parameters of an image
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ImageFlags(pub VkImageCreateFlags);
impl ImageFlags {
    /// Empty bits
    pub const EMPTY: Self = Self(0);
    /// The image will be backed using sparse memory binding
    pub const SPARSE_BINDING: Self = Self(VK_IMAGE_CREATE_SPARSE_BINDING_BIT);
    /// The image can be partially backed using sparse memory binding. This bit is with `SPARSE_BINDING` implicitly
    pub const SPARSE_RESIDENCY: Self = Self(VK_IMAGE_CREATE_SPARSE_BINDING_BIT | VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT);
    /// The image will be backed using sparse memory binding with memory ranges
    /// that might also simultaneously be backing another image. This bit is with `SPARSE_BINDING` implicitly
    pub const SPARSE_ALIASED: Self = Self(VK_IMAGE_CREATE_SPARSE_BINDING_BIT | VK_IMAGE_CREATE_SPARSE_ALIASED_BIT);
    /// The image can be used to create a `ImageView` with a different format from the image
    pub const MUTABLE_FORMAT: Self = Self(VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT);
    /// The image can be used to create a `ImageView` of type `ImageViewType::Cube` or `ImageViewType::CubeArray`
    pub const CUBE_COMPATIBLE: Self = Self(VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT);

    /// The image will be backed using sparse memory binding
    pub const fn sparse_binding(self) -> Self {
        Self(self.0 | Self::SPARSE_BINDING.0)
    }
    /// The image can be partially backed using sparse memory binding. This bit is with `SPARSE_BINDING` implicitly
    pub const fn sparse_residency(self) -> Self {
        Self(self.0 | Self::SPARSE_RESIDENCY.0)
    }
    /// The image will be backed using sparse memory binding with memory ranges
    /// that might also simultaneously be backing another image. This bit is with `SPARSE_BINDING` implicitly
    pub const fn sparse_aliased(self) -> Self {
        Self(self.0 | Self::SPARSE_ALIASED.0)
    }
    /// The image can be used to create a `ImageView` with a different format from the image
    pub const fn mutable_format(self) -> Self {
        Self(self.0 | Self::MUTABLE_FORMAT.0)
    }
    /// The image can be used to create a `ImageView` of type `ImageViewType::Cube` or `ImageViewType::CubeArray`
    pub const fn cube_compatible(self) -> Self {
        Self(self.0 | Self::CUBE_COMPATIBLE.0)
    }

    /// merge two flags (const alias of BitOr)
    pub const fn merge(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl BitOr for ImageFlags {
    type Output = ImageFlags;
    fn bitor(self, other: Self) -> Self {
        ImageFlags(self.0 | other.0)
    }
}
impl BitOrAssign for ImageFlags {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}
impl From<ImageFlags> for VkImageCreateFlags {
    fn from(value: ImageFlags) -> Self {
        value.0
    }
}

/// Bitmask specifying which aspects of an image are included in a view
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[repr(transparent)]
pub struct AspectMask(pub VkImageAspectFlags);
impl AspectMask {
    /// The color aspect
    pub const COLOR: Self = Self(VK_IMAGE_ASPECT_COLOR_BIT);
    /// The depth aspect
    pub const DEPTH: Self = Self(VK_IMAGE_ASPECT_DEPTH_BIT);
    /// The stencil aspect
    pub const STENCIL: Self = Self(VK_IMAGE_ASPECT_STENCIL_BIT);
    /// The metadata aspect, used for sparse sparse resource operations
    pub const METADATA: Self = Self(VK_IMAGE_ASPECT_METADATA_BIT);

    /// The color aspect
    pub const fn color(self) -> Self {
        Self(self.0 | Self::COLOR.0)
    }
    /// The depth aspect
    pub const fn depth(self) -> Self {
        Self(self.0 | Self::DEPTH.0)
    }
    /// The stencil aspect
    pub const fn stencil(self) -> Self {
        Self(self.0 | Self::STENCIL.0)
    }
    /// The metadata aspect, used for sparse sparse resource oeprations
    pub const fn metadata(self) -> Self {
        Self(self.0 | Self::METADATA.0)
    }

    /// merge two flags (const alias of BitOr)
    pub const fn merge(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl BitOr for AspectMask {
    type Output = AspectMask;
    fn bitor(self, other: Self) -> Self {
        AspectMask(self.0 | other.0)
    }
}
impl BitOrAssign for AspectMask {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}
impl From<AspectMask> for VkImageAspectFlags {
    fn from(value: AspectMask) -> Self {
        value.0
    }
}

/// Opaque handle to a image view object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkImageView::OBJECT_TYPE)]
pub struct ImageViewObject<Image: self::Image>(VkImageView, Image);
unsafe impl<Image: self::Image + Sync> Sync for ImageViewObject<Image> {}
unsafe impl<Image: self::Image + Send> Send for ImageViewObject<Image> {}
impl<Image: self::Image> DeviceChild for ImageViewObject<Image> {
    type ConcreteDevice = Image::ConcreteDevice;

    fn device(&self) -> &Self::ConcreteDevice {
        self.1.device()
    }
}
#[implements]
impl<Image: self::Image> Drop for ImageViewObject<Image> {
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.device().native_ptr(), core::ptr::null());
        }
    }
}
impl<Image: self::Image> ImageView for ImageViewObject<Image> {}
impl<Image: self::Image> Deref for ImageViewObject<Image> {
    type Target = Image;

    fn deref(&self) -> &Image {
        &self.1
    }
}
impl<Image: self::Image> DerefMut for ImageViewObject<Image> {
    fn deref_mut(&mut self) -> &mut Image {
        &mut self.1
    }
}
impl<Image: self::Image> ImageChild for ImageViewObject<Image> {
    type ConcreteImage = Image;

    fn image(&self) -> &Image {
        &self.1
    }
}

pub struct ImageViewBuilder<I: Image>(VkImageViewCreateInfo, I);
impl<I: Image> ImageViewBuilder<I> {
    pub fn new(source: I, subresource_range: VkImageSubresourceRange) -> Self {
        Self(
            VkImageViewCreateInfo {
                sType: VkImageViewCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                image: source.native_ptr(),
                viewType: source.dimension(),
                format: source.format(),
                components: VkComponentMapping::default(),
                subresourceRange: subresource_range,
            },
            source,
        )
    }

    pub fn with_format_mutation(mut self, format: VkFormat) -> Self {
        self.0.format = format;
        self
    }

    pub fn with_mapping(mut self, mapping: impl Into<VkComponentMapping>) -> Self {
        self.0.components = mapping.into();
        self
    }

    pub fn with_dimension(mut self, dimension: VkImageViewType) -> Self {
        self.0.viewType = dimension;
        self
    }

    #[implements]
    pub fn create(mut self) -> crate::Result<ImageViewObject<I>> {
        self.0.image = self.1.native_ptr();

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_image_view(self.1.device().native_ptr(), &self.0, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| ImageViewObject(h.assume_init(), self.1))
        }
    }
}
