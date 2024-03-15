use derives::implements;

use crate::{
    ffi_helper::ArrayFFIExtensions, AspectMask, ImageLayout, LoadOp, PipelineStageFlags, RenderPassObject, StoreOp,
    SubpassIndex, VkAccessFlags, VkAttachmentDescription2KHR, VkAttachmentReference2KHR, VkFormat,
    VkRenderPassCreateInfo2KHR, VkSampleCountFlagBits, VkSubpassDependency2KHR, VkSubpassDescription2KHR,
    VulkanStructure, VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT, VK_DEPENDENCY_BY_REGION_BIT,
    VK_PIPELINE_BIND_POINT_GRAPHICS, VK_SAMPLE_COUNT_1_BIT,
};

#[repr(transparent)]
pub struct AttachmentDescription2(VkAttachmentDescription2KHR);
impl AttachmentDescription2 {
    pub const fn new(format: VkFormat) -> Self {
        Self(VkAttachmentDescription2KHR {
            sType: VkAttachmentDescription2KHR::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            format,
            samples: VK_SAMPLE_COUNT_1_BIT,
            loadOp: LoadOp::DontCare as _,
            storeOp: StoreOp::DontCare as _,
            stencilLoadOp: LoadOp::DontCare as _,
            stencilStoreOp: StoreOp::DontCare as _,
            initialLayout: ImageLayout::Undefined as _,
            finalLayout: ImageLayout::Undefined as _,
        })
    }

    #[inline]
    pub fn samples(mut self, samples: VkSampleCountFlagBits) -> Self {
        self.0.samples = samples;
        self
    }

    #[inline]
    pub fn color_memory_op(mut self, load: LoadOp, store: StoreOp) -> Self {
        self.0.loadOp = load as _;
        self.0.storeOp = store as _;
        self
    }

    #[inline]
    pub fn stencil_memory_op(mut self, load: LoadOp, store: StoreOp) -> Self {
        self.0.stencilLoadOp = load as _;
        self.0.stencilStoreOp = store as _;
        self
    }

    #[inline]
    pub fn layout_transition(mut self, init: ImageLayout, fini: ImageLayout) -> Self {
        self.0.initialLayout = init as _;
        self.0.finalLayout = fini as _;
        self
    }

    #[inline]
    pub const fn may_alias(mut self) -> Self {
        self.0.flags |= VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
        self
    }
}

#[repr(transparent)]
pub struct AttachmentReference2(VkAttachmentReference2KHR);
impl AttachmentReference2 {
    pub const fn new(index: u32, aspect_mask: AspectMask, layout: ImageLayout) -> Self {
        Self(VkAttachmentReference2KHR {
            sType: VkAttachmentReference2KHR::TYPE,
            pNext: core::ptr::null(),
            attachment: index,
            layout: layout as _,
            aspectMask: aspect_mask.0,
        })
    }

    #[inline(always)]
    pub const fn color(index: u32, layout: ImageLayout) -> Self {
        Self::new(index, AspectMask::COLOR, layout)
    }

    #[inline(always)]
    pub const fn depth_stencil(index: u32, layout: ImageLayout) -> Self {
        Self::new(index, AspectMask::DEPTH.stencil(), layout)
    }
}

