//! Vulkan Resources

use vk::*;
use std::rc::Rc as RefCounter;

struct DeviceMemoryCell(VkDeviceMemory, ::Device);
/// Opaque handle to a device memory object
pub struct DeviceMemory(RefCounter<DeviceMemoryCell>);
struct BufferCell(VkBuffer, ::Device);
/// Opaque handle to a buffer object
#[derive(Clone)] pub struct Buffer(RefCounter<BufferCell>);
/// Opaque handle to a buffer view object
pub struct BufferView(VkBufferView, Buffer);
struct ImageCell(VkImage, ::Device, VkImageType);
/// Opaque handle to a image object(constructed via `ImageDesc`)
#[derive(Clone)] pub struct Image(RefCounter<ImageCell>);
/// Opaque handle to a image view object
pub struct ImageView(VkImageView, Image);

#[cfg(feature = "FeImplements")]
impl Drop for DeviceMemoryCell { fn drop(&mut self) { unsafe { vkFreeMemory(self.1.native_ptr(), self.0, ::std::ptr::null()) }; } }
#[cfg(feature = "FeImplements")]
impl Drop for BufferCell { fn drop(&mut self) { unsafe { vkDestroyBuffer(self.1.native_ptr(), self.0, ::std::ptr::null()) }; } }
#[cfg(feature = "FeImplements")]
impl Drop for ImageCell { fn drop(&mut self) { unsafe { vkDestroyImage(self.1.native_ptr(), self.0, ::std::ptr::null()) }; } }
#[cfg(feature = "FeImplements")]
impl Drop for BufferView { fn drop(&mut self) { unsafe { vkDestroyBufferView(self.1 .0 .1.native_ptr(), self.0, std::ptr::null()) }; } }
#[cfg(feature = "FeImplements")]
impl Drop for ImageView { fn drop(&mut self) { unsafe { vkDestroyImageView(self.1 .0 .1.native_ptr(), self.0, std::ptr::null()) }; } }

impl ::DeviceChild<VkDeviceMemory> for DeviceMemory
{
	unsafe fn from_unchecked(p: VkDeviceMemory, parent: &::Device) -> Self
	{
		DeviceMemory(RefCounter::new(DeviceMemoryCell(p, parent.clone())))
	}
}
impl ::DeviceChild<VkBuffer> for Buffer
{
	unsafe fn from_unchecked(p: VkBuffer, parent: &::Device) -> Self
	{
		Buffer(RefCounter::new(BufferCell(p, parent.clone())))
	}
}
impl ::DeviceChild<VkImage> for Image
{
	unsafe fn from_unchecked(p: VkImage, parent: &::Device) -> Self
	{
		Image(RefCounter::new(ImageCell(p, parent.clone())))
	}
}

/// Bitmask specifying allowed usage of a buffer
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BufferUsage(pub VkBufferUsageFlags);
impl BufferUsage
{
	/// Empty bits
	pub const EMPTY: Self = BufferUsage(0);
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
				imageType: Size::dimension(), extent: size.expand(), format,
				mipLevels: 1, arrayLayers:1, samples: 1, initialLayout: initial_layout as _,
				.. Default::default()
			},
			sharing_queues: Vec::new()
		}
	}
	pub fn sharing_queue_families(mut self, indices: Vec<u32>) -> Self
	{
		self.sharing_queues = indices; self
	}
	/// bitmask of 1, 2, 4, 8, 16, 32, 64
	pub fn sample_counts(mut self, count_bits: u32) -> Self
	{
		self.cinfo.samples = count_bits; self
	}
	pub fn use_linear_tiling(mut self) -> Self
	{
		self.cinfo.tiling = VK_IMAGE_TILING_LINEAR; self
	}
	#[cfg(features = "FeImplements")]
	pub fn create(mut self, device: &::Device) -> ::Result<Image>
	{
		self.cinfo.sharingMode = if self.sharing_queues.is_empty() { VK_SHARING_MODE_EXCLUSIVE } else { VK_SHARING_MODE_CONCURRENT };
		self.cinfo.queueFamilyIndexCount = self.sharing_queues.len() as _;
		self.cinfo.pQueueFamilyIndices = self.sharing_queues().as_ptr();

		let mut h = unsafe { ::std::mem::zeroed() };
		unsafe { vkCreateImage(device.native_ptr(), &self.cinfo, ::std::ptr::null(), &mut h) }
			.into_result().map(|_| Image(RefCounter::new(ImageCell(h, device.clone(), self.cinfo.imageType))))
	}
}

/// Image Dimension by corresponding extent type
pub trait ImageSize
{
	fn dimension() -> VkImageType;
	fn expand(self) -> VkExtent3D;
}
impl ImageSize for u32
{
	fn dimension() -> VkImageType { VK_IMAGE_TYPE_1D }
	fn expand(self) -> VkExtent3D { VkExtent3D { width: self, height: 1, depth: 1 } }
}
impl ImageSize for VkExtent2D
{
	fn dimension() -> VkImageType { VK_IMAGE_TYPE_2D }
	fn expand(self) -> VkExtent3D { VkExtent3D { width: self.width, height: self.height, depth: 1 } }
}
impl ImageSize for VkExtent3D
{
	fn dimension() -> VkImageType { VK_IMAGE_TYPE_3D }
	fn expand(self) -> VkExtent3D { self }
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
