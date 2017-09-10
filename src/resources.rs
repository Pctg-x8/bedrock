//! Vulkan Resources

use vk::*;
use std::rc::Rc as RefCounter;
#[cfg(feature = "FeImplements")] use VkResultHandler;

struct DeviceMemoryCell(VkDeviceMemory, ::Device);
struct BufferCell(VkBuffer, ::Device);
struct ImageCell(VkImage, ::Device, VkImageType, VkFormat);
/// Opaque handle to a device memory object
pub struct DeviceMemory(RefCounter<DeviceMemoryCell>);
/// Opaque handle to a buffer object(constructed via `BufferDesc`)
#[derive(Clone)] pub struct Buffer(RefCounter<BufferCell>);
/// Opaque handle to a image object(constructed via `ImageDesc`)
#[derive(Clone)] pub struct Image(RefCounter<ImageCell>);
/// Opaque handle to a buffer view object
pub struct BufferView(VkBufferView, Buffer);
/// Opaque handle to a image view object
pub struct ImageView(VkImageView, Image);

#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{
	for DeviceMemoryCell[vkFreeMemory], BufferCell[vkDestroyBuffer], ImageCell[vkDestroyImage]
}
#[cfg(feature = "FeImplements")]
impl Drop for BufferView { fn drop(&mut self) { unsafe { vkDestroyBufferView(self.1 .0 .1.native_ptr(), self.0, ::std::ptr::null()) }; } }
#[cfg(feature = "FeImplements")]
impl Drop for ImageView  { fn drop(&mut self) { unsafe { vkDestroyImageView (self.1 .0 .1.native_ptr(), self.0, ::std::ptr::null()) }; } }

impl ::DeviceChild<VkDeviceMemory> for DeviceMemory
{
	unsafe fn from_unchecked(p: VkDeviceMemory, parent: &::Device) -> Self
	{
		DeviceMemory(RefCounter::new(DeviceMemoryCell(p, parent.clone())))
	}
}

