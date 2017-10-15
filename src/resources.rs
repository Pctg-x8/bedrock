//! Vulkan Resources

use vk::*;
use std::rc::Rc as RefCounter;
use std::ops::Deref;
use {VkHandle, DeviceChild};
#[cfg(feature = "FeImplements")] use VkResultHandler;
#[cfg(feature = "FeImplements")] use std::ptr::null;

struct DeviceMemoryCell(VkDeviceMemory, ::Device);
struct BufferCell(VkBuffer, ::Device);
#[cfg(feature = "VK_KHR_swapchain")]
pub enum ImageCell
{
	DeviceChild(VkImage, ::Device, VkImageType, VkFormat),
	SwapchainChild(VkImage, ::Swapchain, VkFormat)
}
#[cfg(not(feature = "VK_KHR_swapchain"))]
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

impl Deref for BufferView { type Target = Buffer; fn deref(&self) -> &Buffer { &self.1 } }
impl Deref for ImageView { type Target = Image; fn deref(&self) -> &Image { &self.1 } }

#[cfg(feature = "FeImplements")] DeviceChildCommonDrop! { for DeviceMemoryCell[vkFreeMemory], BufferCell[vkDestroyBuffer] }
#[cfg(feature = "FeImplements")] impl Drop for ImageCell
{
	fn drop(&mut self)
	{
		#[cfg(feature = "VK_KHR_swapchain")]
		match self
		{
			&mut ImageCell::DeviceChild(v, ref p, _, _) => unsafe { vkDestroyImage(p.native_ptr(), v, null()); },
			_ => (/* No destroying performed */)
		}
		#[cfg(not(feature = "VK_KHR_swapchain"))]
		unsafe { vkDestroyImage(self.1.native_ptr(), self.0, null()); }
	}
}
#[cfg(feature = "FeImplements")]
impl Drop for BufferView { fn drop(&mut self) { unsafe { vkDestroyBufferView(self.1.device().native_ptr(), self.0, null()) }; } }
#[cfg(feature = "FeImplements")]
impl Drop for ImageView  { fn drop(&mut self) { unsafe { vkDestroyImageView (self.1.device().native_ptr(), self.0, null()) }; } }

impl VkHandle for DeviceMemory { type Handle = VkDeviceMemory; fn native_ptr(&self) -> VkDeviceMemory { self.0 .0 } }
impl VkHandle for Buffer { type Handle = VkBuffer; fn native_ptr(&self) -> VkBuffer { self.0 .0 } }
impl VkHandle for BufferView { type Handle = VkBufferView; fn native_ptr(&self) -> VkBufferView { self.0 } }
impl VkHandle for ImageView  { type Handle = VkImageView;  fn native_ptr(&self) -> VkImageView  { self.0 } }
impl DeviceChild for DeviceMemory { fn device(&self) -> &::Device { &self.0 .1 } }
impl DeviceChild for Buffer { fn device(&self) -> &::Device { &self.0 .1 } }
impl DeviceChild for BufferView { fn device(&self) -> &::Device { self.1.device() } }
impl DeviceChild for ImageView  { fn device(&self) -> &::Device { self.1.device() } }

#[cfg(feature = "VK_KHR_swapchain")]
impl VkHandle for Image
{
	type Handle = VkImage;
	fn native_ptr(&self) -> VkImage
	{
		match self.0.deref()
		{
			&ImageCell::DeviceChild(v, _, _, _) => v,
			&ImageCell::SwapchainChild(v, _, _) => v
		}
	}
}
#[cfg(feature = "VK_KHR_swapchain")]
impl DeviceChild for Image
{
	fn device(&self) -> &::Device
	{
		match self.0.deref()
		{
			&ImageCell::DeviceChild(_, ref d, _, _) => d,
			&ImageCell::SwapchainChild(_, ref s, _) => s.device()
		}
	}
}
#[cfg(not(feature = "VK_KHR_swapchain"))]
impl VkHandle for Image { type Handle = VkImage; fn native_ptr(&self) -> VkImage { self.0 .0 } }
#[cfg(not(feature = "VK_KHR_swapchain"))]
impl DeviceChild for Image { fn device(&self) -> &::Device { &self.0 .1 } }

