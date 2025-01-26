use derives::implements;

use crate::{
    AspectMask, ImageLayout, LayoutTransition, LoadOp, PipelineStageFlags, StoreOp, VkAccessFlags,
    VkAttachmentDescription2KHR, VkAttachmentReference2KHR, VkFormat, VkRenderPassCreateInfo2KHR,
    VkSampleCountFlagBits, VkSubpassDependency2KHR, VkSubpassDescription2KHR, VulkanStructure,
    VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT, VK_DEPENDENCY_BY_REGION_BIT, VK_PIPELINE_BIND_POINT_GRAPHICS,
    VK_SAMPLE_COUNT_1_BIT, VK_SUBPASS_EXTERNAL,
};

use super::{ffi_helper::slice_as_ptr_empty_null, VK_ATTACHMENT_UNUSED};

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

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
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

    pub const fn samples(mut self, samples: VkSampleCountFlagBits) -> Self {
        self.0.samples = samples;
        self
    }

    pub const fn color_memory_op(mut self, load: LoadOp, store: StoreOp) -> Self {
        self.0.loadOp = load as _;
        self.0.storeOp = store as _;
        self
    }

    pub const fn stencil_memory_op(mut self, load: LoadOp, store: StoreOp) -> Self {
        self.0.stencilLoadOp = load as _;
        self.0.stencilStoreOp = store as _;
        self
    }

    pub const fn layout_transition(mut self, init: ImageLayout, fini: ImageLayout) -> Self {
        self.0.initialLayout = init as _;
        self.0.finalLayout = fini as _;
        self
    }

    pub const fn layout(self, layout: ImageLayout) -> Self {
        self.layout_transition(layout, layout)
    }

    pub const fn with_layout_from(self, trans: LayoutTransition) -> Self {
        self.layout_transition(trans.from, trans.to)
    }

    pub const fn with_layout_to(self, trans: LayoutTransition) -> Self {
        self.layout_transition(trans.from, trans.to)
    }

    pub const fn may_alias(mut self) -> Self {
        self.0.flags |= VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
        self
    }
}

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AttachmentReference2(VkAttachmentReference2KHR);
impl AttachmentReference2 {
    pub const fn new(index: u32, aspect_mask: AspectMask, layout: ImageLayout) -> Self {
        Self(VkAttachmentReference2KHR {
            sType: VkAttachmentReference2KHR::TYPE,
            pNext: core::ptr::null(),
            attachment: index,
            layout: layout as _,
            aspectMask: aspect_mask.bits(),
        })
    }

    /// An unused attachment
    pub const UNUSED: Self = Self::new(VK_ATTACHMENT_UNUSED, AspectMask::EMPTY, ImageLayout::Undefined);

    /// Represents an attachment reference that references color aspect of the attachment.
    pub const fn color(index: u32, layout: ImageLayout) -> Self {
        Self::new(index, AspectMask::COLOR, layout)
    }

    /// Represents an attachment reference that references depth and stencil aspect of the attachment.
    pub const fn depth_stencil(index: u32, layout: ImageLayout) -> Self {
        Self::new(index, AspectMask::DEPTH.merge(AspectMask::STENCIL), layout)
    }

    /// Optimal constructor for ShaderReadOnlyOpt reference.
    pub const fn shader_color_readonly_opt(index: u32) -> Self {
        Self::color(index, ImageLayout::ShaderReadOnlyOpt)
    }

    /// Optimal constructor for ColorAttachmentOpt reference.
    pub const fn color_attachment_opt(index: u32) -> Self {
        Self::color(index, ImageLayout::ColorAttachmentOpt)
    }

    /// Optimal constructor for DepthStencilAttachmentOpt reference.
    pub const fn depth_stencil_attachment_opt(index: u32) -> Self {
        Self::depth_stencil(index, ImageLayout::DepthStencilAttachmentOpt)
    }

    /// Optimal constructor for DepthStencilReadOnlyOpt reference.
    pub const fn depth_stencil_readonly_opt(index: u32) -> Self {
        Self::depth_stencil(index, ImageLayout::DepthStencilReadOnlyOpt)
    }
}

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SubpassDescription2<'d>(
    VkSubpassDescription2KHR,
    core::marker::PhantomData<&'d [AttachmentReference2]>,
);
impl<'d> SubpassDescription2<'d> {
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

