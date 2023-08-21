use derives::implements;

use crate::{vk::*, CompareOp, DeviceChild, VkHandle, VkObject, VulkanStructure};

pub trait Sampler: VkHandle<Handle = VkSampler> + DeviceChild {}
DerefContainerBracketImpl!(for Sampler {});
GuardsImpl!(for Sampler {});

DefineStdDeviceChildObject! {
    /// Opaque handle to a sampler object
    SamplerObject(VkSampler): Sampler { drop destroy_sampler }
}

/// Builder object for constructing the sampler object
#[repr(transparent)]
pub struct SamplerBuilder(VkSamplerCreateInfo);
impl Default for SamplerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl Into<VkSamplerCreateInfo> for SamplerBuilder {
    fn into(self) -> VkSamplerCreateInfo {
        self.0
    }
}
impl SamplerBuilder {
    /// Initialize by default sampler parameters: Linear Filtering, Repeat addressing, no anisotrophy and no lod biases
    pub const fn new() -> Self {
        Self(VkSamplerCreateInfo {
            sType: VkSamplerCreateInfo::TYPE,
            pNext: core::ptr::null(),
            flags: 0,
            magFilter: FilterMode::Linear as _,
            minFilter: FilterMode::Linear as _,
            mipmapMode: MipmapFilterMode::Linear as _,
            addressModeU: AddressingMode::Repeat as _,
            addressModeV: AddressingMode::Repeat as _,
            addressModeW: AddressingMode::Repeat as _,
            mipLodBias: 0.0,
            anisotropyEnable: false as _,
            compareEnable: false as _,
            compareOp: CompareOp::Always as _,
            minLod: 0.0,
            maxLod: 0.0,
            borderColor: BorderColor::TransparentBlackF as _,
            unnormalizedCoordinates: false as _,
            maxAnisotropy: 1.0,
        })
    }

    /// The magnification and the minification filters to apply to lookups.  
    /// Default: Magnification=`FilterMode::Linear`, Minification=`FilterMode::Linear`
    pub const fn filter(mut self, mag: FilterMode, min: FilterMode) -> Self {
        self.0.magFilter = mag as _;
        self.0.minFilter = min as _;

        self
    }

    /// The mipmap filter to apply to lookups.  
    /// Default: `MipmapFilterMode::Linear`
    pub const fn mip_filter(mut self, f: MipmapFilterMode) -> Self {
        self.0.mipmapMode = f as _;

        self
    }

    /// The addressing mode for outside [0..1] range for U, V and W coordinates.  
    /// Default: U=`AddressingMode::Repeat`, V=`AddressinMode::Repeat`, W=`AddressingMode::Repeat`
    pub const fn addressing(mut self, u: AddressingMode, v: AddressingMode, w: AddressingMode) -> Self {
        self.0.addressModeU = u as _;
        self.0.addressModeV = v as _;
        self.0.addressModeW = w as _;

        self
    }

    /// The bias to be added to mipmap LOD calculation and bias provided by image sampling functions in SPIR-V,
    /// as described in the `Level-of-Detail Operation` section in Vulkan Specification.  
    /// Default: 0.0
    pub const fn lod_bias(mut self, bias: f32) -> Self {
        self.0.mipLodBias = bias;

        self
    }

    /// The anisotropy value clamp. Specifying `None` switches off the anisotropic filtering  
    /// Default: `None`
    pub const fn max_anisotropy(mut self, level: Option<f32>) -> Self {
        self.0.anisotropyEnable = level.is_some() as _;
        if let Some(l) = level {
            self.0.maxAnisotropy = l;
        }

        self
    }

    /// The comparison function to apply to fetched data before filtering
    /// as described in the `Depth Compare Operation` section in Vulkan Specification.
    /// Specifying `None` switches off the comparison against a reference value during lookups.  
    /// Default: `None`
    pub const fn comparison(mut self, op: Option<CompareOp>) -> Self {
        self.0.compareEnable = op.is_some() as _;
        if let Some(c) = op {
            self.0.compareOp = c as _;
        }

        self
    }

    /// The values used to clamp the computed level-of-detail value,
    /// as described in the `Level-of-Detail Operation` section in Vulkan Specification.  
    /// Default: min_lod=0.0, max_lod=0.0
    /// # Panics
    /// `max_lod` must be greater than or equal to `min_lod`
    pub fn lod_clamp(mut self, min_lod: f32, max_lod: f32) -> Self {
        assert!(max_lod >= min_lod);
        self.0.minLod = min_lod;
        self.0.maxLod = max_lod;

        self
    }

    /// Whether to use unnormalized or normalized texel coordinates to address texels of the image.  
    /// Default: `false`
    /// # Safety
    /// User must meet the constraints as described in the "Valid Usage" section in the `VkSamplerCreateInfo` manual page
    pub const unsafe fn unnormalized_coordinates(mut self, use_unnormalized: bool) -> Self {
        self.0.unnormalizedCoordinates = use_unnormalized as _;

        self
    }

    /// Create a new sampler object
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// * `VK_ERROR_TOO_MANY_OBJECTS`
    #[implements]
    pub fn create<Device: crate::Device>(self, device: Device) -> crate::Result<SamplerObject<Device>> {
        let mut h = core::mem::MaybeUninit::uninit();
        unsafe {
            crate::vkresolve::create_sampler(device.native_ptr(), &self.0, std::ptr::null(), h.as_mut_ptr())
                .into_result()
                .map(|_| SamplerObject(h.assume_init(), device))
        }
    }
}

/// Specify behavior of sampling with texture coordinates outside an image
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum AddressingMode {
    /// The repeat wrap mode
    Repeat = VK_SAMPLER_ADDRESS_MODE_REPEAT as _,
    /// The mirrored repeat wrap mode
    MirroredRepeat = VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT as _,
    /// The clamp to edge wrap mode
    ClampToEdge = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE as _,
    /// The clamp to border wrap mode
    ClampToBorder = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER as _,
    /// The mirror clamp to edge wrap mode
    #[cfg(feature = "VK_KHR_mirror_clamp_to_edge")]
    MirrorClampToEdge = VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE as _,
}

/// Specify filter used for texture lookups
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum FilterMode {
    /// Nearest filtering
    Nearest = VK_FILTER_NEAREST as _,
    /// Linear filtering
    Linear = VK_FILTER_LINEAR as _,
}

/// Specify mipmap mode used for texture lookups
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MipmapFilterMode {
    /// Nearest filtering
    Nearest = VK_SAMPLER_MIPMAP_MODE_NEAREST as _,
    /// Linear filtering
    Linear = VK_SAMPLER_MIPMAP_MODE_LINEAR as _,
}

/// Specify border color used for texture lookups
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum BorderColor {
    /// A transparent, floating-point format, black color
    TransparentBlackF = VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK as _,
    /// A transparent, integer format, black color
    TransparentBlackI = VK_BORDER_COLOR_INT_TRANSPARENT_BLACK as _,
    /// An opaque, floating-point format, black color
    OpaqueBlackF = VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK as _,
    /// An opaque, integer format, black color
    OpaqueBlackI = VK_BORDER_COLOR_INT_OPAQUE_BLACK as _,
    /// An opaque, floating-point format, white color
    OpaqueWhiteF = VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE as _,
    /// An opaque, integer format, white color
    OpaqueWhiteI = VK_BORDER_COLOR_INT_OPAQUE_WHITE as _,
}
