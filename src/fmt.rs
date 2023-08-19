//! Format Helpers

use crate::vk::*;

/// Provides commonly used corresponding VkFormat for types
pub trait AsFormat {
    /// commonly used VkFormat for this type
    const FORMAT: VkFormat;
}

impl AsFormat for f32 {
    const FORMAT: VkFormat = VK_FORMAT_R32_SFLOAT;
}
impl AsFormat for [f32; 2] {
    const FORMAT: VkFormat = VK_FORMAT_R32G32_SFLOAT;
}
impl AsFormat for [f32; 3] {
    const FORMAT: VkFormat = VK_FORMAT_R32G32B32_SFLOAT;
}
impl AsFormat for [f32; 4] {
    const FORMAT: VkFormat = VK_FORMAT_R32G32B32A32_SFLOAT;
}
impl AsFormat for VkExtent2D {
    const FORMAT: VkFormat = VK_FORMAT_R32G32_SFLOAT;
}
impl AsFormat for VkExtent3D {
    const FORMAT: VkFormat = VK_FORMAT_R32G32B32_SFLOAT;
}
impl AsFormat for VkOffset2D {
    const FORMAT: VkFormat = VK_FORMAT_R32G32_SFLOAT;
}
impl AsFormat for VkOffset3D {
    const FORMAT: VkFormat = VK_FORMAT_R32G32B32_SFLOAT;
}

