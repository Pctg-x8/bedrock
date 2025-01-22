//! Vulkan Commands

use derives::implements;

use crate::{
    ffi_helper::ArrayFFIExtensions, vk::*, DescriptorSet, DeviceChild, DeviceChildHandle, LayoutTransition,
    VkDeviceChildNonExtDestroyable, VkHandleMut, VkHandleRef, VkObject, VkRawHandle, VulkanStructure,
};
#[implements]
use crate::{FilterMode, PipelineStageFlags, QueryPipelineStatisticFlags, QueryResultFlags, StencilFaceMask};
use crate::{ImageLayout, VkHandle};
use std::mem::replace;
#[implements]
use std::mem::{size_of, transmute};
use std::ops::Range;

#[derive(VkHandle, VkObject)]
#[VkObject(type = VkCommandPool::OBJECT_TYPE)]
pub struct CommandPoolObject<Device: VkHandle<Handle = VkDevice>>(pub(crate) VkCommandPool, pub(crate) Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for CommandPoolObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for CommandPoolObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for CommandPoolObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for CommandPoolObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for CommandPoolObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: crate::Device> CommandPool for CommandPoolObject<Device> {}
impl<Device: crate::Device> CommandPoolMut for CommandPoolObject<Device> {}
impl<Device: crate::Device> CommandPoolObject<Device> {
    /// Create a new command pool object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline]
    pub fn new(device: Device, info: &CommandPoolCreateInfo) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_command_pool(device.native_ptr(), &info.0, core::ptr::null(), h.as_mut_ptr())
                .into_result()?;

            Ok(Self::manage(h.assume_init(), device))
        }
    }

    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkCommandPool, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkCommandPool, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Device: VkHandle<Handle = VkDevice> + Clone> CommandPoolObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> CommandPoolObject<Device> {
        CommandPoolObject(self.0, self.1.clone())
    }
}

/// Opaque handle to a command buffer object
#[repr(transparent)]
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkCommandBuffer::OBJECT_TYPE)]
pub struct CommandBufferObject<Device>(VkCommandBuffer, core::marker::PhantomData<Device>);
impl<Device> Clone for CommandBufferObject<Device> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0, core::marker::PhantomData)
    }
}
impl<Device> Copy for CommandBufferObject<Device> {}
unsafe impl<Device: Sync> Sync for CommandBufferObject<Device> {}
unsafe impl<Device: Send> Send for CommandBufferObject<Device> {}
impl<Device: crate::Device> CommandBuffer for CommandBufferObject<Device> {}
impl<Device: crate::Device> CommandBufferMut for CommandBufferObject<Device> {}
impl<Device: crate::Device> CommandBufferObject<Device> {
    /// Allocate command buffers from an existing command pool
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline]
    pub fn alloc(device: Device, info: &CommandBufferAllocateInfo) -> crate::Result<Vec<Self>> {
        let mut hs = vec![VkCommandBuffer::NULL; info.0.commandBufferCount as _];

        unsafe {
            crate::vkfn::allocate_command_buffers(device.native_ptr(), &info.0, hs.as_mut_ptr()).into_result()?;

            Ok(transmute(hs))
        }
    }

    /// Allocate a static amount of command buffers from an existing command pool
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    #[inline]
    pub fn alloc_array<const N: usize>(
        device: Device,
        info: &CommandBufferFixedCountAllocateInfo<'_, N>,
    ) -> crate::Result<[Self; N]> {
        let mut hs = [Self(VkCommandBuffer::NULL, core::marker::PhantomData); N];

        unsafe {
            crate::vkfn::allocate_command_buffers(device.native_ptr(), &info.0, hs.as_mut_ptr() as _).into_result()?;

            Ok(hs)
        }
    }
}

/// The recording state of command buffers
#[implements]
#[must_use = "CmdRecord must be consumed by end() (not closed automatically by drop!)"]
pub struct CmdRecord<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, Device: 'd + ?Sized> {
    ptr: &'d mut CommandBuffer,
    device: &'d Device,
}
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer>, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    #[inline(always)]
    pub fn as_dyn_ref<'r>(&'r mut self) -> CmdRecord<'r, dyn VkHandleMut<Handle = VkCommandBuffer> + 'r, Device> {
        CmdRecord {
            ptr: self.ptr as _,
            device: self.device,
        }
    }
}

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