#[cfg(feature = "FeImplements")]
impl DeviceMemory
{
	/// Allocate GPU memory
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_TOO_MANY_OBJECTS`
	pub fn allocate(device: &::Device, size: usize, type_index: u32) -> ::Result<Self>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkAllocateMemory(device.native_ptr(), &VkMemoryAllocateInfo { allocationSize: size as _, memoryTypeIndex: type_index, .. Default::default() },
			::std::ptr::null(), &mut h) }.into_result().map(|_| DeviceMemory(RefCounter::new(DeviceMemoryCell(h, device.clone()))))
	}
	pub fn native_ptr(&self) -> VkDeviceMemory { self.0 .0 }
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
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	#[cfg(feature = "FeImplements")]
	pub fn create(&self, device: &::Device) -> ::Result<Buffer>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateBuffer(device.native_ptr(), &self.cinfo, ::std::ptr::null(), &mut h) }
			.into_result().map(|_| Buffer(RefCounter::new(BufferCell(h, device.clone()))))
	}
}

/// Bitmask specifying intended usage of an image
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageUsage(pub VkImageUsageFlags);
impl ImageUsage
{
	/// The image can be used as the source of a transfer command
	pub const TRANSFER_SRC: Self = ImageUsage(VK_IMAGE_USAGE_TRANSFER_SRC_BIT);
	/// The image can be used as the destination of a transfer command
	pub const TRANSFER_DEST: Self = ImageUsage(VK_IMAGE_USAGE_TRANSFER_DST_BIT);
	/// The image can be used to create `ImageView` suitable for occupying a `DescriptorSet` slot
	/// either of type `DescriptorType::SampledImage` or `DescriptorType::CombinedImageSampler`, and be sampled by a shader
	pub const SAMPLED: Self = ImageUsage(VK_IMAGE_USAGE_SAMPLED_BIT);
	/// The image can be used to create a `ImageView` suitable for occupying a `DescriptorSet` slot of type `DescriptorType::StorageImage`
	pub const STORAGE: Self = ImageUsage(VK_IMAGE_USAGE_STORAGE_BIT);
	/// The image can be used to create a `ImageView` suitable for use as a color or resolve attachment in a `Framebuffer`
	pub const COLOR_ATTACHMENT: Self = ImageUsage(VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT);
	/// The image can be used to create a `ImageView` suitable for use as a depth/stencil attachment in a `Framebuffer`
	pub const DEPTH_STENCIL_ATTACHMENT: Self = ImageUsage(VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT);
	/// The memory bound to this image will have been allocated with the `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`
	/// This bit can be set for any image that can be used to create a `ImageView` suitable for use as a color, resolve, depth/stencil,
	/// or input attachment
	pub const TRANSIENT_ATTACHMENT: Self = ImageUsage(VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT);
	/// The image can be used to create a `ImageView` suitable for occupying `DescriptorSet` slot of type `DescriptorType::InputAttachment`;
	/// be read from a shader as an input attachment; and be used as an input attachment in a framebuffer
	pub const INPUT_ATTACHMENT: Self = ImageUsage(VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT);

	/// The image can be used as the source of a transfer command
	pub fn transfer_src(&self) -> Self { ImageUsage(self.0 | Self::TRANSFER_SRC.0) }
	/// The image can be used as the destination of a transfer command
	pub fn transfer_dest(&self) -> Self { ImageUsage(self.0 | Self::TRANSFER_DEST.0) }
	/// The image can be used to create `ImageView` suitable for occupying a `DescriptorSet` slot
	/// either of type `DescriptorType::SampledImage` or `DescriptorType::CombinedImageSampler`, and be sampled by a shader
	pub fn sampled(&self) -> Self { ImageUsage(self.0 | Self::SAMPLED.0) }
	/// The image can be used to create a `ImageView` suitable for occupying a `DescriptorSet` slot of type `DescriptorType::StorageImage`
	pub fn storage(&self) -> Self { ImageUsage(self.0 | Self::STORAGE.0) }
	/// The image can be used to create a `ImageView` suitable for use as a color or resolve attachment in a `Framebuffer`
	pub fn color_attachment(&self) -> Self { ImageUsage(self.0 | Self::COLOR_ATTACHMENT.0) }
	/// The image can be used to create a `ImageView` suitable for use as a depth/stencil attachment in a `Framebuffer`
	pub fn depth_stencil_attachment(&self) -> Self { ImageUsage(self.0 | Self::DEPTH_STENCIL_ATTACHMENT.0) }
	/// The memory bound to this image will have been allocated with the `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`
	/// This bit can be set for any image that can be used to create a `ImageView` suitable for use as a color, resolve, depth/stencil,
	/// or input attachment
	pub fn transient_attachment(&self) -> Self { ImageUsage(self.0 | Self::TRANSIENT_ATTACHMENT.0) }
	/// The image can be used to create a `ImageView` suitable for occupying `DescriptorSet` slot of type `DescriptorType::InputAttachment`;
	/// be read from a shader as an input attachment; and be used as an input attachment in a framebuffer
	pub fn input_attachment(&self) -> Self { ImageUsage(self.0 | Self::INPUT_ATTACHMENT.0) }
}
/// Bitmask specifying additional parameters of an image
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageFlags(pub VkImageCreateFlags);
impl ImageFlags
{
	/// Empty bits
	pub const EMPTY: Self = ImageFlags(0);
	/// The image will be backed using sparse memory binding
	pub const SPARSE_BINDING: Self = ImageFlags(VK_IMAGE_CREATE_SPARSE_BINDING_BIT);
	/// The image can be partially backed using sparse memory binding. This bit is with `SPARSE_BINDING` implicitly
	pub const SPARSE_RESIDENCY: Self = ImageFlags(VK_IMAGE_CREATE_SPARSE_BINDING_BIT | VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT);
	/// The image will be backed using sparse memory binding with memory ranges
	/// that might also simultaneously be backing another image. This bit is with `SPARSE_BINDING` implicitly
	pub const SPARSE_ALIASED: Self = ImageFlags(VK_IMAGE_CREATE_SPARSE_BINDING_BIT | VK_IMAGE_CREATE_SPARSE_ALIASED_BIT);
	/// The image can be used to create a `ImageView` with a different format from the image
	pub const MUTABLE_FORMAT: Self = ImageFlags(VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT);
	/// The image can be used to create a `ImageView` of type `ImageViewType::Cube` or `ImageViewType::CubeArray`
	pub const CUBE_COMPATIBLE: Self = ImageFlags(VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT);

	/// The image will be backed using sparse memory binding
	pub fn sparse_binding(&self) -> Self { ImageFlags(self.0 | Self::SPARSE_BINDING.0) }
	/// The image can be partially backed using sparse memory binding. This bit is with `SPARSE_BINDING` implicitly
	pub fn sparse_residency(&self) -> Self { ImageFlags(self.0 | Self::SPARSE_RESIDENCY.0) }
	/// The image will be backed using sparse memory binding with memory ranges
	/// that might also simultaneously be backing another image. This bit is with `SPARSE_BINDING` implicitly
	pub fn sparse_aliased(&self) -> Self { ImageFlags(self.0 | Self::SPARSE_ALIASED.0) }
	/// The image can be used to create a `ImageView` with a different format from the image
	pub fn mutable_format(&self) -> Self { ImageFlags(self.0 | Self::MUTABLE_FORMAT.0) }
	/// The image can be used to create a `ImageView` of type `ImageViewType::Cube` or `ImageViewType::CubeArray`
	pub fn cube_compatible(&self) -> Self { ImageFlags(self.0 | Self::CUBE_COMPATIBLE.0) }
}
/// Builder structure specifying the parameters of a newly created image object
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageDesc { cinfo: VkImageCreateInfo, sharing_queues: Vec<u32> }
impl ImageDesc
{
	pub fn new<Size: ImageSize>(size: Size, format: VkFormat, usage: ImageUsage, initial_layout: ImageLayout) -> Self
	{
		ImageDesc
		{
			cinfo: VkImageCreateInfo
			{
				imageType: Size::dimension(), extent: size.into().as_ref().clone(), format, usage: usage.0,
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
	pub fn flags(&mut self, opt: ImageFlags) -> &mut Self
	{
		self.cinfo.flags = opt.0; self
	}
	pub fn array_layers(&mut self, layers: u32) -> &mut Self { self.cinfo.arrayLayers = layers; self }
}

#[cfg(feature = "FeImplements")]
impl ImageDesc
{
	#[cfg(not(feature = "VK_KHR_swapchain"))]
	pub fn create(&self, device: &::Device) -> ::Result<Image>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateImage(device.native_ptr(), &self.cinfo, ::std::ptr::null(), &mut h) }
			.into_result().map(|_| Image(RefCounter::new(ImageCell(h, device.clone(), self.cinfo.imageType, self.cinfo.format))))
	}
	#[cfg(feature = "VK_KHR_swapchain")]
	pub fn create(&self, device: &::Device) -> ::Result<Image>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateImage(device.native_ptr(), &self.cinfo, ::std::ptr::null(), &mut h) }
			.into_result().map(|_| Image(RefCounter::new(ImageCell::DeviceChild(h, device.clone(), self.cinfo.imageType, self.cinfo.format))))
	}
}

#[cfg(feature = "FeImplements")]
impl Buffer
{
	pub fn create_view(&self, format: VkFormat, range: ::std::ops::Range<u64>) -> ::Result<BufferView>
	{
		let cinfo = VkBufferViewCreateInfo
		{
			buffer: self.native_ptr(), format, offset: range.start, range: range.end - range.start, .. Default::default()
		};
		let mut h = unsafe { ::std::mem::zeroed() };
		unsafe { vkCreateBufferView(self.device().native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }
			.into_result().map(|_| BufferView(h, self.clone()))
	}
}
#[cfg(feature = "FeImplements")]
impl Image
{
	#[cfg(feature = "VK_KHR_swapchain")]
	fn format(&self) -> VkFormat
	{
		match self.0.deref()
		{
			&ImageCell::DeviceChild(_, _, f, _) => f,
			&ImageCell::SwapchainChild(_, _, f) => f
		}
	}
	#[cfg(not(feature = "VK_KHR_swapchain"))]
	fn format(&self) -> VkFormat { self.0 .3 }
	#[cfg(feature = "VK_KHR_swapchain")]
	fn dimension(&self) -> VkImageViewType
	{
		match self.0.deref()
		{
			&ImageCell::DeviceChild(_, _, _, d) => match d
			{
				VK_IMAGE_TYPE_1D => VK_IMAGE_VIEW_TYPE_1D,
				VK_IMAGE_TYPE_2D => VK_IMAGE_VIEW_TYPE_2D,
				VK_IMAGE_TYPE_3D => VK_IMAGE_VIEW_TYPE_3D,
				_ => unreachable!()
			},
			&ImageCell::SwapchainChild(_, _, _) => VK_IMAGE_VIEW_TYPE_2D
		}
	}
	#[cfg(not(feature = "VK_KHR_swapchain"))]
	fn dimension(&self) -> VkImageViewType
	{
		match self.0 .2
		{
			VK_IMAGE_TYPE_1D => VK_IMAGE_VIEW_TYPE_1D,
			VK_IMAGE_TYPE_2D => VK_IMAGE_VIEW_TYPE_2D,
			VK_IMAGE_TYPE_3D => VK_IMAGE_VIEW_TYPE_3D,
			_ => unreachable!()
		}
	}
	pub fn create_view(&self, format: Option<VkFormat>, vtype: Option<VkImageViewType>, cmap: &ComponentMapping, subresource_range: &ImageSubresourceRange)
		-> ::Result<ImageView>
	{
		let format = format.unwrap_or(self.format());
		let vtype = vtype.unwrap_or(self.dimension());
		let cinfo = VkImageViewCreateInfo
		{
			image: self.native_ptr(), viewType: vtype, format, components: unsafe { ::std::mem::transmute_copy(cmap) },
			subresourceRange: VkImageSubresourceRange
			{
				aspectMask: subresource_range.aspect_mask.0,
				baseMipLevel: subresource_range.mip_levels.start, levelCount: subresource_range.mip_levels.len() as _,
				baseArrayLayer: subresource_range.array_layers.start, layerCount: subresource_range.array_layers.len() as _
			}, .. Default::default()
		};
		let mut h = unsafe { ::std::mem::zeroed() };
		unsafe { vkCreateImageView(self.device().native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }
			.into_result().map(|_| ImageView(h, self.clone()))
	}
	/// Retrieve information about an image subresource
	/// Subresource: (aspect, mipLevel, arrayLayer)
	pub fn image_subresource_layout(&self, subres_aspect: AspectMask, subres_mip_level: u32, subres_array_layer: u32) -> VkSubresourceLayout
	{
		let mut s = unsafe { ::std::mem::uninitialized() };
		let subres = VkImageSubresource { aspectMask: subres_aspect.0, mipLevel: subres_mip_level, arrayLayer: subres_array_layer };
		unsafe { vkGetImageSubresourceLayout(self.device().native_ptr(), self.native_ptr(), &subres, &mut s) }; s
	}
}

#[cfg(feature = "FeImplements")]
impl DeviceMemory
{
	/// Map a memory object into application address space
	/// # Failure
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_MEMORY_MAP_FAILED`
	pub fn map(&self, range: ::std::ops::Range<usize>) -> ::Result<MappedMemoryRange>
	{
		let mut p = ::std::ptr::null_mut();
		unsafe { vkMapMemory(self.device().native_ptr(), self.native_ptr(), range.start as _, (range.end - range.start) as _,
			0, &mut p) }.into_result().map(|_| MappedMemoryRange(self, p as *mut _, range.start as _ .. range.end as _))
	}
	/// Unmap a previously mapped memory object
	/// # Safety
	/// Caller must guarantee that there is no `MappedMemoryRange` alives.  
	/// Accessing the mapped memory after this call has undefined behavior
	pub unsafe fn unmap(&self)
	{
		vkUnmapMemory(self.0 .1.native_ptr(), self.native_ptr());
	}
	/// Query the current commitment for a `DeviceMemory`
	pub fn commitment_bytes(&self) -> VkDeviceSize
	{
		let mut b = 0;
		unsafe { vkGetDeviceMemoryCommitment(self.device().native_ptr(), self.native_ptr(), &mut b) }; b
	}
}

