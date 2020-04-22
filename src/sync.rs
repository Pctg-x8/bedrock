//! Vulkan Synchronization Primitives(Fence, Semaphore, Event)

#![cfg_attr(not(feature = "Implements"), allow(dead_code))]

use crate::vk::*;
use crate::{VkHandle, DeviceChild, Device};
#[cfg(feature = "Implements")] use crate::{
	VkResultHandler, VkResultBox, Waitable,
	vkresolve::{Resolver, ResolverInterface}
};

/// Opaque handle to a fence object
pub struct Fence(VkFence, Device);
/// Opaque handle to a semaphore object
pub struct Semaphore(VkSemaphore, Device);
/// Opaque handle to a event object
pub struct Event(VkEvent, Device);

#[cfg(feature = "Implements")] DeviceChildCommonDrop!{
	for Fence[destroy_fence], Semaphore[destroy_semaphore], Event[destroy_event]
}
impl VkHandle for Fence { type Handle = VkFence; fn native_ptr(&self) -> VkFence { self.0 } }
impl VkHandle for Semaphore { type Handle = VkSemaphore; fn native_ptr(&self) -> VkSemaphore { self.0 } }
impl VkHandle for Event { type Handle = VkEvent; fn native_ptr(&self) -> VkEvent { self.0 } }
impl DeviceChild for Fence { fn device(&self) -> &Device { &self.1 } }
impl DeviceChild for Semaphore { fn device(&self) -> &Device { &self.1 } }
impl DeviceChild for Event { fn device(&self) -> &Device { &self.1 } }

/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl Fence
{
	/// Create a new fence object
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn new(device: &Device, signaled: bool) -> crate::Result<Self>
	{
		let mut h = VK_NULL_HANDLE as _;
		let flags = if signaled { VK_FENCE_CREATE_SIGNALED_BIT } else { 0 };
		unsafe
		{
			Resolver::get()
				.create_fence(device.native_ptr(), &VkFenceCreateInfo { flags, .. Default::default() }, std::ptr::null(), &mut h)
				.into_result().map(|_| Fence(h, device.clone()))
		}
	}
}
/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl Semaphore
{
	/// Create a new queue semaphore object
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn new(device: &Device) -> crate::Result<Self>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe
		{
			Resolver::get()
				.create_semaphore(device.native_ptr(), &Default::default(), std::ptr::null(), &mut h)
				.into_result().map(|_| Semaphore(h, device.clone()))
		}
	}
}
/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl Event
{
	/// Create a new event object
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn new(device: &Device) -> crate::Result<Self>
	{
		let mut h = VK_NULL_HANDLE as _;
		unsafe
		{
			Resolver::get()
				.create_event(device.native_ptr(), &Default::default(), std::ptr::null(), &mut h)
				.into_result().map(|_| Event(h, device.clone()))
		}
	}
}

/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl Fence
{
	/// Wait for one or more fences to become signaled, returns `Ok(true)` if operation is timed out
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_DEVICE_LOST`
	pub fn wait_multiple(objects: &[&Self], wait_all: bool, timeout: Option<u64>) -> crate::Result<bool>
	{
		let objects_ptr = objects.iter().map(|x| x.0).collect::<Vec<_>>();
		let vr = unsafe
		{
			Resolver::get()
				.wait_for_fences(
					objects[0].1.native_ptr(),
					objects_ptr.len() as _, objects_ptr.as_ptr(), wait_all as _,
					timeout.unwrap_or(std::u64::MAX)
				)
		};
		match vr { VK_SUCCESS => Ok(false), VK_TIMEOUT => Ok(true), _ => Err(VkResultBox(vr)) }
	}
	/// Wait for a fence to become signaled, returns `Ok(true)` if operation is timed out
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_DEVICE_LOST`
	pub fn wait_timeout(&self, timeout: u64) -> crate::Result<bool>
	{
		let vr = unsafe
		{
			Resolver::get()
				.wait_for_fences(self.1.native_ptr(), 1, &self.0, false as _, timeout)
		};
		match vr { VK_SUCCESS => Ok(false), VK_TIMEOUT => Ok(true), _ => Err(VkResultBox(vr)) }
	}
	/// Resets one or more fence objects
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn reset_multiple(objects: &[&Self]) -> crate::Result<()>
	{
		let objects_ptr = objects.iter().map(|x| x.0).collect::<Vec<_>>();
		unsafe
		{
			Resolver::get()
				.reset_fences(objects[0].1.native_ptr(), objects_ptr.len() as _, objects_ptr.as_ptr())
				.into_result()
		}
	}
	/// Resets a fence object
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn reset(&self) -> crate::Result<()>
	{
		unsafe
		{
			Resolver::get()
				.reset_fences(self.1.native_ptr(), 1, &self.0)
				.into_result()
		}
	}
}
/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl Event
{
	/// Set an event to signaled state
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn set(&self) -> crate::Result<()>
	{
		unsafe { Resolver::get().set_event(self.1.native_ptr(), self.0).into_result() }
	}
	/// Reset an event to non-signaled state
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	pub fn reset(&self) -> crate::Result<()>
	{
		unsafe { Resolver::get().reset_event(self.1.native_ptr(), self.0).into_result() }
	}
}

#[cfg(feature = "Implements")]
pub trait Status
{
	/// [feature = "Implements"] Retrieve the status(whether is signaled or not) of a synchronize object
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_DEVICE_LOST`
	fn status(&self) -> crate::Result<bool>;
}
#[cfg(feature = "Implements")]
impl Status for Fence
{
	fn status(&self) -> crate::Result<bool>
	{
		let vr = unsafe { Resolver::get().get_fence_status(self.1.native_ptr(), self.0) };
		match vr { VK_SUCCESS => Ok(true), VK_NOT_READY => Ok(false), _ => Err(VkResultBox(vr)) }
	}
}
#[cfg(feature = "Implements")]
impl Status for Event
{
	fn status(&self) -> crate::Result<bool>
	{
		let vr = unsafe { Resolver::get().get_event_status(self.1.native_ptr(), self.0) };
		match vr { VK_EVENT_SET => Ok(true), VK_EVENT_RESET => Ok(false), _ => Err(VkResultBox(vr)) }
	}
}
#[cfg(feature = "Implements")]
impl Waitable for Fence
{
	/// [feature = "Implements"] Wait for a fence to become signaled
	/// # Failures
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_DEVICE_LOST`
	fn wait(&self) -> crate::Result<()> { self.wait_timeout(std::u64::MAX).map(drop) }
}

unsafe impl Send for Fence {}
unsafe impl Sync for Fence {}
