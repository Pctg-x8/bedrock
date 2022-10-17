//! VK_EXT_debug_utils extensions

pub const VK_EXT_DEBUG_UTILS_SPEC_VERSION: usize = 2;
pub static VK_EXT_DEBUG_UTILS_EXTENSION_NAME: &'static str = "VK_EXT_debug_utils";

use super::*;

mod nd_handle_base_ts {
    pub enum VkDebugUtilsMessengerEXT {}
}
pub type VkDebugUtilsMessengerEXT = VK_NON_DISPATCHABLE_HANDLE!(VkDebugUtilsMessengerEXT);
pub const VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT: VkObjectType = 1000128000;

pub type VkDebugUtilsMessengerCallbackDataFlagsEXT = VkFlags;

pub type VkDebugUtilsMessageSeverityFlagBitsEXT = VkFlags;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00000001;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00000010;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00000100;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00001000;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_FLAG_BITS_MAX_ENUM_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 0x7fffffff;

pub type VkDebugUtilsMessageTypeFlagBitsEXT = VkFlags;
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 0x00000001;
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 0x00000002;
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 0x00000004;
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_FLAG_BITS_MAX_ENUM_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 0x7fffffff;

pub type VkDebugUtilsMessageTypeFlagsEXT = VkFlags;
pub type VkDebugUtilsMessageSeverityFlagsEXT = VkFlags;
pub type VkDebugUtilsMessenegerCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT"]
pub struct VkDebugUtilsLabelEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pLabelName: *const c_char,
    pub color: [c_float; 4],
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT"]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub pObjectName: *const c_char,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, VulkanStructure)]
#[structure_type = "VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT"]
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

#[repr(C)]
#[derive(Clone)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessenegerCreateFlagsEXT,
    pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
    pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut c_void,
}
unsafe impl crate::ext::VulkanStructure for VkDebugUtilsMessengerCreateInfoEXT {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VkDebugUtilsObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub tagName: u64,
    pub tagSize: size_t,
    pub pTag: *const c_void,
}
unsafe impl crate::ext::VulkanStructure for VkDebugUtilsObjectTagInfoEXT {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT;
}

pub type PFN_vkSetDebugUtilsObjectNameEXT =
    extern "system" fn(device: VkDevice, pNameInfo: *const VkDebugUtilsObjectNameInfoEXT) -> VkResult;
pub type PFN_vkSetDebugUtilsObjectTagEXT =
    extern "system" fn(device: VkDevice, pTagInfo: *const VkDebugUtilsObjectTagInfoEXT) -> VkResult;
pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
    extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
pub type PFN_vkQueueEndDebugUtilsLabelEXT = extern "system" fn(queue: VkQueue);
pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
    extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
pub type PFN_vkCmdBeginDebugUtilsLabelEXT =
    extern "system" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
pub type PFN_vkCmdEndDebugUtilsLabelEXT = extern "system" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdInsertDebugUtilsLabelEXT =
    extern "system" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
pub type PFN_vkCreateDebugUtilsMessengerEXT = extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMessenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult;
pub type PFN_vkDestroyDebugUtilsMessengerEXT = extern "system" fn(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkSubmitDebugUtilsMessageEXT = extern "system" fn(
    instance: VkInstance,
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    pub fn PFN_vkSetDebugUtilsObjectNameEXT(
        device: VkDevice,
        pNameInfo: *const VkDebugUtilsObjectNameInfoEXT,
    ) -> VkResult;
    pub fn PFN_vkSetDebugUtilsObjectTagEXT(device: VkDevice, pTagInfo: *const VkDebugUtilsObjectTagInfoEXT)
        -> VkResult;
    pub fn PFN_vkQueueBeginDebugUtilsLabelEXT(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
    pub fn PFN_vkQueueEndDebugUtilsLabelEXT(queue: VkQueue);
    pub fn PFN_vkQueueInsertDebugUtilsLabelEXT(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
    pub fn PFN_vkCmdBeginDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
    pub fn PFN_vkCmdEndDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer);
    pub fn PFN_vkCmdInsertDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
    pub fn PFN_vkCreateDebugUtilsMessengerEXT(
        instance: VkInstance,
        pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
        pMessenger: *mut VkDebugUtilsMessengerEXT,
    ) -> VkResult;
    pub fn PFN_vkDestroyDebugUtilsMessengerEXT(
        instance: VkInstance,
        messenger: VkDebugUtilsMessengerEXT,
        pAllocator: *const VkAllocationCallbacks,
    );
    pub fn PFN_vkSubmitDebugUtilsMessageEXT(
        instance: VkInstance,
        messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
        messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
        pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
    );
}
