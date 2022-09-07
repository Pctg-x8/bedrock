//! Vulkan RenderPass/Framebuffer

use crate::vk::*;
#[cfg(feature = "Implements")]
use crate::{
    vkresolve::{Resolver, ResolverInterface},
    VkResultHandler,
};
use crate::{ImageLayout, VkHandle};
use std::ops::*;

/// Opaque handle to a render pass object
#[derive(VkHandle)]
#[object_type = "VK_OBJECT_TYPE_RENDER_PASS"]
pub struct RenderPass<Device>(VkRenderPass, Device)
where
    Device: VkHandle<Handle = VkDevice>;
#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>> Drop for RenderPass<Device> {
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_render_pass(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}

/// Opaque handle to a framebuffer object
#[derive(VkHandle)]
#[object_type = "VK_OBJECT_TYPE_FRAMEBUFFER"]
pub struct Framebuffer<Device, ImageView>(VkFramebuffer, Device, Vec<ImageView>, VkExtent2D)
where
    Device: VkHandle<Handle = VkDevice>,
    ImageView: VkHandle<Handle = VkImageView>;
#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>, ImageView: VkHandle<Handle = VkImageView>> Drop
    for Framebuffer<Device, ImageView>
{
    fn drop(&mut self) {
        unsafe {
            Resolver::get().destroy_framebuffer(self.1.native_ptr(), self.0, std::ptr::null());
        }
    }
}

#[cfg(feature = "Multithreaded")]
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync> Sync for RenderPass<Device> {}
#[cfg(feature = "Multithreaded")]
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send> Send for RenderPass<Device> {}
#[cfg(feature = "Multithreaded")]
unsafe impl<Device: VkHandle<Handle = VkDevice> + Sync, ImageView: VkHandle<Handle = VkImageView> + Sync> Sync
    for Framebuffer<Device, ImageView>
{
}
#[cfg(feature = "Multithreaded")]
unsafe impl<Device: VkHandle<Handle = VkDevice> + Send, ImageView: VkHandle<Handle = VkImageView> + Send> Send
    for Framebuffer<Device, ImageView>
{
}

