//! FFI Function Conversions

use crate::vk::*;

pub trait FnTransmute: Sized {
    /// Transmute unknown function pointer into another function pointer
    /// # Safety
    /// If source pointer is not pointed valid function entry point, calling returned pointer has undefined behavior
    unsafe fn from_fn(p: PFN_vkVoidFunction) -> Self;
}
macro_rules! FnTransmuteImpl {
	(for $($t: ty),*) => {
		$(impl FnTransmute for $t { unsafe fn from_fn(p: PFN_vkVoidFunction) -> Self { unsafe { core::mem::transmute(p) } } })*
	}
}

#[cfg(feature = "VK_KHR_surface")]
FnTransmuteImpl!(for PFN_vkDestroySurfaceKHR, PFN_vkGetPhysicalDeviceSurfaceSupportKHR, PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
	PFN_vkGetPhysicalDeviceSurfaceFormatsKHR, PFN_vkGetPhysicalDeviceSurfacePresentModesKHR);

#[cfg(feature = "VK_EXT_debug_report")]
FnTransmuteImpl!(for PFN_vkCreateDebugReportCallbackEXT, PFN_vkDebugReportMessageEXT, PFN_vkDestroyDebugReportCallbackEXT);

#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
FnTransmuteImpl!(for PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR, PFN_vkGetPhysicalDeviceSurfaceFormats2KHR);

#[cfg(feature = "VK_KHR_swapchain")]
FnTransmuteImpl!(for extern "system" fn(VkDevice, VkSwapchainKHR) -> VkResult); // vkAcquireFullScreenExclusiveModeEXT, vkReleaseFullScreenExclusiveModeEXT

#[cfg(feature = "VK_KHR_external_memory_win32")]
FnTransmuteImpl!(for PFN_vkGetMemoryWin32HandlePropertiesKHR, PFN_vkGetMemoryWin32HandleKHR);

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
FnTransmuteImpl!(for PFN_vkImportSemaphoreWin32HandleKHR, PFN_vkGetSemaphoreWin32HandleKHR);

#[cfg(feature = "VK_KHR_external_fence_fd")]
FnTransmuteImpl!(for PFN_vkGetFenceFdKHR, PFN_vkImportFenceFdKHR);

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
FnTransmuteImpl!(for PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR);

#[cfg(feature = "VK_EXT_direct_mode_display")]
FnTransmuteImpl!(for PFN_vkReleaseDisplayEXT);

#[cfg(feature = "VK_EXT_acquire_xlib_display")]
FnTransmuteImpl!(for PFN_vkAcquireXlibDisplayEXT, PFN_vkGetRandROutputDisplayEXT);

#[cfg(feature = "VK_KHR_external_memory_fd")]
FnTransmuteImpl!(for PFN_vkGetMemoryFdKHR, PFN_vkGetMemoryFdPropertiesKHR);

#[cfg(feature = "VK_EXT_external_memory_host")]
FnTransmuteImpl!(for PFN_vkGetMemoryHostPointerPropertiesEXT);

#[cfg(feature = "VK_EXT_debug_utils")]
FnTransmuteImpl!(for PFN_vkCreateDebugUtilsMessengerEXT, PFN_vkDestroyDebugUtilsMessengerEXT, PFN_vkSetDebugUtilsObjectNameEXT);

#[cfg(feature = "VK_EXT_full_screen_exclusive")]
FnTransmuteImpl!(for PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT);

#[cfg(feature = "VK_KHR_descriptor_update_template")]
FnTransmuteImpl!(for PFN_vkCreateDescriptorUpdateTemplateKHR, PFN_vkDestroyDescriptorUpdateTemplateKHR, PFN_vkUpdateDescriptorSetWithTemplateKHR);

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
FnTransmuteImpl!(for PFN_vkGetPhysicalDeviceFeatures2KHR, PFN_vkGetPhysicalDeviceProperties2KHR, PFN_vkGetPhysicalDeviceFormatProperties2KHR);

#[cfg(feature = "VK_KHR_maintenance1")]
FnTransmuteImpl!(for PFN_vkTrimCommandPoolKHR);

#[cfg(feature = "VK_KHR_bind_memory2")]
FnTransmuteImpl!(for PFN_vkBindBufferMemory2KHR, PFN_vkBindImageMemory2KHR);

#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
FnTransmuteImpl!(for PFN_vkGetImageDrmFormatModifierPropertiesEXT);