    pub const fn inputs(mut self, xs: &'d [AttachmentReference2]) -> Self {
        self.0.inputAttachmentCount = xs.len() as _;
        self.0.pInputAttachments = slice_as_ptr_empty_null(xs) as _;
        self
    }

    pub const fn colors(mut self, xs: &'d [AttachmentReference2]) -> Self {
        self.0.colorAttachmentCount = xs.len() as _;
        self.0.pColorAttachments = slice_as_ptr_empty_null(xs) as _;
        self
    }

    pub fn color_resolves(mut self, xs: &'d [AttachmentReference2]) -> Self {
        if !xs.is_empty() {
            assert_eq!(
                xs.len() as u32,
                self.0.colorAttachmentCount,
                "A number of resolve attachments must match color's"
            );
        }

        self.0.pResolveAttachments = slice_as_ptr_empty_null(xs) as _;
        self
    }

    pub const fn depth_stencil(mut self, x: &'d AttachmentReference2) -> Self {
        self.0.pDepthStencilAttachment = x as *const _ as _;
        self
    }

    pub const fn preserves(mut self, xs: &'d [u32]) -> Self {
        self.0.preserveAttachmentCount = xs.len() as _;
        self.0.pPreserveAttachments = slice_as_ptr_empty_null(xs);
        self
    }

    pub const fn view_mask(mut self, mask: u32) -> Self {
        self.0.viewMask = mask;
        self
    }
}

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SubpassDependency2(VkSubpassDependency2KHR);
impl SubpassDependency2 {
    pub const fn new(src: SubpassIndex, dst: SubpassIndex) -> Self {
        Self(VkSubpassDependency2KHR {
            sType: VkSubpassDependency2KHR::TYPE,
            pNext: core::ptr::null(),
            srcSubpass: src.as_vk(),
            dstSubpass: dst.as_vk(),
            srcStageMask: 0,
            dstStageMask: 0,
            srcAccessMask: 0,
            dstAccessMask: 0,
            dependencyFlags: 0,
            viewOffset: 0,
        })
    }

    pub const fn of_execution(mut self, src: PipelineStageFlags, dst: PipelineStageFlags) -> Self {
        self.0.srcStageMask = src.0;
        self.0.dstStageMask = dst.0;
        self
    }

    pub const fn of_memory(mut self, src: VkAccessFlags, dst: VkAccessFlags) -> Self {
        self.0.srcAccessMask = src;
        self.0.dstAccessMask = dst;
        self
    }

    pub const fn by_region(mut self) -> Self {
        self.0.dependencyFlags |= VK_DEPENDENCY_BY_REGION_BIT;
        self
    }

    pub const fn view_offset(mut self, offset: i32) -> Self {
        self.0.viewOffset = offset;
        self
    }
}

#[repr(transparent)]
pub struct RenderPassCreateInfo2<'d>(
    VkRenderPassCreateInfo2KHR,
    core::marker::PhantomData<(
        &'d [AttachmentDescription2],
        &'d [SubpassDescription2<'d>],
        &'d [SubpassDependency2],
    )>,
);
impl<'d> RenderPassCreateInfo2<'d> {
    pub const fn new(
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
                pAttachments: slice_as_ptr_empty_null(attachments) as _,
                subpassCount: subpasses.len() as _,
                pSubpasses: slice_as_ptr_empty_null(subpasses) as _,
                dependencyCount: dependencies.len() as _,
                pDependencies: slice_as_ptr_empty_null(dependencies) as _,
                correlatedViewMaskCount: 0,
                pCorrelatedViewMasks: core::ptr::null(),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkRenderPassCreateInfo2KHR) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkRenderPassCreateInfo2KHR {
        self.0
    }

    pub(crate) const fn as_raw_ref(&self) -> &VkRenderPassCreateInfo2KHR {
        &self.0
    }
}
#[implements]
impl super::AnyRenderPassCreateInfo for RenderPassCreateInfo2<'_> {
    fn execute(
        &self,
        device: &(impl crate::Device + ?Sized),
        allocation_callbacks: Option<&super::VkAllocationCallbacks>,
    ) -> crate::Result<super::VkRenderPass> {
        device.new_render_pass2(&self, allocation_callbacks)
    }
}