/// Builder structure to construct the `VkAttachmentDescription`
#[repr(transparent)]
#[derive(Clone)]
pub struct AttachmentDescription(VkAttachmentDescription);
impl AttachmentDescription {
    pub const fn new(format: VkFormat, init_layout: ImageLayout, fin_layout: ImageLayout) -> Self {
        AttachmentDescription(VkAttachmentDescription {
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
    pub const fn load_op(mut self, op: LoadOp) -> Self {
        self.0.loadOp = op as _;
        self
    }
    pub const fn store_op(mut self, op: StoreOp) -> Self {
        self.0.storeOp = op as _;
        self
    }
    pub const fn stencil_load_op(mut self, op: LoadOp) -> Self {
        self.0.stencilLoadOp = op as _;
        self
    }
    pub const fn stencil_store_op(mut self, op: StoreOp) -> Self {
        self.0.stencilStoreOp = op as _;
        self
    }
    pub const fn init_layout(mut self, layout: ImageLayout) -> Self {
        self.0.initialLayout = layout as _;
        self
    }
    pub const fn fin_layout(mut self, layout: ImageLayout) -> Self {
        self.0.finalLayout = layout as _;
        self
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

    pub fn add_attachment(&mut self, desc: impl Into<VkAttachmentDescription>) -> &mut Self {
        self.attachments.push(desc.into());
        self
    }
    pub fn add_subpass(&mut self, desc: SubpassDescription) -> &mut Self {
        self.subpasses.push(desc);
        self
    }
    pub fn add_dependency(&mut self, desc: VkSubpassDependency) -> &mut Self {
        self.dependencies.push(desc);
        self
    }

    pub fn add_attachments<A: Into<VkAttachmentDescription>>(
        &mut self,
        collection: impl IntoIterator<Item = A>,
    ) -> &mut Self {
        self.attachments.extend(collection.into_iter().map(Into::into));
        self
    }
    pub fn add_subpasses(&mut self, collection: impl IntoIterator<Item = SubpassDescription>) -> &mut Self {
        self.subpasses.extend(collection);
        self
    }
    pub fn add_dependencies(&mut self, collection: impl IntoIterator<Item = VkSubpassDependency>) -> &mut Self {
        self.dependencies.extend(collection);
        self
    }

    pub fn mod_attachment(&mut self, index: usize) -> &mut AttachmentDescription {
        unsafe { &mut *(&mut self.attachments[index] as *mut _ as *mut _) }
    }
    pub fn mod_subpass(&mut self, index: usize) -> &mut SubpassDescription {
        &mut self.subpasses[index]
    }
    pub fn mod_dependency(&mut self, index: usize) -> &mut VkSubpassDependency {
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
    pub fn create<Device: VkHandle<Handle = VkDevice>>(&self, device: Device) -> crate::Result<RenderPass<Device>> {
        let subpasses = self
            .subpasses
            .iter()
            .map(|x| VkSubpassDescription {
                inputAttachmentCount: x.input_attachments.len() as _,
                pInputAttachments: x.input_attachments.as_ptr(),
                colorAttachmentCount: x.color_attachments.len() as _,
                pColorAttachments: x.color_attachments.as_ptr(),
                pResolveAttachments: if x.resolve_attachments.is_empty() {
                    std::ptr::null()
                } else {
                    x.resolve_attachments.as_ptr()
                },
                pDepthStencilAttachment: if let Some(ref x) = x.depth_stencil_attachment {
                    x
                } else {
                    std::ptr::null()
                },
                preserveAttachmentCount: x.preserve_attachments.len() as _,
                pPreserveAttachments: x.preserve_attachments.as_ptr(),
                ..Default::default()
            })
            .collect::<Vec<_>>();
        let cinfo = VkRenderPassCreateInfo {
            attachmentCount: self.attachments.len() as _,
            pAttachments: self.attachments.as_ptr(),
            subpassCount: subpasses.len() as _,
            pSubpasses: subpasses.as_ptr(),
            dependencyCount: self.dependencies.len() as _,
            pDependencies: self.dependencies.as_ptr(),
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe { Resolver::get().create_render_pass(device.native_ptr(), &cinfo, std::ptr::null(), &mut h) }
            .into_result()
            .map(|_| RenderPass(h, device))
    }
}
#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>, ImageView: VkHandle<Handle = VkImageView>> Framebuffer<Device, ImageView> {
    /// Create a new framebuffer object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn new(
        mold: &RenderPass<Device>,
        attachment_objects: Vec<ImageView>,
        size: &VkExtent2D,
        layers: u32,
    ) -> crate::Result<Self>
    where
        Device: Clone,
    {
        let views = attachment_objects.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();
        let cinfo = VkFramebufferCreateInfo {
            renderPass: mold.0,
            attachmentCount: views.len() as _,
            pAttachments: views.as_ptr(),
            width: size.width,
            height: size.height,
            layers,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe { Resolver::get().create_framebuffer(mold.1.native_ptr(), &cinfo, std::ptr::null(), &mut h) }
            .into_result()
            .map(|_| Framebuffer(h, mold.1.clone(), attachment_objects, size.as_ref().clone()))
    }

    /// Create a new framebuffer object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    pub fn new_ext_device(
        device: Device,
        mold: &RenderPass<Device>,
        attachment_objects: Vec<ImageView>,
        size: &VkExtent2D,
        layers: u32,
    ) -> crate::Result<Self> {
        let views = attachment_objects.iter().map(|x| x.native_ptr()).collect::<Vec<_>>();
        let cinfo = VkFramebufferCreateInfo {
            renderPass: mold.0,
            attachmentCount: views.len() as _,
            pAttachments: views.as_ptr(),
            width: size.width,
            height: size.height,
            layers,
            ..Default::default()
        };
        let mut h = VK_NULL_HANDLE as _;
        unsafe { Resolver::get().create_framebuffer(device.native_ptr(), &cinfo, std::ptr::null(), &mut h) }
            .into_result()
            .map(|_| Framebuffer(h, device, attachment_objects, size.as_ref().clone()))
    }
}
impl<Device: VkHandle<Handle = VkDevice>, ImageView: VkHandle<Handle = VkImageView>> Framebuffer<Device, ImageView> {
    pub const fn size(&self) -> &VkExtent2D {
        &self.3
    }

    pub fn resources(&self) -> &[ImageView] {
        &self.2
    }
}

#[cfg(feature = "Implements")]
impl<Device: VkHandle<Handle = VkDevice>> RenderPass<Device> {
    /// Returns the granularity for optimal render area
    pub fn optimal_granularity(&self) -> VkExtent2D {
        let mut e = std::mem::MaybeUninit::uninit();
        unsafe {
            Resolver::get().get_render_area_granularity(self.1.native_ptr(), self.0, e.as_mut_ptr());

            e.assume_init()
        }
    }
}
