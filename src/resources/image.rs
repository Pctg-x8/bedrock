use bedrock_vk::{self as brvk, TypedVulkanStructure, VkRawHandle};

use crate::ffi_helper::slice_as_ptr_empty_null;
use crate::*;
use derives::{bitflags_newtype, implements};

pub trait Image: VkHandle<Handle = brvk::VkImage> + DeviceChildHandle {
    /// The pixel format of an image
    fn format(&self) -> brvk::VkFormat;

    /// The size of an image
    fn size(&self) -> &brvk::VkExtent3D;

    fn dimension(&self) -> brvk::VkImageViewType;

    /// Query a count of the memory requirements for a sparse image
    #[implements]
    #[inline]
    fn sparse_requirement_count(&self) -> u32 {
        let mut n = 0;
        unsafe {
            brvk::fns::get_image_sparse_memory_requirements(
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
    fn sparse_requirements(&self, sink: &mut [core::mem::MaybeUninit<brvk::VkSparseImageMemoryRequirements>]) -> u32 {
        let mut n = sink.len() as _;
        unsafe {
            brvk::fns::get_image_sparse_memory_requirements(
                self.device_handle(),
                self.native_ptr(),
                &mut n,
                sink.as_mut_ptr() as _,
            );
        }

        n
    }

    /// Query the memory requirements for a sparse image
    #[implements("alloc")]
    fn sparse_requirements_alloc(&self) -> Vec<brvk::VkSparseImageMemoryRequirements> {
        let n = self.sparse_requirement_count() as usize;
        if n == 0 {
            // no items
            return crate::alloc::empty_sink_buffer();
        }

        let mut xs = Vec::with_capacity(n);
        let n = self.sparse_requirements(xs.spare_capacity_mut()) as usize;
        unsafe {
            xs.set_len(n);
        }

        xs
    }

    /// Retrieve information about an image subresource
    #[implements]
    fn layout_info(&self, subresource: &ImageSubresource) -> brvk::VkSubresourceLayout {
        let mut s = core::mem::MaybeUninit::uninit();
        unsafe {
            brvk::fns::get_image_subresource_layout(
                self.device_handle(),
                self.native_ptr(),
                &subresource.0,
                s.as_mut_ptr(),
            );

            s.assume_init()
        }
    }

    #[inline]
    fn memory_barrier(&self, subresource_range: ImageSubresourceRange, trans: LayoutTransition) -> ImageMemoryBarrier {
        ImageMemoryBarrier::new(self, subresource_range.0, trans)
    }

    #[cfg(feature = "VK_KHR_synchronization2")]
    #[inline]
    fn memory_barrier2<'r>(&'r self, subresource_range: ImageSubresourceRange) -> crate::ImageMemoryBarrier2<'r> {
        crate::ImageMemoryBarrier2::new(self, subresource_range.0)
    }

    /// Returns an image's DRM format modifier
    #[implements("VK_EXT_image_drm_format_modifier")]
    #[inline]
    unsafe fn drm_format_modifier_properties_raw(
        &self,
        sink: &mut core::mem::MaybeUninit<brvk::VkImageDrmFormatModifierPropertiesEXT>,
    ) -> crate::Result<()>
    where
        Self: DeviceChild<ConcreteDevice: DeviceImageDrmFormatModifierExtension>,
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
    fn drm_format_modifier_properties(&self) -> crate::Result<brvk::VkImageDrmFormatModifierPropertiesEXT>
    where
        Self: DeviceChild<ConcreteDevice: DeviceImageDrmFormatModifierExtension>,
    {
        let mut properties = core::mem::MaybeUninit::<brvk::VkImageDrmFormatModifierPropertiesEXT>::uninit();
        unsafe {
            let p = properties.as_mut_ptr();
            core::ptr::addr_of_mut!((*p).sType).write(brvk::VkImageDrmFormatModifierPropertiesEXT::TYPE);
            core::ptr::addr_of_mut!((*p).pNext).write(core::ptr::null_mut());

            self.drm_format_modifier_properties_raw(&mut properties)?;

            Ok(properties.assume_init())
        }
    }
}
DerefContainerBracketImpl!(for Image {
    #[inline(always)]
    fn format(&self) -> brvk::VkFormat {
        T::format(self)
    }

    #[inline(always)]
    fn size(&self) -> &brvk::VkExtent3D {
        T::size(self)
    }

    #[inline(always)]
    fn dimension(&self) -> brvk::VkImageViewType {
        T::dimension(self)
    }
});
GuardsImpl!(for Image {
    #[inline(always)]
    fn format(&self) -> brvk::VkFormat {
        T::format(self)
    }

    #[inline(always)]
    fn size(&self) -> &brvk::VkExtent3D {
        T::size(self)
    }

    #[inline(always)]
    fn dimension(&self) -> brvk::VkImageViewType {
        T::dimension(self)
    }
});

pub trait DeviceChildImage: Image + DeviceChild {}
impl<T: Image + DeviceChild> DeviceChildImage for T {}

pub trait ImageChild {
    type ConcreteImage: crate::Image;

