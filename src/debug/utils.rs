use derives::implements;

use crate::ffi_helper::opt_cstr_ptr;
use crate::*;

pub trait DebugUtilsMessenger: VkHandle<Handle = VkDebugUtilsMessengerEXT> + InstanceChild {}
DerefContainerBracketImpl!(for DebugUtilsMessenger {});
GuardsImpl!(for DebugUtilsMessenger {});

#[derive(VkHandle, VkObject, InstanceChild)]
#[VkObject(type = VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT)]
pub struct DebugUtilsMessengerObject<Instance: crate::Instance + InstanceExtensions>(
    VkDebugUtilsMessengerEXT,
    #[parent] Instance,
);
#[implements]
impl<Instance: crate::Instance + InstanceExtensions> Drop for DebugUtilsMessengerObject<Instance> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.1.destroy_debug_utils_messenger_raw(self.native_ptr(), None);
        }
    }
}
unsafe impl<Instance: crate::Instance + InstanceExtensions + Sync> Sync for DebugUtilsMessengerObject<Instance> {}
unsafe impl<Instance: crate::Instance + InstanceExtensions + Send> Send for DebugUtilsMessengerObject<Instance> {}
impl<Instance: crate::Instance + InstanceExtensions> DebugUtilsMessenger for DebugUtilsMessengerObject<Instance> {}
impl<Instance: crate::Instance + InstanceExtensions> DebugUtilsMessengerObject<Instance> {
    /// Create a debug messenger object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * [`VK_ERROR_OUT_OF_HOST_MEMORY`]
    #[implements]
    pub fn new(instance: Instance, info: &DebugUtilsMessengerCreateInfo) -> crate::Result<Self> {
        let h = unsafe { instance.new_debug_utils_messenger_raw(info, None)? };

        Ok(Self(h, instance))
    }

    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: VkDebugUtilsMessengerEXT, parent: Instance) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (VkDebugUtilsMessengerEXT, Instance) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Instance: crate::Instance + InstanceExtensions + Clone> DebugUtilsMessengerObject<&'_ Instance> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> DebugUtilsMessengerObject<Instance> {
        let r = DebugUtilsMessengerObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugUtilsMessageSeverityFlag {
    /// The most verbose output indicating all diagnostic messages
    /// from the Vulkan loader, layers, and drivers should be captured.
    Verbose = VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT,
    /// An informational message such as resource details that may be handy when debugging an application.
    Info = VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT,
    /// Use of Vulkan that *may* expose an app bug.
    /// Such cases may not be immediately harmful, such as a fragment shader outputting to a location with no attachment.
    /// Other cases *may* point to behavior that is almost certainly bad when unintended
    /// such as using an image whose memory has not been filled.
    /// In general if you see a warning but you know that the behavior is intended/desired,
    /// then simply ignore the warning.
    Warning = VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT,
    /// The application has violated a valid usage condition of the specification.
    Error = VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
}

/// Bitmask specifying which severities of events cause a debug messenger callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct DebugUtilsMessageSeverityFlags(VkDebugUtilsMessageSeverityFlagsEXT);
impl DebugUtilsMessageSeverityFlags {
    /// The most verbose output indicating all diagnostic messages
    /// from the Vulkan loader, layers, and drivers should be captured.
    pub const VERSBOSE: Self = Self(VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT);
    /// An informational message such as resource details that may be handy when debugging an application.
    pub const INFO: Self = Self(VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT);
    /// Use of Vulkan that *may* expose an app bug.
    /// Such cases may not be immediately harmful, such as a fragment shader outputting to a location with no attachment.
    /// Other cases *may* point to behavior that is almost certainly bad when unintended
    /// such as using an image whose memory has not been filled.
    /// In general if you see a warning but you know that the behavior is intended/desired,
    /// then simply ignore the warning.
    pub const WARNING: Self = Self(VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT);
    /// The application has violated a valid usage condition of the specification.
    pub const ERROR: Self = Self(VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT);
}

/// Bitmask specifying which types of events cause a debug messenger callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitflags_newtype]
pub struct DebugUtilsMessageTypeFlags(VkDebugUtilsMessageTypeFlagsEXT);
impl DebugUtilsMessageTypeFlags {
    /// Some general event has occured. This is a typically a non-specification, non-performance event.
    pub const GENERAL: Self = Self(VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT);
    /// Something has occured during validation against the Vulkan specification that may indicate invalid behavior.
    pub const VALIDATION: Self = Self(VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT);
    /// A potentially non-optimal use of Vulkan would have worked.
    pub const PERFORMANCE: Self = Self(VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT);
}

pub type DebugUtilsMessengerCreateInfo = VkDebugUtilsMessengerCreateInfoEXT;
impl DebugUtilsMessengerCreateInfo {
    pub const fn new(
        message_severity: DebugUtilsMessageSeverityFlags,
        message_type: DebugUtilsMessageTypeFlags,
        callback: PFN_vkDebugUtilsMessengerCallbackEXT,
    ) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            messageSeverity: message_severity.bits(),
            messageType: message_type.bits(),
            pfnUserCallback: callback,
            pUserData: core::ptr::null_mut(),
        }
    }

    pub const fn with_user_data<T>(mut self, p: *mut T) -> Self {
        self.pUserData = p as _;
        self
    }
}

#[repr(transparent)]
pub struct DebugUtilsObjectNameInfo<'d>(
    pub(crate) VkDebugUtilsObjectNameInfoEXT,
    core::marker::PhantomData<Option<&'d core::ffi::CStr>>,
);
impl<'d> DebugUtilsObjectNameInfo<'d> {
    #[inline(always)]
    pub fn new<H>(handle: &'d H, name: Option<&'d core::ffi::CStr>) -> Self
    where
        H: VkHandle<Handle: VkRawHandle> + VkObject + ?Sized + 'd,
    {
        unsafe { Self::new_raw(H::TYPE, handle.native_ptr().raw_handle_value(), name) }
    }

    pub const unsafe fn new_raw(ty: VkObjectType, handle: u64, name: Option<&'d core::ffi::CStr>) -> Self {
        DebugUtilsObjectNameInfo(
            VkDebugUtilsObjectNameInfoEXT {
                sType: VkDebugUtilsObjectNameInfoEXT::TYPE,
                pNext: core::ptr::null(),
                objectType: ty,
                objectHandle: handle,
                pObjectName: opt_cstr_ptr(name),
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkDebugUtilsObjectNameInfoEXT) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkDebugUtilsObjectNameInfoEXT {
        self.0
    }
}
