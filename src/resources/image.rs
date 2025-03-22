#[implements]
use crate::DeviceMemory;
use crate::{
    Device, Extent3D, Format, ImageMemoryBarrier, MemoryBound, VkHandle, VkObject, VulkanStructure,
    VulkanStructureAsRef,
    ffi_helper::{opt_pointer, slice_as_ptr_empty_null},
    vk::*,
};
use core::{marker::PhantomData, mem::MaybeUninit, ops::Range, ptr::null_mut};
use derives::{bitflags_newtype, implements};

// Vulkan ReExports //
pub type SparseImageMemoryRequirements = VkSparseImageMemoryRequirements;

/// Opaque handle to an image object.
#[repr(transparent)]
pub struct Image(VkImage_T);
#[implements]
impl Image {
    /// Destroy an image object.
    pub unsafe fn destroy(&mut self, device: &Device, allocation_callbacks: Option<&VkAllocationCallbacks>) {
        unsafe {
            crate::vkfn::destroy_image(
                device as *const _ as _,
                self as *mut _ as _,
                opt_pointer(allocation_callbacks),
            )
        }
    }

    /// Query a count of the memory requirements for a sparse image
    #[inline]
    pub unsafe fn sparse_requirement_count(&self, device: &Device) -> u32 {
        let mut n = 0;
        unsafe {
            crate::vkfn::get_image_sparse_memory_requirements(
                device as *const _ as _,
                self as *const _ as _,
                &mut n,
                null_mut(),
            );
        }

        n
    }

    /// Query the memory requirements for a sparse image
    #[inline]
    pub unsafe fn sparse_requirements(&self, device: &Device, sink: &mut [SparseImageMemoryRequirements]) -> u32 {
        let mut n = sink.len() as _;
        unsafe {
            crate::vkfn::get_image_sparse_memory_requirements(
                device as *const _ as _,
                self as *const _ as _,
                &mut n,
                sink.as_mut_ptr(),
            );
        }

        n
    }

    /// Query the memory requirements for a sparse image
    #[cfg(feature = "alloc")]
    pub unsafe fn sparse_requirements_alloc(&self, device: &Device) -> Vec<SparseImageMemoryRequirements> {
        let n = unsafe { self.sparse_requirement_count(device) };
        if n == 0 {
            // no items
            return crate::alloc::empty_sink_buffer();
        }

        let mut xs = unsafe { crate::alloc::alloc_sink_buffer(n as _) };
        unsafe {
            self.sparse_requirements(device, &mut xs);
        }

        xs
    }

    /// Retrieve information about an image subresource.
    #[inline]
    pub unsafe fn get_subresource_layout(
        &self,
        device: &Device,
        subresource: &ImageSubresource,
        sink: &mut MaybeUninit<SubresourceLayout>,
    ) {
        unsafe {
            crate::vkfn::get_image_subresource_layout(
                device as *const _ as _,
                self as *const _ as _,
                subresource,
                sink.as_mut_ptr(),
            )
        }
    }

    /// Retrieve information about an image subresource.
    #[inline]
    pub unsafe fn subresource_layout(&self, device: &Device, subresource: &ImageSubresource) -> SubresourceLayout {
        let mut sink = MaybeUninit::uninit();
        unsafe {
            self.get_subresource_layout(device, subresource, &mut sink);
        }

        unsafe { sink.assume_init() }
    }
}
impl Image {
    #[inline]
    pub fn memory_barrier(&self, subresource: &ImageSubresource, trans: LayoutTransition) -> ImageMemoryBarrier {
        ImageMemoryBarrier::new(self, subresource, trans)
    }

    #[cfg(feature = "VK_KHR_synchronization2")]
    #[inline]
    pub fn memory_barrier2<'r>(&'r self, subresource: &'r ImageSubresource) -> crate::ImageMemoryBarrier2<'r> {
        crate::ImageMemoryBarrier2::new(self, subresource)
    }
}
impl MemoryBound for Image {
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    type MemoryRequirementsInfo2<'b> = ImageMemoryRequirementsInfo2<'b>;

    #[implements]
    unsafe fn get_memory_requirements(&self, device: &Device, sink: &mut MaybeUninit<super::MemoryRequirements>) {
        unsafe {
            crate::vkfn::get_image_memory_requirements(
                device as *const _ as _,
                self as *const _ as _,
                sink.as_mut_ptr(),
            )
        }
    }

