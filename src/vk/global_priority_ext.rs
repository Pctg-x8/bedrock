//! VK_EXT_global_priority extension

pub const VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION: usize = 2;
pub const VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME: &str = "VK_EXT_global_priority";

use super::*;
use std::mem::zeroed;

pub type VkQueueGlobalPriorityEXT = i32;
pub const VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT: VkQueueGlobalPriorityEXT = 128;
pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT: VkQueueGlobalPriorityEXT = 256;
pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT: VkQueueGlobalPriorityEXT = 512;
pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT: VkQueueGlobalPriorityEXT = 1024;

#[repr(C)] #[derive(Clone, Debug)]
pub struct VkDeviceQueueGlobalPriorityCreateInfoEXT
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub globalPriority: VkQueueGlobalPriorityEXT
}
impl Default for VkDeviceQueueGlobalPriorityCreateInfoEXT
{
    fn default() -> Self
    {
        VkDeviceQueueGlobalPriorityCreateInfoEXT
        {
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT,
            .. unsafe { zeroed() }
        }
    }
}
