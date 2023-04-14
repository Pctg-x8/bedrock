use crate::vk::*;

/// Boxed version of `VkResult`
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VkResultBox(pub VkResult);
impl std::fmt::Debug for VkResultBox {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "[{:?}] {}", self.0, match self.0 {
            // Success Codes //
            VK_SUCCESS => "Command successfully completed", VK_NOT_READY => "A fence or query has not yet completed",
            VK_TIMEOUT => "A wait operation has not completed in the specified time", VK_EVENT_SET => "An event is signaled",
            VK_EVENT_RESET => "An event is unsignaled", VK_INCOMPLETE => "A return array was too small for the result",
            #[cfg(feature = "VK_KHR_swapchain")]
            VK_SUBOPTIMAL_KHR => "Sub-optimal swapchain",
            // Error Codes //
            VK_ERROR_OUT_OF_HOST_MEMORY => "A host memory allocation has failed",
            VK_ERROR_OUT_OF_DEVICE_MEMORY => "A device memory allocation has failed",
            VK_ERROR_INITIALIZATION_FAILED => "Initialization of an object could not be completed for implementation-specific reasons",
            VK_ERROR_DEVICE_LOST => "The logical or physical device has been lost",
            VK_ERROR_MEMORY_MAP_FAILED => "Mapping of a memory object has failed",
            VK_ERROR_LAYER_NOT_PRESENT => "A requested layer is not presented or could not be loaded",
            VK_ERROR_EXTENSION_NOT_PRESENT => "A requested extension is not supported",
            VK_ERROR_FEATURE_NOT_PRESENT => "A requested feature is not supported",
            VK_ERROR_INCOMPATIBLE_DRIVER => "The requested version of Vulkan is not supported by the driver or is otherwise incompatible for implementation-specific reasons",
            VK_ERROR_TOO_MANY_OBJECTS => "Too many objects of the type have already been created",
            VK_ERROR_FORMAT_NOT_SUPPORTED => "A requested format is not supported on this device",
            VK_ERROR_FRAGMENTED_POOL => "A pool allocation has failed due to fragmentation of the pool's memory",
            #[cfg(feature = "VK_KHR_surface")]
            VK_ERROR_SURFACE_LOST_KHR => "Surface lost",
            #[cfg(feature = "VK_KHR_surface")]
            VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => "Native window is in use",
            #[cfg(feature = "VK_KHR_swapchain")]
            VK_ERROR_OUT_OF_DATE_KHR => "Out of date",
            #[cfg(feature = "VK_KHR_display_swapchain")]
            VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => "The display used by a swapchain does not use the same presentable image layout",
            #[cfg(feature = "VK_EXT_debug_report")]
            VK_ERROR_VALIDATION_FAILED_EXT => "Validation failed",
            #[cfg(feature = "VK_NV_glsl_shader")]
            VK_ERROR_INVALID_SHADER_NV => "Invalid GLSL shader",
            #[cfg(feature = "VK_KHR_maintenance1")]
            VK_ERROR_OUT_OF_POOL_MEMORY_KHR => "A pool memory allocation has failed",
            #[cfg(feature = "VK_KHR_external_memory_capabilities")]
            VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR => "An external handle is not a valid handle of ths specified type",
            _ => "Unknown or extension-specific error"
        })
    }
}
impl std::fmt::Display for VkResultBox {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, fmt)
    }
}
impl std::error::Error for VkResultBox {}
impl VkResultBox {
    #[inline]
    pub fn is_err(&self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub fn into_result(self) -> Result<Self, Self> {
        if self.is_err() {
            Err(self)
        } else {
            Ok(self)
        }
    }
}