pub trait CommandPool: VkHandle<Handle = VkCommandPool> + DeviceChild {}
DerefContainerBracketImpl!(for CommandPool {});
GuardsImpl!(for CommandPool {});

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommandBufferLevel {
    Primary = VK_COMMAND_BUFFER_LEVEL_PRIMARY,
    Secondary = VK_COMMAND_BUFFER_LEVEL_SECONDARY,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandBufferAllocateInfo<'r>(
    VkCommandBufferAllocateInfo,
    core::marker::PhantomData<&'r mut dyn VkHandleMut<Handle = VkCommandPool>>,
);
impl<'r> CommandBufferAllocateInfo<'r> {
    #[inline(always)]
    pub fn new(
        command_pool: &'r mut (impl VkHandleMut<Handle = VkCommandPool> + ?Sized),
        count: u32,
        level: CommandBufferLevel,
    ) -> Self {
        Self(
            VkCommandBufferAllocateInfo {
                sType: VkCommandBufferAllocateInfo::TYPE,
                pNext: core::ptr::null(),
                commandPool: command_pool.native_ptr_mut(),
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
    VkCommandBufferAllocateInfo,
    core::marker::PhantomData<&'r mut dyn VkHandleMut<Handle = VkCommandPool>>,
);
impl<'r, const N: usize> CommandBufferFixedCountAllocateInfo<'r, N> {
    #[inline(always)]
    pub fn new(
        command_pool: &'r mut (impl VkHandleMut<Handle = VkCommandPool> + ?Sized),
        level: CommandBufferLevel,
    ) -> Self {
        assert!(N <= u32::MAX as usize, "too many command buffers will be allocated");

        Self(
            VkCommandBufferAllocateInfo {
                sType: VkCommandBufferAllocateInfo::TYPE,
                pNext: core::ptr::null(),
                commandPool: command_pool.native_ptr_mut(),
                level: level as _,
                commandBufferCount: N as _,
            },
            core::marker::PhantomData,
        )
    }
}

pub trait CommandPoolMut: CommandPool + VkHandleMut {
    /// Resets a command pool
    /// # Safety
    /// Application cannot use command buffers after this call
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements]
    unsafe fn reset(&mut self, flags: VkCommandPoolResetFlags) -> crate::Result<()> {
        crate::vkfn::reset_command_pool(self.device_handle(), self.native_ptr_mut(), flags)
            .into_result()
            .map(drop)
    }

    /// Free command buffers
    /// # Safety
    /// Each member of `buffers` must be externally synchronized
    #[implements]
    unsafe fn free(&mut self, buffers: &[impl CommandBuffer]) {
        crate::vkfn::free_command_buffers(
            self.device().native_ptr(),
            self.native_ptr_mut(),
            buffers.len() as _,
            buffers.as_ptr_empty_null() as *const _,
        );
    }

    /// Trim a command pool
    #[implements("VK_KHR_maintenance1")]
    fn trim(&mut self) {
        #[cfg(feature = "Allow1_1APIs")]
        unsafe {
            crate::vkfn::trim_command_pool(self.device_handle(), self.native_ptr_mut(), 0);
        }
        #[cfg(not(feature = "Allow1_1APIs"))]
        unsafe {
            use crate::Device;

            self.device().get_trim_command_pool_khr_fn().0(self.device_handle(), self.native_ptr_mut(), 0);
        }
    }
}
DerefContainerBracketImpl!(for mut CommandPoolMut {});
GuardsImpl!(for mut CommandPoolMut {});

pub trait CommandBuffer: VkHandle<Handle = VkCommandBuffer> {}
DerefContainerBracketImpl!(for CommandBuffer {});
GuardsImpl!(for CommandBuffer {});

pub trait CommandBufferMut: CommandBuffer + VkHandleMut {
    /// Start recording a command buffer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///
    /// # Safety
    /// The `CommandPool` that this commandBuffer was allocated from must be externally synchronized.
    #[implements]
    unsafe fn begin_raw<'d, Device: 'd + crate::Device + ?Sized>(
        &'d mut self,
        info: &VkCommandBufferBeginInfo,
        device: &'d Device,
    ) -> crate::Result<CmdRecord<'d, Self, Device>> {
        crate::vkfn::begin_command_buffer(self.native_ptr_mut(), info)
            .into_result()
            .map(move |_| CmdRecord { ptr: self, device })
    }

    /// Start recording a primary command buffer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Safety
    /// The `CommandPool` that this commandBuffer was allocated from must be externally synchronized.
    #[implements]
    unsafe fn begin<'d, Device: 'd + crate::Device + ?Sized>(
        &'d mut self,
        device: &'d Device,
    ) -> crate::Result<CmdRecord<'d, Self, Device>> {
        let info = VkCommandBufferBeginInfo {
            sType: VkCommandBufferBeginInfo::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            pInheritanceInfo: core::ptr::null(),
        };

        self.begin_raw(&info, device)
    }

    /// Start recording a primary command buffer that will be submitted once
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Safety
    /// The `CommandPool` that this commandBuffer was allocated from must be externally synchronized.
    #[implements]
    unsafe fn begin_once<'d, Device: 'd + crate::Device + ?Sized>(
        &'d mut self,
        device: &'d Device,
    ) -> crate::Result<CmdRecord<'d, Self, Device>> {
        let info = VkCommandBufferBeginInfo {
            sType: VkCommandBufferBeginInfo::TYPE,
            pNext: core::ptr::null(),
            flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
            pInheritanceInfo: std::ptr::null(),
        };

