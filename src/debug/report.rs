use crate::{vk::*, VulkanStructure};

/// Opaque handle to a debug report callback object
#[repr(transparent)]
pub struct DebugReportCallback(VkDebugReportCallbackEXT);

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