/// For testing format traits
pub trait PixelFormat {
    fn bit_width(self) -> usize;
    fn components(self) -> FormatComponents;
    fn element_type(self) -> ElementType;
}
impl PixelFormat for VkFormat {
    fn bit_width(self) -> usize {
        match self {
            VK_FORMAT_R4G4_UNORM_PACK8
            | VK_FORMAT_R8_UNORM
            | VK_FORMAT_R8_SNORM
            | VK_FORMAT_R8_USCALED
            | VK_FORMAT_R8_SSCALED
            | VK_FORMAT_R8_UINT
            | VK_FORMAT_R8_SINT
            | VK_FORMAT_R8_SRGB
            | VK_FORMAT_S8_UINT => 8,
            VK_FORMAT_R4G4B4A4_UNORM_PACK16
            | VK_FORMAT_B4G4R4A4_UNORM_PACK16
            | VK_FORMAT_R5G6B5_UNORM_PACK16
            | VK_FORMAT_B5G6R5_UNORM_PACK16
            | VK_FORMAT_R5G5B5A1_UNORM_PACK16
            | VK_FORMAT_B5G5R5A1_UNORM_PACK16
            | VK_FORMAT_A1R5G5B5_UNORM_PACK16
            | VK_FORMAT_R8G8_UNORM
            | VK_FORMAT_R8G8_SNORM
            | VK_FORMAT_R8G8_USCALED
            | VK_FORMAT_R8G8_SSCALED
            | VK_FORMAT_R8G8_UINT
            | VK_FORMAT_R8G8_SINT
            | VK_FORMAT_R8G8_SRGB
            | VK_FORMAT_R16_UNORM
            | VK_FORMAT_R16_SNORM
            | VK_FORMAT_R16_USCALED
            | VK_FORMAT_R16_SSCALED
            | VK_FORMAT_R16_UINT
            | VK_FORMAT_R16_SINT
            | VK_FORMAT_R16_SFLOAT
            | VK_FORMAT_D16_UNORM => 16,
            VK_FORMAT_R8G8B8_UNORM
            | VK_FORMAT_R8G8B8_SNORM
            | VK_FORMAT_R8G8B8_USCALED
            | VK_FORMAT_R8G8B8_SSCALED
            | VK_FORMAT_R8G8B8_UINT
            | VK_FORMAT_R8G8B8_SINT
            | VK_FORMAT_R8G8B8_SRGB
            | VK_FORMAT_B8G8R8_UNORM
            | VK_FORMAT_B8G8R8_SNORM
            | VK_FORMAT_B8G8R8_USCALED
            | VK_FORMAT_B8G8R8_SSCALED
            | VK_FORMAT_B8G8R8_UINT
            | VK_FORMAT_B8G8R8_SINT
            | VK_FORMAT_B8G8R8_SRGB
            | VK_FORMAT_D16_UNORM_S8_UINT => 24,
            VK_FORMAT_R8G8B8A8_UNORM
            | VK_FORMAT_R8G8B8A8_SNORM
            | VK_FORMAT_R8G8B8A8_USCALED
            | VK_FORMAT_R8G8B8A8_SSCALED
            | VK_FORMAT_R8G8B8A8_UINT
            | VK_FORMAT_R8G8B8A8_SINT
            | VK_FORMAT_R8G8B8A8_SRGB
            | VK_FORMAT_B8G8R8A8_UNORM
            | VK_FORMAT_B8G8R8A8_SNORM
            | VK_FORMAT_B8G8R8A8_USCALED
            | VK_FORMAT_B8G8R8A8_SSCALED
            | VK_FORMAT_B8G8R8A8_UINT
            | VK_FORMAT_B8G8R8A8_SINT
            | VK_FORMAT_B8G8R8A8_SRGB
            | VK_FORMAT_A8B8G8R8_UNORM_PACK32
            | VK_FORMAT_A8B8G8R8_SNORM_PACK32
            | VK_FORMAT_A8B8G8R8_USCALED_PACK32
            | VK_FORMAT_A8B8G8R8_SSCALED_PACK32
            | VK_FORMAT_A8B8G8R8_UINT_PACK32
            | VK_FORMAT_A8B8G8R8_SINT_PACK32
            | VK_FORMAT_A8B8G8R8_SRGB_PACK32
            | VK_FORMAT_A2R10G10B10_UNORM_PACK32
            | VK_FORMAT_A2R10G10B10_SNORM_PACK32
            | VK_FORMAT_A2R10G10B10_USCALED_PACK32
            | VK_FORMAT_A2R10G10B10_SSCALED_PACK32
            | VK_FORMAT_A2R10G10B10_UINT_PACK32
            | VK_FORMAT_A2R10G10B10_SINT_PACK32
            | VK_FORMAT_R16G16_UNORM
            | VK_FORMAT_R16G16_SNORM
            | VK_FORMAT_R16G16_USCALED
            | VK_FORMAT_R16G16_SSCALED
            | VK_FORMAT_R16G16_UINT
            | VK_FORMAT_R16G16_SINT
            | VK_FORMAT_R16G16_SFLOAT
            | VK_FORMAT_R32_UINT
            | VK_FORMAT_R32_SINT
            | VK_FORMAT_R32_SFLOAT
            | VK_FORMAT_B10G11R11_UFLOAT_PACK32
            | VK_FORMAT_E5B9G9R9_UFLOAT_PACK32
            | VK_FORMAT_X8_D24_UNORM_PACK32
            | VK_FORMAT_D32_SFLOAT
            | VK_FORMAT_D24_UNORM_S8_UINT => 32,
            VK_FORMAT_D32_SFLOAT_S8_UINT => 40,
            VK_FORMAT_R16G16B16_UNORM
            | VK_FORMAT_R16G16B16_SNORM
            | VK_FORMAT_R16G16B16_USCALED
            | VK_FORMAT_R16G16B16_SSCALED
            | VK_FORMAT_R16G16B16_UINT
            | VK_FORMAT_R16G16B16_SINT
            | VK_FORMAT_R16G16B16_SFLOAT => 48,
            VK_FORMAT_R16G16B16A16_UNORM
            | VK_FORMAT_R16G16B16A16_SNORM
            | VK_FORMAT_R16G16B16A16_USCALED
            | VK_FORMAT_R16G16B16A16_SSCALED
            | VK_FORMAT_R16G16B16A16_UINT
            | VK_FORMAT_R16G16B16A16_SINT
            | VK_FORMAT_R16G16B16A16_SFLOAT
            | VK_FORMAT_R32G32_UINT
            | VK_FORMAT_R32G32_SINT
            | VK_FORMAT_R32G32_SFLOAT
            | VK_FORMAT_R64_UINT
            | VK_FORMAT_R64_SINT
            | VK_FORMAT_R64_SFLOAT => 64,
            VK_FORMAT_R32G32B32_UINT | VK_FORMAT_R32G32B32_SINT | VK_FORMAT_R32G32B32_SFLOAT => 96,
            VK_FORMAT_R32G32B32A32_UINT
            | VK_FORMAT_R32G32B32A32_SINT
            | VK_FORMAT_R32G32B32A32_SFLOAT
            | VK_FORMAT_R64G64_UINT
            | VK_FORMAT_R64G64_SINT
            | VK_FORMAT_R64G64_SFLOAT => 128,
            VK_FORMAT_R64G64B64_UINT | VK_FORMAT_R64G64B64_SINT | VK_FORMAT_R64G64B64_SFLOAT => 192,
            VK_FORMAT_R64G64B64A64_UINT | VK_FORMAT_R64G64B64A64_SINT | VK_FORMAT_R64G64B64A64_SFLOAT => 256,
            _ => 0,
        }
    }
    fn components(self) -> FormatComponents {
        match self {
            VK_FORMAT_UNDEFINED => FormatComponents::Undefined,
            VK_FORMAT_R8_UNORM
            | VK_FORMAT_R8_SNORM
            | VK_FORMAT_R8_USCALED
            | VK_FORMAT_R8_SSCALED
            | VK_FORMAT_R8_UINT
            | VK_FORMAT_R8_SINT
            | VK_FORMAT_R8_SRGB
            | VK_FORMAT_R16_UNORM
            | VK_FORMAT_R16_SNORM
            | VK_FORMAT_R16_USCALED
            | VK_FORMAT_R16_SSCALED
            | VK_FORMAT_R16_UINT
            | VK_FORMAT_R16_SINT
            | VK_FORMAT_R16_SFLOAT
            | VK_FORMAT_R32_UINT
            | VK_FORMAT_R32_SINT
            | VK_FORMAT_R32_SFLOAT
            | VK_FORMAT_R64_UINT
            | VK_FORMAT_R64_SINT
            | VK_FORMAT_R64_SFLOAT => FormatComponents::R,
            VK_FORMAT_R4G4_UNORM_PACK8
            | VK_FORMAT_R8G8_UNORM
            | VK_FORMAT_R8G8_SNORM
            | VK_FORMAT_R8G8_USCALED
            | VK_FORMAT_R8G8_SSCALED
            | VK_FORMAT_R8G8_UINT
            | VK_FORMAT_R8G8_SINT
            | VK_FORMAT_R8G8_SRGB
            | VK_FORMAT_R16G16_UNORM
            | VK_FORMAT_R16G16_SNORM
            | VK_FORMAT_R16G16_USCALED
            | VK_FORMAT_R16G16_SSCALED
            | VK_FORMAT_R16G16_UINT
            | VK_FORMAT_R16G16_SINT
            | VK_FORMAT_R16G16_SFLOAT
            | VK_FORMAT_R32G32_UINT
            | VK_FORMAT_R32G32_SINT
            | VK_FORMAT_R32G32_SFLOAT
            | VK_FORMAT_R64G64_UINT
            | VK_FORMAT_R64G64_SINT
            | VK_FORMAT_R64G64_SFLOAT => FormatComponents::RG,
            VK_FORMAT_R5G6B5_UNORM_PACK16
            | VK_FORMAT_B5G6R5_UNORM_PACK16
            | VK_FORMAT_R8G8B8_UNORM
            | VK_FORMAT_R8G8B8_SNORM
            | VK_FORMAT_R8G8B8_USCALED
            | VK_FORMAT_R8G8B8_SSCALED
            | VK_FORMAT_R8G8B8_UINT
            | VK_FORMAT_R8G8B8_SINT
            | VK_FORMAT_R8G8B8_SRGB
            | VK_FORMAT_B8G8R8_UNORM
            | VK_FORMAT_B8G8R8_SNORM
            | VK_FORMAT_B8G8R8_USCALED
            | VK_FORMAT_B8G8R8_SSCALED
            | VK_FORMAT_B8G8R8_UINT
            | VK_FORMAT_B8G8R8_SINT
            | VK_FORMAT_B8G8R8_SRGB
            | VK_FORMAT_R16G16B16_UNORM
            | VK_FORMAT_R16G16B16_SNORM
            | VK_FORMAT_R16G16B16_USCALED
            | VK_FORMAT_R16G16B16_SSCALED
            | VK_FORMAT_R16G16B16_UINT
            | VK_FORMAT_R16G16B16_SINT
            | VK_FORMAT_R16G16B16_SFLOAT
            | VK_FORMAT_R32G32B32_UINT
            | VK_FORMAT_R32G32B32_SINT
            | VK_FORMAT_R32G32B32_SFLOAT
            | VK_FORMAT_R64G64B64_UINT
            | VK_FORMAT_R64G64B64_SINT
            | VK_FORMAT_R64G64B64_SFLOAT
            | VK_FORMAT_B10G11R11_UFLOAT_PACK32 => FormatComponents::RGB,
            VK_FORMAT_R4G4B4A4_UNORM_PACK16
            | VK_FORMAT_B4G4R4A4_UNORM_PACK16
            | VK_FORMAT_R5G5B5A1_UNORM_PACK16
            | VK_FORMAT_B5G5R5A1_UNORM_PACK16
            | VK_FORMAT_A1R5G5B5_UNORM_PACK16
            | VK_FORMAT_R8G8B8A8_UNORM
            | VK_FORMAT_R8G8B8A8_SNORM
            | VK_FORMAT_R8G8B8A8_USCALED
            | VK_FORMAT_R8G8B8A8_SSCALED
            | VK_FORMAT_R8G8B8A8_UINT
            | VK_FORMAT_R8G8B8A8_SINT
            | VK_FORMAT_R8G8B8A8_SRGB
            | VK_FORMAT_B8G8R8A8_UNORM
            | VK_FORMAT_B8G8R8A8_SNORM
            | VK_FORMAT_B8G8R8A8_USCALED
            | VK_FORMAT_B8G8R8A8_SSCALED
            | VK_FORMAT_B8G8R8A8_UINT
            | VK_FORMAT_B8G8R8A8_SINT
            | VK_FORMAT_B8G8R8A8_SRGB
            | VK_FORMAT_A8B8G8R8_UNORM_PACK32
            | VK_FORMAT_A8B8G8R8_SNORM_PACK32
            | VK_FORMAT_A8B8G8R8_USCALED_PACK32
            | VK_FORMAT_A8B8G8R8_SSCALED_PACK32
            | VK_FORMAT_A8B8G8R8_UINT_PACK32
            | VK_FORMAT_A8B8G8R8_SINT_PACK32
            | VK_FORMAT_A8B8G8R8_SRGB_PACK32
            | VK_FORMAT_A2R10G10B10_UNORM_PACK32
            | VK_FORMAT_A2R10G10B10_SNORM_PACK32
            | VK_FORMAT_A2R10G10B10_USCALED_PACK32
            | VK_FORMAT_A2R10G10B10_SSCALED_PACK32
            | VK_FORMAT_A2R10G10B10_UINT_PACK32
            | VK_FORMAT_A2R10G10B10_SINT_PACK32
            | VK_FORMAT_A2B10G10R10_UNORM_PACK32
            | VK_FORMAT_A2B10G10R10_SNORM_PACK32
            | VK_FORMAT_A2B10G10R10_USCALED_PACK32
            | VK_FORMAT_A2B10G10R10_SSCALED_PACK32
            | VK_FORMAT_A2B10G10R10_UINT_PACK32
            | VK_FORMAT_A2B10G10R10_SINT_PACK32
            | VK_FORMAT_R16G16B16A16_UNORM
            | VK_FORMAT_R16G16B16A16_SNORM
            | VK_FORMAT_R16G16B16A16_USCALED
            | VK_FORMAT_R16G16B16A16_SSCALED
            | VK_FORMAT_R16G16B16A16_UINT
            | VK_FORMAT_R16G16B16A16_SINT
            | VK_FORMAT_R16G16B16A16_SFLOAT
            | VK_FORMAT_R32G32B32A32_UINT
            | VK_FORMAT_R32G32B32A32_SINT
            | VK_FORMAT_R32G32B32A32_SFLOAT
            | VK_FORMAT_R64G64B64A64_SINT
            | VK_FORMAT_R64G64B64A64_UINT
            | VK_FORMAT_R64G64B64A64_SFLOAT => FormatComponents::RGBA,
            VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 => FormatComponents::EBGR,
            VK_FORMAT_D16_UNORM | VK_FORMAT_X8_D24_UNORM_PACK32 | VK_FORMAT_D32_SFLOAT => FormatComponents::D,
            VK_FORMAT_S8_UINT => FormatComponents::S,
            VK_FORMAT_D16_UNORM_S8_UINT | VK_FORMAT_D24_UNORM_S8_UINT | VK_FORMAT_D32_SFLOAT_S8_UINT => {
                FormatComponents::DS
            }
            _ => FormatComponents::Compressed,
        }
    }
    #[allow(non_upper_case_globals)]
    fn element_type(self) -> ElementType {
        match self {
            VK_FORMAT_UNDEFINED => ElementType::Undefined,
            VK_FORMAT_R4G4_UNORM_PACK8
            | VK_FORMAT_R4G4B4A4_UNORM_PACK16
            | VK_FORMAT_B4G4R4A4_UNORM_PACK16
            | VK_FORMAT_R5G6B5_UNORM_PACK16
            | VK_FORMAT_B5G6R5_UNORM_PACK16
            | VK_FORMAT_R5G5B5A1_UNORM_PACK16
            | VK_FORMAT_B5G5R5A1_UNORM_PACK16
            | VK_FORMAT_A1R5G5B5_UNORM_PACK16
            | VK_FORMAT_R8_UNORM
            | VK_FORMAT_R8G8_UNORM
            | VK_FORMAT_R8G8B8_UNORM
            | VK_FORMAT_B8G8R8_UNORM
            | VK_FORMAT_R8G8B8A8_UNORM
            | VK_FORMAT_B8G8R8A8_UNORM
            | VK_FORMAT_A8B8G8R8_UNORM_PACK32
            | VK_FORMAT_A2R10G10B10_UNORM_PACK32
            | VK_FORMAT_A2B10G10R10_UNORM_PACK32
            | VK_FORMAT_R16G16B16_UNORM
            | VK_FORMAT_D16_UNORM
            | VK_FORMAT_X8_D24_UNORM_PACK32
            | VK_FORMAT_BC1_RGB_UNORM_BLOCK
            | VK_FORMAT_BC1_RGBA_UNORM_BLOCK
            | VK_FORMAT_BC2_UNORM_BLOCK
            | VK_FORMAT_BC3_UNORM_BLOCK
            | VK_FORMAT_BC4_UNORM_BLOCK
            | VK_FORMAT_BC5_UNORM_BLOCK
            | VK_FORMAT_BC7_UNORM_BLOCK
            | VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK
            | VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK
            | VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK
            | VK_FORMAT_EAC_R11_UNORM_BLOCK
            | VK_FORMAT_EAC_R11G11_UNORM_BLOCK
            | VK_FORMAT_ASTC_4x4_UNORM_BLOCK
            | VK_FORMAT_ASTC_5x4_UNORM_BLOCK
            | VK_FORMAT_ASTC_5x5_UNORM_BLOCK
            | VK_FORMAT_ASTC_6x5_UNORM_BLOCK
            | VK_FORMAT_ASTC_6x6_UNORM_BLOCK
            | VK_FORMAT_ASTC_8x5_UNORM_BLOCK
            | VK_FORMAT_ASTC_8x6_UNORM_BLOCK
            | VK_FORMAT_ASTC_8x8_UNORM_BLOCK
            | VK_FORMAT_ASTC_10x5_UNORM_BLOCK
            | VK_FORMAT_ASTC_10x6_UNORM_BLOCK
            | VK_FORMAT_ASTC_10x8_UNORM_BLOCK
            | VK_FORMAT_ASTC_10x10_UNORM_BLOCK
            | VK_FORMAT_ASTC_12x10_UNORM_BLOCK
            | VK_FORMAT_ASTC_12x12_UNORM_BLOCK => ElementType::UNORM,
            #[cfg(feature = "VK_IMG_format_pvrtc")]
            VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG
            | VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG
            | VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG
            | VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG => ElementType::UNORM,
            VK_FORMAT_R8_SNORM
            | VK_FORMAT_R8G8_SNORM
            | VK_FORMAT_R8G8B8_SNORM
            | VK_FORMAT_B8G8R8_SNORM
            | VK_FORMAT_R8G8B8A8_SNORM
            | VK_FORMAT_B8G8R8A8_SNORM
            | VK_FORMAT_A8B8G8R8_SNORM_PACK32
            | VK_FORMAT_A2R10G10B10_SNORM_PACK32
            | VK_FORMAT_A2B10G10R10_SNORM_PACK32
            | VK_FORMAT_R16_SNORM
            | VK_FORMAT_R16G16_SNORM
            | VK_FORMAT_R16G16B16_SNORM
            | VK_FORMAT_R16G16B16A16_SNORM
            | VK_FORMAT_BC4_SNORM_BLOCK
            | VK_FORMAT_BC5_SNORM_BLOCK
            | VK_FORMAT_EAC_R11_SNORM_BLOCK
            | VK_FORMAT_EAC_R11G11_SNORM_BLOCK => ElementType::SNORM,
            VK_FORMAT_R8_USCALED
            | VK_FORMAT_R8G8_USCALED
            | VK_FORMAT_R8G8B8_USCALED
            | VK_FORMAT_B8G8R8_USCALED
            | VK_FORMAT_R8G8B8A8_USCALED
            | VK_FORMAT_B8G8R8A8_USCALED
            | VK_FORMAT_A8B8G8R8_USCALED_PACK32
            | VK_FORMAT_A2R10G10B10_USCALED_PACK32
            | VK_FORMAT_A2B10G10R10_USCALED_PACK32
            | VK_FORMAT_R16_USCALED
            | VK_FORMAT_R16G16_USCALED
            | VK_FORMAT_R16G16B16_USCALED
            | VK_FORMAT_R16G16B16A16_USCALED => ElementType::USCALED,
            VK_FORMAT_R8_SSCALED
            | VK_FORMAT_R8G8_SSCALED
            | VK_FORMAT_R8G8B8_SSCALED
            | VK_FORMAT_B8G8R8_SSCALED
            | VK_FORMAT_R8G8B8A8_SSCALED
            | VK_FORMAT_B8G8R8A8_SSCALED
            | VK_FORMAT_A8B8G8R8_SSCALED_PACK32
            | VK_FORMAT_A2R10G10B10_SSCALED_PACK32
            | VK_FORMAT_A2B10G10R10_SSCALED_PACK32
            | VK_FORMAT_R16_SSCALED
            | VK_FORMAT_R16G16_SSCALED
            | VK_FORMAT_R16G16B16_SSCALED
            | VK_FORMAT_R16G16B16A16_SSCALED => ElementType::SSCALED,
            VK_FORMAT_R8_UINT
            | VK_FORMAT_R8G8_UINT
            | VK_FORMAT_R8G8B8_UINT
            | VK_FORMAT_B8G8R8_UINT
            | VK_FORMAT_R8G8B8A8_UINT
            | VK_FORMAT_B8G8R8A8_UINT
            | VK_FORMAT_A8B8G8R8_UINT_PACK32
            | VK_FORMAT_A2R10G10B10_UINT_PACK32
            | VK_FORMAT_A2B10G10R10_UINT_PACK32
            | VK_FORMAT_R16_UINT
            | VK_FORMAT_R16G16_UINT
            | VK_FORMAT_R16G16B16_UINT
            | VK_FORMAT_R16G16B16A16_UINT
            | VK_FORMAT_R32_UINT
            | VK_FORMAT_R32G32_UINT
            | VK_FORMAT_R32G32B32_UINT
            | VK_FORMAT_R32G32B32A32_UINT
            | VK_FORMAT_R64_UINT
            | VK_FORMAT_R64G64_UINT
            | VK_FORMAT_R64G64B64_UINT
            | VK_FORMAT_R64G64B64A64_UINT
            | VK_FORMAT_S8_UINT => ElementType::UINT,
            VK_FORMAT_R8_SINT
            | VK_FORMAT_R8G8_SINT
            | VK_FORMAT_R8G8B8_SINT
            | VK_FORMAT_B8G8R8_SINT
            | VK_FORMAT_R8G8B8A8_SINT
            | VK_FORMAT_B8G8R8A8_SINT
            | VK_FORMAT_A8B8G8R8_SINT_PACK32
            | VK_FORMAT_A2R10G10B10_SINT_PACK32
            | VK_FORMAT_A2B10G10R10_SINT_PACK32
            | VK_FORMAT_R16_SINT
            | VK_FORMAT_R16G16_SINT
            | VK_FORMAT_R16G16B16_SINT
            | VK_FORMAT_R16G16B16A16_SINT
            | VK_FORMAT_R32_SINT
            | VK_FORMAT_R32G32_SINT
            | VK_FORMAT_R32G32B32_SINT
            | VK_FORMAT_R32G32B32A32_SINT
            | VK_FORMAT_R64_SINT
            | VK_FORMAT_R64G64_SINT
            | VK_FORMAT_R64G64B64_SINT
            | VK_FORMAT_R64G64B64A64_SINT => ElementType::SINT,
            VK_FORMAT_R8_SRGB
            | VK_FORMAT_R8G8_SRGB
            | VK_FORMAT_R8G8B8_SRGB
            | VK_FORMAT_B8G8R8_SRGB
            | VK_FORMAT_R8G8B8A8_SRGB
            | VK_FORMAT_B8G8R8A8_SRGB
            | VK_FORMAT_A8B8G8R8_SRGB_PACK32
            | VK_FORMAT_BC1_RGB_SRGB_BLOCK
            | VK_FORMAT_BC1_RGBA_SRGB_BLOCK
            | VK_FORMAT_BC2_SRGB_BLOCK
            | VK_FORMAT_BC3_SRGB_BLOCK
            | VK_FORMAT_BC7_SRGB_BLOCK
            | VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK
            | VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK
            | VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK
            | VK_FORMAT_ASTC_4x4_SRGB_BLOCK
            | VK_FORMAT_ASTC_5x4_SRGB_BLOCK
            | VK_FORMAT_ASTC_5x5_SRGB_BLOCK
            | VK_FORMAT_ASTC_6x5_SRGB_BLOCK
            | VK_FORMAT_ASTC_6x6_SRGB_BLOCK
            | VK_FORMAT_ASTC_8x5_SRGB_BLOCK
            | VK_FORMAT_ASTC_8x6_SRGB_BLOCK
            | VK_FORMAT_ASTC_8x8_SRGB_BLOCK
            | VK_FORMAT_ASTC_10x5_SRGB_BLOCK
            | VK_FORMAT_ASTC_10x6_SRGB_BLOCK
            | VK_FORMAT_ASTC_10x8_SRGB_BLOCK
            | VK_FORMAT_ASTC_10x10_SRGB_BLOCK
            | VK_FORMAT_ASTC_12x10_SRGB_BLOCK
            | VK_FORMAT_ASTC_12x12_SRGB_BLOCK => ElementType::SRGB,
            #[cfg(feature = "VK_IMG_format_pvrtc")]
            VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG
            | VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG
            | VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG
            | VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG => ElementType::SRGB,
            VK_FORMAT_R16_SFLOAT
            | VK_FORMAT_R16G16_SFLOAT
            | VK_FORMAT_R16G16B16_SFLOAT
            | VK_FORMAT_R16G16B16A16_SFLOAT
            | VK_FORMAT_R32_SFLOAT
            | VK_FORMAT_R32G32_SFLOAT
            | VK_FORMAT_R32G32B32_SFLOAT
            | VK_FORMAT_R32G32B32A32_SFLOAT
            | VK_FORMAT_R64_SFLOAT
            | VK_FORMAT_R64G64_SFLOAT
            | VK_FORMAT_R64G64B64_SFLOAT
            | VK_FORMAT_R64G64B64A64_SFLOAT
            | VK_FORMAT_D32_SFLOAT
            | VK_FORMAT_BC6H_SFLOAT_BLOCK => ElementType::SFLOAT,
            VK_FORMAT_B10G11R11_UFLOAT_PACK32 | VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 | VK_FORMAT_BC6H_UFLOAT_BLOCK => {
                ElementType::UFLOAT
            }
            _ => ElementType::Compound,
        }
    }
}

