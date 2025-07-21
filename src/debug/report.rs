use crate::*;
use derives::implements;

pub trait DebugReportCallback: VkHandle<Handle = VkDebugReportCallbackEXT> + InstanceChild {}
DerefContainerBracketImpl!(for DebugReportCallback {});
GuardsImpl!(for DebugReportCallback {});

/// Opaque object to a debug report callback object
#[derive(VkHandle, VkObject, InstanceChild)]
#[VkObject(type = VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT)]
pub struct DebugReportCallbackObject<Instance: crate::Instance + InstanceDebugReportExtension>(
    pub(crate) VkDebugReportCallbackEXT,
    #[parent] pub(crate) Instance,
);
#[implements]
impl<Instance: crate::Instance + InstanceDebugReportExtension> Drop for DebugReportCallbackObject<Instance> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.1.destroy_debug_report_callback_raw(self.native_ptr(), None);
        }
    }
}
unsafe impl<Instance: crate::Instance + InstanceDebugReportExtension + Sync> Sync
    for DebugReportCallbackObject<Instance>
{
}
unsafe impl<Instance: crate::Instance + InstanceDebugReportExtension + Send> Send
    for DebugReportCallbackObject<Instance>
{
}
impl<Instance: crate::Instance + InstanceDebugReportExtension> DebugReportCallback
    for DebugReportCallbackObject<Instance>
{
}
impl<Instance: crate::Instance + InstanceDebugReportExtension> DebugReportCallbackObject<Instance> {
    /// Register a debug report callback
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    #[implements]
    #[inline]
    pub fn new(instance: Instance, info: &DebugReportCallbackCreateInfo) -> crate::Result<Self> {
        let h = unsafe { instance.new_debug_report_callback_raw(info, None)? };

        Ok(Self(h, instance))
    }
}
impl<Instance: crate::Instance + InstanceDebugReportExtension + Clone> DebugReportCallbackObject<&'_ Instance> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> DebugReportCallbackObject<Instance> {
        let r = DebugReportCallbackObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}

#[repr(transparent)]
pub struct DebugReportCallbackCreateInfo(VkDebugReportCallbackCreateInfoEXT);
impl DebugReportCallbackCreateInfo {
    pub const fn new(flags: VkDebugReportFlagsEXT, callback: PFN_vkDebugReportCallbackEXT) -> Self {
        Self(VkDebugReportCallbackCreateInfoEXT {
            sType: VkDebugReportCallbackCreateInfoEXT::TYPE,
            pNext: core::ptr::null(),
            flags,
            pfnCallback: callback,
            pUserData: core::ptr::null_mut(),
        })
    }

    pub const unsafe fn from_raw(raw: VkDebugReportCallbackCreateInfoEXT) -> Self {
        Self(raw)
    }

    pub const fn into_raw(self) -> VkDebugReportCallbackCreateInfoEXT {
        self.0
    }

    pub(crate) const fn as_raw_ref(&self) -> &VkDebugReportCallbackCreateInfoEXT {
        &self.0
    }

    pub const fn with_user_data<T>(mut self, data: *mut T) -> Self {
        self.0.pUserData = data as _;
        self
    }
}
