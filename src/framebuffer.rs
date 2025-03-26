//! Vulkan Framebuffer

use crate::ffi_helper::slice_as_ptr_empty_null;
use crate::*;
use core::ops::*;
use derives::implements;

/// Opaque handle to a framebuffer object
#[derive(VkHandle, VkObject)]
#[VkObject(type = VkFramebuffer::OBJECT_TYPE)]
pub struct FramebufferObject<'r, Device: VkHandle<Handle = VkDevice>> {
    #[handle]
    pub(crate) handle: VkFramebuffer,
    pub(crate) parent: Device,
    pub(crate) _under_resources: core::marker::PhantomData<&'r [Box<dyn crate::VkHandle<Handle = VkImageView> + 'r>]>,
}
#[implements]
impl<Device: VkHandle<Handle = VkDevice>> Drop for FramebufferObject<'_, Device> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            crate::vkfn::destroy_framebuffer(self.parent.native_ptr(), self.handle, std::ptr::null());
        }
    }
}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for FramebufferObject<'_, Device> {}
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for FramebufferObject<'_, Device> {}
impl<Device: VkHandle<Handle = VkDevice>> DeviceChildHandle for FramebufferObject<'_, Device> {
    #[inline(always)]
    fn device_handle(&self) -> VkDevice {
        self.parent.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for FramebufferObject<'_, Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.parent
    }
}
impl<Device: VkHandle<Handle = VkDevice>> Framebuffer for FramebufferObject<'_, Device> {}
impl<Device: VkHandle<Handle = VkDevice>> FramebufferObject<'_, Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkFramebuffer, parent: Device) -> Self {
        Self {
            handle,
            parent,
            _under_resources: core::marker::PhantomData,
        }
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkFramebuffer, Device) {
        let h = self.handle;
        let p = unsafe { core::ptr::read(&self.parent) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<'r, Device: VkHandle<Handle = VkDevice> + Clone> FramebufferObject<'r, &'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> FramebufferObject<'r, Device> {
        let r = FramebufferObject {
            handle: self.handle,
            parent: self.parent.clone(),
            _under_resources: self._under_resources,
        };
        core::mem::forget(self);

        r
    }
}
impl<'r, Device: crate::Device> FramebufferObject<'r, Device> {
    /// Create a new framebuffer object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    /// * [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]
    #[implements]
    #[inline]
    pub fn new(device: Device, info: &FramebufferCreateInfo) -> crate::Result<Self> {
        Ok(unsafe { Self::manage(device.new_framebuffer_raw(info, None)?, device) })
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramebufferCreateInfo<'r, 'rs>(
    VkFramebufferCreateInfo,
    core::marker::PhantomData<(
        &'r dyn VkHandle<Handle = VkRenderPass>,
        &'rs [&'r dyn VkHandle<Handle = VkImageView>],
    )>,
);
impl<'r, 'rs> FramebufferCreateInfo<'r, 'rs> {
    #[inline(always)]
    pub fn new(
        render_pass: &'r (impl VkHandle<Handle = VkRenderPass> + ?Sized),
        attachments: &'rs [VkHandleRef<'r, VkImageView>],
        width: u32,
        height: u32,
    ) -> Self {
        Self(
            VkFramebufferCreateInfo {
                sType: VkFramebufferCreateInfo::TYPE,
                pNext: core::ptr::null(),
                renderPass: render_pass.native_ptr(),
                flags: 0,
                attachmentCount: attachments.len() as _,
                pAttachments: slice_as_ptr_empty_null(attachments) as *const _,
                width,
                height,
                layers: 1,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkFramebufferCreateInfo) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkFramebufferCreateInfo {
        self.0
    }

    pub const fn with_layers(mut self, layers: u32) -> Self {
        self.0.layers = layers;
        self
    }
}

pub trait Framebuffer: VkHandle<Handle = VkFramebuffer> {}
DerefContainerBracketImpl!(for Framebuffer {});
GuardsImpl!(for Framebuffer {});
