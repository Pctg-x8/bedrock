//! Vulkan Commands

use derives::{bitflags_newtype, implements};

use crate::{
    Buffer, ClearAttachment, ClearRect, DependencyFlags, DescriptorSet, DeviceSize, Event, Framebuffer, Image,
    ImageSubresourceRange, LayoutTransition, Pipeline, PipelineLayout, QueryPipelineStatisticFlags, Rect2D, SubpassRef,
    Viewport, VkHandleMut, VkObject, VkRawHandle, VulkanStructure, ffi_helper::slice_as_ptr_empty_null, vk::*,
};
#[implements]
use crate::{FilterMode, PipelineStageFlags, QueryResultFlags, StencilFaceMask};
use crate::{ImageLayout, VkHandle};
use core::ops::Range;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandPoolCreateInfo(VkCommandPoolCreateInfo);
impl CommandPoolCreateInfo {
    pub const fn new(queue_family_index: u32) -> Self {
        Self(VkCommandPoolCreateInfo {
            sType: VkCommandPoolCreateInfo::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            queueFamilyIndex: queue_family_index,
        })
    }

    pub const unsafe fn from_raw(raw: VkCommandPoolCreateInfo) -> Self {
        Self(raw)
    }

    pub const fn into_raw(self) -> VkCommandPoolCreateInfo {
        self.0
    }

    pub const fn transient(mut self) -> Self {
        self.0.flags |= VK_COMMAND_POOL_CREATE_TRANSIENT_BIT;
        self
    }

    pub const fn individual_resettable(mut self) -> Self {
        self.0.flags |= VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
        self
    }
}

