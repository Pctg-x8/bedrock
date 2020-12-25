//! Physical Device Extended Queries

use crate::vk::*;
#[cfg(feature = "Implements")] use crate::{ResolverInterface, VkResultHandler};

impl super::PhysicalDevice {
    #[cfg(all(feature = "VK_KHR_get_surface_capabilities2", feature = "Implements"))]
    pub fn surface_capabilities2(
        &self, surface_info: &VkPhysicalDeviceSurfaceInfo2KHR
    ) -> crate::Result<VkSurfaceCapabilities2KHR> {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::Resolver::get().get_physical_device_surface_capabilities2_khr(self.0, surface_info, p.as_mut_ptr())
                .into_result()
                .map(move |_| p.assume_init())
        }
    }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
unsafe impl crate::ext::VulkanStructure for VkPhysicalDeviceSurfaceInfo2KHR {
    const TYPE: VkStructureType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR;
}

impl super::PhysicalDevice {
    #[cfg(all(feature = "VK_KHR_get_physical_device_properties2", feature = "Implements"))]
    /// [VK_KHR_get_physical_device_properties2][Implements] Returns properties of a physical device
    pub fn properties2(&self) -> VkPhysicalDeviceProperties2 {
        let mut p = std::mem::MaybeUninit::uninit();
        unsafe {
            crate::Resolver::get().get_physical_device_properties2(self.0, p.as_mut_ptr());
            p.assume_init()
        }
    }
}
