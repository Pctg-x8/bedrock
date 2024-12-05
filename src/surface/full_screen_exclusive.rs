use crate::{vk::*, VulkanSinkStructure, VulkanStructure};

impl VkSurfaceFullScreenExclusiveInfoEXT {
    /// Constructs the structure, specifying the preferred full-screen transition behavior.
    pub const fn new(flags: VkFullScreenExclusiveEXT) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null_mut(),
            fullScreenExclusive: flags,
        }
    }
}

#[cfg(windows)]
impl VkSurfaceFullScreenExclusiveWin32InfoEXT {
    /// Constructs the structure, with a handle identifying the display to create the surface with.
    pub const fn new(hmonitor: windows::Win32::Graphics::Gdi::HMONITOR) -> Self {
        Self {
            sType: Self::TYPE,
            pNext: core::ptr::null(),
            hmonitor,
        }
    }
}
