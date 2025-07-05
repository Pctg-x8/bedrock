pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: usize = 1;
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &str = "VK_KHR_external_memory_capabilities";

use crate::vk2::*;
use derives::{promote_1_1, vk_ext_command};

use super::*;

vk_ext_command! {
    pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHR(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR);
    suffix = "KHR";
    promote = "1.1";
}