/// Bitmask specifying allowed usage of a buffer
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BufferUsage(pub VkBufferUsageFlags);
impl BufferUsage
{
	/// Specifies that the buffer can be used as the source of a transfer command
	pub const TRANSFER_SRC: Self = BufferUsage(VK_BUFFER_USAGE_TRANSFER_SRC_BIT);
	/// Specifies that the buffer can be used as the destination of a transfer command
	pub const TRANSFER_DEST: Self = BufferUsage(VK_BUFFER_USAGE_TRANSFER_DST_BIT);
	/// Specifies that the buffer can be used to create a `BufferView` suitable for
	/// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`
	pub const UNIFORM_TEXEL_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT);
	/// Specifies that the buffer can be used to create a `BufferView` suitable for
	/// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`
	pub const STORAGE_TEXEL_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT);
	/// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
	/// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`
	pub const UNIFORM_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT);
	/// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
	/// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
	pub const STORAGE_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_STORAGE_BUFFER_BIT);
	/// Specifies that the buffer is suitable for passing as the `buffer` parameter to `DrawCommandBuffer::bind_index_buffer`
	pub const INDEX_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_INDEX_BUFFER_BIT);
	/// Specifies that the buffer is suitable for passing as an element of the `buffers` array to `DrawCommandBuffer::bind_vertex_buffers`
	pub const VERTEX_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_VERTEX_BUFFER_BIT);
	/// Specifies that the buffer is suitable for passing as the `buffer` parameter to
	/// `DrawCommandBuffer::draw_indirect`, `DrawCommandBuffer::draw_indexed_indirect`, or `ComputeCommandBuffer::dispatch_indirect`
	pub const INDIRECT_BUFFER: Self = BufferUsage(VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT);

	/// Specifies that the buffer can be used as the source of a transfer command
	pub fn transfer_src(&self) -> Self { BufferUsage(self.0 | Self::TRANSFER_SRC.0) }
	/// Specifies that the buffer can be used as the destination of a transfer command
	pub fn transfer_dest(&self) -> Self { BufferUsage(self.0 | Self::TRANSFER_DEST.0) }
	/// Specifies that the buffer can be used to create a `BufferView` suitable for
	/// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`
	pub fn uniform_texel_buffer(&self) -> Self { BufferUsage(self.0 | Self::UNIFORM_TEXEL_BUFFER.0) }
	/// Specifies that the buffer can be used to create a `BufferView` suitable for
	/// occupying a `DescriptorSet` slot of type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`
	pub fn storage_texel_buffer(&self) -> Self { BufferUsage(self.0 | Self::STORAGE_TEXEL_BUFFER.0) }
	/// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
	/// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`
	pub fn uniform_buffer(&self) -> Self { BufferUsage(self.0 | Self::UNIFORM_BUFFER.0) }
	/// Specifies that the buffer can be used in a `DescriptorBufferInfo` suitable for
	/// occupying a `DescriptorSet` slot either of type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
	pub fn storage_buffer(&self) -> Self { BufferUsage(self.0 | Self::STORAGE_BUFFER.0) }
	/// Specifies that the buffer is suitable for passing as the `buffer` parameter to `DrawCommandBuffer::bind_index_buffer`
	pub fn index_buffer(&self) -> Self { BufferUsage(self.0 | Self::INDEX_BUFFER.0) }
	/// Specifies that the buffer is suitable for passing as an element of the `buffers` array to `DrawCommandBuffer::bind_vertex_buffers`
	pub fn vertex_buffer(&self) -> Self { BufferUsage(self.0 | Self::VERTEX_BUFFER.0) }
	/// Specifies that the buffer is suitable for passing as the `buffer` parameter to
	/// `DrawCommandBuffer::draw_indirect`, `DrawCommandBuffer::draw_indexed_indirect`, or `ComputeCommandBuffer::dispatch_indirect`
	pub fn indirect_buffer(&self) -> Self { BufferUsage(self.0 | Self::INDIRECT_BUFFER.0) }
}
/// Bitset specifying additional parameters of a buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)] #[repr(C)] pub enum BufferSparseBinding
{
	/// No sparse binding features
	None = 0,
	/// the buffer will be backed using sparse memory binding
	Bound = VK_BUFFER_CREATE_SPARSE_BINDING_BIT as _,
	/// the buffer can be partially backed using sparse memory binding.
	Residency = (VK_BUFFER_CREATE_SPARSE_BINDING_BIT | VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT) as _,
	/// the buffer will be backed using sparse memory binding with memory ranges
	/// that might also simultaneously be backing another buffer (or another portion of the same buffer)
	Aliased = (VK_BUFFER_CREATE_SPARSE_BINDING_BIT | VK_BUFFER_CREATE_SPARSE_ALIASED_BIT) as _,
	/// Aliased and Residency
	Both = (VK_BUFFER_CREATE_SPARSE_BINDING_BIT | VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT | VK_BUFFER_CREATE_SPARSE_ALIASED_BIT) as _
}

/// Builder structure specifying the parameters of a newly created buffer object
pub struct BufferDesc { cinfo: VkBufferCreateInfo, #[allow(dead_code)] sharing_queues: Vec<u32> }
impl BufferDesc
{
	pub fn new(byte_size: usize, usage: BufferUsage) -> Self
	{
		BufferDesc
		{
			cinfo: VkBufferCreateInfo
			{
				size: byte_size as _, usage: usage.0, .. Default::default()
			}, sharing_queues: Vec::new()
		}
	}
	/// A list of queue families that will access this buffer
	pub fn sharing_queue_families(&mut self, indices: Vec<u32>) -> &mut Self
	{
		self.sharing_queues = indices;
		self.cinfo.sharingMode = if self.sharing_queues.is_empty() { VK_SHARING_MODE_EXCLUSIVE } else { VK_SHARING_MODE_CONCURRENT };
		self.cinfo.queueFamilyIndexCount = self.sharing_queues.len() as _;
		self.cinfo.pQueueFamilyIndices = self.sharing_queues.as_ptr();
		self
	}
	/// A bitmask of `BufferSparseBinding` specifying additional parameters of the buffer
	pub fn sparse_binding_opt(&mut self, opt: BufferSparseBinding) -> &mut Self
	{
		self.cinfo.flags = opt as _; self
	}
	/// Create a new buffer object
	/// # Failure
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	#[cfg(feature = "FeImplements")]
	pub fn create(&self, device: &::Device) -> ::Result<Buffer>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateBuffer(device.native_ptr(), &self.cinfo, ::std::ptr::null(), &mut h) }
			.into_result().map(|_| Buffer(RefCounter::new(BufferCell(h, device.clone()))))
	}
}