        self.begin_raw(&info, device)
    }

    /// Start recording a secondary command buffer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Safety
    /// The `CommandPool` that this commandBuffer was allocated from must be externally synchronized.
    #[implements]
    unsafe fn begin_inherit<'d, Device: 'd + crate::Device + ?Sized>(
        &'d mut self,
        device: &'d Device,
        renderpass: Option<(
            Option<&(impl crate::Framebuffer + ?Sized)>,
            &(impl crate::RenderPass + ?Sized),
            u32,
        )>,
        query: Option<(OcclusionQuery, QueryPipelineStatisticFlags)>,
    ) -> crate::Result<CmdRecord<'d, Self, Device>> {
        use crate::VkRawHandle;

        let flags = if renderpass.is_some() {
            VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT
        } else {
            0
        };
        let (fb, rp, s) = match renderpass {
            Some((f, r, s)) => (f.map_or(VkFramebuffer::NULL, |x| x.native_ptr()), r.native_ptr(), s),
            None => (VkFramebuffer::NULL, VkRenderPass::NULL, 0),
        };
        let (oq, psq) = query.map_or((OcclusionQuery::Disable, 0), |(o, p)| (o, p.0));
        let inherit = VkCommandBufferInheritanceInfo {
            sType: VkCommandBufferInheritanceInfo::TYPE,
            pNext: core::ptr::null(),
            framebuffer: fb,
            renderPass: rp,
            subpass: s,
            occlusionQueryEnable: (oq != OcclusionQuery::Disable) as _,
            queryFlags: if oq == OcclusionQuery::Precise {
                VK_QUERY_CONTROL_PRECISE_BIT
            } else {
                0
            },
            pipelineStatistics: psq,
        };
        let binfo = VkCommandBufferBeginInfo {
            sType: VkCommandBufferBeginInfo::TYPE,
            pNext: core::ptr::null(),
            flags,
            pInheritanceInfo: &inherit,
        };

        self.begin_raw(&binfo, device)
    }

    /// Reset a command buffer to the initial state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Safety
    /// The `CommandPool` that this commandBuffer was allocated from must be externally synchronized.
    #[implements]
    unsafe fn reset(&mut self, release_resources: bool) -> crate::Result<()> {
        let flags = if release_resources {
            VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT
        } else {
            0
        };

        crate::vkfn::reset_command_buffer(self.native_ptr_mut(), flags)
            .into_result()
            .map(drop)
    }

    /// Locking CommandBuffer with CommandPool to satisfy externally synchronization restriction.
    /// # Safety
    /// This command buffer must be allocated from `pool`.
    unsafe fn synchronize_with<'p, 'b: 'p, Pool: 'p + crate::CommandPoolMut + ?Sized>(
        &'b mut self,
        pool: &'p mut Pool,
    ) -> SynchronizedCommandBuffer<'p, 'b, Pool, Self> {
        SynchronizedCommandBuffer { pool, buffer: self }
    }
}
DerefContainerBracketImpl!(for mut CommandBufferMut {});
GuardsImpl!(for mut CommandBufferMut {});

pub struct SynchronizedCommandBuffer<
    'p,
    'b: 'p,
    Pool: crate::CommandPoolMut + ?Sized + 'p,
    Buffer: crate::CommandBufferMut + ?Sized + 'b,
> {
    pool: &'p mut Pool,
    buffer: &'b mut Buffer,
}
#[implements]
impl<'p, 'b: 'p, Pool: crate::CommandPoolMut + 'p, Buffer: crate::CommandBufferMut + 'b>
    SynchronizedCommandBuffer<'p, 'b, Pool, Buffer>
{
    /// Start recording a command buffer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn begin_raw(
        &mut self,
        info: &VkCommandBufferBeginInfo,
    ) -> crate::Result<CmdRecord<Buffer, Pool::ConcreteDevice>> {
        unsafe { self.buffer.begin_raw(info, self.pool.device()) }
    }

    /// Start recording a primary command buffer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn begin(&mut self) -> crate::Result<CmdRecord<Buffer, Pool::ConcreteDevice>> {
        unsafe { self.buffer.begin(self.pool.device()) }
    }

    /// Start recording a primary command buffer that will be submitted once
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn begin_once(&mut self) -> crate::Result<CmdRecord<Buffer, Pool::ConcreteDevice>> {
        unsafe { self.buffer.begin_once(self.pool.device()) }
    }

    /// Start recording a secondary command buffer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn begin_inherit(
        &mut self,
        renderpass: Option<(
            Option<&(impl crate::Framebuffer + ?Sized)>,
            &(impl crate::RenderPass + ?Sized),
            u32,
        )>,
        query: Option<(OcclusionQuery, QueryPipelineStatisticFlags)>,
    ) -> crate::Result<CmdRecord<Buffer, Pool::ConcreteDevice>> {
        unsafe { self.buffer.begin_inherit(self.pool.device(), renderpass, query) }
    }

    /// Reset a command buffer to the initial state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn reset(&mut self, release_resources: bool) -> crate::Result<()> {
        unsafe { self.buffer.reset(release_resources) }
    }
}

/// Common Commands: End Recording
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Finish recording a command buffer
    pub fn end(self) -> crate::Result<()> {
        unsafe {
            crate::vkfn::end_command_buffer(self.ptr.native_ptr())
                .into_result()
                .map(drop)
        }
    }
}

