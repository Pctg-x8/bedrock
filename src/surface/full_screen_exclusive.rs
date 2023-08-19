use crate::{vk::*, Extendable, VulkanStructure, VulkanStructureProvider};

/// Wraps VkSurfaceFullScreenExclusiveInfoEXT structure: Specifying the preferred full-screen transition behavior
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct FullScreenExclusiveInfoEXT(VkSurfaceFullScreenExclusiveInfoEXT);
impl FullScreenExclusiveInfoEXT {
    /// Constructs the structure, specifying the preferred full-screen transition behavior.
    pub const fn new(flags: FullScreenExclusiveEXT) -> Self {
        Self(VkSurfaceFullScreenExclusiveInfoEXT {
            sType: VkSurfaceFullScreenExclusiveInfoEXT::TYPE,
            pNext: core::ptr::null_mut(),
            fullScreenExclusive: flags as _,
        })
    }
}
impl From<FullScreenExclusiveInfoEXT> for VkSurfaceFullScreenExclusiveInfoEXT {
    fn from(v: FullScreenExclusiveInfoEXT) -> Self {
        v.0
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<T> Extendable<FullScreenExclusiveInfoEXT> for T where
    T: VulkanStructureProvider<RootStructure = VkSwapchainCreateInfoKHR>
{
}

/// Hint values an application can specify affecting full-screen transition behavior
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FullScreenExclusiveEXT {
    /// The implementation should determine the appropriate full-screen method by whatever means it deems appropriate.
    Default = VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT as _,
    /// The implementation may use full-screen exclusive mechanisms when available.
    /// Such mechanisms may result in better performance and/or the availability of different presentation capabilities,
    /// but may require a more disruptive transition during swapchain initialization,
    /// first presentation and/or destruction.
    Allowed = VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT as _,
    /// The implementation should avoid using full-screen mechanisms which rely on disruptive transitions.
    Disallowed = VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT as _,
    /// The application will manage full-screen exclusive mode by using the `vkAcquireFullScreenExclusiveModeEXT` and
    /// `vkReleaseFullScreenExclusiveEXT` commands.
    ApplicationControlled = VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT as _,
}

/// Wraps VkSurfaceFullScreenExclusiveWin32InfoEXT structure: Specifying additional creation parameters specific to Win32 fullscreen exclusive mode.
#[cfg(windows)]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct FullScreenExclusiveWin32InfoEXT(VkSurfaceFullScreenExclusiveWin32InfoEXT);
#[cfg(windows)]
impl FullScreenExclusiveWin32InfoEXT {
    /// Constructs the structure, with a handle identifying the display to create the surface with.
    pub const fn new(handle: windows::Win32::Graphics::Gdi::HMONITOR) -> Self {
        Self(VkSurfaceFullScreenExclusiveWin32InfoEXT {
            sType: VkSurfaceFullScreenExclusiveWin32InfoEXT::TYPE,
            pNext: core::ptr::null(),
            hmonitor: handle,
        })
    }
}
#[cfg(windows)]
impl From<FullScreenExclusiveWin32InfoEXT> for VkSurfaceFullScreenExclusiveWin32InfoEXT {
    fn from(v: FullScreenExclusiveWin32InfoEXT) -> Self {
        v.0
    }
}
#[cfg(windows)]
#[cfg(feature = "VK_KHR_swapchain")]
impl<T> Extendable<FullScreenExclusiveWin32InfoEXT> for T where
    T: VulkanStructureProvider<RootStructure = VkSwapchainCreateInfoKHR>
{
}