/// Builder structure specifying the parameters of a newly created image object
pub struct ImageDesc { cinfo: VkImageCreateInfo, sharing_queues: Vec<u32> }
impl ImageDesc
{
	pub fn new<Size: ImageSize>(size: Size, format: VkFormat, initial_layout: ImageLayout) -> Self
	{
		ImageDesc
		{
			cinfo: VkImageCreateInfo
			{
				imageType: Size::dimension(), extent: unsafe { ::std::mem::transmute(size.expand()) }, format,
				mipLevels: 1, arrayLayers:1, samples: 1, initialLayout: initial_layout as _,
				.. Default::default()
			},
			sharing_queues: Vec::new()
		}
	}
	pub fn sharing_queue_families(&mut self, indices: Vec<u32>) -> &mut Self
	{
		self.sharing_queues = indices;
		self.cinfo.sharingMode = if self.sharing_queues.is_empty() { VK_SHARING_MODE_EXCLUSIVE } else { VK_SHARING_MODE_CONCURRENT };
		self.cinfo.queueFamilyIndexCount = self.sharing_queues.len() as _;
		self.cinfo.pQueueFamilyIndices = self.sharing_queues.as_ptr();
		self
	}
	/// bitmask of 1, 2, 4, 8, 16, 32, 64
	pub fn sample_counts(&mut self, count_bits: u32) -> &mut Self
	{
		self.cinfo.samples = count_bits; self
	}
	pub fn use_linear_tiling(&mut self) -> &mut Self
	{
		self.cinfo.tiling = VK_IMAGE_TILING_LINEAR; self
	}
	pub fn mutable_format(&mut self) -> &mut Self
	{
		self.cinfo.flags |= VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT; self
	}
	#[cfg(features = "FeImplements")]
	pub fn create(&self, device: &::Device) -> ::Result<Image>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateImage(device.native_ptr(), &self.cinfo, std::ptr::null(), &mut h) }
			.into_result().map(|_| Image(RefCounter::new(ImageCell(h, device.clone(), self.cinfo.imageType, self.cinfo.format))))
	}
}