/// Graphics Commands: Manipulating with Render Passes
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Begin a new render pass
    #[inline]
    pub fn begin_render_pass(
        self,
        pass: &(impl crate::RenderPass + ?Sized),
        framebuffer: &(impl crate::Framebuffer + ?Sized),
        render_area: VkRect2D,
        clear_values: &[ClearValue],
        inline_commands: bool,
    ) -> Self {
        let binfo = VkRenderPassBeginInfo {
            sType: VkRenderPassBeginInfo::TYPE,
            pNext: core::ptr::null(),
            renderPass: pass.native_ptr(),
            framebuffer: framebuffer.native_ptr(),
            renderArea: render_area,
            clearValueCount: clear_values.len() as _,
            pClearValues: clear_values.as_ptr_empty_null(),
        };
        let contents = if inline_commands {
            VK_SUBPASS_CONTENTS_INLINE
        } else {
            VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS
        };
        unsafe {
            crate::vkfn::cmd_begin_render_pass(self.ptr.native_ptr_mut(), &binfo, contents);
        }

        self
    }

    /// Transition to the next subpass of a render pass
    #[inline]
    pub fn next_subpass(self, inline_commands: bool) -> Self {
        let contents = if inline_commands {
            VK_SUBPASS_CONTENTS_INLINE
        } else {
            VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS
        };
        unsafe {
            crate::vkfn::cmd_next_subpass(self.ptr.native_ptr_mut(), contents);
        }

        self
    }

    /// End the current render pass
    #[inline]
    pub fn end_render_pass(self) -> Self {
        unsafe { crate::vkfn::cmd_end_render_pass(self.ptr.native_ptr_mut()) };

        self
    }

    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[inline]
    pub fn begin_render_pass_2(
        self,
        begin_info: &crate::RenderPassBeginInfo<'_, impl crate::RenderPass + ?Sized, impl crate::Framebuffer + ?Sized>,
        subpass_begin_info: &VkSubpassBeginInfoKHR,
    ) -> Self {
        #[cfg(feature = "Allow1_3APIs")]
        unsafe {
            crate::vkfn::cmd_begin_render_pass2(self.ptr.native_ptr(), begin_info.as_ref(), subpass_begin_info);
        }

        #[cfg(not(feature = "Allow1_3APIs"))]
        unsafe {
            (self.device.cmd_begin_render_pass_2_khr_fn().0)(
                self.ptr.native_ptr(),
                begin_info.as_ref(),
                subpass_begin_info,
            );
        }

        self
    }

    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[inline]
    pub fn next_subpass_2(
        self,
        subpass_begin_info: &VkSubpassBeginInfoKHR,
        subpass_end_info: &VkSubpassEndInfoKHR,
    ) -> Self
    where
        Device: crate::Device,
    {
        #[cfg(feature = "Allow1_3APIs")]
        unsafe {
            crate::vkfn::cmd_next_subpass2(self.ptr.native_ptr(), subpass_begin_info, subpass_end_info);
        }

        #[cfg(not(feature = "Allow1_3APIs"))]
        unsafe {
            (self.device.cmd_next_subpass_2_khr_fn().0)(self.ptr.native_ptr(), subpass_begin_info, subpass_end_info);
        }

        self
    }

    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[inline]
    pub fn end_render_pass_2(self, subpass_end_info: &VkSubpassEndInfoKHR) -> Self {
        #[cfg(feature = "Allow1_3APIs")]
        unsafe {
            crate::vkfn::cmd_end_render_pass2(self.ptr.native_ptr(), subpass_end_info);
        }

        #[cfg(not(feature = "Allow1_3APIs"))]
        unsafe {
            (self.device.cmd_end_render_pass_2_khr_fn().0)(self.ptr.native_ptr(), subpass_end_info);
        }

        self
    }
}

/// Graphics/Compute Commands: Pipeline Setup
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Bind a pipeline object to a command buffer
    pub fn bind_graphics_pipeline(self, pipeline: &(impl VkHandle<Handle = VkPipeline> + ?Sized)) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_pipeline(
                self.ptr.native_ptr_mut(),
                VK_PIPELINE_BIND_POINT_GRAPHICS,
                pipeline.native_ptr(),
            );
        }
        self
    }

    /// Bind a pipeline object to a command buffer
    pub fn bind_compute_pipeline(self, pipeline: &(impl VkHandle<Handle = VkPipeline> + ?Sized)) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_pipeline(
                self.ptr.native_ptr_mut(),
                VK_PIPELINE_BIND_POINT_COMPUTE,
                pipeline.native_ptr(),
            );
        }
        self
    }

    /// Binds descriptor sets to a command buffer
    pub fn bind_graphics_descriptor_sets(
        self,
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
        first: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_descriptor_sets(
                self.ptr.native_ptr_mut(),
                VK_PIPELINE_BIND_POINT_GRAPHICS,
                pipeline_layout.native_ptr(),
                first,
                descriptor_sets.len() as _,
                descriptor_sets.as_ptr_empty_null() as _,
                dynamic_offsets.len() as _,
                dynamic_offsets.as_ptr_empty_null(),
            );
        }

        self
    }

    /// Binds descriptor sets to a command buffer
    pub fn bind_compute_descriptor_sets(
        self,
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
        first: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_descriptor_sets(
                self.ptr.native_ptr_mut(),
                VK_PIPELINE_BIND_POINT_COMPUTE,
                pipeline_layout.native_ptr(),
                first,
                descriptor_sets.len() as _,
                descriptor_sets.as_ptr_empty_null() as _,
                dynamic_offsets.len() as _,
                dynamic_offsets.as_ptr_empty_null(),
            );
        }
        self
    }

    /// Update the value of push constant
    pub fn push_constant<T>(
        self,
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
        stage: VkShaderStageFlags,
        offset: u32,
        value: &T,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_push_constants(
                self.ptr.native_ptr_mut(),
                pipeline_layout.native_ptr(),
                stage,
                offset,
                size_of::<T>() as _,
                value as *const T as *const _,
            );
        }
        self
    }

    /// Push descriptor updates into a command buffer
    #[cfg(feature = "VK_KHR_push_descriptor")]
    #[inline(always)]
    pub unsafe fn push_descriptor_set_raw(
        self,
        pipeline_bind_point: VkPipelineBindPoint,
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
        set: u32,
        writes: &[VkWriteDescriptorSet],
    ) -> Self {
        #[cfg(feature = "Allow1_4APIs")]
        crate::vkfn::cmd_push_descriptor_set(
            self.ptr.native_ptr_mut(),
            pipeline_bind_point,
            pipeline_layout.native_ptr(),
            set,
            writes.len() as _,
            writes.as_ptr_empty_null(),
        );
        #[cfg(not(feature = "Allow1_4APIs"))]
        (self.device.cmd_push_descriptor_set_khr_fn())(
            self.ptr.native_ptr_mut(),
            pipeline_bind_point,
            pipeline_layout.native_ptr(),
            set,
            writes.len() as _,
            writes.as_ptr_empty_null(),
        );

        self
    }

    /// Push descriptor updates into a command buffer
    #[cfg(feature = "VK_KHR_push_descriptor")]
    pub fn push_graphics_descriptor_set(
        self,
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
        set: u32,
        writes: &[crate::DescriptorSetWriteInfo],
    ) -> Self {
        let w = writes.iter().map(|x| x.make_structure()).collect::<Vec<_>>();

        unsafe { self.push_descriptor_set_raw(VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline_layout, set, &w) }
    }

    /// Push descriptor updates into a command buffer
    #[cfg(feature = "VK_KHR_push_descriptor")]
    pub fn push_compute_descriptor_set(
        self,
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
        set: u32,
        writes: &[crate::DescriptorSetWriteInfo],
    ) -> Self {
        let w = writes.iter().map(|x| x.make_structure()).collect::<Vec<_>>();

        unsafe { self.push_descriptor_set_raw(VK_PIPELINE_BIND_POINT_COMPUTE, pipeline_layout, set, &w) }
    }
}