/// Enumerant specifying a command buffer level.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommandBufferLevel {
    /// A primary command buffer.
    Primary = VK_COMMAND_BUFFER_LEVEL_PRIMARY,
    /// A secondary command buffer.
    Secondary = VK_COMMAND_BUFFER_LEVEL_SECONDARY,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandBufferAllocateInfo<'r>(
    pub(crate) VkCommandBufferAllocateInfo,
    core::marker::PhantomData<&'r mut CommandPool>,
);
impl<'r> CommandBufferAllocateInfo<'r> {
    #[inline(always)]
    pub const fn new(command_pool: &'r mut CommandPool, count: u32, level: CommandBufferLevel) -> Self {
        Self(
            VkCommandBufferAllocateInfo {
                sType: VkCommandBufferAllocateInfo::TYPE,
                pNext: core::ptr::null(),
                commandPool: command_pool as *mut _ as _,
                level: level as _,
                commandBufferCount: count,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkCommandBufferAllocateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkCommandBufferAllocateInfo {
        self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandBufferFixedCountAllocateInfo<'r, const N: usize>(
    pub(crate) VkCommandBufferAllocateInfo,
    core::marker::PhantomData<&'r mut CommandPool>,
);
impl<'r, const N: usize> CommandBufferFixedCountAllocateInfo<'r, N> {
    #[inline(always)]
    pub const fn new(command_pool: &'r mut CommandPool, level: CommandBufferLevel) -> Self {
        assert!(N <= u32::MAX as usize, "too many command buffers will be allocated");

        Self(
            VkCommandBufferAllocateInfo {
                sType: VkCommandBufferAllocateInfo::TYPE,
                pNext: core::ptr::null(),
                commandPool: command_pool as *mut _ as _,
                level: level as _,
                commandBufferCount: N as _,
            },
            core::marker::PhantomData,
        )
    }
}

/// Opaque handle to a command pool object.
#[repr(transparent)]
pub struct CommandPool(VkCommandPool_T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct CommandPoolResetFlags(VkCommandPoolResetFlags);
impl CommandPoolResetFlags {
    /// Empty bits.
    pub const EMPTY: Self = Self(0);

    /// Resetting a command pool recycles all of the resources from the command pool back to the system.
    pub const RELEASE_RESOURCES: Self = Self(VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT);
}

#[cfg(feature = "VK_KHR_maintenance1")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct CommandPoolTrimFlags(VkCommandPoolTrimFlagsKHR);
impl CommandPoolTrimFlags {
    /// Empty bits.
    pub const EMPTY: Self = Self(0);
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandBufferBeginInfo<'d>(
    VkCommandBufferBeginInfo,
    core::marker::PhantomData<Option<&'d CommandBufferInheritanceInfo<'d>>>,
);
impl<'d> CommandBufferBeginInfo<'d> {
    pub const fn new() -> Self {
        Self(
            VkCommandBufferBeginInfo {
                sType: VkCommandBufferBeginInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                pInheritanceInfo: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkCommandBufferBeginInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkCommandBufferBeginInfo {
        self.0
    }

    pub const fn as_raw_ref(&self) -> &VkCommandBufferBeginInfo {
        &self.0
    }

    pub const fn onetime_submit(mut self) -> Self {
        self.0.flags |= VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;
        self
    }

    pub const fn renderpass_continue(mut self) -> Self {
        self.0.flags |= VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT;
        self
    }

    pub const fn simultaneous_use(mut self) -> Self {
        self.0.flags |= VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT;
        self
    }

    pub const fn with_inheritance_info(mut self, info: &'d CommandBufferInheritanceInfo<'d>) -> Self {
        self.0.pInheritanceInfo = info as *const _ as _;
        self
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandBufferInheritanceInfo<'d>(
    VkCommandBufferInheritanceInfo,
    core::marker::PhantomData<(
        Option<&'d dyn VkHandle<Handle = VkRenderPass>>,
        Option<&'d dyn VkHandle<Handle = VkFramebuffer>>,
    )>,
);
impl<'d> CommandBufferInheritanceInfo<'d> {
    pub const fn new() -> Self {
        CommandBufferInheritanceInfo(
            VkCommandBufferInheritanceInfo {
                sType: VkCommandBufferInheritanceInfo::TYPE,
                pNext: core::ptr::null(),
                renderPass: VkRenderPass::NULL,
                subpass: 0,
                framebuffer: VkFramebuffer::NULL,
                occlusionQueryEnable: false as _,
                queryFlags: 0,
                pipelineStatistics: 0,
            },
            core::marker::PhantomData,
        )
    }

    #[inline]
    pub const fn of_rendering(render_pass: SubpassRef<'d>, framebuffer: Option<&'d Framebuffer>) -> Self {
        Self::new().rendering(render_pass, framebuffer)
    }

    #[inline]
    pub const fn rendering(mut self, render_pass: SubpassRef<'d>, framebuffer: Option<&'d Framebuffer>) -> Self {
        self.0.renderPass = render_pass.0 as *const _ as _;
        self.0.subpass = render_pass.1;
        self.0.framebuffer = match framebuffer {
            Some(x) => x as *const _ as _,
            None => VK_NULL_HANDLE as _,
        };

        self
    }

    pub const fn occlusion_query(mut self, q: OcclusionQuery) -> Self {
        self.0.occlusionQueryEnable = match q {
            OcclusionQuery::Disable => false as _,
            _ => true as _,
        };
        self.0.queryFlags = match q {
            OcclusionQuery::Precise => VK_QUERY_CONTROL_PRECISE_BIT,
            _ => 0,
        };
        self
    }

    pub const fn pipeline_statistics(mut self, stats: QueryPipelineStatisticFlags) -> Self {
        self.0.pipelineStatistics = stats.0;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct CommandBufferResetFlags(VkCommandBufferResetFlags);
impl CommandBufferResetFlags {
    /// Empty bits.
    pub const EMPTY: Self = Self(0);

    /// Most or all memory resources currently owned by the command buffer should be returned to the parent command pool.
    /// If this flag is not set, then the command buffer may hold onto memory resources and reuse them when recording commands.
    /// `commandBuffer` is moved to the [initial state](https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#commandbuffers-lifecycle).
    pub const RELEASE_RESOURCES: Self = Self(VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT);
}

/// Opaque handle to a command buffer object
#[repr(transparent)]
pub struct CommandBuffer(VkCommandBuffer);
#[implements]
impl CommandBuffer {
    /// Start recording a command buffer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    ///
    /// # Safety
    /// The `CommandPool` that this commandBuffer was allocated from must be externally synchronized.
    #[inline]
    unsafe fn begin<'d>(&'d mut self, info: &CommandBufferBeginInfo) -> crate::Result<CmdRecord<'d>> {
        unsafe {
            crate::vkfn::begin_command_buffer(self as *mut _ as _, info.as_raw_ref()).into_result()?;
        }

        Ok(CmdRecord { ptr: self })
    }

    /// Reset a command buffer to the initial state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    ///
    /// # Safety
    /// The `CommandPool` that this commandBuffer was allocated from must be externally synchronized.
    #[implements]
    #[inline]
    unsafe fn reset(&mut self, flags: CommandBufferResetFlags) -> crate::Result<()> {
        unsafe {
            crate::vkfn::reset_command_buffer(self as *mut _ as _, flags.bits())
                .into_result()
                .map(drop)
        }
    }
}

/// The recording state of command buffers
#[implements]
#[must_use = "CmdRecord must be consumed by end() (not closed automatically by drop!)"]
pub struct CmdRecord<'d> {
    ptr: &'d mut CommandBuffer,
}

/// Common Commands: End Recording
#[implements]
impl<'d> CmdRecord<'d> {
    /// Finish recording a command buffer
    #[inline]
    pub fn end(self) -> crate::Result<()> {
        unsafe {
            crate::vkfn::end_command_buffer(self.ptr as *mut _ as _)
                .into_result()
                .map(drop)
        }
    }
}

/// Specify how commands in the first subpass of a render pass are provided2
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubpassContents {
    /// The contents of the subpass will be recorded inline the primary command buffer
    Inline = VK_SUBPASS_CONTENTS_INLINE,
    /// The contents are recorded in secondary command buffers that will be called from the primary command buffer
    SecondaryCommandBuffers = VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS,
}

/// Specify the bind point of a pipeline object to a command buffer
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineBindPoint {
    /// Binding as a graphics pipeline
    Graphics = VK_PIPELINE_BIND_POINT_GRAPHICS,
    /// Binding as a compute pipeline
    Compute = VK_PIPELINE_BIND_POINT_COMPUTE,
}

/// Graphics Commands: Manipulating with Render Passes
#[implements]
impl<'d> CmdRecord<'d> {
    /// Begin a new render pass
    #[inline]
    pub fn begin_render_pass(self, info: &crate::RenderPassBeginInfo, contents: SubpassContents) -> Self {
        unsafe {
            crate::vkfn::cmd_begin_render_pass(self.ptr as *mut _ as _, info.as_ref(), contents as _);
        }

        self
    }

    /// Transition to the next subpass of a render pass
    #[inline]
    pub fn next_subpass(self, contents: SubpassContents) -> Self {
        unsafe {
            crate::vkfn::cmd_next_subpass(self.ptr as *mut _ as _, contents as _);
        }

        self
    }

    /// End the current render pass
    #[inline]
    pub fn end_render_pass(self) -> Self {
        unsafe { crate::vkfn::cmd_end_render_pass(self.ptr as *mut _ as _) };

        self
    }

    #[cfg(feature = "Allow1_2APIs")]
    #[inline]
    pub fn begin_render_pass_2(
        self,
        begin_info: &crate::RenderPassBeginInfo,
        subpass_begin_info: &VkSubpassBeginInfoKHR,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_begin_render_pass2(self.ptr as *mut _ as _, begin_info.as_ref(), subpass_begin_info);
        }

        self
    }

    #[cfg(feature = "Allow1_2APIs")]
    #[inline]
    pub fn next_subpass_2(
        self,
        subpass_begin_info: &VkSubpassBeginInfoKHR,
        subpass_end_info: &VkSubpassEndInfoKHR,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_next_subpass2(self.ptr as *mut _ as _, subpass_begin_info, subpass_end_info);
        }

        self
    }

    #[cfg(feature = "Allow1_2APIs")]
    #[inline]
    pub fn end_render_pass_2(self, subpass_end_info: &VkSubpassEndInfoKHR) -> Self {
        unsafe {
            crate::vkfn::cmd_end_render_pass2(self.ptr as *mut _ as _, subpass_end_info);
        }

        self
    }
}

/// Graphics/Compute Commands: Pipeline Setup
#[implements]
impl<'d> CmdRecord<'d> {
    /// Bind a pipeline object to a command buffer
    #[inline]
    pub fn bind_pipeline(self, bind_point: PipelineBindPoint, pipeline: &Pipeline) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_pipeline(self.ptr as *mut _ as _, bind_point as _, pipeline as *const _ as _);
        }

        self
    }

    /// Binds descriptor sets to a command buffer
    #[inline]
    pub fn bind_descriptor_sets(
        self,
        bind_point: PipelineBindPoint,
        pipeline_layout: &PipelineLayout,
        first: u32,
        descriptor_sets: &[&DescriptorSet],
        dynamic_offsets: &[u32],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_descriptor_sets(
                self.ptr as *mut _ as _,
                bind_point as _,
                pipeline_layout as *const _ as _,
                first,
                descriptor_sets.len() as _,
                slice_as_ptr_empty_null(descriptor_sets) as _,
                dynamic_offsets.len() as _,
                slice_as_ptr_empty_null(dynamic_offsets),
            );
        }

        self
    }

    /// Update the value of push constant
    #[inline]
    pub fn push_constant<T>(
        self,
        pipeline_layout: &PipelineLayout,
        stage: VkShaderStageFlags,
        offset: u32,
        value: &T,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_push_constants(
                self.ptr as *mut _ as _,
                pipeline_layout as *const _ as _,
                stage,
                offset,
                core::mem::size_of::<T>() as _,
                value as *const T as *const _,
            );
        }
        self
    }

    /// Push descriptor updates into a command buffer
    #[cfg(feature = "Allow1_4APIs")]
    #[inline(always)]
    pub unsafe fn push_descriptor_set_raw(
        self,
        bind_point: PipelineBindPoint,
        pipeline_layout: &PipelineLayout,
        set: u32,
        writes: &[VkWriteDescriptorSet],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_push_descriptor_set(
                self.ptr as *mut _ as _,
                bind_point as _,
                pipeline_layout as *const _ as _,
                set,
                writes.len() as _,
                slice_as_ptr_empty_null(writes),
            );
        }

        self
    }

    /// Push descriptor updates into a command buffer
    #[cfg(feature = "VK_KHR_push_descriptor")]
    #[cfg(feature = "alloc")]
    pub fn push_descriptor_set_alloc(
        self,
        bind_point: PipelineBindPoint,
        pipeline_layout: &PipelineLayout,
        set: u32,
        writes: &[crate::DescriptorSetWriteInfo],
    ) -> Self {
        unsafe {
            self.push_descriptor_set_raw(
                bind_point,
                pipeline_layout,
                set,
                &crate::alloc::collect_vec(writes.iter().map(|x| x.make_structure())),
            )
        }
    }
}

/// Graphics Commands: Updating dynamic states
#[implements]
impl CmdRecord<'_> {
    /// Set the viewport on a command buffer
    #[inline(always)]
    pub fn set_viewport(self, first: u32, viewports: &[Viewport]) -> Self {
        unsafe {
            crate::vkfn::cmd_set_viewport(
                self.ptr as *mut _ as _,
                first,
                viewports.len() as _,
                slice_as_ptr_empty_null(viewports),
            );
        }

        self
    }

