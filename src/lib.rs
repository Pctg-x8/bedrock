
extern crate libc;
// Platform Extras
#[cfg(any(
    feature = "VK_KHR_win32_surface", feature = "VK_KHR_external_memory_win32",
    feature = "VK_KHR_external_semaphore_win32", feature = "VK_KHR_external_fence_win32",
    feature = "VK_NV_external_memory_win32"
))]
extern crate winapi;
#[cfg(any(feature = "VK_KHR_xlib_surface", feature = "VK_EXT_acquire_xlib_display"))]
extern crate x11;

#[macro_use]
mod vk;
use vk::*;
// use std::ffi::CString;

#[cfg(feature = "FeImplements")] mod fnconv;

pub type Result<T> = std::result::Result<T, VkResult>;
pub trait VkResultHandler
{
	fn into_result(self) -> Result<()>;
}
impl VkResultHandler for VkResult
{
	fn into_result(self) -> Result<()> { if self == VK_SUCCESS { Ok(()) } else { Err(self) } }
}

/// Construction from Pointer that is not checked
pub trait DeviceChild<P>
{
    /// Construct a object from unchecked handle pointer
    /// # Safety
    /// Caller and callee do not guarantee that the passed pointer is valid 
    unsafe fn from_unchecked(p: P, parent: &Device) -> Self;
}
#[cfg(feature = "FeImplements")]
macro_rules! DeviceChildCommonDrop
{
	{ for $($t: ty [$d: expr]),* } =>
	{
		$(
			impl Drop for $t { fn drop(&mut self) { unsafe { $d(self.1.native_ptr(), self.0, ::std::ptr::null()) }; } }
		)*
	}
}

/// size elements
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct Extent1D(pub u32);
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extent2D(pub u32, pub u32);
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extent3D(pub u32, pub u32, pub u32);
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct Offset1D(pub i32);
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset2D(pub i32, pub i32);
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset3D(pub i32, pub i32, pub i32);
// into conversion to larger dimension //
impl Into<Extent2D> for Extent1D { fn into(self) -> Extent2D { Extent2D(self.0, 1) } }
impl Into<Extent3D> for Extent1D { fn into(self) -> Extent3D { Extent3D(self.0, 1, 1) } }
impl Into<Extent3D> for Extent2D { fn into(self) -> Extent3D { Extent3D(self.0, self.1, 1) } }
impl Into<Offset2D> for Offset1D { fn into(self) -> Offset2D { Offset2D(self.0, 0) } }
impl Into<Offset3D> for Offset1D { fn into(self) -> Offset3D { Offset3D(self.0, 0, 0) } }
impl Into<Offset3D> for Offset2D { fn into(self) -> Offset3D { Offset3D(self.0, self.1, 0) } }
// cheap conversion by transmuting //
impl AsRef<u32> for Extent1D { fn as_ref(&self) -> &u32 { &self.0 } }
impl AsRef<VkExtent2D> for Extent2D { fn as_ref(&self) -> &VkExtent2D { unsafe { std::mem::transmute(self) } } }
impl AsRef<VkExtent3D> for Extent3D { fn as_ref(&self) -> &VkExtent3D { unsafe { std::mem::transmute(self) } } }
impl AsRef<i32> for Offset1D { fn as_ref(&self) -> &i32 { &self.0 } }
impl AsRef<VkOffset2D> for Offset2D { fn as_ref(&self) -> &VkOffset2D { unsafe { std::mem::transmute(self) } } }
impl AsRef<VkOffset3D> for Offset3D { fn as_ref(&self) -> &VkOffset3D { unsafe { std::mem::transmute(self) } } }

mod base; pub use base::*;
mod device; pub use device::*;
mod sync; pub use sync::*;
mod resources; pub use resources::*;
