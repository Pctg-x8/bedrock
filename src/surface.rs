//! Vulkan Surface/Swapchain Extensions

use super::*;

/// A semaphore or a fence
pub enum CompletionHandler<Fence: crate::Fence, Semaphore: crate::Semaphore> {
    /// A Host synchronizer(aka Fence)
    Host(Fence),
    /// A Queue synchronizer(aka Semaphore)
    Queue(Semaphore),
}

/// A semaphore or a fence, externally synchronized on host access
pub enum CompletionHandlerMut<'d> {
    /// A Host synchronizer(aka Fence)
    Host(VkHandleRefMut<'d, VkFence>),
    /// A Queue synchronizer(aka Semaphore)
    Queue(VkHandleRefMut<'d, VkSemaphore>),
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_surface")] {
        mod surface;
        pub use self::surface::*;
    }
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_swapchain")] {
        mod swapchain;
        pub use self::swapchain::*;
    }
}

cfg_if! {
    if #[cfg(all(feature = "VK_KHR_swapchain", feature = "VK_KHR_surface"))] {
        mod swapchain_surface_integrated;
        pub use self::swapchain_surface_integrated::*;
    }
}

cfg_if! {
    if #[cfg(feature = "VK_EXT_full_screen_exclusive")] {
        mod full_screen_exclusive;
    }
}