    fn image(&self) -> &Self::ConcreteImage;
}
DerefContainerBracketImpl!(for ImageChild {
    type ConcreteImage = T::ConcreteImage;

    #[inline(always)]
    fn image(&self) -> &Self::ConcreteImage { T::image(self) }
});
GuardsImpl!(for ImageChild {
    type ConcreteImage = T::ConcreteImage;

    #[inline(always)]
    fn image(&self) -> &Self::ConcreteImage { T::image(self) }
});

pub trait ImageChildMut: ImageChild {
    fn image_mut(&mut self) -> &mut Self::ConcreteImage;
}
DerefContainerBracketImpl!(for mut ImageChildMut {
    #[inline(always)]
    fn image_mut(&mut self) -> &mut Self::ConcreteImage {
        T::image_mut(self)
    }
});
GuardsImpl!(for mut ImageChildMut {
    #[inline(always)]
    fn image_mut(&mut self) -> &mut Self::ConcreteImage {
        T::image_mut(self)
    }
});

pub trait ImageView: VkHandle<Handle = brvk::VkImageView> {}
DerefContainerBracketImpl!(for ImageView {});
GuardsImpl!(for ImageView {});

pub trait ConcreteDeviceImageView: ImageView + DeviceChild {}
DerefContainerBracketImpl!(for ConcreteDeviceImageView {});
GuardsImpl!(for ConcreteDeviceImageView {});

/// Image Dimension by corresponding extent type
pub trait ImageSize {
    const DIMENSION: brvk::VkImageType;

    fn conv(self) -> brvk::VkExtent3D;
}
impl ImageSize for u32 {
    const DIMENSION: brvk::VkImageType = brvk::VK_IMAGE_TYPE_1D;

    #[inline(always)]
    fn conv(self) -> brvk::VkExtent3D {
        brvk::VkExtent3D {
            width: self,
            height: 1,
            depth: 1,
        }
    }
}
impl ImageSize for brvk::VkExtent2D {
    const DIMENSION: brvk::VkImageType = brvk::VK_IMAGE_TYPE_2D;

    #[inline(always)]
    fn conv(self) -> brvk::VkExtent3D {
        brvk::VkExtent3D {
            width: self.width,
            height: self.height,
            depth: 1,
        }
    }
}
impl ImageSize for brvk::VkExtent3D {
    const DIMENSION: brvk::VkImageType = brvk::VK_IMAGE_TYPE_3D;

    #[inline(always)]
    fn conv(self) -> brvk::VkExtent3D {
        self
    }
}

/// Opaque handle to a image object(constructed via `ImageDesc`)
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VkImage::OBJECT_TYPE)]
pub struct ImageObject<Device: VkHandle<Handle = brvk::VkDevice>>(
    brvk::VkImage,
    Device,
    brvk::VkImageType,
    brvk::VkFormat,
    brvk::VkExtent3D,
);
#[implements]
impl<Device: VkHandle<Handle = brvk::VkDevice>> Drop for ImageObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn_wrapper::destroy_image(self.device_transparent_ref(), VkHandleRefMut::dangling(self.0), None);
        }
    }
}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Sync> Sync for ImageObject<Device> {}
unsafe impl<Device: VkHandle<Handle = brvk::VkDevice> + Send> Send for ImageObject<Device> {}
impl<Device: VkHandle<Handle = brvk::VkDevice>> DeviceChildHandle for ImageObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice {
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
impl<Device: VkHandle<Handle = brvk::VkDevice>> Image for ImageObject<Device> {
    fn format(&self) -> brvk::VkFormat {
        self.3
    }

    fn size(&self) -> &brvk::VkExtent3D {
        &self.4
    }

