//! Glue library between Vulkan and Rust
//!
//! # Copyright
//! Some documentation comments are from Vulkan Manual Page.
//! Copyright (c) 2014-2017 Khronos Group.
//!
//! # Compile Options
//! - `Implements`: Enable Vulkan implementations(functions)
//! - `Multithreaded`: Enables to use objects from some threads(experimental)
//! - `Presentation`: Enable rendering features to Window/Display(`VK_KHR_surface`/`VK_KHR_swapchain`/`VK_KHR_display`)
//! - `alloc`(default): Enable extra functionalities that may allocate some memory inside
//! - `VK_***`: Enable Vulkan extensions(same name as each extensions)
#![warn(clippy::all)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(html_root_url = "https://docs.ct2.io/bedrock/mod-peridot/")]

// Platform Extras
#[cfg(feature = "VK_KHR_android_surface")]
extern crate android;
#[cfg(feature = "DynamicLoaded")]
extern crate libloading;
#[cfg(any(feature = "VK_KHR_xlib_surface", feature = "VK_EXT_acquire_xlib_display"))]
extern crate x11;
#[cfg(feature = "VK_KHR_xcb_surface")]
extern crate xcb;

use cfg_if::cfg_if;
use derives::*;

pub use derives::SpecializationConstants;

#[macro_use]
pub mod vk;
use vk::*;
pub mod error;
mod resolver;
#[cfg(feature = "Implements")]
pub use resolver::ResolverInterface;
pub use resolver::{PFN, StaticCallable};

#[cfg(feature = "Implements")]
#[allow(dead_code)]
pub mod vkfn;
#[cfg(feature = "Implements")]
pub mod vkfn_wrapper;

