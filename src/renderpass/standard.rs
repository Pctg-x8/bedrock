use bedrock_vk::{self as brvk, TypedVulkanStructure};

use crate::ffi_helper::slice_as_ptr_empty_null;
use crate::*;

#[repr(transparent)]
pub struct AttachmentDescription(brvk::VkAttachmentDescription);
impl AttachmentDescription {
    pub const fn new(format: brvk::VkFormat, init_layout: ImageLayout, fin_layout: ImageLayout) -> Self {
        Self(brvk::VkAttachmentDescription {
            format,
            samples: 1,
            loadOp: brvk::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            storeOp: brvk::VK_ATTACHMENT_STORE_OP_DONT_CARE,
            stencilLoadOp: brvk::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            stencilStoreOp: brvk::VK_ATTACHMENT_STORE_OP_DONT_CARE,
            initialLayout: init_layout as _,
            finalLayout: fin_layout as _,
            flags: 0,
        })
    }

    pub const fn format(mut self, fmt: brvk::VkFormat) -> Self {
        self.0.format = fmt;
        self
    }

    /// default: don't care
    pub const fn load_op(mut self, op: LoadOp) -> Self {
        self.0.loadOp = op as _;
        self
    }
    /// default: don't care
    pub const fn store_op(mut self, op: StoreOp) -> Self {
        self.0.storeOp = op as _;
        self
    }
    pub const fn color_memory_op(self, load: LoadOp, store: StoreOp) -> Self {
        self.load_op(load).store_op(store)
    }

    /// default: don't care
    pub const fn stencil_load_op(mut self, op: LoadOp) -> Self {
        self.0.stencilLoadOp = op as _;
        self
    }
    /// Default: don't care
    pub const fn stencil_store_op(mut self, op: StoreOp) -> Self {
        self.0.stencilStoreOp = op as _;
        self
    }
    pub const fn stencil_memory_op(self, load: LoadOp, store: StoreOp) -> Self {
        self.stencil_load_op(load).stencil_store_op(store)
    }

    pub const fn init_layout(mut self, layout: ImageLayout) -> Self {
        self.0.initialLayout = layout as _;
        self
    }
    pub const fn fin_layout(mut self, layout: ImageLayout) -> Self {
        self.0.finalLayout = layout as _;
        self
    }
    pub const fn image_layout_transition(self, init_layout: ImageLayout, fin_layout: ImageLayout) -> Self {
        self.init_layout(init_layout).fin_layout(fin_layout)
    }
    pub const fn with_layout_from(self, trans: LayoutTransition) -> Self {
        self.init_layout(trans.from).fin_layout(trans.to)
    }

    pub const fn may_alias(mut self) -> Self {
        self.0.flags |= brvk::VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
        self
    }
    pub const fn no_alias(mut self) -> Self {
        self.0.flags &= !brvk::VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
        self
    }
    pub const fn samples(mut self, count: u32) -> Self {
        self.0.samples = count;
        self
    }
}

#[repr(transparent)]
pub struct AttachmentReference(brvk::VkAttachmentReference);
impl AttachmentReference {
    pub const UNUSED: Self = Self(brvk::VkAttachmentReference {
        attachment: brvk::VK_ATTACHMENT_UNUSED,
        layout: 0,
    });

    pub const fn new(attachment_index: u32, layout: ImageLayout) -> Self {
        Self(brvk::VkAttachmentReference {
            attachment: attachment_index,
            layout: layout as _,
        })
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
    base: brvk::VkSubpassDescription,
    input_lifetime: core::marker::PhantomData<&'r [brvk::VkAttachmentReference]>,
    color_lifetime: core::marker::PhantomData<&'r [brvk::VkAttachmentReference]>,
    resolve_lifetime: core::marker::PhantomData<&'r [brvk::VkAttachmentReference]>,
    depth_stencil_lifetime: core::marker::PhantomData<Option<&'r brvk::VkAttachmentReference>>,
    preserve_lifetime: core::marker::PhantomData<&'r [u32]>,
}
impl<'r> SubpassDescription<'r> {
    pub const fn new() -> Self {
        Self {
            base: brvk::VkSubpassDescription {
                pipelineBindPoint: brvk::VK_PIPELINE_BIND_POINT_GRAPHICS,
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

    pub const fn input_attachments(mut self, inputs: &'r [brvk::VkAttachmentReference]) -> Self {
        self.base.inputAttachmentCount = inputs.len() as _;
        self.base.pInputAttachments = slice_as_ptr_empty_null(inputs);

        self
    }

    pub const fn color_attachments(
        mut self,
        colors: &'r [brvk::VkAttachmentReference],
        resolves: &'r [brvk::VkAttachmentReference],
    ) -> Self {
        assert!(resolves.is_empty() || resolves.len() == colors.len());

        self.base.colorAttachmentCount = colors.len() as _;
        self.base.pColorAttachments = slice_as_ptr_empty_null(colors);
        self.base.pResolveAttachments = slice_as_ptr_empty_null(resolves);

        self
    }

    pub const fn depth_stencil_attachment(mut self, a: &'r brvk::VkAttachmentReference) -> Self {
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
    base: brvk::VkRenderPassCreateInfo,
    attachments: core::marker::PhantomData<&'r [AttachmentDescription]>,
    subpasses: core::marker::PhantomData<&'r [SubpassDescription<'r>]>,
    dependencies: core::marker::PhantomData<&'r [brvk::VkSubpassDependency]>,
}
impl<'r> RenderPassCreateInfo<'r> {
    pub const fn new(
        attachments: &'r [AttachmentDescription],
        subpasses: &'r [SubpassDescription<'r>],
        dependencies: &'r [brvk::VkSubpassDependency],
    ) -> Self {
        Self {
            base: brvk::VkRenderPassCreateInfo {
                sType: brvk::VkRenderPassCreateInfo::TYPE,
                pNext: core::ptr::null(),
                flags: 0,
                attachmentCount: attachments.len() as _,
                pAttachments: slice_as_ptr_empty_null(attachments).cast(),
                subpassCount: subpasses.len() as _,
                pSubpasses: slice_as_ptr_empty_null(subpasses).cast(),
                dependencyCount: dependencies.len() as _,
                pDependencies: slice_as_ptr_empty_null(dependencies) as _,
            },
            attachments: core::marker::PhantomData,
            subpasses: core::marker::PhantomData,
            dependencies: core::marker::PhantomData,
        }
    }

    /// # Safety
    ///
    /// `raw` must be a valid [`VkRenderPassCreateInfo`] struct.
    pub const unsafe fn from_raw(raw: brvk::VkRenderPassCreateInfo) -> Self {
        Self {
            base: raw,
            attachments: core::marker::PhantomData,
            subpasses: core::marker::PhantomData,
            dependencies: core::marker::PhantomData,
        }
    }

    pub const fn into_raw(self) -> brvk::VkRenderPassCreateInfo {
        self.base
    }
}
#[implements]
impl super::AnyRenderPassCreateInfo for RenderPassCreateInfo<'_> {
    fn execute(
        &self,
        device: &(impl crate::Device + ?Sized),
        allocation_callbacks: Option<&brvk::VkAllocationCallbacks>,
    ) -> crate::Result<brvk::VkRenderPass> {
        device.new_render_pass(self, allocation_callbacks)
    }
}
