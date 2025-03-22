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
mod vkfn;

#[cfg(feature = "Implements")]
mod fnconv;

macro_rules! DerefContainerBracketImpl {
    (for mut $t: path { $($required: item)* }) => {
        impl<'s, T> $t for &'s mut T where T: $t + ?Sized { $($required)* }
        impl<T> $t for Box<T> where T: $t + ?Sized { $($required)* }
    };
    (for $t: path { $($required: item)* }) => {
        impl<'s, T> $t for &'s T where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::rc::Rc<T> where T: $t + ?Sized { $($required)* }
        impl<T> $t for std::sync::Arc<T> where T: $t + ?Sized { $($required)* }

        DerefContainerBracketImpl!(for mut $t { $($required)* });
    }
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

// Vulkan ReExports //
pub type LayerProperties = crate::vk::VkLayerProperties;
pub type ExtensionProperties = crate::vk::VkExtensionProperties;
pub type Format = crate::vk::VkFormat;
pub type Extent2D = crate::vk::VkExtent2D;
pub type Offset2D = crate::vk::VkOffset2D;
pub type Extent3D = crate::vk::VkExtent3D;
pub type Offset3D = crate::vk::VkOffset3D;
pub type DeviceSize = crate::vk::VkDeviceSize;
pub type Rect2D = crate::vk::VkRect2D;
pub type Viewport = crate::vk::VkViewport;
pub type ClearAttachment = crate::vk::VkClearAttachment;
pub type ClearRect = crate::vk::VkClearRect;

/// Enumeration API return codes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumerationResult {
    Complete,
    Incomplete,
}

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
        Self: DeviceChild,
        Self::ConcreteDevice: InstanceChild,
        Self::Handle: VkRawHandle,
    {
        self.device()
            .set_object_name(&DebugUtilsObjectNameInfo::new(self, name))
    }
}
impl<T> VkObject for &'_ T
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for &'_ mut T
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for std::rc::Rc<T>
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for std::sync::Arc<T>
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for std::cell::Ref<'_, T>
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for std::cell::RefMut<'_, T>
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}
impl<T> VkObject for std::sync::MutexGuard<'_, T>
where
    T: VkObject + ?Sized,
{
    const TYPE: VkObjectType = T::TYPE;
}

// A single Number or a Range
pub trait AnalogNumRange<T> {
    fn begin(&self) -> T;
    fn end(&self) -> T;
    fn count(&self) -> T
    where
        T: ::std::ops::Sub<T, Output = T> + Copy,
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

/// A Value Object represents the Vulkan version number
// Note: (MSB) major | minor | patch (LSB) の順でビットが割り当てられているので合成した状態の比較で正しい順序になる
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(u32);
impl Version {
    /// Version 1.0.0
    pub const V1: Self = Self::new(0, 1, 0, 0);

    /// Construct an object from discrete values
    pub const fn new(variant: u8, major: u16, minor: u16, patch: u16) -> Self {
        Self(crate::vk::VK_MAKE_VERSION(variant, major, minor, patch))
    }

    /// Construct an object from a raw value
    pub const fn from_raw(v: u32) -> Self {
        Self(v)
    }

    /// Gets a raw value from this object
    pub const fn raw(&self) -> u32 {
        self.0
    }

    /// Version variant number
    pub const fn variant(&self) -> u8 {
        crate::vk::VK_VARIANT_VERSION(self.0)
    }

    /// Major version number
    pub const fn major(&self) -> u16 {
        crate::vk::VK_MAJOR_VERSION(self.0)
    }

    /// Minor version number
    pub const fn minor(&self) -> u16 {
        crate::vk::VK_MINOR_VERSION(self.0)
    }

