//! Vulkan Commands

use vk::*;
use {VkHandle, DeviceChild};
#[cfg(feature = "FeImplements")] use VkResultHandler;

/// Opaque handle to a command pool object
#[derive(Clone)] pub struct CommandPool(VkCommandPool, ::Device);
/// Opaque handle to a command buffer object
#[repr(C)] #[derive(Clone, Copy)] pub struct CommandBuffer(VkCommandBuffer);

#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{ for CommandPool[vkDestroyCommandPool] }
impl VkHandle for CommandPool   { type Handle = VkCommandPool;   fn native_ptr(&self) -> VkCommandPool   { self.0 } }
impl VkHandle for CommandBuffer { type Handle = VkCommandBuffer; fn native_ptr(&self) -> VkCommandBuffer { self.0 } }
impl DeviceChild for CommandPool { fn device(&self) -> &::Device { &self.1 } }

/// The recording state of commandbuffers
#[cfg(feature = "FeImplements")]
pub struct CmdRecord<'d> { ptr: &'d CommandBuffer, layout: [Option<&'d ::PipelineLayout>; 2] }

/// Implicitly closing the recording state. This may cause a panic when there are errors in commands
#[cfg(feature = "FeImplements")]
impl<'d> Drop for CmdRecord<'d>
{
	fn drop(&mut self)
	{
		unsafe { vkEndCommandBuffer(self.ptr.native_ptr()) }.into_result().expect("Error closing command recording state");
	}
}

#[cfg(feature = "FeImplements")]
impl CommandPool
{
	/// Create a new command pool object
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn new(device: &::Device, queue_family: u32, transient: bool, indiv_resettable: bool) -> ::Result<Self>
	{
		let cinfo = VkCommandPoolCreateInfo
		{
			queueFamilyIndex: queue_family, flags: if transient { VK_COMMAND_POOL_CREATE_TRANSIENT_BIT } else { 0 }
				| if indiv_resettable { VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT } else { 0 },
			.. Default::default()
		};
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateCommandPool(device.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }.into_result().map(|_| CommandPool(h, device.clone()))
	}
	/// Allocate command buffers from an existing command pool
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn alloc(&self, count: u32, primary: bool) -> ::Result<Vec<CommandBuffer>>
	{
		let ainfo = VkCommandBufferAllocateInfo
		{
			commandBufferCount: count, level: if primary { VK_COMMAND_BUFFER_LEVEL_PRIMARY } else { VK_COMMAND_BUFFER_LEVEL_SECONDARY },
			commandPool: self.0, .. Default::default()
		};
		let mut hs = vec![VK_NULL_HANDLE as _; count as _];
		unsafe { vkAllocateCommandBuffers(self.1.native_ptr(), &ainfo, hs.as_mut_ptr()) }.into_result()
			.map(|_| unsafe { ::std::mem::transmute(hs) })
	}
    /// Resets a command pool
    /// # Safety
    /// Application cannot use command buffers after this call
    /// # Failures
    /// On failure, this command returns
	///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn reset(&self, release_resources: bool) -> ::Result<()>
	{
		unsafe { vkResetCommandPool(self.1.native_ptr(), self.0, if release_resources { VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT } else { 0 }) }
			.into_result()
	}
	/// Free command buffers
	pub fn free(&self, buffers: &[CommandBuffer])
	{
		unsafe { vkFreeCommandBuffers(self.1.native_ptr(), self.0, buffers.len() as _, buffers.as_ptr() as *const _) };
	}
}

#[cfg(feature = "FeImplements")]
impl CommandBuffer
{
	/// Start recording a primary command buffer
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn begin(&self) -> ::Result<CmdRecord>
	{
		unsafe { vkBeginCommandBuffer(self.0, &Default::default()) }.into_result().map(|_| CmdRecord { ptr: self, layout: [None, None] })
	}
	/// Start recording a secondary command buffer
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn begin_inherit(&self, renderpass: Option<(&::Framebuffer, &::RenderPass, u32)>, query: Option<(OcclusionQuery, ::QueryPipelineStatisticFlags)>)
		-> ::Result<CmdRecord>
	{
		let (fb, rp, s) = renderpass.map(|(f, r, s)| (f.native_ptr(), r.native_ptr(), s)).unwrap_or((VK_NULL_HANDLE as _, VK_NULL_HANDLE as _, 0));
		let (oq, psq) = query.map(|(o, p)| (o, p.0)).unwrap_or((OcclusionQuery::Disable, 0));
		let inherit = VkCommandBufferInheritanceInfo
		{
			framebuffer: fb, renderPass: rp, subpass: s, occlusionQueryEnable: (oq != OcclusionQuery::Disable) as _,
			queryFlags: if oq == OcclusionQuery::Precise { VK_QUERY_CONTROL_PRECISE_BIT } else { 0 }, pipelineStatistics: psq,
			.. Default::default()
		};
		let binfo = VkCommandBufferBeginInfo { pInheritanceInfo: &inherit, .. Default::default() };
		unsafe { vkBeginCommandBuffer(self.0, &binfo) }.into_result().map(|_| CmdRecord { ptr: self, layout: [None, None] })
	}
}

/// Graphics/Compute Commands: Pipeline Setup
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Bind a pipeline object to a command buffer
	pub fn bind_graphics_pipeline(&mut self, pipeline: &::Pipeline, layout: &'d ::PipelineLayout) -> &mut Self
	{
		unsafe { vkCmdBindPipeline(self.ptr.native_ptr(), VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline.native_ptr()) };
		self.layout[VK_PIPELINE_BIND_POINT_GRAPHICS as usize] = Some(layout);
		self
	}
	/// Bind a pipeline object to a command buffer
	pub fn bind_compute_pipeline(&mut self, pipeline: &::Pipeline, layout: &'d ::PipelineLayout) -> &mut Self
	{
		unsafe { vkCmdBindPipeline(self.ptr.native_ptr(), VK_PIPELINE_BIND_POINT_COMPUTE, pipeline.native_ptr()) };
		self.layout[VK_PIPELINE_BIND_POINT_COMPUTE as usize] = Some(layout);
		self
	}
	/// Binds descriptor sets to a command buffer
	pub fn bind_graphics_descriptor_sets(&mut self, first: u32, descriptor_sets: &[VkDescriptorSet], dynamic_offsets: &[u32]) -> &mut Self
	{
		unsafe { vkCmdBindDescriptorSets(self.ptr.native_ptr(), VK_PIPELINE_BIND_POINT_GRAPHICS,
			self.layout[VK_PIPELINE_BIND_POINT_GRAPHICS as usize].expect("Pipeline is not bound for graphics pipeline").native_ptr(),
			first, descriptor_sets.len() as _, descriptor_sets.as_ptr(), dynamic_offsets.len() as _, dynamic_offsets.as_ptr()) };
		self
	}
	/// Binds descriptor sets to a command buffer
	pub fn bind_compute_descriptor_sets(&mut self, first: u32, descriptor_sets:&[VkDescriptorSet], dynamic_offsets: &[u32]) -> &mut Self
	{
		unsafe { vkCmdBindDescriptorSets(self.ptr.native_ptr(), VK_PIPELINE_BIND_POINT_COMPUTE,
			self.layout[VK_PIPELINE_BIND_POINT_COMPUTE as usize].expect("Pipeline is not bound for compute pipeline").native_ptr(),
			first, descriptor_sets.len() as _, descriptor_sets.as_ptr(), dynamic_offsets.len() as _, dynamic_offsets.as_ptr()) };
		self
	}
	/// Update the values of push constants
	pub fn push_graphics_constants<T>(&mut self, stage: ::ShaderStage, offset: u32, values: &[T]) -> &mut Self
	{
		unsafe { vkCmdPushConstants(self.ptr.native_ptr(),
			self.layout[VK_PIPELINE_BIND_POINT_GRAPHICS as usize].expect("Pipeline is not bound for graphics pipeline").native_ptr(),
			stage.0, offset, values.len() as _, values.as_ptr() as *const _) };
		self
	}
	/// Update the values of push constants
	pub fn push_compute_constants<T>(&mut self, stage: ::ShaderStage, offset: u32, values: &[T]) -> &mut Self
	{
		unsafe { vkCmdPushConstants(self.ptr.native_ptr(),
			self.layout[VK_PIPELINE_BIND_POINT_COMPUTE as usize].expect("Pipeline is not bound for graphics pipeline").native_ptr(),
			stage.0, offset, values.len() as _, values.as_ptr() as *const _) };
		self
	}

	/// Push descriptor updates into a command buffer
	#[cfg(feature = "VK_KHR_push_descriptor")]
	pub fn push_graphics_descriptor_set(&mut self, set: u32, writes: &[DescriptorSetWriteInfo]) -> &mut Self
	{
		// save flatten results
		let wt = writes.iter().map(|x|
		{
			let p = x.3.decomposite();
			(x.0, x.1, x.2, p.0, p.1,
				p.2.iter().map(|&(s, v, l)| VkDescriptorImageInfo { sampler: s.map(|x| x.0).unwrap_or(VK_NULL_HANDLE as _), imageView: v.0, imageLayout: l as _ }).collect::<Vec<_>>(),
				p.3.iter().map(|&(b, ref r)| VkDescriptorBufferInfo { buffer: b.native_ptr(), offset: r.start as _, range: (r.end - r.start) as _ }).collect::<Vec<_>>(),
				p.4.iter().map(|x| x.0).collect::<Vec<_>>())
		}).collect::<Vec<_>>();
		let w = wt.iter().map(|&(set, binding, array, dty, count, ref iv, ref bv, ref bvv)| VkWriteDescriptorSet
		{
			dstSet: set, dstBinding: binding, dstArrayElement: array, descriptorType: dty as _, descriptorCount: count,
			pImageInfo: iv.as_ptr(), pBufferInfo: bv.as_ptr(), pTexelBufferView: bvv.as_ptr(), .. Default::default()
		}).collect::<Vec<_>>();
		unsafe { vkCmdPushDescriptorSetKHR(self.ptr.native_ptr(), VK_PIPELINE_BIND_POINT_GRAPHICS, self.layout[VK_PIPELINE_BIND_POINT_GRAPHICS].0,
			set, w.len() as _, w.as_ptr()) };
		self
	}
	/// Push descriptor updates into a command buffer
	#[cfg(feature = "VK_KHR_push_descriptor")]
	pub fn push_compute_descriptor_set(&mut self, set: u32, writes: &[DescriptorSetWriteInfo]) -> &mut Self
	{
		// save flatten results
		let wt = writes.iter().map(|x|
		{
			let p = x.3.decomposite();
			(x.0, x.1, x.2, p.0, p.1,
				p.2.iter().map(|&(s, v, l)| VkDescriptorImageInfo { sampler: s.map(|x| x.0).unwrap_or(VK_NULL_HANDLE as _), imageView: v.0, imageLayout: l as _ }).collect::<Vec<_>>(),
				p.3.iter().map(|&(b, ref r)| VkDescriptorBufferInfo { buffer: b.native_ptr(), offset: r.start as _, range: (r.end - r.start) as _ }).collect::<Vec<_>>(),
				p.4.iter().map(|x| x.0).collect::<Vec<_>>())
		}).collect::<Vec<_>>();
		let w = wt.iter().map(|&(set, binding, array, dty, count, ref iv, ref bv, ref bvv)| VkWriteDescriptorSet
		{
			dstSet: set, dstBinding: binding, dstArrayElement: array, descriptorType: dty as _, descriptorCount: count,
			pImageInfo: iv.as_ptr(), pBufferInfo: bv.as_ptr(), pTexelBufferView: bvv.as_ptr(), .. Default::default()
		}).collect::<Vec<_>>();
		unsafe { vkCmdPushDescriptorSetKHR(self.ptr.native_ptr(), VK_PIPELINE_BIND_POINT_COMPUTE, self.layout[VK_PIPELINE_BIND_POINT_COMPUTE].0,
			set, w.len() as _, w.as_ptr()) };
		self
	}
}

/// Graphics Commands: Updating dynamic states
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Set the viewport on a command buffer
	pub fn set_viewport(&mut self, first: u32, viewports: &[VkViewport]) -> &mut Self
	{
		unsafe { vkCmdSetViewport(self.ptr.native_ptr(), first, viewports.len() as _, viewports.as_ptr()) };
		self
	}
	/// Set the dynamic scissor rectangles on a command buffer
	pub fn set_scissor(&mut self, first: u32, scissors: &[VkRect2D]) -> &mut Self
	{
		unsafe { vkCmdSetScissor(self.ptr.native_ptr(), first, scissors.len() as _, scissors.as_ptr()) };
		self
	}
	/// Set the dynamic line width state
	pub fn set_line_width(&mut self, w: f32) -> &Self { unsafe { vkCmdSetLineWidth(self.ptr.native_ptr(), w) }; self }
	/// Set the depth bias dynamic state
	pub fn set_depth_bias(&mut self, constant_factor: f32, clamp: f32, slope_factor: f32) -> &mut Self
	{
		unsafe { vkCmdSetDepthBias(self.ptr.native_ptr(), constant_factor, clamp, slope_factor) };
		self
	}
	/// Set the values of blend constants
	pub fn set_blend_constants(&mut self, blend_constants: [f32; 4]) -> &mut Self
	{
		unsafe { vkCmdSetBlendConstants(self.ptr.native_ptr(), blend_constants) };
		self
	}
	/// Set the depth bounds test values for a command buffer
	pub fn set_depth_bounds(&mut self, bounds: ::std::ops::Range<f32>) -> &mut Self
	{
		unsafe { vkCmdSetDepthBounds(self.ptr.native_ptr(), bounds.start, bounds.end) };
		self
	}
	/// Set the stencil compare mask dynamic state
	pub fn set_stencil_compare_mask(&mut self, face_mask: ::StencilFaceMask, compare_mask: u32) -> &mut Self
	{
		unsafe { vkCmdSetStencilCompareMask(self.ptr.native_ptr(), face_mask as _, compare_mask) };
		self
	}
	/// Set the stencil write mask dynamic state
	pub fn set_stencil_write_mask(&mut self, face_mask: ::StencilFaceMask, write_mask: u32) -> &mut Self
	{
		unsafe { vkCmdSetStencilWriteMask(self.ptr.native_ptr(), face_mask as _, write_mask) };
		self
	}
	/// Set the stencil reference dynamic state
	pub fn set_stencil_reference(&mut self, face_mask: ::StencilFaceMask, reference: u32) -> &mut Self
	{
		unsafe { vkCmdSetStencilReference(self.ptr.native_ptr(), face_mask as _, reference) };
		self
	}
}

/// Graphics Commands: Binding Buffers
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Bind an index buffer to a command buffer
	pub fn bind_index_buffer(&mut self, buffer: &::Buffer, offset: usize, index_type: IndexType) -> &mut Self
	{
		unsafe { vkCmdBindIndexBuffer(self.ptr.native_ptr(), buffer.native_ptr(), offset as _, index_type as _) };
		self
	}
	/// Bind vertex buffers to a command buffer
	pub fn bind_vertex_buffers(&mut self, first: u32, buffers: &[(&::Buffer, usize)]) -> &mut Self
	{
		let (bufs, ofs): (Vec<_>, Vec<_>) = buffers.into_iter().map(|&(b, o)| (b.native_ptr(), o as VkDeviceSize)).unzip();
		unsafe { vkCmdBindVertexBuffers(self.ptr.native_ptr(), first, bufs.len() as _, bufs.as_ptr(), ofs.as_ptr()) };
		self
	}
}