    #[implements("VK_KHR_get_memory_requirements2")]
    unsafe fn memory_requirements2<'b>(&'b self, device: &'b Device) -> Self::MemoryRequirementsInfo2<'b> {
        unsafe { ImageMemoryRequirementsInfo2::new(self, device) }
    }

    #[implements]
    unsafe fn bind(
        &mut self,
        device: &Device,
        memory: &(impl DeviceMemory + ?Sized),
        offset: usize,
    ) -> crate::Result<()> {
        unsafe {
            crate::vkfn::bind_image_memory(
                device as *const _ as _,
                self as *mut _ as _,
                memory.native_ptr(),
                offset as _,
            )
            .into_result()
            .map(drop)
        }
    }
}

/// Opaque handle to an image view object.
#[repr(transparent)]
pub struct ImageView(VkImageView_T);
#[implements]
impl ImageView {
    /// Destroy an image view object.
    pub unsafe fn destroy(&mut self, device: &Device, allocation_callbacks: Option<&VkAllocationCallbacks>) {
        unsafe {
            crate::vkfn::destroy_image_view(
                device as *const _ as _,
                self as *mut _ as _,
                opt_pointer(allocation_callbacks),
            )
        }
    }
}

/// Image Dimension by corresponding extent type
pub trait ImageSize {
    const DIMENSION: ImageType;

    fn conv(self) -> Extent3D;
}
impl ImageSize for u32 {
    const DIMENSION: ImageType = ImageType::Dim1;

    fn conv(self) -> Extent3D {
        Extent3D {
            width: self,
            height: 1,
            depth: 1,
        }
    }
}
impl ImageSize for VkExtent2D {
    const DIMENSION: ImageType = ImageType::Dim2;

    fn conv(self) -> Extent3D {
        self.with_depth(1)
    }
}
impl ImageSize for VkExtent3D {
    const DIMENSION: ImageType = ImageType::Dim2;