    /// Set the dynamic scissor rectangles on a command buffer
    #[inline(always)]
    pub fn set_scissor(self, first: u32, scissors: &[Rect2D]) -> Self {
        unsafe {
            crate::vkfn::cmd_set_scissor(
                self.ptr as *mut _ as _,
                first,
                scissors.len() as _,
                slice_as_ptr_empty_null(scissors),
            );
        }

        self
    }

    /// Set the dynamic line width state
    #[inline(always)]
    pub fn set_line_width(self, w: f32) -> Self {
        unsafe {
            crate::vkfn::cmd_set_line_width(self.ptr as *mut _ as _, w);
        }

        self
    }

    /// Set the depth bias dynamic state
    #[inline(always)]
    pub fn set_depth_bias(self, constant_factor: f32, clamp: f32, slope_factor: f32) -> Self {
        unsafe {
            crate::vkfn::cmd_set_depth_bias(self.ptr as *mut _ as _, constant_factor, clamp, slope_factor);
        }

        self
    }

    /// Set the values of blend constants
    #[inline(always)]
    pub fn set_blend_constants(self, blend_constants: &[f32; 4]) -> Self {
        unsafe {
            crate::vkfn::cmd_set_blend_constants(self.ptr as *mut _ as _, blend_constants.as_ptr());
        }

        self
    }

    /// Set the depth bounds test values for a command buffer
    #[inline(always)]
    pub fn set_depth_bounds(self, bounds: Range<f32>) -> Self {
        unsafe {
            crate::vkfn::cmd_set_depth_bounds(self.ptr as *mut _ as _, bounds.start, bounds.end);
        }

        self
    }

