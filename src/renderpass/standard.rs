use ffi_helper::slice_as_ptr_empty_null;

use crate::vk::*;
use crate::*;

impl VkAttachmentDescription {
    pub const fn new(format: VkFormat, init_layout: ImageLayout, fin_layout: ImageLayout) -> Self {
        Self {
            format,
            samples: 1,
            loadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            storeOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
            stencilLoadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            stencilStoreOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
            initialLayout: init_layout as _,
            finalLayout: fin_layout as _,
            flags: 0,
        }
    }

    pub const fn format(self, fmt: VkFormat) -> Self {
        Self { format: fmt, ..self }
    }

    /// default: don't care
    pub const fn load_op(self, op: LoadOp) -> Self {
        Self {
            loadOp: op as _,
            ..self
        }
    }
    /// default: don't care
    pub const fn store_op(self, op: StoreOp) -> Self {
        Self {
            storeOp: op as _,
            ..self
        }
    }
    pub const fn color_memory_op(self, load: LoadOp, store: StoreOp) -> Self {
        self.load_op(load).store_op(store)
    }

    /// default: don't care
    pub const fn stencil_load_op(self, op: LoadOp) -> Self {
        Self {
            stencilLoadOp: op as _,
            ..self
        }
    }
    /// default: don't care
    pub const fn stencil_store_op(self, op: StoreOp) -> Self {
        Self {
            stencilStoreOp: op as _,
            ..self
        }
    }
    pub const fn stencil_memory_op(self, load: LoadOp, store: StoreOp) -> Self {
        self.stencil_load_op(load).stencil_store_op(store)
    }

    pub const fn init_layout(self, layout: ImageLayout) -> Self {
        Self {
            initialLayout: layout as _,
            ..self
        }
    }
    pub const fn fin_layout(self, layout: ImageLayout) -> Self {
        Self {
            finalLayout: layout as _,
            ..self
        }
    }
    pub const fn image_layout_transition(self, init_layout: ImageLayout, fin_layout: ImageLayout) -> Self {
        self.init_layout(init_layout).fin_layout(fin_layout)
    }
    pub const fn with_layout_from(self, trans: LayoutTransition) -> Self {
        self.init_layout(trans.from).fin_layout(trans.to)
    }

    pub const fn may_alias(mut self) -> Self {
        self.flags |= VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
        self
    }
    pub const fn no_alias(mut self) -> Self {
        self.flags &= !VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
        self
    }
    pub const fn samples(self, count: u32) -> Self {
        Self { samples: count, ..self }
    }
}

impl VkAttachmentReference {
    pub const UNUSED: Self = Self {
        attachment: VK_ATTACHMENT_UNUSED,
        layout: 0,
    };

    pub const fn new(attachment_index: u32, layout: ImageLayout) -> Self {
        Self {
            attachment: attachment_index,
            layout: layout as _,
        }
    }
}

/// Builder structure to construct the `VkSubpassDescription`
///
/// ## The `layout` parameter of each attachment
///
/// The `layout` parameter describes what layout the attachment will be in during the subpass.
///
/// ## How *input attachments* work
///
/// * Each element of the array corresponds to an input attachment unit number in the shader.
///   * i. e. if the shader declares an input variable `layout(input_attachment_index=X, set=Y, binding=Z)`
///     then it uses the attachment provided in `input_attachments[X]`.
/// * Input attachments *must* also be bound to the pipeline with a descriptor set, with the input attachment descriptor
///   written in the location (set=Y, binding=Z).
/// * Fragment shaders *can* use subpass input variables to access the contents of an input attachment at the fragment's
///   (x, y, layer) framebuffer coordinates.
///
#[repr(transparent)]
#[derive(Clone)]
pub struct SubpassDescription<'r> {
    base: VkSubpassDescription,
    input_lifetime: core::marker::PhantomData<&'r [VkAttachmentReference]>,
    color_lifetime: core::marker::PhantomData<&'r [VkAttachmentReference]>,
    resolve_lifetime: core::marker::PhantomData<&'r [VkAttachmentReference]>,
    depth_stencil_lifetime: core::marker::PhantomData<Option<&'r VkAttachmentReference>>,
    preserve_lifetime: core::marker::PhantomData<&'r [u32]>,
}
impl<'r> SubpassDescription<'r> {
    pub const fn new() -> Self {
        Self {
            base: VkSubpassDescription {
                pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
                flags: 0,
                inputAttachmentCount: 0,
                pInputAttachments: core::ptr::null(),
                colorAttachmentCount: 0,
                pColorAttachments: core::ptr::null(),
                pResolveAttachments: core::ptr::null(),
                pDepthStencilAttachment: core::ptr::null(),
                preserveAttachmentCount: 0,
                pPreserveAttachments: core::ptr::null(),
            },
            input_lifetime: core::marker::PhantomData,
            color_lifetime: core::marker::PhantomData,
            resolve_lifetime: core::marker::PhantomData,
            depth_stencil_lifetime: core::marker::PhantomData,
            preserve_lifetime: core::marker::PhantomData,
        }
    }

