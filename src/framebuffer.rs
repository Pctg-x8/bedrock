//! Vulkan Framebuffer

use derives::implements;

use crate::{
    ffi_helper::ArrayFFIExtensions, vk::*, DeviceChild, DeviceChildTransferrable, Image, VkHandle, VkObject,
    VkRawHandle, VulkanStructure,
};
use std::ops::*;

/// Opaque handle to a framebuffer object
#[derive(VkHandle, VkObject, DeviceChild)]
#[VkObject(type = VK_OBJECT_TYPE_FRAMEBUFFER)]
pub struct FramebufferObject<'r, Device: crate::Device> {
    #[handle]
    pub(crate) handle: VkFramebuffer,
    #[parent]
    pub(crate) parent: Device,
    pub(crate) _under_resources: Vec<Box<dyn crate::ImageView<ConcreteDevice = Device> + 'r>>,
    pub(crate) size: VkExtent2D,
}
unsafe impl<Device> Sync for FramebufferObject<'_, Device> where Device: crate::Device + Sync {}
unsafe impl<Device> Send for FramebufferObject<'_, Device> where Device: crate::Device + Send {}
#[implements]
impl<Device: crate::Device> Drop for FramebufferObject<'_, Device> {
    fn drop(&mut self) {
        unsafe {
            crate::vkresolve::destroy_framebuffer(self.parent.native_ptr(), self.handle, std::ptr::null());
        }
    }
}
impl<Device: crate::Device> Framebuffer for FramebufferObject<'_, Device> {}

pub struct FramebufferBuilder<'r, RenderPass: self::RenderPass> {
    info: VkFramebufferCreateInfo,
    init_size: Option<VkExtent2D>,
    render_pass: RenderPass,
    under_resources: Vec<Box<dyn crate::ImageView<ConcreteDevice = RenderPass::ConcreteDevice> + 'r>>,
}
impl<'r, RenderPass: self::RenderPass> FramebufferBuilder<'r, RenderPass> {
    pub const fn new(render_pass: RenderPass) -> Self {
        Self {
            info: VkFramebufferCreateInfo {
                sType: VkFramebufferCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                renderPass: VkRenderPass::NULL,
                attachmentCount: 0,
                pAttachments: core::ptr::null(),
                width: 0,
                height: 0,
                layers: 1,
            },
            init_size: None,
            render_pass,
            under_resources: Vec::new(),
        }
    }

    pub const fn empty(render_pass: RenderPass, size: VkExtent2D) -> Self {
        Self::new(render_pass).size(size.width, size.height)
    }

    pub fn new_with_attachment(
        render_pass: RenderPass,
        attachment: impl crate::ImageView<ConcreteDevice = RenderPass::ConcreteDevice> + crate::ImageChild + 'r,
    ) -> Self {
        Self::new_with_attachments(render_pass, vec![attachment])
    }

    pub fn new_with_attachments(
        render_pass: RenderPass,
        attachments: Vec<impl crate::ImageView<ConcreteDevice = RenderPass::ConcreteDevice> + crate::ImageChild + 'r>,
    ) -> Self {
        let size = attachments[0].image().size().wh();

        Self {
            info: VkFramebufferCreateInfo {
                sType: VkFramebufferCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                renderPass: VkRenderPass::NULL,
                attachmentCount: 0,
                pAttachments: core::ptr::null(),
                width: size.width,
                height: size.height,
                layers: 1,
            },
            init_size: Some(size),
            render_pass,
            under_resources: attachments.into_iter().map(|x| Box::new(x) as _).collect(),
        }
    }

    pub fn with_attachment(
        mut self,
        attachment: impl crate::ImageView<ConcreteDevice = RenderPass::ConcreteDevice> + crate::ImageChild + 'r,
    ) -> Self {
        if self.under_resources.is_empty() && self.init_size.is_none() {
            self.init_size = Some(attachment.image().size().wh());
        }

        self.under_resources.push(Box::new(attachment) as _);

        self
    }

    pub fn with_attachments(
        mut self,
        attachments: impl IntoIterator<
            Item = impl crate::ImageView<ConcreteDevice = RenderPass::ConcreteDevice> + crate::ImageChild + 'r,
        >,
    ) -> Self {
        let mut attachments_iter = attachments.into_iter();

        if self.under_resources.is_empty() && self.init_size.is_none() {
            let Some(first_res) = attachments_iter.next() else {
                return self;
            };

            self.init_size = Some(first_res.image().size().wh());
            self.under_resources.push(Box::new(first_res) as _);
        }

        self.under_resources.extend(attachments_iter.map(|x| Box::new(x) as _));

        self
    }

    /// default: 1
    pub const fn layers(mut self, layers: u32) -> Self {
        self.info.layers = layers;

        self
    }

    /// default: first attachment size
    pub const fn size(mut self, width: u32, height: u32) -> Self {
        self.init_size = Some(VkExtent2D { width, height });

        self
    }

    #[implements]
    pub fn create(mut self) -> crate::Result<FramebufferObject<'r, RenderPass::ConcreteDevice>>
    where
        RenderPass: DeviceChildTransferrable,
    {
        let size = self.init_size.as_ref().expect("auto-sized builder but no attachments");
        let views = self.under_resources.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();

        self.info.attachmentCount = views.len() as _;
        self.info.pAttachments = views.as_ptr_empty_null();
        self.info.renderPass = self.render_pass.native_ptr();
        self.info.width = size.width;
        self.info.height = size.height;

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_framebuffer(
                self.render_pass.device().native_ptr(),
                &self.info,
                core::ptr::null(),
                h.as_mut_ptr(),
            )
            .into_result()
            .map(|_| FramebufferObject {
                handle: h.assume_init(),
                parent: self.render_pass.transfer_device(),
                _under_resources: self.under_resources,
                size: VkExtent2D {
                    width: self.info.width,
                    height: self.info.height,
                },
            })
        }
    }

    #[implements]
    pub fn create_with_device(
        mut self,
        device: RenderPass::ConcreteDevice,
    ) -> crate::Result<FramebufferObject<'r, RenderPass::ConcreteDevice>> {
        let size = self.init_size.as_ref().expect("auto-sized builder but no attachments");
        let views = self.under_resources.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();

        self.info.attachmentCount = views.len() as _;
        self.info.pAttachments = views.as_ptr_empty_null();
        self.info.renderPass = self.render_pass.native_ptr();
        self.info.width = size.width;
        self.info.height = size.height;

        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_framebuffer(device.native_ptr(), &self.info, core::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| FramebufferObject {
                    handle: h.assume_init(),
                    parent: device,
                    _under_resources: self.under_resources,
                    size: VkExtent2D {
                        width: self.info.width,
                        height: self.info.height,
                    },
                })
        }
    }
}

pub trait RenderPass: VkHandle<Handle = VkRenderPass> + DeviceChild {
    /// Returns the granularity for optimal render area
    #[cfg(feature = "Implements")]
    fn optimal_granularity(&self) -> VkExtent2D {
        let mut e = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::get_render_area_granularity(
                self.device().native_ptr(),
                self.native_ptr(),
                e.as_mut_ptr(),
            );

            e.assume_init()
        }
    }
}
DerefContainerBracketImpl!(for RenderPass {});
GuardsImpl!(for RenderPass {});

pub trait Framebuffer: VkHandle<Handle = VkFramebuffer> + DeviceChild {}
DerefContainerBracketImpl!(for Framebuffer {});
GuardsImpl!(for Framebuffer {});

impl<Device: crate::Device> FramebufferObject<'_, Device> {
    pub const fn size(&self) -> &VkExtent2D {
        &self.size
    }
}