/// Graphics Commands: Updating dynamic states
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Set the viewport on a command buffer
    #[inline(always)]
    pub fn set_viewport(self, first: u32, viewports: &[VkViewport]) -> Self {
        unsafe {
            crate::vkfn::cmd_set_viewport(
                self.ptr.native_ptr_mut(),
                first,
                viewports.len() as _,
                viewports.as_ptr_empty_null(),
            );
        }
        self
    }

    /// Set the dynamic scissor rectangles on a command buffer
    #[inline(always)]
    pub fn set_scissor(self, first: u32, scissors: &[VkRect2D]) -> Self {
        unsafe {
            crate::vkfn::cmd_set_scissor(
                self.ptr.native_ptr_mut(),
                first,
                scissors.len() as _,
                scissors.as_ptr_empty_null(),
            );
        }
        self
    }

    /// Set the dynamic line width state
    #[inline(always)]
    pub fn set_line_width(self, w: f32) -> Self {
        unsafe {
            crate::vkfn::cmd_set_line_width(self.ptr.native_ptr_mut(), w);
        }
        self
    }

    /// Set the depth bias dynamic state
    #[inline(always)]
    pub fn set_depth_bias(self, constant_factor: f32, clamp: f32, slope_factor: f32) -> Self {
        unsafe {
            crate::vkfn::cmd_set_depth_bias(self.ptr.native_ptr_mut(), constant_factor, clamp, slope_factor);
        }
        self
    }

    /// Set the values of blend constants
    #[inline(always)]
    pub fn set_blend_constants(self, blend_constants: &[f32; 4]) -> Self {
        unsafe {
            crate::vkfn::cmd_set_blend_constants(self.ptr.native_ptr_mut(), blend_constants.as_ptr());
        }
        self
    }

    /// Set the depth bounds test values for a command buffer
    #[inline(always)]
    pub fn set_depth_bounds(self, bounds: Range<f32>) -> Self {
        unsafe {
            crate::vkfn::cmd_set_depth_bounds(self.ptr.native_ptr_mut(), bounds.start, bounds.end);
        }
        self
    }

    /// Set the stencil compare mask dynamic state
    #[inline(always)]
    pub fn set_stencil_compare_mask(self, face_mask: StencilFaceMask, compare_mask: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_set_stencil_compare_mask(self.ptr.native_ptr_mut(), face_mask as _, compare_mask);
        }
        self
    }

    /// Set the stencil write mask dynamic state
    #[inline(always)]
    pub fn set_stencil_write_mask(self, face_mask: StencilFaceMask, write_mask: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_set_stencil_write_mask(self.ptr.native_ptr_mut(), face_mask as _, write_mask);
        }
        self
    }

    /// Set the stencil reference dynamic state
    #[inline(always)]
    pub fn set_stencil_reference(self, face_mask: StencilFaceMask, reference: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_set_stencil_reference(self.ptr.native_ptr_mut(), face_mask as _, reference);
        }
        self
    }

    /// Set the sample locations state
    #[cfg(feature = "VK_EXT_sample_locations")]
    #[inline(always)]
    pub fn set_sample_locations(self, _info: &VkSampleLocationsInfoEXT) -> Self {
        // unsafe {
        //     Resolver::get().cmd_set_sample_locations_ext(self.ptr.native_ptr_mut(), info as _);
        // }
        // self
        unimplemented!("acquire ext function in command recording");
    }
}

/// Graphics Commands: Binding Buffers
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Bind an index buffer to a command buffer
    #[inline(always)]
    pub fn bind_index_buffer(
        self,
        buffer: &(impl VkHandle<Handle = VkBuffer> + ?Sized),
        offset: usize,
        index_type: IndexType,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_index_buffer(
                self.ptr.native_ptr_mut(),
                buffer.native_ptr(),
                offset as _,
                index_type as _,
            );
        }
        self
    }

    /// Bind vertex buffers to a command buffer
    #[inline(always)]
    pub fn bind_vertex_buffers(self, first: u32, buffers: &[VkHandleRef<VkBuffer>], offsets: &[VkDeviceSize]) -> Self {
        assert_eq!(buffers.len(), offsets.len());

        unsafe {
            crate::vkfn::cmd_bind_vertex_buffers(
                self.ptr.native_ptr_mut(),
                first,
                buffers.len() as _,
                buffers.as_ptr_empty_null() as _,
                offsets.as_ptr_empty_null(),
            );
        }
        self
    }
}

