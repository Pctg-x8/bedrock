//! Vulkan Commands

use crate::ffi_helper::slice_as_ptr_empty_null;
use crate::*;
use core::ops::Range;
use derives::implements;

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
impl<Device: VkHandle<Handle = VkDevice>> CommandPoolObject<Device> {
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
        let r = CommandPoolObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}
impl<Device: crate::Device> CommandPoolObject<Device> {
    /// Create a new command pool object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn new(device: Device, info: &CommandPoolCreateInfo) -> crate::Result<Self> {
        let h = device.new_command_pool_raw(info, None)?;

        Ok(unsafe { Self::manage(h, device) })
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
impl<Device> CommandBuffer for CommandBufferObject<Device> {}
impl<Device> CommandBufferMut for CommandBufferObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> CommandBufferObject<Device> {
    /// Allocate command buffers from an existing command pool
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("alloc")]
    #[inline]
    pub fn alloc(device: Device, info: &CommandBufferAllocateInfo) -> crate::Result<Vec<Self>> {
        let mut hs = vec![VkCommandBuffer::NULL; info.0.commandBufferCount as _];

        unsafe {
            crate::vkfn::allocate_command_buffers(device.native_ptr(), &info.0, hs.as_mut_ptr()).into_result()?;

            Ok(core::mem::transmute(hs))
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
impl<Device: Clone> CommandBufferObject<&'_ Device> {
    /// clones internally-referenced parent object
    pub const fn clone_parent(&self) -> CommandBufferObject<Device> {
        CommandBufferObject(self.0, core::marker::PhantomData)
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

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommandBufferLevel {
    Primary = VK_COMMAND_BUFFER_LEVEL_PRIMARY,
    Secondary = VK_COMMAND_BUFFER_LEVEL_SECONDARY,
}

/// Bitmask controlling behavior of a command pool reset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct CommandPoolResetFlags(VkCommandPoolResetFlags);
impl CommandPoolResetFlags {
    /// Empty bits.
    pub const EMPTY: Self = Self(0);

    /// Resetting a command pool recycles all of the resources from the command pool back to the system.
    pub const RELEASE_RESOURCES: Self = Self(VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT);
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandBufferAllocateInfo<'r>(
    pub(crate) VkCommandBufferAllocateInfo,
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

pub trait CommandPool: VkHandle<Handle = VkCommandPool> + DeviceChild {}
DerefContainerBracketImpl!(for CommandPool {});
GuardsImpl!(for CommandPool {});

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
        unsafe {
            crate::vkfn::reset_command_pool(self.device_handle(), self.native_ptr_mut(), flags)
                .into_result()
                .map(drop)
        }
    }

    /// Free command buffers
    /// # Safety
    /// Application cannot use passed command buffers after this call
    #[implements]
    unsafe fn free(&mut self, buffers: &[VkHandleRefMut<VkCommandBuffer>]) {
        unsafe {
            crate::vkfn::free_command_buffers(
                self.device().native_ptr(),
                self.native_ptr_mut(),
                buffers.len() as _,
                slice_as_ptr_empty_null(buffers) as *const _,
            )
        }
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
    pub fn of_rendering(
        render_pass: SubpassRef<'d, impl VkHandle<Handle = VkRenderPass> + ?Sized>,
        framebuffer: Option<&'d (impl VkHandle<Handle = VkFramebuffer> + ?Sized)>,
    ) -> Self {
        Self::new().rendering(render_pass, framebuffer)
    }

    #[inline]
    pub fn rendering(
        mut self,
        render_pass: SubpassRef<'d, impl VkHandle<Handle = VkRenderPass> + ?Sized>,
        framebuffer: Option<&'d (impl VkHandle<Handle = VkFramebuffer> + ?Sized)>,
    ) -> Self {
        self.0.renderPass = render_pass.0.native_ptr();
        self.0.subpass = render_pass.1;
        self.0.framebuffer = framebuffer.map_or(VkFramebuffer::NULL, VkHandle::native_ptr);
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

pub trait CommandBuffer: VkHandle<Handle = VkCommandBuffer> {}
DerefContainerBracketImpl!(for CommandBuffer {});
GuardsImpl!(for CommandBuffer {});

pub trait CommandBufferMut: CommandBuffer + VkHandleMut {
    /// Start recording a command buffer
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    ///
    /// # Safety
    /// The `CommandPool` that this commandBuffer was allocated from must be externally synchronized.
    #[implements]
    unsafe fn begin<'d, ExtFnProvider: 'd + ?Sized>(
        &'d mut self,
        info: &CommandBufferBeginInfo,
        ext_fn_provider: &'d ExtFnProvider,
    ) -> crate::Result<CmdRecord<'d, Self, ExtFnProvider>> {
        unsafe {
            crate::vkfn::begin_command_buffer(self.native_ptr_mut(), info.as_raw_ref()).into_result()?;
        }

        Ok(CmdRecord {
            ptr: self,
            ext_fn_provider,
        })
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
    unsafe fn reset(&mut self, flags: VkCommandBufferResetFlags) -> crate::Result<()> {
        unsafe {
            crate::vkfn::reset_command_buffer(self.native_ptr_mut(), flags)
                .into_result()
                .map(drop)
        }
    }

    /// Locking CommandBuffer with CommandPool to satisfy externally synchronization restriction.
    /// # Safety
    /// This command buffer must be allocated from `pool`.
    unsafe fn synchronize_with<'p, 'b: 'p, Pool: 'p + crate::CommandPoolMut + ?Sized>(
        &'b mut self,
        pool: &'p mut Pool,
    ) -> SynchronizedCommandBuffer<'p, 'b, Pool, Self> {
        SynchronizedCommandBuffer {
            _pool: pool,
            buffer: self,
        }
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
    _pool: &'p mut Pool,
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
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline(always)]
    pub fn begin<ExtFnProvider: 'b + ?Sized>(
        &'b mut self,
        info: &CommandBufferBeginInfo,
        ext_fn_provider: &'b ExtFnProvider,
    ) -> crate::Result<CmdRecord<'b, Buffer, ExtFnProvider>> {
        unsafe { self.buffer.begin(info, ext_fn_provider) }
    }

    /// Reset a command buffer to the initial state
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[inline(always)]
    pub fn reset(&mut self, flags: VkCommandBufferResetFlags) -> crate::Result<()> {
        unsafe { self.buffer.reset(flags) }
    }
}

/// Functions from extension
pub trait DeviceExtCommandFunctionProvider {
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[cfg(not(feature = "Allow1_2APIs"))]
    fn cmd_begin_render_pass_2_khr_fn(&self) -> PFN_vkCmdBeginRenderPass2KHR;
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[cfg(not(feature = "Allow1_2APIs"))]
    fn cmd_end_render_pass_2_khr_fn(&self) -> PFN_vkCmdEndRenderPass2KHR;
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[cfg(not(feature = "Allow1_2APIs"))]
    fn cmd_next_subpass_2_khr_fn(&self) -> PFN_vkCmdNextSubpass2KHR;

    #[cfg(feature = "VK_KHR_synchronization2")]
    #[cfg(not(feature = "Allow1_3APIs"))]
    fn cmd_pipeline_barrier_2_khr_fn(&self) -> PFN_vkCmdPipelineBarrier2KHR;

    #[cfg(feature = "VK_KHR_push_descriptor")]
    #[cfg(not(feature = "Allow1_4APIs"))]
    fn cmd_push_descriptor_set_khr_fn(&self) -> PFN_vkCmdPushDescriptorSetKHR;

    #[cfg(feature = "VK_EXT_sample_locations")]
    fn cmd_set_sample_locations_ext_fn(&self) -> PFN_vkCmdSetSampleLocationsEXT;
}
DerefContainerWithGuardsBracketImpl!(for DeviceExtCommandFunctionProvider {
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[cfg(not(feature = "Allow1_2APIs"))]
    ForwardFnPtr!(deref cmd_begin_render_pass_2_khr_fn -> PFN_vkCmdBeginRenderPass2KHR);
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[cfg(not(feature = "Allow1_2APIs"))]
    ForwardFnPtr!(deref cmd_end_render_pass_2_khr_fn -> PFN_vkCmdEndRenderPass2KHR);
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[cfg(not(feature = "Allow1_2APIs"))]
    ForwardFnPtr!(deref cmd_next_subpass_2_khr_fn -> PFN_vkCmdNextSubpass2KHR);

    #[cfg(feature = "VK_KHR_synchronization2")]
    #[cfg(not(feature = "Allow1_3APIs"))]
    ForwardFnPtr!(deref cmd_pipeline_barrier_2_khr_fn -> PFN_vkCmdPipelineBarrier2KHR);

    #[cfg(feature = "VK_KHR_push_descriptor")]
    #[cfg(not(feature = "Allow1_4APIs"))]
    ForwardFnPtr!(deref cmd_push_descriptor_set_khr_fn -> PFN_vkCmdPushDescriptorSetKHR);

    #[cfg(feature = "VK_EXT_sample_locations")]
    ForwardFnPtr!(deref cmd_set_sample_locations_ext_fn -> PFN_vkCmdSetSampleLocationsEXT);
});

/// The recording state of command buffers
#[implements]
#[must_use = "CmdRecord must be consumed by end() (not closed automatically by drop!)"]
pub struct CmdRecord<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, ExtFnProvider: 'd + ?Sized>
{
    ptr: &'d mut CommandBuffer,
    ext_fn_provider: &'d ExtFnProvider,
}
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
{
    pub const fn new(ptr: &'d mut CommandBuffer, ext_fn_provider: &'d ExtFnProvider) -> Self {
        Self { ptr, ext_fn_provider }
    }
}
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer>, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
{
    #[inline(always)]
    pub fn as_dyn_ref<'r>(
        &'r mut self,
    ) -> CmdRecord<'r, dyn VkHandleMut<Handle = VkCommandBuffer> + 'r, ExtFnProvider> {
        CmdRecord {
            ptr: self.ptr as _,
            ext_fn_provider: self.ext_fn_provider,
        }
    }
}

/// Common Commands: End Recording
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
{
    /// Finish recording a command buffer
    #[inline]
    pub fn end(self) -> crate::Result<()> {
        unsafe {
            crate::vkfn::end_command_buffer(self.ptr.native_ptr())
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
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
{
    /// Begin a new render pass
    #[inline]
    pub fn begin_render_pass(self, info: &crate::RenderPassBeginInfo, contents: SubpassContents) -> Self {
        unsafe {
            crate::vkfn::cmd_begin_render_pass(self.ptr.native_ptr_mut(), info.as_ref(), contents as _);
        }

        self
    }

    /// Transition to the next subpass of a render pass
    #[inline]
    pub fn next_subpass(self, contents: SubpassContents) -> Self {
        unsafe {
            crate::vkfn::cmd_next_subpass(self.ptr.native_ptr_mut(), contents as _);
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
    #[cfg(not(feature = "Allow1_2APIs"))]
    #[inline]
    pub fn begin_render_pass2(self, begin_info: &RenderPassBeginInfo, subpass_begin_info: &SubpassBeginInfo) -> Self
    where
        ExtFnProvider: DeviceExtCommandFunctionProvider,
    {
        unsafe {
            (self.ext_fn_provider.cmd_begin_render_pass_2_khr_fn().0)(
                self.ptr.native_ptr_mut(),
                begin_info as *const _ as _,
                subpass_begin_info as *const _ as _,
            );
        }

        self
    }

    #[cfg(feature = "Allow1_2APIs")]
    #[inline]
    pub fn begin_render_pass2(self, begin_info: &RenderPassBeginInfo, subpass_begin_info: &SubpassBeginInfo) -> Self {
        unsafe {
            crate::vkfn::cmd_begin_render_pass2(
                self.ptr.native_ptr(),
                begin_info as *const _ as _,
                subpass_begin_info as *const _ as _,
            );
        }

        self
    }

    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[cfg(not(feature = "Allow1_2APIs"))]
    #[inline]
    pub fn next_subpass2(self, subpass_begin_info: &SubpassBeginInfo, subpass_end_info: &SubpassEndInfo) -> Self
    where
        ExtFnProvider: DeviceExtCommandFunctionProvider,
    {
        unsafe {
            (self.ext_fn_provider.cmd_next_subpass_2_khr_fn().0)(
                self.ptr.native_ptr_mut(),
                subpass_begin_info as *const _ as _,
                subpass_end_info as *const _ as _,
            );
        }

        self
    }

    #[cfg(feature = "Allow1_2APIs")]
    #[inline]
    pub fn next_subpass2(self, subpass_begin_info: &SubpassBeginInfo, subpass_end_info: &SubpassEndInfo) -> Self {
        unsafe {
            crate::vkfn::cmd_next_subpass2(
                self.ptr.native_ptr_mut(),
                subpass_begin_info as *const _ as _,
                subpass_end_info as *const _ as _,
            );
        }

        self
    }

    #[cfg(feature = "VK_KHR_create_renderpass2")]
    #[cfg(not(feature = "Allow1_2APIs"))]
    #[inline]
    pub fn end_render_pass2(self, subpass_end_info: &SubpassEndInfo) -> Self
    where
        ExtFnProvider: DeviceExtCommandFunctionProvider,
    {
        unsafe {
            (self.device.cmd_end_render_pass_2_khr_fn().0)(
                self.ptr.native_ptr_mut(),
                subpass_end_info as *const _ as _,
            );
        }

        self
    }

    #[cfg(feature = "Allow1_2APIs")]
    #[inline]
    pub fn end_render_pass2(self, subpass_end_info: &SubpassEndInfo) -> Self {
        unsafe {
            crate::vkfn::cmd_end_render_pass2(self.ptr.native_ptr_mut(), subpass_end_info);
        }

        self
    }
}

/// Graphics/Compute Commands: Pipeline Setup
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
{
    /// Bind a pipeline object to a command buffer
    #[inline]
    pub fn bind_pipeline(
        self,
        bind_point: PipelineBindPoint,
        pipeline: &(impl VkHandle<Handle = VkPipeline> + ?Sized),
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_pipeline(self.ptr.native_ptr_mut(), bind_point as _, pipeline.native_ptr());
        }
        self
    }

    /// Binds descriptor sets to a command buffer
    #[inline]
    pub fn bind_descriptor_sets(
        self,
        bind_point: PipelineBindPoint,
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
        first: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_bind_descriptor_sets(
                self.ptr.native_ptr_mut(),
                bind_point as _,
                pipeline_layout.native_ptr(),
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
                core::mem::size_of::<T>() as _,
                value as *const T as *const _,
            );
        }
        self
    }

    /// Push descriptor updates into a command buffer
    #[cfg(feature = "VK_KHR_push_descriptor")]
    #[cfg(not(feature = "Allow1_4APIs"))]
    #[inline(always)]
    pub unsafe fn push_descriptor_set_raw(
        self,
        bind_point: PipelineBindPoint,
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
        set: u32,
        writes: &[VkWriteDescriptorSet],
    ) -> Self
    where
        ExtFnProvider: DeviceExtCommandFunctionProvider,
    {
        unsafe {
            (self.ext_fn_provider.cmd_push_descriptor_set_khr_fn().0)(
                self.ptr.native_ptr_mut(),
                bind_point as _,
                pipeline_layout.native_ptr(),
                set,
                writes.len() as _,
                slice_as_ptr_empty_null(writes),
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
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
        set: u32,
        writes: &[VkWriteDescriptorSet],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_push_descriptor_set(
                self.ptr.native_ptr_mut(),
                bind_point as _,
                pipeline_layout.native_ptr(),
                set,
                writes.len() as _,
                slice_as_ptr_empty_null(writes),
            );
        }

        self
    }

    /// Push descriptor updates into a command buffer
    #[cfg(feature = "VK_KHR_push_descriptor")]
    #[cfg(not(feature = "Allow1_4APIs"))]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn push_descriptor_set_alloc(
        self,
        bind_point: PipelineBindPoint,
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
        set: u32,
        writes: &[crate::DescriptorSetWriteInfo],
    ) -> Self
    where
        ExtFnProvider: DeviceExtCommandFunctionProvider,
    {
        unsafe {
            self.push_descriptor_set_raw(
                bind_point,
                pipeline_layout,
                set,
                &crate::alloc::collect_vec(writes.iter().map(|x| x.make_structure())),
            )
        }
    }

    /// Push descriptor updates into a command buffer
    #[cfg(feature = "Allow1_4APIs")]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn push_descriptor_set_alloc(
        self,
        bind_point: PipelineBindPoint,
        pipeline_layout: &(impl VkHandle<Handle = VkPipelineLayout> + ?Sized),
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
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
{
    /// Set the viewport on a command buffer
    #[inline(always)]
    pub fn set_viewport(self, first: u32, viewports: &[Viewport]) -> Self {
        unsafe {
            crate::vkfn::cmd_set_viewport(
                self.ptr.native_ptr_mut(),
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
                self.ptr.native_ptr_mut(),
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
    #[inline]
    pub fn set_sample_locations(self, info: &VkSampleLocationsInfoEXT) -> Self
    where
        ExtFnProvider: DeviceExtCommandFunctionProvider,
    {
        unsafe {
            (self.ext_fn_provider.cmd_set_sample_locations_ext_fn().0)(self.ptr.native_ptr_mut(), info);
        }

        self
    }
}

/// Graphics Commands: Binding Buffers
#[implements]
impl<'d, CommandBuffer: 'd + VkHandleMut<Handle = VkCommandBuffer> + ?Sized, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
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
    pub fn bind_vertex_buffers(self, first: u32, buffers: &[VkHandleRef<VkBuffer>], offsets: &[DeviceSize]) -> Self {
        assert_eq!(buffers.len(), offsets.len());

        unsafe {
            crate::vkfn::cmd_bind_vertex_buffers(
                self.ptr.native_ptr_mut(),
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
        buffers: &[VkHandleRef<VkBuffer>; N],
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
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
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
        offset: DeviceSize,
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
        offset: DeviceSize,
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
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
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
        offset: DeviceSize,
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_dispatch_indirect(self.ptr.native_ptr_mut(), buffer.native_ptr(), offset);
        }
        self
    }
}

/// Transfer Commands: Copying resources
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
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
                slice_as_ptr_empty_null(regions) as _,
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
                slice_as_ptr_empty_null(regions),
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
                slice_as_ptr_empty_null(regions),
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
                slice_as_ptr_empty_null(regions),
            );
        }
        self
    }

    /// Update a buffer's contents from host memory
    #[inline(always)]
    pub fn update_buffer<T>(
        self,
        dst: &(impl crate::VkHandle<Handle = VkBuffer> + ?Sized),
        dst_offset: DeviceSize,
        size: DeviceSize,
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
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
{
    /// Fill a region of a buffer with a fixed value.
    /// `size` is number of bytes to fill
    #[inline(always)]
    pub fn fill_buffer(
        self,
        dst: &(impl crate::VkHandle<Handle = VkBuffer> + ?Sized),
        dst_offset: DeviceSize,
        size: DeviceSize,
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
                slice_as_ptr_empty_null(ranges),
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
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
{
    /// Execute a secondary command buffer from a primary command buffer
    /// # Safety
    ///
    /// Caller must be primary buffer and in the render pass when executing secondary command buffer
    #[inline(always)]
    pub unsafe fn execute_commands(self, buffers: &[VkHandleRef<VkCommandBuffer>]) -> Self {
        unsafe {
            crate::vkfn::cmd_execute_commands(
                self.ptr.native_ptr_mut(),
                buffers.len() as _,
                slice_as_ptr_empty_null(buffers) as _,
            );
        }
        self
    }
}

/// Graphics Commands: Resolving an image to another image
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
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
                slice_as_ptr_empty_null(regions),
            )
        };
        self
    }
}

/// Graphics/Compute Commands: Synchronization between command buffers/queues
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
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
        events: &[VkHandleRef<VkEvent>],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[VkMemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_wait_events(
                self.ptr.native_ptr_mut(),
                events.len() as _,
                slice_as_ptr_empty_null(events) as _,
                src_stage_mask.0,
                dst_stage_mask.0,
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
        dependency_flags: VkDependencyFlags,
        memory_barriers: &[VkMemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) -> Self {
        unsafe {
            crate::vkfn::cmd_pipeline_barrier(
                self.ptr.native_ptr_mut(),
                src_stage_mask.0,
                dst_stage_mask.0,
                dependency_flags,
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
    #[cfg(feature = "VK_KHR_synchronization2")]
    #[cfg(not(feature = "Allow1_3APIs"))]
    #[inline(always)]
    pub fn pipeline_barrier_2(self, dependency_info: &crate::DependencyInfo) -> Self
    where
        ExtFnProvider: DeviceExtCommandFunctionProvider,
    {
        unsafe {
            (self.ext_fn_provider.cmd_pipeline_barrier_2_khr_fn().0)(
                self.ptr.native_ptr_mut(),
                dependency_info as *const _ as _,
            )
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
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
{
    /// Begin a query
    #[inline(always)]
    pub fn begin_query(
        self,
        pool: &(impl VkHandle<Handle = VkQueryPool> + ?Sized),
        query: u32,
        flags: VkQueryControlFlags,
    ) -> Self {
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
        dst: &(impl VkHandle<Handle = VkBuffer> + ?Sized),
        dst_offset: DeviceSize,
        stride: DeviceSize,
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
                flags.bits(),
            );
        }
        self
    }
}

/// Graphics/Compute Commands: Miscellaneous
#[implements]
impl<'d, CommandBuffer: VkHandleMut<Handle = VkCommandBuffer> + ?Sized + 'd, ExtFnProvider: 'd + ?Sized>
    CmdRecord<'d, CommandBuffer, ExtFnProvider>
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