/// Graphics Commands: Inside a Render Pass
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Draw primitives
	pub fn draw(&mut self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) -> &mut Self
	{
		unsafe { vkCmdDraw(self.ptr.native_ptr(), vertex_count, instance_count, first_vertex, first_instance) };
		self
	}
	/// Issue an indexed draw into a command buffer
	pub fn draw_indexed(&mut self, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) -> &mut Self
	{
		unsafe { vkCmdDrawIndexed(self.ptr.native_ptr(), index_count, instance_count, first_index, vertex_offset, first_instance) };
		self
	}
	/// Issue an indirect draw into a command buffer
	pub fn draw_indirect(&mut self, buffer: &::Buffer, offset: usize, draw_count: u32, stride: u32) -> &mut Self
	{
		unsafe { vkCmdDrawIndirect(self.ptr.native_ptr(), buffer.native_ptr(), offset as _, draw_count, stride) };
		self
	}
	/// Perform an indexed indirect draw
	pub fn draw_indexed_indirect(&mut self, buffer: &::Buffer, offset: usize, draw_count: u32, stride: u32) -> &mut Self
	{
		unsafe { vkCmdDrawIndexedIndirect(self.ptr.native_ptr(), buffer.native_ptr(), offset as _, draw_count, stride) };
		self
	}
}