/// Arbitrary queries of Format
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FormatQuery(pub VkFormat);
impl FormatQuery {
    pub fn eq_bit_width(self, w: usize) -> Self {
        if self.0.bit_width() == w {
            self
        } else {
            FormatQuery(VK_FORMAT_UNDEFINED)
        }
    }
    pub fn has_components(self, c: FormatComponents) -> Self {
        if c.satisfy(self.0) {
            self
        } else {
            FormatQuery(VK_FORMAT_UNDEFINED)
        }
    }
    pub fn is_component_of(self, c: FormatComponents) -> Self {
        if c.satisfy_eq(self.0) {
            self
        } else {
            FormatQuery(VK_FORMAT_UNDEFINED)
        }
    }
    pub fn has_element_of(self, e: ElementType) -> Self {
        if self.0.element_type() == e {
            self
        } else {
            FormatQuery(VK_FORMAT_UNDEFINED)
        }
    }
    pub const fn passed(self) -> bool {
        self.0 != VK_FORMAT_UNDEFINED
    }

    /// convert UNORM to SRGB if exists
    pub const fn srgb(self) -> Option<VkFormat> {
        match self.0 {
            VK_FORMAT_R8_UNORM => Some(VK_FORMAT_R8_SRGB),
            VK_FORMAT_R8G8_UNORM => Some(VK_FORMAT_R8G8_SRGB),
            VK_FORMAT_R8G8B8_UNORM => Some(VK_FORMAT_R8G8B8_SRGB),
            VK_FORMAT_B8G8R8_UNORM => Some(VK_FORMAT_B8G8R8_SRGB),
            VK_FORMAT_R8G8B8A8_UNORM => Some(VK_FORMAT_R8G8B8A8_SRGB),
            VK_FORMAT_B8G8R8A8_UNORM => Some(VK_FORMAT_B8G8R8A8_SRGB),
            VK_FORMAT_A8B8G8R8_UNORM_PACK32 => Some(VK_FORMAT_A8B8G8R8_SRGB_PACK32),
            _ => None,
        }
    }
    /// convert to UNORM if exists
    pub const fn unorm(self) -> Option<VkFormat> {
        match self.0 {
            VK_FORMAT_R8_SRGB | VK_FORMAT_R8_UNORM => Some(VK_FORMAT_R8_UNORM),
            VK_FORMAT_R8G8_SRGB | VK_FORMAT_R8G8_UNORM => Some(VK_FORMAT_R8G8_UNORM),
            VK_FORMAT_R8G8B8_SRGB | VK_FORMAT_R8G8B8_UNORM => Some(VK_FORMAT_R8G8B8_UNORM),
            VK_FORMAT_B8G8R8_SRGB | VK_FORMAT_B8G8R8_UNORM => Some(VK_FORMAT_B8G8R8_UNORM),
            VK_FORMAT_R8G8B8A8_SRGB | VK_FORMAT_R8G8B8A8_UNORM => Some(VK_FORMAT_R8G8B8A8_UNORM),
            VK_FORMAT_B8G8R8A8_SRGB | VK_FORMAT_B8G8R8A8_UNORM => Some(VK_FORMAT_B8G8R8A8_UNORM),
            VK_FORMAT_A8B8G8R8_SRGB_PACK32 | VK_FORMAT_A8B8G8R8_UNORM_PACK32 => Some(VK_FORMAT_A8B8G8R8_UNORM_PACK32),
            _ => None,
        }
    }
}
/// Predication style of Format Selection Query
#[derive(Clone)]
pub struct FormatQueryPred {
    bit_width: Option<usize>,
    req_components: Option<FormatComponents>,
    req_elements_of: Option<ElementType>,
}
/// Empty data
impl Default for FormatQueryPred {
    fn default() -> Self {
        FormatQueryPred {
            bit_width: None,
            req_components: None,
            req_elements_of: None,
        }
    }
}
impl FormatQueryPred {
    pub fn bit(&mut self, b: usize) -> &mut Self {
        self.bit_width = Some(b);
        self
    }
    pub fn components(&mut self, c: FormatComponents) -> &mut Self {
        self.req_components = Some(c);
        self
    }
    pub fn elements(&mut self, e: ElementType) -> &mut Self {
        self.req_elements_of = Some(e);
        self
    }