    /// Set the stencil compare mask dynamic state
    #[inline(always)]
    pub fn set_stencil_compare_mask(self, face_mask: StencilFaceMask, compare_mask: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_set_stencil_compare_mask(self.ptr as *mut _ as _, face_mask as _, compare_mask);
        }

        self
    }

    /// Set the stencil write mask dynamic state
    #[inline(always)]
    pub fn set_stencil_write_mask(self, face_mask: StencilFaceMask, write_mask: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_set_stencil_write_mask(self.ptr as *mut _ as _, face_mask as _, write_mask);
        }

        self
    }

    /// Set the stencil reference dynamic state
    #[inline(always)]
    pub fn set_stencil_reference(self, face_mask: StencilFaceMask, reference: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_set_stencil_reference(self.ptr as *mut _ as _, face_mask as _, reference);
        }

        self
    }

    /// Set the sample locations state
    #[cfg(feature = "VK_EXT_sample_locations")]
    #[inline(always)]
    pub fn set_sample_locations(self, info: &VkSampleLocationsInfoEXT) -> Self {
        unsafe {
            self.device().cmd_set_sample_locations_ext_fn().0(self.ptr as *mut _ as _, info);
        }

        self
    }
}

/// Graphics Commands: Binding Buffers
#[implements]
impl CmdRecord<'_> {
    /// Bind an index buffer to a command buffer
    #[inline(always)]
    pub fn bind_index_buffer(self, buffer: &Buffer, offset: usize, index_type: IndexType) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_index_buffer(
                self.ptr as *mut _ as _,
                buffer as *const _ as _,
                offset as _,
                index_type as _,
            );
        }

        self
    }

    /// Bind vertex buffers to a command buffer
    #[inline(always)]
    pub fn bind_vertex_buffers(self, first: u32, buffers: &[&Buffer], offsets: &[DeviceSize]) -> Self {
        assert_eq!(buffers.len(), offsets.len());

        unsafe {
            crate::vkfn::cmd_bind_vertex_buffers(
                self.ptr as *mut _ as _,
                first,
                buffers.len() as _,
                slice_as_ptr_empty_null(buffers) as _,
                slice_as_ptr_empty_null(offsets),
            );
        }

        self
    }

    /// Bind vertex buffers to a command buffer
    #[inline(always)]
    pub fn bind_vertex_buffer_array<const N: usize>(
        self,
        first: u32,
        buffers: &[&Buffer; N],
        offsets: &[DeviceSize; N],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_vertex_buffers(
                self.ptr.native_ptr_mut(),
                first,
                N as _,
                slice_as_ptr_empty_null(buffers) as _,
                slice_as_ptr_empty_null(offsets),
            );
        }

        self
    }
}

/// Graphics Commands: Inside a Render Pass
#[implements]
impl CmdRecord<'_> {
    /// Draw primitives
    #[inline(always)]
    pub fn draw(self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_draw(
                self.ptr as *mut _ as _,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            );
        }

        self
    }

    /// Issue an indexed draw into a command buffer
    #[inline(always)]
    pub fn draw_indexed(
        self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_draw_indexed(
                self.ptr as *mut _ as _,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            );
        }

        self
    }

    /// Issue an indirect draw into a command buffer
    #[inline(always)]
    pub fn draw_indirect(self, buffer: &Buffer, offset: DeviceSize, draw_count: u32, stride: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_draw_indirect(
                self.ptr as *mut _ as _,
                buffer as *const _ as _,
                offset,
                draw_count,
                stride,
            );
        }

        self
    }

    /// Perform an indexed indirect draw
    #[inline(always)]
    pub fn draw_indexed_indirect(self, buffer: &Buffer, offset: DeviceSize, draw_count: u32, stride: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_draw_indexed_indirect(
                self.ptr as *mut _ as _,
                buffer as *const _ as _,
                offset,
                draw_count,
                stride,
            );
        }

        self
    }
}

/// Compute Commands: Dispatching kernels
#[implements]
impl CmdRecord<'_> {
    /// Dispatch compute work items
    #[inline(always)]
    pub fn dispatch(self, group_count_x: u32, group_count_y: u32, group_count_z: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_dispatch(self.ptr as *mut _ as _, group_count_x, group_count_y, group_count_z);
        }

        self
    }

    /// Dispatch compute work items using indirect parameters
    #[inline(always)]
    pub fn dispatch_indirect(self, buffer: &Buffer, offset: DeviceSize) -> Self {
        unsafe {
            crate::vkfn::cmd_dispatch_indirect(self.ptr as *mut _ as _, buffer as *const _ as _, offset);
        }

        self
    }
}

