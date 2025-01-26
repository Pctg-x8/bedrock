use crate::{
    ffi_helper::slice_as_ptr_empty_null, vk::*, DeviceChild, DeviceChildHandle, ImageMemoryBarrier, MemoryBound,
    VkDeviceChildNonExtDestroyable, VkHandle, VkObject, VkRawHandle, VulkanStructure, VulkanStructureAsRef,
};
#[implements]
use crate::{DeviceMemory, VkHandleMut};
use derives::{bitflags_newtype, implements};

pub trait Image: VkHandle<Handle = VkImage> + DeviceChildHandle {
    /// The pixel format of an image
    fn format(&self) -> VkFormat;

    /// The size of an image
    fn size(&self) -> &VkExtent3D;

    fn dimension(&self) -> VkImageViewType;

    /// Query a count of the memory requirements for a sparse image
    #[implements]
    #[inline]
    fn sparse_requirement_count(&self) -> u32 {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_image_sparse_memory_requirements(
                self.device_handle(),
                self.native_ptr(),
                &mut n,
                core::ptr::null_mut(),
            );
        }

        n
    }

    /// Query the memory requirements for a sparse image
    #[implements]
    #[inline]
    fn sparse_requirements(&self, sink: &mut [VkSparseImageMemoryRequirements]) -> u32 {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_image_sparse_memory_requirements(
                self.device_handle(),
                self.native_ptr(),
                &mut n,
                sink.as_mut_ptr(),
            );
        }

        n
    }

    /// Query the memory requirements for a sparse image
    #[implements("alloc")]
    fn sparse_requirements_alloc(&self) -> Vec<VkSparseImageMemoryRequirements> {
        let n = self.sparse_requirement_count();
        if n == 0 {
            // no items
            return crate::alloc::empty_sink_buffer();
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        self.sparse_requirements(&mut xs);

        xs
    }

    /// Returns an image's DRM format modifier
    #[implements("VK_EXT_image_drm_format_modifier")]
    #[inline]
    unsafe fn drm_format_modifier_properties_raw(
        &self,
        sink: &mut core::mem::MaybeUninit<VkImageDrmFormatModifierPropertiesEXT>,
    ) -> crate::Result<()>
    where
        Self: DeviceChild,
    {
        use crate::Device;

        self.device().get_image_drm_format_modifier_properties_ext_fn().0(
            self.device_handle(),
            self.native_ptr(),
            sink.as_mut_ptr(),
        )
        .into_result()
        .map(drop)
    }

    /// Returns an image's DRM format modifier
    #[implements("VK_EXT_image_drm_format_modifier")]
    fn drm_format_modifier_properties(&self) -> crate::Result<VkImageDrmFormatModifierPropertiesEXT>
    where
        Self: DeviceChild,
    {
        let mut properties = core::mem::MaybeUninit::<VkImageDrmFormatModifierPropertiesEXT>::uninit();
        unsafe {
            let p = properties.as_mut_ptr();
            core::ptr::addr_of_mut!((*p).sType).write(VkImageDrmFormatModifierPropertiesEXT::TYPE);
            core::ptr::addr_of_mut!((*p).pNext).write(core::ptr::null_mut());

            self.drm_format_modifier_properties_raw(&mut properties)?;

            Ok(properties.assume_init())
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

pub trait DeviceChildImage: Image + DeviceChild {}
impl<T: Image + DeviceChild> DeviceChildImage for T {}

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
        mip_level: core::ops::Range<u32>,
        array_layers: core::ops::Range<u32>,
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

pub trait ImageView: VkHandle<Handle = VkImageView> {}
DerefContainerBracketImpl!(for ImageView {});
GuardsImpl!(for ImageView {});

pub trait ConcreteDeviceImageView: ImageView + DeviceChild {}
DerefContainerBracketImpl!(for ConcreteDeviceImageView {});
GuardsImpl!(for ConcreteDeviceImageView {});

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
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkImage::OBJECT_TYPE)]
pub struct ImageObject<Device: VkHandle<Handle = VkDevice>>(VkImage, Device, VkImageType, VkFormat, VkExtent3D);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for ImageObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for ImageObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for ImageObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for ImageObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for ImageObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: VkHandle<Handle = VkDevice>> Image for ImageObject<Device> {
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
impl<Device: VkHandle<Handle = VkDevice>> MemoryBound for ImageObject<Device> {
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    type MemoryRequirementsInfo2<'b>
        = ImageMemoryRequirementsInfo2<'b, Self>
    where
        Device: 'b;

    #[implements]
    fn requirements(&self) -> VkMemoryRequirements {
        let mut p = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_image_memory_requirements(self.1.native_ptr(), self.0, p.as_mut_ptr());

            p.assume_init()
        }
    }

    #[implements("VK_KHR_get_memory_requirements2")]
    fn requirements2<'b>(&'b self) -> Self::MemoryRequirementsInfo2<'b> {
        ImageMemoryRequirementsInfo2::new(self)
    }

    #[implements]
    fn bind(&mut self, memory: &(impl DeviceMemory + ?Sized), offset: usize) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkfn::bind_image_memory(self.1.native_ptr(), self.0, memory.native_ptr(), offset as _)
                .into_result()
                .map(drop)
        }
    }
}
impl<Device: VkHandle<Handle = VkDevice>> ImageObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the parent
    pub const unsafe fn manage(
        handle: VkImage,
        parent: Device,
        image_type: VkImageType,
        format: VkFormat,
        extent: VkExtent3D,
    ) -> Self {
        Self(handle, parent, image_type, format, extent)
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkImage, Device, VkImageType, VkFormat, VkExtent3D) {
        let v = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        let (t, f, x) = (self.2, self.3, self.4);
        core::mem::forget(self);

        (v, p, t, f, x)
    }
}
impl<Device: VkHandle<Handle = VkDevice> + Clone> ImageObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> ImageObject<Device> {
        let r = ImageObject(self.0, self.1.clone(), self.2, self.3, self.4);
        core::mem::forget(self);

