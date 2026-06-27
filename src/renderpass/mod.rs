use crate::ffi_helper::slice_as_ptr_empty_null;

mod standard;
pub use self::standard::*;

#[cfg(feature = "VK_KHR_create_renderpass2")]
mod extensible;
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub use self::extensible::*;

use crate::*;

pub trait RenderPass: VkHandle<Handle = VkRenderPass> + DeviceChildHandle {
    /// Returns the granularity for optimal render area
    #[implements]
    fn optimal_granularity(&self) -> VkExtent2D {
        let mut e = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::get_render_area_granularity(self.device_handle(), self.native_ptr(), e.as_mut_ptr());

            e.assume_init()
        }
    }

    #[inline(always)]
    fn subpass<'a>(&'a self, index: u32) -> SubpassRef<'a, Self> {
        SubpassRef(self, index)
    }
}
DerefContainerBracketImpl!(for RenderPass {});
GuardsImpl!(for RenderPass {});

pub trait ConcreteDeviceRenderPass: RenderPass + DeviceChild {}
DerefContainerBracketImpl!(for ConcreteDeviceRenderPass {});
GuardsImpl!(for ConcreteDeviceRenderPass {});

/// Opaque handle to a render pass object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkRenderPass::OBJECT_TYPE)]
pub struct RenderPassObject<Device: VkHandle<Handle = VkDevice>>(pub(crate) VkRenderPass, pub(crate) Device);
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for RenderPassObject<Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn::destroy_render_pass(self.1.native_ptr(), self.0, core::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for RenderPassObject<Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for RenderPassObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for RenderPassObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for RenderPassObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: VkHandle<Handle = VkDevice>> RenderPass for RenderPassObject<Device> {}
impl<Device: VkHandle<Handle = VkDevice>> RenderPassObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the parent
    pub const unsafe fn manage(handle: VkRenderPass, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges internal values (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkRenderPass, Device) {
        let v = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (v, p)
    }
}
impl<Device: VkHandle<Handle = VkDevice> + Clone> RenderPassObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> RenderPassObject<Device> {
        let r = RenderPassObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}
impl<Device: crate::Device> RenderPassObject<Device> {
    #[implements]
    #[inline(always)]
    pub fn new(device: Device, create_info: &(impl AnyRenderPassCreateInfo + ?Sized)) -> crate::Result<Self> {
        create_info.execute(&device, None).map(move |x| Self(x, device))
    }
}

#[implements]
pub trait AnyRenderPassCreateInfo {
    fn execute(
        &self,
        device: &(impl crate::Device + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkRenderPass>;
}

#[repr(transparent)]
pub struct RenderPassBeginInfo<'d>(
    VkRenderPassBeginInfo,
    #[allow(clippy::type_complexity)]
    core::marker::PhantomData<(
        &'d dyn VkHandle<Handle = VkRenderPass>,
        &'d dyn VkHandle<Handle = VkFramebuffer>,
        &'d [ClearValue],
    )>,
);
impl<'d> RenderPassBeginInfo<'d> {
    #[inline]
    pub fn new(
        render_pass: &'d (impl VkHandle<Handle = VkRenderPass> + ?Sized),
        framebuffer: &'d (impl VkHandle<Handle = VkFramebuffer> + ?Sized),
        render_area: VkRect2D,
        clear_values: &'d [ClearValue],
    ) -> Self {
        Self(
            VkRenderPassBeginInfo {
                sType: VkRenderPassBeginInfo::TYPE,
                pNext: core::ptr::null(),
                renderPass: render_pass.native_ptr(),
                framebuffer: framebuffer.native_ptr(),
                renderArea: render_area,
                clearValueCount: clear_values.len() as _,
                pClearValues: slice_as_ptr_empty_null(clear_values),
            },
            core::marker::PhantomData,
        )
    }
}
impl<'d> AsRef<VkRenderPassBeginInfo> for RenderPassBeginInfo<'d> {
    #[inline(always)]
    fn as_ref(&self) -> &VkRenderPassBeginInfo {
        &self.0
    }
}

#[cfg(feature = "VK_KHR_create_renderpass2")]
pub type SubpassBeginInfo = VkSubpassBeginInfoKHR;
#[cfg(feature = "VK_KHR_create_renderpass2")]
impl SubpassBeginInfo {
    pub const fn new(contents: SubpassContents) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            contents: contents as _,
        }
    }
}

#[cfg(feature = "VK_KHR_create_renderpass2")]
pub type SubpassEndInfo = VkSubpassEndInfoKHR;
#[cfg(feature = "VK_KHR_create_renderpass2")]
impl SubpassEndInfo {
    pub const fn new() -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
        }
    }
}

/// A reference to a subpass in a render pass object.
pub struct SubpassRef<'r, RenderPass: 'r + ?Sized + VkHandle<Handle = VkRenderPass>>(pub &'r RenderPass, pub u32);
impl<'r, RenderPass: 'r + ?Sized + VkHandle<Handle = VkRenderPass>> Clone for SubpassRef<'r, RenderPass> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'r, RenderPass: 'r + ?Sized + VkHandle<Handle = VkRenderPass>> Copy for SubpassRef<'r, RenderPass> {}
impl<'r, RenderPass: 'r + ?Sized + VkHandle<Handle = VkRenderPass>> PartialEq for SubpassRef<'r, RenderPass> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.0, other.0) && self.1 == other.1
    }
}
impl<'r, RenderPass: 'r + ?Sized + VkHandle<Handle = VkRenderPass>> Eq for SubpassRef<'r, RenderPass> {}
impl<'r, RenderPass: 'r + ?Sized + VkHandle<Handle = VkRenderPass>> core::hash::Hash for SubpassRef<'r, RenderPass> {
    #[inline(always)]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const RenderPass, self.1).hash(state)
    }
}
impl<'r, RenderPass: 'r + ?Sized + VkHandle<Handle = VkRenderPass>> core::fmt::Debug for SubpassRef<'r, RenderPass> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RenderPass({:p}).{}", self.0, self.1)
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadOp {
    /// The previous contents of the image within the render area will be preserved.
    ///
    /// ## Used access types
    ///
    /// This operation uses the "Read" access
    Load = VK_ATTACHMENT_LOAD_OP_LOAD,
    /// The contents within the render area will be cleared to a uniform value, which is
    /// specified when a render pass instance is begun.
    ///
    /// ## Used access types
    ///
    /// This operation uses the "Write" access
    Clear = VK_ATTACHMENT_LOAD_OP_CLEAR,
    /// The previous contents within the area need not be preserved;
    /// the contents of the attachment will be undefined inside the render area.
    ///
    /// ## Used access types
    ///
    /// This operation uses the "Write" access
    DontCare = VK_ATTACHMENT_LOAD_OP_DONT_CARE,
}

/// Possible argument values of `AttachmentDescription::store_op` and `stencil_store_op`,
/// specifying how the contents of the attachment are treated.
///
/// ## Used access types
///
/// Both items use the "Write" access
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreOp {
    /// The contents generated during the render pass and within the render area are written to memory.
    Store = VK_ATTACHMENT_STORE_OP_STORE,
    /// The contents within the render area are not needed after rendering, and *may* be discarded;
    /// the contents of the attachment will be undefined inside the render area.
    DontCare = VK_ATTACHMENT_STORE_OP_DONT_CARE,
}