/// Transfer Commands: Copying resources
#[implements]
impl CmdRecord<'_> {
    /// Copy data between buffer regions
    #[inline(always)]
    pub fn copy_buffer(self, src: &Buffer, dst: &Buffer, regions: &[BufferCopy]) -> Self {
        unsafe {
            crate::vkfn::cmd_copy_buffer(
                self.ptr as *mut _ as _,
                src as *const _ as _,
                dst as *const _ as _,
                regions.len() as _,
                slice_as_ptr_empty_null(regions) as _,
            );
        }

        self
    }

    /// Copy data between images
    #[inline(always)]
    pub fn copy_image(
        self,
        src: &Image,
        src_layout: ImageLayout,
        dst: &Image,
        dst_layout: ImageLayout,
        regions: &[ImageCopy],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_copy_image(
                self.ptr as *mut _ as _,
                src as *const _ as _,
                src_layout as _,
                dst as *const _ as _,
                dst_layout as _,
                regions.len() as _,
                slice_as_ptr_empty_null(regions),
            );
        }

        self
    }

    /// Copy regions of an image, potentially performing format conversion
    #[inline(always)]
    pub fn blit_image(
        self,
        src: &Image,
        src_layout: ImageLayout,
        dst: &Image,
        dst_layout: ImageLayout,
        regions: &[ImageBlit],
        filter: FilterMode,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_blit_image(
                self.ptr as *mut _ as _,
                src as *const _ as _,
                src_layout as _,
                dst as *const _ as _,
                dst_layout as _,
                regions.len() as _,
                slice_as_ptr_empty_null(regions),
                filter as _,
            );
        }

        self
    }

    /// Copy data from a buffer into an image
    #[inline(always)]
    pub fn copy_buffer_to_image(
        self,
        src_buffer: &Buffer,
        dst_image: &Image,
        dst_layout: ImageLayout,
        regions: &[BufferImageCopy],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_copy_buffer_to_image(
                self.ptr as *mut _ as _,
                src_buffer as *const _ as _,
                dst_image as *const _ as _,
                dst_layout as _,
                regions.len() as _,
                slice_as_ptr_empty_null(regions),
            );
        }

        self
    }

    /// Copy image data into a buffer
    #[inline(always)]
    pub fn copy_image_to_buffer(
        self,
        src_image: &Image,
        src_layout: ImageLayout,
        dst_buffer: &Buffer,
        regions: &[BufferImageCopy],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_copy_image_to_buffer(
                self.ptr as *mut _ as _,
                src_image as *const _ as _,
                src_layout as _,
                dst_buffer as *const _ as _,
                regions.len() as _,
                slice_as_ptr_empty_null(regions),
            );
        }

        self
    }

    /// Update a buffer's contents from host memory
    #[inline(always)]
    pub fn update_buffer<T>(self, dst: &Buffer, dst_offset: DeviceSize, size: DeviceSize, data: &T) -> Self {
        assert!(
            size <= size_of::<T>() as DeviceSize,
            "Updated size exceeds size of datatype"
        );

        unsafe {
            crate::vkfn::cmd_update_buffer(
                self.ptr as *mut _ as _,
                dst as *const _ as _,
                dst_offset,
                size,
                data as *const _ as _,
            );
        }

        self
    }
}

/// Graphics/Compute Commands: Transfer-like(clearing/filling) commands
#[implements]
impl CmdRecord<'_> {
    /// Fill a region of a buffer with a fixed value.
    /// `size` is number of bytes to fill
    #[inline(always)]
    pub fn fill_buffer(self, dst: &Buffer, dst_offset: DeviceSize, size: DeviceSize, data: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_fill_buffer(self.ptr as *mut _ as _, dst as *const _ as _, dst_offset, size, data);
        }

        self
    }

    /// Clear regions of a color image
    #[inline(always)]
    pub fn clear_color_image(
        self,
        image: &Image,
        layout: ImageLayout,
        colors: &[ClearColorValue],
        ranges: &[ImageSubresourceRange],
    ) -> Self {
        assert_eq!(colors.len(), ranges.len());

        unsafe {
            crate::vkfn::cmd_clear_color_image(
                self.ptr as *mut _ as _,
                image as *const _ as _,
                layout as _,
                slice_as_ptr_empty_null(colors),
                ranges.len() as _,
                slice_as_ptr_empty_null(ranges),
            );
        }

        self
    }

    /// Fill regions of a combined depth/stencil image
    #[inline(always)]
    pub fn clear_depth_stencil_image(
        self,
        image: &Image,
        layout: ImageLayout,
        depth: f32,
        stencil: u32,
        ranges: &[ImageSubresourceRange],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_clear_depth_stencil_image(
                self.ptr as *mut _ as _,
                image as *const _ as _,
                layout as _,
                &VkClearDepthStencilValue { depth, stencil },
                ranges.len() as _,
                slice_as_ptr_empty_null(ranges),
            );
        }

        self
    }

    /// Clear regions within currently bound framebuffer attachments
    #[inline(always)]
    pub fn clear_attachments(self, attachments: &[ClearAttachment], rects: &[ClearRect]) -> Self {
        unsafe {
            crate::vkfn::cmd_clear_attachments(
                self.ptr as *mut _ as _,
                attachments.len() as _,
                slice_as_ptr_empty_null(attachments),
                rects.len() as _,
                slice_as_ptr_empty_null(rects),
            );
        }

        self
    }
}

/// Graphics Commands: Executing Subcommands
#[implements]
impl CmdRecord<'_> {
    /// Execute a secondary command buffer from a primary command buffer
    /// # Safety
    /// Caller must be primary buffer and in the render pass when executing secondary command buffer
    #[inline(always)]
    pub unsafe fn execute_commands(self, buffers: &[&CommandBuffer]) -> Self {
        unsafe {
            crate::vkfn::cmd_execute_commands(
                self.ptr as *mut _ as _,
                buffers.len() as _,
                slice_as_ptr_empty_null(buffers) as _,
            );
        }

        self
    }
}

/// Graphics Commands: Resolving an image to another image
#[implements]
impl CmdRecord<'_> {
    /// Resolve regions of an image
    #[inline(always)]
    pub fn resolve_image(
        self,
        src: &Image,
        src_layout: ImageLayout,
        dst: &Image,
        dst_layout: ImageLayout,
        regions: &[ImageResolve],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_resolve_image(
                self.ptr as *mut _ as _,
                src as *const _ as _,
                src_layout as _,
                dst as *const _ as _,
                dst_layout as _,
                regions.len() as _,
                slice_as_ptr_empty_null(regions),
            )
        };

        self
    }
}

