//! Vulkan RenderPass/Framebuffer

use vk::*;
use {VkHandle, DeviceChild};
#[cfg(feature = "FeImplements")] use VkResultHandler;

/// Opaque handle to a render pass object
pub struct RenderPass(VkRenderPass, ::Device);
/// Opaque handle to a framebuffer object
pub struct Framebuffer(VkFramebuffer, ::Device);

#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{
	for RenderPass[vkDestroyRenderPass], Framebuffer[vkDestroyFramebuffer]
}
impl VkHandle for RenderPass { type Handle = VkRenderPass; fn native_ptr(&self) -> VkRenderPass { self.0 } }
impl VkHandle for Framebuffer { type Handle = VkFramebuffer; fn native_ptr(&self) -> VkFramebuffer { self.0 } }
impl DeviceChild for RenderPass { fn device(&self) -> &::Device { &self.1 } }
impl DeviceChild for Framebuffer { fn device(&self) -> &::Device { &self.1 } }

/// Builder structure to construct the `VkSubpassDescription`
pub struct SubpassDescription
{
	input_attachments: Vec<VkAttachmentReference>, color_attachments: Vec<VkAttachmentReference>,
	resolve_attachments: Vec<VkAttachmentReference>, depth_stencil_attachment: Option<VkAttachmentReference>,
	preserve_attachments: Vec<u32>
}
/// Builder structure to construct the `RenderPass`
pub struct RenderPassBuilder
{
	attachments: Vec<VkAttachmentDescription>, subpasses: Vec<SubpassDescription>,
	dependencies: Vec<VkSubpassDependency>
}
impl RenderPassBuilder
{
	pub fn new() -> Self { RenderPassBuilder { attachments: Vec::new(), subpasses: Vec::new(), dependencies: Vec::new() } }
	pub fn add_attachment(&mut self, desc: VkAttachmentDescription) -> &mut Self { self.attachments.push(desc); self }
	pub fn add_subpass(&mut self, desc: SubpassDescription) -> &mut Self { self.subpasses.push(desc); self }
	pub fn add_dependency(&mut self, desc: VkSubpassDependency) -> &mut Self { self.dependencies.push(desc); self }
	pub fn add_attachments<Collection: IntoIterator<Item = VkAttachmentDescription>>(&mut self, collection: Collection) -> &mut Self
	{
		for d in collection { self.add_attachment(d); } self
	}
	pub fn add_subpasses<Collection: IntoIterator<Item = SubpassDescription>>(&mut self, collection: Collection) -> &mut Self
	{
		for d in collection { self.add_subpass(d); } self
	}
	pub fn add_dependencies<Collection: IntoIterator<Item = VkSubpassDependency>>(&mut self, collection: Collection) -> &mut Self
	{
		for d in collection { self.add_dependency(d); } self
	}
}
impl SubpassDescription
{
	pub fn new() -> Self
	{
		SubpassDescription
		{
			input_attachments: Vec::new(), color_attachments: Vec::new(), resolve_attachments: Vec::new(), depth_stencil_attachment: None,
			preserve_attachments: Vec::new()
		}
	}
	pub fn add_input(&mut self, index: u32, layout: ::ImageLayout) -> &mut Self
	{
		self.input_attachments.push(VkAttachmentReference { attachment: index, layout: layout as _ }); self
	}
	pub fn add_color_output(&mut self, index: u32, layout: ::ImageLayout, resolve: Option<(u32, ::ImageLayout)>) -> &mut Self
	{
		if let Some((i, l)) = resolve
		{
			while self.resolve_attachments.len() < self.color_attachments.len()
			{
				self.resolve_attachments.push(VkAttachmentReference { attachment: VK_ATTACHMENT_UNUSED, layout: 0 as _ });
			}
			self.resolve_attachments.push(VkAttachmentReference { attachment: i, layout: l as _ });
		}
		self.color_attachments.push(VkAttachmentReference { attachment: index, layout: layout as _ });
		self
	}
	pub fn depth_stencil(&mut self, index: u32, layout: ::ImageLayout) -> &mut Self
	{
		self.depth_stencil_attachment = Some(VkAttachmentReference { attachment: index, layout: layout as _ }); self
	}
	pub fn add_preserve(&mut self, index: u32) -> &mut Self
	{
		self.preserve_attachments.push(index); self
	}
	pub fn add_preserves<Collection: IntoIterator<Item = u32>>(&mut self, collection: Collection) -> &mut Self
	{
		for i in collection { self.add_preserve(i); } self
	}
}
#[cfg(feature = "FeImplements")]
impl RenderPassBuilder
{
	/// Create a new render pass object
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn create(&self, device: &::Device) -> ::Result<RenderPass>
	{
		let subpasses = self.subpasses.iter().map(|x| VkSubpassDescription
		{
			inputAttachmentCount: x.input_attachments.len() as _, pInputAttachments: x.input_attachments.as_ptr(),
			colorAttachmentCount: x.color_attachments.len() as _, pColorAttachments: x.color_attachments.as_ptr(),
			pResolveAttachments: if x.resolve_attachments.is_empty() { ::std::ptr::null() } else { x.resolve_attachments.as_ptr() },
			pDepthStencilAttachment: if let Some(ref x) = x.depth_stencil_attachment { x } else { ::std::ptr::null() },
			preserveAttachmentCount: x.preserve_attachments.len() as _, pPreserveAttachments: x.preserve_attachments.as_ptr(),
			.. Default::default()
		}).collect::<Vec<_>>();
		let cinfo = VkRenderPassCreateInfo
		{
			attachmentCount: self.attachments.len() as _, pAttachments: self.attachments.as_ptr(),
			subpassCount: subpasses.len() as _, pSubpasses: subpasses.as_ptr(),
			dependencyCount: self.dependencies.len() as _, pDependencies: self.dependencies.as_ptr(),
			.. Default::default()
		};
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateRenderPass(device.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }.into_result()
			.map(|_| RenderPass(h, device.clone()))
	}
}
#[cfg(feature = "FeImplements")]
impl Framebuffer
{
	/// Create a new framebuffer object
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn new(mold: &RenderPass, attachment_objects: &[&::ImageView], size: ::Extent2D, layers: u32) -> ::Result<Self>
	{
		let views = attachment_objects.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();
		let cinfo = VkFramebufferCreateInfo
		{
			renderPass: mold.0, attachmentCount: views.len() as _, pAttachments: views.as_ptr(),
			width: size.0, height: size.1, layers, .. Default::default()
		};
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateFramebuffer(mold.1.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }.into_result()
			.map(|_| Framebuffer(h, mold.1.clone()))
	}
}

#[cfg(feature = "FeImplements")]
impl RenderPass
{
	/// Returns the granularity for optimal render area
	pub fn optimal_granularity(&self) -> ::Extent2D
	{
		let mut e = ::Extent2D(0, 0);
		unsafe { vkGetRenderAreaGranularity(self.1.native_ptr(), self.0, ::std::mem::transmute(&mut e)) }; e
	}
}