/// Compute Commands: Dispatching kernels
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Dispatch compute work items
	pub fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) -> &mut Self
	{
		unsafe { vkCmdDispatch(self.ptr.native_ptr(), group_count_x, group_count_y, group_count_z) };
		self
	}
	/// Dispatch compute work items using indirect parameters
	pub fn dispatch_indirect(&mut self, buffer: &::Buffer, offset: usize) -> &mut Self
	{
		unsafe { vkCmdDispatchIndirect(self.ptr.native_ptr(), buffer.native_ptr(), offset as _) };
		self
	}
}

/// Transfer Commands: Copying resources
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Copy data between buffer regions
	pub fn copy_buffer(&mut self, src: &::Buffer, dst: &::Buffer, regions: &[VkBufferCopy]) -> &mut Self
	{
		unsafe { vkCmdCopyBuffer(self.ptr.native_ptr(), src.native_ptr(), dst.native_ptr(), regions.len() as _, regions.as_ptr()) };
		self
	}
	/// Copy data between images
	pub fn copy_image(&mut self, src: &::Image, src_layout: ::ImageLayout, dst: &::Image, dst_layout: ::ImageLayout, regions: &[VkImageCopy]) -> &mut Self
	{
		unsafe { vkCmdCopyImage(self.ptr.native_ptr(), src.native_ptr(), src_layout as _, dst.native_ptr(), dst_layout as _, regions.len() as _, regions.as_ptr()) };
		self
	}
	/// Copy regions of an image, potentially performing format conversion
	pub fn blit_image(&mut self, src: &::Image, src_layout: ::ImageLayout, dst: &::Image, dst_layout: ::ImageLayout,
		regions: &[VkImageBlit], filter: ::FilterMode) -> &mut Self
	{
		unsafe { vkCmdBlitImage(self.ptr.native_ptr(), src.native_ptr(), src_layout as _, dst.native_ptr(), dst_layout as _,
			regions.len() as _, regions.as_ptr(), filter as _) };
		self
	}
	/// Copy data from a buffer into an image
	pub fn copy_buffer_to_image(&mut self, src_buffer: &::Buffer, dst_image: &::Image, dst_layout: ::ImageLayout, regions: &[VkBufferImageCopy]) -> &mut Self
	{
		unsafe { vkCmdCopyBufferToImage(self.ptr.native_ptr(), src_buffer.native_ptr(), dst_image.native_ptr(), dst_layout as _, regions.len() as _, regions.as_ptr()) };
		self
	}
	/// Copy image data into a buffer
	pub fn copy_image_to_buffer(&mut self, src_image: &::Image, src_layout: ::ImageLayout, dst_buffer: &::Buffer, regions: &[VkBufferImageCopy]) -> &mut Self
	{
		unsafe { vkCmdCopyImageToBuffer(self.ptr.native_ptr(), src_image.native_ptr(), src_layout as _, dst_buffer.native_ptr(), regions.len() as _, regions.as_ptr()) };
		self
	}
	/// Update a buffer's contents from host memory
	pub fn update_buffer<T>(&mut self, dst: &::Buffer, dst_offset: usize, size: usize, data: &T) -> &mut Self
	{
		assert!(size <= ::std::mem::size_of::<T>());
		unsafe { vkCmdUpdateBuffer(self.ptr.native_ptr(), dst.native_ptr(), dst_offset as _, size as _, data as *const T as *const _) };
		self
	}
}