/// Graphics/Compute Commands: Synchronization between command buffers/queues
#[implements]
impl CmdRecord<'_> {
    /// Set an event object to signaled state
    #[inline(always)]
    pub fn set_event(self, event: &Event, stage_mask: PipelineStageFlags) -> Self {
        unsafe {
            crate::vkfn::cmd_set_event(self.ptr as *mut _ as _, event as *const _ as _, stage_mask.bits());
        }

        self
    }

    /// Reset an event object to non-signaled state
    #[inline(always)]
    pub fn reset_event(self, event: &Event, stage_mask: PipelineStageFlags) -> Self {
        unsafe {
            crate::vkfn::cmd_reset_event(self.ptr as *mut _ as _, event as *const _ as _, stage_mask.bits());
        }

        self
    }

    /// Wait for one or more events and insert a set of memory
    #[inline(always)]
    pub fn wait_events(
        self,
        events: &[&Event],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_wait_events(
                self.ptr as *mut _ as _,
                events.len() as _,
                slice_as_ptr_empty_null(events) as _,
                src_stage_mask.bits(),
                dst_stage_mask.bits(),
                memory_barriers.len() as _,
                slice_as_ptr_empty_null(memory_barriers),
                buffer_memory_barriers.len() as _,
                slice_as_ptr_empty_null(buffer_memory_barriers) as _,
                image_memory_barriers.len() as _,
                slice_as_ptr_empty_null(image_memory_barriers) as _,
            );
        }
        self
    }

    /// Insert a memory dependency
    #[inline(always)]
    pub fn pipeline_barrier(
        self,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_pipeline_barrier(
                self.ptr as *mut _ as _,
                src_stage_mask.bits(),
                dst_stage_mask.bits(),
                dependency_flags.bits(),
                memory_barriers.len() as _,
                slice_as_ptr_empty_null(memory_barriers),
                buffer_memory_barriers.len() as _,
                slice_as_ptr_empty_null(buffer_memory_barriers) as _,
                image_memory_barriers.len() as _,
                slice_as_ptr_empty_null(image_memory_barriers) as _,
            );
        }

        self
    }

    /// Insert a memory dependency
    #[cfg(feature = "Allow1_3APIs")]
    #[inline(always)]
    pub fn pipeline_barrier_2(self, dependency_info: &crate::DependencyInfo) -> Self {
        unsafe {
            crate::vkfn::cmd_pipeline_barrier2(self.ptr as *mut _ as _, dependency_info as *const _ as _);
        }

        self
    }
}

/// Graphics/Compute Commands: Querying
#[implements]
impl CmdRecord<'_> {
    /// Begin a query
    #[inline(always)]
    pub fn begin_query(
        self,
        pool: &(impl VkHandle<Handle = VkQueryPool> + ?Sized),
        query: u32,
        flags: VkQueryControlFlags,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_begin_query(self.ptr as *mut _ as _, pool.native_ptr(), query, flags);
        }

        self
    }

    /// Ends a query
    #[inline(always)]
    pub fn end_query(self, pool: &(impl VkHandle<Handle = VkQueryPool> + ?Sized), query: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_end_query(self.ptr as *mut _ as _, pool.native_ptr(), query);
        }

        self
    }

    /// Reset queries in a query pool
    #[inline(always)]
    pub fn reset_query_pool(self, pool: &(impl VkHandle<Handle = VkQueryPool> + ?Sized), range: Range<u32>) -> Self {
        unsafe {
            crate::vkfn::cmd_reset_query_pool(
                self.ptr as *mut _ as _,
                pool.native_ptr(),
                range.start,
                range.end - range.start,
            );
        }
        self
    }

    /// Write a device timestamp into a query object
    #[inline(always)]
    pub fn write_timestamp(
        self,
        stage: PipelineStageFlags,
        pool: &(impl VkHandle<Handle = VkQueryPool> + ?Sized),
        query: u32,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_write_timestamp(self.ptr as *mut _ as _, stage.bits(), pool.native_ptr(), query);
        }
        self
    }

    /// Copy the results of queries in a query pool to a buffer object
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub fn copy_query_pool_results(
        self,
        pool: &(impl VkHandle<Handle = VkQueryPool> + ?Sized),
        range: Range<u32>,
        dst: &Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_copy_query_pool_results(
                self.ptr as *mut _ as _,
                pool.native_ptr(),
                range.start,
                range.end - range.start,
                dst as *const _ as _,
                dst_offset,
                stride,
                flags.bits(),
            );
        }
        self
    }
}

/// Graphics/Compute Commands: Miscellaneous
#[implements]
impl CmdRecord<'_> {
    /// Inject imperative command generation in method-chaining
    #[inline(always)]
    pub fn inject(self, op: impl FnOnce(Self) -> Self) -> Self {
        op(self)
    }
}

/// A color value representation for clearing operations.
/// Constructable from RGBA values using `From::from`.
pub type ClearColorValue = VkClearColorValue;
impl From<[f32; 4]> for ClearColorValue {
    #[inline(always)]
    fn from(c: [f32; 4]) -> Self {
        VkClearColorValue { float32: c }
    }
}
impl From<[i32; 4]> for ClearColorValue {
    #[inline(always)]
    fn from(c: [i32; 4]) -> Self {
        VkClearColorValue { int32: c }
    }
}
impl From<[u32; 4]> for ClearColorValue {
    #[inline(always)]
    fn from(c: [u32; 4]) -> Self {
        VkClearColorValue { uint32: c }
    }
}

pub type ClearValue = VkClearValue;
impl ClearValue {
    /// Constructs a `ClearValue` which represents clearing color value
    #[inline(always)]
    pub fn color(c: impl Into<ClearColorValue>) -> Self {
        Self { color: c.into() }
    }