/// Graphics Commands: Inside a Render Pass
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Draw primitives
    #[inline(always)]
    pub fn draw(self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_draw(
                self.ptr.native_ptr_mut(),
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
                self.ptr.native_ptr_mut(),
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
    pub fn draw_indirect(
        self,
        buffer: &(impl VkHandle<Handle = VkBuffer> + ?Sized),
        offset: VkDeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_draw_indirect(
                self.ptr.native_ptr_mut(),
                buffer.native_ptr(),
                offset,
                draw_count,
                stride,
            );
        }
        self
    }

    /// Perform an indexed indirect draw
    #[inline(always)]
    pub fn draw_indexed_indirect(
        self,
        buffer: &(impl VkHandle<Handle = VkBuffer> + ?Sized),
        offset: VkDeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_draw_indexed_indirect(
                self.ptr.native_ptr_mut(),
                buffer.native_ptr(),
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
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Dispatch compute work items
    #[inline(always)]
    pub fn dispatch(self, group_count_x: u32, group_count_y: u32, group_count_z: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_dispatch(self.ptr.native_ptr_mut(), group_count_x, group_count_y, group_count_z);
        }
        self
    }

    /// Dispatch compute work items using indirect parameters
    #[inline(always)]
    pub fn dispatch_indirect(
        self,
        buffer: &(impl crate::VkHandle<Handle = VkBuffer> + ?Sized),
        offset: VkDeviceSize,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_dispatch_indirect(self.ptr.native_ptr_mut(), buffer.native_ptr(), offset);
        }
        self
    }
}

/// Transfer Commands: Copying resources
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Copy data between buffer regions
    #[inline(always)]
    pub fn copy_buffer(
        self,
        src: &(impl crate::VkHandle<Handle = VkBuffer> + ?Sized),
        dst: &(impl crate::VkHandle<Handle = VkBuffer> + ?Sized),
        regions: &[BufferCopy],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_copy_buffer(
                self.ptr.native_ptr_mut(),
                src.native_ptr(),
                dst.native_ptr(),
                regions.len() as _,
                regions.as_ptr_empty_null() as _,
            );
        }
        self
    }

    /// Copy data between images
    #[inline(always)]
    pub fn copy_image(
        self,
        src: &(impl crate::VkHandle<Handle = VkImage> + ?Sized),
        src_layout: ImageLayout,
        dst: &(impl crate::VkHandle<Handle = VkImage> + ?Sized),
        dst_layout: ImageLayout,
        regions: &[VkImageCopy],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_copy_image(
                self.ptr.native_ptr_mut(),
                src.native_ptr(),
                src_layout as _,
                dst.native_ptr(),
                dst_layout as _,
                regions.len() as _,
                regions.as_ptr_empty_null(),
            );
        }
        self
    }

    /// Copy regions of an image, potentially performing format conversion
    #[inline(always)]
    pub fn blit_image(
        self,
        src: &(impl crate::VkHandle<Handle = VkImage> + ?Sized),
        src_layout: ImageLayout,
        dst: &(impl crate::VkHandle<Handle = VkImage> + ?Sized),
        dst_layout: ImageLayout,
        regions: &[VkImageBlit],
        filter: FilterMode,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_blit_image(
                self.ptr.native_ptr_mut(),
                src.native_ptr(),
                src_layout as _,
                dst.native_ptr(),
                dst_layout as _,
                regions.len() as _,
                regions.as_ptr_empty_null(),
                filter as _,
            );
        }
        self
    }

    /// Copy data from a buffer into an image
    #[inline(always)]
    pub fn copy_buffer_to_image(
        self,
        src_buffer: &(impl crate::VkHandle<Handle = VkBuffer> + ?Sized),
        dst_image: &(impl crate::VkHandle<Handle = VkImage> + ?Sized),
        dst_layout: ImageLayout,
        regions: &[VkBufferImageCopy],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_copy_buffer_to_image(
                self.ptr.native_ptr_mut(),
                src_buffer.native_ptr(),
                dst_image.native_ptr(),
                dst_layout as _,
                regions.len() as _,
                regions.as_ptr_empty_null(),
            );
        }
        self
    }

    /// Copy image data into a buffer
    #[inline(always)]
    pub fn copy_image_to_buffer(
        self,
        src_image: &(impl crate::VkHandle<Handle = VkImage> + ?Sized),
        src_layout: ImageLayout,
        dst_buffer: &(impl crate::VkHandle<Handle = VkBuffer> + ?Sized),
        regions: &[VkBufferImageCopy],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_copy_image_to_buffer(
                self.ptr.native_ptr_mut(),
                src_image.native_ptr(),
                src_layout as _,
                dst_buffer.native_ptr(),
                regions.len() as _,
                regions.as_ptr_empty_null(),
            );
        }
        self
    }

    /// Update a buffer's contents from host memory
    #[inline(always)]
    pub fn update_buffer<T>(
        self,
        dst: &(impl crate::VkHandle<Handle = VkBuffer> + ?Sized),
        dst_offset: VkDeviceSize,
        size: VkDeviceSize,
        data: &T,
    ) -> Self {
        assert!(
            size <= size_of::<T>() as VkDeviceSize,
            "Updated size exceeds size of datatype"
        );

        unsafe {
            crate::vkfn::cmd_update_buffer(
                self.ptr.native_ptr_mut(),
                dst.native_ptr(),
                dst_offset,
                size,
                data as *const T as *const _,
            );
        }
        self
    }
}