/// Graphics/Compute Commands: Transfer-like(clearing/filling) commands
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Fill a region of a buffer with a fixed value.  
	/// `size` is number of bytes to fill
	pub fn fill_buffer(&mut self, dst: &::Buffer, dst_offset: usize, size: usize, data: u32) -> &mut Self
	{
		unsafe { vkCmdFillBuffer(self.ptr.native_ptr(), dst.native_ptr(), dst_offset as _, size as _, data) };
		self
	}
	/// Clear regions of a color image
	pub fn clear_color_image<T: ClearColorValue>(&mut self, image: &::Image, layout: ::ImageLayout, color: &T, ranges: &[VkImageSubresourceRange]) -> &mut Self
	{
		unsafe { vkCmdClearColorImage(self.ptr.native_ptr(), image.native_ptr(), layout as _, color.represent(), ranges.len() as _, ranges.as_ptr()) };
		self
	}
	/// Fill regions of a combined depth/stencil image
	pub fn clear_depth_stencil_image(&mut self, image: &::Image, layout: ::ImageLayout, depth: f32, stencil: u32, ranges: &[VkImageSubresourceRange]) -> &mut Self
	{
		unsafe { vkCmdClearDepthStencilImage(self.ptr.native_ptr(), image.native_ptr(), layout as _, &VkClearDepthStencilValue { depth, stencil },
			ranges.len() as _, ranges.as_ptr()) };
		self
	}
}

