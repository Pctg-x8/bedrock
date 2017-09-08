
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

#[cfg(feature = "FeImplements")]
mod fnconv;
mod base;
pub use base::*;

pub type Result<T> = std::result::Result<T, VkResult>;
pub trait VkResultHandler
{
	fn into_result(self) -> Result<()>;
}
impl VkResultHandler for VkResult
{
	fn into_result(self) -> Result<()> { if self == VK_SUCCESS { Ok(()) } else { Err(self) } }
}

/// Set of bit of queue flags
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct QueueFlags(VkQueueFlags);
impl QueueFlags
{
	/// Empty Bits
	pub fn new() -> Self { QueueFlags(0) }
	/// Supports only graphics operations
	pub fn graphics1() -> Self { QueueFlags(VK_QUEUE_GRAPHICS_BIT) }
	/// Supports only compute operations
	pub fn compute1() -> Self { QueueFlags(VK_QUEUE_COMPUTE_BIT) }
	/// Supports only transfer operations
	pub fn transfer1() -> Self { QueueFlags(VK_QUEUE_TRANSFER_BIT) }
	/// Supports only sparse memory management operations
	pub fn sparse_binding1() -> Self { QueueFlags(VK_QUEUE_SPARSE_BINDING_BIT) }
	/// Supports graphics operations
	pub fn graphics(self) -> Self { QueueFlags(self.0 | VK_QUEUE_GRAPHICS_BIT) }
	/// Supports compute operations
	pub fn compute(self) -> Self { QueueFlags(self.0 | VK_QUEUE_COMPUTE_BIT) }
	/// Supports transfer operations
	pub fn transfer(self) -> Self { QueueFlags(self.0 | VK_QUEUE_TRANSFER_BIT) }
	/// Supports sparse memory management operatinons
	pub fn sparse_binding(self) -> Self { QueueFlags(self.0 | VK_QUEUE_SPARSE_BINDING_BIT) }
}
/// List of queue families
pub struct QueueFamilies(Vec<VkQueueFamilyProperties>);
impl QueueFamilies
{
	/// Find a queue family index containing specified bitflags
	#[allow(non_snake_case)]
	pub fn find_matching_index(&self, flags: QueueFlags) -> Option<u32>
	{
		self.0.iter().enumerate().find(|&(_, &VkQueueFamilyProperties { queueFlags, .. })| (queueFlags & flags.0) != 0).map(|(n, _)| n as _)
	}
	/// Number of queue families
	pub fn count(&self) -> u32 { self.0.len() as _ }
	/// Number of queues in selected queue family
	pub fn queue_count(&self, family_index: u32) -> u32 { self.0[family_index as usize].queueCount }
	/// Unsigned integer count of meaningful bits in the timestamps written via `vkCmdWriteTimestamp`
	pub fn timestamp_valid_bits(&self, family_index: u32) -> u32 { self.0[family_index as usize].timestampValidBits }
	/// Minimum granularity supported for image transfer operations on the queues in selected queue family
	pub fn minimum_image_transfer_granularity(&self, family_index: u32) -> &VkExtent3D { &self.0[family_index as usize].minImageTransferGranularity }
}