/// Graphics/Compute Commands: Transfer-like(clearing/filling) commands
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Fill a region of a buffer with a fixed value.
    /// `size` is number of bytes to fill
    #[inline(always)]
    pub fn fill_buffer(
        self,
        dst: &(impl crate::VkHandle<Handle = VkBuffer> + ?Sized),
        dst_offset: VkDeviceSize,
        size: VkDeviceSize,
        data: u32,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_fill_buffer(self.ptr.native_ptr_mut(), dst.native_ptr(), dst_offset, size, data);
        }
        self
    }

    /// Clear regions of a color image
    #[inline(always)]
    pub fn clear_color_image(
        self,
        image: &(impl crate::VkHandle<Handle = VkImage> + ?Sized),
        layout: ImageLayout,
        colors: &[ClearColorValue],
        ranges: &[VkImageSubresourceRange],
    ) -> Self {
        assert_eq!(colors.len(), ranges.len());

        unsafe {
            crate::vkfn::cmd_clear_color_image(
                self.ptr.native_ptr_mut(),
                image.native_ptr(),
                layout as _,
                colors.as_ptr_empty_null(),
                ranges.len() as _,
                ranges.as_ptr_empty_null(),
            );
        }
        self
    }

    /// Fill regions of a combined depth/stencil image
    #[inline(always)]
    pub fn clear_depth_stencil_image(
        self,
        image: &(impl crate::VkHandle<Handle = VkImage> + ?Sized),
        layout: ImageLayout,
        depth: f32,
        stencil: u32,
        ranges: &[VkImageSubresourceRange],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_clear_depth_stencil_image(
                self.ptr.native_ptr_mut(),
                image.native_ptr(),
                layout as _,
                &VkClearDepthStencilValue { depth, stencil },
                ranges.len() as _,
                ranges.as_ptr_empty_null(),
            );
        }
        self
    }

    /// Clear regions within currently bound framebuffer attachments
    #[inline(always)]
    pub fn clear_attachments(self, attachments: &[VkClearAttachment], rects: &[VkClearRect]) -> Self {
        unsafe {
            crate::vkfn::cmd_clear_attachments(
                self.ptr.native_ptr_mut(),
                attachments.len() as _,
                attachments.as_ptr_empty_null(),
                rects.len() as _,
                rects.as_ptr_empty_null(),
            );
        }
        self
    }
}

/// Graphics Commands: Executing Subcommands
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Execute a secondary command buffer from a primary command buffer
    /// # Safety
    ///
    /// Caller must be primary buffer and in the render pass when executing secondary command buffer
    #[inline(always)]
    pub unsafe fn execute_commands(self, buffers: &[VkCommandBuffer]) -> Self {
        crate::vkfn::cmd_execute_commands(
            self.ptr.native_ptr_mut(),
            buffers.len() as _,
            buffers.as_ptr_empty_null(),
        );
        self
    }
}

/// Graphics Commands: Resolving an image to another image
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Resolve regions of an image
    #[inline(always)]
    pub fn resolve_image(
        self,
        src: &(impl crate::VkHandle<Handle = VkImage> + ?Sized),
        src_layout: ImageLayout,
        dst: &(impl crate::VkHandle<Handle = VkImage> + ?Sized),
        dst_layout: ImageLayout,
        regions: &[VkImageResolve],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_resolve_image(
                self.ptr.native_ptr_mut(),
                src.native_ptr(),
                src_layout as _,
                dst.native_ptr(),
                dst_layout as _,
                regions.len() as _,
                regions.as_ptr_empty_null(),
            )
        };
        self
    }
}