/// Graphics Commands: Attachment clearing
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Clear regions within currently bound framebuffer attachments
	pub fn clear_attachments(&mut self, attachments: &[VkClearAttachment], rects: &[VkClearRect]) -> &mut Self
	{
		unsafe { vkCmdClearAttachments(self.ptr.native_ptr(), attachments.len() as _, attachments.as_ptr(), rects.len() as _, rects.as_ptr()) };
		self
	}
}

/// Graphics Commands: Resolving an image to another image
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Resolve regions of an image
	pub fn resolve_image(&mut self, src: &::Image, src_layout: ::ImageLayout, dst: &::Image, dst_layout: ::ImageLayout, regions: &[VkImageResolve]) -> &mut Self
	{
		unsafe { vkCmdResolveImage(self.ptr.native_ptr(), src.native_ptr(), src_layout as _, dst.native_ptr(), dst_layout as _, regions.len() as _, regions.as_ptr()) };
		self
	}
}

/// Graphics/Compute Commands: Synchronization between command buffers/queues
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Set an event object to signaled state
	pub fn set_event(&mut self, event: &::Event, stage_mask: ::PipelineStageFlags) -> &mut Self
	{
		unsafe { vkCmdSetEvent(self.ptr.native_ptr(), event.0, stage_mask.0) }; self
	}
	/// Reset an event object to non-signaled state
	pub fn reset_event(&mut self, event: &::Event, stage_mask: ::PipelineStageFlags) -> &mut Self
	{
		unsafe { vkCmdResetEvent(self.ptr.native_ptr(), event.0, stage_mask.0) }; self
	}
	/// Wait for one or more events and insert a set of memory
	pub fn wait_events(&mut self, events: &[&::Event], src_stage_mask: ::PipelineStageFlags, dst_stage_mask: ::PipelineStageFlags,
		memory_barriers: &[VkMemoryBarrier], buffer_memory_barriers: &[VkBufferMemoryBarrier], image_memory_barriers: &[VkImageMemoryBarrier]) -> &mut Self
	{
		let evs = events.into_iter().map(|x| x.0).collect::<Vec<_>>();
		unsafe { vkCmdWaitEvents(self.ptr.native_ptr(), evs.len() as _, evs.as_ptr(), src_stage_mask.0, dst_stage_mask.0,
			memory_barriers.len() as _, memory_barriers.as_ptr(), buffer_memory_barriers.len() as _, buffer_memory_barriers.as_ptr(),
			image_memory_barriers.len() as _, image_memory_barriers.as_ptr()) };
		self
	}
	/// Insert a memory dependency
	pub fn pipeline_barrier(&mut self, src_stage_mask: ::PipelineStageFlags, dst_stage_mask: ::PipelineStageFlags, by_region: bool,
		memory_barriers: &[VkMemoryBarrier], buffer_memory_barriers: &[VkBufferMemoryBarrier], image_memory_barriers: &[VkImageMemoryBarrier]) -> &mut Self
	{
		unsafe { vkCmdPipelineBarrier(self.ptr.native_ptr(), src_stage_mask.0, dst_stage_mask.0, if by_region { VK_DEPENDENCY_BY_REGION_BIT } else { 0 },
			memory_barriers.len() as _, memory_barriers.as_ptr(), buffer_memory_barriers.len() as _, buffer_memory_barriers.as_ptr(),
			image_memory_barriers.len() as _, image_memory_barriers.as_ptr()) };
		self
	}
}

