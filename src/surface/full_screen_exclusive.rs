use crate::*;

/// Hint values an application can specify affecting full-screen transition behavior.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FullScreenExclusive {
    /// The implementation should determine the appropriate full-screen method by whatever means it deems appropriate.
    Default = VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT,
    /// The implementation may use full-screen exclusive mechanisms when available.
    Allowed = VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT,
    /// The implementation should avoid using full-screen mechanisms which rely on disruptive transitions.
    Disallowed = VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT,
    /// The application will manage full-screen exclusive mode by using the
    /// [`vkAcquireFullScreenExclusiveModeEXT`] and [`vkReleaseFullScreenExclusiveModeEXT`] commands.
    ApplicationControlled = VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT,
}

#[repr(transparent)]
pub struct SurfaceFullScreenExclusiveInfo<'n>(
    VkSurfaceFullScreenExclusiveInfoEXT,
    core::marker::PhantomData<Option<&'n dyn VulkanStructure>>,
);
unsafe impl VulkanStructure for SurfaceFullScreenExclusiveInfo<'_> {
    fn as_generic(&self) -> &GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
unsafe impl TypedVulkanStructure for SurfaceFullScreenExclusiveInfo<'_> {
    const TYPE: crate::vk::VkStructureType = <VkSurfaceFullScreenExclusiveInfoEXT as TypedVulkanStructure>::TYPE;
}
impl<'n> SurfaceFullScreenExclusiveInfo<'n> {
    /// Constructs the structure, specifying the preferred full-screen transition behavior.
    pub const fn new(flags: VkFullScreenExclusiveEXT) -> Self {
        Self(
            VkSurfaceFullScreenExclusiveInfoEXT {
                sType: <VkSurfaceFullScreenExclusiveInfoEXT as TypedVulkanSinkStructure>::TYPE,
                pNext: core::ptr::null_mut(),
                fullScreenExclusive: flags,
            },
            core::marker::PhantomData,
        )
    }

    pub const unsafe fn from_raw(raw: VkSurfaceFullScreenExclusiveInfoEXT) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> VkSurfaceFullScreenExclusiveInfoEXT {
        self.0
    }

    pub const fn with_next(mut self, next: &'n (impl VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
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
