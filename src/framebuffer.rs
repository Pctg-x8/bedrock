//! Vulkan Framebuffer

use derives::implements;

use crate::{
    Device, ImageView, RenderPass, VkObject, VulkanStructure,
    ffi_helper::{opt_pointer, slice_as_ptr_empty_null},
    vk::*,
};
use core::marker::PhantomData;

/// Opaque handle to a framebuffer object
#[repr(transparent)]
pub struct Framebuffer(VkFramebuffer_T);
#[implements]
impl Framebuffer {
    /// Destroy a framebuffer object.
    #[inline]
    pub unsafe fn destroy(&mut self, device: &Device, allocation_callbacks: Option<&VkAllocationCallbacks>) {
        unsafe {
            crate::vkfn::destroy_framebuffer(
                device as *const _ as _,
                self as *mut _ as _,
                opt_pointer(allocation_callbacks),
            )
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramebufferCreateInfo<'r, 'rs>(
    VkFramebufferCreateInfo,
    PhantomData<(&'r RenderPass, &'rs [&'r ImageView])>,
);
impl<'r, 'rs> FramebufferCreateInfo<'r, 'rs> {
    #[inline(always)]
    pub const fn new(render_pass: &'r ImageView, attachments: &'rs [&'r ImageView], width: u32, height: u32) -> Self {
        Self(
            VkFramebufferCreateInfo {
                sType: VkFramebufferCreateInfo::TYPE,
                pNext: core::ptr::null(),
                renderPass: render_pass as *const _ as _,
                flags: 0,
                attachmentCount: attachments.len() as _,
                pAttachments: slice_as_ptr_empty_null(attachments) as *const _,
                width,
                height,
                layers: 1,
            },
            PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkFramebufferCreateInfo) -> Self {
        Self(raw, PhantomData)
    }

    pub const fn into_raw(self) -> VkFramebufferCreateInfo {
        self.0
    }

    pub const fn with_layers(mut self, layers: u32) -> Self {
        self.0.layers = layers;
        self
    }
}
