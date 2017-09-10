//! Vulkan Device and Queues

#![cfg_attr(not(feature = "FeImplements"), allow(dead_code))]

use vk::*;
use PhysicalDevice;
use std::ffi::CString;
use std::borrow::Cow;
#[cfg(    feature = "FeMultithreaded") ] use std::sync::Arc as RefCounter;
#[cfg(not(feature = "FeMultithreaded"))] use std::rc::Rc as RefCounter;
#[cfg(feature = "FeImplements")] use {DeviceChild, VkResultHandler};

/// Set of bit of queue flags
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct QueueFlags(pub VkQueueFlags);
impl QueueFlags
{
	/// Empty bits
	pub const EMPTY: Self = QueueFlags(0);
	/// Supports only graphics operations
	pub const GRAPHICS: Self = QueueFlags(VK_QUEUE_GRAPHICS_BIT);
	/// Supports only compute operations
	pub const COMPUTE: Self = QueueFlags(VK_QUEUE_COMPUTE_BIT);
	/// Supports only transfer operations
	pub const TRANSFER: Self = QueueFlags(VK_QUEUE_TRANSFER_BIT);
	/// Supports only sparse memory management operations
	pub const SPARSE_BINDING: Self = QueueFlags(VK_QUEUE_SPARSE_BINDING_BIT);
	/// Supports graphics operations
	pub fn graphics(self) -> Self { QueueFlags(self.0 | Self::GRAPHICS.0) }
	/// Supports compute operations
	pub fn compute(self) -> Self { QueueFlags(self.0 | Self::COMPUTE.0) }
	/// Supports transfer operations
	pub fn transfer(self) -> Self { QueueFlags(self.0 | Self::TRANSFER.0) }
	/// Supports sparse memory management operatinons
	pub fn sparse_binding(self) -> Self { QueueFlags(self.0 | Self::SPARSE_BINDING.0) }
}
/// List of queue families
pub struct QueueFamilies(pub Vec<VkQueueFamilyProperties>);
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