/// Common operations for memory bound objects
pub trait MemoryBound
{
	/// Returns the memory requirements for specified Vulkan object
	fn requirements(&self) -> VkMemoryRequirements;
	/// Bind device memory to the object
	/// # Failure
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	fn bind(&self, memory: &DeviceMemory, offset: usize) -> ::Result<()>;
}
#[cfg(feature = "FeImplements")]
impl MemoryBound for Buffer
{
	fn requirements(&self) -> VkMemoryRequirements
	{
		let mut p = unsafe { ::std::mem::uninitialized() };
		unsafe { vkGetBufferMemoryRequirements(self.device().native_ptr(), self.native_ptr(), &mut p) }; p
	}
	fn bind(&self, memory: &DeviceMemory, offset: usize) -> ::Result<()>
	{
		unsafe { vkBindBufferMemory(self.device().native_ptr(), self.native_ptr(), memory.native_ptr(), offset as _) }.into_result()
	}
}
#[cfg(feature = "FeImplements")]
impl MemoryBound for Image
{
	fn requirements(&self) -> VkMemoryRequirements
	{
		let mut p = unsafe { ::std::mem::uninitialized() };
		unsafe { vkGetImageMemoryRequirements(self.device().native_ptr(), self.native_ptr(), &mut p) }; p
	}
	fn bind(&self, memory: &DeviceMemory, offset: usize) -> ::Result<()>
	{
		unsafe { vkBindImageMemory(self.device().native_ptr(), self.native_ptr(), memory.native_ptr(), offset as _) }.into_result()
	}
}
#[cfg(feature = "FeImplements")]
impl Image
{
	/// Query the memory requirements for a sparse image
	pub fn sparse_requirements(&self) -> Vec<VkSparseImageMemoryRequirements>
	{
		let mut n = 0;
		unsafe { vkGetImageSparseMemoryRequirements(self.device().native_ptr(), self.native_ptr(), &mut n, ::std::ptr::null_mut()) };
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { vkGetImageSparseMemoryRequirements(self.device().native_ptr(), self.native_ptr(), &mut n, v.as_mut_ptr()) };
		v
	}
}

