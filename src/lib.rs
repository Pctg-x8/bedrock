
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

mod base; pub use base::*;
mod device; pub use device::*;
mod sync; pub use sync::*;
mod resources; pub use resources::*;
