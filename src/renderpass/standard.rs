use crate::vk::*;
use crate::*;

use self::ffi_helper::ArrayFFIExtensions;

/// Builder structure to construct the `VkAttachmentDescription`
#[repr(transparent)]
#[derive(Clone)]
pub struct AttachmentDescription(VkAttachmentDescription);
impl AttachmentDescription {
    pub const fn new(format: VkFormat, init_layout: ImageLayout, fin_layout: ImageLayout) -> Self {
        Self(VkAttachmentDescription {
            format,
            samples: 1,
            loadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            storeOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
            stencilLoadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            stencilStoreOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
            initialLayout: init_layout as _,
            finalLayout: fin_layout as _,
            flags: 0,
        })
    }

    pub const fn format(mut self, fmt: VkFormat) -> Self {
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
    /// default: don't care
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

    pub const fn may_alias(mut self) -> Self {
        self.0.flags |= VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
        self
    }
    pub const fn no_alias(mut self) -> Self {
        self.0.flags &= !VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
        self
    }
    pub const fn samples(mut self, count: u32) -> Self {
        self.0.samples = count;
        self
    }

    pub fn mod_format(&mut self, fmt: VkFormat) -> &mut Self {
        self.0.format = fmt;
        self
    }
    pub fn mod_load_op(&mut self, op: LoadOp) -> &mut Self {
        self.0.loadOp = op as _;
        self
    }
    pub fn mod_store_op(&mut self, op: StoreOp) -> &mut Self {
        self.0.storeOp = op as _;
        self
    }
    pub fn mod_stencil_load_op(&mut self, op: LoadOp) -> &mut Self {
        self.0.stencilLoadOp = op as _;
        self
    }
    pub fn mod_stencil_store_op(&mut self, op: StoreOp) -> &mut Self {
        self.0.stencilStoreOp = op as _;
        self
    }
    pub fn mod_init_layout(&mut self, layout: ImageLayout) -> &mut Self {
        self.0.initialLayout = layout as _;
        self
    }
    pub fn mod_fin_layout(&mut self, layout: ImageLayout) -> &mut Self {
        self.0.finalLayout = layout as _;
        self
    }
    pub fn mod_may_alias(&mut self) -> &mut Self {
        self.0.flags |= VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
        self
    }
    pub fn mod_no_alias(&mut self) -> &mut Self {
        self.0.flags &= !VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
        self
    }
    pub fn mod_samples(&mut self, count: u32) -> &mut Self {
        self.0.samples = count;
        self
    }
}
impl AsRef<VkAttachmentDescription> for AttachmentDescription {
    fn as_ref(&self) -> &VkAttachmentDescription {
        &self.0
    }
}
impl core::ops::Deref for AttachmentDescription {
    type Target = VkAttachmentDescription;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<AttachmentDescription> for VkAttachmentDescription {
    fn from(x: AttachmentDescription) -> Self {
        x.0
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
pub struct SubpassDescription {
    input_attachments: Vec<VkAttachmentReference>,
    color_attachments: Vec<VkAttachmentReference>,
    resolve_attachments: Vec<VkAttachmentReference>,
    depth_stencil_attachment: Option<VkAttachmentReference>,
    preserve_attachments: Vec<u32>,
}

/// Builder structure to construct the `RenderPass`
pub struct RenderPassBuilder {
    attachments: Vec<VkAttachmentDescription>,
    subpasses: Vec<SubpassDescription>,
    dependencies: Vec<VkSubpassDependency>,
}
impl RenderPassBuilder {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            attachments: Vec::new(),
            subpasses: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    pub fn add_attachment(mut self, desc: impl Into<VkAttachmentDescription>) -> Self {
        self.attachments.push(desc.into());
        self
    }
    pub fn add_subpass(mut self, desc: SubpassDescription) -> Self {
        self.subpasses.push(desc);
        self
    }
    pub fn add_dependency(mut self, desc: VkSubpassDependency) -> Self {
        self.dependencies.push(desc);
        self
    }

    pub fn add_attachments<A: Into<VkAttachmentDescription>>(
        mut self,
        collection: impl IntoIterator<Item = A>,
    ) -> Self {
        self.attachments.extend(collection.into_iter().map(Into::into));
        self
    }
    pub fn add_subpasses(mut self, collection: impl IntoIterator<Item = SubpassDescription>) -> Self {
        self.subpasses.extend(collection);
        self
    }
    pub fn add_dependencies(mut self, collection: impl IntoIterator<Item = VkSubpassDependency>) -> Self {
        self.dependencies.extend(collection);
        self
    }

    pub fn attachment_mut(&mut self, index: usize) -> &mut AttachmentDescription {
        unsafe { &mut *(&mut self.attachments[index] as *mut _ as *mut _) }
    }
    pub fn subpass_mut(&mut self, index: usize) -> &mut SubpassDescription {
        &mut self.subpasses[index]
    }
    pub fn dependency_mut(&mut self, index: usize) -> &mut VkSubpassDependency {
        &mut self.dependencies[index]
    }
}

impl SubpassDescription {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            input_attachments: Vec::new(),
            color_attachments: Vec::new(),
            resolve_attachments: Vec::new(),
            depth_stencil_attachment: None,
            preserve_attachments: Vec::new(),
        }
    }

    pub fn add_input(mut self, index: u32, layout: ImageLayout) -> Self {
        self.input_attachments.push(VkAttachmentReference {
            attachment: index,
            layout: layout as _,
        });
        self
    }
    pub fn add_color_output(mut self, index: u32, layout: ImageLayout, resolve: Option<(u32, ImageLayout)>) -> Self {
        if let Some((i, l)) = resolve {
            while self.resolve_attachments.len() < self.color_attachments.len() {
                self.resolve_attachments.push(VkAttachmentReference {
                    attachment: VK_ATTACHMENT_UNUSED,
                    layout: 0 as _,
                });
            }
            self.resolve_attachments.push(VkAttachmentReference {
                attachment: i,
                layout: l as _,
            });
        }
        self.color_attachments.push(VkAttachmentReference {
            attachment: index,
            layout: layout as _,
        });
        self
    }
    pub fn depth_stencil(mut self, index: u32, layout: ImageLayout) -> Self {
        self.depth_stencil_attachment = Some(VkAttachmentReference {
            attachment: index,
            layout: layout as _,
        });
        self
    }
    pub fn add_preserve(mut self, index: u32) -> Self {
        self.preserve_attachments.push(index);
        self
    }
    pub fn add_preserves(mut self, indices: impl IntoIterator<Item = u32>) -> Self {
        self.preserve_attachments.extend(indices);
        self
    }

    pub fn add_input_borrow(&mut self, index: u32, layout: ImageLayout) -> &mut Self {
        self.input_attachments.push(VkAttachmentReference {
            attachment: index,
            layout: layout as _,
        });
        self
    }
    pub fn add_color_output_borrow(
        &mut self,
        index: u32,
        layout: ImageLayout,
        resolve: Option<(u32, ImageLayout)>,
    ) -> &mut Self {
        if let Some((i, l)) = resolve {
            while self.resolve_attachments.len() < self.color_attachments.len() {
                self.resolve_attachments.push(VkAttachmentReference {
                    attachment: VK_ATTACHMENT_UNUSED,
                    layout: 0 as _,
                });
            }
            self.resolve_attachments.push(VkAttachmentReference {
                attachment: i,
                layout: l as _,
            });
        }
        self.color_attachments.push(VkAttachmentReference {
            attachment: index,
            layout: layout as _,
        });
        self
    }
    pub fn depth_stencil_borrow(&mut self, index: u32, layout: ImageLayout) -> &mut Self {
        self.depth_stencil_attachment = Some(VkAttachmentReference {
            attachment: index,
            layout: layout as _,
        });
        self
    }
    pub fn add_preserve_borrow(&mut self, index: u32) -> &mut Self {
        self.preserve_attachments.push(index);
        self
    }
    pub fn add_preserves_borrow(&mut self, indices: impl IntoIterator<Item = u32>) -> &mut Self {
        self.preserve_attachments.extend(indices);
        self
    }
}
#[cfg(feature = "Implements")]
impl RenderPassBuilder {
    /// Create a new render pass object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn create<Device: crate::Device>(&self, device: Device) -> crate::Result<RenderPassObject<Device>> {
        let subpasses = self
            .subpasses
            .iter()
            .map(|x| VkSubpassDescription {
                flags: 0,
                pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
                inputAttachmentCount: x.input_attachments.len() as _,
                pInputAttachments: x.input_attachments.as_ptr_empty_null(),
                colorAttachmentCount: x.color_attachments.len() as _,
                pColorAttachments: x.color_attachments.as_ptr_empty_null(),
                pResolveAttachments: x.resolve_attachments.as_ptr_empty_null(),
                pDepthStencilAttachment: x
                    .depth_stencil_attachment
                    .as_ref()
                    .map_or(core::ptr::null(), |p| p as *const _),
                preserveAttachmentCount: x.preserve_attachments.len() as _,
                pPreserveAttachments: x.preserve_attachments.as_ptr_empty_null(),
            })
            .collect::<Vec<_>>();
        let cinfo = VkRenderPassCreateInfo {
            sType: VkRenderPassCreateInfo::TYPE,
            pNext: std::ptr::null(),
            flags: 0,
            attachmentCount: self.attachments.len() as _,
            pAttachments: self.attachments.as_ptr_empty_null(),
            subpassCount: subpasses.len() as _,
            pSubpasses: subpasses.as_ptr_empty_null(),
            dependencyCount: self.dependencies.len() as _,
            pDependencies: self.dependencies.as_ptr_empty_null(),
        };

        let mut h = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_render_pass(device.native_ptr(), &cinfo, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| RenderPassObject(h.assume_init(), device))
        }
    }
}