        r
    }
}
impl<Device: crate::Device> ImageObject<Device> {
    /// Create a new image object
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_COMPRESSION_EXHAUSTED_EXT`]
    /// * [`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
    #[implements]
    #[inline]
    pub fn new(device: Device, info: &ImageCreateInfo) -> crate::Result<Self> {
        Ok(unsafe {
            Self::manage(
                device.new_image_raw(info, None)?,
                device,
                info.0.imageType,
                info.0.format,
                info.0.extent,
            )
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageCreateInfo<'d>(
    VkImageCreateInfo,
    core::marker::PhantomData<(Option<&'d dyn VulkanStructureAsRef>, Option<&'d [u32]>)>,
);
impl<'d> ImageCreateInfo<'d> {
    #[inline(always)]
    pub fn new<Size: ImageSize>(size: Size, format: VkFormat) -> Self {
        Self(
            VkImageCreateInfo {
                sType: VkImageCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                imageType: Size::DIMENSION,
                extent: size.conv(),
                format,
                usage: 0,
                mipLevels: 1,
                arrayLayers: 1,
                samples: 1,
                initialLayout: VK_IMAGE_LAYOUT_UNDEFINED,
                tiling: VK_IMAGE_TILING_OPTIMAL,
                sharingMode: VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkImageCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkImageCreateInfo {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }

    /// Sets an size and a dimension of the created image.
    #[inline(always)]
    pub fn size<Size: ImageSize>(mut self, size: Size) -> Self {
        self.0.extent = size.conv();
        self.0.imageType = Size::DIMENSION;

        self
    }

    /// Sets an initial layout for the created image.
    /// default: Undefined layout
    pub const fn init_layout(mut self, layout: ImageLayout) -> Self {
        self.0.initialLayout = layout as _;
        self
    }

    /// A list of queue families that will access this image,
    /// or an empty list if no queue families can access this image simultaneously
    pub const fn sharing_queue_families(mut self, indices: &'d [u32]) -> Self {
        self.0.sharingMode = if indices.is_empty() {
            VK_SHARING_MODE_EXCLUSIVE
        } else {
            VK_SHARING_MODE_CONCURRENT
        };
        self.0.queueFamilyIndexCount = indices.len() as _;
        self.0.pQueueFamilyIndices = slice_as_ptr_empty_null(indices);

        self
    }

    /// The number of sub-data element samples in the image
    /// bitmask of 1(default), 2, 4, 8, 16, 32, 64
    pub const fn sample_counts(mut self, count_bits: u32) -> Self {
        self.0.samples = count_bits;
        self
    }

    /// Sets the tiling arrangement of the data elements in memory as "linear tiling"
    /// default: optimal tiling
    pub const fn use_linear_tiling(mut self) -> Self {
        self.0.tiling = VK_IMAGE_TILING_LINEAR;
        self
    }

    /// A bitmask of `ImageFlags`describing additional parameters of the image
    /// default: none
    pub const fn flags(mut self, opt: ImageFlags) -> Self {
        self.0.flags = opt.0;
        self
    }

    /// The number of layers in the image
    /// default: 1
    pub const fn array_layers(mut self, layers: u32) -> Self {
        self.0.arrayLayers = layers;
        self
    }

    /// The number of levels of detail available for minified sampling of the image
    /// default: 1
    pub const fn mip_levels(mut self, levels: u32) -> Self {
        self.0.mipLevels = levels;
        self
    }

    /// Sets the created image will be sampled.
    pub const fn sampled(self) -> Self {
        self.usage_with(ImageUsageFlags::SAMPLED)
    }

    /// Sets the created resource will be the destination of transferring operation.
    pub const fn transfer_dest(self) -> Self {
        self.usage_with(ImageUsageFlags::TRANSFER_DEST)
    }

    /// Sets the created image can be used as a Storage Image.
    pub const fn use_as_storage(self) -> Self {
        self.usage_with(ImageUsageFlags::STORAGE)
    }

    /// Sets the created image can be used as a color attachment.
    pub const fn as_color_attachment(self) -> Self {
        self.usage_with(ImageUsageFlags::COLOR_ATTACHMENT)
    }

    /// Sets the created image can be used as an input attachment.
    pub const fn as_input_attachment(self) -> Self {
        self.usage_with(ImageUsageFlags::INPUT_ATTACHMENT)
    }

    /// Sets the created image can be used as a depth stencil attachment.
    pub const fn as_depth_stencil_attachment(self) -> Self {
        self.usage_with(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
    }

    /// Sets the created image as transient-used attachment.
    pub const fn as_transient_attachment(self) -> Self {
        self.usage_with(ImageUsageFlags::TRANSIENT_ATTACHMENT)
    }

    /// Merges some custom usage flag bits.
    pub const fn usage_with(mut self, bits: ImageUsageFlags) -> Self {
        self.0.usage |= bits.0;
        self
    }

    /// Overwrites all of custom usage flag bits.
    pub const fn set_usage(mut self, bits: ImageUsageFlags) -> Self {
        self.0.usage = bits.0;
        self
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
            crate::vkfn::get_image_subresource_layout(
                self.0.device_handle(),
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
impl<'r, S: Image> ImageSubresourceRange<&'r S> {
    #[inline]
    pub fn memory_barrier(self, trans: LayoutTransition) -> ImageMemoryBarrier {
        ImageMemoryBarrier::new(self.0, self.1, trans)
    }

    #[cfg(feature = "VK_KHR_synchronization2")]
    #[inline]
    pub fn memory_barrier2(self) -> crate::ImageMemoryBarrier2<'r> {
        crate::ImageMemoryBarrier2::new(self.0, self.1)
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
        Image: crate::DeviceChild,
    {
        #[cfg(feature = "Allow1_1APIs")]
        unsafe {
            crate::vkfn::get_image_memory_requirements2(self.1.device().native_ptr(), &self.0, sink.as_mut_ptr());
        }

        #[cfg(not(feature = "Allow1_1APIs"))]
        unsafe {
            use crate::Device;

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

    /// Constructs the transition between image layouts.
    #[inline(always)]
    pub const fn to(self, after: Self) -> LayoutTransition {
        LayoutTransition { from: self, to: after }
    }
    /// Constructs the transition between image layouts (in reverse order).
    #[inline(always)]
    pub const fn from(self, before: Self) -> LayoutTransition {
        LayoutTransition { from: before, to: self }
    }
    /// Constructs the transition from undefined layout to this layout (convenient function).
    #[inline(always)]
    pub const fn from_undefined(self) -> LayoutTransition {
        self.from(Self::Undefined)
    }

    /// Constructs the empty transition.
    #[inline(always)]
    pub const fn keep(self) -> LayoutTransition {
        LayoutTransition { from: self, to: self }
    }
}

/// Represents the transition between image layouts.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct LayoutTransition {
    pub from: ImageLayout,
    pub to: ImageLayout,
}

/// Bitmask specifying intended usage of an image.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[bitflags_newtype]
pub struct ImageUsageFlags(VkImageUsageFlags);
impl ImageUsageFlags {
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
}

/// Bitmask specifying additional parameters of an image
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[bitflags_newtype]
pub struct ImageFlags(VkImageCreateFlags);
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
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageSparseMemoryRequirementsInfo2<'r>(
    pub(crate) VkImageSparseMemoryRequirementsInfo2KHR,
    core::marker::PhantomData<(Option<&'r dyn VulkanStructureAsRef>, &'r dyn VkHandle<Handle = VkImage>)>,
);
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'r> ImageSparseMemoryRequirementsInfo2<'r> {
    #[inline]
    pub fn new(image: &'r (impl VkHandle<Handle = VkImage> + ?Sized)) -> Self {
        Self(
            VkImageSparseMemoryRequirementsInfo2KHR {
                sType: VkImageSparseMemoryRequirementsInfo2KHR::TYPE,
                pNext: core::ptr::null(),
                image: image.native_ptr(),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkImageSparseMemoryRequirementsInfo2KHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkImageSparseMemoryRequirementsInfo2KHR {
        self.0
    }

    #[inline]
    pub fn with_next(mut self, next: &'r (impl VulkanStructureAsRef + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}

/// Bitmask specifying which aspects of an image are included in a view
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[bitflags_newtype]
pub struct AspectMask(VkImageAspectFlags);
impl AspectMask {
    /// No aspect mask
    pub const EMPTY: Self = Self(0);
    /// The color aspect
    pub const COLOR: Self = Self(VK_IMAGE_ASPECT_COLOR_BIT);
    /// The depth aspect
    pub const DEPTH: Self = Self(VK_IMAGE_ASPECT_DEPTH_BIT);
    /// The stencil aspect
    pub const STENCIL: Self = Self(VK_IMAGE_ASPECT_STENCIL_BIT);
    /// The metadata aspect, used for sparse sparse resource operations
    pub const METADATA: Self = Self(VK_IMAGE_ASPECT_METADATA_BIT);
}

/// Opaque handle to a image view object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkImageView::OBJECT_TYPE)]
pub struct ImageViewObject<Image: DeviceChildHandle>(VkImageView, Image);
#[implements]
impl<Image: DeviceChildHandle> Drop for ImageViewObject<Image> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.device_handle(), core::ptr::null());
        }
    }
}
unsafe impl<Image: DeviceChildHandle + Sync> Sync for ImageViewObject<Image> {}
unsafe impl<Image: DeviceChildHandle + Send> Send for ImageViewObject<Image> {}
impl<Image: DeviceChildHandle> DeviceChildHandle for ImageViewObject<Image> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.device_handle()
    }
}
impl<Image: DeviceChild> DeviceChild for ImageViewObject<Image> {
    type ConcreteDevice = Image::ConcreteDevice;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        self.1.device()
    }
}
impl<Image: self::Image> ImageChild for ImageViewObject<Image> {
    type ConcreteImage = Image;

    #[inline(always)]
    fn image(&self) -> &Image {
        &self.1
    }
}
impl<Image: DeviceChildHandle> ImageView for ImageViewObject<Image> {}
impl<Image: DeviceChildHandle> ImageViewObject<Image> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the parent
    pub const unsafe fn manage(handle: VkImageView, parent: Image) -> Self {
        Self(handle, parent)
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkImageView, Image) {
        let v = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (v, p)
    }
}
impl<Image: DeviceChildHandle + Clone> ImageViewObject<&'_ Image> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> ImageViewObject<Image> {
        let r = ImageViewObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}
impl<Image: DeviceChild> ImageViewObject<Image> {
    /// Create a new image view from an existing image
    /// # Failure
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
    #[implements]
    #[inline]
    pub fn new(image: Image, info: &ImageViewCreateInfo) -> crate::Result<Self> {
        use crate::Device;

        Ok(unsafe { Self::manage(image.device().new_image_view_raw(info, None)?, image) })
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageViewCreateInfo<'r>(
    VkImageViewCreateInfo,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = VkImage>>,
);
impl<'r> ImageViewCreateInfo<'r> {
    pub fn new(
        source: &'r (impl VkHandle<Handle = VkImage> + ?Sized),
        subresource_range: VkImageSubresourceRange,
        view_type: VkImageViewType,
        format: VkFormat,
    ) -> Self {
        Self(
            VkImageViewCreateInfo {
                sType: VkImageViewCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                image: source.native_ptr(),
                viewType: view_type,
                format,
                components: VkComponentMapping::default(),
                subresourceRange: subresource_range,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkImageViewCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkImageViewCreateInfo {
        self.0
    }

    pub const fn with_format_mutation(mut self, format: VkFormat) -> Self {
        self.0.format = format;
        self
    }

    pub const fn with_mapping(mut self, mapping: VkComponentMapping) -> Self {
        self.0.components = mapping;
        self
    }

    pub const fn with_dimension(mut self, dimension: VkImageViewType) -> Self {
        self.0.viewType = dimension;
        self
    }
}

pub struct ImageViewBuilder<I: Image>(ImageViewCreateInfo<'static>, I);
impl<I: Image> ImageViewBuilder<I> {
    pub fn new(source: I, subresource_range: VkImageSubresourceRange) -> Self {
        Self(
            unsafe {
                ImageViewCreateInfo::from_raw(VkImageViewCreateInfo {
                    sType: VkImageViewCreateInfo::TYPE,
                    pNext: core::ptr::null(),
                    flags: 0,
                    image: source.native_ptr(),
                    viewType: source.dimension(),
                    format: source.format(),
                    components: VkComponentMapping::default(),
                    subresourceRange: subresource_range,
                })
            },
            source,
        )
    }

    pub const fn with_format_mutation(mut self, format: VkFormat) -> Self {
        self.0 = self.0.with_format_mutation(format);
        self
    }

    pub const fn with_mapping(mut self, mapping: VkComponentMapping) -> Self {
        self.0 = self.0.with_mapping(mapping);
        self
    }

    pub const fn with_dimension(mut self, dimension: VkImageViewType) -> Self {
        self.0 = self.0.with_dimension(dimension);
        self
    }

    /// Create a new image view from an existing image
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
    #[implements]
    pub fn create(self) -> crate::Result<ImageViewObject<I>>
    where
        I: DeviceChild,
    {
        ImageViewObject::new(self.1, &self.0)
    }
}