    fn conv(self) -> Extent3D {
        self
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

pub type SubresourceLayout = VkSubresourceLayout;
pub type ImageSubresource = VkImageSubresource;

pub type ImageSubresourceRange = VkImageSubresourceRange;
impl ImageSubresourceRange {
    /// Creates subresource-range
    #[inline]
    pub const fn new(aspect_mask: AspectMask, mip_level: Range<u32>, array_layers: Range<u32>) -> Self {
        Self {
            aspectMask: aspect_mask.0,
            baseMipLevel: mip_level.start,
            levelCount: mip_level.len() as _,
            baseArrayLayer: array_layers.start,
            layerCount: array_layers.len() as _,
        }
    }

    /// Retrieves single subresource in this range
    pub fn subresource(&self, mip_level_offset: u32, array_layer_offset: u32) -> ImageSubresource {
        ImageSubresource {
            aspectMask: self.1.aspectMask,
            mipLevel: self.1.baseMipLevel + mip_level_offset,
            arrayLayer: self.1.baseArrayLayer + array_layer_offset,
        }
    }
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct ImageMemoryRequirementsInfo2<'b>(VkImageMemoryRequirementsInfo2KHR, &'b Device, PhantomData<&'b Image>);
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'b> ImageMemoryRequirementsInfo2<'b> {
    pub const unsafe fn new(image: &'b Image, device: &'b Device) -> Self {
        Self(
            VkImageMemoryRequirementsInfo2KHR {
                sType: VkImageMemoryRequirementsInfo2KHR::TYPE,
                pNext: core::ptr::null(),
                image: image as *const _ as _,
            },
            device,
            PhantomData,
        )
    }

    #[implements("Allow1_1APIs")]
    pub fn query(self, sink: &mut MaybeUninit<VkMemoryRequirements2KHR>) {
        unsafe {
            crate::vkfn::get_image_memory_requirements2(self.1 as *const _ as _, &self.0, sink.as_mut_ptr());
        }
    }
}

/// Layouts of image and image subresources
#[repr(i32)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum ImageLayout {
    /// does not support device access
    Undefined = VK_IMAGE_LAYOUT_UNDEFINED,
    /// does not support device access. host can be written to this memory immediately
    Preinitialized = VK_IMAGE_LAYOUT_PREINITIALIZED,
    /// supports all types of device access
    General = VK_IMAGE_LAYOUT_GENERAL,
    /// must only be used as a color or resolve attachment in a `Framebuffer`
    ColorAttachmentOpt = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
    /// must only be used as a depth/stencil attachment in a `Framebuffer`
    DepthStencilAttachmentOpt = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    /// must only be used as a read-only depth/stencil attachment in a `Framebuffer`
    /// and/or as a read-only image in a shader (which can be read as a sampled image,
    /// combined image/sampler and/or input attachment).
    DepthStencilReadOnlyOpt = VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL,
    /// must only be used as a read-only image in a shader (which can be read as a sampled image,
    /// combined image/sampler and/or input attachment).
    ShaderReadOnlyOpt = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
    /// must only be used as a source image of a transfer command
    TransferSrcOpt = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
    /// must only be used as a destination image of a transfer command
    TransferDestOpt = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
    /// must only be used for presenting a swapchain image for display
    #[cfg(feature = "VK_KHR_swapchain")]
    PresentSrc = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
}
impl ImageLayout {
    /// Commonly used access types with the layout
    pub const fn default_access_mask(self) -> VkAccessFlags {
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
    pub const fn to(self, after: Self) -> LayoutTransition {
        LayoutTransition { from: self, to: after }
    }

    /// Constructs the transition between image layouts (in reverse order).
    pub const fn from(self, before: Self) -> LayoutTransition {
        LayoutTransition { from: before, to: self }
    }

    /// Constructs the transition from undefined layout to this layout (convenient function).
    pub const fn from_undefined(self) -> LayoutTransition {
        self.from(Self::Undefined)
    }

    /// Constructs the empty transition.
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

/// Specifies the tiling arrangement of data in an image.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImageTiling {
    /// Optimal tiling
    /// (texels are laid out in an implementation-dependent arrangement, for more efficient memory access).
    Optimal = VK_IMAGE_TILING_OPTIMAL,
    /// Linear tiling
    /// (texels are laid out in memory in row-major order, possibly with some padding on each row).
    Linear = VK_IMAGE_TILING_LINEAR,
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    /// The image's tiling is defined by a [Linux DRM format modifier](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#glossary-drm-format-modifier).
    /// The modifier is specified at image creation with [`VkImageDrmFormatModifierListCreateInfoEXT`]
    /// or [`VkImageDrmFormatModifierExplicitCreateInfoEXT`],
    /// and can be queried with [`vkGetImageDrmFormatModifierPropertiesEXT`]
    DrmFormatModifierEXT = VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT,
}

/// Specifies the type of an image object.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImageType {
    /// one-dimensional image.
    Dim1 = VK_IMAGE_TYPE_1D,
    /// two-dimensional image.
    Dim2 = VK_IMAGE_TYPE_2D,
    /// three-dimensional image.
    Dim3 = VK_IMAGE_TYPE_3D,
}

/// Image view types.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImageViewType {
    Dim1 = VK_IMAGE_VIEW_TYPE_1D,
    Dim2 = VK_IMAGE_VIEW_TYPE_2D,
    Dim3 = VK_IMAGE_VIEW_TYPE_3D,
    Cube = VK_IMAGE_VIEW_TYPE_CUBE,
    Dim1Array = VK_IMAGE_VIEW_TYPE_1D_ARRAY,
    Dim2Array = VK_IMAGE_VIEW_TYPE_2D_ARRAY,
    CubeArray = VK_IMAGE_VIEW_TYPE_CUBE_ARRAY,
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageSparseMemoryRequirementsInfo2<'r>(
    pub(crate) VkImageSparseMemoryRequirementsInfo2KHR,
    PhantomData<(Option<&'r dyn VulkanStructureAsRef>, &'r Image)>,
);
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'r> ImageSparseMemoryRequirementsInfo2<'r> {
    pub const fn new(image: &'r Image) -> Self {
        Self(
            VkImageSparseMemoryRequirementsInfo2KHR {
                sType: VkImageSparseMemoryRequirementsInfo2KHR::TYPE,
                pNext: core::ptr::null(),
                image: image as *const _ as _,
            },
            PhantomData,
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

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageViewCreateInfo<'r>(VkImageViewCreateInfo, PhantomData<&'r Image>);
impl<'r> ImageViewCreateInfo<'r> {
    pub const fn new(
        source: &'r Image,
        subresource_range: ImageSubresourceRange,
        view_type: ImageViewType,
        format: Format,
    ) -> Self {
        Self(
            VkImageViewCreateInfo {
                sType: VkImageViewCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                image: source as *const _ as _,
                viewType: view_type,
                format,
                components: VkComponentMapping::default(),
                subresourceRange: subresource_range,
            },
            PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkImageViewCreateInfo) -> Self {
        Self(raw, PhantomData)
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