#[repr(transparent)]
pub struct SubpassDescription2<'d>(
    VkSubpassDescription2KHR,
    core::marker::PhantomData<&'d [AttachmentReference2]>,
);
impl<'d> SubpassDescription2<'d> {
    #[inline]
    pub const fn new() -> Self {
        Self(
            VkSubpassDescription2KHR {
                sType: VkSubpassDescription2KHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
                viewMask: 0,
                inputAttachmentCount: 0,
                pInputAttachments: core::ptr::null(),
                colorAttachmentCount: 0,
                pColorAttachments: core::ptr::null(),
                pResolveAttachments: core::ptr::null(),
                pDepthStencilAttachment: core::ptr::null(),
                preserveAttachmentCount: 0,
                pPreserveAttachments: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    #[inline]
    pub fn inputs(mut self, xs: &'d [AttachmentReference2]) -> Self {
        self.0.inputAttachmentCount = xs.len() as _;
        self.0.pInputAttachments = xs.as_ptr_empty_null() as _;
        self
    }

    #[inline]
    pub fn colors(mut self, xs: &'d [AttachmentReference2]) -> Self {
        self.0.colorAttachmentCount = xs.len() as _;
        self.0.pColorAttachments = xs.as_ptr_empty_null() as _;
        self
    }

    #[inline]
    pub fn color_resolves(mut self, xs: &'d [AttachmentReference2]) -> Self {
        if !xs.is_empty() {
            assert_eq!(
                xs.len() as u32,
                self.0.colorAttachmentCount,
                "A number of resolve attachments must match color's"
            );
        }

        self.0.pResolveAttachments = xs.as_ptr_empty_null() as _;
        self
    }

    #[inline]
    pub const fn depth_stencil(mut self, x: &'d AttachmentReference2) -> Self {
        self.0.pDepthStencilAttachment = x as *const _ as _;
        self
    }

    #[inline]
    pub fn preserves(mut self, xs: &'d [u32]) -> Self {
        self.0.preserveAttachmentCount = xs.len() as _;
        self.0.pPreserveAttachments = xs.as_ptr_empty_null();
        self
    }

    #[inline]
    pub const fn view_mask(mut self, mask: u32) -> Self {
        self.0.viewMask = mask;
        self
    }
}

#[repr(transparent)]
pub struct SubpassDependency2(VkSubpassDependency2KHR);
impl SubpassDependency2 {
    #[inline]
    pub const fn new(src: SubpassIndex, dst: SubpassIndex) -> Self {
        Self(VkSubpassDependency2KHR {
            sType: VkSubpassDependency2KHR::TYPE,
            pNext: core::ptr::null(),
            srcSubpass: src.as_api_value(),
            dstSubpass: dst.as_api_value(),
            srcStageMask: 0,
            dstStageMask: 0,
            srcAccessMask: 0,
            dstAccessMask: 0,
            dependencyFlags: 0,
            viewOffset: 0,
        })
    }

    #[inline]
    pub const fn of_execution(mut self, src: PipelineStageFlags, dst: PipelineStageFlags) -> Self {
        self.0.srcStageMask = src.0;
        self.0.dstStageMask = dst.0;
        self
    }

    #[inline]
    pub const fn of_memory(mut self, src: VkAccessFlags, dst: VkAccessFlags) -> Self {
        self.0.srcAccessMask = src;
        self.0.dstAccessMask = dst;
        self
    }

    #[inline]
    pub const fn by_region(mut self) -> Self {
        self.0.dependencyFlags |= VK_DEPENDENCY_BY_REGION_BIT;
        self
    }

    #[inline]
    pub const fn view_offset(mut self, offset: i32) -> Self {
        self.0.viewOffset = offset;
        self
    }
}

#[repr(transparent)]
pub struct RenderPassBuilder2<'d>(
    VkRenderPassCreateInfo2KHR,
    core::marker::PhantomData<(
        &'d [AttachmentDescription2],
        &'d [SubpassDescription2<'d>],
        &'d [SubpassDependency2],
    )>,
);
impl<'d> RenderPassBuilder2<'d> {
    #[inline]
    pub fn new(
        attachments: &'d [AttachmentDescription2],
        subpasses: &'d [SubpassDescription2<'d>],
        dependencies: &'d [SubpassDependency2],
    ) -> Self {
        Self(
            VkRenderPassCreateInfo2KHR {
                sType: VkRenderPassCreateInfo2KHR::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                attachmentCount: attachments.len() as _,
                pAttachments: attachments.as_ptr_empty_null() as _,
                subpassCount: subpasses.len() as _,
                pSubpasses: subpasses.as_ptr_empty_null() as _,
                dependencyCount: dependencies.len() as _,
                pDependencies: dependencies.as_ptr_empty_null() as _,
                correlatedViewMaskCount: 0,
                pCorrelatedViewMasks: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    #[implements]
    pub fn create<Device: crate::Device>(self, device: Device) -> crate::Result<RenderPassObject<Device>> {
        let mut h = core::mem::MaybeUninit::uninit();
        #[cfg(feature = "Allow1_3APIs")]
        unsafe {
            crate::vkresolve::create_render_pass_2(device.native_ptr(), &self.0, core::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| RenderPassObject(h.assume_init(), device))
        }

        #[cfg(not(feature = "Allow1_3APIs"))]
        unsafe {
            crate::vkresolve::create_render_pass_2_khr(device.native_ptr(), &self.0, core::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(move |_| RenderPassObject(h.assume_init(), device))
        }
    }
}
