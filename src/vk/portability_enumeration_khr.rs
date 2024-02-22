//! VK_KHR_portability_enumeration extensions

pub const VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION: usize = 1;
pub const VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME: &'static str = "VK_KHR_portability_enumeration";

use super::*;

vk_bitmask! {
    extending enum VkInstanceCreateFlagBits {
        pub VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR: 0
    }
}
