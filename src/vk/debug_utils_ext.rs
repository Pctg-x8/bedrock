//! VK_EXT_debug_utils extensions

pub const VK_EXT_DEBUG_UTILS_SPEC_VERSION: usize = 2;
pub static VK_EXT_DEBUG_UTILS_EXTENSION_NAME: &'static str = "VK_EXT_debug_utils";

use super::*;
use crate::PFN;

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
#[derive(Clone, Debug, PartialEq, Eq, VulkanStructure)]
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
#[derive(Clone, Debug, PartialEq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT)]
pub struct VkDebugUtilsLabelEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pLabelName: *const c_char,
    pub color: [c_float; 4],
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub pObjectName: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT)]
pub struct VkDebugUtilsObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub tagName: u64,
    pub tagSize: size_t,
    pub pTag: *const c_void,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, VulkanStructure)]
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

pub type PFN_vkDebugUtilsMessengerCallbackEXT = extern "system" fn(
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
    pUserData: *mut c_void,
) -> VkBool32;

#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkSetDebugUtilsObjectNameEXT)]
pub struct PFN_vkSetDebugUtilsObjectNameEXT(
    pub unsafe extern "system" fn(device: VkDevice, pNameInfo: *const VkDebugUtilsObjectNameInfoEXT) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkSetDebugUtilsObjectTagEXT)]
pub struct PFN_vkSetDebugUtilsObjectTagEXT(
    pub unsafe extern "system" fn(device: VkDevice, pTagInfo: *const VkDebugUtilsObjectTagInfoEXT) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkQueueBeginDebugUtilsLabelEXT)]
pub struct PFN_vkQueueBeginDebugUtilsLabelEXT(
    pub unsafe extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkQueueEndDebugUtilsLabelEXT)]
pub struct PFN_vkQueueEndDebugUtilsLabelEXT(pub unsafe extern "system" fn(queue: VkQueue));
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkQueueInsertDebugUtilsLabelEXT)]
pub struct PFN_vkQueueInsertDebugUtilsLabelEXT(
    pub unsafe extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdBeginDebugUtilsLabelEXT)]
pub struct PFN_vkCmdBeginDebugUtilsLabelEXT(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdEndDebugUtilsLabelEXT)]
pub struct PFN_vkCmdEndDebugUtilsLabelEXT(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer));
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdInsertDebugUtilsLabelEXT)]
pub struct PFN_vkCmdInsertDebugUtilsLabelEXT(
    pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCreateDebugUtilsMessengerEXT)]
pub struct PFN_vkCreateDebugUtilsMessengerEXT(
    pub  unsafe extern "system" fn(
        instance: VkInstance,
        pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
        pMessenger: *mut VkDebugUtilsMessengerEXT,
    ) -> VkResult,
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkDestroyDebugUtilsMessengerEXT)]
pub struct PFN_vkDestroyDebugUtilsMessengerEXT(
    pub  unsafe extern "system" fn(
        instance: VkInstance,
        messenger: VkDebugUtilsMessengerEXT,
        pAllocator: *const VkAllocationCallbacks,
    ),
);
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkSubmitDebugUtilsMessageEXT)]
pub struct PFN_vkSubmitDebugUtilsMessageEXT(
    pub  unsafe extern "system" fn(
        instance: VkInstance,
        messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
        messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
        pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
    ),
);
