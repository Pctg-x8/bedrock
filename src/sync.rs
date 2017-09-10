//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

#![cfg_attr(not(feature = "FeImplements"), allow(dead_code))]

use vk::*;
#[cfg(feature = "FeImplements")] use VkResultHandler;

/// Opaque handle to a fence object
pub struct Fence(pub VkFence, ::Device);
/// Opaque handle to a semaphore object
pub struct Semaphore(pub VkSemaphore, ::Device);
/// Opaque handle to a event object
pub struct Event(pub VkEvent, ::Device);

#[cfg(feature = "FeImplements")]
impl Fence
{
	/// Create a new fence object
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn new(device: &::Device, signaled: bool) -> ::Result<Self>
	{
		let mut h = VK_NULL_HANDLE as _;
		let flags = if signaled { ::vk::VK_FENCE_CREATE_SIGNALED_BIT } else { 0 };
		unsafe { vkCreateFence(device.native_ptr(), &VkFenceCreateInfo { flags, .. Default::default() }, ::std::ptr::null(), &mut h) }
			.into_result().map(|_| Fence(h, device.clone()))
	}
}
#[cfg(feature = "FeImplements")]
impl Semaphore
{
	/// Create a new queue semaphore object
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn new(device: &::Device) -> ::Result<Self>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateSemaphore(device.native_ptr(), &Default::default(), ::std::ptr::null(), &mut h) }
			.into_result().map(|_| Semaphore(h, device.clone()))
	}
}
#[cfg(feature = "FeImplements")]
impl Event
{
	/// Create a new event object
	/// # Failures
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	pub fn new(device: &::Device) -> ::Result<Self>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe { vkCreateEvent(device.native_ptr(), &Default::default(), ::std::ptr::null(), &mut h) }
			.into_result().map(|_| Event(h, device.clone()))
	}
}

#[cfg(feature = "FeImplements")] DeviceChildCommonDrop!{
	for Fence[vkDestroyFence], Semaphore[vkDestroySemaphore], Event[vkDestroyEvent]
}

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
	pub fn wait_timeout(&self, timeout: u64) -> ::Result<bool>
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