/// Graphics/Compute Commands: Querying
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Begin a query
	pub fn begin_query(&mut self, pool: &::QueryPool, query: u32, precise_query: bool) -> &mut Self
	{
		unsafe { vkCmdBeginQuery(self.ptr.native_ptr(), pool.0, query, if precise_query { VK_QUERY_CONTROL_PRECISE_BIT } else { 0 }) };
		self
	}
	/// Ends a query
	pub fn end_query(&mut self, pool: &::QueryPool, query: u32) -> &mut Self
	{
		unsafe { vkCmdEndQuery(self.ptr.native_ptr(), pool.0, query) }; self
	}
	/// Reset queries in a query pool
	pub fn reset_query_pool(&mut self, pool: &::QueryPool, range: ::std::ops::Range<u32>) -> &mut Self
	{
		unsafe { vkCmdResetQueryPool(self.ptr.native_ptr(), pool.0, range.start, range.end - range.start) }; self
	}
	/// Write a device timestamp into a query object
	pub fn write_timestamp(&mut self, stage: ::PipelineStageFlags, pool: &::QueryPool, query: u32) -> &mut Self
	{
		unsafe { vkCmdWriteTimestamp(self.ptr.native_ptr(), stage.0, pool.0, query) }; self
	}
	/// Copy the results of queries in a query pool to a buffer object
	pub fn copy_query_pool_results(&mut self, pool: &::QueryPool, range: ::std::ops::Range<u32>, dst: &::Buffer, dst_offset: usize, stride: usize,
		wide_result: bool, flags: ::QueryResultFlags) -> &mut Self
	{
		unsafe { vkCmdCopyQueryPoolResults(self.ptr.native_ptr(), pool.0, range.start, range.end - range.start,
			dst.native_ptr(), dst_offset as _, stride as _, flags.0 | if wide_result { VK_QUERY_RESULT_64_BIT } else { 0 }) };
		self
	}
}

