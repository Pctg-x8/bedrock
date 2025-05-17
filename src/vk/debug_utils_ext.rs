//! VK_EXT_debug_utils extensions

pub const VK_EXT_DEBUG_UTILS_SPEC_VERSION: usize = 2;
pub static VK_EXT_DEBUG_UTILS_EXTENSION_NAME: &'static str = "VK_EXT_debug_utils";

use derives::vk_ext_command;

use super::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[vk_raw_handle(object_type = VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT)]
pub struct VkDebugUtilsMessengerEXT(pub u64);

pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT: VkStructureType = ext_enum_value(129, 0) as _;
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT: VkStructureType = ext_enum_value(129, 1) as _;
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT: VkStructureType = ext_enum_value(129, 2) as _;
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: VkStructureType = ext_enum_value(129, 3) as _;
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: VkStructureType = ext_enum_value(129, 4) as _;

pub const VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT: VkObjectType = ext_enum_value(129, 0) as _;

pub type VkDebugUtilsMessageSeverityFlagsEXT = VkFlags;
vk_bitmask! {
    pub enum VkDebugUtilsMessageSeverityFlagBitsEXT {
        pub VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT: 0,
        pub VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT: 4,
        pub VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT: 8,
        pub VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT: 12
    }
}

pub type VkDebugUtilsMessageTypeFlagsEXT = VkFlags;
vk_bitmask! {
    pub enum VkDebugUtilsMessengerTypeFlagBitsEXT {
        pub VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT: 0,
        pub VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT: 1,
        pub VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT: 2
    }
}

pub type VkDebugUtilsMessengerCallbackDataFlagsEXT = VkFlags;
vk_bitmask! {
    pub enum VkDebugUtilsMessengerCallbackDataFlagBitsEXT {}
}

pub type VkDebugUtilsMessengerCreateFlagsEXT = VkFlags;
vk_bitmask! {
    pub enum VkDebugUtilsMessengerCreateFlagBitsEXT {}
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
    pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
    pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT)]
pub struct VkDebugUtilsLabelEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pLabelName: *const c_char,
    pub color: [c_float; 4],
}
impl VkDebugUtilsLabelEXT {
    #[inline]
    pub const unsafe fn label_name_cstr(&self) -> &core::ffi::CStr {
        unsafe { core::ffi::CStr::from_ptr(self.pLabelName) }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub pObjectName: *const c_char,
}
impl VkDebugUtilsObjectNameInfoEXT {
    #[inline]
    pub unsafe fn object_name_cstr(&self) -> Option<&core::ffi::CStr> {
        if self.pObjectName.is_null() {
            None
        } else {
            Some(unsafe { core::ffi::CStr::from_ptr(self.pObjectName) })
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT)]
pub struct VkDebugUtilsObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub tagName: u64,
    pub tagSize: usize,
    pub pTag: *const c_void,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, TypedVulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
    pub pMessageIdName: *const c_char,
    pub messageIdNumber: i32,
    pub pMessage: *const c_char,
    pub queueLabelCount: u32,
    pub pQueueLabels: *const VkDebugUtilsLabelEXT,
    pub cmdBufLabelCount: u32,
    pub pCmdBufLabels: *const VkDebugUtilsLabelEXT,
    pub objectCount: u32,
    pub pObjects: *const VkDebugUtilsObjectNameInfoEXT,
}
impl VkDebugUtilsMessengerCallbackDataEXT {
    #[inline]
    pub const unsafe fn queue_labels(&self) -> &[VkDebugUtilsLabelEXT] {
        if self.queueLabelCount == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.pQueueLabels, self.queueLabelCount as _) }
        }
    }

    #[inline]
    pub const unsafe fn cmd_buf_labels(&self) -> &[VkDebugUtilsLabelEXT] {
        if self.cmdBufLabelCount == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.pCmdBufLabels, self.cmdBufLabelCount as _) }
        }
    }

    #[inline]
    pub const unsafe fn objects(&self) -> &[VkDebugUtilsObjectNameInfoEXT] {
        if self.objectCount == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.pObjects, self.objectCount as _) }
        }
    }
}

#[allow(non_camel_case_types)]
pub type PFN_vkDebugUtilsMessengerCallbackEXT = extern "system" fn(
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
    pUserData: *mut c_void,
) -> VkBool32;

vk_ext_command! {
    pub fn vkSetDebugUtilsObjectNameEXT(device: VkDevice, pNameInfo: *const VkDebugUtilsObjectNameInfoEXT) -> VkResult;
}

vk_ext_command! {
    pub fn vkSetDebugUtilsObjectTagEXT(device: VkDevice, pTagInfo: *const VkDebugUtilsObjectTagInfoEXT) -> VkResult;
}

vk_ext_command! {
    pub fn vkQueueBeginDebugUtilsLabelEXT(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
}

vk_ext_command! {
    pub fn vkQueueEndDebugUtilsLabelEXT(queue: VkQueue);
}

vk_ext_command! {
    pub fn vkQueueInsertDebugUtilsLabelEXT(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
}

vk_ext_command! {
    pub fn vkCmdBeginDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
}

vk_ext_command! {
    pub fn vkCmdEndDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer);
}

vk_ext_command! {
    pub fn vkCmdInsertDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
}

vk_ext_command! {
    pub fn vkCreateDebugUtilsMessengerEXT(instance: VkInstance, pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pMessenger: *mut VkDebugUtilsMessengerEXT) -> VkResult;
}

vk_ext_command! {
    pub fn vkDestroyDebugUtilsMessengerEXT(instance: VkInstance, messenger: VkDebugUtilsMessengerEXT, pAllocator: *const VkAllocationCallbacks);
}

vk_ext_command! {
    pub fn vkSubmitDebugUtilsMessageEXT(instance: VkInstance, messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT, messageTypes: VkDebugUtilsMessageTypeFlagsEXT, pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT);
}