#[cfg(feature = "FeImplements")]
impl Buffer
{
	pub fn create_view(&self, format: VkFormat, range: ::std::ops::Range<u64>) -> ::Result<BufferView>
	{
		let cinfo = VkBufferViewCreateInfo
		{
			buffer: self.0 .0, format, offset: range.start, range: range.end - range.start, .. Default::default()
		};
		let mut h = unsafe { ::std::mem::zeroed() };
		unsafe { vkCreateBufferView(self.0 .1.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }
			.into_result().map(|_| BufferView(h, self.clone()))
	}
}
#[cfg(feature = "FeImplements")]
impl Image
{
	pub fn create_view(&self, format: Option<VkFormat>, cmap: &ComponentMapping, subresource_range: &ImageSubresourceRange)
		-> ::Result<ImageView>
	{
		let format = format.unwrap_or(self.0 .3);
		let cinfo = VkImageViewCreateInfo
		{
			image: self.0 .0, viewType: self.0 .2, format, components: unsafe { ::std::mem::transmute_copy(cmap) },
			subresourceRange: VkImageSubresourceRange
			{
				aspectMask: subresource_range.aspect_mask.0,
				baseMipLevel: subresource_range.mip_levels.start, levelCount: subresource_range.mip_levels.len() as _,
				baseArrayLayer: subresource_range.array_layers.start, layerCount: subresource_range.array_layers.len() as _
			}, .. Default::default()
		};
		let mut h = unsafe { ::std::mem::zeroed() };
		unsafe { vkCreateImageView(self.0 .1.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }
			.into_result().map(|_| ImageView(h, self.clone()))
	}
}

/// Image Dimension by corresponding extent type
pub trait ImageSize
{
	fn dimension() -> VkImageType;
	fn expand(self) -> ::Extent3D;
}
impl ImageSize for ::Extent1D
{
	fn dimension() -> VkImageType { VK_IMAGE_TYPE_1D }
	fn expand(self) -> ::Extent3D { ::Extent3D(self.0, 1, 1) }
}
impl ImageSize for ::Extent2D
{
	fn dimension() -> VkImageType { VK_IMAGE_TYPE_2D }
	fn expand(self) -> ::Extent3D { ::Extent3D(self.0, self.1, 1) }
}
impl ImageSize for ::Extent3D
{
	fn dimension() -> VkImageType { VK_IMAGE_TYPE_3D }
	fn expand(self) -> ::Extent3D { self }
}

/// Layouts of image and image subresources
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ImageLayout
{
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
	TransferDestOpt = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL as _
}

/// Structure specifying a color component mapping
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentMapping(pub ComponentSwizzle, pub ComponentSwizzle, pub ComponentSwizzle, pub ComponentSwizzle);
/// Specify how a component is swizzled
#[repr(u32)] #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentSwizzle
{
	/// the component is set to the identity swizzle
	Identity = VK_COMPONENT_SWIZZLE_IDENTITY as _,
	/// the component is set to zero
	Zero = VK_COMPONENT_SWIZZLE_ZERO as _,
	/// the component is set to either 1 or 1.0, depending on whether
	/// the type of the image view format is integer of floating-pointer respectively
	One = VK_COMPONENT_SWIZZLE_ONE as _,
	/// the component is set to the value of the R component of the image
	R = VK_COMPONENT_SWIZZLE_R as _,
	/// the component is set to the value of the G component of the image
	G = VK_COMPONENT_SWIZZLE_G as _,
	/// the component is set to the value of the B component of the image
	B = VK_COMPONENT_SWIZZLE_B as _,
	/// the component is set to the value of the A component of the image
	A = VK_COMPONENT_SWIZZLE_A as _
}
impl Default for ComponentMapping { fn default() -> Self { Self::all(ComponentSwizzle::Identity) } }
impl ComponentMapping
{
	/// Set same value to all component
	pub fn all(s: ComponentSwizzle) -> Self { ComponentMapping(s, s, s, s) }
	/// Set 2 values with repeating
	pub fn set2(a: ComponentSwizzle, b: ComponentSwizzle) -> Self { ComponentMapping(a, b, a, b) }
}
/// Structure specifying a image subresource range
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageSubresourceRange
{
	aspect_mask: AspectMask, mip_levels: ::std::ops::Range<u32>, array_layers: ::std::ops::Range<u32>
}
/// Bitmask specifying which aspects of an image are included in a view
#[derive(Debug, Clone, PartialEq, Eq, Copy)] #[repr(C)]
pub struct AspectMask(pub VkImageAspectFlags);
impl AspectMask
{
	/// The color aspect
	pub const COLOR: Self = AspectMask(VK_IMAGE_ASPECT_COLOR_BIT);
	/// The depth aspect
	pub const DEPTH: Self = AspectMask(VK_IMAGE_ASPECT_DEPTH_BIT);
	/// The stencil aspect
	pub const STENCIL: Self = AspectMask(VK_IMAGE_ASPECT_STENCIL_BIT);
	/// The metadata aspect, used for sparse sparse resource operations
	pub const METADATA: Self = AspectMask(VK_IMAGE_ASPECT_METADATA_BIT);

	/// The color aspect
	pub fn color(&self) -> Self { AspectMask(self.0 | Self::COLOR.0) }
	/// The depth aspect
	pub fn depth(&self) -> Self { AspectMask(self.0 | Self::DEPTH.0) }
	/// The stencil aspect
	pub fn stencil(&self) -> Self { AspectMask(self.0 | Self::STENCIL.0) }
	/// The metadata aspect, used for sparse sparse resource oeprations
	pub fn metadata(&self) -> Self { AspectMask(self.0 | Self::METADATA.0) }
}