struct DeviceCell(VkDevice);
/// Opaque handle to a device object
#[derive(Clone)]
pub struct Device(RefCounter<DeviceCell>);
#[cfg(feature = "FeMultithreaded")] unsafe impl Sync for Device {}
/// Opaque handle to a queue object
pub struct Queue(VkQueue, Device);
/// Family Index, Queue Priorities
pub struct DeviceQueueCreateInfo(pub u32, pub Vec<f32>);
/// Builder object for constructing a `Device`
pub struct DeviceBuilder<'p>
{
	pdev_ref: &'p PhysicalDevice, queue_infos: Vec<DeviceQueueCreateInfo>,
	layers: Vec<CString>, extensions: Vec<CString>, features: VkPhysicalDeviceFeatures
}
impl<'p> DeviceBuilder<'p>
{
	pub fn new(pdev: &'p PhysicalDevice) -> Self
	{
		DeviceBuilder { pdev_ref: pdev, queue_infos: Vec::new(), layers: Vec::new(), extensions: Vec::new(), features: VkPhysicalDeviceFeatures::default() }
	}
	pub fn add_layer(mut self, name: &str) -> Self { self.layers.push(CString::new(name).unwrap()); self }
	pub fn add_extension(mut self, name: &str) -> Self { self.extensions.push(CString::new(name).unwrap()); self }
	pub fn add_layers<'s, Layers: IntoIterator<Item = &'s str>>(mut self, layers: Layers) -> Self
	{
		for l in layers { self = self.add_layer(l); } self
	}
	pub fn add_extensions<'s, Extensions: IntoIterator<Item = &'s str>>(mut self, extensions: Extensions) -> Self
	{
		for e in extensions { self = self.add_extension(e); } self
	}
	pub fn add_queue(mut self, info: DeviceQueueCreateInfo) -> Self { self.queue_infos.push(info); self }
	pub fn add_queues<Queues: IntoIterator<Item = DeviceQueueCreateInfo>>(mut self, queues: Queues) -> Self
	{
		for q in queues { self = self.add_queue(q); } self
	}
	#[cfg(feature = "FeImplements")]
	pub fn create(self) -> ::Result<Device>
	{
		let qinfos = self.queue_infos.iter().map(|&DeviceQueueCreateInfo(fi, ref ps)| ::vk::VkDeviceQueueCreateInfo
		{
			queueFamilyIndex: fi, queueCount: ps.len() as _, pQueuePriorities: ps.as_ptr(), .. Default::default()
		}).collect::<Vec<_>>();
		let layers = self.layers.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
		let extensions = self.extensions.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
		let cinfo = ::vk::VkDeviceCreateInfo
		{
			queueCreateInfoCount: qinfos.len() as _, pQueueCreateInfos: qinfos.as_ptr(),
			enabledLayerCount: layers.len() as _, ppEnabledLayerNames: layers.as_ptr(),
			enabledExtensionCount: extensions.len() as _, ppEnabledExtensionNames: extensions.as_ptr(),
			pEnabledFeatures: &self.features, .. Default::default()
		};
		let mut h = unsafe { ::std::mem::zeroed() };
		unsafe { vkCreateDevice(::std::mem::transmute(self.pdev_ref), &cinfo, ::std::ptr::null(), &mut h) }.into_result()
			.map(|_| Device(RefCounter::new(DeviceCell(h))))
	}
}
#[cfg(feature = "FeImplements")]
impl Drop for DeviceCell { fn drop(&mut self) { unsafe { ::vk::vkDestroyDevice(self.0, ::std::ptr::null()) }; } }
#[cfg(feature = "FeImplements")]
impl Device
{
	pub fn native_ptr(&self) -> VkDevice { (self.0).0 }
	/// Return a function pointer for a command
	/// # Failures
	/// If function is not provided by instance or `name` is empty, returns `None`
	pub fn extra_procedure<F: ::fnconv::FnTransmute>(&self, name: &str) -> Option<F>
	{
		if name.is_empty() { None }
		else
		{
			let p = unsafe { vkGetDeviceProcAddr(self.native_ptr(), CString::new(name).unwrap().as_ptr()) };
			if unsafe { ::std::mem::transmute::<_, usize>(p) == 0 } { None } else { unsafe { Some(::fnconv::FnTransmute::from_fn(p)) } }
		}
	}
	/// Get a queue handle from a device
	pub fn queue(&self, family_index: u32, queue_index: u32) -> Queue
	{
		let mut h = unsafe { ::std::mem::zeroed() };
		unsafe { vkGetDeviceQueue(self.native_ptr(), family_index, queue_index, &mut h) }
		Queue(h, self.clone())
	}
	/// Flush `MappedMemoryRange`s
	/// Flushing the memory range allows that host writes to the memory ranges can
	/// be made available to device access
	pub fn flush_memory_range(&self, ranges: &[::MappedMemoryRanges]) -> ::Result<()>
	{
		let mut v = ranges.into_iter().map(|r| r.manual_flush()).collect::<Vec<_>>();
		unsafe { vkFlushMappedMemoryRanges(self.native_ptr(), v.len() as _, v.as_ptr()) }.into_result()
	}
	/// Invalidate `MappedMemoryRange`s
	/// Invalidating the memory range allows that device writes to the memory ranges
	/// which have been made visible to the `VK_ACCESS_HOST_WRITE_BIT` and `VK_ACCESS_HOST_READ_BIT`
	/// are made visible to the host
	pub fn invalidate_memory_range(&self, ranges: &[::MappedMemoryRanges]) -> ::Result<()>
	{
		let mut v = ranges.into_iter().map(|r| r.manual_flush()).collect::<Vec<_>>();
		unsafe { vkInvalidateMappedMemoryRanges(self.native_ptr(), v.len() as _, v.as_ptr()) }.into_result()
	}
}

/// Supports blocking wait operation
pub trait Waitable
{
	/// Wait for a object to become idle
	fn wait(&self) -> ::Result<()>;
}
#[cfg(feature = "FeImplements")]
impl Waitable for Device { fn wait(&self) -> ::Result<()> { unsafe { ::vk::vkDeviceWaitIdle(self.native_ptr()) }.into_result() } }
#[cfg(feature = "FeImplements")]
impl Waitable for Queue { fn wait(&self) -> ::Result<()> { unsafe { ::vk::vkQueueWaitIdle(self.0) }.into_result() } }

/// Sparse Binding operation batch
pub struct SparseBindingOpBatch<'s>
{
	/// An array of semaphores upon which to wait on before the sparse binding operations
	/// for this batch begin execution
	pub wait_semaphores: Cow<'s, [&'s ::Semaphore]>,
	/// An array of `VkSparseBufferMemoryBindInfo` structures
	pub buffer_binds: Cow<'s, [VkSparseBufferMemoryBindInfo]>,
	/// An array of `VkSparseImageOpaqueMemoryBindInfo` structures
	pub image_opaque_binds: Cow<'s, [VkSparseImageOpaqueMemoryBindInfo]>,
	/// An array of `VkSparseImageMemoryBindInfo` structures
	pub image_binds: Cow<'s, [VkSparseImageMemoryBindInfo]>,
	/// An array of semaphores which will be signaled when the sparse binding
	/// operations for this batch have completed execution
	pub signal_semaphores: Cow<'s, [&'s ::Semaphore]>
}
impl<'s> Default for SparseBindingOpBatch<'s>
{
	fn default() -> Self
	{
		SparseBindingOpBatch
		{
			wait_semaphores: Cow::Owned(Vec::new()),
			buffer_binds: Cow::Owned(Vec::new()), image_opaque_binds: Cow::Owned(Vec::new()), image_binds: Cow::Owned(Vec::new()),
			signal_semaphores: Cow::Owned(Vec::new())
		}
	}
}
#[cfg(feature = "FeImplements")]
impl Queue
{
	/// Bind device memory to a sparse resource object
	/// # Failure
	/// On failure, this command returns
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_DEVICE_LOST
	pub fn bind_sparse(&self, batches: &[SparseBindingOpBatch], fence: Option<&::Fence>) -> ::Result<()>
	{
		let batches = batches.iter().map(|x| VkBindSparseInfo
		{
			waitSemaphoreCount: x.wait_semaphores.len() as _, pWaitSemaphores: x.wait_semaphores.as_ptr(),
			bufferBindCount: x.buffer_binds.len() as _, pBufferBinds: x.buffer_binds.as_ptr(),
			imageOpaqueBindCount: x.image_opaque_binds.len() as _, pImageOpaqueBints: x.image_opaque_binds.as_ptr(),
			imageBindCount: x.image_binds.len() as _, pImageBinds: x.image_binds.as_ptr(),
			signalSemaphoreCount: x.signal_semaphores.len() as _, pSignalSemaphores: x.signal_semaphores.as_ptr(),
			.. Default::default()
		}).collect::<Vec<_>>();
		unsafe { vkQueueBindSparse(self.0, batches.len() as _, batches.as_ptr(), fence.map(|x| x.0).unwrap_or(VK_NULL_HANDLE as _)) }
			.into_result()
	}
}