/// Image Dimension by corresponding extent type
pub trait ImageSize : Into<::Extent3D>
{
	fn dimension() -> VkImageType;
}
impl ImageSize for ::Extent1D
{
	fn dimension() -> VkImageType { VK_IMAGE_TYPE_1D }
}
impl ImageSize for ::Extent2D
{
	fn dimension() -> VkImageType { VK_IMAGE_TYPE_2D }
}
impl ImageSize for ::Extent3D
{
	fn dimension() -> VkImageType { VK_IMAGE_TYPE_3D }
}

/// Specifies the block of mapped memory in a `DeviceMemory`
pub struct MappedMemoryRange<'m>(&'m DeviceMemory, *mut u8, ::std::ops::Range<VkDeviceSize>);
impl<'m> MappedMemoryRange<'m>
{
	/// Get a reference in mapped memory with byte offsets
	/// # Safety
	/// Caller must guarantee that the pointer and its alignment are valid
	pub unsafe fn get<T>(&self, offset: usize) -> &T
	{
		::std::mem::transmute(self.1.offset(offset as _))
	}
	/// Get a mutable reference in mapped memory with byte offsets
	/// # Safety
	/// Caller must guarantee that the pointer and its alignment are valid
	pub unsafe fn get_mut<T>(&self, offset: usize) -> &mut T
	{
		::std::mem::transmute(self.1.offset(offset as _))
	}
	/// Flushes the memory range manually. Returns a structure for flush operation
	pub fn manual_flush(self) -> VkMappedMemoryRange
	{
		let (m, r) = (self.0 .0 .0, self.2.clone()); ::std::mem::forget(self);
		VkMappedMemoryRange
		{
			offset: r.start, size: r.end - r.start, memory: m, .. Default::default()
		}
	}
}
#[cfg(feature = "FeImplements")]
impl<'m> Drop for MappedMemoryRange<'m>
{
	fn drop(&mut self)
	{
		unsafe { vkFlushMappedMemoryRanges(self.0 .0 .1.native_ptr(), 1, &::std::mem::replace(self, ::std::mem::uninitialized()).manual_flush()) }
			.into_result().unwrap();
	}
}

