//! Glue library between Vulkan and Rust
//!
//! # Copyright
//! Some documentation comments are from Vulkan Manual Page.
//! Copyright (c) 2014-2017 Khronos Group.
//!
//! # Compile Options
//! - `Implements`: Enable Vulkan implementations(functions)
//! - `Multithreaded`: Enables to use objects from some threads(experimental)
//! - `Presentation`: Enable rendering features to Window/Display(`brvk::VK_KHR_surface`/`brvk::VK_KHR_swapchain`/`brvk::VK_KHR_display`)
//! - `alloc`(default): Enable extra functionalities that may allocate some memory inside
//! - `brvk::VK_***`: Enable Vulkan extensions(same name as each extensions)
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.ct2.io/bedrock/mod-peridot/")]

// Platform Extras
#[cfg(feature = "VK_KHR_android_surface")]
extern crate android;
#[cfg(any(feature = "VK_KHR_xlib_surface", feature = "VK_EXT_acquire_xlib_display"))]
extern crate x11;
#[cfg(feature = "VK_KHR_xcb_surface")]
extern crate xcb;

pub use bedrock_vk::{
    self as vk, VkDeviceSize as DeviceSize, VkExtent2D as Extent2D, VkExtent3D as Extent3D, VkFormat as Format,
    VkOffset2D as Offset2D, VkOffset3D as Offset3D, VkRect2D as Rect2D, VkViewport as Viewport,
};
use cfg_if::cfg_if;
use derives::*;

#[cfg(feature = "CustomResolver")]
pub use bedrock_vk::set_resolver;
#[cfg(feature = "DynamicLoaded")]
pub use bedrock_vk::{ResolvedFnCell, ResolverInterface};

pub use derives::SpecializationConstants;

pub mod error;

#[cfg(feature = "Implements")]
pub mod vkfn_wrapper;

macro_rules! DerefContainerWithGuardsBracketImpl {
    (for mut $t: path { $($required: item)* }) => {
        DerefContainerBracketImpl!(for mut $t { $($required)* });
        GuardsImpl!(for mut $t { $($required)* });
    };
    (for $t: path { $($required: item)* }) => {
        DerefContainerBracketImpl!(for $t { $($required)* });
        GuardsImpl!(for $t { $($required)* });
    };
}
macro_rules! DerefContainerBracketImpl {
    (unsafe for mut $t: path { $($required: item)* }) => {
        unsafe impl<'s, T> $t for &'s mut T where T: $t + ?Sized { $($required)* }
        unsafe impl<T> $t for Box<T> where T: $t + ?Sized { $($required)* }
        unsafe impl<T> $t for core::mem::ManuallyDrop<T> where T: $t { $($required)* }
    };
    (unsafe for $t: path { $($required: item)* }) => {
        unsafe impl<'s, T> $t for &'s T where T: $t + ?Sized { $($required)* }
        unsafe impl<T> $t for std::rc::Rc<T> where T: $t + ?Sized { $($required)* }
        unsafe impl<T> $t for std::sync::Arc<T> where T: $t + ?Sized { $($required)* }

        DerefContainerBracketImpl!(unsafe for mut $t { $($required)* });
    };
    (for mut $t: path { $($required: item)* }) => {
        impl<'s, T> $t for &'s mut T where T: $t + ?Sized { $($required)* }
        impl<T> $t for Box<T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for core::mem::ManuallyDrop<T> where T: $t { $($required)* }
    };
    (for $t: path { $($required: item)* }) => {
        impl<'s, T> $t for &'s T where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::rc::Rc<T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::sync::Arc<T> where T: $t + ?Sized { $($required)* }

        DerefContainerBracketImpl!(for mut $t { $($required)* });
    };
}
macro_rules! GuardsImpl {
    (for mut $t: path { $($required: item)* }) => {
        impl<T> $t for std::cell::RefMut<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::sync::RwLockWriteGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::sync::MutexGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for parking_lot::MutexGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for parking_lot::MappedMutexGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for parking_lot::RwLockWriteGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for parking_lot::MappedRwLockWriteGuard<'_, T> where T: $t + ?Sized { $($required)* }
    };
    (for $t: path { $($required: item)* }) => {
        impl<T> $t for std::cell::Ref<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::sync::RwLockReadGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for parking_lot::RwLockReadGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for parking_lot::MappedRwLockReadGuard<'_, T> where T: $t + ?Sized { $($required)* }

        GuardsImpl!(for mut $t { $($required)* });
    };
}
macro_rules! ForwardFnPtr {
    (deref $name: ident -> $t: ty) => {
        #[inline(always)]
        fn $name(&self) -> $t {
            (**self).$name()
        }
    };
}

pub type Result<T> = core::result::Result<T, ResultCode>;

#[cfg(feature = "alloc")]
pub(crate) mod alloc;

