mod standard;
use core::{marker::PhantomData, mem::MaybeUninit, ptr::NonNull};
use ffi_helper::{opt_pointer, slice_as_ptr_empty_null};

pub use self::standard::*;

#[cfg(feature = "VK_KHR_create_renderpass2")]
mod extensible;
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub use self::extensible::*;

use crate::*;

/// Opaque handle to a render pass object.
#[repr(transparent)]
pub struct RenderPass(VkRenderPass_T);
#[implements]
impl RenderPass {
    /// Destroy a render pass object.
    pub unsafe fn destroy(&mut self, device: &Device, allocation_callbacks: Option<&VkAllocationCallbacks>) {
        unsafe {
            crate::vkfn::destroy_render_pass(
                device as *const _ as _,
                self as *mut _ as _,
                opt_pointer(allocation_callbacks),
            )
        }
    }

    /// Returns the granularity for optimal render area
    pub unsafe fn get_render_area_granularity(&self, device: &Device, sink: &mut MaybeUninit<Extent2D>) {
        unsafe {
            crate::vkfn::get_render_area_granularity(device as *const _ as _, self as *const _ as _, sink.as_mut_ptr());
        }
    }

    /// Returns the granularity for optimal render area
    pub unsafe fn render_area_granularity(&self, device: &Device) -> VkExtent2D {
        let mut e = MaybeUninit::uninit();
        unsafe {
            self.get_render_area_granularity(device, &mut e);
        }

        unsafe { e.assume_init() }
    }

    #[inline(always)]
    pub const fn subpass(&self, index: u32) -> SubpassRef {
        SubpassRef(self, index)
    }
}

#[implements]
pub trait AnyRenderPassCreateInfo {
    fn execute(
        &self,
        device: &Device,
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<NonNull<RenderPass>>;
}

#[repr(transparent)]
pub struct RenderPassBeginInfo<'d>(
    VkRenderPassBeginInfo,
    PhantomData<(
        &'d RenderPass,
        &'d dyn VkHandle<Handle = VkFramebuffer>,
        &'d [ClearValue],
    )>,
);
impl<'d> RenderPassBeginInfo<'d> {
    #[inline]
    pub fn new(
        render_pass: &'d RenderPass,
        framebuffer: &'d (impl VkHandle<Handle = VkFramebuffer> + ?Sized),
        render_area: VkRect2D,
        clear_values: &'d [ClearValue],
    ) -> Self {
        Self(
            VkRenderPassBeginInfo {
                sType: VkRenderPassBeginInfo::TYPE,
                pNext: core::ptr::null(),
                renderPass: render_pass as *const _ as _,
                framebuffer: framebuffer.native_ptr(),
                renderArea: render_area,
                clearValueCount: clear_values.len() as _,
                pClearValues: slice_as_ptr_empty_null(clear_values),
            },
            PhantomData,
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
impl VkSubpassBeginInfoKHR {
    pub const fn new(contents: VkSubpassContents) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            contents,
        }
    }
}

#[cfg(feature = "VK_KHR_create_renderpass2")]
impl VkSubpassEndInfoKHR {
    pub const fn new() -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
        }
    }
}

/// A reference to a subpass in a render pass object.
pub struct SubpassRef<'r>(pub &'r RenderPass, pub u32);
impl<'r> Clone for SubpassRef<'r> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}
impl<'r> Copy for SubpassRef<'r> {}
impl<'r> PartialEq for SubpassRef<'r> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.0, other.0) && self.1 == other.1
    }
}
impl<'r> Eq for SubpassRef<'r> {}
impl<'r> core::hash::Hash for SubpassRef<'r> {
    #[inline(always)]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const RenderPass, self.1).hash(state)
    }
}
impl<'r> core::fmt::Debug for SubpassRef<'r> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RenderPass({:p}).{}", self.0, self.1)
    }
}

/// Index specifying a subpass
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SubpassIndex {
    /// Out of the render pass
    External,
    /// In the render pass
    Internal(u32),
}
impl SubpassIndex {
    #[inline(always)]
    pub(crate) const fn as_vk(self) -> u32 {
        match self {
            Self::External => VK_SUBPASS_EXTERNAL,
            Self::Internal(x) => x,
        }
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
