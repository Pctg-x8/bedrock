//! Direct Display Rendering

use crate::vk::*;
use derives::*;
use std::ops::Deref;
#[cfg(feature = "Implements")] use crate::{Resolver, ResolverInterface, VkResultHandler};

#[cfg(feature = "VK_KHR_display")]
#[repr(transparent)]
#[derive(VkHandle)]
pub struct Display(VkDisplayKHR);
#[cfg(feature = "VK_KHR_display")]
#[repr(transparent)]
#[derive(VkHandle)]
pub struct DisplayMode(VkDisplayModeKHR);

#[cfg(all(feature = "VK_KHR_display", feature = "Implements"))]
impl crate::PhysicalDevice {
	/// [Implements][VK_KHR_display] Query information about the available displays.
	/// # Failures
	/// On failure, this command returns
	///
	/// * VK_ERROR_OUT_OF_HOST_MEMORY
	/// * VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn display_properties(&self) -> crate::Result<Vec<DisplayProperties>> {
		unsafe {
			let mut n = 0;
			Resolver::get().get_physical_device_display_properties_khr(self.0, &mut n, std::ptr::null_mut())
				.into_result()?;
			let mut v = Vec::with_capacity(n as usize); v.set_len(n as usize);
			Resolver::get().get_physical_device_display_properties_khr(self.0, &mut n, v.as_mut_ptr() as *mut _)
				.into_result()
				.map(move |_| v)
		}
	}

	/// [Implements][VK_KHR_display] Query the plane properties.
	/// # Failures
	/// On failure, this command returns
	///
	/// * VK_ERROR_OUT_OF_HOST_MEMORY
	/// * VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn display_plane_properties(&self) -> crate::Result<Vec<DisplayPlaneProperties>> {
		unsafe {
			let mut n = 0;
			Resolver::get().get_physical_device_display_plane_properties_khr(self.0, &mut n, std::ptr::null_mut())
				.into_result()?;
			let mut v = Vec::with_capacity(n as usize); v.set_len(n as usize);
			Resolver::get().get_physical_device_display_plane_properties_khr(self.0, &mut n, v.as_mut_ptr() as *mut _)
				.into_result()
				.map(move |_| v)
		}
	}

	/// [Implements][VK_KHR_display] Query the list of displays a plane supports.
	/// # Failures
	/// On failure, this command returns
	///
	/// * VK_ERROR_OUT_OF_HOST_MEMORY
	/// * VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn display_plane_supported_displays(&self, plane_index: u32) -> crate::Result<Vec<Display>> {
		unsafe {
			let mut n = 0;
			Resolver::get().get_display_plane_supported_displays_khr(
				self.0, plane_index, &mut n, std::ptr::null_mut()
			).into_result()?;
			let mut v = Vec::with_capacity(n as usize); v.set_len(n as usize);
			Resolver::get().get_display_plane_supported_displays_khr(
				self.0, plane_index, &mut n, v.as_mut_ptr() as *mut _
			).into_result().map(move |_| v)
		}
	}
}

#[repr(transparent)]
pub struct DisplayProperties(VkDisplayPropertiesKHR);
impl From<VkDisplayPropertiesKHR> for DisplayProperties { fn from(v: VkDisplayPropertiesKHR) -> Self { Self(v) } }
impl From<DisplayProperties> for VkDisplayPropertiesKHR { fn from(v: DisplayProperties) -> Self { v.0 } }
impl Deref for DisplayProperties {
	type Target = VkDisplayPropertiesKHR;
	fn deref(&self) -> &VkDisplayPropertiesKHR { &self.0 }
}
impl AsRef<VkDisplayPropertiesKHR> for DisplayProperties {
	fn as_ref(&self) -> &VkDisplayPropertiesKHR { &self.0 }
}
impl DisplayProperties {
	/// The name of the display.
	pub fn display_name(&self) -> &std::ffi::CStr {
		unsafe { std::ffi::CStr::from_ptr(self.displayName) }
	}
	/// Whether the planes on this display can have their z order changed.
	pub fn can_reorder_plane(&self) -> bool { self.planeReorderPossible == VK_TRUE }
	/// Whether the display supports self-refresh/internal buffering.
	pub fn has_persistent_content(&self) -> bool { self.persistentContent == VK_TRUE }
}

#[repr(transparent)]
pub struct DisplayPlaneProperties(VkDisplayPlanePropertiesKHR);
impl From<VkDisplayPlanePropertiesKHR> for DisplayPlaneProperties {
	fn from(v: VkDisplayPlanePropertiesKHR) -> Self { Self(v) }
}
impl From<DisplayPlaneProperties> for VkDisplayPlanePropertiesKHR {
	fn from(v: DisplayPlaneProperties) -> Self { v.0 }
}
impl Deref for DisplayPlaneProperties {
	type Target = VkDisplayPlanePropertiesKHR;
	fn deref(&self) -> &VkDisplayPlanePropertiesKHR { &self.0 }
}
impl AsRef<VkDisplayPlanePropertiesKHR> for DisplayPlaneProperties {
	fn as_ref(&self) -> &VkDisplayPlanePropertiesKHR { &self.0 }
}
impl DisplayPlaneProperties {

}