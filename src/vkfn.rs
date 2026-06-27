use crate::vk::*;
use core::ffi::*;

#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_instance(create_info: *const VkInstanceCreateInfo, allocator: *const VkAllocationCallbacks, instance_out: *mut VkInstance) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_instance.0)(create_info, allocator, instance_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateInstance(create_info, allocator, instance_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_instance(instance: VkInstance, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_instance.0)(instance, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyInstance(instance, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn enumerate_physical_devices(instance: VkInstance, physical_devices_count_out: *mut u32, physical_devices_out: *mut VkPhysicalDevice) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.enumerate_physical_devices.0)(instance, physical_devices_count_out, physical_devices_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkEnumeratePhysicalDevices(instance, physical_devices_count_out, physical_devices_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_features(physical_device: VkPhysicalDevice, features_out: *mut VkPhysicalDeviceFeatures) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_features.0)(physical_device, features_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceFeatures(physical_device, features_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, format_properties_out: *mut VkFormatProperties) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_format_properties.0)(physical_device, format, format_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceFormatProperties(physical_device, format, format_properties_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_image_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, image_type: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, image_format_properties_out: *mut VkImageFormatProperties) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_image_format_properties.0)(physical_device, format, image_type, tiling, usage, flags, image_format_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceImageFormatProperties(physical_device, format, image_type, tiling, usage, flags, image_format_properties_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_properties(physical_device: VkPhysicalDevice, properties_out: *mut VkPhysicalDeviceProperties) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_properties.0)(physical_device, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceProperties(physical_device, properties_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_queue_family_properties(physical_device: VkPhysicalDevice, queue_family_properties_count_out: *mut u32, queue_family_properties_out: *mut VkQueueFamilyProperties) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_queue_family_properties.0)(physical_device, queue_family_properties_count_out, queue_family_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceQueueFamilyProperties(physical_device, queue_family_properties_count_out, queue_family_properties_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_memory_properties(physical_device: VkPhysicalDevice, memory_properties_out: *mut VkPhysicalDeviceMemoryProperties) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_memory_properties.0)(physical_device, memory_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceMemoryProperties(physical_device, memory_properties_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_instance_proc_addr(instance: VkInstance, name: *const c_char) -> Option<PFN_vkVoidFunction> {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_instance_proc_addr.0)(instance, name) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetInstanceProcAddr(instance, name) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_device_proc_addr(device: VkDevice, name: *const c_char) -> Option<PFN_vkVoidFunction> {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_device_proc_addr.0)(device, name) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetDeviceProcAddr(device, name) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_device(physical_device: VkPhysicalDevice, create_info: *const VkDeviceCreateInfo, allocator: *const VkAllocationCallbacks, device_out: *mut VkDevice) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_device.0)(physical_device, create_info, allocator, device_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateDevice(physical_device, create_info, allocator, device_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_device(device: VkDevice, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_device.0)(device, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyDevice(device, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn enumerate_instance_extension_properties(layer_name: *const c_char, property_count_out: *mut u32, properties_out: *mut VkExtensionProperties) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.enumerate_instance_extension_properties.0)(layer_name, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkEnumerateInstanceExtensionProperties(layer_name, property_count_out, properties_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn enumerate_device_extension_properties(physical_device: VkPhysicalDevice, layer_name: *const c_char, property_count_out: *mut u32, properties_out: *mut VkExtensionProperties) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.enumerate_device_extension_properties.0)(physical_device, layer_name, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkEnumerateDeviceExtensionProperties(physical_device, layer_name, property_count_out, properties_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn enumerate_instance_layer_properties(property_count_out: *mut u32, properties_out: *mut VkLayerProperties) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.enumerate_instance_layer_properties.0)(property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkEnumerateInstanceLayerProperties(property_count_out, properties_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn enumerate_device_layer_properties(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkLayerProperties) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.enumerate_device_layer_properties.0)(physical_device, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkEnumerateDeviceLayerProperties(physical_device, property_count_out, properties_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_device_queue(device: VkDevice, queue_family_index: u32, queue_index: u32, queue_out: *mut VkQueue) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_device_queue.0)(device, queue_family_index, queue_index, queue_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetDeviceQueue(device, queue_family_index, queue_index, queue_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn queue_submit(queue: VkQueue, submit_count: u32, submits: *const VkSubmitInfo, fence: Option<VkFence>) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.queue_submit.0)(queue, submit_count, submits, fence) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkQueueSubmit(queue, submit_count, submits, fence) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn queue_wait_idle(queue: VkQueue) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.queue_wait_idle.0)(queue) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkQueueWaitIdle(queue) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn device_wait_idle(device: VkDevice) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.device_wait_idle.0)(device) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDeviceWaitIdle(device) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn allocate_memory(device: VkDevice, allocate_info: *const VkMemoryAllocateInfo, allocator: *const VkAllocationCallbacks, memory_out: *mut VkDeviceMemory) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.allocate_memory.0)(device, allocate_info, allocator, memory_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkAllocateMemory(device, allocate_info, allocator, memory_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn free_memory(device: VkDevice, memory: VkDeviceMemory, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.free_memory.0)(device, memory, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkFreeMemory(device, memory, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn map_memory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, data_ptr_out: *mut *mut c_void) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.map_memory.0)(device, memory, offset, size, flags, data_ptr_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkMapMemory(device, memory, offset, size, flags, data_ptr_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn unmap_memory(device: VkDevice, memory: VkDeviceMemory) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.unmap_memory.0)(device, memory) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkUnmapMemory(device, memory) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn flush_mapped_memory_ranges(device: VkDevice, memory_range_count: u32, memory_ranges: *const VkMappedMemoryRange) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.flush_mapped_memory_ranges.0)(device, memory_range_count, memory_ranges) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkFlushMappedMemoryRanges(device, memory_range_count, memory_ranges) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn invalidate_mapped_memory_ranges(device: VkDevice, memory_range_count: u32, memory_ranges: *const VkMappedMemoryRange) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.invalidate_mapped_memory_ranges.0)(device, memory_range_count, memory_ranges) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkInvalidateMappedMemoryRanges(device, memory_range_count, memory_ranges) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_device_memory_commitment(device: VkDevice, memory: VkDeviceMemory, committed_memory_bytes_out: *mut VkDeviceSize) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_device_memory_commitment.0)(device, memory, committed_memory_bytes_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetDeviceMemoryCommitment(device, memory, committed_memory_bytes_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn bind_buffer_memory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memory_offset: VkDeviceSize) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.bind_buffer_memory.0)(device, buffer, memory, memory_offset) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkBindBufferMemory(device, buffer, memory, memory_offset) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn bind_image_memory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memory_offset: VkDeviceSize) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.bind_image_memory.0)(device, image, memory, memory_offset) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkBindImageMemory(device, image, memory, memory_offset) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_buffer_memory_requirements(device: VkDevice, buffer: VkBuffer, memory_requirements_out: *mut VkMemoryRequirements) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_buffer_memory_requirements.0)(device, buffer, memory_requirements_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetBufferMemoryRequirements(device, buffer, memory_requirements_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_image_memory_requirements(device: VkDevice, image: VkImage, memory_requirements_out: *mut VkMemoryRequirements) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_image_memory_requirements.0)(device, image, memory_requirements_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetImageMemoryRequirements(device, image, memory_requirements_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_image_sparse_memory_requirements(device: VkDevice, image: VkImage, sparse_memory_requirement_count_out: *mut u32, sparse_memory_requirements_out: *mut VkSparseImageMemoryRequirements) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_image_sparse_memory_requirements.0)(device, image, sparse_memory_requirement_count_out, sparse_memory_requirements_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetImageSparseMemoryRequirements(device, image, sparse_memory_requirement_count_out, sparse_memory_requirements_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_sparse_image_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling, property_count_out: *mut u32, properties_out: *mut VkSparseImageFormatProperties) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_sparse_image_format_properties.0)(physical_device, format, r#type, samples, usage, tiling, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceSparseImageFormatProperties(physical_device, format, r#type, samples, usage, tiling, property_count_out, properties_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn queue_bind_sparse(queue: VkQueue, bind_info_count: u32, bind_info: *const VkBindSparseInfo, fence: Option<VkFence>) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.queue_bind_sparse.0)(queue, bind_info_count, bind_info, fence) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkQueueBindSparse(queue, bind_info_count, bind_info, fence) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_fence(device: VkDevice, create_info: *const VkFenceCreateInfo, allocator: *const VkAllocationCallbacks, fence_out: *mut VkFence) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_fence.0)(device, create_info, allocator, fence_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateFence(device, create_info, allocator, fence_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_fence(device: VkDevice, fence: VkFence, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_fence.0)(device, fence, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyFence(device, fence, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn reset_fences(device: VkDevice, fence_count: u32, fences: *const VkFence) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.reset_fences.0)(device, fence_count, fences) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkResetFences(device, fence_count, fences) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_fence_status(device: VkDevice, fence: VkFence) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_fence_status.0)(device, fence) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetFenceStatus(device, fence) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn wait_for_fences(device: VkDevice, fence_count: u32, fences: *const VkFence, wait_all: VkBool32, timeout: u64) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.wait_for_fences.0)(device, fence_count, fences, wait_all, timeout) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkWaitForFences(device, fence_count, fences, wait_all, timeout) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_semaphore(device: VkDevice, create_info: *const VkSemaphoreCreateInfo, allocator: *const VkAllocationCallbacks, semaphore_out: *mut VkSemaphore) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_semaphore.0)(device, create_info, allocator, semaphore_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateSemaphore(device, create_info, allocator, semaphore_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_semaphore(device: VkDevice, semaphore: VkSemaphore, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_semaphore.0)(device, semaphore, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroySemaphore(device, semaphore, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_event(device: VkDevice, create_info: *const VkEventCreateInfo, allocator: *const VkAllocationCallbacks, event_out: *mut VkEvent) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_event.0)(device, create_info, allocator, event_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateEvent(device, create_info, allocator, event_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_event(device: VkDevice, event: VkEvent, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_event.0)(device, event, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyEvent(device, event, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_event_status(device: VkDevice, event: VkEvent) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_event_status.0)(device, event) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetEventStatus(device, event) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn set_event(device: VkDevice, event: VkEvent) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.set_event.0)(device, event) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkSetEvent(device, event) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn reset_event(device: VkDevice, event: VkEvent) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.reset_event.0)(device, event) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkResetEvent(device, event) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_query_pool(device: VkDevice, create_info: *const VkQueryPoolCreateInfo, allocator: *const VkAllocationCallbacks, query_pool_out: *mut VkQueryPool) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_query_pool.0)(device, create_info, allocator, query_pool_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateQueryPool(device, create_info, allocator, query_pool_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_query_pool(device: VkDevice, query_pool: VkQueryPool, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_query_pool.0)(device, query_pool, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyQueryPool(device, query_pool, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_query_pool_results(device: VkDevice, query_pool: VkQueryPool, first_query: u32, query_count: u32, data_size: usize, data_out: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_query_pool_results.0)(device, query_pool, first_query, query_count, data_size, data_out, stride, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetQueryPoolResults(device, query_pool, first_query, query_count, data_size, data_out, stride, flags) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_buffer(device: VkDevice, create_info: *const VkBufferCreateInfo, allocator: *const VkAllocationCallbacks, buffer_out: *mut VkBuffer) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_buffer.0)(device, create_info, allocator, buffer_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateBuffer(device, create_info, allocator, buffer_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_buffer(device: VkDevice, buffer: VkBuffer, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_buffer.0)(device, buffer, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyBuffer(device, buffer, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_buffer_view(device: VkDevice, create_info: *const VkBufferViewCreateInfo, allocator: *const VkAllocationCallbacks, view_out: *mut VkBufferView) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_buffer_view.0)(device, create_info, allocator, view_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateBufferView(device, create_info, allocator, view_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_buffer_view(device: VkDevice, buffer_view: VkBufferView, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_buffer_view.0)(device, buffer_view, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyBufferView(device, buffer_view, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_image(device: VkDevice, create_info: *const VkImageCreateInfo, allocator: *const VkAllocationCallbacks, image_out: *mut VkImage) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_image.0)(device, create_info, allocator, image_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateImage(device, create_info, allocator, image_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_image(device: VkDevice, image: VkImage, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_image.0)(device, image, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyImage(device, image, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_image_subresource_layout(device: VkDevice, image: VkImage, subresource: *const VkImageSubresource, layout_out: *mut VkSubresourceLayout) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_image_subresource_layout.0)(device, image, subresource, layout_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetImageSubresourceLayout(device, image, subresource, layout_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_image_view(device: VkDevice, create_info: *const VkImageViewCreateInfo, allocator: *const VkAllocationCallbacks, view_out: *mut VkImageView) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_image_view.0)(device, create_info, allocator, view_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateImageView(device, create_info, allocator, view_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_image_view(device: VkDevice, image_view: VkImageView, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_image_view.0)(device, image_view, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyImageView(device, image_view, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_shader_module(device: VkDevice, create_info: *const VkShaderModuleCreateInfo, allocator: *const VkAllocationCallbacks, shader_module_out: *mut VkShaderModule) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_shader_module.0)(device, create_info, allocator, shader_module_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateShaderModule(device, create_info, allocator, shader_module_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_shader_module(device: VkDevice, shader_module: VkShaderModule, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_shader_module.0)(device, shader_module, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyShaderModule(device, shader_module, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_pipeline_cache(device: VkDevice, create_info: *const VkPipelineCacheCreateInfo, allocator: *const VkAllocationCallbacks, pipeline_cache_out: *mut VkPipelineCache) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_pipeline_cache.0)(device, create_info, allocator, pipeline_cache_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreatePipelineCache(device, create_info, allocator, pipeline_cache_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_pipeline_cache(device: VkDevice, pipeline_cache: VkPipelineCache, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_pipeline_cache.0)(device, pipeline_cache, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyPipelineCache(device, pipeline_cache, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_pipeline_cache_data(device: VkDevice, pipeline_cache: VkPipelineCache, data_size_out: *mut usize, data_out: *mut c_void) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_pipeline_cache_data.0)(device, pipeline_cache, data_size_out, data_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPipelineCacheData(device, pipeline_cache, data_size_out, data_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn merge_pipeline_caches(device: VkDevice, dst_cache: VkPipelineCache, src_cache_count: u32, src_caches: *const VkPipelineCache) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.merge_pipeline_caches.0)(device, dst_cache, src_cache_count, src_caches) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkMergePipelineCaches(device, dst_cache, src_cache_count, src_caches) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_graphics_pipelines(device: VkDevice, pipeline_cache: Option<VkPipelineCache>, create_info_count: u32, create_infos: *const VkGraphicsPipelineCreateInfo, allocator: *const VkAllocationCallbacks, pipelines_out: *mut VkPipeline) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_graphics_pipelines.0)(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateGraphicsPipelines(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_compute_pipelines(device: VkDevice, pipeline_cache: Option<VkPipelineCache>, create_info_count: u32, create_infos: *const VkComputePipelineCreateInfo, allocator: *const VkAllocationCallbacks, pipelines_out: *mut VkPipeline) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_compute_pipelines.0)(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateComputePipelines(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_pipeline(device: VkDevice, pipeline: VkPipeline, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_pipeline.0)(device, pipeline, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyPipeline(device, pipeline, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_pipeline_layout(device: VkDevice, create_info: *const VkPipelineLayoutCreateInfo, allocator: *const VkAllocationCallbacks, pipeline_layout_out: *mut VkPipelineLayout) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_pipeline_layout.0)(device, create_info, allocator, pipeline_layout_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreatePipelineLayout(device, create_info, allocator, pipeline_layout_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_pipeline_layout(device: VkDevice, pipeline_layout: VkPipelineLayout, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_pipeline_layout.0)(device, pipeline_layout, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyPipelineLayout(device, pipeline_layout, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_sampler(device: VkDevice, create_info: *const VkSamplerCreateInfo, allocator: *const VkAllocationCallbacks, sampler_out: *mut VkSampler) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_sampler.0)(device, create_info, allocator, sampler_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateSampler(device, create_info, allocator, sampler_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_sampler(device: VkDevice, sampler: VkSampler, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_sampler.0)(device, sampler, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroySampler(device, sampler, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_descriptor_set_layout(device: VkDevice, create_info: *const VkDescriptorSetLayoutCreateInfo, allocator: *const VkAllocationCallbacks, set_layout_out: *mut VkDescriptorSetLayout) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_descriptor_set_layout.0)(device, create_info, allocator, set_layout_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateDescriptorSetLayout(device, create_info, allocator, set_layout_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_descriptor_set_layout(device: VkDevice, descriptor_set_layout: VkDescriptorSetLayout, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_descriptor_set_layout.0)(device, descriptor_set_layout, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyDescriptorSetLayout(device, descriptor_set_layout, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_descriptor_pool(device: VkDevice, create_info: *const VkDescriptorPoolCreateInfo, allocator: *const VkAllocationCallbacks, descriptor_pool_out: *mut VkDescriptorPool) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_descriptor_pool.0)(device, create_info, allocator, descriptor_pool_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateDescriptorPool(device, create_info, allocator, descriptor_pool_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_descriptor_pool(device: VkDevice, descriptor_pool: VkDescriptorPool, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_descriptor_pool.0)(device, descriptor_pool, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyDescriptorPool(device, descriptor_pool, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn reset_descriptor_pool(device: VkDevice, descriptor_pool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.reset_descriptor_pool.0)(device, descriptor_pool, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkResetDescriptorPool(device, descriptor_pool, flags) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn allocate_descriptor_sets(device: VkDevice, allocate_info: *const VkDescriptorSetAllocateInfo, descriptor_sets_out: *mut VkDescriptorSet) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.allocate_descriptor_sets.0)(device, allocate_info, descriptor_sets_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkAllocateDescriptorSets(device, allocate_info, descriptor_sets_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn free_descriptor_sets(device: VkDevice, descriptor_pool: VkDescriptorPool, descriptor_set_count: u32, descriptor_sets: *const VkDescriptorSet) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.free_descriptor_sets.0)(device, descriptor_pool, descriptor_set_count, descriptor_sets) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkFreeDescriptorSets(device, descriptor_pool, descriptor_set_count, descriptor_sets) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn update_descriptor_sets(device: VkDevice, descriptor_write_count: u32, descriptor_writes: *const VkWriteDescriptorSet, descriptor_copy_count: u32, descriptor_copies: *const VkCopyDescriptorSet) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.update_descriptor_sets.0)(device, descriptor_write_count, descriptor_writes, descriptor_copy_count, descriptor_copies) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkUpdateDescriptorSets(device, descriptor_write_count, descriptor_writes, descriptor_copy_count, descriptor_copies) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_framebuffer(device: VkDevice, create_info: *const VkFramebufferCreateInfo, allocator: *const VkAllocationCallbacks, framebuffer_out: *mut VkFramebuffer) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_framebuffer.0)(device, create_info, allocator, framebuffer_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateFramebuffer(device, create_info, allocator, framebuffer_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_framebuffer(device: VkDevice, framebuffer: VkFramebuffer, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_framebuffer.0)(device, framebuffer, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyFramebuffer(device, framebuffer, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_render_pass(device: VkDevice, create_info: *const VkRenderPassCreateInfo, allocator: *const VkAllocationCallbacks, render_pass_out: *mut VkRenderPass) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_render_pass.0)(device, create_info, allocator, render_pass_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateRenderPass(device, create_info, allocator, render_pass_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_render_pass(device: VkDevice, render_pass: VkRenderPass, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_render_pass.0)(device, render_pass, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyRenderPass(device, render_pass, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_render_area_granularity(device: VkDevice, render_pass: VkRenderPass, granularity_out: *mut VkExtent2D) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_render_area_granularity.0)(device, render_pass, granularity_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetRenderAreaGranularity(device, render_pass, granularity_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_command_pool(device: VkDevice, create_info: *const VkCommandPoolCreateInfo, allocator: *const VkAllocationCallbacks, command_pool_out: *mut VkCommandPool) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_command_pool.0)(device, create_info, allocator, command_pool_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateCommandPool(device, create_info, allocator, command_pool_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_command_pool(device: VkDevice, command_pool: VkCommandPool, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_command_pool.0)(device, command_pool, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyCommandPool(device, command_pool, allocator) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn reset_command_pool(device: VkDevice, command_pool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.reset_command_pool.0)(device, command_pool, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkResetCommandPool(device, command_pool, flags) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn allocate_command_buffers(device: VkDevice, allocate_info: *const VkCommandBufferAllocateInfo, command_buffers_out: *mut VkCommandBuffer) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.allocate_command_buffers.0)(device, allocate_info, command_buffers_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkAllocateCommandBuffers(device, allocate_info, command_buffers_out) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn free_command_buffers(device: VkDevice, command_pool: VkCommandPool, command_buffer_count: u32, command_buffers: *const VkCommandBuffer) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.free_command_buffers.0)(device, command_pool, command_buffer_count, command_buffers) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkFreeCommandBuffers(device, command_pool, command_buffer_count, command_buffers) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn begin_command_buffer(command_buffer: VkCommandBuffer, begin_info: *const VkCommandBufferBeginInfo) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.begin_command_buffer.0)(command_buffer, begin_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkBeginCommandBuffer(command_buffer, begin_info) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn end_command_buffer(command_buffer: VkCommandBuffer) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.end_command_buffer.0)(command_buffer) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkEndCommandBuffer(command_buffer) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn reset_command_buffer(command_buffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.reset_command_buffer.0)(command_buffer, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkResetCommandBuffer(command_buffer, flags) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_bind_pipeline(command_buffer: VkCommandBuffer, pipeline_bind_point: VkPipelineBindPoint, pipeline: VkPipeline) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_bind_pipeline.0)(command_buffer, pipeline_bind_point, pipeline) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdBindPipeline(command_buffer, pipeline_bind_point, pipeline) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_set_viewport(command_buffer: VkCommandBuffer, first_viewport: u32, viewport_count: u32, viewports: *const VkViewport) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_set_viewport.0)(command_buffer, first_viewport, viewport_count, viewports) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdSetViewport(command_buffer, first_viewport, viewport_count, viewports) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_set_scissor(command_buffer: VkCommandBuffer, first_scissor: u32, scissor_count: u32, scissors: *const VkRect2D) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_set_scissor.0)(command_buffer, first_scissor, scissor_count, scissors) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdSetScissor(command_buffer, first_scissor, scissor_count, scissors) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_set_line_width(command_buffer: VkCommandBuffer, line_width: c_float) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_set_line_width.0)(command_buffer, line_width) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdSetLineWidth(command_buffer, line_width) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_set_depth_bias(command_buffer: VkCommandBuffer, depth_bias_constant_factor: c_float, depth_bias_clamp: c_float, depth_bias_slope_factor: c_float) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_set_depth_bias.0)(command_buffer, depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdSetDepthBias(command_buffer, depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_set_blend_constants(command_buffer: VkCommandBuffer, blend_constants: *const c_float) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_set_blend_constants.0)(command_buffer, blend_constants) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdSetBlendConstants(command_buffer, blend_constants) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_set_depth_bounds(command_buffer: VkCommandBuffer, min_depth_bounds: c_float, max_depth_bounds: c_float) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_set_depth_bounds.0)(command_buffer, min_depth_bounds, max_depth_bounds) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdSetDepthBounds(command_buffer, min_depth_bounds, max_depth_bounds) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_set_stencil_compare_mask(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, compare_mask: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_set_stencil_compare_mask.0)(command_buffer, face_mask, compare_mask) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdSetStencilCompareMask(command_buffer, face_mask, compare_mask) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_set_stencil_write_mask(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, write_mask: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_set_stencil_write_mask.0)(command_buffer, face_mask, write_mask) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdSetStencilWriteMask(command_buffer, face_mask, write_mask) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_set_stencil_reference(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, reference: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_set_stencil_reference.0)(command_buffer, face_mask, reference) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdSetStencilReference(command_buffer, face_mask, reference) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_bind_descriptor_sets(command_buffer: VkCommandBuffer, pipeline_bind_point: VkPipelineBindPoint, layout: VkPipelineLayout, first_set: u32, descriptor_set_count: u32, descriptor_sets: *const VkDescriptorSet, dynamic_offset_count: u32, dynamic_offsets: *const u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_bind_descriptor_sets.0)(command_buffer, pipeline_bind_point, layout, first_set, descriptor_set_count, descriptor_sets, dynamic_offset_count, dynamic_offsets) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdBindDescriptorSets(command_buffer, pipeline_bind_point, layout, first_set, descriptor_set_count, descriptor_sets, dynamic_offset_count, dynamic_offsets) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_bind_index_buffer(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, index_type: VkIndexType) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_bind_index_buffer.0)(command_buffer, buffer, offset, index_type) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdBindIndexBuffer(command_buffer, buffer, offset, index_type) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_bind_vertex_buffers(command_buffer: VkCommandBuffer, first_binding: u32, binding_count: u32, buffers: *const VkBuffer, offsets: *const VkDeviceSize) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_bind_vertex_buffers.0)(command_buffer, first_binding, binding_count, buffers, offsets) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdBindVertexBuffers(command_buffer, first_binding, binding_count, buffers, offsets) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_draw(command_buffer: VkCommandBuffer, vertex_count: u32, instance_count: u32, first_vertex: u32, first_index: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_draw.0)(command_buffer, vertex_count, instance_count, first_vertex, first_index) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdDraw(command_buffer, vertex_count, instance_count, first_vertex, first_index) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_draw_indexed(command_buffer: VkCommandBuffer, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_draw_indexed.0)(command_buffer, index_count, instance_count, first_index, vertex_offset, first_instance) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdDrawIndexed(command_buffer, index_count, instance_count, first_index, vertex_offset, first_instance) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_draw_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, draw_count: u32, stride: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_draw_indirect.0)(command_buffer, buffer, offset, draw_count, stride) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdDrawIndirect(command_buffer, buffer, offset, draw_count, stride) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_draw_indexed_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, draw_count: u32, stride: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_draw_indexed_indirect.0)(command_buffer, buffer, offset, draw_count, stride) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdDrawIndexedIndirect(command_buffer, buffer, offset, draw_count, stride) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_dispatch(command_buffer: VkCommandBuffer, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_dispatch.0)(command_buffer, group_count_x, group_count_y, group_count_z) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdDispatch(command_buffer, group_count_x, group_count_y, group_count_z) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_dispatch_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_dispatch_indirect.0)(command_buffer, buffer, offset) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdDispatchIndirect(command_buffer, buffer, offset) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_copy_buffer(command_buffer: VkCommandBuffer, src_buffer: VkBuffer, dst_buffer: VkBuffer, region_count: u32, regions: *const VkBufferCopy) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_copy_buffer.0)(command_buffer, src_buffer, dst_buffer, region_count, regions) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdCopyBuffer(command_buffer, src_buffer, dst_buffer, region_count, regions) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_copy_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageCopy) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_copy_image.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdCopyImage(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_blit_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageBlit, filter: VkFilter) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_blit_image.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions, filter) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdBlitImage(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions, filter) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_copy_buffer_to_image(command_buffer: VkCommandBuffer, src_buffer: VkBuffer, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkBufferImageCopy) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_copy_buffer_to_image.0)(command_buffer, src_buffer, dst_image, dst_image_layout, region_count, regions) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdCopyBufferToImage(command_buffer, src_buffer, dst_image, dst_image_layout, region_count, regions) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_copy_image_to_buffer(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_buffer: VkBuffer, region_count: u32, regions: *const VkBufferImageCopy) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_copy_image_to_buffer.0)(command_buffer, src_image, src_image_layout, dst_buffer, region_count, regions) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdCopyImageToBuffer(command_buffer, src_image, src_image_layout, dst_buffer, region_count, regions) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_update_buffer(command_buffer: VkCommandBuffer, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, data_size: VkDeviceSize, data: *const c_void) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_update_buffer.0)(command_buffer, dst_buffer, dst_offset, data_size, data) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdUpdateBuffer(command_buffer, dst_buffer, dst_offset, data_size, data) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_fill_buffer(command_buffer: VkCommandBuffer, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, size: VkDeviceSize, data: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_fill_buffer.0)(command_buffer, dst_buffer, dst_offset, size, data) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdFillBuffer(command_buffer, dst_buffer, dst_offset, size, data) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_clear_color_image(command_buffer: VkCommandBuffer, image: VkImage, image_layout: VkImageLayout, color: *const VkClearColorValue, range_count: u32, ranges: *const VkImageSubresourceRange) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_clear_color_image.0)(command_buffer, image, image_layout, color, range_count, ranges) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdClearColorImage(command_buffer, image, image_layout, color, range_count, ranges) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_clear_depth_stencil_image(command_buffer: VkCommandBuffer, image: VkImage, image_layout: VkImageLayout, depth_stencil: *const VkClearDepthStencilValue, range_count: u32, ranges: *const VkImageSubresourceRange) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_clear_depth_stencil_image.0)(command_buffer, image, image_layout, depth_stencil, range_count, ranges) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdClearDepthStencilImage(command_buffer, image, image_layout, depth_stencil, range_count, ranges) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_clear_attachments(command_buffer: VkCommandBuffer, attachment_count: u32, attachments: *const VkClearAttachment, rect_count: u32, rects: *const VkClearRect) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_clear_attachments.0)(command_buffer, attachment_count, attachments, rect_count, rects) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdClearAttachments(command_buffer, attachment_count, attachments, rect_count, rects) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_resolve_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageResolve) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_resolve_image.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdResolveImage(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_set_event(command_buffer: VkCommandBuffer, event: VkEvent, stage_mask: VkPipelineStageFlags) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_set_event.0)(command_buffer, event, stage_mask) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdSetEvent(command_buffer, event, stage_mask) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_reset_event(command_buffer: VkCommandBuffer, event: VkEvent, stage_mask: VkPipelineStageFlags) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_reset_event.0)(command_buffer, event, stage_mask) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdResetEvent(command_buffer, event, stage_mask) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_wait_events(command_buffer: VkCommandBuffer, event_count: u32, events: *const VkEvent, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags, memory_barrier_count: u32, memory_barriers: *const VkMemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const VkBufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const VkImageMemoryBarrier) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_wait_events.0)(command_buffer, event_count, events, src_stage_mask, dst_stage_mask, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdWaitEvents(command_buffer, event_count, events, src_stage_mask, dst_stage_mask, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_pipeline_barrier(command_buffer: VkCommandBuffer, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags, dependency_flags: VkDependencyFlags, memory_barrier_count: u32, memory_barriers: *const VkMemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const VkBufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const VkImageMemoryBarrier) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_pipeline_barrier.0)(command_buffer, src_stage_mask, dst_stage_mask, dependency_flags, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdPipelineBarrier(command_buffer, src_stage_mask, dst_stage_mask, dependency_flags, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_begin_query(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, query: u32, flags: VkQueryControlFlags) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_begin_query.0)(command_buffer, query_pool, query, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdBeginQuery(command_buffer, query_pool, query, flags) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_end_query(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, query: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_end_query.0)(command_buffer, query_pool, query) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdEndQuery(command_buffer, query_pool, query) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_reset_query_pool(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, first_query: u32, query_count: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_reset_query_pool.0)(command_buffer, query_pool, first_query, query_count) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdResetQueryPool(command_buffer, query_pool, first_query, query_count) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_write_timestamp(command_buffer: VkCommandBuffer, pipeline_stage: VkPipelineStageFlags, query_pool: VkQueryPool, query: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_write_timestamp.0)(command_buffer, pipeline_stage, query_pool, query) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdWriteTimestamp(command_buffer, pipeline_stage, query_pool, query) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_copy_query_pool_results(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, first_query: u32, query_count: u32, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_copy_query_pool_results.0)(command_buffer, query_pool, first_query, query_count, dst_buffer, dst_offset, stride, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdCopyQueryPoolResults(command_buffer, query_pool, first_query, query_count, dst_buffer, dst_offset, stride, flags) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_push_constants(command_buffer: VkCommandBuffer, layout: VkPipelineLayout, stage_flags: VkShaderStageFlags, offset: u32, size: u32, values: *const c_void) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_push_constants.0)(command_buffer, layout, stage_flags, offset, size, values) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdPushConstants(command_buffer, layout, stage_flags, offset, size, values) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_begin_render_pass(command_buffer: VkCommandBuffer, render_pass_begin_info: *const VkRenderPassBeginInfo, contents: VkSubpassContents) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_begin_render_pass.0)(command_buffer, render_pass_begin_info, contents) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdBeginRenderPass(command_buffer, render_pass_begin_info, contents) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_next_subpass(command_buffer: VkCommandBuffer, contents: VkSubpassContents) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_next_subpass.0)(command_buffer, contents) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdNextSubpass(command_buffer, contents) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_end_render_pass(command_buffer: VkCommandBuffer) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_end_render_pass.0)(command_buffer) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdEndRenderPass(command_buffer) }
}
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_execute_commands(command_buffer: VkCommandBuffer, command_buffer_count: u32, command_buffers: *const VkCommandBuffer) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_execute_commands.0)(command_buffer, command_buffer_count, command_buffers) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdExecuteCommands(command_buffer, command_buffer_count, command_buffers) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn enumerate_instance_version(api_version: *mut u32) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.enumerate_instance_version.0)(api_version) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkEnumerateInstanceVersion(api_version) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn trim_command_pool(device: VkDevice, command_pool: VkCommandPool, flags: VkCommandPoolTrimFlags) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.trim_command_pool.0)(device, command_pool, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkTrimCommandPool(device, command_pool, flags) }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_surface_khr(instance: VkInstance, surface: VkSurfaceKHR, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_surface_khr.0)(instance, surface, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroySurfaceKHR(instance, surface, allocator) }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_surface_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, surface: VkSurfaceKHR, supported_out: *mut VkBool32) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_surface_support_khr.0)(physical_device, queue_family_index, surface, supported_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceSurfaceSupportKHR(physical_device, queue_family_index, surface, supported_out) }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_surface_capabilities_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, surface_capabilities_out: *mut VkSurfaceCapabilitiesKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_surface_capabilities_khr.0)(physical_device, surface, surface_capabilities_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, surface, surface_capabilities_out) }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_surface_formats_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, surface_format_count_out: *mut u32, surface_formats_out: *mut VkSurfaceFormatKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_surface_formats_khr.0)(physical_device, surface, surface_format_count_out, surface_formats_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, surface, surface_format_count_out, surface_formats_out) }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_surface_present_modes_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, present_mode_count_out: *mut u32, present_modes_out: *mut VkPresentModeKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_surface_present_modes_khr.0)(physical_device, surface, present_mode_count_out, present_modes_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device, surface, present_mode_count_out, present_modes_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_swapchain_khr(device: VkDevice, create_info: *const VkSwapchainCreateInfoKHR, allocator: *const VkAllocationCallbacks, swapchain_out: *mut VkSwapchainKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_swapchain_khr.0)(device, create_info, allocator, swapchain_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateSwapchainKHR(device, create_info, allocator, swapchain_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_swapchain_khr(device: VkDevice, swapchain: VkSwapchainKHR, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_swapchain_khr.0)(device, swapchain, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroySwapchainKHR(device, swapchain, allocator) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_swapchain_images_khr(device: VkDevice, swapchain: VkSwapchainKHR, swapchain_image_count_out: *mut u32, swapchain_images_out: *mut VkImage) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_swapchain_images_khr.0)(device, swapchain, swapchain_image_count_out, swapchain_images_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetSwapchainImagesKHR(device, swapchain, swapchain_image_count_out, swapchain_images_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn acquire_next_image_khr(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: Option<VkSemaphore>, fence: Option<VkFence>, image_index_out: *mut u32) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.acquire_next_image_khr.0)(device, swapchain, timeout, semaphore, fence, image_index_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkAcquireNextImageKHR(device, swapchain, timeout, semaphore, fence, image_index_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn queue_present_khr(queue: VkQueue, present_info: *const VkPresentInfoKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.queue_present_khr.0)(queue, present_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkQueuePresentKHR(queue, present_info) }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_xlib_surface_khr(instance: VkInstance, create_info: *const VkXlibSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_xlib_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateXlibSurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_xlib_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, dpy: *mut x11::xlib::Display, visual_id: x11::xlib::VisualID) -> VkBool32 {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_xlib_presentation_support_khr.0)(physical_device, queue_family_index, dpy, visual_id) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceXlibPresentationSupportKHR(physical_device, queue_family_index, dpy, visual_id) }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_xcb_surface_khr(instance: VkInstance, create_info: *const VkXcbSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_xcb_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateXcbSurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_xcb_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, connection: *mut xcb::ffi::xcb_connection_t, visual_id: xcb::x::Visualid) -> VkBool32 {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_xcb_presentation_support_khr.0)(physical_device, queue_family_index, connection, visual_id) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceXcbPresentationSupportKHR(physical_device, queue_family_index, connection, visual_id) }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_wayland_surface_khr(instance: VkInstance, create_info: *const VkWaylandSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_wayland_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateWaylandSurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_wayland_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, display: *mut c_void) -> VkBool32 {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_wayland_presentation_support_khr.0)(physical_device, queue_family_index, display) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceWaylandPresentationSupportKHR(physical_device, queue_family_index, display) }
}
#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_android_surface_khr(instance: VkInstance, create_info: *const VkAndroidSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_android_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateAndroidSurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_win32_surface_khr(instance: VkInstance, create_info: *const VkWin32SurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_win32_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateWin32SurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_win32_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32) -> VkBool32 {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_win32_presentation_support_khr.0)(physical_device, queue_family_index) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceWin32PresentationSupportKHR(physical_device, queue_family_index) }
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_macos_surface_mvk(instance: VkInstance, create_info: *const VkMacOSSurfaceCreateInfoMVK, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_macos_surface_mvk.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateMacOSSurfaceMVK(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_metal_surface_ext(instance: VkInstance, create_info: *const VkMetalSurfaceCreateInfoEXT, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_metal_surface_ext.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateMetalSurfaceEXT(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_display_properties_khr(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkDisplayPropertiesKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_display_properties_khr.0)(physical_device, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceDisplayPropertiesKHR(physical_device, property_count_out, properties_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_display_plane_properties_khr(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkDisplayPlanePropertiesKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_display_plane_properties_khr.0)(physical_device, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physical_device, property_count_out, properties_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_display_plane_supported_displays_khr(physical_device: VkPhysicalDevice, plane_index: u32, display_count_out: *mut u32, displays_out: *mut VkDisplayKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_display_plane_supported_displays_khr.0)(physical_device, plane_index, display_count_out, displays_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetDisplayPlaneSupportedDisplaysKHR(physical_device, plane_index, display_count_out, displays_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_display_mode_properties_khr(physical_device: VkPhysicalDevice, display: VkDisplayKHR, property_count_out: *mut u32, properties_out: *mut VkDisplayModePropertiesKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_display_mode_properties_khr.0)(physical_device, display, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetDisplayModePropertiesKHR(physical_device, display, property_count_out, properties_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_display_mode_khr(physical_device: VkPhysicalDevice, display: VkDisplayKHR, create_info: *const VkDisplayModeCreateInfoKHR, allocator: *const VkAllocationCallbacks, mode_out: *mut VkDisplayModeKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_display_mode_khr.0)(physical_device, display, create_info, allocator, mode_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateDisplayModeKHR(physical_device, display, create_info, allocator, mode_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_display_plane_capabilities_khr(physcial_device: VkPhysicalDevice, mode: VkDisplayModeKHR, plane_index: u32, capabilities_out: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_display_plane_capabilities_khr.0)(physcial_device, mode, plane_index, capabilities_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetDisplayPlaneCapabilitiesKHR(physcial_device, mode, plane_index, capabilities_out) }
}
#[cfg(all(feature = "VK_KHR_display", feature = "VK_KHR_surface"))]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_display_plane_surface_khr(instance: VkInstance, create_info: *const VkDisplaySurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_display_plane_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateDisplayPlaneSurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_features2(physical_device: VkPhysicalDevice, features_out: *mut VkPhysicalDeviceFeatures2) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_features2.0)(physical_device, features_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceFeatures2(physical_device, features_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_format_properties2(physical_device: VkPhysicalDevice, format: VkFormat, format_properties_out: *mut VkFormatProperties2) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_format_properties2.0)(physical_device, format, format_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceFormatProperties2(physical_device, format, format_properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_image_format_properties2(physical_device: VkPhysicalDevice, image_format_info: *const VkPhysicalDeviceImageFormatInfo2, image_format_properties_out: *mut VkImageFormatProperties2) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_image_format_properties2.0)(physical_device, image_format_info, image_format_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceImageFormatProperties2(physical_device, image_format_info, image_format_properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_memory_properties2(physical_device: VkPhysicalDevice, memory_properties_out: *mut VkPhysicalDeviceMemoryProperties2) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_memory_properties2.0)(physical_device, memory_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceMemoryProperties2(physical_device, memory_properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_properties2(physical_device: VkPhysicalDevice, properties_out: *mut VkPhysicalDeviceProperties2) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_properties2.0)(physical_device, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceProperties2(physical_device, properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_queue_family_properties2(physical_device: VkPhysicalDevice, queue_family_property_count: *mut u32, queue_family_properties_out: *mut VkQueueFamilyProperties2) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_queue_family_properties2.0)(physical_device, queue_family_property_count, queue_family_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceQueueFamilyProperties2(physical_device, queue_family_property_count, queue_family_properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_physical_device_sparse_image_format_properties2(physical_device: VkPhysicalDevice, format_info: *const VkPhysicalDeviceSparseImageFormatInfo2, property_count: *mut u32, properties_out: *mut VkSparseImageFormatProperties2) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_physical_device_sparse_image_format_properties2.0)(physical_device, format_info, property_count, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetPhysicalDeviceSparseImageFormatProperties2(physical_device, format_info, property_count, properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_descriptor_update_template(device: VkDevice, create_info: *const VkDescriptorUpdateTemplateCreateInfo, allocator: *const VkAllocationCallbacks, descriptor_update_template_out: *mut VkDescriptorUpdateTemplate) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_descriptor_update_template.0)(device, create_info, allocator, descriptor_update_template_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateDescriptorUpdateTemplate(device, create_info, allocator, descriptor_update_template_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn destroy_descriptor_update_template(device: VkDevice, descriptor_update_template: VkDescriptorUpdateTemplate, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.destroy_descriptor_update_template.0)(device, descriptor_update_template, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkDestroyDescriptorUpdateTemplate(device, descriptor_update_template, allocator) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn update_descriptor_set_with_template(device: VkDevice, descriptor_set: VkDescriptorSet, descriptor_update_template: VkDescriptorUpdateTemplate, data: *const core::ffi::c_void) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.update_descriptor_set_with_template.0)(device, descriptor_set, descriptor_update_template, data) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkUpdateDescriptorSetWithTemplate(device, descriptor_set, descriptor_update_template, data) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_buffer_memory_requirements2(device: VkDevice, info: *const VkBufferMemoryRequirementsInfo2, memory_requirements: *mut VkMemoryRequirements2) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_buffer_memory_requirements2.0)(device, info, memory_requirements) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetBufferMemoryRequirements2(device, info, memory_requirements) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_image_memory_requirements2(device: VkDevice, info: *const VkImageMemoryRequirementsInfo2, memory_requirements: *mut VkMemoryRequirements2) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_image_memory_requirements2.0)(device, info, memory_requirements) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetImageMemoryRequirements2(device, info, memory_requirements) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_image_sparse_memory_requirements2(device: VkDevice, info: *const VkImageSparseMemoryRequirementsInfo2, sparse_memory_requirements_count: *mut u32, sparse_memory_requirements: *mut VkSparseImageMemoryRequirements2) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_image_sparse_memory_requirements2.0)(device, info, sparse_memory_requirements_count, sparse_memory_requirements) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetImageSparseMemoryRequirements2(device, info, sparse_memory_requirements_count, sparse_memory_requirements) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn bind_buffer_memory2(device: VkDevice, bind_info_count: u32, bind_infos: *const VkBindBufferMemoryInfo) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.bind_buffer_memory2.0)(device, bind_info_count, bind_infos) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkBindBufferMemory2(device, bind_info_count, bind_infos) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn bind_image_memory2(device: VkDevice, bind_info_count: u32, bind_infos: *const VkBindImageMemoryInfo) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.bind_image_memory2.0)(device, bind_info_count, bind_infos) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkBindImageMemory2(device, bind_info_count, bind_infos) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn create_render_pass2(device: VkDevice, create_info: *const VkRenderPassCreateInfo2, allocator: *const VkAllocationCallbacks, out: *mut VkRenderPass) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.create_render_pass2.0)(device, create_info, allocator, out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCreateRenderPass2(device, create_info, allocator, out) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn get_semaphore_counter_value(device: VkDevice, semaphore: VkSemaphore, value: *mut u64) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.get_semaphore_counter_value.0)(device, semaphore, value) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkGetSemaphoreCounterValue(device, semaphore, value) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn signal_semaphore(device: VkDevice, signal_info: *const VkSemaphoreSignalInfo) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.signal_semaphore.0)(device, signal_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkSignalSemaphore(device, signal_info) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn wait_semaphores(device: VkDevice, wait_info: *const VkSemaphoreWaitInfo, timeout: u64) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.wait_semaphores.0)(device, wait_info, timeout) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkWaitSemaphores(device, wait_info, timeout) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_begin_render_pass2(command_buffer: VkCommandBuffer, begin_info: *const VkRenderPassBeginInfo, begin_subpass_info: *const VkSubpassBeginInfo) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_begin_render_pass2.0)(command_buffer, begin_info, begin_subpass_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdBeginRenderPass2(command_buffer, begin_info, begin_subpass_info) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_next_subpass2(command_buffer: VkCommandBuffer, begin_subpass_info: *const VkSubpassBeginInfo, end_subpass_info: *const VkSubpassEndInfo) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_next_subpass2.0)(command_buffer, begin_subpass_info, end_subpass_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdNextSubpass2(command_buffer, begin_subpass_info, end_subpass_info) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_end_render_pass2(command_buffer: VkCommandBuffer, end_subpass_info: *const VkSubpassEndInfo) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_end_render_pass2.0)(command_buffer, end_subpass_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdEndRenderPass2(command_buffer, end_subpass_info) }
}
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_pipeline_barrier2(command_buffer: VkCommandBuffer, dependency_info: *const VkDependencyInfo) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_pipeline_barrier2.0)(command_buffer, dependency_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdPipelineBarrier2(command_buffer, dependency_info) }
}
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn queue_submit2(queue: VkQueue, submit_count: u32, submits: *const VkSubmitInfo2, fence: Option<VkFence>) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.queue_submit2.0)(queue, submit_count, submits, fence) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkQueueSubmit2(queue, submit_count, submits, fence) }
}
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip] #[inline(always)] #[allow(clippy::too_many_arguments)]
/// # Safety
///
/// direct calling ffi functions.
pub unsafe fn cmd_push_descriptor_set(command_buffer: VkCommandBuffer, pipeline_bind_point: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptor_write_count: u32, descriptor_writes: *const VkWriteDescriptorSet) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] unsafe { (FPTBL.cmd_push_descriptor_set.0)(command_buffer, pipeline_bind_point, layout, set, descriptor_write_count, descriptor_writes) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] unsafe { vkCmdPushDescriptorSet(command_buffer, pipeline_bind_point, layout, set, descriptor_write_count, descriptor_writes) }
}

#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
struct FunctionPointerTable {
    create_instance: PFN_vkCreateInstance,
    destroy_instance: PFN_vkDestroyInstance,
    enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
    get_physical_device_features: PFN_vkGetPhysicalDeviceFeatures,
    get_physical_device_format_properties: PFN_vkGetPhysicalDeviceFormatProperties,
    get_physical_device_image_format_properties: PFN_vkGetPhysicalDeviceImageFormatProperties,
    get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties,
    get_physical_device_queue_family_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    get_physical_device_memory_properties: PFN_vkGetPhysicalDeviceMemoryProperties,
    get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    create_device: PFN_vkCreateDevice,
    destroy_device: PFN_vkDestroyDevice,
    enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties,
    enumerate_device_extension_properties: PFN_vkEnumerateDeviceExtensionProperties,
    enumerate_instance_layer_properties: PFN_vkEnumerateInstanceLayerProperties,
    enumerate_device_layer_properties: PFN_vkEnumerateDeviceLayerProperties,
    get_device_queue: PFN_vkGetDeviceQueue,
    queue_submit: PFN_vkQueueSubmit,
    queue_wait_idle: PFN_vkQueueWaitIdle,
    device_wait_idle: PFN_vkDeviceWaitIdle,
    allocate_memory: PFN_vkAllocateMemory,
    free_memory: PFN_vkFreeMemory,
    map_memory: PFN_vkMapMemory,
    unmap_memory: PFN_vkUnmapMemory,
    flush_mapped_memory_ranges: PFN_vkFlushMappedMemoryRanges,
    invalidate_mapped_memory_ranges: PFN_vkInvalidateMappedMemoryRanges,
    get_device_memory_commitment: PFN_vkGetDeviceMemoryCommitment,
    bind_buffer_memory: PFN_vkBindBufferMemory,
    bind_image_memory: PFN_vkBindImageMemory,
    get_buffer_memory_requirements: PFN_vkGetBufferMemoryRequirements,
    get_image_memory_requirements: PFN_vkGetImageMemoryRequirements,
    get_image_sparse_memory_requirements: PFN_vkGetImageSparseMemoryRequirements,
    get_physical_device_sparse_image_format_properties: PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
    queue_bind_sparse: PFN_vkQueueBindSparse,
    create_fence: PFN_vkCreateFence,
    destroy_fence: PFN_vkDestroyFence,
    reset_fences: PFN_vkResetFences,
    get_fence_status: PFN_vkGetFenceStatus,
    wait_for_fences: PFN_vkWaitForFences,
    create_semaphore: PFN_vkCreateSemaphore,
    destroy_semaphore: PFN_vkDestroySemaphore,
    create_event: PFN_vkCreateEvent,
    destroy_event: PFN_vkDestroyEvent,
    get_event_status: PFN_vkGetEventStatus,
    set_event: PFN_vkSetEvent,
    reset_event: PFN_vkResetEvent,
    create_query_pool: PFN_vkCreateQueryPool,
    destroy_query_pool: PFN_vkDestroyQueryPool,
    get_query_pool_results: PFN_vkGetQueryPoolResults,
    create_buffer: PFN_vkCreateBuffer,
    destroy_buffer: PFN_vkDestroyBuffer,
    create_buffer_view: PFN_vkCreateBufferView,
    destroy_buffer_view: PFN_vkDestroyBufferView,
    create_image: PFN_vkCreateImage,
    destroy_image: PFN_vkDestroyImage,
    get_image_subresource_layout: PFN_vkGetImageSubresourceLayout,
    create_image_view: PFN_vkCreateImageView,
    destroy_image_view: PFN_vkDestroyImageView,
    create_shader_module: PFN_vkCreateShaderModule,
    destroy_shader_module: PFN_vkDestroyShaderModule,
    create_pipeline_cache: PFN_vkCreatePipelineCache,
    destroy_pipeline_cache: PFN_vkDestroyPipelineCache,
    get_pipeline_cache_data: PFN_vkGetPipelineCacheData,
    merge_pipeline_caches: PFN_vkMergePipelineCaches,
    create_graphics_pipelines: PFN_vkCreateGraphicsPipelines,
    create_compute_pipelines: PFN_vkCreateComputePipelines,
    destroy_pipeline: PFN_vkDestroyPipeline,
    create_pipeline_layout: PFN_vkCreatePipelineLayout,
    destroy_pipeline_layout: PFN_vkDestroyPipelineLayout,
    create_sampler: PFN_vkCreateSampler,
    destroy_sampler: PFN_vkDestroySampler,
    create_descriptor_set_layout: PFN_vkCreateDescriptorSetLayout,
    destroy_descriptor_set_layout: PFN_vkDestroyDescriptorSetLayout,
    create_descriptor_pool: PFN_vkCreateDescriptorPool,
    destroy_descriptor_pool: PFN_vkDestroyDescriptorPool,
    reset_descriptor_pool: PFN_vkResetDescriptorPool,
    allocate_descriptor_sets: PFN_vkAllocateDescriptorSets,
    free_descriptor_sets: PFN_vkFreeDescriptorSets,
    update_descriptor_sets: PFN_vkUpdateDescriptorSets,
    create_framebuffer: PFN_vkCreateFramebuffer,
    destroy_framebuffer: PFN_vkDestroyFramebuffer,
    create_render_pass: PFN_vkCreateRenderPass,
    destroy_render_pass: PFN_vkDestroyRenderPass,
    get_render_area_granularity: PFN_vkGetRenderAreaGranularity,
    create_command_pool: PFN_vkCreateCommandPool,
    destroy_command_pool: PFN_vkDestroyCommandPool,
    reset_command_pool: PFN_vkResetCommandPool,
    allocate_command_buffers: PFN_vkAllocateCommandBuffers,
    free_command_buffers: PFN_vkFreeCommandBuffers,
    begin_command_buffer: PFN_vkBeginCommandBuffer,
    end_command_buffer: PFN_vkEndCommandBuffer,
    reset_command_buffer: PFN_vkResetCommandBuffer,
    cmd_bind_pipeline: PFN_vkCmdBindPipeline,
    cmd_set_viewport: PFN_vkCmdSetViewport,
    cmd_set_scissor: PFN_vkCmdSetScissor,
    cmd_set_line_width: PFN_vkCmdSetLineWidth,
    cmd_set_depth_bias: PFN_vkCmdSetDepthBias,
    cmd_set_blend_constants: PFN_vkCmdSetBlendConstants,
    cmd_set_depth_bounds: PFN_vkCmdSetDepthBounds,
    cmd_set_stencil_compare_mask: PFN_vkCmdSetStencilCompareMask,
    cmd_set_stencil_write_mask: PFN_vkCmdSetStencilWriteMask,
    cmd_set_stencil_reference: PFN_vkCmdSetStencilReference,
    cmd_bind_descriptor_sets: PFN_vkCmdBindDescriptorSets,
    cmd_bind_index_buffer: PFN_vkCmdBindIndexBuffer,
    cmd_bind_vertex_buffers: PFN_vkCmdBindVertexBuffers,
    cmd_draw: PFN_vkCmdDraw,
    cmd_draw_indexed: PFN_vkCmdDrawIndexed,
    cmd_draw_indirect: PFN_vkCmdDrawIndirect,
    cmd_draw_indexed_indirect: PFN_vkCmdDrawIndexedIndirect,
    cmd_dispatch: PFN_vkCmdDispatch,
    cmd_dispatch_indirect: PFN_vkCmdDispatchIndirect,
    cmd_copy_buffer: PFN_vkCmdCopyBuffer,
    cmd_copy_image: PFN_vkCmdCopyImage,
    cmd_blit_image: PFN_vkCmdBlitImage,
    cmd_copy_buffer_to_image: PFN_vkCmdCopyBufferToImage,
    cmd_copy_image_to_buffer: PFN_vkCmdCopyImageToBuffer,
    cmd_update_buffer: PFN_vkCmdUpdateBuffer,
    cmd_fill_buffer: PFN_vkCmdFillBuffer,
    cmd_clear_color_image: PFN_vkCmdClearColorImage,
    cmd_clear_depth_stencil_image: PFN_vkCmdClearDepthStencilImage,
    cmd_clear_attachments: PFN_vkCmdClearAttachments,
    cmd_resolve_image: PFN_vkCmdResolveImage,
    cmd_set_event: PFN_vkCmdSetEvent,
    cmd_reset_event: PFN_vkCmdResetEvent,
    cmd_wait_events: PFN_vkCmdWaitEvents,
    cmd_pipeline_barrier: PFN_vkCmdPipelineBarrier,
    cmd_begin_query: PFN_vkCmdBeginQuery,
    cmd_end_query: PFN_vkCmdEndQuery,
    cmd_reset_query_pool: PFN_vkCmdResetQueryPool,
    cmd_write_timestamp: PFN_vkCmdWriteTimestamp,
    cmd_copy_query_pool_results: PFN_vkCmdCopyQueryPoolResults,
    cmd_push_constants: PFN_vkCmdPushConstants,
    cmd_begin_render_pass: PFN_vkCmdBeginRenderPass,
    cmd_next_subpass: PFN_vkCmdNextSubpass,
    cmd_end_render_pass: PFN_vkCmdEndRenderPass,
    cmd_execute_commands: PFN_vkCmdExecuteCommands,
    #[cfg(feature = "Allow1_1APIs")]
    enumerate_instance_version: PFN_vkEnumerateInstanceVersion,
    #[cfg(feature = "Allow1_1APIs")]
    trim_command_pool: PFN_vkTrimCommandPool,
    #[cfg(feature = "VK_KHR_surface")]
    destroy_surface_khr: PFN_vkDestroySurfaceKHR,
    #[cfg(feature = "VK_KHR_surface")]
    get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    #[cfg(feature = "VK_KHR_surface")]
    get_physical_device_surface_capabilities_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    #[cfg(feature = "VK_KHR_surface")]
    get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    #[cfg(feature = "VK_KHR_surface")]
    get_physical_device_surface_present_modes_khr: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
    #[cfg(feature = "VK_KHR_swapchain")]
    create_swapchain_khr: PFN_vkCreateSwapchainKHR,
    #[cfg(feature = "VK_KHR_swapchain")]
    destroy_swapchain_khr: PFN_vkDestroySwapchainKHR,
    #[cfg(feature = "VK_KHR_swapchain")]
    get_swapchain_images_khr: PFN_vkGetSwapchainImagesKHR,
    #[cfg(feature = "VK_KHR_swapchain")]
    acquire_next_image_khr: PFN_vkAcquireNextImageKHR,
    #[cfg(feature = "VK_KHR_swapchain")]
    queue_present_khr: PFN_vkQueuePresentKHR,
    #[cfg(feature = "VK_KHR_xlib_surface")]
    create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
    #[cfg(feature = "VK_KHR_xlib_surface")]
    get_physical_device_xlib_presentation_support_khr: PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
    #[cfg(feature = "VK_KHR_xcb_surface")]
    create_xcb_surface_khr: PFN_vkCreateXcbSurfaceKHR,
    #[cfg(feature = "VK_KHR_xcb_surface")]
    get_physical_device_xcb_presentation_support_khr: PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
    #[cfg(feature = "VK_KHR_wayland_surface")]
    create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR,
    #[cfg(feature = "VK_KHR_wayland_surface")]
    get_physical_device_wayland_presentation_support_khr: PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
    #[cfg(feature = "VK_KHR_android_surface")]
    create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
    #[cfg(feature = "VK_KHR_win32_surface")]
    create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
    #[cfg(feature = "VK_KHR_win32_surface")]
    get_physical_device_win32_presentation_support_khr: PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
    #[cfg(feature = "VK_MVK_macos_surface")]
    create_macos_surface_mvk: PFN_vkCreateMacOSSurfaceMVK,
    #[cfg(feature = "VK_EXT_metal_surface")]
    create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
    #[cfg(feature = "VK_KHR_display")]
    get_physical_device_display_properties_khr: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
    #[cfg(feature = "VK_KHR_display")]
    get_physical_device_display_plane_properties_khr: PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
    #[cfg(feature = "VK_KHR_display")]
    get_display_plane_supported_displays_khr: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
    #[cfg(feature = "VK_KHR_display")]
    get_display_mode_properties_khr: PFN_vkGetDisplayModePropertiesKHR,
    #[cfg(feature = "VK_KHR_display")]
    create_display_mode_khr: PFN_vkCreateDisplayModeKHR,
    #[cfg(feature = "VK_KHR_display")]
    get_display_plane_capabilities_khr: PFN_vkGetDisplayPlaneCapabilitiesKHR,
    #[cfg(all(feature = "VK_KHR_display", feature = "VK_KHR_surface"))]
    create_display_plane_surface_khr: PFN_vkCreateDisplayPlaneSurfaceKHR,
    #[cfg(feature = "Allow1_1APIs")]
    get_physical_device_features2: PFN_vkGetPhysicalDeviceFeatures2,
    #[cfg(feature = "Allow1_1APIs")]
    get_physical_device_format_properties2: PFN_vkGetPhysicalDeviceFormatProperties2,
    #[cfg(feature = "Allow1_1APIs")]
    get_physical_device_image_format_properties2: PFN_vkGetPhysicalDeviceImageFormatProperties2,
    #[cfg(feature = "Allow1_1APIs")]
    get_physical_device_memory_properties2: PFN_vkGetPhysicalDeviceMemoryProperties2,
    #[cfg(feature = "Allow1_1APIs")]
    get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2,
    #[cfg(feature = "Allow1_1APIs")]
    get_physical_device_queue_family_properties2: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    #[cfg(feature = "Allow1_1APIs")]
    get_physical_device_sparse_image_format_properties2: PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
    #[cfg(feature = "Allow1_1APIs")]
    create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate,
    #[cfg(feature = "Allow1_1APIs")]
    destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate,
    #[cfg(feature = "Allow1_1APIs")]
    update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate,
    #[cfg(feature = "Allow1_1APIs")]
    get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
    #[cfg(feature = "Allow1_1APIs")]
    get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
    #[cfg(feature = "Allow1_1APIs")]
    get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
    #[cfg(feature = "Allow1_1APIs")]
    bind_buffer_memory2: PFN_vkBindBufferMemory2,
    #[cfg(feature = "Allow1_1APIs")]
    bind_image_memory2: PFN_vkBindImageMemory2,
    #[cfg(feature = "Allow1_2APIs")]
    create_render_pass2: PFN_vkCreateRenderPass2,
    #[cfg(feature = "Allow1_2APIs")]
    get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue,
    #[cfg(feature = "Allow1_2APIs")]
    signal_semaphore: PFN_vkSignalSemaphore,
    #[cfg(feature = "Allow1_2APIs")]
    wait_semaphores: PFN_vkWaitSemaphores,
    #[cfg(feature = "Allow1_2APIs")]
    cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
    #[cfg(feature = "Allow1_2APIs")]
    cmd_next_subpass2: PFN_vkCmdNextSubpass2,
    #[cfg(feature = "Allow1_2APIs")]
    cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
    #[cfg(feature = "Allow1_3APIs")]
    cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
    #[cfg(feature = "Allow1_3APIs")]
    queue_submit2: PFN_vkQueueSubmit2,
    #[cfg(feature = "Allow1_4APIs")]
    cmd_push_descriptor_set: PFN_vkCmdPushDescriptorSet
}
#[rustfmt::skip]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
static mut FPTBL: FunctionPointerTable = FunctionPointerTable::INIT;

#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
impl FunctionPointerTable {
    const INIT: Self = Self {
        create_instance: PFN_vkCreateInstance(stub_create_instance),
        destroy_instance: PFN_vkDestroyInstance(stub_destroy_instance),
        enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices(stub_enumerate_physical_devices),
        get_physical_device_features: PFN_vkGetPhysicalDeviceFeatures(stub_get_physical_device_features),
        get_physical_device_format_properties: PFN_vkGetPhysicalDeviceFormatProperties(stub_get_physical_device_format_properties),
        get_physical_device_image_format_properties: PFN_vkGetPhysicalDeviceImageFormatProperties(stub_get_physical_device_image_format_properties),
        get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties(stub_get_physical_device_properties),
        get_physical_device_queue_family_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties(stub_get_physical_device_queue_family_properties),
        get_physical_device_memory_properties: PFN_vkGetPhysicalDeviceMemoryProperties(stub_get_physical_device_memory_properties),
        get_instance_proc_addr: PFN_vkGetInstanceProcAddr(stub_get_instance_proc_addr),
        get_device_proc_addr: PFN_vkGetDeviceProcAddr(stub_get_device_proc_addr),
        create_device: PFN_vkCreateDevice(stub_create_device),
        destroy_device: PFN_vkDestroyDevice(stub_destroy_device),
        enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties(stub_enumerate_instance_extension_properties),
        enumerate_device_extension_properties: PFN_vkEnumerateDeviceExtensionProperties(stub_enumerate_device_extension_properties),
        enumerate_instance_layer_properties: PFN_vkEnumerateInstanceLayerProperties(stub_enumerate_instance_layer_properties),
        enumerate_device_layer_properties: PFN_vkEnumerateDeviceLayerProperties(stub_enumerate_device_layer_properties),
        get_device_queue: PFN_vkGetDeviceQueue(stub_get_device_queue),
        queue_submit: PFN_vkQueueSubmit(stub_queue_submit),
        queue_wait_idle: PFN_vkQueueWaitIdle(stub_queue_wait_idle),
        device_wait_idle: PFN_vkDeviceWaitIdle(stub_device_wait_idle),
        allocate_memory: PFN_vkAllocateMemory(stub_allocate_memory),
        free_memory: PFN_vkFreeMemory(stub_free_memory),
        map_memory: PFN_vkMapMemory(stub_map_memory),
        unmap_memory: PFN_vkUnmapMemory(stub_unmap_memory),
        flush_mapped_memory_ranges: PFN_vkFlushMappedMemoryRanges(stub_flush_mapped_memory_ranges),
        invalidate_mapped_memory_ranges: PFN_vkInvalidateMappedMemoryRanges(stub_invalidate_mapped_memory_ranges),
        get_device_memory_commitment: PFN_vkGetDeviceMemoryCommitment(stub_get_device_memory_commitment),
        bind_buffer_memory: PFN_vkBindBufferMemory(stub_bind_buffer_memory),
        bind_image_memory: PFN_vkBindImageMemory(stub_bind_image_memory),
        get_buffer_memory_requirements: PFN_vkGetBufferMemoryRequirements(stub_get_buffer_memory_requirements),
        get_image_memory_requirements: PFN_vkGetImageMemoryRequirements(stub_get_image_memory_requirements),
        get_image_sparse_memory_requirements: PFN_vkGetImageSparseMemoryRequirements(stub_get_image_sparse_memory_requirements),
        get_physical_device_sparse_image_format_properties: PFN_vkGetPhysicalDeviceSparseImageFormatProperties(stub_get_physical_device_sparse_image_format_properties),
        queue_bind_sparse: PFN_vkQueueBindSparse(stub_queue_bind_sparse),
        create_fence: PFN_vkCreateFence(stub_create_fence),
        destroy_fence: PFN_vkDestroyFence(stub_destroy_fence),
        reset_fences: PFN_vkResetFences(stub_reset_fences),
        get_fence_status: PFN_vkGetFenceStatus(stub_get_fence_status),
        wait_for_fences: PFN_vkWaitForFences(stub_wait_for_fences),
        create_semaphore: PFN_vkCreateSemaphore(stub_create_semaphore),
        destroy_semaphore: PFN_vkDestroySemaphore(stub_destroy_semaphore),
        create_event: PFN_vkCreateEvent(stub_create_event),
        destroy_event: PFN_vkDestroyEvent(stub_destroy_event),
        get_event_status: PFN_vkGetEventStatus(stub_get_event_status),
        set_event: PFN_vkSetEvent(stub_set_event),
        reset_event: PFN_vkResetEvent(stub_reset_event),
        create_query_pool: PFN_vkCreateQueryPool(stub_create_query_pool),
        destroy_query_pool: PFN_vkDestroyQueryPool(stub_destroy_query_pool),
        get_query_pool_results: PFN_vkGetQueryPoolResults(stub_get_query_pool_results),
        create_buffer: PFN_vkCreateBuffer(stub_create_buffer),
        destroy_buffer: PFN_vkDestroyBuffer(stub_destroy_buffer),
        create_buffer_view: PFN_vkCreateBufferView(stub_create_buffer_view),
        destroy_buffer_view: PFN_vkDestroyBufferView(stub_destroy_buffer_view),
        create_image: PFN_vkCreateImage(stub_create_image),
        destroy_image: PFN_vkDestroyImage(stub_destroy_image),
        get_image_subresource_layout: PFN_vkGetImageSubresourceLayout(stub_get_image_subresource_layout),
        create_image_view: PFN_vkCreateImageView(stub_create_image_view),
        destroy_image_view: PFN_vkDestroyImageView(stub_destroy_image_view),
        create_shader_module: PFN_vkCreateShaderModule(stub_create_shader_module),
        destroy_shader_module: PFN_vkDestroyShaderModule(stub_destroy_shader_module),
        create_pipeline_cache: PFN_vkCreatePipelineCache(stub_create_pipeline_cache),
        destroy_pipeline_cache: PFN_vkDestroyPipelineCache(stub_destroy_pipeline_cache),
        get_pipeline_cache_data: PFN_vkGetPipelineCacheData(stub_get_pipeline_cache_data),
        merge_pipeline_caches: PFN_vkMergePipelineCaches(stub_merge_pipeline_caches),
        create_graphics_pipelines: PFN_vkCreateGraphicsPipelines(stub_create_graphics_pipelines),
        create_compute_pipelines: PFN_vkCreateComputePipelines(stub_create_compute_pipelines),
        destroy_pipeline: PFN_vkDestroyPipeline(stub_destroy_pipeline),
        create_pipeline_layout: PFN_vkCreatePipelineLayout(stub_create_pipeline_layout),
        destroy_pipeline_layout: PFN_vkDestroyPipelineLayout(stub_destroy_pipeline_layout),
        create_sampler: PFN_vkCreateSampler(stub_create_sampler),
        destroy_sampler: PFN_vkDestroySampler(stub_destroy_sampler),
        create_descriptor_set_layout: PFN_vkCreateDescriptorSetLayout(stub_create_descriptor_set_layout),
        destroy_descriptor_set_layout: PFN_vkDestroyDescriptorSetLayout(stub_destroy_descriptor_set_layout),
        create_descriptor_pool: PFN_vkCreateDescriptorPool(stub_create_descriptor_pool),
        destroy_descriptor_pool: PFN_vkDestroyDescriptorPool(stub_destroy_descriptor_pool),
        reset_descriptor_pool: PFN_vkResetDescriptorPool(stub_reset_descriptor_pool),
        allocate_descriptor_sets: PFN_vkAllocateDescriptorSets(stub_allocate_descriptor_sets),
        free_descriptor_sets: PFN_vkFreeDescriptorSets(stub_free_descriptor_sets),
        update_descriptor_sets: PFN_vkUpdateDescriptorSets(stub_update_descriptor_sets),
        create_framebuffer: PFN_vkCreateFramebuffer(stub_create_framebuffer),
        destroy_framebuffer: PFN_vkDestroyFramebuffer(stub_destroy_framebuffer),
        create_render_pass: PFN_vkCreateRenderPass(stub_create_render_pass),
        destroy_render_pass: PFN_vkDestroyRenderPass(stub_destroy_render_pass),
        get_render_area_granularity: PFN_vkGetRenderAreaGranularity(stub_get_render_area_granularity),
        create_command_pool: PFN_vkCreateCommandPool(stub_create_command_pool),
        destroy_command_pool: PFN_vkDestroyCommandPool(stub_destroy_command_pool),
        reset_command_pool: PFN_vkResetCommandPool(stub_reset_command_pool),
        allocate_command_buffers: PFN_vkAllocateCommandBuffers(stub_allocate_command_buffers),
        free_command_buffers: PFN_vkFreeCommandBuffers(stub_free_command_buffers),
        begin_command_buffer: PFN_vkBeginCommandBuffer(stub_begin_command_buffer),
        end_command_buffer: PFN_vkEndCommandBuffer(stub_end_command_buffer),
        reset_command_buffer: PFN_vkResetCommandBuffer(stub_reset_command_buffer),
        cmd_bind_pipeline: PFN_vkCmdBindPipeline(stub_cmd_bind_pipeline),
        cmd_set_viewport: PFN_vkCmdSetViewport(stub_cmd_set_viewport),
        cmd_set_scissor: PFN_vkCmdSetScissor(stub_cmd_set_scissor),
        cmd_set_line_width: PFN_vkCmdSetLineWidth(stub_cmd_set_line_width),
        cmd_set_depth_bias: PFN_vkCmdSetDepthBias(stub_cmd_set_depth_bias),
        cmd_set_blend_constants: PFN_vkCmdSetBlendConstants(stub_cmd_set_blend_constants),
        cmd_set_depth_bounds: PFN_vkCmdSetDepthBounds(stub_cmd_set_depth_bounds),
        cmd_set_stencil_compare_mask: PFN_vkCmdSetStencilCompareMask(stub_cmd_set_stencil_compare_mask),
        cmd_set_stencil_write_mask: PFN_vkCmdSetStencilWriteMask(stub_cmd_set_stencil_write_mask),
        cmd_set_stencil_reference: PFN_vkCmdSetStencilReference(stub_cmd_set_stencil_reference),
        cmd_bind_descriptor_sets: PFN_vkCmdBindDescriptorSets(stub_cmd_bind_descriptor_sets),
        cmd_bind_index_buffer: PFN_vkCmdBindIndexBuffer(stub_cmd_bind_index_buffer),
        cmd_bind_vertex_buffers: PFN_vkCmdBindVertexBuffers(stub_cmd_bind_vertex_buffers),
        cmd_draw: PFN_vkCmdDraw(stub_cmd_draw),
        cmd_draw_indexed: PFN_vkCmdDrawIndexed(stub_cmd_draw_indexed),
        cmd_draw_indirect: PFN_vkCmdDrawIndirect(stub_cmd_draw_indirect),
        cmd_draw_indexed_indirect: PFN_vkCmdDrawIndexedIndirect(stub_cmd_draw_indexed_indirect),
        cmd_dispatch: PFN_vkCmdDispatch(stub_cmd_dispatch),
        cmd_dispatch_indirect: PFN_vkCmdDispatchIndirect(stub_cmd_dispatch_indirect),
        cmd_copy_buffer: PFN_vkCmdCopyBuffer(stub_cmd_copy_buffer),
        cmd_copy_image: PFN_vkCmdCopyImage(stub_cmd_copy_image),
        cmd_blit_image: PFN_vkCmdBlitImage(stub_cmd_blit_image),
        cmd_copy_buffer_to_image: PFN_vkCmdCopyBufferToImage(stub_cmd_copy_buffer_to_image),
        cmd_copy_image_to_buffer: PFN_vkCmdCopyImageToBuffer(stub_cmd_copy_image_to_buffer),
        cmd_update_buffer: PFN_vkCmdUpdateBuffer(stub_cmd_update_buffer),
        cmd_fill_buffer: PFN_vkCmdFillBuffer(stub_cmd_fill_buffer),
        cmd_clear_color_image: PFN_vkCmdClearColorImage(stub_cmd_clear_color_image),
        cmd_clear_depth_stencil_image: PFN_vkCmdClearDepthStencilImage(stub_cmd_clear_depth_stencil_image),
        cmd_clear_attachments: PFN_vkCmdClearAttachments(stub_cmd_clear_attachments),
        cmd_resolve_image: PFN_vkCmdResolveImage(stub_cmd_resolve_image),
        cmd_set_event: PFN_vkCmdSetEvent(stub_cmd_set_event),
        cmd_reset_event: PFN_vkCmdResetEvent(stub_cmd_reset_event),
        cmd_wait_events: PFN_vkCmdWaitEvents(stub_cmd_wait_events),
        cmd_pipeline_barrier: PFN_vkCmdPipelineBarrier(stub_cmd_pipeline_barrier),
        cmd_begin_query: PFN_vkCmdBeginQuery(stub_cmd_begin_query),
        cmd_end_query: PFN_vkCmdEndQuery(stub_cmd_end_query),
        cmd_reset_query_pool: PFN_vkCmdResetQueryPool(stub_cmd_reset_query_pool),
        cmd_write_timestamp: PFN_vkCmdWriteTimestamp(stub_cmd_write_timestamp),
        cmd_copy_query_pool_results: PFN_vkCmdCopyQueryPoolResults(stub_cmd_copy_query_pool_results),
        cmd_push_constants: PFN_vkCmdPushConstants(stub_cmd_push_constants),
        cmd_begin_render_pass: PFN_vkCmdBeginRenderPass(stub_cmd_begin_render_pass),
        cmd_next_subpass: PFN_vkCmdNextSubpass(stub_cmd_next_subpass),
        cmd_end_render_pass: PFN_vkCmdEndRenderPass(stub_cmd_end_render_pass),
        cmd_execute_commands: PFN_vkCmdExecuteCommands(stub_cmd_execute_commands),
        #[cfg(feature = "Allow1_1APIs")]
        enumerate_instance_version: PFN_vkEnumerateInstanceVersion(stub_enumerate_instance_version),
        #[cfg(feature = "Allow1_1APIs")]
        trim_command_pool: PFN_vkTrimCommandPool(stub_trim_command_pool),
        #[cfg(feature = "VK_KHR_surface")]
        destroy_surface_khr: PFN_vkDestroySurfaceKHR(stub_destroy_surface_khr),
        #[cfg(feature = "VK_KHR_surface")]
        get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR(stub_get_physical_device_surface_support_khr),
        #[cfg(feature = "VK_KHR_surface")]
        get_physical_device_surface_capabilities_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(stub_get_physical_device_surface_capabilities_khr),
        #[cfg(feature = "VK_KHR_surface")]
        get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR(stub_get_physical_device_surface_formats_khr),
        #[cfg(feature = "VK_KHR_surface")]
        get_physical_device_surface_present_modes_khr: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR(stub_get_physical_device_surface_present_modes_khr),
        #[cfg(feature = "VK_KHR_swapchain")]
        create_swapchain_khr: PFN_vkCreateSwapchainKHR(stub_create_swapchain_khr),
        #[cfg(feature = "VK_KHR_swapchain")]
        destroy_swapchain_khr: PFN_vkDestroySwapchainKHR(stub_destroy_swapchain_khr),
        #[cfg(feature = "VK_KHR_swapchain")]
        get_swapchain_images_khr: PFN_vkGetSwapchainImagesKHR(stub_get_swapchain_images_khr),
        #[cfg(feature = "VK_KHR_swapchain")]
        acquire_next_image_khr: PFN_vkAcquireNextImageKHR(stub_acquire_next_image_khr),
        #[cfg(feature = "VK_KHR_swapchain")]
        queue_present_khr: PFN_vkQueuePresentKHR(stub_queue_present_khr),
        #[cfg(feature = "VK_KHR_xlib_surface")]
        create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR(stub_create_xlib_surface_khr),
        #[cfg(feature = "VK_KHR_xlib_surface")]
        get_physical_device_xlib_presentation_support_khr: PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR(stub_get_physical_device_xlib_presentation_support_khr),
        #[cfg(feature = "VK_KHR_xcb_surface")]
        create_xcb_surface_khr: PFN_vkCreateXcbSurfaceKHR(stub_create_xcb_surface_khr),
        #[cfg(feature = "VK_KHR_xcb_surface")]
        get_physical_device_xcb_presentation_support_khr: PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR(stub_get_physical_device_xcb_presentation_support_khr),
        #[cfg(feature = "VK_KHR_wayland_surface")]
        create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR(stub_create_wayland_surface_khr),
        #[cfg(feature = "VK_KHR_wayland_surface")]
        get_physical_device_wayland_presentation_support_khr: PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR(stub_get_physical_device_wayland_presentation_support_khr),
        #[cfg(feature = "VK_KHR_android_surface")]
        create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR(stub_create_android_surface_khr),
        #[cfg(feature = "VK_KHR_win32_surface")]
        create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR(stub_create_win32_surface_khr),
        #[cfg(feature = "VK_KHR_win32_surface")]
        get_physical_device_win32_presentation_support_khr: PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR(stub_get_physical_device_win32_presentation_support_khr),
        #[cfg(feature = "VK_MVK_macos_surface")]
        create_macos_surface_mvk: PFN_vkCreateMacOSSurfaceMVK(stub_create_macos_surface_mvk),
        #[cfg(feature = "VK_EXT_metal_surface")]
        create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT(stub_create_metal_surface_ext),
        #[cfg(feature = "VK_KHR_display")]
        get_physical_device_display_properties_khr: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR(stub_get_physical_device_display_properties_khr),
        #[cfg(feature = "VK_KHR_display")]
        get_physical_device_display_plane_properties_khr: PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR(stub_get_physical_device_display_plane_properties_khr),
        #[cfg(feature = "VK_KHR_display")]
        get_display_plane_supported_displays_khr: PFN_vkGetDisplayPlaneSupportedDisplaysKHR(stub_get_display_plane_supported_displays_khr),
        #[cfg(feature = "VK_KHR_display")]
        get_display_mode_properties_khr: PFN_vkGetDisplayModePropertiesKHR(stub_get_display_mode_properties_khr),
        #[cfg(feature = "VK_KHR_display")]
        create_display_mode_khr: PFN_vkCreateDisplayModeKHR(stub_create_display_mode_khr),
        #[cfg(feature = "VK_KHR_display")]
        get_display_plane_capabilities_khr: PFN_vkGetDisplayPlaneCapabilitiesKHR(stub_get_display_plane_capabilities_khr),
        #[cfg(all(feature = "VK_KHR_display", feature = "VK_KHR_surface"))]
        create_display_plane_surface_khr: PFN_vkCreateDisplayPlaneSurfaceKHR(stub_create_display_plane_surface_khr),
        #[cfg(feature = "Allow1_1APIs")]
        get_physical_device_features2: PFN_vkGetPhysicalDeviceFeatures2(stub_get_physical_device_features2),
        #[cfg(feature = "Allow1_1APIs")]
        get_physical_device_format_properties2: PFN_vkGetPhysicalDeviceFormatProperties2(stub_get_physical_device_format_properties2),
        #[cfg(feature = "Allow1_1APIs")]
        get_physical_device_image_format_properties2: PFN_vkGetPhysicalDeviceImageFormatProperties2(stub_get_physical_device_image_format_properties2),
        #[cfg(feature = "Allow1_1APIs")]
        get_physical_device_memory_properties2: PFN_vkGetPhysicalDeviceMemoryProperties2(stub_get_physical_device_memory_properties2),
        #[cfg(feature = "Allow1_1APIs")]
        get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2(stub_get_physical_device_properties2),
        #[cfg(feature = "Allow1_1APIs")]
        get_physical_device_queue_family_properties2: PFN_vkGetPhysicalDeviceQueueFamilyProperties2(stub_get_physical_device_queue_family_properties2),
        #[cfg(feature = "Allow1_1APIs")]
        get_physical_device_sparse_image_format_properties2: PFN_vkGetPhysicalDeviceSparseImageFormatProperties2(stub_get_physical_device_sparse_image_format_properties2),
        #[cfg(feature = "Allow1_1APIs")]
        create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate(stub_create_descriptor_update_template),
        #[cfg(feature = "Allow1_1APIs")]
        destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate(stub_destroy_descriptor_update_template),
        #[cfg(feature = "Allow1_1APIs")]
        update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate(stub_update_descriptor_set_with_template),
        #[cfg(feature = "Allow1_1APIs")]
        get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2(stub_get_buffer_memory_requirements2),
        #[cfg(feature = "Allow1_1APIs")]
        get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2(stub_get_image_memory_requirements2),
        #[cfg(feature = "Allow1_1APIs")]
        get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2(stub_get_image_sparse_memory_requirements2),
        #[cfg(feature = "Allow1_1APIs")]
        bind_buffer_memory2: PFN_vkBindBufferMemory2(stub_bind_buffer_memory2),
        #[cfg(feature = "Allow1_1APIs")]
        bind_image_memory2: PFN_vkBindImageMemory2(stub_bind_image_memory2),
        #[cfg(feature = "Allow1_2APIs")]
        create_render_pass2: PFN_vkCreateRenderPass2(stub_create_render_pass2),
        #[cfg(feature = "Allow1_2APIs")]
        get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue(stub_get_semaphore_counter_value),
        #[cfg(feature = "Allow1_2APIs")]
        signal_semaphore: PFN_vkSignalSemaphore(stub_signal_semaphore),
        #[cfg(feature = "Allow1_2APIs")]
        wait_semaphores: PFN_vkWaitSemaphores(stub_wait_semaphores),
        #[cfg(feature = "Allow1_2APIs")]
        cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2(stub_cmd_begin_render_pass2),
        #[cfg(feature = "Allow1_2APIs")]
        cmd_next_subpass2: PFN_vkCmdNextSubpass2(stub_cmd_next_subpass2),
        #[cfg(feature = "Allow1_2APIs")]
        cmd_end_render_pass2: PFN_vkCmdEndRenderPass2(stub_cmd_end_render_pass2),
        #[cfg(feature = "Allow1_3APIs")]
        cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2(stub_cmd_pipeline_barrier2),
        #[cfg(feature = "Allow1_3APIs")]
        queue_submit2: PFN_vkQueueSubmit2(stub_queue_submit2),
        #[cfg(feature = "Allow1_4APIs")]
        cmd_push_descriptor_set: PFN_vkCmdPushDescriptorSet(stub_cmd_push_descriptor_set)
    };
    #[inline(always)] #[rustfmt::skip] pub(crate) fn reset() { unsafe { FPTBL = Self::INIT; } }
}

#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_instance(create_info: *const VkInstanceCreateInfo, allocator: *const VkAllocationCallbacks, instance_out: *mut VkInstance) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateInstance = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_instance = fp; }
    unsafe { (fp.0)(create_info, allocator, instance_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_instance(instance: VkInstance, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyInstance = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_instance = fp; }
    unsafe { (fp.0)(instance, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_physical_devices(instance: VkInstance, physical_devices_count_out: *mut u32, physical_devices_out: *mut VkPhysicalDevice) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkEnumeratePhysicalDevices = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.enumerate_physical_devices = fp; }
    unsafe { (fp.0)(instance, physical_devices_count_out, physical_devices_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_features(physical_device: VkPhysicalDevice, features_out: *mut VkPhysicalDeviceFeatures) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceFeatures = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_features = fp; }
    unsafe { (fp.0)(physical_device, features_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, format_properties_out: *mut VkFormatProperties) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceFormatProperties = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_format_properties = fp; }
    unsafe { (fp.0)(physical_device, format, format_properties_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_image_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, image_type: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, image_format_properties_out: *mut VkImageFormatProperties) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_image_format_properties = fp; }
    unsafe { (fp.0)(physical_device, format, image_type, tiling, usage, flags, image_format_properties_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_properties(physical_device: VkPhysicalDevice, properties_out: *mut VkPhysicalDeviceProperties) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceProperties = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_properties = fp; }
    unsafe { (fp.0)(physical_device, properties_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_queue_family_properties(physical_device: VkPhysicalDevice, queue_family_properties_count_out: *mut u32, queue_family_properties_out: *mut VkQueueFamilyProperties) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_queue_family_properties = fp; }
    unsafe { (fp.0)(physical_device, queue_family_properties_count_out, queue_family_properties_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_memory_properties(physical_device: VkPhysicalDevice, memory_properties_out: *mut VkPhysicalDeviceMemoryProperties) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceMemoryProperties = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_memory_properties = fp; }
    unsafe { (fp.0)(physical_device, memory_properties_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_instance_proc_addr(instance: VkInstance, name: *const c_char) -> Option<PFN_vkVoidFunction> {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetInstanceProcAddr = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_instance_proc_addr = fp; }
    unsafe { (fp.0)(instance, name) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_device_proc_addr(device: VkDevice, name: *const c_char) -> Option<PFN_vkVoidFunction> {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetDeviceProcAddr = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_device_proc_addr = fp; }
    unsafe { (fp.0)(device, name) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_device(physical_device: VkPhysicalDevice, create_info: *const VkDeviceCreateInfo, allocator: *const VkAllocationCallbacks, device_out: *mut VkDevice) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateDevice = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_device = fp; }
    unsafe { (fp.0)(physical_device, create_info, allocator, device_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_device(device: VkDevice, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyDevice = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_device = fp; }
    unsafe { (fp.0)(device, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_instance_extension_properties(layer_name: *const c_char, property_count_out: *mut u32, properties_out: *mut VkExtensionProperties) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkEnumerateInstanceExtensionProperties = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.enumerate_instance_extension_properties = fp; }
    unsafe { (fp.0)(layer_name, property_count_out, properties_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_device_extension_properties(physical_device: VkPhysicalDevice, layer_name: *const c_char, property_count_out: *mut u32, properties_out: *mut VkExtensionProperties) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkEnumerateDeviceExtensionProperties = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.enumerate_device_extension_properties = fp; }
    unsafe { (fp.0)(physical_device, layer_name, property_count_out, properties_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_instance_layer_properties(property_count_out: *mut u32, properties_out: *mut VkLayerProperties) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkEnumerateInstanceLayerProperties = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.enumerate_instance_layer_properties = fp; }
    unsafe { (fp.0)(property_count_out, properties_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_device_layer_properties(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkLayerProperties) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkEnumerateDeviceLayerProperties = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.enumerate_device_layer_properties = fp; }
    unsafe { (fp.0)(physical_device, property_count_out, properties_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_device_queue(device: VkDevice, queue_family_index: u32, queue_index: u32, queue_out: *mut VkQueue) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetDeviceQueue = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_device_queue = fp; }
    unsafe { (fp.0)(device, queue_family_index, queue_index, queue_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_queue_submit(queue: VkQueue, submit_count: u32, submits: *const VkSubmitInfo, fence: Option<VkFence>) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkQueueSubmit = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.queue_submit = fp; }
    unsafe { (fp.0)(queue, submit_count, submits, fence) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_queue_wait_idle(queue: VkQueue) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkQueueWaitIdle = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.queue_wait_idle = fp; }
    unsafe { (fp.0)(queue) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_device_wait_idle(device: VkDevice) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDeviceWaitIdle = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.device_wait_idle = fp; }
    unsafe { (fp.0)(device) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_allocate_memory(device: VkDevice, allocate_info: *const VkMemoryAllocateInfo, allocator: *const VkAllocationCallbacks, memory_out: *mut VkDeviceMemory) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkAllocateMemory = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.allocate_memory = fp; }
    unsafe { (fp.0)(device, allocate_info, allocator, memory_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_free_memory(device: VkDevice, memory: VkDeviceMemory, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkFreeMemory = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.free_memory = fp; }
    unsafe { (fp.0)(device, memory, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_map_memory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, data_ptr_out: *mut *mut c_void) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkMapMemory = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.map_memory = fp; }
    unsafe { (fp.0)(device, memory, offset, size, flags, data_ptr_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_unmap_memory(device: VkDevice, memory: VkDeviceMemory) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkUnmapMemory = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.unmap_memory = fp; }
    unsafe { (fp.0)(device, memory) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_flush_mapped_memory_ranges(device: VkDevice, memory_range_count: u32, memory_ranges: *const VkMappedMemoryRange) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkFlushMappedMemoryRanges = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.flush_mapped_memory_ranges = fp; }
    unsafe { (fp.0)(device, memory_range_count, memory_ranges) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_invalidate_mapped_memory_ranges(device: VkDevice, memory_range_count: u32, memory_ranges: *const VkMappedMemoryRange) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkInvalidateMappedMemoryRanges = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.invalidate_mapped_memory_ranges = fp; }
    unsafe { (fp.0)(device, memory_range_count, memory_ranges) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_device_memory_commitment(device: VkDevice, memory: VkDeviceMemory, committed_memory_bytes_out: *mut VkDeviceSize) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetDeviceMemoryCommitment = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_device_memory_commitment = fp; }
    unsafe { (fp.0)(device, memory, committed_memory_bytes_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_bind_buffer_memory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memory_offset: VkDeviceSize) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkBindBufferMemory = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.bind_buffer_memory = fp; }
    unsafe { (fp.0)(device, buffer, memory, memory_offset) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_bind_image_memory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memory_offset: VkDeviceSize) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkBindImageMemory = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.bind_image_memory = fp; }
    unsafe { (fp.0)(device, image, memory, memory_offset) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_buffer_memory_requirements(device: VkDevice, buffer: VkBuffer, memory_requirements_out: *mut VkMemoryRequirements) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetBufferMemoryRequirements = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_buffer_memory_requirements = fp; }
    unsafe { (fp.0)(device, buffer, memory_requirements_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_image_memory_requirements(device: VkDevice, image: VkImage, memory_requirements_out: *mut VkMemoryRequirements) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetImageMemoryRequirements = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_image_memory_requirements = fp; }
    unsafe { (fp.0)(device, image, memory_requirements_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_image_sparse_memory_requirements(device: VkDevice, image: VkImage, sparse_memory_requirement_count_out: *mut u32, sparse_memory_requirements_out: *mut VkSparseImageMemoryRequirements) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetImageSparseMemoryRequirements = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_image_sparse_memory_requirements = fp; }
    unsafe { (fp.0)(device, image, sparse_memory_requirement_count_out, sparse_memory_requirements_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_sparse_image_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling, property_count_out: *mut u32, properties_out: *mut VkSparseImageFormatProperties) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_sparse_image_format_properties = fp; }
    unsafe { (fp.0)(physical_device, format, r#type, samples, usage, tiling, property_count_out, properties_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_queue_bind_sparse(queue: VkQueue, bind_info_count: u32, bind_info: *const VkBindSparseInfo, fence: Option<VkFence>) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkQueueBindSparse = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.queue_bind_sparse = fp; }
    unsafe { (fp.0)(queue, bind_info_count, bind_info, fence) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_fence(device: VkDevice, create_info: *const VkFenceCreateInfo, allocator: *const VkAllocationCallbacks, fence_out: *mut VkFence) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateFence = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_fence = fp; }
    unsafe { (fp.0)(device, create_info, allocator, fence_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_fence(device: VkDevice, fence: VkFence, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyFence = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_fence = fp; }
    unsafe { (fp.0)(device, fence, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_reset_fences(device: VkDevice, fence_count: u32, fences: *const VkFence) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkResetFences = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.reset_fences = fp; }
    unsafe { (fp.0)(device, fence_count, fences) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_fence_status(device: VkDevice, fence: VkFence) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetFenceStatus = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_fence_status = fp; }
    unsafe { (fp.0)(device, fence) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_wait_for_fences(device: VkDevice, fence_count: u32, fences: *const VkFence, wait_all: VkBool32, timeout: u64) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkWaitForFences = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.wait_for_fences = fp; }
    unsafe { (fp.0)(device, fence_count, fences, wait_all, timeout) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_semaphore(device: VkDevice, create_info: *const VkSemaphoreCreateInfo, allocator: *const VkAllocationCallbacks, semaphore_out: *mut VkSemaphore) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateSemaphore = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_semaphore = fp; }
    unsafe { (fp.0)(device, create_info, allocator, semaphore_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_semaphore(device: VkDevice, semaphore: VkSemaphore, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroySemaphore = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_semaphore = fp; }
    unsafe { (fp.0)(device, semaphore, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_event(device: VkDevice, create_info: *const VkEventCreateInfo, allocator: *const VkAllocationCallbacks, event_out: *mut VkEvent) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateEvent = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_event = fp; }
    unsafe { (fp.0)(device, create_info, allocator, event_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_event(device: VkDevice, event: VkEvent, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyEvent = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_event = fp; }
    unsafe { (fp.0)(device, event, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_event_status(device: VkDevice, event: VkEvent) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetEventStatus = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_event_status = fp; }
    unsafe { (fp.0)(device, event) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_set_event(device: VkDevice, event: VkEvent) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkSetEvent = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.set_event = fp; }
    unsafe { (fp.0)(device, event) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_reset_event(device: VkDevice, event: VkEvent) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkResetEvent = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.reset_event = fp; }
    unsafe { (fp.0)(device, event) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_query_pool(device: VkDevice, create_info: *const VkQueryPoolCreateInfo, allocator: *const VkAllocationCallbacks, query_pool_out: *mut VkQueryPool) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateQueryPool = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_query_pool = fp; }
    unsafe { (fp.0)(device, create_info, allocator, query_pool_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_query_pool(device: VkDevice, query_pool: VkQueryPool, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyQueryPool = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_query_pool = fp; }
    unsafe { (fp.0)(device, query_pool, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_query_pool_results(device: VkDevice, query_pool: VkQueryPool, first_query: u32, query_count: u32, data_size: usize, data_out: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetQueryPoolResults = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_query_pool_results = fp; }
    unsafe { (fp.0)(device, query_pool, first_query, query_count, data_size, data_out, stride, flags) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_buffer(device: VkDevice, create_info: *const VkBufferCreateInfo, allocator: *const VkAllocationCallbacks, buffer_out: *mut VkBuffer) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateBuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_buffer = fp; }
    unsafe { (fp.0)(device, create_info, allocator, buffer_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_buffer(device: VkDevice, buffer: VkBuffer, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyBuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_buffer = fp; }
    unsafe { (fp.0)(device, buffer, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_buffer_view(device: VkDevice, create_info: *const VkBufferViewCreateInfo, allocator: *const VkAllocationCallbacks, view_out: *mut VkBufferView) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateBufferView = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_buffer_view = fp; }
    unsafe { (fp.0)(device, create_info, allocator, view_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_buffer_view(device: VkDevice, buffer_view: VkBufferView, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyBufferView = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_buffer_view = fp; }
    unsafe { (fp.0)(device, buffer_view, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_image(device: VkDevice, create_info: *const VkImageCreateInfo, allocator: *const VkAllocationCallbacks, image_out: *mut VkImage) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateImage = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_image = fp; }
    unsafe { (fp.0)(device, create_info, allocator, image_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_image(device: VkDevice, image: VkImage, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyImage = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_image = fp; }
    unsafe { (fp.0)(device, image, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_image_subresource_layout(device: VkDevice, image: VkImage, subresource: *const VkImageSubresource, layout_out: *mut VkSubresourceLayout) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetImageSubresourceLayout = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_image_subresource_layout = fp; }
    unsafe { (fp.0)(device, image, subresource, layout_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_image_view(device: VkDevice, create_info: *const VkImageViewCreateInfo, allocator: *const VkAllocationCallbacks, view_out: *mut VkImageView) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateImageView = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_image_view = fp; }
    unsafe { (fp.0)(device, create_info, allocator, view_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_image_view(device: VkDevice, image_view: VkImageView, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyImageView = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_image_view = fp; }
    unsafe { (fp.0)(device, image_view, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_shader_module(device: VkDevice, create_info: *const VkShaderModuleCreateInfo, allocator: *const VkAllocationCallbacks, shader_module_out: *mut VkShaderModule) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateShaderModule = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_shader_module = fp; }
    unsafe { (fp.0)(device, create_info, allocator, shader_module_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_shader_module(device: VkDevice, shader_module: VkShaderModule, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyShaderModule = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_shader_module = fp; }
    unsafe { (fp.0)(device, shader_module, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_pipeline_cache(device: VkDevice, create_info: *const VkPipelineCacheCreateInfo, allocator: *const VkAllocationCallbacks, pipeline_cache_out: *mut VkPipelineCache) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreatePipelineCache = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_pipeline_cache = fp; }
    unsafe { (fp.0)(device, create_info, allocator, pipeline_cache_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_pipeline_cache(device: VkDevice, pipeline_cache: VkPipelineCache, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyPipelineCache = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_pipeline_cache = fp; }
    unsafe { (fp.0)(device, pipeline_cache, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_pipeline_cache_data(device: VkDevice, pipeline_cache: VkPipelineCache, data_size_out: *mut usize, data_out: *mut c_void) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPipelineCacheData = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_pipeline_cache_data = fp; }
    unsafe { (fp.0)(device, pipeline_cache, data_size_out, data_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_merge_pipeline_caches(device: VkDevice, dst_cache: VkPipelineCache, src_cache_count: u32, src_caches: *const VkPipelineCache) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkMergePipelineCaches = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.merge_pipeline_caches = fp; }
    unsafe { (fp.0)(device, dst_cache, src_cache_count, src_caches) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_graphics_pipelines(device: VkDevice, pipeline_cache: Option<VkPipelineCache>, create_info_count: u32, create_infos: *const VkGraphicsPipelineCreateInfo, allocator: *const VkAllocationCallbacks, pipelines_out: *mut VkPipeline) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateGraphicsPipelines = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_graphics_pipelines = fp; }
    unsafe { (fp.0)(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_compute_pipelines(device: VkDevice, pipeline_cache: Option<VkPipelineCache>, create_info_count: u32, create_infos: *const VkComputePipelineCreateInfo, allocator: *const VkAllocationCallbacks, pipelines_out: *mut VkPipeline) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateComputePipelines = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_compute_pipelines = fp; }
    unsafe { (fp.0)(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_pipeline(device: VkDevice, pipeline: VkPipeline, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyPipeline = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_pipeline = fp; }
    unsafe { (fp.0)(device, pipeline, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_pipeline_layout(device: VkDevice, create_info: *const VkPipelineLayoutCreateInfo, allocator: *const VkAllocationCallbacks, pipeline_layout_out: *mut VkPipelineLayout) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreatePipelineLayout = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_pipeline_layout = fp; }
    unsafe { (fp.0)(device, create_info, allocator, pipeline_layout_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_pipeline_layout(device: VkDevice, pipeline_layout: VkPipelineLayout, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyPipelineLayout = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_pipeline_layout = fp; }
    unsafe { (fp.0)(device, pipeline_layout, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_sampler(device: VkDevice, create_info: *const VkSamplerCreateInfo, allocator: *const VkAllocationCallbacks, sampler_out: *mut VkSampler) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateSampler = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_sampler = fp; }
    unsafe { (fp.0)(device, create_info, allocator, sampler_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_sampler(device: VkDevice, sampler: VkSampler, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroySampler = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_sampler = fp; }
    unsafe { (fp.0)(device, sampler, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_descriptor_set_layout(device: VkDevice, create_info: *const VkDescriptorSetLayoutCreateInfo, allocator: *const VkAllocationCallbacks, set_layout_out: *mut VkDescriptorSetLayout) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateDescriptorSetLayout = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_descriptor_set_layout = fp; }
    unsafe { (fp.0)(device, create_info, allocator, set_layout_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_descriptor_set_layout(device: VkDevice, descriptor_set_layout: VkDescriptorSetLayout, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyDescriptorSetLayout = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_descriptor_set_layout = fp; }
    unsafe { (fp.0)(device, descriptor_set_layout, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_descriptor_pool(device: VkDevice, create_info: *const VkDescriptorPoolCreateInfo, allocator: *const VkAllocationCallbacks, descriptor_pool_out: *mut VkDescriptorPool) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateDescriptorPool = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_descriptor_pool = fp; }
    unsafe { (fp.0)(device, create_info, allocator, descriptor_pool_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_descriptor_pool(device: VkDevice, descriptor_pool: VkDescriptorPool, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyDescriptorPool = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_descriptor_pool = fp; }
    unsafe { (fp.0)(device, descriptor_pool, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_reset_descriptor_pool(device: VkDevice, descriptor_pool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkResetDescriptorPool = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.reset_descriptor_pool = fp; }
    unsafe { (fp.0)(device, descriptor_pool, flags) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_allocate_descriptor_sets(device: VkDevice, allocate_info: *const VkDescriptorSetAllocateInfo, descriptor_sets_out: *mut VkDescriptorSet) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkAllocateDescriptorSets = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.allocate_descriptor_sets = fp; }
    unsafe { (fp.0)(device, allocate_info, descriptor_sets_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_free_descriptor_sets(device: VkDevice, descriptor_pool: VkDescriptorPool, descriptor_set_count: u32, descriptor_sets: *const VkDescriptorSet) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkFreeDescriptorSets = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.free_descriptor_sets = fp; }
    unsafe { (fp.0)(device, descriptor_pool, descriptor_set_count, descriptor_sets) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_update_descriptor_sets(device: VkDevice, descriptor_write_count: u32, descriptor_writes: *const VkWriteDescriptorSet, descriptor_copy_count: u32, descriptor_copies: *const VkCopyDescriptorSet) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkUpdateDescriptorSets = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.update_descriptor_sets = fp; }
    unsafe { (fp.0)(device, descriptor_write_count, descriptor_writes, descriptor_copy_count, descriptor_copies) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_framebuffer(device: VkDevice, create_info: *const VkFramebufferCreateInfo, allocator: *const VkAllocationCallbacks, framebuffer_out: *mut VkFramebuffer) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateFramebuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_framebuffer = fp; }
    unsafe { (fp.0)(device, create_info, allocator, framebuffer_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_framebuffer(device: VkDevice, framebuffer: VkFramebuffer, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyFramebuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_framebuffer = fp; }
    unsafe { (fp.0)(device, framebuffer, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_render_pass(device: VkDevice, create_info: *const VkRenderPassCreateInfo, allocator: *const VkAllocationCallbacks, render_pass_out: *mut VkRenderPass) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateRenderPass = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_render_pass = fp; }
    unsafe { (fp.0)(device, create_info, allocator, render_pass_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_render_pass(device: VkDevice, render_pass: VkRenderPass, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyRenderPass = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_render_pass = fp; }
    unsafe { (fp.0)(device, render_pass, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_render_area_granularity(device: VkDevice, render_pass: VkRenderPass, granularity_out: *mut VkExtent2D) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetRenderAreaGranularity = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_render_area_granularity = fp; }
    unsafe { (fp.0)(device, render_pass, granularity_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_command_pool(device: VkDevice, create_info: *const VkCommandPoolCreateInfo, allocator: *const VkAllocationCallbacks, command_pool_out: *mut VkCommandPool) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateCommandPool = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_command_pool = fp; }
    unsafe { (fp.0)(device, create_info, allocator, command_pool_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_command_pool(device: VkDevice, command_pool: VkCommandPool, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyCommandPool = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_command_pool = fp; }
    unsafe { (fp.0)(device, command_pool, allocator) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_reset_command_pool(device: VkDevice, command_pool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkResetCommandPool = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.reset_command_pool = fp; }
    unsafe { (fp.0)(device, command_pool, flags) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_allocate_command_buffers(device: VkDevice, allocate_info: *const VkCommandBufferAllocateInfo, command_buffers_out: *mut VkCommandBuffer) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkAllocateCommandBuffers = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.allocate_command_buffers = fp; }
    unsafe { (fp.0)(device, allocate_info, command_buffers_out) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_free_command_buffers(device: VkDevice, command_pool: VkCommandPool, command_buffer_count: u32, command_buffers: *const VkCommandBuffer) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkFreeCommandBuffers = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.free_command_buffers = fp; }
    unsafe { (fp.0)(device, command_pool, command_buffer_count, command_buffers) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_begin_command_buffer(command_buffer: VkCommandBuffer, begin_info: *const VkCommandBufferBeginInfo) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkBeginCommandBuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.begin_command_buffer = fp; }
    unsafe { (fp.0)(command_buffer, begin_info) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_end_command_buffer(command_buffer: VkCommandBuffer) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkEndCommandBuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.end_command_buffer = fp; }
    unsafe { (fp.0)(command_buffer) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_reset_command_buffer(command_buffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkResetCommandBuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.reset_command_buffer = fp; }
    unsafe { (fp.0)(command_buffer, flags) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_bind_pipeline(command_buffer: VkCommandBuffer, pipeline_bind_point: VkPipelineBindPoint, pipeline: VkPipeline) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdBindPipeline = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_bind_pipeline = fp; }
    unsafe { (fp.0)(command_buffer, pipeline_bind_point, pipeline) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_viewport(command_buffer: VkCommandBuffer, first_viewport: u32, viewport_count: u32, viewports: *const VkViewport) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdSetViewport = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_set_viewport = fp; }
    unsafe { (fp.0)(command_buffer, first_viewport, viewport_count, viewports) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_scissor(command_buffer: VkCommandBuffer, first_scissor: u32, scissor_count: u32, scissors: *const VkRect2D) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdSetScissor = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_set_scissor = fp; }
    unsafe { (fp.0)(command_buffer, first_scissor, scissor_count, scissors) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_line_width(command_buffer: VkCommandBuffer, line_width: c_float) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdSetLineWidth = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_set_line_width = fp; }
    unsafe { (fp.0)(command_buffer, line_width) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_depth_bias(command_buffer: VkCommandBuffer, depth_bias_constant_factor: c_float, depth_bias_clamp: c_float, depth_bias_slope_factor: c_float) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdSetDepthBias = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_set_depth_bias = fp; }
    unsafe { (fp.0)(command_buffer, depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_blend_constants(command_buffer: VkCommandBuffer, blend_constants: *const c_float) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdSetBlendConstants = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_set_blend_constants = fp; }
    unsafe { (fp.0)(command_buffer, blend_constants) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_depth_bounds(command_buffer: VkCommandBuffer, min_depth_bounds: c_float, max_depth_bounds: c_float) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdSetDepthBounds = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_set_depth_bounds = fp; }
    unsafe { (fp.0)(command_buffer, min_depth_bounds, max_depth_bounds) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_stencil_compare_mask(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, compare_mask: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdSetStencilCompareMask = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_set_stencil_compare_mask = fp; }
    unsafe { (fp.0)(command_buffer, face_mask, compare_mask) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_stencil_write_mask(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, write_mask: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdSetStencilWriteMask = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_set_stencil_write_mask = fp; }
    unsafe { (fp.0)(command_buffer, face_mask, write_mask) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_stencil_reference(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, reference: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdSetStencilReference = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_set_stencil_reference = fp; }
    unsafe { (fp.0)(command_buffer, face_mask, reference) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_bind_descriptor_sets(command_buffer: VkCommandBuffer, pipeline_bind_point: VkPipelineBindPoint, layout: VkPipelineLayout, first_set: u32, descriptor_set_count: u32, descriptor_sets: *const VkDescriptorSet, dynamic_offset_count: u32, dynamic_offsets: *const u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdBindDescriptorSets = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_bind_descriptor_sets = fp; }
    unsafe { (fp.0)(command_buffer, pipeline_bind_point, layout, first_set, descriptor_set_count, descriptor_sets, dynamic_offset_count, dynamic_offsets) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_bind_index_buffer(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, index_type: VkIndexType) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdBindIndexBuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_bind_index_buffer = fp; }
    unsafe { (fp.0)(command_buffer, buffer, offset, index_type) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_bind_vertex_buffers(command_buffer: VkCommandBuffer, first_binding: u32, binding_count: u32, buffers: *const VkBuffer, offsets: *const VkDeviceSize) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdBindVertexBuffers = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_bind_vertex_buffers = fp; }
    unsafe { (fp.0)(command_buffer, first_binding, binding_count, buffers, offsets) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_draw(command_buffer: VkCommandBuffer, vertex_count: u32, instance_count: u32, first_vertex: u32, first_index: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdDraw = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_draw = fp; }
    unsafe { (fp.0)(command_buffer, vertex_count, instance_count, first_vertex, first_index) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_draw_indexed(command_buffer: VkCommandBuffer, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdDrawIndexed = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_draw_indexed = fp; }
    unsafe { (fp.0)(command_buffer, index_count, instance_count, first_index, vertex_offset, first_instance) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_draw_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, draw_count: u32, stride: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdDrawIndirect = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_draw_indirect = fp; }
    unsafe { (fp.0)(command_buffer, buffer, offset, draw_count, stride) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_draw_indexed_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, draw_count: u32, stride: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdDrawIndexedIndirect = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_draw_indexed_indirect = fp; }
    unsafe { (fp.0)(command_buffer, buffer, offset, draw_count, stride) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_dispatch(command_buffer: VkCommandBuffer, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdDispatch = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_dispatch = fp; }
    unsafe { (fp.0)(command_buffer, group_count_x, group_count_y, group_count_z) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_dispatch_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdDispatchIndirect = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_dispatch_indirect = fp; }
    unsafe { (fp.0)(command_buffer, buffer, offset) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_copy_buffer(command_buffer: VkCommandBuffer, src_buffer: VkBuffer, dst_buffer: VkBuffer, region_count: u32, regions: *const VkBufferCopy) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdCopyBuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_copy_buffer = fp; }
    unsafe { (fp.0)(command_buffer, src_buffer, dst_buffer, region_count, regions) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_copy_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageCopy) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdCopyImage = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_copy_image = fp; }
    unsafe { (fp.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_blit_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageBlit, filter: VkFilter) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdBlitImage = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_blit_image = fp; }
    unsafe { (fp.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions, filter) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_copy_buffer_to_image(command_buffer: VkCommandBuffer, src_buffer: VkBuffer, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkBufferImageCopy) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdCopyBufferToImage = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_copy_buffer_to_image = fp; }
    unsafe { (fp.0)(command_buffer, src_buffer, dst_image, dst_image_layout, region_count, regions) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_copy_image_to_buffer(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_buffer: VkBuffer, region_count: u32, regions: *const VkBufferImageCopy) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdCopyImageToBuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_copy_image_to_buffer = fp; }
    unsafe { (fp.0)(command_buffer, src_image, src_image_layout, dst_buffer, region_count, regions) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_update_buffer(command_buffer: VkCommandBuffer, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, data_size: VkDeviceSize, data: *const c_void) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdUpdateBuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_update_buffer = fp; }
    unsafe { (fp.0)(command_buffer, dst_buffer, dst_offset, data_size, data) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_fill_buffer(command_buffer: VkCommandBuffer, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, size: VkDeviceSize, data: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdFillBuffer = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_fill_buffer = fp; }
    unsafe { (fp.0)(command_buffer, dst_buffer, dst_offset, size, data) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_clear_color_image(command_buffer: VkCommandBuffer, image: VkImage, image_layout: VkImageLayout, color: *const VkClearColorValue, range_count: u32, ranges: *const VkImageSubresourceRange) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdClearColorImage = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_clear_color_image = fp; }
    unsafe { (fp.0)(command_buffer, image, image_layout, color, range_count, ranges) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_clear_depth_stencil_image(command_buffer: VkCommandBuffer, image: VkImage, image_layout: VkImageLayout, depth_stencil: *const VkClearDepthStencilValue, range_count: u32, ranges: *const VkImageSubresourceRange) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdClearDepthStencilImage = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_clear_depth_stencil_image = fp; }
    unsafe { (fp.0)(command_buffer, image, image_layout, depth_stencil, range_count, ranges) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_clear_attachments(command_buffer: VkCommandBuffer, attachment_count: u32, attachments: *const VkClearAttachment, rect_count: u32, rects: *const VkClearRect) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdClearAttachments = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_clear_attachments = fp; }
    unsafe { (fp.0)(command_buffer, attachment_count, attachments, rect_count, rects) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_resolve_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageResolve) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdResolveImage = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_resolve_image = fp; }
    unsafe { (fp.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_event(command_buffer: VkCommandBuffer, event: VkEvent, stage_mask: VkPipelineStageFlags) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdSetEvent = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_set_event = fp; }
    unsafe { (fp.0)(command_buffer, event, stage_mask) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_reset_event(command_buffer: VkCommandBuffer, event: VkEvent, stage_mask: VkPipelineStageFlags) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdResetEvent = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_reset_event = fp; }
    unsafe { (fp.0)(command_buffer, event, stage_mask) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_wait_events(command_buffer: VkCommandBuffer, event_count: u32, events: *const VkEvent, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags, memory_barrier_count: u32, memory_barriers: *const VkMemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const VkBufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const VkImageMemoryBarrier) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdWaitEvents = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_wait_events = fp; }
    unsafe { (fp.0)(command_buffer, event_count, events, src_stage_mask, dst_stage_mask, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_pipeline_barrier(command_buffer: VkCommandBuffer, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags, dependency_flags: VkDependencyFlags, memory_barrier_count: u32, memory_barriers: *const VkMemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const VkBufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const VkImageMemoryBarrier) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdPipelineBarrier = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_pipeline_barrier = fp; }
    unsafe { (fp.0)(command_buffer, src_stage_mask, dst_stage_mask, dependency_flags, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_begin_query(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, query: u32, flags: VkQueryControlFlags) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdBeginQuery = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_begin_query = fp; }
    unsafe { (fp.0)(command_buffer, query_pool, query, flags) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_end_query(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, query: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdEndQuery = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_end_query = fp; }
    unsafe { (fp.0)(command_buffer, query_pool, query) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_reset_query_pool(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, first_query: u32, query_count: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdResetQueryPool = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_reset_query_pool = fp; }
    unsafe { (fp.0)(command_buffer, query_pool, first_query, query_count) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_write_timestamp(command_buffer: VkCommandBuffer, pipeline_stage: VkPipelineStageFlags, query_pool: VkQueryPool, query: u32) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdWriteTimestamp = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_write_timestamp = fp; }
    unsafe { (fp.0)(command_buffer, pipeline_stage, query_pool, query) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_copy_query_pool_results(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, first_query: u32, query_count: u32, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdCopyQueryPoolResults = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_copy_query_pool_results = fp; }
    unsafe { (fp.0)(command_buffer, query_pool, first_query, query_count, dst_buffer, dst_offset, stride, flags) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_push_constants(command_buffer: VkCommandBuffer, layout: VkPipelineLayout, stage_flags: VkShaderStageFlags, offset: u32, size: u32, values: *const c_void) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdPushConstants = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_push_constants = fp; }
    unsafe { (fp.0)(command_buffer, layout, stage_flags, offset, size, values) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_begin_render_pass(command_buffer: VkCommandBuffer, render_pass_begin_info: *const VkRenderPassBeginInfo, contents: VkSubpassContents) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdBeginRenderPass = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_begin_render_pass = fp; }
    unsafe { (fp.0)(command_buffer, render_pass_begin_info, contents) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_next_subpass(command_buffer: VkCommandBuffer, contents: VkSubpassContents) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdNextSubpass = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_next_subpass = fp; }
    unsafe { (fp.0)(command_buffer, contents) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_end_render_pass(command_buffer: VkCommandBuffer) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdEndRenderPass = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_end_render_pass = fp; }
    unsafe { (fp.0)(command_buffer) }
}
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_execute_commands(command_buffer: VkCommandBuffer, command_buffer_count: u32, command_buffers: *const VkCommandBuffer) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdExecuteCommands = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_execute_commands = fp; }
    unsafe { (fp.0)(command_buffer, command_buffer_count, command_buffers) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_instance_version(api_version: *mut u32) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkEnumerateInstanceVersion = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.enumerate_instance_version = fp; }
    unsafe { (fp.0)(api_version) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_trim_command_pool(device: VkDevice, command_pool: VkCommandPool, flags: VkCommandPoolTrimFlags) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkTrimCommandPool = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.trim_command_pool = fp; }
    unsafe { (fp.0)(device, command_pool, flags) }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_surface_khr(instance: VkInstance, surface: VkSurfaceKHR, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroySurfaceKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_surface_khr = fp; }
    unsafe { (fp.0)(instance, surface, allocator) }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_surface_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, surface: VkSurfaceKHR, supported_out: *mut VkBool32) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_surface_support_khr = fp; }
    unsafe { (fp.0)(physical_device, queue_family_index, surface, supported_out) }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_surface_capabilities_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, surface_capabilities_out: *mut VkSurfaceCapabilitiesKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_surface_capabilities_khr = fp; }
    unsafe { (fp.0)(physical_device, surface, surface_capabilities_out) }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_surface_formats_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, surface_format_count_out: *mut u32, surface_formats_out: *mut VkSurfaceFormatKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_surface_formats_khr = fp; }
    unsafe { (fp.0)(physical_device, surface, surface_format_count_out, surface_formats_out) }
}
#[cfg(feature = "VK_KHR_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_surface_present_modes_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, present_mode_count_out: *mut u32, present_modes_out: *mut VkPresentModeKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_surface_present_modes_khr = fp; }
    unsafe { (fp.0)(physical_device, surface, present_mode_count_out, present_modes_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_swapchain_khr(device: VkDevice, create_info: *const VkSwapchainCreateInfoKHR, allocator: *const VkAllocationCallbacks, swapchain_out: *mut VkSwapchainKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateSwapchainKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_swapchain_khr = fp; }
    unsafe { (fp.0)(device, create_info, allocator, swapchain_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_swapchain_khr(device: VkDevice, swapchain: VkSwapchainKHR, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroySwapchainKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_swapchain_khr = fp; }
    unsafe { (fp.0)(device, swapchain, allocator) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_swapchain_images_khr(device: VkDevice, swapchain: VkSwapchainKHR, swapchain_image_count_out: *mut u32, swapchain_images_out: *mut VkImage) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetSwapchainImagesKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_swapchain_images_khr = fp; }
    unsafe { (fp.0)(device, swapchain, swapchain_image_count_out, swapchain_images_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_acquire_next_image_khr(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: Option<VkSemaphore>, fence: Option<VkFence>, image_index_out: *mut u32) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkAcquireNextImageKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.acquire_next_image_khr = fp; }
    unsafe { (fp.0)(device, swapchain, timeout, semaphore, fence, image_index_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_queue_present_khr(queue: VkQueue, present_info: *const VkPresentInfoKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkQueuePresentKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.queue_present_khr = fp; }
    unsafe { (fp.0)(queue, present_info) }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_xlib_surface_khr(instance: VkInstance, create_info: *const VkXlibSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateXlibSurfaceKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_xlib_surface_khr = fp; }
    unsafe { (fp.0)(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_xlib_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, dpy: *mut x11::xlib::Display, visual_id: x11::xlib::VisualID) -> VkBool32 {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_xlib_presentation_support_khr = fp; }
    unsafe { (fp.0)(physical_device, queue_family_index, dpy, visual_id) }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_xcb_surface_khr(instance: VkInstance, create_info: *const VkXcbSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateXcbSurfaceKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_xcb_surface_khr = fp; }
    unsafe { (fp.0)(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_xcb_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, connection: *mut xcb::ffi::xcb_connection_t, visual_id: xcb::x::Visualid) -> VkBool32 {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_xcb_presentation_support_khr = fp; }
    unsafe { (fp.0)(physical_device, queue_family_index, connection, visual_id) }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_wayland_surface_khr(instance: VkInstance, create_info: *const VkWaylandSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateWaylandSurfaceKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_wayland_surface_khr = fp; }
    unsafe { (fp.0)(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_wayland_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, display: *mut c_void) -> VkBool32 {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_wayland_presentation_support_khr = fp; }
    unsafe { (fp.0)(physical_device, queue_family_index, display) }
}
#[cfg(feature = "VK_KHR_android_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_android_surface_khr(instance: VkInstance, create_info: *const VkAndroidSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateAndroidSurfaceKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_android_surface_khr = fp; }
    unsafe { (fp.0)(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_win32_surface_khr(instance: VkInstance, create_info: *const VkWin32SurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateWin32SurfaceKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_win32_surface_khr = fp; }
    unsafe { (fp.0)(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_win32_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32) -> VkBool32 {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_win32_presentation_support_khr = fp; }
    unsafe { (fp.0)(physical_device, queue_family_index) }
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_macos_surface_mvk(instance: VkInstance, create_info: *const VkMacOSSurfaceCreateInfoMVK, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateMacOSSurfaceMVK = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_macos_surface_mvk = fp; }
    unsafe { (fp.0)(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_EXT_metal_surface")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_metal_surface_ext(instance: VkInstance, create_info: *const VkMetalSurfaceCreateInfoEXT, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateMetalSurfaceEXT = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_metal_surface_ext = fp; }
    unsafe { (fp.0)(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_display_properties_khr(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkDisplayPropertiesKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_display_properties_khr = fp; }
    unsafe { (fp.0)(physical_device, property_count_out, properties_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_display_plane_properties_khr(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkDisplayPlanePropertiesKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_display_plane_properties_khr = fp; }
    unsafe { (fp.0)(physical_device, property_count_out, properties_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_display_plane_supported_displays_khr(physical_device: VkPhysicalDevice, plane_index: u32, display_count_out: *mut u32, displays_out: *mut VkDisplayKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_display_plane_supported_displays_khr = fp; }
    unsafe { (fp.0)(physical_device, plane_index, display_count_out, displays_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_display_mode_properties_khr(physical_device: VkPhysicalDevice, display: VkDisplayKHR, property_count_out: *mut u32, properties_out: *mut VkDisplayModePropertiesKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetDisplayModePropertiesKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_display_mode_properties_khr = fp; }
    unsafe { (fp.0)(physical_device, display, property_count_out, properties_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_display_mode_khr(physical_device: VkPhysicalDevice, display: VkDisplayKHR, create_info: *const VkDisplayModeCreateInfoKHR, allocator: *const VkAllocationCallbacks, mode_out: *mut VkDisplayModeKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateDisplayModeKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_display_mode_khr = fp; }
    unsafe { (fp.0)(physical_device, display, create_info, allocator, mode_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_display_plane_capabilities_khr(physcial_device: VkPhysicalDevice, mode: VkDisplayModeKHR, plane_index: u32, capabilities_out: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_display_plane_capabilities_khr = fp; }
    unsafe { (fp.0)(physcial_device, mode, plane_index, capabilities_out) }
}
#[cfg(all(feature = "VK_KHR_display", feature = "VK_KHR_surface"))]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_display_plane_surface_khr(instance: VkInstance, create_info: *const VkDisplaySurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_display_plane_surface_khr = fp; }
    unsafe { (fp.0)(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_features2(physical_device: VkPhysicalDevice, features_out: *mut VkPhysicalDeviceFeatures2) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceFeatures2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_features2 = fp; }
    unsafe { (fp.0)(physical_device, features_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_format_properties2(physical_device: VkPhysicalDevice, format: VkFormat, format_properties_out: *mut VkFormatProperties2) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_format_properties2 = fp; }
    unsafe { (fp.0)(physical_device, format, format_properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_image_format_properties2(physical_device: VkPhysicalDevice, image_format_info: *const VkPhysicalDeviceImageFormatInfo2, image_format_properties_out: *mut VkImageFormatProperties2) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_image_format_properties2 = fp; }
    unsafe { (fp.0)(physical_device, image_format_info, image_format_properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_memory_properties2(physical_device: VkPhysicalDevice, memory_properties_out: *mut VkPhysicalDeviceMemoryProperties2) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_memory_properties2 = fp; }
    unsafe { (fp.0)(physical_device, memory_properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_properties2(physical_device: VkPhysicalDevice, properties_out: *mut VkPhysicalDeviceProperties2) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceProperties2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_properties2 = fp; }
    unsafe { (fp.0)(physical_device, properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_queue_family_properties2(physical_device: VkPhysicalDevice, queue_family_property_count: *mut u32, queue_family_properties_out: *mut VkQueueFamilyProperties2) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_queue_family_properties2 = fp; }
    unsafe { (fp.0)(physical_device, queue_family_property_count, queue_family_properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_sparse_image_format_properties2(physical_device: VkPhysicalDevice, format_info: *const VkPhysicalDeviceSparseImageFormatInfo2, property_count: *mut u32, properties_out: *mut VkSparseImageFormatProperties2) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_physical_device_sparse_image_format_properties2 = fp; }
    unsafe { (fp.0)(physical_device, format_info, property_count, properties_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_descriptor_update_template(device: VkDevice, create_info: *const VkDescriptorUpdateTemplateCreateInfo, allocator: *const VkAllocationCallbacks, descriptor_update_template_out: *mut VkDescriptorUpdateTemplate) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateDescriptorUpdateTemplate = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_descriptor_update_template = fp; }
    unsafe { (fp.0)(device, create_info, allocator, descriptor_update_template_out) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_descriptor_update_template(device: VkDevice, descriptor_update_template: VkDescriptorUpdateTemplate, allocator: *const VkAllocationCallbacks) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkDestroyDescriptorUpdateTemplate = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.destroy_descriptor_update_template = fp; }
    unsafe { (fp.0)(device, descriptor_update_template, allocator) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_update_descriptor_set_with_template(device: VkDevice, descriptor_set: VkDescriptorSet, descriptor_update_template: VkDescriptorUpdateTemplate, data: *const core::ffi::c_void) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkUpdateDescriptorSetWithTemplate = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.update_descriptor_set_with_template = fp; }
    unsafe { (fp.0)(device, descriptor_set, descriptor_update_template, data) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_buffer_memory_requirements2(device: VkDevice, info: *const VkBufferMemoryRequirementsInfo2, memory_requirements: *mut VkMemoryRequirements2) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetBufferMemoryRequirements2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_buffer_memory_requirements2 = fp; }
    unsafe { (fp.0)(device, info, memory_requirements) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_image_memory_requirements2(device: VkDevice, info: *const VkImageMemoryRequirementsInfo2, memory_requirements: *mut VkMemoryRequirements2) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetImageMemoryRequirements2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_image_memory_requirements2 = fp; }
    unsafe { (fp.0)(device, info, memory_requirements) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_image_sparse_memory_requirements2(device: VkDevice, info: *const VkImageSparseMemoryRequirementsInfo2, sparse_memory_requirements_count: *mut u32, sparse_memory_requirements: *mut VkSparseImageMemoryRequirements2) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetImageSparseMemoryRequirements2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_image_sparse_memory_requirements2 = fp; }
    unsafe { (fp.0)(device, info, sparse_memory_requirements_count, sparse_memory_requirements) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_bind_buffer_memory2(device: VkDevice, bind_info_count: u32, bind_infos: *const VkBindBufferMemoryInfo) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkBindBufferMemory2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.bind_buffer_memory2 = fp; }
    unsafe { (fp.0)(device, bind_info_count, bind_infos) }
}
#[cfg(feature = "Allow1_1APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_bind_image_memory2(device: VkDevice, bind_info_count: u32, bind_infos: *const VkBindImageMemoryInfo) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkBindImageMemory2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.bind_image_memory2 = fp; }
    unsafe { (fp.0)(device, bind_info_count, bind_infos) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_render_pass2(device: VkDevice, create_info: *const VkRenderPassCreateInfo2, allocator: *const VkAllocationCallbacks, out: *mut VkRenderPass) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCreateRenderPass2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.create_render_pass2 = fp; }
    unsafe { (fp.0)(device, create_info, allocator, out) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_semaphore_counter_value(device: VkDevice, semaphore: VkSemaphore, value: *mut u64) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkGetSemaphoreCounterValue = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.get_semaphore_counter_value = fp; }
    unsafe { (fp.0)(device, semaphore, value) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_signal_semaphore(device: VkDevice, signal_info: *const VkSemaphoreSignalInfo) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkSignalSemaphore = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.signal_semaphore = fp; }
    unsafe { (fp.0)(device, signal_info) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_wait_semaphores(device: VkDevice, wait_info: *const VkSemaphoreWaitInfo, timeout: u64) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkWaitSemaphores = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.wait_semaphores = fp; }
    unsafe { (fp.0)(device, wait_info, timeout) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_begin_render_pass2(command_buffer: VkCommandBuffer, begin_info: *const VkRenderPassBeginInfo, begin_subpass_info: *const VkSubpassBeginInfo) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdBeginRenderPass2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_begin_render_pass2 = fp; }
    unsafe { (fp.0)(command_buffer, begin_info, begin_subpass_info) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_next_subpass2(command_buffer: VkCommandBuffer, begin_subpass_info: *const VkSubpassBeginInfo, end_subpass_info: *const VkSubpassEndInfo) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdNextSubpass2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_next_subpass2 = fp; }
    unsafe { (fp.0)(command_buffer, begin_subpass_info, end_subpass_info) }
}
#[cfg(feature = "Allow1_2APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_end_render_pass2(command_buffer: VkCommandBuffer, end_subpass_info: *const VkSubpassEndInfo) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdEndRenderPass2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_end_render_pass2 = fp; }
    unsafe { (fp.0)(command_buffer, end_subpass_info) }
}
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_pipeline_barrier2(command_buffer: VkCommandBuffer, dependency_info: *const VkDependencyInfo) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdPipelineBarrier2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_pipeline_barrier2 = fp; }
    unsafe { (fp.0)(command_buffer, dependency_info) }
}
#[cfg(feature = "Allow1_3APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_queue_submit2(queue: VkQueue, submit_count: u32, submits: *const VkSubmitInfo2, fence: Option<VkFence>) -> VkResult {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkQueueSubmit2 = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.queue_submit2 = fp; }
    unsafe { (fp.0)(queue, submit_count, submits, fence) }
}
#[cfg(feature = "Allow1_4APIs")]
#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_push_descriptor_set(command_buffer: VkCommandBuffer, pipeline_bind_point: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptor_write_count: u32, descriptor_writes: *const VkWriteDescriptorSet) {
    use crate::resolver::ResolverInterface;
    let fp: PFN_vkCmdPushDescriptorSet = unsafe { crate::resolver::get_resolver().load_function_unconstrainted() };
    unsafe { FPTBL.cmd_push_descriptor_set = fp; }
    unsafe { (fp.0)(command_buffer, pipeline_bind_point, layout, set, descriptor_write_count, descriptor_writes) }
}