#[cfg(all(feature = "FeImplements", feature = "VK_KHR_swapchain"))]
impl ::Swapchain
{
	/// Obtain the array of presentable images associated with a swapchain
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn get_images(&self) -> ::Result<Vec<Image>>
	{
		let mut n = 0;
		unsafe { vkGetSwapchainImagesKHR(self.device().native_ptr(), self.native_ptr(), &mut n, ::std::ptr::null_mut()) }.into_result()?;
		let mut v = Vec::with_capacity(n as _); unsafe { v.set_len(n as _) };
		unsafe { vkGetSwapchainImagesKHR(self.device().native_ptr(), self.native_ptr(), &mut n, v.as_mut_ptr()) }.into_result()
			.map(|_| v.into_iter().map(|r| Image(RefCounter::new(ImageCell::SwapchainChild(r, self.clone(), self.format())))).collect())
	}
}

/// Layouts of image and image subresources
#[repr(u32)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
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
	TransferDestOpt = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL as _,
	/// must only be used for presenting a swapchain image for display
	#[cfg(feature = "VK_KHR_swapchain")]
	PresentSrc = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR as _
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
	pub aspect_mask: AspectMask, pub mip_levels: ::std::ops::Range<u32>, pub array_layers: ::std::ops::Range<u32>
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