    fn dimension(&self) -> brvk::VkImageViewType {
        match self.2 {
            brvk::VK_IMAGE_TYPE_1D => brvk::VK_IMAGE_VIEW_TYPE_1D,
            brvk::VK_IMAGE_TYPE_2D => brvk::VK_IMAGE_VIEW_TYPE_2D,
            brvk::VK_IMAGE_TYPE_3D => brvk::VK_IMAGE_VIEW_TYPE_3D,
            _ => unreachable!(),
        }
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice>> MemoryBound for ImageObject<Device> {
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    type MemoryRequirementsInfo2<'b>
        = ImageMemoryRequirementsInfo2<'b, Self>
    where
        Device: 'b;

    #[implements]
    #[inline(always)]
    fn requirements(&self) -> brvk::VkMemoryRequirements {
        unsafe {
            crate::vkfn_wrapper::get_image_memory_requirements(self.device_transparent_ref(), self.as_transparent_ref())
        }
    }

    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    fn requirements2<'b>(&'b self) -> Self::MemoryRequirementsInfo2<'b> {
        ImageMemoryRequirementsInfo2::new(self)
    }

    #[implements]
    #[inline(always)]
    fn bind(
        &mut self,
        memory: &(impl VkHandle<Handle = brvk::VkDeviceMemory> + ?Sized),
        offset: usize,
    ) -> crate::Result<()>
    where
        Self: VkHandleMut,
    {
        unsafe {
            crate::vkfn_wrapper::bind_image_memory(
                self.device_transparent_ref(),
                VkHandleRefMut::dangling(self.0),
                memory.as_transparent_ref(),
                offset as _,
            )
        }
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice>> ImageObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the parent
    pub const unsafe fn manage(
        handle: brvk::VkImage,
        parent: Device,
        image_type: brvk::VkImageType,
        format: brvk::VkFormat,
        extent: brvk::VkExtent3D,
    ) -> Self {
        Self(handle, parent, image_type, format, extent)
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(
        self,
    ) -> (
        brvk::VkImage,
        Device,
        brvk::VkImageType,
        brvk::VkFormat,
        brvk::VkExtent3D,
    ) {
        let v = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        let (t, f, x) = (self.2, self.3, self.4);
        core::mem::forget(self);

        (v, p, t, f, x)
    }
}
impl<Device: VkHandle<Handle = brvk::VkDevice> + Clone> ImageObject<&'_ Device> {
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
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_COMPRESSION_EXHAUSTED_EXT`]
    /// * [`brvk::VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
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
#[derive(Clone)]
pub struct ImageCreateInfo<'d>(
    brvk::VkImageCreateInfo,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(Option<&'d dyn brvk::VulkanStructure>, Option<&'d [u32]>)>,
);
impl<'d> ImageCreateInfo<'d> {
    #[inline(always)]
    pub fn new<Size: ImageSize>(size: Size, format: brvk::VkFormat) -> Self {
        Self(
            brvk::VkImageCreateInfo {
                sType: brvk::VkImageCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                imageType: Size::DIMENSION,
                extent: size.conv(),
                format,
                usage: 0,
                mipLevels: 1,
                arrayLayers: 1,
                samples: 1,
                initialLayout: brvk::VK_IMAGE_LAYOUT_UNDEFINED,
                tiling: brvk::VK_IMAGE_TILING_OPTIMAL,
                sharingMode: brvk::VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkImageCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkImageCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkImageCreateInfo {
        self.0
    }

    #[inline(always)]
    pub fn with_next(mut self, next: &'d (impl brvk::TypedVulkanStructure + ?Sized)) -> Self {
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
            brvk::VK_SHARING_MODE_EXCLUSIVE
        } else {
            brvk::VK_SHARING_MODE_CONCURRENT
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
        self.0.tiling = brvk::VK_IMAGE_TILING_LINEAR;
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

    /// Merges some custom usage flag bits.
    pub const fn with_usage(mut self, bits: ImageUsageFlags) -> Self {
        self.0.usage |= bits.0;
        self
    }

    /// Overwrites all of custom usage flag bits.
    pub const fn set_usage(mut self, bits: ImageUsageFlags) -> Self {
        self.0.usage = bits.0;
        self
    }
}
impl AsRef<brvk::VkImageCreateInfo> for ImageCreateInfo<'_> {
    #[inline(always)]
    fn as_ref(&self) -> &brvk::VkImageCreateInfo {
        &self.0
    }
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct ImageMemoryRequirementsInfo2<'b, Image: VkHandle<Handle = brvk::VkImage> + 'b>(
    brvk::VkImageMemoryRequirementsInfo2KHR,
    &'b Image,
);
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'b, Image: VkHandle<Handle = brvk::VkImage> + 'b> ImageMemoryRequirementsInfo2<'b, Image> {
    pub fn new(image: &'b Image) -> Self {
        Self(
            brvk::VkImageMemoryRequirementsInfo2KHR {
                sType: brvk::VkImageMemoryRequirementsInfo2KHR::TYPE,
                pNext: core::ptr::null(),
                image: image.native_ptr(),
            },
            image,
        )
    }

    #[implements("Allow1_1APIs")]
    #[inline(always)]
    pub fn query(self, sink: &mut core::mem::MaybeUninit<brvk::VkMemoryRequirements2KHR>)
    where
        Image: crate::DeviceChild,
    {
        unsafe {
            crate::vkfn_wrapper::get_image_memory_requirements2(self.1.device_transparent_ref(), &self, sink);
        }
    }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<Image: VkHandle<Handle = brvk::VkImage>> AsRef<brvk::VkImageMemoryRequirementsInfo2KHR>
    for ImageMemoryRequirementsInfo2<'_, Image>
{
    fn as_ref(&self) -> &brvk::VkImageMemoryRequirementsInfo2KHR {
        &self.0
    }
}

#[cfg(feature = "VK_KHR_bind_memory2")]
#[repr(transparent)]
pub struct BindImageMemoryInfo<'b>(
    brvk::VkBindImageMemoryInfoKHR,
    core::marker::PhantomData<(VkHandleRef<'b, brvk::VkImage>, VkHandleRef<'b, brvk::VkDeviceMemory>)>,
);
#[cfg(feature = "VK_KHR_bind_memory2")]
impl<'b> BindImageMemoryInfo<'b> {
    pub fn new(
        image: &'b (impl VkHandle<Handle = brvk::VkImage> + ?Sized),
        memory: &'b (impl VkHandle<Handle = brvk::VkDeviceMemory> + ?Sized),
        offset: brvk::VkDeviceSize,
    ) -> Self {
        Self(
            brvk::VkBindImageMemoryInfoKHR {
                sType: brvk::VkBindImageMemoryInfoKHR::TYPE,
                pNext: core::ptr::null(),
                image: image.native_ptr(),
                memory: memory.native_ptr(),
                memoryOffset: offset,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkBindImageMemoryInfoKHR`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkBindImageMemoryInfoKHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkBindImageMemoryInfoKHR {
        self.0
    }
}

/// Layouts of image and image subresources
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum ImageLayout {
    /// does not support device access
    Undefined = brvk::VK_IMAGE_LAYOUT_UNDEFINED as _,
    /// does not support device access. host can be written to this memory immediately
    Preinitialized = brvk::VK_IMAGE_LAYOUT_PREINITIALIZED as _,
    /// supports all types of device access
    General = brvk::VK_IMAGE_LAYOUT_GENERAL as _,
    /// must only be used as a color or resolve attachment in a `Framebuffer`
    ColorAttachmentOpt = brvk::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL as _,
    /// must only be used as a depth/stencil attachment in a `Framebuffer`
    DepthStencilAttachmentOpt = brvk::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL as _,
    /// must only be used as a read-only depth/stencil attachment in a `Framebuffer`
    /// and/or as a read-only image in a shader (which can be read as a sampled image,
    /// combined image/sampler and/or input attachment).
    DepthStencilReadOnlyOpt = brvk::VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL as _,
    /// must only be used as a read-only image in a shader (which can be read as a sampled image,
    /// combined image/sampler and/or input attachment).
    ShaderReadOnlyOpt = brvk::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL as _,
    /// must only be used as a source image of a transfer command
    TransferSrcOpt = brvk::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL as _,
    /// must only be used as a destination image of a transfer command
    TransferDestOpt = brvk::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL as _,
    /// must only be used for presenting a swapchain image for display
    #[cfg(feature = "VK_KHR_swapchain")]
    PresentSrc = brvk::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR as _,
}
impl ImageLayout {
    /// Commonly used access types with the layout
    pub fn default_access_mask(self) -> brvk::VkAccessFlags {
        match self {
            Self::Undefined | Self::Preinitialized => 0,
            Self::General => brvk::VK_ACCESS_MEMORY_READ_BIT,
            Self::ColorAttachmentOpt => brvk::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            Self::DepthStencilAttachmentOpt => brvk::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
            Self::DepthStencilReadOnlyOpt => brvk::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT,
            Self::ShaderReadOnlyOpt => brvk::VK_ACCESS_SHADER_READ_BIT,
            Self::TransferSrcOpt => brvk::VK_ACCESS_TRANSFER_READ_BIT,
            Self::TransferDestOpt => brvk::VK_ACCESS_TRANSFER_WRITE_BIT,
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::PresentSrc => brvk::VK_ACCESS_MEMORY_READ_BIT,
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
pub struct ImageUsageFlags(brvk::VkImageUsageFlags);
impl ImageUsageFlags {
    /// The image can be used as the source of a transfer command
    pub const TRANSFER_SRC: Self = Self(brvk::VK_IMAGE_USAGE_TRANSFER_SRC_BIT);
    /// The image can be used as the destination of a transfer command
    pub const TRANSFER_DEST: Self = Self(brvk::VK_IMAGE_USAGE_TRANSFER_DST_BIT);
    /// The image can be used to create `ImageView` suitable for occupying a `DescriptorSet` slot
    /// either of type `DescriptorType::SampledImage` or `DescriptorType::CombinedImageSampler`, and be sampled by a shader
    pub const SAMPLED: Self = Self(brvk::VK_IMAGE_USAGE_SAMPLED_BIT);
    /// The image can be used to create a `ImageView` suitable for occupying a `DescriptorSet` slot of type `DescriptorType::StorageImage`
    pub const STORAGE: Self = Self(brvk::VK_IMAGE_USAGE_STORAGE_BIT);
    /// The image can be used to create a `ImageView` suitable for use as a color or resolve attachment in a `Framebuffer`
    pub const COLOR_ATTACHMENT: Self = Self(brvk::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT);
    /// The image can be used to create a `ImageView` suitable for use as a depth/stencil attachment in a `Framebuffer`
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(brvk::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT);
    /// The memory bound to this image will have been allocated with the `brvk::VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`
    /// This bit can be set for any image that can be used to create a `ImageView` suitable for use as a color, resolve, depth/stencil,
    /// or input attachment
    pub const TRANSIENT_ATTACHMENT: Self = Self(brvk::VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT);
    /// The image can be used to create a `ImageView` suitable for occupying `DescriptorSet` slot of type `DescriptorType::InputAttachment`;
    /// be read from a shader as an input attachment; and be used as an input attachment in a framebuffer
    pub const INPUT_ATTACHMENT: Self = Self(brvk::VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT);
}

/// Bitmask specifying additional parameters of an image
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[bitflags_newtype]
pub struct ImageFlags(brvk::VkImageCreateFlags);
impl ImageFlags {
    /// Empty bits
    pub const EMPTY: Self = Self(0);
    /// The image will be backed using sparse memory binding
    pub const SPARSE_BINDING: Self = Self(brvk::VK_IMAGE_CREATE_SPARSE_BINDING_BIT);
    /// The image can be partially backed using sparse memory binding. This bit is with `SPARSE_BINDING` implicitly
    pub const SPARSE_RESIDENCY: Self =
        Self(brvk::VK_IMAGE_CREATE_SPARSE_BINDING_BIT | brvk::VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT);
    /// The image will be backed using sparse memory binding with memory ranges
    /// that might also simultaneously be backing another image. This bit is with `SPARSE_BINDING` implicitly
    pub const SPARSE_ALIASED: Self =
        Self(brvk::VK_IMAGE_CREATE_SPARSE_BINDING_BIT | brvk::VK_IMAGE_CREATE_SPARSE_ALIASED_BIT);
    /// The image can be used to create a `ImageView` with a different format from the image
    pub const MUTABLE_FORMAT: Self = Self(brvk::VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT);
    /// The image can be used to create a `ImageView` of type `ImageViewType::Cube` or `ImageViewType::CubeArray`
    pub const CUBE_COMPATIBLE: Self = Self(brvk::VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT);
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[repr(transparent)]
#[derive(Clone)]
pub struct ImageSparseMemoryRequirementsInfo2<'r>(
    pub(crate) brvk::VkImageSparseMemoryRequirementsInfo2KHR,
    core::marker::PhantomData<(
        Option<&'r dyn brvk::VulkanStructure>,
        &'r dyn VkHandle<Handle = brvk::VkImage>,
    )>,
);
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'r> ImageSparseMemoryRequirementsInfo2<'r> {
    #[inline]
    pub fn new(image: &'r (impl VkHandle<Handle = brvk::VkImage> + ?Sized)) -> Self {
        Self(
            brvk::VkImageSparseMemoryRequirementsInfo2KHR {
                sType: brvk::VkImageSparseMemoryRequirementsInfo2KHR::TYPE,
                pNext: core::ptr::null(),
                image: image.native_ptr(),
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkImageSparseMemoryRequirementsInfo2KHR`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkImageSparseMemoryRequirementsInfo2KHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkImageSparseMemoryRequirementsInfo2KHR {
        self.0
    }

    #[inline]
    pub fn with_next(mut self, next: &'r (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next.as_generic() as *const _ as _;
        self
    }
}

/// Bitmask specifying which aspects of an image are included in a view
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[bitflags_newtype]
pub struct AspectMask(brvk::VkImageAspectFlags);
impl AspectMask {
    /// No aspect mask
    pub const EMPTY: Self = Self(0);
    /// The color aspect
    pub const COLOR: Self = Self(brvk::VK_IMAGE_ASPECT_COLOR_BIT);
    /// The depth aspect
    pub const DEPTH: Self = Self(brvk::VK_IMAGE_ASPECT_DEPTH_BIT);
    /// The stencil aspect
    pub const STENCIL: Self = Self(brvk::VK_IMAGE_ASPECT_STENCIL_BIT);
    /// The metadata aspect, used for sparse sparse resource operations
    pub const METADATA: Self = Self(brvk::VK_IMAGE_ASPECT_METADATA_BIT);
}

#[repr(transparent)]
pub struct ImageSubresource(pub brvk::VkImageSubresource);
impl ImageSubresource {
    pub const fn new(aspect_mask: AspectMask, mip_level: u32, array_layer: u32) -> Self {
        Self(brvk::VkImageSubresource {
            aspectMask: aspect_mask.bits(),
            mipLevel: mip_level,
            arrayLayer: array_layer,
        })
    }
}

#[repr(transparent)]
pub struct ImageSubresourceLayers(pub brvk::VkImageSubresourceLayers);
impl ImageSubresourceLayers {
    pub const fn new(aspect_mask: AspectMask, mip_level: u32, layer_range: core::ops::Range<u32>) -> Self {
        Self(brvk::VkImageSubresourceLayers {
            aspectMask: aspect_mask.bits(),
            mipLevel: mip_level,
            baseArrayLayer: layer_range.start,
            layerCount: layer_range.end - layer_range.start,
        })
    }
}

#[repr(transparent)]
pub struct ImageSubresourceRange(pub brvk::VkImageSubresourceRange);
impl ImageSubresourceRange {
    /// Constructs a new [`ImageSubresourceRange`] data
    pub const fn new(
        aspect_mask: AspectMask,
        mip_range: core::ops::Range<u32>,
        array_layer_range: core::ops::Range<u32>,
    ) -> Self {
        Self(brvk::VkImageSubresourceRange {
            aspectMask: aspect_mask.bits(),
            baseMipLevel: mip_range.start,
            levelCount: mip_range.end - mip_range.start,
            baseArrayLayer: array_layer_range.start,
            layerCount: array_layer_range.end - array_layer_range.start,
        })
    }

    /// Retrieves single subresource slice in this range
    pub const fn subresource(&self, mip_level_offset: u32, array_layer_offset: u32) -> ImageSubresource {
        ImageSubresource(brvk::VkImageSubresource {
            aspectMask: self.0.aspectMask,
            mipLevel: self.0.baseMipLevel + mip_level_offset,
            arrayLayer: self.0.baseArrayLayer + array_layer_offset,
        })
    }
}

/// Opaque handle to a image view object
#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VkImageView::OBJECT_TYPE)]
pub struct ImageViewObject<Image: DeviceChildHandle>(brvk::VkImageView, Image);
#[implements]
impl<Image: DeviceChildHandle> Drop for ImageViewObject<Image> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn_wrapper::destroy_image_view(
                self.device_transparent_ref(),
                VkHandleRefMut::dangling(self.0),
                None,
            );
        }
    }
}
unsafe impl<Image: DeviceChildHandle + Sync> Sync for ImageViewObject<Image> {}
unsafe impl<Image: DeviceChildHandle + Send> Send for ImageViewObject<Image> {}
impl<Image: DeviceChildHandle> DeviceChildHandle for ImageViewObject<Image> {
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice {
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
impl<Image: self::Image> ImageChildMut for ImageViewObject<Image> {
    #[inline(always)]
    fn image_mut(&mut self) -> &mut Image {
        &mut self.1
    }
}
impl<Image: DeviceChildHandle> ImageView for ImageViewObject<Image> {}
impl<Image: DeviceChildHandle> ImageViewObject<Image> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the parent
    pub const unsafe fn manage(handle: brvk::VkImageView, parent: Image) -> Self {
        Self(handle, parent)
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (brvk::VkImageView, Image) {
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
    /// * [`brvk::VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    /// * [`brvk::VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`]
    #[implements]
    #[inline]
    pub fn new(image: Image, info: &ImageViewCreateInfo) -> crate::Result<Self> {
        use crate::Device;

        Ok(unsafe { Self::manage(image.device().new_image_view_raw(info, None)?, image) })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ImageViewCreateInfo<'r>(
    brvk::VkImageViewCreateInfo,
    core::marker::PhantomData<&'r dyn VkHandle<Handle = brvk::VkImage>>,
);
impl<'r> ImageViewCreateInfo<'r> {
    pub fn new(
        source: &'r (impl VkHandle<Handle = brvk::VkImage> + ?Sized),
        subresource_range: ImageSubresourceRange,
        view_type: brvk::VkImageViewType,
        format: brvk::VkFormat,
    ) -> Self {
        Self(
            brvk::VkImageViewCreateInfo {
                sType: brvk::VkImageViewCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                image: source.native_ptr(),
                viewType: view_type,
                format,
                components: brvk::VkComponentMapping {
                    r: brvk::VK_COMPONENT_SWIZZLE_R,
                    g: brvk::VK_COMPONENT_SWIZZLE_G,
                    b: brvk::VK_COMPONENT_SWIZZLE_B,
                    a: brvk::VK_COMPONENT_SWIZZLE_A,
                },
                subresourceRange: subresource_range.0,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`brvk::VkImageViewCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkImageViewCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkImageViewCreateInfo {
        self.0
    }

    pub const fn with_format_mutation(mut self, format: brvk::VkFormat) -> Self {
        self.0.format = format;
        self
    }

    pub const fn with_mapping(mut self, mapping: brvk::VkComponentMapping) -> Self {
        self.0.components = mapping;
        self
    }

    pub const fn with_dimension(mut self, dimension: brvk::VkImageViewType) -> Self {
        self.0.viewType = dimension;
        self
    }
}

pub struct ImageViewBuilder<I: Image>(ImageViewCreateInfo<'static>, I);
impl<I: Image> ImageViewBuilder<I> {
    pub fn new(source: I, subresource_range: brvk::VkImageSubresourceRange) -> Self {
        Self(
            unsafe {
                ImageViewCreateInfo::from_raw(brvk::VkImageViewCreateInfo {
                    sType: brvk::VkImageViewCreateInfo::TYPE,
                    pNext: core::ptr::null(),
                    flags: 0,
                    image: source.native_ptr(),
                    viewType: source.dimension(),
                    format: source.format(),
                    components: brvk::VkComponentMapping {
                        r: brvk::VK_COMPONENT_SWIZZLE_R,
                        g: brvk::VK_COMPONENT_SWIZZLE_G,
                        b: brvk::VK_COMPONENT_SWIZZLE_B,
                        a: brvk::VK_COMPONENT_SWIZZLE_A,
                    },
                    subresourceRange: subresource_range,
                })
            },
            source,
        )
    }

    pub const fn with_format_mutation(mut self, format: brvk::VkFormat) -> Self {
        self.0 = self.0.with_format_mutation(format);
        self
    }

    pub const fn with_mapping(mut self, mapping: brvk::VkComponentMapping) -> Self {
        self.0 = self.0.with_mapping(mapping);
        self
    }

    pub const fn with_dimension(mut self, dimension: brvk::VkImageViewType) -> Self {
        self.0 = self.0.with_dimension(dimension);
        self
    }

    /// Create a new image view from an existing image
    /// # Failure
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `brvk::VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
    #[implements]
    pub fn create(self) -> crate::Result<ImageViewObject<I>>
    where
        I: DeviceChild,
    {
        ImageViewObject::new(self.1, &self.0)
    }
}
