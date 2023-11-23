//! Vulkan RenderPass/Framebuffer

use derives::implements;

use crate::{
    ffi_helper::ArrayFFIExtensions, vk::*, DeviceChild, DeviceChildTransferrable, Image, ImageLayout, VkHandle,
    VkObject, VkRawHandle, VulkanStructure,
};
use std::ops::*;

DefineStdDeviceChildObject! {
    /// Opaque handle to a render pass object
    RenderPassObject(VkRenderPass): RenderPass
}

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
impl Deref for AttachmentDescription {
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