    /// Constructs a `ClearValue` which represents clearing color value
    pub const fn color_f32(c: [f32; 4]) -> Self {
        Self {
            color: VkClearColorValue { float32: c },
        }
    }
    /// Constructs a `ClearValue` which represents clearing color value
    pub const fn color_u32(c: [u32; 4]) -> Self {
        Self {
            color: VkClearColorValue { uint32: c },
        }
    }
    /// Constructs a `ClearValue` which represents clearing color value
    pub const fn color_i32(c: [i32; 4]) -> Self {
        Self {
            color: VkClearColorValue { int32: c },
        }
    }
    /// Constructs a `ClearValue` which represents clearing both depth and stencil values
    pub const fn depth_stencil(depth: f32, stencil: u32) -> Self {
        Self {
            depthStencil: VkClearDepthStencilValue { depth, stencil },
        }
    }
}

/// Type of index buffer indices
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexType {
    /// Indices are 8-bit unsigned integer values
    #[cfg(feature = "VK_KHR_index_type_uint8")]
    U8 = VK_INDEX_TYPE_UINT8_KHR,
    /// Indices are 16-bit unsigned integer values
    U16 = VK_INDEX_TYPE_UINT16,
    /// Indices are 32-bit unsigned integer values
    U32 = VK_INDEX_TYPE_UINT32,
}

/// Enabling or disabling the occlusion query
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcclusionQuery {
    Disable,
    Enable,
    /// `VK_QUERY_CONTROL_PRECISE_BIT`
    Precise,
}

/// Access Types
pub struct AccessFlags {
    pub read: VkAccessFlags,
    pub write: VkAccessFlags,
}
impl AccessFlags {
    /// Specifies read access to an indirect command structure read as part of an indirect drawing or dispatch command.
    pub const INDIRECT_COMMAND_READ: VkAccessFlags = VK_ACCESS_INDIRECT_COMMAND_READ_BIT;
    /// Specifies read access to an index buffer as part of an indexed drawing command, bound by `vkCmdBindIndexBuffer`.
    pub const INDEX_READ: VkAccessFlags = VK_ACCESS_INDEX_READ_BIT;
    /// Specifies read access to a vertex buffer as part of a drawing command, bound by `vkCmdBindVertexBuffers`.
    pub const VERTEX_ATTRIBUTE_READ: VkAccessFlags = VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT;
    /// Specifies read access to a [uniform buffer](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-uniformbuffer).
    pub const UNIFORM_READ: VkAccessFlags = VK_ACCESS_UNIFORM_READ_BIT;
    /// Specifies read access to an [input attachment](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#renderpass) within a render pass during fragment shading.
    pub const INPUT_ATTACHMENT_READ: VkAccessFlags = VK_ACCESS_INPUT_ATTACHMENT_READ_BIT;
    /// Specifies read/write access to a [storage buffer](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-storagebuffer),
    /// [uniform texel buffer](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-uniformtexelbuffer)(read only),
    /// [storage texel buffer](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-storagetexelbuffer),
    /// [samples image](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-sampledimage)(read only),
    /// or [storage image](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#descriptorsets-storageimage).
    pub const SHADER: Self = AccessFlags {
        read: VK_ACCESS_SHADER_READ_BIT,
        write: VK_ACCESS_SHADER_WRITE_BIT,
    };
    /// - `read`: Specifies read access to a [color attachment](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#renderpass),
    ///   such as via [blending](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#framebuffer-blending),
    ///   [logic operations](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#framebuffer-logicop),
    ///   or via certain [subpass load operations](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#framebuffer-logicop).
    /// - `write`: specifies write access to a [color or resolve attachment](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#renderpass)
    ///   during a [render pass](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#renderpass)
    ///   or via certain [subpass load and store operations](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#renderpass-load-store-ops).
    pub const COLOR_ATTACHMENT: Self = AccessFlags {
        read: VK_ACCESS_COLOR_ATTACHMENT_READ_BIT,
        write: VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
    };
    /// - `read`: Specifies read access to a [depth/stencil attachment](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#renderpass),
    ///   via [depth or stencil operations](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#fragops-ds-state)
    ///   or via certain [subpass load operations](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#renderpass-load-store-ops).
    /// - `write`: Specifies write access to a [depth/stencil attachment](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#renderpass),
    ///   via [depth or stencil operations](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#fragops-ds-state)
    ///   or via certain [subpass load and store operations](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#renderpass-load-store-ops).
    pub const DEPTH_STENCIL_ATTACHMENT: Self = AccessFlags {
        read: VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT,
        write: VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
    };
    /// Specifies read/write access to an image or buffer in a [clear](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#clears)(write only)
    /// or [copy](https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#copies) operation.
    pub const TRANSFER: Self = AccessFlags {
        read: VK_ACCESS_TRANSFER_READ_BIT,
        write: VK_ACCESS_TRANSFER_WRITE_BIT,
    };
    /// Specifies read/write access by a host operation.
    /// Accesses of this type are not performed through a resource, but directly on memory.
    pub const HOST: Self = AccessFlags {
        read: VK_ACCESS_HOST_READ_BIT,
        write: VK_ACCESS_HOST_WRITE_BIT,
    };
    /// Specifies read/write access via non-specific entities.
    /// These entities include the Vulkan device and host, but *may* also include entities external to the Vulkan device
    /// or otherwise not part of the core Vulkan pipeline.
    ///
    /// - When the `write` mask included in a source access mask, all writes that are performed by entities known to the
    ///   Vulkan device are made available.
    /// - When included in a destination access mask, makes all available writes visible to all future read accesses on
    ///   entities known to the Vulkan device.
    pub const MEMORY: Self = AccessFlags {
        read: VK_ACCESS_MEMORY_READ_BIT,
        write: VK_ACCESS_MEMORY_WRITE_BIT,
    };
}

