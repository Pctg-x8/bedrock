//! Vulkan Framebuffer

use derives::implements;

use crate::{
    ffi_helper::{slice_as_ptr_empty_null, ArrayFFIExtensions},
    vk::*,
    DeviceChild, DeviceChildHandle, DeviceChildTransferrable, Image, Transparent, VkHandle, VkObject, VkRawHandle,
    VulkanStructure,
};
use std::ops::*;

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
impl<'r, Device: VkHandle<Handle = VkDevice>> FramebufferObject<'r, Device> {
    #[implements]
    #[inline]
    pub fn new(device: Device, info: &FramebufferCreateInfo<'r, '_>) -> crate::Result<Self> {
        let mut h = core::mem::MaybeUninit::uninit();

        unsafe {
            crate::vkfn::create_framebuffer(device.native_ptr(), &info.0, core::ptr::null(), h.as_mut_ptr())
                .into_result()?;

            Ok(Self::manage(h.assume_init(), device))
        }
    }
}
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

/// Marker trait that can be used as an attachment of a framebuffer (composited trait)
pub trait FramebufferAttachment:
    crate::VkHandle<Handle = VkImageView> + crate::DeviceChild + crate::ImageChild
{
}
impl<T: crate::VkHandle<Handle = VkImageView> + crate::DeviceChild + crate::ImageChild> FramebufferAttachment for T {}

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
        attachments: &'rs [impl Transparent<Target = VkImageView> + 'r],
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

#[deprecated = "use FramebufferCreateInfo"]
pub struct FramebufferBuilder<'r, RenderPass: crate::RenderPass + crate::DeviceChild> {
    info: VkFramebufferCreateInfo,
    render_pass: RenderPass,
    under_resources: Vec<VkImageView>,
    under_resources_marker: core::marker::PhantomData<(
        &'r RenderPass,
        &'r [Box<dyn crate::VkHandle<Handle = VkImageView> + 'r>],
    )>,
}
#[allow(deprecated)]
impl<'r, RenderPass: crate::RenderPass + crate::DeviceChild> FramebufferBuilder<'r, RenderPass> {
    pub fn new(render_pass: RenderPass) -> Self {
        Self {
            info: VkFramebufferCreateInfo {
                sType: VkFramebufferCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                renderPass: render_pass.native_ptr(),
                attachmentCount: 0,
                pAttachments: core::ptr::null(),
                width: 0,
                height: 0,
                layers: 1,
            },
            render_pass,
            under_resources: Vec::new(),
            under_resources_marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn empty(render_pass: RenderPass, size: VkExtent2D) -> Self {
        Self::new(render_pass).size(size.width, size.height)
    }

    #[inline]
    pub fn new_with_attachment(
        render_pass: RenderPass,
        attachment: &'r (impl FramebufferAttachment<ConcreteDevice = RenderPass::ConcreteDevice> + 'r),
    ) -> Self {
        Self::new_with_attachments(render_pass, vec![attachment])
    }

    pub fn new_with_attachments(
        render_pass: RenderPass,
        attachments: Vec<&'r (impl FramebufferAttachment<ConcreteDevice = RenderPass::ConcreteDevice> + 'r)>,
    ) -> Self {
        let size = attachments[0].image().size().wh();

        Self {
            info: VkFramebufferCreateInfo {
                sType: VkFramebufferCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                renderPass: render_pass.native_ptr(),
                attachmentCount: 0,
                pAttachments: core::ptr::null(),
                width: size.width,
                height: size.height,
                layers: 1,
            },
            render_pass,
            under_resources: attachments.into_iter().map(crate::VkHandle::native_ptr).collect(),
            under_resources_marker: core::marker::PhantomData,
        }
    }

    pub fn with_attachment(
        mut self,
        attachment: &'r (impl FramebufferAttachment<ConcreteDevice = RenderPass::ConcreteDevice> + 'r),
    ) -> Self {
        if self.under_resources.is_empty() {
            let size = attachment.image().size().wh();

            self.info.width = size.width;
            self.info.height = size.height;
        }

        self.under_resources.push(attachment.native_ptr());

        self
    }

    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<
            Item = &'r (impl FramebufferAttachment<ConcreteDevice = RenderPass::ConcreteDevice> + 'r),
        >,
    ) -> Self {
        let mut attachments_iter = attachments.into_iter();

        if self.under_resources.is_empty() {
            let Some(first_res) = attachments_iter.next() else {
                return self;
            };

            let size = first_res.image().size().wh();
            self.info.width = size.width;
            self.info.height = size.height;
            self.under_resources.push(first_res.native_ptr());
        }

        self.under_resources
            .extend(attachments_iter.map(crate::VkHandle::native_ptr));

        self
    }

    /// default: 1
    #[inline]
    pub const fn layers(mut self, layers: u32) -> Self {
        self.info.layers = layers;

        self
    }

    /// default: first attachment size
    #[inline]
    pub const fn size(mut self, width: u32, height: u32) -> Self {
        self.info.width = width;
        self.info.height = height;

        self
    }

    #[implements]
    pub fn create(mut self) -> crate::Result<FramebufferObject<'r, RenderPass::ConcreteDevice>>
    where
        RenderPass: DeviceChildTransferrable,
    {
        self.info.attachmentCount = self.under_resources.len() as _;
        self.info.pAttachments = self.under_resources.as_ptr_empty_null();

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_framebuffer(
                self.render_pass.device().native_ptr(),
                &self.info,
                core::ptr::null(),
                h.as_mut_ptr(),
            )
            .into_result()
            .map(|_| FramebufferObject {
                handle: h.assume_init(),
                parent: self.render_pass.transfer_device(),
                _under_resources: core::marker::PhantomData,
            })
        }
    }

    #[implements]
    pub fn create_with_device(
        mut self,
        device: RenderPass::ConcreteDevice,
    ) -> crate::Result<FramebufferObject<'r, RenderPass::ConcreteDevice>> {
        self.info.attachmentCount = self.under_resources.len() as _;
        self.info.pAttachments = self.under_resources.as_ptr_empty_null();

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkfn::create_framebuffer(device.native_ptr(), &self.info, core::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| FramebufferObject {
                    handle: h.assume_init(),
                    parent: device,
                    _under_resources: core::marker::PhantomData,
                })
        }
    }
}

pub trait Framebuffer: VkHandle<Handle = VkFramebuffer> {}
DerefContainerBracketImpl!(for Framebuffer {});
GuardsImpl!(for Framebuffer {});