    pub fn satisfy(&self, f: VkFormat) -> bool {
        self.bit_width.map_or(true, |b| f.bit_width() == b)
            && self.req_components.map_or(true, |c| c.satisfy(f))
            && self.req_elements_of.map_or(true, |e| f.element_type() == e)
    }
}

/// Containing Components in Format(Order is not considered)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FormatComponents {
    Undefined,
    R,
    RG,
    RGB,
    RGBA,
    EBGR,
    D,
    S,
    DS,
    Compressed,
}
/// Containing component element in format
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Undefined,
    UNORM,
    SNORM,
    UINT,
    SINT,
    SFLOAT,
    UFLOAT,
    SRGB,
    USCALED,
    SSCALED,
    Compound,
}

impl FormatComponents {
    pub fn has(self, o: Self) -> bool {
        use self::FormatComponents::*;
        match self {
            R => o == R || o == RG || o == RGB || o == RGBA,
            RG => o == RG || o == RGB || o == RGBA,
            RGB => o == RGB || o == RGBA,
            D => o == D || o == DS,
            S => o == S || o == DS,
            t => t == o,
        }
    }

    pub fn satisfy(self, f: VkFormat) -> bool {
        self.has(f.components())
    }

    pub fn satisfy_eq(self, f: VkFormat) -> bool {
        f.components() == self
    }
}