/// Wrapper object of `VkImageMemoryBarrier`, derscribes a memory barrier of an image.
#[derive(Clone)]
#[repr(transparent)]
pub struct ImageMemoryBarrier(VkImageMemoryBarrier);
impl ImageMemoryBarrier {
    /// Construct a new barrier descriptor from discrete pair of resource and subresource range
    pub fn new(
        res: &(impl VkHandle<Handle = VkImage> + ?Sized),
        subres: impl Into<VkImageSubresourceRange>,
        trans: LayoutTransition,
    ) -> Self {
        Self(VkImageMemoryBarrier {
            sType: VkImageMemoryBarrier::TYPE,
            pNext: core::ptr::null(),
            image: res.native_ptr(),
            subresourceRange: subres.into(),
            oldLayout: trans.from as _,
            newLayout: trans.to as _,
            srcAccessMask: trans.from.default_access_mask(),
            dstAccessMask: trans.to.default_access_mask(),
            srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
        })
    }

    /// Update the source access mask
    #[inline]
    pub fn src_access_mask(mut self, mask: VkAccessFlags) -> Self {
        self.0.srcAccessMask = mask;
        self
    }

    /// Update the destination access mask
    #[inline]
    pub fn dest_access_mask(mut self, mask: VkAccessFlags) -> Self {
        self.0.dstAccessMask = mask;
        self
    }

    /// Update the access mask transition
    #[inline]
    pub fn access_mask_transition(mut self, src: VkAccessFlags, dst: VkAccessFlags) -> Self {
        self.0.srcAccessMask = src;
        self.0.dstAccessMask = dst;
        self
    }

    /// Flip access masks and image layouts
    #[inline]
    pub fn flip(mut self) -> Self {
        core::mem::swap(&mut self.0.srcAccessMask, &mut self.0.dstAccessMask);
        core::mem::swap(&mut self.0.oldLayout, &mut self.0.newLayout);
        self
    }
}
impl From<VkImageMemoryBarrier> for ImageMemoryBarrier {
    #[inline]
    fn from(v: VkImageMemoryBarrier) -> Self {
        Self(v)
    }
}
impl From<ImageMemoryBarrier> for VkImageMemoryBarrier {
    #[inline]
    fn from(v: ImageMemoryBarrier) -> Self {
        v.0
    }
}

pub type MemoryBarrier = VkMemoryBarrier;

/// Wrapper object of `VkBufferMemoryBarrier`, describes a memory barrier of a buffer.
#[derive(Clone)]
#[repr(transparent)]
pub struct BufferMemoryBarrier(VkBufferMemoryBarrier);
impl BufferMemoryBarrier {
    /// Construct a new buffer descriptor
    pub fn new(
        buf: &(impl VkHandle<Handle = VkBuffer> + ?Sized),
        range: core::ops::Range<VkDeviceSize>,
        src_access_mask: VkAccessFlags,
        dst_access_mask: VkAccessFlags,
    ) -> Self {
        Self(VkBufferMemoryBarrier {
            sType: VkBufferMemoryBarrier::TYPE,
            pNext: core::ptr::null(),
            buffer: buf.native_ptr(),
            offset: range.start,
            size: range.end - range.start,
            srcAccessMask: src_access_mask,
            dstAccessMask: dst_access_mask,
            srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
        })
    }

    /// Update the source access mask
    #[inline]
    pub fn src_access_mask(mut self, mask: VkAccessFlags) -> Self {
        self.0.srcAccessMask = mask;
        self
    }

    /// Update the destination access mask
    #[inline]
    pub fn dest_access_mask(mut self, mask: VkAccessFlags) -> Self {
        self.0.dstAccessMask = mask;
        self
    }

    /// Update the access mask transition
    #[inline]
    pub fn access_mask_transition(self, src: VkAccessFlags, dst: VkAccessFlags) -> Self {
        self.src_access_mask(src).dest_access_mask(dst)
    }

    /// Flip access masks
    #[inline]
    pub fn flip(mut self) -> Self {
        core::mem::swap(&mut self.0.srcAccessMask, &mut self.0.dstAccessMask);
        self
    }
}
impl From<VkBufferMemoryBarrier> for BufferMemoryBarrier {
    #[inline]
    fn from(v: VkBufferMemoryBarrier) -> Self {
        BufferMemoryBarrier(v)
    }
}
impl From<BufferMemoryBarrier> for VkBufferMemoryBarrier {
    #[inline]
    fn from(v: BufferMemoryBarrier) -> Self {
        v.0
    }
}

#[repr(transparent)]
pub struct BufferCopy(pub VkBufferCopy);
impl BufferCopy {
    #[inline(always)]
    pub const fn mirror(offset: VkDeviceSize, size: VkDeviceSize) -> Self {
        Self(VkBufferCopy {
            srcOffset: offset,
            dstOffset: offset,
            size,
        })
    }

    #[inline(always)]
    pub const fn mirror_data<T>(offset: VkDeviceSize) -> Self {
        Self::mirror(offset, core::mem::size_of::<T>() as _)
    }

    #[inline(always)]
    pub const fn copy_data<T>(src_offset: VkDeviceSize, dst_offset: VkDeviceSize) -> Self {
        Self(VkBufferCopy {
            srcOffset: src_offset,
            dstOffset: dst_offset,
            size: core::mem::size_of::<T>() as _,
        })
    }
}
impl From<VkBufferCopy> for BufferCopy {
    fn from(value: VkBufferCopy) -> Self {
        Self(value)
    }
}
impl From<BufferCopy> for VkBufferCopy {
    fn from(value: BufferCopy) -> Self {
        value.0
    }
}

pub type ImageCopy = VkImageCopy;
pub type ImageBlit = VkImageBlit;
pub type BufferImageCopy = VkBufferImageCopy;
pub type ImageResolve = VkImageResolve;