mod handle;
use crate::error::ResultCode;

pub use self::handle::*;

/// An result type of querying an array of objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use = "array query may finished incompletely"]
pub struct ArrayQueryResult<T> {
    /// The result value of the query.
    pub result: T,
    /// * false: Objects are queried successfully.
    /// * true: There are some objects left(i.e. buffer was too small).
    pub is_incomplete: bool,
}
impl ArrayQueryResult<()> {
    #[inline(always)]
    pub(crate) fn from_vk_result(r: vk::VkResult) -> Result<Self> {
        match r {
            vk::VK_SUCCESS => Ok(Self {
                result: (),
                is_incomplete: false,
            }),
            vk::VK_INCOMPLETE => Ok(Self {
                result: (),
                is_incomplete: true,
            }),
            e => match ResultCode(e) {
                e if e.is_err() => Err(e),
                e => unreachable!("unexpected result: {e:?}"),
            },
        }
    }
}
impl<T> ArrayQueryResult<T> {
    #[inline(always)]
    pub fn with_result<U>(self, v: U) -> ArrayQueryResult<U> {
        ArrayQueryResult {
            result: v,
            is_incomplete: self.is_incomplete,
        }
    }

    #[inline(always)]
    pub fn map_result<U>(self, f: impl FnOnce(T) -> U) -> ArrayQueryResult<U> {
        ArrayQueryResult {
            result: f(self.result),
            is_incomplete: self.is_incomplete,
        }
    }
}

#[cfg(feature = "VK_KHR_swapchain")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentResult {
    /// All requests presented successfully.
    Success,
    /// The presentation was suboptimal, but the swapchain can still be used.
    Suboptimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeoutableWaitResult {
    /// Wait operations completed successfully.
    Success,
    /// Wait operations timed out.
    Timeout,
}
impl TimeoutableWaitResult {
    pub(crate) fn from_vk_result(r: vk::VkResult) -> Self {
        match r {
            vk::VK_SUCCESS => Self::Success,
            vk::VK_TIMEOUT => Self::Timeout,
            e => unreachable!("unexpected result: {:?}", ResultCode(e)),
        }
    }
}

/// An object in Vulkan
pub trait VkObject: VkHandle {
    const TYPE: vk::VkObjectType;

    /// Give a user-friendly name to this object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `brvk::VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `brvk::VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_EXT_debug_utils")]
    fn set_name(&self, name: Option<&core::ffi::CStr>) -> crate::Result<()>
    where
        Self: DeviceChild<ConcreteDevice: InstanceChild<ConcreteInstance: InstanceDebugUtilsExtension>>,
        Self::Handle: vk::VkRawHandle,
    {
        self.device()
            .set_object_name(&DebugUtilsObjectNameInfo::new(self, name))
    }
}
impl<T: VkObject + ?Sized> VkObject for &'_ T {
    const TYPE: vk::VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for &'_ mut T {
    const TYPE: vk::VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for std::rc::Rc<T> {
    const TYPE: vk::VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for std::sync::Arc<T> {
    const TYPE: vk::VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for std::cell::Ref<'_, T> {
    const TYPE: vk::VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for std::cell::RefMut<'_, T> {
    const TYPE: vk::VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for std::sync::MutexGuard<'_, T> {
    const TYPE: vk::VkObjectType = T::TYPE;
}

// A single Number or a Range
pub trait AnalogNumRange<T> {
    fn begin(&self) -> T;
    fn end(&self) -> T;
    fn count(&self) -> T
    where
        T: std::ops::Sub<T, Output = T> + Copy,
    {
        self.end() - self.begin()
    }
}
impl<T> AnalogNumRange<T> for T
where
    T: std::ops::Add<u32, Output = T> + Copy,
{
    fn begin(&self) -> T {
        *self
    }
    fn end(&self) -> T {
        *self + 1
    }
}
impl<T> AnalogNumRange<T> for std::ops::Range<T>
where
    T: Copy,
{
    fn begin(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
}

pub(crate) mod ffi_helper;

mod base;
pub use base::*;
mod device;
pub use device::*;
mod sync;
pub use sync::*;
pub mod resources;
pub use resources::*;
#[macro_use]
mod descriptor;
pub use descriptor::*;
mod renderpass;
pub use self::renderpass::*;
mod framebuffer;
pub use framebuffer::*;
mod shading;
pub use shading::*;
mod command;
pub use command::*;
mod surface;
pub use surface::*;
mod debug;
#[allow(unused_imports)]
pub use debug::*;
mod ext;
pub use self::ext::*;
mod external;
#[allow(unused_imports)]
pub use external::*;
mod batching;
pub use self::batching::*;
mod dependency;
#[allow(unused_imports)]
pub use self::dependency::*;
mod query;
pub use self::query::*;

mod fmt;
pub use self::fmt::*;
