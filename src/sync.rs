//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

#![cfg_attr(not(feature = "FeImplements"), allow(dead_code))]

use vk::{VkFence, VkSemaphore, VkEvent};
#[cfg(feature = "FeImplements")] use VkResultHandler;

/// Opaque handle to a fence object
pub struct Fence(VkFence, ::Device);
/// Opaque handle to a semaphore object
pub struct Semaphore(VkSemaphore, ::Device);
/// Opaque handle to a event object
pub struct Event(VkEvent, ::Device);

impl ::DeviceChild<VkFence> for Fence { unsafe fn from_unchecked(p: VkFence, parent: &::Device) -> Self { Fence(p, parent.clone()) } }
impl ::DeviceChild<VkSemaphore> for Semaphore { unsafe fn from_unchecked(p: VkSemaphore, parent: &::Device) -> Self { Semaphore(p, parent.clone()) } }
impl ::DeviceChild<VkEvent> for Event { unsafe fn from_unchecked(p: VkEvent, parent: &::Device) -> Self { Event(p, parent.clone()) } }
#[cfg(feature = "FeImplements")]
impl Drop for Fence { fn drop(&mut self) { unsafe { ::vk::vkDestroyFence(self.1.native_ptr(), self.0, ::std::ptr::null()) }; } }
#[cfg(feature = "FeImplements")]
impl Drop for Semaphore { fn drop(&mut self) { unsafe { ::vk::vkDestroySemaphore(self.1.native_ptr(), self.0, ::std::ptr::null()) }; } }
#[cfg(feature = "FeImplements")]
impl Drop for Event { fn drop(&mut self) { unsafe { ::vk::vkDestroyEvent(self.1.native_ptr(), self.0, ::std::ptr::null()) }; } }

#[cfg(feature = "FeImplements")]
impl Fence
{
	/// Wait for one or more fences to become signaled, returns `Ok(true)` if operation is timed out
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_DEVICE_LOST
	pub fn wait_multiple(objects: &[&Self], wait_all: bool, timeout: Option<u64>) -> ::Result<bool>
	{
		let objects_ptr = objects.iter().map(|x| x.0).collect::<Vec<_>>();
		let vr = unsafe { ::vk::vkWaitForFences(objects[0].1.native_ptr(), objects_ptr.len() as _, objects_ptr.as_ptr(), wait_all as _, timeout.unwrap_or(::std::u64::MAX)) };
		match vr { ::vk::VK_SUCCESS => Ok(false), ::vk::VK_TIMEOUT => Ok(true), _ => Err(vr) }
	}
	/// Wait for a fence to become signaled, returns `Ok(true)` if operation is timed out
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_DEVICE_LOST
	fn wait_timeout(&self, timeout: u64) -> ::Result<bool>
	{
		let vr = unsafe { ::vk::vkWaitForFences(self.1.native_ptr(), 1, &self.0, false as _, timeout) };
		match vr { ::vk::VK_SUCCESS => Ok(false), ::vk::VK_TIMEOUT => Ok(true), _ => Err(vr) }
	}
	/// Resets one or more fence objects
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn reset_multiple(objects: &[&Self]) -> ::Result<()>
	{
		let objects_ptr = objects.iter().map(|x| x.0).collect::<Vec<_>>();
		unsafe { ::vk::vkResetFences(objects[0].1.native_ptr(), objects_ptr.len() as _, objects_ptr.as_ptr()) }.into_result()
	}
	/// Resets a fence object
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn reset(&self) -> ::Result<()> { unsafe { ::vk::vkResetFences(self.1.native_ptr(), 1, &self.0) }.into_result() }
}
#[cfg(feature = "FeImplements")]
impl Event
{
	/// Set an event to signaled state
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn set(&self) -> ::Result<()> { unsafe { ::vk::vkSetEvent(self.1.native_ptr(), self.0) }.into_result() }
	/// Reset an event to non-signaled state
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn reset(&self) -> ::Result<()> { unsafe { ::vk::vkResetEvent(self.1.native_ptr(), self.0) }.into_result() }
}

pub trait Status
{
	/// Retrieve the status(which is signaled) of a synchronize object
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_DEVICE_LOST
	fn status(&self) -> ::Result<bool>;
}
#[cfg(feature = "FeImplements")]
impl Status for Fence
{
	fn status(&self) -> ::Result<bool>
	{
		let vr = unsafe { ::vk::vkGetFenceStatus(self.1.native_ptr(), self.0) };
		match vr { ::vk::VK_SUCCESS => Ok(true), ::vk::VK_NOT_READY => Ok(false), _ => Err(vr) }
	}
}
#[cfg(feature = "FeImplements")]
impl Status for Event
{
	fn status(&self) -> ::Result<bool>
	{
		let vr = unsafe { ::vk::vkGetEventStatus(self.1.native_ptr(), self.0) };
		match vr { ::vk::VK_EVENT_SET => Ok(true), ::vk::VK_EVENT_RESET => Ok(false), _ => Err(vr) }
	}
}
#[cfg(feature = "FeImplements")]
impl ::Waitable for Fence
{
	/// Wait for a fence to become signaled
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_DEVICE_LOST
	fn wait(&self) -> ::Result<()> { self.wait_timeout(::std::u64::MAX).map(|_| ()) }
}
