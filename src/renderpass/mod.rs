mod standard;
use self::ffi_helper::ArrayFFIExtensions;
pub use self::standard::*;

cfg_if! {
    if #[cfg(feature = "VK_KHR_create_renderpass2")] {
        mod extensible;
        pub use self::extensible::*;
    }
}

use crate::*;

/// Opaque handle to a render pass object
#[derive(VkHandle, VkObject, DeviceChild)]
#[VkObject(type = VK_OBJECT_TYPE_RENDER_PASS)]
pub struct RenderPassObject<Device: crate::Device>(pub(crate) VkRenderPass, #[parent] pub(crate) Device);
unsafe impl<Device: crate::Device + Sync> Sync for RenderPassObject<Device> {}
unsafe impl<Device: crate::Device + Send> Send for RenderPassObject<Device> {}
#[implements]
impl<Device: crate::Device> Drop for RenderPassObject<Device> {
    fn drop(&mut self) {
        unsafe {
            self.0.destroy(self.1.native_ptr(), core::ptr::null());
        }
    }
}
impl<Device: crate::Device> RenderPass for RenderPassObject<Device> {}

#[repr(transparent)]
pub struct RenderPassBeginInfo<'d, R: RenderPass + 'd, F: Framebuffer + 'd>(
    VkRenderPassBeginInfo,
    core::marker::PhantomData<(&'d R, &'d F, &'d [ClearValue])>,
);
impl<'d, R: RenderPass + 'd, F: Framebuffer + 'd> RenderPassBeginInfo<'d, R, F> {
    #[inline]
    pub fn new(render_pass: &'d R, framebuffer: &'d F, render_area: VkRect2D, clear_values: &'d [ClearValue]) -> Self {
        Self(
            VkRenderPassBeginInfo {
                sType: VkRenderPassBeginInfo::TYPE,
                pNext: core::ptr::null(),
                renderPass: render_pass.native_ptr(),
                framebuffer: framebuffer.native_ptr(),
                renderArea: render_area,
                clearValueCount: clear_values.len() as _,
                pClearValues: clear_values.as_ptr_empty_null(),
            },
            core::marker::PhantomData,
        )
    }
}
impl<'d, R: RenderPass + 'd, F: Framebuffer + 'd> AsRef<VkRenderPassBeginInfo> for RenderPassBeginInfo<'d, R, F> {
    fn as_ref(&self) -> &VkRenderPassBeginInfo {
        &self.0
    }
}

#[cfg(feature = "VK_KHR_create_renderpass2")]
#[repr(transparent)]
pub struct SubpassBeginInfo(VkSubpassBeginInfo);
impl SubpassBeginInfo {
    #[inline]
    pub const fn new(contents: VkSubpassContents) -> Self {
        Self(VkSubpassBeginInfoKHR {
            sType: VkSubpassBeginInfoKHR::TYPE,
            pNext: core::ptr::null(),
            contents,
        })
    }
}
#[cfg(feature = "VK_KHR_create_renderpass2")]
impl AsRef<VkSubpassBeginInfo> for SubpassBeginInfo {
    fn as_ref(&self) -> &VkSubpassBeginInfo {
        &self.0
    }
}

#[cfg(feature = "VK_KHR_create_renderpass2")]
#[repr(transparent)]
pub struct SubpassEndInfo(VkSubpassEndInfo);
impl SubpassEndInfo {
    #[inline]
    pub const fn new() -> Self {
        Self(VkSubpassEndInfoKHR {
            sType: VkSubpassEndInfoKHR::TYPE,
            pNext: core::ptr::null(),
        })
    }
}
#[cfg(feature = "VK_KHR_create_renderpass2")]
impl AsRef<VkSubpassEndInfo> for SubpassEndInfo {
    fn as_ref(&self) -> &VkSubpassEndInfo {
        &self.0
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
    pub(crate) const fn as_api_value(self) -> u32 {
        match self {
            Self::External => VK_SUBPASS_EXTERNAL,
            Self::Internal(x) => x,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadOp {
    /// The previous contents of the image within the render area will be preserved.
    ///
    /// ## Used access types
    ///
    /// This operation uses the "Read" access
    Load = VK_ATTACHMENT_LOAD_OP_LOAD as _,
    /// The contents within the render area will be cleared to a uniform value, which is
    /// specified when a render pass instance is begun.
    ///
    /// ## Used access types
    ///
    /// This operation uses the "Write" access
    Clear = VK_ATTACHMENT_LOAD_OP_CLEAR as _,
    /// The previous contents within the area need not be preserved;
    /// the contents of the attachment will be undefined inside the render area.
    ///
    /// ## Used access types
    ///
    /// This operation uses the "Write" access
    DontCare = VK_ATTACHMENT_LOAD_OP_DONT_CARE as _,
}

/// Possible argument values of `AttachmentDescription::store_op` and `stencil_store_op`,
/// specifying how the contents of the attachment are treated.
///
/// ## Used access types
///
/// Both items use the "Write" access
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreOp {
    /// The contents generated during the render pass and within the render area are written to memory.
    Store = VK_ATTACHMENT_STORE_OP_STORE as _,
    /// The contents within the render area are not needed after rendering, and *may* be discarded;
    /// the contents of the attachment will be undefined inside the render area.
    DontCare = VK_ATTACHMENT_STORE_OP_DONT_CARE as _,
}
