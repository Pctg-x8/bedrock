//! Format Helpers

/// Provides commonly used corresponding VkFormat for types
pub trait AsFormat {
    /// commonly used VkFormat for this type
    const FORMAT: crate::vk::VkFormat;
}

impl AsFormat for f32 { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32_SFLOAT; }
impl AsFormat for [f32; 2] { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32_SFLOAT; }
impl AsFormat for [f32; 3] { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32B32_SFLOAT; }
impl AsFormat for [f32; 4] { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32B32A32_SFLOAT; }
impl AsFormat for crate::Extent1D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32_SFLOAT; }
impl AsFormat for crate::Extent2D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32_SFLOAT; }
impl AsFormat for crate::Extent3D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32B32_SFLOAT; }
impl AsFormat for crate::Extent4D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32B32A32_SFLOAT; }
impl AsFormat for crate::Offset1D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32_SFLOAT; }
impl AsFormat for crate::Offset2D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32_SFLOAT; }
impl AsFormat for crate::Offset3D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32B32_SFLOAT; }
impl AsFormat for crate::Offset4D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32B32A32_SFLOAT; }
impl AsFormat for crate::vk::VkExtent2D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32_SFLOAT; }
impl AsFormat for crate::vk::VkExtent3D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32B32_SFLOAT; }
impl AsFormat for crate::vk::VkOffset2D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32_SFLOAT; }
impl AsFormat for crate::vk::VkOffset3D { const FORMAT: crate::vk::VkFormat = crate::vk::VK_FORMAT_R32G32B32_SFLOAT; }
