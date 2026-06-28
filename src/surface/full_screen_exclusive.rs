use bedrock_vk::{self as brvk};

/// Hint values an application can specify affecting full-screen transition behavior.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FullScreenExclusive {
    /// The implementation should determine the appropriate full-screen method by whatever means it deems appropriate.
    Default = brvk::VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT,
    /// The implementation may use full-screen exclusive mechanisms when available.
    Allowed = brvk::VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT,
    /// The implementation should avoid using full-screen mechanisms which rely on disruptive transitions.
    Disallowed = brvk::VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT,
    /// The application will manage full-screen exclusive mode by using the
    /// [`vkAcquireFullScreenExclusiveModeEXT`] and [`vkReleaseFullScreenExclusiveModeEXT`] commands.
    ApplicationControlled = brvk::VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT,
}

#[repr(transparent)]
pub struct SurfaceFullScreenExclusiveInfo<'n>(
    brvk::VkSurfaceFullScreenExclusiveInfoEXT,
    core::marker::PhantomData<Option<&'n dyn brvk::VulkanStructure>>,
);
unsafe impl brvk::VulkanStructure for SurfaceFullScreenExclusiveInfo<'_> {
    fn as_generic(&self) -> &brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }

    fn as_generic_mut(&mut self) -> &mut brvk::GenericVulkanStructure {
        unsafe { core::mem::transmute(self) }
    }
}
impl brvk::TypedVulkanStructure for SurfaceFullScreenExclusiveInfo<'_> {
    const TYPE: crate::vk::VkStructureType =
        <brvk::VkSurfaceFullScreenExclusiveInfoEXT as brvk::TypedVulkanStructure>::TYPE;
}
impl<'n> SurfaceFullScreenExclusiveInfo<'n> {
    /// Constructs the structure, specifying the preferred full-screen transition behavior.
    pub const fn new(flags: brvk::VkFullScreenExclusiveEXT) -> Self {
        Self(
            brvk::VkSurfaceFullScreenExclusiveInfoEXT {
                sType: <brvk::VkSurfaceFullScreenExclusiveInfoEXT as brvk::TypedVulkanSinkStructure>::TYPE,
                pNext: core::ptr::null_mut(),
                fullScreenExclusive: flags,
            },
            core::marker::PhantomData,
        )
    }

    /// # Safety
    ///
    /// `raw` must be a valid `VkSurfaceFullScreenExclusiveInfoEXT` structure.
    pub const unsafe fn from_raw(raw: brvk::VkSurfaceFullScreenExclusiveInfoEXT) -> Self {
        Self(raw, core::marker::PhantomData)
    }

    pub const fn into_raw(self) -> brvk::VkSurfaceFullScreenExclusiveInfoEXT {
        self.0
    }

    pub const fn with_next(mut self, next: &'n (impl brvk::VulkanStructure + ?Sized)) -> Self {
        self.0.pNext = next as *const _ as _;
        self
    }
}

#[cfg(windows)]
#[repr(transparent)]
pub struct SurfaceFullScreenExclusiveWin32Info(brvk::VkSurfaceFullScreenExclusiveWin32InfoEXT);
#[cfg(windows)]
impl SurfaceFullScreenExclusiveWin32Info {
    /// Constructs the structure, with a handle identifying the display to create the surface with.
    pub const fn new(hmonitor: windows::Win32::Graphics::Gdi::HMONITOR) -> Self {
        use bedrock_vk::TypedVulkanStructure;

        Self(brvk::VkSurfaceFullScreenExclusiveWin32InfoEXT {
            sType: brvk::VkSurfaceFullScreenExclusiveWin32InfoEXT::TYPE,
            pNext: core::ptr::null(),
            hmonitor,
        })
    }
}