/// Opaque handle to a sampler object
pub struct Sampler(VkSampler, ::Device);
#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{ for Sampler[vkDestroySampler] }

impl VkHandle for Sampler { type Handle = VkSampler; fn native_ptr(&self) -> VkSampler { self.0 } }
impl DeviceChild for Sampler { fn device(&self) -> &::Device { &self.1 } }

/// Specify behavior of sampling with texture coordinates outside an image
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum AddressingMode
{
    /// The repeat wrap mode
    Repeat = VK_SAMPLER_ADDRESS_MODE_REPEAT as _,
    /// The mirrored repeat wrap mode
    MirroredRepeat = VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT as _,
    /// The clamp to edge wrap mode
    ClampToEdge = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE as _,
    /// The clamp to border wrap mode
    ClampToBorder = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER as _,
    /// The mirror clamp to edge wrap mode
    #[cfg(feature = "VK_KHR_mirror_clamp_to_edge")]
    MirrorClampToEdge = VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE as _
}
/// Specify filter used for texture lookups
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum FilterMode
{
    /// Nearest filtering
    Nearest = VK_FILTER_NEAREST as _,
    /// Linear filtering
    Linear = VK_FILTER_LINEAR as _
}
/// Specify mipmap mode used for texture lookups
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MipmapFilterMode
{
    /// Nearest filtering
    Nearest = VK_SAMPLER_MIPMAP_MODE_NEAREST as _,
    /// Linear filtering
    Linear = VK_SAMPLER_MIPMAP_MODE_LINEAR as _
}
/// Specify border color used for texture lookups
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum BorderColor
{
    /// A transparent, floating-point format, black color
    TransparentBlackF = VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK as _,
    /// A transparent, integer format, black color
    TransparentBlackI = VK_BORDER_COLOR_INT_TRANSPARENT_BLACK as _,
    /// An opaque, floating-point format, black color
    OpaqueBlackF = VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK as _,
    /// An opaque, integer format, black color
    OpaqueBlackI = VK_BORDER_COLOR_INT_OPAQUE_BLACK as _,
    /// An opaque, floating-point format, white color
    OpaqueWhiteF = VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE as _,
    /// An opaque, integer format, white color
    OpaqueWhiteI = VK_BORDER_COLOR_INT_OPAQUE_WHITE as _
}
/// Builder object for constructing the sampler object
pub struct SamplerBuilder(VkSamplerCreateInfo);
impl SamplerBuilder
{
    pub fn new() -> Self
    {
        SamplerBuilder(VkSamplerCreateInfo
        {
            magFilter: FilterMode::Linear as _, minFilter: FilterMode::Linear as _, mipmapMode: MipmapFilterMode::Linear as _,
            addressModeU: AddressingMode::Repeat as _, addressModeV: AddressingMode::Repeat as _, addressModeW: AddressingMode::Repeat as _,
            mipLodBias: 0.0, anisotropyEnable: false as _, compareEnable: false as _, compareOp: ::CompareOp::Always as _,
            minLod: 0.0, maxLod: 0.0, borderColor: BorderColor::TransparentBlackF as _, unnormalizedCoordinates: false as _,
            .. Default::default()
        })
    }
    /// The magnification and the minification filters to apply to lookups.  
    /// Default: Magnification=`FilterMode::Linear`, Minification=`FilterMode::Linear`
    pub fn filter(&mut self, mag: FilterMode, min: FilterMode) -> &mut Self
    {
        self.0.magFilter = mag as _; self.0.minFilter = min as _; self
    }
    /// The mipmap filter to apply to lookups.  
	/// Default: `MipmapFilterMode::Linear`
    pub fn mip_filter(&mut self, f: MipmapFilterMode) -> &mut Self
    {
        self.0.mipmapMode = f as _; self
    }
    /// The addressing mode for outside [0..1] range for U, V and W coordinates.  
    /// Default: U=`AddressingMode::Repeat`, V=`AddressinMode::Repeat`, W=`AddressingMode::Repeat`
    pub fn addressing(&mut self, u: AddressingMode, v: AddressingMode, w: AddressingMode) -> &mut Self
    {
        self.0.addressModeU = u as _; self.0.addressModeV = v as _; self.0.addressModeW = w as _; self
    }
    /// The bias to be added to mipmap LOD calculation and bias provided by image sampling functions in SPIR-V,
    /// as described in the `Level-of-Detail Operation` section in Vulkan Specification.  
    /// Default: 0.0
    pub fn lod_bias(&mut self, bias: f32) -> &mut Self { self.0.mipLodBias = bias; self }
    /// The anisotropy value clamp. Specifying `None` switches off the anisotropic filtering  
    /// Default: `None`
    pub fn max_anisotropy(&mut self, level: Option<f32>) -> &mut Self
    {
        self.0.anisotropyEnable = level.is_some() as _;
        self.0.maxAnisotropy = level.unwrap_or_default(); self
    }
    /// The comparison function to apply to fetched data before filtering
    /// as described in the `Depth Compare Operation` section in Vulkan Specification.
    /// Specifying `None` switches off the comparison against a reference value during lookups.  
    /// Default: `None`
    pub fn comparison(&mut self, op: Option<::CompareOp>) -> &mut Self
    {
        self.0.compareEnable = op.is_some() as _;
        self.0.compareOp = op.unwrap_or(::CompareOp::Always) as _; self
    }
    /// The values used to clamp the computed level-of-detail value,
    /// as described in the `Level-of-Detail Operation` section in Vulkan Specification.  
    /// Default: min_lod=0.0, max_lod=0.0
    /// # Panics
    /// `max_lod` must be greater than or equal to `min_lod`
    pub fn lod_clamp(&mut self, min_lod: f32, max_lod: f32) -> &mut Self
    {
        assert!(max_lod >= min_lod);
        self.0.minLod = min_lod; self.0.maxLod = max_lod; self
    }
    /// Whether to use unnormalized or normalized texel coordinates to address texels of the image.  
    /// Default: `false`
    /// # Safety
    /// User must meet the constraints as described in the "Valid Usage" section in the `VkSamplerCreateInfo` manual page
    pub unsafe fn unnormalized_coordinates(&mut self, use_unnormalized: bool) -> &mut Self
    {
        self.0.unnormalizedCoordinates = use_unnormalized as _; self
    }

    /// Create a new sampler object
    /// # Failures
    /// On failure, this command returns
	/// 
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    #[cfg(feature = "FeImplements")]
    pub fn create(&self, device: &::Device) -> ::Result<Sampler>
    {
        let mut h = VK_NULL_HANDLE as _;
        unsafe { vkCreateSampler(device.native_ptr(), &self.0, ::std::ptr::null(), &mut h) }
            .into_result().map(|_| Sampler(h, device.clone()))
    }
}