/// Graphics/Compute Commands: Synchronization between command buffers/queues
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Set an event object to signaled state
    #[inline(always)]
    pub fn set_event(self, event: &(impl VkHandle<Handle = VkEvent> + ?Sized), stage_mask: PipelineStageFlags) -> Self {
        unsafe {
            crate::vkfn::cmd_set_event(self.ptr.native_ptr_mut(), event.native_ptr(), stage_mask.0);
        }
        self
    }

    /// Reset an event object to non-signaled state
    #[inline(always)]
    pub fn reset_event(
        self,
        event: &(impl VkHandle<Handle = VkEvent> + ?Sized),
        stage_mask: PipelineStageFlags,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_reset_event(self.ptr.native_ptr_mut(), event.native_ptr(), stage_mask.0);
        }
        self
    }

    /// Wait for one or more events and insert a set of memory
    #[inline(always)]
    pub fn wait_events(
        self,
        events: &[impl crate::Event],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[VkMemoryBarrier],
        buffer_memory_barriers: &[VkBufferMemoryBarrier],
        image_memory_barriers: &[VkImageMemoryBarrier],
    ) -> Self {
        let evs = events.iter().map(|e| e.native_ptr()).collect::<Vec<_>>();
        unsafe {
            crate::vkfn::cmd_wait_events(
                self.ptr.native_ptr_mut(),
                evs.len() as _,
                evs.as_ptr_empty_null(),
                src_stage_mask.0,
                dst_stage_mask.0,
                memory_barriers.len() as _,
                memory_barriers.as_ptr_empty_null(),
                buffer_memory_barriers.len() as _,
                buffer_memory_barriers.as_ptr_empty_null(),
                image_memory_barriers.len() as _,
                image_memory_barriers.as_ptr_empty_null(),
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
        by_region: bool,
        memory_barriers: &[VkMemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_pipeline_barrier(
                self.ptr.native_ptr_mut(),
                src_stage_mask.0,
                dst_stage_mask.0,
                if by_region { VK_DEPENDENCY_BY_REGION_BIT } else { 0 },
                memory_barriers.len() as _,
                memory_barriers.as_ptr_empty_null(),
                buffer_memory_barriers.len() as _,
                buffer_memory_barriers.as_ptr_empty_null() as _,
                image_memory_barriers.len() as _,
                image_memory_barriers.as_ptr_empty_null() as _,
            );
        }
        self
    }

    /// Insert a memory dependency
    #[cfg(all(feature = "VK_KHR_synchronization2", not(feature = "Allow1_3APIs")))]
    #[inline(always)]
    pub fn pipeline_barrier_2(self, dependency_info: &crate::DependencyInfo) -> Self
    where
        Device: crate::Device,
    {
        unsafe {
            (self.device.cmd_pipeline_barrier_2_khr_fn().0)(self.ptr.native_ptr_mut(), dependency_info as *const _ as _)
        }

        self
    }

    /// Insert a memory dependency
    #[cfg(feature = "Allow1_3APIs")]
    #[inline(always)]
    pub fn pipeline_barrier_2(self, dependency_info: &crate::DependencyInfo) -> Self {
        unsafe { crate::vkfn::cmd_pipeline_barrier2(self.ptr.native_ptr_mut(), dependency_info as *const _ as _) }

        self
    }
}

/// Graphics/Compute Commands: Querying
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
    /// Begin a query
    #[inline(always)]
    pub fn begin_query(
        self,
        pool: &(impl VkHandle<Handle = VkQueryPool> + ?Sized),
        query: u32,
        precise_query: bool,
    ) -> Self {
        let flags = if precise_query { VK_QUERY_CONTROL_PRECISE_BIT } else { 0 };
        unsafe {
            crate::vkfn::cmd_begin_query(self.ptr.native_ptr_mut(), pool.native_ptr(), query, flags);
        }
        self
    }

    /// Ends a query
    #[inline(always)]
    pub fn end_query(self, pool: &(impl VkHandle<Handle = VkQueryPool> + ?Sized), query: u32) -> Self {
        unsafe {
            crate::vkfn::cmd_end_query(self.ptr.native_ptr_mut(), pool.native_ptr(), query);
        }
        self
    }

    /// Reset queries in a query pool
    #[inline(always)]
    pub fn reset_query_pool(self, pool: &(impl VkHandle<Handle = VkQueryPool> + ?Sized), range: Range<u32>) -> Self {
        unsafe {
            crate::vkfn::cmd_reset_query_pool(
                self.ptr.native_ptr_mut(),
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
            crate::vkfn::cmd_write_timestamp(self.ptr.native_ptr_mut(), stage.0, pool.native_ptr(), query);
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
        dst: &(impl crate::VkHandle<Handle = VkBuffer> + ?Sized),
        dst_offset: VkDeviceSize,
        stride: VkDeviceSize,
        wide_result: bool,
        flags: QueryResultFlags,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_copy_query_pool_results(
                self.ptr.native_ptr_mut(),
                pool.native_ptr(),
                range.start,
                range.end - range.start,
                dst.native_ptr(),
                dst_offset,
                stride,
                flags.0 | if wide_result { VK_QUERY_RESULT_64_BIT } else { 0 },
            );
        }
        self
    }
}

/// Graphics/Compute Commands: Miscellaneous
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, Device: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, Device>
{
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
        VkClearValue { color: c.into() }
    }

    /// Constructs a `ClearValue` which represents clearing color value
    #[inline(always)]
    pub const fn color_f32(c: [f32; 4]) -> Self {
        VkClearValue {
            color: VkClearColorValue { float32: c },
        }
    }
    /// Constructs a `ClearValue` which represents clearing color value
    #[inline(always)]
    pub const fn color_u32(c: [u32; 4]) -> Self {
        VkClearValue {
            color: VkClearColorValue { uint32: c },
        }
    }
    /// Constructs a `ClearValue` which represents clearing color value
    #[inline(always)]
    pub const fn color_i32(c: [i32; 4]) -> Self {
        VkClearValue {
            color: VkClearColorValue { int32: c },
        }
    }
    /// Constructs a `ClearValue` which represents clearing both depth and stencil values
    #[inline(always)]
    pub const fn depth_stencil(depth: f32, stencil: u32) -> Self {
        VkClearValue {
            depthStencil: VkClearDepthStencilValue { depth, stencil },
        }
    }
}

/// Type of index buffer indices
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexType {
    /// Indices are 16-bit unsigned integer values
    U16 = VK_INDEX_TYPE_UINT16 as _,
    /// Indices are 32-bit unsigned integer values
    U32 = VK_INDEX_TYPE_UINT32 as _,
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
            pNext: std::ptr::null(),
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
        self.0.dstAccessMask = replace(&mut self.0.srcAccessMask, self.0.dstAccessMask);
        self.0.newLayout = replace(&mut self.0.oldLayout, self.0.newLayout);
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

/// Wrapper object of `VkBufferMemoryBarrier`, describes a memory barrier of a buffer.
#[derive(Clone)]
#[repr(transparent)]
pub struct BufferMemoryBarrier(VkBufferMemoryBarrier);
impl BufferMemoryBarrier {
    /// Construct a new buffer descriptor
    pub fn new(
        buf: &(impl VkHandle<Handle = VkBuffer> + ?Sized),
        range: Range<VkDeviceSize>,
        src_access_mask: VkAccessFlags,
        dst_access_mask: VkAccessFlags,
    ) -> Self {
        Self(VkBufferMemoryBarrier {
            sType: VkBufferMemoryBarrier::TYPE,
            pNext: std::ptr::null(),
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
        self.0.dstAccessMask = replace(&mut self.0.srcAccessMask, self.0.dstAccessMask);
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