    pub const fn input_attachments(mut self, inputs: &'r [VkAttachmentReference]) -> Self {
        self.base.inputAttachmentCount = inputs.len() as _;
        self.base.pInputAttachments = slice_as_ptr_empty_null(inputs);

        self
    }

    pub const fn color_attachments(
        mut self,
        colors: &'r [VkAttachmentReference],
        resolves: &'r [VkAttachmentReference],
    ) -> Self {
        assert!(resolves.is_empty() || resolves.len() == colors.len());

        self.base.colorAttachmentCount = colors.len() as _;
        self.base.pColorAttachments = slice_as_ptr_empty_null(colors);
        self.base.pResolveAttachments = slice_as_ptr_empty_null(resolves);

        self
    }

    pub const fn depth_stencil_attachment(mut self, a: &'r VkAttachmentReference) -> Self {
        self.base.pDepthStencilAttachment = a as *const _ as _;

        self
    }

    pub const fn preserved_attachments(mut self, a: &'r [u32]) -> Self {
        self.base.preserveAttachmentCount = a.len() as _;
        self.base.pPreserveAttachments = if a.is_empty() { core::ptr::null() } else { a.as_ptr() };

        self
    }
}

/// Builder structure to construct the `RenderPass`
#[repr(transparent)]
#[derive(Clone)]
pub struct RenderPassCreateInfo<'r> {
    base: VkRenderPassCreateInfo,
    attachments: core::marker::PhantomData<&'r [VkAttachmentDescription]>,
    subpasses: core::marker::PhantomData<&'r [SubpassDescription<'r>]>,
    dependencies: core::marker::PhantomData<&'r [VkSubpassDependency]>,
}
impl<'r> RenderPassCreateInfo<'r> {
    pub const fn new(
        attachments: &'r [VkAttachmentDescription],
        subpasses: &'r [SubpassDescription<'r>],
        dependencies: &'r [VkSubpassDependency],
    ) -> Self {
        Self {
            base: VkRenderPassCreateInfo {
                sType: VkRenderPassCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                attachmentCount: attachments.len() as _,
                pAttachments: slice_as_ptr_empty_null(attachments) as _,
                subpassCount: subpasses.len() as _,
                pSubpasses: slice_as_ptr_empty_null(subpasses) as _,
                dependencyCount: dependencies.len() as _,
                pDependencies: slice_as_ptr_empty_null(dependencies) as _,
            },
            attachments: core::marker::PhantomData,
            subpasses: core::marker::PhantomData,
            dependencies: core::marker::PhantomData,
        }
    }

    pub const unsafe fn from_raw(raw: VkRenderPassCreateInfo) -> Self {
        Self {
            base: raw,
            attachments: core::marker::PhantomData,
            subpasses: core::marker::PhantomData,
            dependencies: core::marker::PhantomData,
        }
    }

    pub const fn into_raw(self) -> VkRenderPassCreateInfo {
        self.base
    }

    pub(crate) const fn as_raw_ref(&self) -> &VkRenderPassCreateInfo {
        &self.base
    }
}
#[implements]
impl super::AnyRenderPassCreateInfo for RenderPassCreateInfo<'_> {
    fn execute(
        &self,
        device: &(impl crate::Device + ?Sized),
        allocation_callbacks: Option<&VkAllocationCallbacks>,
    ) -> crate::Result<VkRenderPass> {
        device.new_render_pass(self, allocation_callbacks)
    }
}