macro_rules! DerefContainerWithGuardsBracketImpl {
    (for $t: path { $($required: item)* }) => {
        DerefContainerBracketImpl!(for $t { $($required)* });
        GuardsImpl!(for $t { $($required)* });
    }
}
macro_rules! DerefContainerBracketImpl {
    (unsafe for mut $t: path { $($required: item)* }) => {
        unsafe impl<'s, T> $t for &'s mut T where T: $t + ?Sized { $($required)* }
        unsafe impl<T> $t for Box<T> where T: $t + ?Sized { $($required)* }
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
        impl<T> $t for parking_lot::RwLockWriteGuard<'_, T> where T: $t + ?Sized { $($required)* }
    };
    (for $t: path { $($required: item)* }) => {
        impl<T> $t for std::cell::Ref<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::sync::RwLockReadGuard<'_, T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for parking_lot::RwLockReadGuard<'_, T> where T: $t + ?Sized { $($required)* }

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

pub type Result<T> = core::result::Result<T, VkResult>;

#[cfg(feature = "alloc")]
pub(crate) mod alloc;

mod handle;
pub use self::handle::*;

/// An object in Vulkan
pub trait VkObject: VkHandle {
    const TYPE: VkObjectType;

    /// Give a user-friendly name to this object.
    /// # Failures
    /// On failure, this command returns
    ///
    /// * `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    #[implements("VK_EXT_debug_utils")]
    fn set_name(&self, name: Option<&core::ffi::CStr>) -> crate::Result<()>
    where
        Self: DeviceChild<ConcreteDevice: InstanceChild<ConcreteInstance: InstanceExtensions>>,
        Self::Handle: VkRawHandle,
    {
        self.device()
            .set_object_name(&DebugUtilsObjectNameInfo::new(self, name))
    }
}
impl<T: VkObject + ?Sized> VkObject for &'_ T {
    const TYPE: VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for &'_ mut T {
    const TYPE: VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for std::rc::Rc<T> {
    const TYPE: VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for std::sync::Arc<T> {
    const TYPE: VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for std::cell::Ref<'_, T> {
    const TYPE: VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for std::cell::RefMut<'_, T> {
    const TYPE: VkObjectType = T::TYPE;
}
impl<T: VkObject + ?Sized> VkObject for std::sync::MutexGuard<'_, T> {
    const TYPE: VkObjectType = T::TYPE;
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

// Vulkan ReExports //
pub type DeviceSize = VkDeviceSize;
pub type Extent2D = VkExtent2D;
pub type Offset2D = VkOffset2D;
pub type Rect2D = VkRect2D;
pub type Viewport = VkViewport;
pub type Extent3D = VkExtent3D;
pub type Offset3D = VkOffset3D;
pub type Format = VkFormat;

// Spreading single value to all dimensions
impl Extent2D {
    pub const fn spread1(value: u32) -> Self {
        Self {
            width: value,
            height: value,
        }
    }
}
impl Extent3D {
    pub const fn spread1(value: u32) -> Self {
        Self {
            width: value,
            height: value,
            depth: value,
        }
    }
}
impl Offset2D {
    pub const fn spread1(value: i32) -> Self {
        Self { x: value, y: value }
    }
}
impl Offset3D {
    pub const fn spread1(value: i32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}

// into conversion to larger dimension //
impl Extent2D {
    pub const fn with_depth(self, depth: u32) -> Extent3D {
        Extent3D {
            width: self.width,
            height: self.height,
            depth,
        }
    }
}
impl Offset2D {
    pub const fn with_z(self, z: i32) -> Offset3D {
        Offset3D {
            x: self.x,
            y: self.y,
            z,
        }
    }
}
// AsRef for self //
impl AsRef<Extent3D> for Extent3D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<Extent2D> for Extent2D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<Offset3D> for Offset3D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<Offset2D> for Offset2D {
    fn as_ref(&self) -> &Self {
        self
    }
}

// AsRef Conversion to smaller-dimension
impl AsRef<Extent2D> for Extent3D {
    fn as_ref(&self) -> &Extent2D {
        unsafe { core::mem::transmute(self) }
    }
}
impl AsRef<Offset2D> for Offset3D {
    fn as_ref(&self) -> &Offset2D {
        unsafe { core::mem::transmute(self) }
    }
}

// Swizzling
impl Extent3D {
    pub const fn wh(&self) -> Extent2D {
        Extent2D {
            width: self.width,
            height: self.height,
        }
    }

    pub const fn wd(&self) -> Extent2D {
        Extent2D {
            width: self.width,
            height: self.depth,
        }
    }

    pub const fn hw(&self) -> Extent2D {
        Extent2D {
            width: self.height,
            height: self.width,
        }
    }

    pub const fn hd(&self) -> Extent2D {
        Extent2D {
            width: self.height,
            height: self.depth,
        }
    }

    pub const fn dw(&self) -> Extent2D {
        Extent2D {
            width: self.depth,
            height: self.width,
        }
    }

    pub const fn dh(&self) -> Extent2D {
        Extent2D {
            width: self.depth,
            height: self.height,
        }
    }
}
impl Offset3D {
    pub const fn xy(&self) -> Offset2D {
        Offset2D { x: self.x, y: self.y }
    }

    pub const fn xz(&self) -> Offset2D {
        Offset2D { x: self.x, y: self.z }
    }

    pub const fn yx(&self) -> Offset2D {
        Offset2D { x: self.y, y: self.x }
    }

    pub const fn yz(&self) -> Offset2D {
        Offset2D { x: self.y, y: self.z }
    }

    pub const fn zx(&self) -> Offset2D {
        Offset2D { x: self.z, y: self.x }
    }

    pub const fn zy(&self) -> Offset2D {
        Offset2D { x: self.z, y: self.y }
    }
}

/// Utility Constants
impl Extent2D {
    pub const ONE: Self = Self::spread1(1);
}
impl Extent3D {
    pub const ONE: Self = Self::spread1(1);
}
impl Offset2D {
    pub const ZERO: Self = Self::spread1(0);
}
impl Offset3D {
    pub const ZERO: Self = Self::spread1(0);
}

/// Viewport and Rect Util Functions
impl Extent2D {
    pub const fn into_rect(self, offset: Offset2D) -> Rect2D {
        Rect2D { offset, extent: self }
    }
}
impl From<Viewport> for Rect2D {
    fn from(vp: Viewport) -> Self {
        Rect2D {
            offset: Offset2D {
                x: vp.x as _,
                y: vp.y as _,
            },
            extent: Extent2D {
                width: vp.width as _,
                height: vp.height as _,
            },
        }
    }
}
impl Rect2D {
    pub const fn make_viewport(&self, depth_range: std::ops::Range<f32>) -> Viewport {
        Viewport {
            x: self.offset.x as _,
            y: self.offset.y as _,
            width: self.extent.width as _,
            height: self.extent.height as _,
            minDepth: depth_range.start,
            maxDepth: depth_range.end,
        }
    }
}
impl Viewport {
    pub const fn from_rect_with_depth_range(rect: &Rect2D, depth_range: core::ops::Range<f32>) -> Self {
        rect.make_viewport(depth_range)
    }

    pub const fn set_offset(&mut self, offset: &Offset2D) -> &mut Self {
        self.x = offset.x as _;
        self.y = offset.y as _;
        self
    }
    pub const fn set_extent(&mut self, extent: &Extent2D) -> &mut Self {
        self.width = extent.width as _;
        self.height = extent.height as _;
        self
    }
    pub const fn set_depth_range(&mut self, range: core::ops::Range<f32>) -> &mut Self {
        self.minDepth = range.start;
        self.maxDepth = range.end;
        self
    }
}

mod base;
pub(crate) mod ffi_helper;
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