    /// Patch version number
    pub const fn patch(&self) -> u16 {
        crate::vk::VK_PATCH_VERSION(self.0)
    }
}
impl core::fmt::Display for Version {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Variantを含めたバージョン表示
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

// Spreading single value to all dimensions
impl VkExtent2D {
    pub const fn spread1(value: u32) -> Self {
        Self {
            width: value,
            height: value,
        }
    }
}
impl VkExtent3D {
    pub const fn spread1(value: u32) -> Self {
        Self {
            width: value,
            height: value,
            depth: value,
        }
    }
}
impl VkOffset2D {
    pub const fn spread1(value: i32) -> Self {
        Self { x: value, y: value }
    }
}
impl VkOffset3D {
    pub const fn spread1(value: i32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}

// into conversion to larger dimension //
impl VkExtent2D {
    pub const fn with_depth(self, depth: u32) -> VkExtent3D {
        VkExtent3D {
            width: self.width,
            height: self.height,
            depth,
        }
    }
}
impl VkOffset2D {
    pub const fn with_z(self, z: i32) -> VkOffset3D {
        VkOffset3D {
            x: self.x,
            y: self.y,
            z,
        }
    }
}
// AsRef for self //
impl AsRef<VkExtent3D> for VkExtent3D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<VkExtent2D> for VkExtent2D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<VkOffset3D> for VkOffset3D {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsRef<VkOffset2D> for VkOffset2D {
    fn as_ref(&self) -> &Self {
        self
    }
}

// AsRef Conversion to smaller-dimension
impl AsRef<VkExtent2D> for VkExtent3D {
    fn as_ref(&self) -> &VkExtent2D {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsRef<VkOffset2D> for VkOffset3D {
    fn as_ref(&self) -> &VkOffset2D {
        unsafe { std::mem::transmute(self) }
    }
}

// Swizzling
impl VkExtent3D {
    #[inline]
    pub const fn wh(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.width,
            height: self.height,
        }
    }

    #[inline]
    pub const fn wd(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.width,
            height: self.depth,
        }
    }

    #[inline]
    pub const fn hw(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.height,
            height: self.width,
        }
    }

    #[inline]
    pub const fn hd(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.height,
            height: self.depth,
        }
    }

    #[inline]
    pub const fn dw(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.depth,
            height: self.width,
        }
    }

    #[inline]
    pub const fn dh(&self) -> VkExtent2D {
        VkExtent2D {
            width: self.depth,
            height: self.height,
        }
    }
}
impl VkOffset3D {
    #[inline]
    pub const fn xy(&self) -> VkOffset2D {
        VkOffset2D { x: self.x, y: self.y }
    }

    #[inline]
    pub const fn xz(&self) -> VkOffset2D {
        VkOffset2D { x: self.x, y: self.z }
    }

    #[inline]
    pub const fn yx(&self) -> VkOffset2D {
        VkOffset2D { x: self.y, y: self.x }
    }

    #[inline]
    pub const fn yz(&self) -> VkOffset2D {
        VkOffset2D { x: self.y, y: self.z }
    }

    #[inline]
    pub const fn zx(&self) -> VkOffset2D {
        VkOffset2D { x: self.z, y: self.x }
    }

    #[inline]
    pub const fn zy(&self) -> VkOffset2D {
        VkOffset2D { x: self.z, y: self.y }
    }
}

/// Utility Constants
impl VkExtent2D {
    pub const ONE: Self = Self::spread1(1);
}
impl VkExtent3D {
    pub const ONE: Self = Self::spread1(1);
}
impl VkOffset2D {
    pub const ZERO: Self = Self::spread1(0);
}
impl VkOffset3D {
    pub const ZERO: Self = Self::spread1(0);
}

/// Viewport and Rect Util Functions
impl VkExtent2D {
    pub const fn into_rect(self, offset: VkOffset2D) -> VkRect2D {
        VkRect2D { offset, extent: self }
    }
}
impl From<VkViewport> for VkRect2D {
    fn from(vp: VkViewport) -> Self {
        VkRect2D {
            offset: VkOffset2D {
                x: vp.x as _,
                y: vp.y as _,
            },
            extent: VkExtent2D {
                width: vp.width as _,
                height: vp.height as _,
            },
        }
    }
}
impl VkRect2D {
    pub const fn make_viewport(&self, depth_range: std::ops::Range<f32>) -> VkViewport {
        VkViewport {
            x: self.offset.x as _,
            y: self.offset.y as _,
            width: self.extent.width as _,
            height: self.extent.height as _,
            minDepth: depth_range.start,
            maxDepth: depth_range.end,
        }
    }
}
impl VkViewport {
    pub const fn from_rect_with_depth_range(rect: &VkRect2D, depth_range: std::ops::Range<f32>) -> Self {
        rect.make_viewport(depth_range)
    }

    pub fn set_offset(&mut self, offset: &VkOffset2D) -> &mut Self {
        self.x = offset.x as _;
        self.y = offset.y as _;
        self
    }
    pub fn set_extent(&mut self, extent: &VkExtent2D) -> &mut Self {
        self.width = extent.width as _;
        self.height = extent.height as _;
        self
    }
    pub fn set_depth_range(&mut self, range: std::ops::Range<f32>) -> &mut Self {
        self.minDepth = range.start;
        self.maxDepth = range.end;
        self
    }
}

mod base;
pub use base::*;
pub(crate) mod ffi_helper;
mod instance;
pub use instance::*;
mod physical_device;
pub use physical_device::*;
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