/// Graphics Commands: Manipulating with Render Passes
#[cfg(feature = "FeImplements")]
impl<'d> CmdRecord<'d>
{
	/// Begin a new render pass
	pub fn begin_render_pass(&mut self, pass: &::RenderPass, framebuffer: &::Framebuffer, render_area: VkRect2D, clear_values: &[ClearValue],
		inline_commands: bool) -> &mut Self
	{
		let cvalues = clear_values.into_iter().map(|x| match x
		{
			&ClearValue::Color(ref color) => VkClearValue { color: VkClearColorValue { float32: color.clone() } },
			&ClearValue::DepthStencil(depth, stencil) => VkClearValue { depthStencil: VkClearDepthStencilValue { depth, stencil } }
		}).collect::<Vec<_>>();
		let binfo = VkRenderPassBeginInfo
		{
			renderPass: pass.native_ptr(), framebuffer: framebuffer.native_ptr(), renderArea: render_area, clearValueCount: cvalues.len() as _,
			pClearValues: cvalues.as_ptr(), .. Default::default()
		};
		unsafe { vkCmdBeginRenderPass(self.ptr.native_ptr(), &binfo,
			if inline_commands { VK_SUBPASS_CONTENTS_INLINE } else { VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS }) };
		self
	}
	/// Transition to the next subpass of a render pass
	pub fn next_subpass(&mut self, inline_commands: bool) -> &mut Self
	{
		unsafe { vkCmdNextSubpass(self.ptr.native_ptr(),
			if inline_commands { VK_SUBPASS_CONTENTS_INLINE } else { VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS }) };
		self
	}
	/// End the current render pass
	pub fn end_render_pass(&mut self) -> &mut Self { unsafe { vkCmdEndRenderPass(self.ptr.native_ptr()) }; self }
}

/// The trait representation of `VkClearColorValue`
pub trait ClearColorValue
{
	fn represent(&self) -> &VkClearColorValue;
}
impl ClearColorValue for [f32; 4] { fn represent(&self) -> &VkClearColorValue { unsafe { ::std::mem::transmute(self) } } }
impl ClearColorValue for [i32; 4] { fn represent(&self) -> &VkClearColorValue { unsafe { ::std::mem::transmute(self) } } }
impl ClearColorValue for [u32; 4] { fn represent(&self) -> &VkClearColorValue { unsafe { ::std::mem::transmute(self) } } }

/// The enum representation of `VkClearValue`
pub enum ClearValue
{
	/// Color Value: r, g, b, a
	Color([f32; 4]),
	/// Depth and Stencil Value: depth, stencil
	DepthStencil(f32, u32)
}

/// Type of index buffer indices
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexType
{
	/// Indices are 16-bit unsigned integer values
	U16 = VK_INDEX_TYPE_UINT16 as _,
	/// Indices are 32-bit unsigned integer values
	U32 = VK_INDEX_TYPE_UINT32 as _
}

/// Enabling or disabling the occlusion query
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcclusionQuery
{
	Disable, Enable,
	/// `VK_QUERY_CONTROL_PRECISE_BIT`
	Precise
}