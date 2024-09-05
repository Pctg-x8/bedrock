use crate::vk::*;
use core::ffi::*;

#[inline(always)]
pub unsafe fn create_instance(create_info: *const VkInstanceCreateInfo, allocator: *const VkAllocationCallbacks, instance_out: *mut VkInstance) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_instance.0)(create_info, allocator, instance_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateInstance(create_info, allocator, instance_out) }
}
#[inline(always)]
pub unsafe fn destroy_instance(instance: VkInstance, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_instance.0)(instance, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyInstance(instance, allocator) }
}
#[inline(always)]
pub unsafe fn enumerate_physical_devices(instance: VkInstance, physical_devices_count_out: *mut u32, physical_devices_out: *mut VkPhysicalDevice) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.enumerate_physical_devices.0)(instance, physical_devices_count_out, physical_devices_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkEnumeratePhysicalDevices(instance, physical_devices_count_out, physical_devices_out) }
}
#[inline(always)]
pub unsafe fn get_physical_device_features(physical_device: VkPhysicalDevice, features_out: *mut VkPhysicalDeviceFeatures) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_features.0)(physical_device, features_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceFeatures(physical_device, features_out) }
}
#[inline(always)]
pub unsafe fn get_physical_device_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, format_properties_out: *mut VkFormatProperties) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_format_properties.0)(physical_device, format, format_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceFormatProperties(physical_device, format, format_properties_out) }
}
#[inline(always)]
pub unsafe fn get_physical_device_image_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, image_type: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, image_format_properties_out: *mut VkImageFormatProperties) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_image_format_properties.0)(physical_device, format, image_type, tiling, usage, flags, image_format_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceImageFormatProperties(physical_device, format, image_type, tiling, usage, flags, image_format_properties_out) }
}
#[inline(always)]
pub unsafe fn get_physical_device_properties(physical_device: VkPhysicalDevice, properties_out: *mut VkPhysicalDeviceProperties) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_properties.0)(physical_device, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceProperties(physical_device, properties_out) }
}
#[inline(always)]
pub unsafe fn get_physical_device_queue_family_properties(physical_device: VkPhysicalDevice, queue_family_properties_count_out: *mut u32, queue_family_properties_out: *mut VkQueueFamilyProperties) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_queue_family_properties.0)(physical_device, queue_family_properties_count_out, queue_family_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceQueueFamilyProperties(physical_device, queue_family_properties_count_out, queue_family_properties_out) }
}
#[inline(always)]
pub unsafe fn get_physical_device_memory_properties(physical_device: VkPhysicalDevice, memory_properties_out: *mut VkPhysicalDeviceMemoryProperties) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_memory_properties.0)(physical_device, memory_properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceMemoryProperties(physical_device, memory_properties_out) }
}
#[inline(always)]
pub unsafe fn get_instance_proc_addr(instance: VkInstance, name: *const c_char) -> Option<PFN_vkVoidFunction> {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_instance_proc_addr.0)(instance, name) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetInstanceProcAddr(instance, name) }
}
#[inline(always)]
pub unsafe fn get_device_proc_addr(device: VkDevice, name: *const c_char) -> Option<PFN_vkVoidFunction> {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_device_proc_addr.0)(device, name) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetDeviceProcAddr(device, name) }
}
#[inline(always)]
pub unsafe fn create_device(physical_device: VkPhysicalDevice, create_info: *const VkDeviceCreateInfo, allocator: *const VkAllocationCallbacks, device_out: *mut VkDevice) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_device.0)(physical_device, create_info, allocator, device_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateDevice(physical_device, create_info, allocator, device_out) }
}
#[inline(always)]
pub unsafe fn destroy_device(device: VkDevice, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_device.0)(device, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyDevice(device, allocator) }
}
#[inline(always)]
pub unsafe fn enumerate_instance_extension_properties(layer_name: *const c_char, property_count_out: *mut u32, properties_out: *mut VkExtensionProperties) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.enumerate_instance_extension_properties.0)(layer_name, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkEnumerateInstanceExtensionProperties(layer_name, property_count_out, properties_out) }
}
#[inline(always)]
pub unsafe fn enumerate_device_extension_properties(physical_device: VkPhysicalDevice, layer_name: *const c_char, property_count_out: *mut u32, properties_out: *mut VkExtensionProperties) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.enumerate_device_extension_properties.0)(physical_device, layer_name, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkEnumerateDeviceExtensionProperties(physical_device, layer_name, property_count_out, properties_out) }
}
#[inline(always)]
pub unsafe fn enumerate_instance_layer_properties(property_count_out: *mut u32, properties_out: *mut VkLayerProperties) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.enumerate_instance_layer_properties.0)(property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkEnumerateInstanceLayerProperties(property_count_out, properties_out) }
}
#[inline(always)]
pub unsafe fn enumerate_device_layer_properties(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkLayerProperties) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.enumerate_device_layer_properties.0)(physical_device, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkEnumerateDeviceLayerProperties(physical_device, property_count_out, properties_out) }
}
#[inline(always)]
pub unsafe fn get_device_queue(device: VkDevice, queue_family_index: u32, queue_index: u32, queue_out: *mut VkQueue) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_device_queue.0)(device, queue_family_index, queue_index, queue_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetDeviceQueue(device, queue_family_index, queue_index, queue_out) }
}
#[inline(always)]
pub unsafe fn queue_submit(queue: VkQueue, submit_count: u32, submits: *const VkSubmitInfo, fence: VkFence) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.queue_submit.0)(queue, submit_count, submits, fence) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkQueueSubmit(queue, submit_count, submits, fence) }
}
#[inline(always)]
pub unsafe fn queue_wait_idle(queue: VkQueue) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.queue_wait_idle.0)(queue) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkQueueWaitIdle(queue) }
}
#[inline(always)]
pub unsafe fn device_wait_idle(device: VkDevice) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.device_wait_idle.0)(device) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDeviceWaitIdle(device) }
}
#[inline(always)]
pub unsafe fn allocate_memory(device: VkDevice, allocate_info: *const VkMemoryAllocateInfo, allocator: *const VkAllocationCallbacks, memory_out: *mut VkDeviceMemory) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.allocate_memory.0)(device, allocate_info, allocator, memory_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkAllocateMemory(device, allocate_info, allocator, memory_out) }
}
#[inline(always)]
pub unsafe fn free_memory(device: VkDevice, memory: VkDeviceMemory, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.free_memory.0)(device, memory, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkFreeMemory(device, memory, allocator) }
}
#[inline(always)]
pub unsafe fn map_memory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, data_ptr_out: *mut *mut c_void) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.map_memory.0)(device, memory, offset, size, flags, data_ptr_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkMapMemory(device, memory, offset, size, flags, data_ptr_out) }
}
#[inline(always)]
pub unsafe fn unmap_memory(device: VkDevice, memory: VkDeviceMemory) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.unmap_memory.0)(device, memory) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkUnmapMemory(device, memory) }
}
#[inline(always)]
pub unsafe fn flush_mapped_memory_ranges(device: VkDevice, memory_range_count: u32, memory_ranges: *const VkMappedMemoryRange) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.flush_mapped_memory_ranges.0)(device, memory_range_count, memory_ranges) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkFlushMappedMemoryRanges(device, memory_range_count, memory_ranges) }
}
#[inline(always)]
pub unsafe fn invalidate_mapped_memory_ranges(device: VkDevice, memory_range_count: u32, memory_ranges: *const VkMappedMemoryRange) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.invalidate_mapped_memory_ranges.0)(device, memory_range_count, memory_ranges) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkInvalidateMappedMemoryRanges(device, memory_range_count, memory_ranges) }
}
#[inline(always)]
pub unsafe fn get_device_memory_commitment(device: VkDevice, memory: VkDeviceMemory, committed_memory_bytes_out: *mut VkDeviceSize) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_device_memory_commitment.0)(device, memory, committed_memory_bytes_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetDeviceMemoryCommitment(device, memory, committed_memory_bytes_out) }
}
#[inline(always)]
pub unsafe fn bind_buffer_memory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memory_offset: VkDeviceSize) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.bind_buffer_memory.0)(device, buffer, memory, memory_offset) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkBindBufferMemory(device, buffer, memory, memory_offset) }
}
#[inline(always)]
pub unsafe fn bind_image_memory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memory_offset: VkDeviceSize) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.bind_image_memory.0)(device, image, memory, memory_offset) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkBindImageMemory(device, image, memory, memory_offset) }
}
#[inline(always)]
pub unsafe fn get_buffer_memory_requirements(device: VkDevice, buffer: VkBuffer, memory_requirements_out: *mut VkMemoryRequirements) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_buffer_memory_requirements.0)(device, buffer, memory_requirements_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetBufferMemoryRequirements(device, buffer, memory_requirements_out) }
}
#[inline(always)]
pub unsafe fn get_image_memory_requirements(device: VkDevice, image: VkImage, memory_requirements_out: *mut VkMemoryRequirements) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_image_memory_requirements.0)(device, image, memory_requirements_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetImageMemoryRequirements(device, image, memory_requirements_out) }
}
#[inline(always)]
pub unsafe fn get_image_sparse_memory_requirements(device: VkDevice, image: VkImage, sparse_memory_requirement_count_out: *mut u32, sparse_memory_requirements_out: *mut VkSparseImageMemoryRequirements) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_image_sparse_memory_requirements.0)(device, image, sparse_memory_requirement_count_out, sparse_memory_requirements_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetImageSparseMemoryRequirements(device, image, sparse_memory_requirement_count_out, sparse_memory_requirements_out) }
}
#[inline(always)]
pub unsafe fn get_physical_device_sparse_image_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling, property_count_out: *mut u32, properties_out: *mut VkSparseImageFormatProperties) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_sparse_image_format_properties.0)(physical_device, format, r#type, samples, usage, tiling, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceSparseImageFormatProperties(physical_device, format, r#type, samples, usage, tiling, property_count_out, properties_out) }
}
#[inline(always)]
pub unsafe fn queue_bind_sparse(queue: VkQueue, bind_info_count: u32, bind_info: *const VkBindSparseInfo, fence: VkFence) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.queue_bind_sparse.0)(queue, bind_info_count, bind_info, fence) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkQueueBindSparse(queue, bind_info_count, bind_info, fence) }
}
#[inline(always)]
pub unsafe fn create_fence(device: VkDevice, create_info: *const VkFenceCreateInfo, allocator: *const VkAllocationCallbacks, fence_out: *mut VkFence) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_fence.0)(device, create_info, allocator, fence_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateFence(device, create_info, allocator, fence_out) }
}
#[inline(always)]
pub unsafe fn destroy_fence(device: VkDevice, fence: VkFence, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_fence.0)(device, fence, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyFence(device, fence, allocator) }
}
#[inline(always)]
pub unsafe fn reset_fences(device: VkDevice, fence_count: u32, fences: *const VkFence) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.reset_fences.0)(device, fence_count, fences) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkResetFences(device, fence_count, fences) }
}
#[inline(always)]
pub unsafe fn get_fence_status(device: VkDevice, fence: VkFence) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_fence_status.0)(device, fence) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetFenceStatus(device, fence) }
}
#[inline(always)]
pub unsafe fn wait_for_fences(device: VkDevice, fence_count: u32, fences: *const VkFence, wait_all: VkBool32, timeout: u64) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.wait_for_fences.0)(device, fence_count, fences, wait_all, timeout) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkWaitForFences(device, fence_count, fences, wait_all, timeout) }
}
#[inline(always)]
pub unsafe fn create_semaphore(device: VkDevice, create_info: *const VkSemaphoreCreateInfo, allocator: *const VkAllocationCallbacks, semaphore_out: *mut VkSemaphore) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_semaphore.0)(device, create_info, allocator, semaphore_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateSemaphore(device, create_info, allocator, semaphore_out) }
}
#[inline(always)]
pub unsafe fn destroy_semaphore(device: VkDevice, semaphore: VkSemaphore, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_semaphore.0)(device, semaphore, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroySemaphore(device, semaphore, allocator) }
}
#[inline(always)]
pub unsafe fn create_event(device: VkDevice, create_info: *const VkEventCreateInfo, allocator: *const VkAllocationCallbacks, event_out: *mut VkEvent) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_event.0)(device, create_info, allocator, event_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateEvent(device, create_info, allocator, event_out) }
}
#[inline(always)]
pub unsafe fn destroy_event(device: VkDevice, event: VkEvent, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_event.0)(device, event, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyEvent(device, event, allocator) }
}
#[inline(always)]
pub unsafe fn get_event_status(device: VkDevice, event: VkEvent) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_event_status.0)(device, event) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetEventStatus(device, event) }
}
#[inline(always)]
pub unsafe fn set_event(device: VkDevice, event: VkEvent) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.set_event.0)(device, event) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkSetEvent(device, event) }
}
#[inline(always)]
pub unsafe fn reset_event(device: VkDevice, event: VkEvent) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.reset_event.0)(device, event) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkResetEvent(device, event) }
}
#[inline(always)]
pub unsafe fn create_query_pool(device: VkDevice, create_info: *const VkQueryPoolCreateInfo, allocator: *const VkAllocationCallbacks, query_pool_out: *mut VkQueryPool) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_query_pool.0)(device, create_info, allocator, query_pool_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateQueryPool(device, create_info, allocator, query_pool_out) }
}
#[inline(always)]
pub unsafe fn destroy_query_pool(device: VkDevice, query_pool: VkQueryPool, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_query_pool.0)(device, query_pool, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyQueryPool(device, query_pool, allocator) }
}
#[inline(always)]
pub unsafe fn get_query_pool_results(device: VkDevice, query_pool: VkQueryPool, first_query: u32, query_count: u32, data_size: usize, data_out: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_query_pool_results.0)(device, query_pool, first_query, query_count, data_size, data_out, stride, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetQueryPoolResults(device, query_pool, first_query, query_count, data_size, data_out, stride, flags) }
}
#[inline(always)]
pub unsafe fn create_buffer(device: VkDevice, create_info: *const VkBufferCreateInfo, allocator: *const VkAllocationCallbacks, buffer_out: *mut VkBuffer) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_buffer.0)(device, create_info, allocator, buffer_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateBuffer(device, create_info, allocator, buffer_out) }
}
#[inline(always)]
pub unsafe fn destroy_buffer(device: VkDevice, buffer: VkBuffer, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_buffer.0)(device, buffer, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyBuffer(device, buffer, allocator) }
}
#[inline(always)]
pub unsafe fn create_buffer_view(device: VkDevice, create_info: *const VkBufferViewCreateInfo, allocator: *const VkAllocationCallbacks, view_out: *mut VkBufferView) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_buffer_view.0)(device, create_info, allocator, view_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateBufferView(device, create_info, allocator, view_out) }
}
#[inline(always)]
pub unsafe fn destroy_buffer_view(device: VkDevice, buffer_view: VkBufferView, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_buffer_view.0)(device, buffer_view, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyBufferView(device, buffer_view, allocator) }
}
#[inline(always)]
pub unsafe fn create_image(device: VkDevice, create_info: *const VkImageCreateInfo, allocator: *const VkAllocationCallbacks, image_out: *mut VkImage) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_image.0)(device, create_info, allocator, image_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateImage(device, create_info, allocator, image_out) }
}
#[inline(always)]
pub unsafe fn destroy_image(device: VkDevice, image: VkImage, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_image.0)(device, image, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyImage(device, image, allocator) }
}
#[inline(always)]
pub unsafe fn get_image_subresource_layout(device: VkDevice, image: VkImage, subresource: *const VkImageSubresource, layout_out: *mut VkSubresourceLayout) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_image_subresource_layout.0)(device, image, subresource, layout_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetImageSubresourceLayout(device, image, subresource, layout_out) }
}
#[inline(always)]
pub unsafe fn create_image_view(device: VkDevice, create_info: *const VkImageViewCreateInfo, allocator: *const VkAllocationCallbacks, view_out: *mut VkImageView) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_image_view.0)(device, create_info, allocator, view_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateImageView(device, create_info, allocator, view_out) }
}
#[inline(always)]
pub unsafe fn destroy_image_view(device: VkDevice, image_view: VkImageView, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_image_view.0)(device, image_view, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyImageView(device, image_view, allocator) }
}
#[inline(always)]
pub unsafe fn create_shader_module(device: VkDevice, create_info: *const VkShaderModuleCreateInfo, allocator: *const VkAllocationCallbacks, shader_module_out: *mut VkShaderModule) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_shader_module.0)(device, create_info, allocator, shader_module_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateShaderModule(device, create_info, allocator, shader_module_out) }
}
#[inline(always)]
pub unsafe fn destroy_shader_module(device: VkDevice, shader_module: VkShaderModule, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_shader_module.0)(device, shader_module, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyShaderModule(device, shader_module, allocator) }
}
#[inline(always)]
pub unsafe fn create_pipeline_cache(device: VkDevice, create_info: *const VkPipelineCacheCreateInfo, allocator: *const VkAllocationCallbacks, pipeline_cache_out: *mut VkPipelineCache) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_pipeline_cache.0)(device, create_info, allocator, pipeline_cache_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreatePipelineCache(device, create_info, allocator, pipeline_cache_out) }
}
#[inline(always)]
pub unsafe fn destroy_pipeline_cache(device: VkDevice, pipeline_cache: VkPipelineCache, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_pipeline_cache.0)(device, pipeline_cache, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyPipelineCache(device, pipeline_cache, allocator) }
}
#[inline(always)]
pub unsafe fn get_pipeline_cache_data(device: VkDevice, pipeline_cache: VkPipelineCache, data_size_out: *mut usize, data_out: *mut c_void) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_pipeline_cache_data.0)(device, pipeline_cache, data_size_out, data_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPipelineCacheData(device, pipeline_cache, data_size_out, data_out) }
}
#[inline(always)]
pub unsafe fn merge_pipeline_caches(device: VkDevice, dst_cache: VkPipelineCache, src_cache_count: u32, src_caches: *const VkPipelineCache) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.merge_pipeline_caches.0)(device, dst_cache, src_cache_count, src_caches) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkMergePipelineCaches(device, dst_cache, src_cache_count, src_caches) }
}
#[inline(always)]
pub unsafe fn create_graphics_pipelines(device: VkDevice, pipeline_cache: VkPipelineCache, create_info_count: u32, create_infos: *const VkGraphicsPipelineCreateInfo, allocator: *const VkAllocationCallbacks, pipelines_out: *mut VkPipeline) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_graphics_pipelines.0)(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateGraphicsPipelines(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out) }
}
#[inline(always)]
pub unsafe fn create_compute_pipelines(device: VkDevice, pipeline_cache: VkPipelineCache, create_info_count: u32, create_infos: *const VkComputePipelineCreateInfo, allocator: *const VkAllocationCallbacks, pipelines_out: *mut VkPipeline) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_compute_pipelines.0)(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateComputePipelines(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out) }
}
#[inline(always)]
pub unsafe fn destroy_pipeline(device: VkDevice, pipeline: VkPipeline, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_pipeline.0)(device, pipeline, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyPipeline(device, pipeline, allocator) }
}
#[inline(always)]
pub unsafe fn create_pipeline_layout(device: VkDevice, create_info: *const VkPipelineLayoutCreateInfo, allocator: *const VkAllocationCallbacks, pipeline_layout_out: *mut VkPipelineLayout) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_pipeline_layout.0)(device, create_info, allocator, pipeline_layout_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreatePipelineLayout(device, create_info, allocator, pipeline_layout_out) }
}
#[inline(always)]
pub unsafe fn destroy_pipeline_layout(device: VkDevice, pipeline_layout: VkPipelineLayout, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_pipeline_layout.0)(device, pipeline_layout, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyPipelineLayout(device, pipeline_layout, allocator) }
}
#[inline(always)]
pub unsafe fn create_sampler(device: VkDevice, create_info: *const VkSamplerCreateInfo, allocator: *const VkAllocationCallbacks, sampler_out: *mut VkSampler) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_sampler.0)(device, create_info, allocator, sampler_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateSampler(device, create_info, allocator, sampler_out) }
}
#[inline(always)]
pub unsafe fn destroy_sampler(device: VkDevice, sampler: VkSampler, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_sampler.0)(device, sampler, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroySampler(device, sampler, allocator) }
}
#[inline(always)]
pub unsafe fn create_descriptor_set_layout(device: VkDevice, create_info: *const VkDescriptorSetLayoutCreateInfo, allocator: *const VkAllocationCallbacks, set_layout_out: *mut VkDescriptorSetLayout) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_descriptor_set_layout.0)(device, create_info, allocator, set_layout_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateDescriptorSetLayout(device, create_info, allocator, set_layout_out) }
}
#[inline(always)]
pub unsafe fn destroy_descriptor_set_layout(device: VkDevice, descriptor_set_layout: VkDescriptorSetLayout, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_descriptor_set_layout.0)(device, descriptor_set_layout, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyDescriptorSetLayout(device, descriptor_set_layout, allocator) }
}
#[inline(always)]
pub unsafe fn create_descriptor_pool(device: VkDevice, create_info: *const VkDescriptorPoolCreateInfo, allocator: *const VkAllocationCallbacks, descriptor_pool_out: *mut VkDescriptorPool) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_descriptor_pool.0)(device, create_info, allocator, descriptor_pool_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateDescriptorPool(device, create_info, allocator, descriptor_pool_out) }
}
#[inline(always)]
pub unsafe fn destroy_descriptor_pool(device: VkDevice, descriptor_pool: VkDescriptorPool, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_descriptor_pool.0)(device, descriptor_pool, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyDescriptorPool(device, descriptor_pool, allocator) }
}
#[inline(always)]
pub unsafe fn reset_descriptor_pool(device: VkDevice, descriptor_pool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.reset_descriptor_pool.0)(device, descriptor_pool, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkResetDescriptorPool(device, descriptor_pool, flags) }
}
#[inline(always)]
pub unsafe fn allocate_descriptor_sets(device: VkDevice, allocate_info: *const VkDescriptorSetAllocateInfo, descriptor_sets_out: *mut VkDescriptorSet) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.allocate_descriptor_sets.0)(device, allocate_info, descriptor_sets_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkAllocateDescriptorSets(device, allocate_info, descriptor_sets_out) }
}
#[inline(always)]
pub unsafe fn free_descriptor_sets(device: VkDevice, descriptor_pool: VkDescriptorPool, descriptor_set_count: u32, descriptor_sets: *const VkDescriptorSet) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.free_descriptor_sets.0)(device, descriptor_pool, descriptor_set_count, descriptor_sets) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkFreeDescriptorSets(device, descriptor_pool, descriptor_set_count, descriptor_sets) }
}
#[inline(always)]
pub unsafe fn update_descriptor_sets(device: VkDevice, descriptor_write_count: u32, descriptor_writes: *const VkWriteDescriptorSet, descriptor_copy_count: u32, descriptor_copies: *const VkCopyDescriptorSet) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.update_descriptor_sets.0)(device, descriptor_write_count, descriptor_writes, descriptor_copy_count, descriptor_copies) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkUpdateDescriptorSets(device, descriptor_write_count, descriptor_writes, descriptor_copy_count, descriptor_copies) }
}
#[inline(always)]
pub unsafe fn create_framebuffer(device: VkDevice, create_info: *const VkFramebufferCreateInfo, allocator: *const VkAllocationCallbacks, framebuffer_out: *mut VkFramebuffer) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_framebuffer.0)(device, create_info, allocator, framebuffer_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateFramebuffer(device, create_info, allocator, framebuffer_out) }
}
#[inline(always)]
pub unsafe fn destroy_framebuffer(device: VkDevice, framebuffer: VkFramebuffer, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_framebuffer.0)(device, framebuffer, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyFramebuffer(device, framebuffer, allocator) }
}
#[inline(always)]
pub unsafe fn create_render_pass(device: VkDevice, create_info: *const VkRenderPassCreateInfo, allocator: *const VkAllocationCallbacks, render_pass_out: *mut VkRenderPass) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_render_pass.0)(device, create_info, allocator, render_pass_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateRenderPass(device, create_info, allocator, render_pass_out) }
}
#[inline(always)]
pub unsafe fn destroy_render_pass(device: VkDevice, render_pass: VkRenderPass, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_render_pass.0)(device, render_pass, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyRenderPass(device, render_pass, allocator) }
}
#[inline(always)]
pub unsafe fn get_render_area_granularity(device: VkDevice, render_pass: VkRenderPass, granularity_out: *mut VkExtent2D) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_render_area_granularity.0)(device, render_pass, granularity_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetRenderAreaGranularity(device, render_pass, granularity_out) }
}
#[inline(always)]
pub unsafe fn create_command_pool(device: VkDevice, create_info: *const VkCommandPoolCreateInfo, allocator: *const VkAllocationCallbacks, command_pool_out: *mut VkCommandPool) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_command_pool.0)(device, create_info, allocator, command_pool_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateCommandPool(device, create_info, allocator, command_pool_out) }
}
#[inline(always)]
pub unsafe fn destroy_command_pool(device: VkDevice, command_pool: VkCommandPool, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_command_pool.0)(device, command_pool, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroyCommandPool(device, command_pool, allocator) }
}
#[inline(always)]
pub unsafe fn reset_command_pool(device: VkDevice, command_pool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.reset_command_pool.0)(device, command_pool, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkResetCommandPool(device, command_pool, flags) }
}
#[inline(always)]
pub unsafe fn allocate_command_buffers(device: VkDevice, allocate_info: *const VkCommandBufferAllocateInfo, command_buffers_out: *mut VkCommandBuffer) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.allocate_command_buffers.0)(device, allocate_info, command_buffers_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkAllocateCommandBuffers(device, allocate_info, command_buffers_out) }
}
#[inline(always)]
pub unsafe fn free_command_buffers(device: VkDevice, command_pool: VkCommandPool, command_buffer_count: u32, command_buffers: *const VkCommandBuffer) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.free_command_buffers.0)(device, command_pool, command_buffer_count, command_buffers) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkFreeCommandBuffers(device, command_pool, command_buffer_count, command_buffers) }
}
#[inline(always)]
pub unsafe fn begin_command_buffer(command_buffer: VkCommandBuffer, begin_info: *const VkCommandBufferBeginInfo) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.begin_command_buffer.0)(command_buffer, begin_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkBeginCommandBuffer(command_buffer, begin_info) }
}
#[inline(always)]
pub unsafe fn end_command_buffer(command_buffer: VkCommandBuffer) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.end_command_buffer.0)(command_buffer) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkEndCommandBuffer(command_buffer) }
}
#[inline(always)]
pub unsafe fn reset_command_buffer(command_buffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.reset_command_buffer.0)(command_buffer, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkResetCommandBuffer(command_buffer, flags) }
}
#[inline(always)]
pub unsafe fn cmd_bind_pipeline(command_buffer: VkCommandBuffer, pipeline_bind_point: VkPipelineBindPoint, pipeline: VkPipeline) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_bind_pipeline.0)(command_buffer, pipeline_bind_point, pipeline) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdBindPipeline(command_buffer, pipeline_bind_point, pipeline) }
}
#[inline(always)]
pub unsafe fn cmd_set_viewport(command_buffer: VkCommandBuffer, first_viewport: u32, viewport_count: u32, viewports: *const VkViewport) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_set_viewport.0)(command_buffer, first_viewport, viewport_count, viewports) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdSetViewport(command_buffer, first_viewport, viewport_count, viewports) }
}
#[inline(always)]
pub unsafe fn cmd_set_scissor(command_buffer: VkCommandBuffer, first_scissor: u32, scissor_count: u32, scissors: *const VkRect2D) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_set_scissor.0)(command_buffer, first_scissor, scissor_count, scissors) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdSetScissor(command_buffer, first_scissor, scissor_count, scissors) }
}
#[inline(always)]
pub unsafe fn cmd_set_line_width(command_buffer: VkCommandBuffer, line_width: c_float) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_set_line_width.0)(command_buffer, line_width) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdSetLineWidth(command_buffer, line_width) }
}
#[inline(always)]
pub unsafe fn cmd_set_depth_bias(command_buffer: VkCommandBuffer, depth_bias_constant_factor: c_float, depth_bias_clamp: c_float, depth_bias_slope_factor: c_float) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_set_depth_bias.0)(command_buffer, depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdSetDepthBias(command_buffer, depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor) }
}
#[inline(always)]
pub unsafe fn cmd_set_blend_constants(command_buffer: VkCommandBuffer, blend_constants: *const c_float) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_set_blend_constants.0)(command_buffer, blend_constants) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdSetBlendConstants(command_buffer, blend_constants) }
}
#[inline(always)]
pub unsafe fn cmd_set_depth_bounds(command_buffer: VkCommandBuffer, min_depth_bounds: c_float, max_depth_bounds: c_float) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_set_depth_bounds.0)(command_buffer, min_depth_bounds, max_depth_bounds) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdSetDepthBounds(command_buffer, min_depth_bounds, max_depth_bounds) }
}
#[inline(always)]
pub unsafe fn cmd_set_stencil_compare_mask(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, compare_mask: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_set_stencil_compare_mask.0)(command_buffer, face_mask, compare_mask) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdSetStencilCompareMask(command_buffer, face_mask, compare_mask) }
}
#[inline(always)]
pub unsafe fn cmd_set_stencil_write_mask(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, write_mask: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_set_stencil_write_mask.0)(command_buffer, face_mask, write_mask) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdSetStencilWriteMask(command_buffer, face_mask, write_mask) }
}
#[inline(always)]
pub unsafe fn cmd_set_stencil_reference(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, reference: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_set_stencil_reference.0)(command_buffer, face_mask, reference) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdSetStencilReference(command_buffer, face_mask, reference) }
}
#[inline(always)]
pub unsafe fn cmd_bind_descriptor_sets(command_buffer: VkCommandBuffer, pipeline_bind_point: VkPipelineBindPoint, layout: VkPipelineLayout, first_set: u32, descriptor_set_count: u32, descriptor_sets: *const VkDescriptorSet, dynamic_offset_count: u32, dynamic_offsets: *const u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_bind_descriptor_sets.0)(command_buffer, pipeline_bind_point, layout, first_set, descriptor_set_count, descriptor_sets, dynamic_offset_count, dynamic_offsets) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdBindDescriptorSets(command_buffer, pipeline_bind_point, layout, first_set, descriptor_set_count, descriptor_sets, dynamic_offset_count, dynamic_offsets) }
}
#[inline(always)]
pub unsafe fn cmd_bind_index_buffer(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, index_type: VkIndexType) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_bind_index_buffer.0)(command_buffer, buffer, offset, index_type) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdBindIndexBuffer(command_buffer, buffer, offset, index_type) }
}
#[inline(always)]
pub unsafe fn cmd_bind_vertex_buffers(command_buffer: VkCommandBuffer, first_binding: u32, binding_count: u32, buffers: *const VkBuffer, offsets: *const VkDeviceSize) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_bind_vertex_buffers.0)(command_buffer, first_binding, binding_count, buffers, offsets) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdBindVertexBuffers(command_buffer, first_binding, binding_count, buffers, offsets) }
}
#[inline(always)]
pub unsafe fn cmd_draw(command_buffer: VkCommandBuffer, vertex_count: u32, instance_count: u32, first_vertex: u32, first_index: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_draw.0)(command_buffer, vertex_count, instance_count, first_vertex, first_index) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdDraw(command_buffer, vertex_count, instance_count, first_vertex, first_index) }
}
#[inline(always)]
pub unsafe fn cmd_draw_indexed(command_buffer: VkCommandBuffer, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_draw_indexed.0)(command_buffer, index_count, instance_count, first_index, vertex_offset, first_instance) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdDrawIndexed(command_buffer, index_count, instance_count, first_index, vertex_offset, first_instance) }
}
#[inline(always)]
pub unsafe fn cmd_draw_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, draw_count: u32, stride: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_draw_indirect.0)(command_buffer, buffer, offset, draw_count, stride) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdDrawIndirect(command_buffer, buffer, offset, draw_count, stride) }
}
#[inline(always)]
pub unsafe fn cmd_draw_indexed_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, draw_count: u32, stride: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_draw_indexed_indirect.0)(command_buffer, buffer, offset, draw_count, stride) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdDrawIndexedIndirect(command_buffer, buffer, offset, draw_count, stride) }
}
#[inline(always)]
pub unsafe fn cmd_dispatch(command_buffer: VkCommandBuffer, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_dispatch.0)(command_buffer, group_count_x, group_count_y, group_count_z) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdDispatch(command_buffer, group_count_x, group_count_y, group_count_z) }
}
#[inline(always)]
pub unsafe fn cmd_dispatch_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_dispatch_indirect.0)(command_buffer, buffer, offset) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdDispatchIndirect(command_buffer, buffer, offset) }
}
#[inline(always)]
pub unsafe fn cmd_copy_buffer(command_buffer: VkCommandBuffer, src_buffer: VkBuffer, dst_buffer: VkBuffer, region_count: u32, regions: *const VkBufferCopy) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_copy_buffer.0)(command_buffer, src_buffer, dst_buffer, region_count, regions) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdCopyBuffer(command_buffer, src_buffer, dst_buffer, region_count, regions) }
}
#[inline(always)]
pub unsafe fn cmd_copy_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageCopy) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_copy_image.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdCopyImage(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions) }
}
#[inline(always)]
pub unsafe fn cmd_blit_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageBlit, filter: VkFilter) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_blit_image.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions, filter) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdBlitImage(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions, filter) }
}
#[inline(always)]
pub unsafe fn cmd_copy_buffer_to_image(command_buffer: VkCommandBuffer, src_buffer: VkBuffer, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkBufferImageCopy) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_copy_buffer_to_image.0)(command_buffer, src_buffer, dst_image, dst_image_layout, region_count, regions) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdCopyBufferToImage(command_buffer, src_buffer, dst_image, dst_image_layout, region_count, regions) }
}
#[inline(always)]
pub unsafe fn cmd_copy_image_to_buffer(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_buffer: VkBuffer, region_count: u32, regions: *const VkBufferImageCopy) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_copy_image_to_buffer.0)(command_buffer, src_image, src_image_layout, dst_buffer, region_count, regions) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdCopyImageToBuffer(command_buffer, src_image, src_image_layout, dst_buffer, region_count, regions) }
}
#[inline(always)]
pub unsafe fn cmd_update_buffer(command_buffer: VkCommandBuffer, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, data_size: VkDeviceSize, data: *const c_void) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_update_buffer.0)(command_buffer, dst_buffer, dst_offset, data_size, data) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdUpdateBuffer(command_buffer, dst_buffer, dst_offset, data_size, data) }
}
#[inline(always)]
pub unsafe fn cmd_fill_buffer(command_buffer: VkCommandBuffer, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, size: VkDeviceSize, data: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_fill_buffer.0)(command_buffer, dst_buffer, dst_offset, size, data) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdFillBuffer(command_buffer, dst_buffer, dst_offset, size, data) }
}
#[inline(always)]
pub unsafe fn cmd_clear_color_image(command_buffer: VkCommandBuffer, image: VkImage, image_layout: VkImageLayout, color: *const VkClearColorValue, range_count: u32, ranges: *const VkImageSubresourceRange) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_clear_color_image.0)(command_buffer, image, image_layout, color, range_count, ranges) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdClearColorImage(command_buffer, image, image_layout, color, range_count, ranges) }
}
#[inline(always)]
pub unsafe fn cmd_clear_depth_stencil_image(command_buffer: VkCommandBuffer, image: VkImage, image_layout: VkImageLayout, depth_stencil: *const VkClearDepthStencilValue, range_count: u32, ranges: *const VkImageSubresourceRange) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_clear_depth_stencil_image.0)(command_buffer, image, image_layout, depth_stencil, range_count, ranges) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdClearDepthStencilImage(command_buffer, image, image_layout, depth_stencil, range_count, ranges) }
}
#[inline(always)]
pub unsafe fn cmd_clear_attachments(command_buffer: VkCommandBuffer, attachment_count: u32, attachments: *const VkClearAttachment, rect_count: u32, rects: *const VkClearRect) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_clear_attachments.0)(command_buffer, attachment_count, attachments, rect_count, rects) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdClearAttachments(command_buffer, attachment_count, attachments, rect_count, rects) }
}
#[inline(always)]
pub unsafe fn cmd_resolve_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageResolve) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_resolve_image.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdResolveImage(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions) }
}
#[inline(always)]
pub unsafe fn cmd_set_event(command_buffer: VkCommandBuffer, event: VkEvent, stage_mask: VkPipelineStageFlags) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_set_event.0)(command_buffer, event, stage_mask) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdSetEvent(command_buffer, event, stage_mask) }
}
#[inline(always)]
pub unsafe fn cmd_reset_event(command_buffer: VkCommandBuffer, event: VkEvent, stage_mask: VkPipelineStageFlags) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_reset_event.0)(command_buffer, event, stage_mask) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdResetEvent(command_buffer, event, stage_mask) }
}
#[inline(always)]
pub unsafe fn cmd_wait_events(command_buffer: VkCommandBuffer, event_count: u32, events: *const VkEvent, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags, memory_barrier_count: u32, memory_barriers: *const VkMemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const VkBufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const VkImageMemoryBarrier) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_wait_events.0)(command_buffer, event_count, events, src_stage_mask, dst_stage_mask, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdWaitEvents(command_buffer, event_count, events, src_stage_mask, dst_stage_mask, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers) }
}
#[inline(always)]
pub unsafe fn cmd_pipeline_barrier(command_buffer: VkCommandBuffer, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags, dependency_flags: VkDependencyFlags, memory_barrier_count: u32, memory_barriers: *const VkMemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const VkBufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const VkImageMemoryBarrier) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_pipeline_barrier.0)(command_buffer, src_stage_mask, dst_stage_mask, dependency_flags, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdPipelineBarrier(command_buffer, src_stage_mask, dst_stage_mask, dependency_flags, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers) }
}
#[inline(always)]
pub unsafe fn cmd_begin_query(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, query: u32, flags: VkQueryControlFlags) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_begin_query.0)(command_buffer, query_pool, query, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdBeginQuery(command_buffer, query_pool, query, flags) }
}
#[inline(always)]
pub unsafe fn cmd_end_query(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, query: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_end_query.0)(command_buffer, query_pool, query) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdEndQuery(command_buffer, query_pool, query) }
}
#[inline(always)]
pub unsafe fn cmd_reset_query_pool(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, first_query: u32, query_count: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_reset_query_pool.0)(command_buffer, query_pool, first_query, query_count) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdResetQueryPool(command_buffer, query_pool, first_query, query_count) }
}
#[inline(always)]
pub unsafe fn cmd_write_timestamp(command_buffer: VkCommandBuffer, pipeline_stage: VkPipelineStageFlags, query_pool: VkQueryPool, query: u32) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_write_timestamp.0)(command_buffer, pipeline_stage, query_pool, query) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdWriteTimestamp(command_buffer, pipeline_stage, query_pool, query) }
}
#[inline(always)]
pub unsafe fn cmd_copy_query_pool_results(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, first_query: u32, query_count: u32, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_copy_query_pool_results.0)(command_buffer, query_pool, first_query, query_count, dst_buffer, dst_offset, stride, flags) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdCopyQueryPoolResults(command_buffer, query_pool, first_query, query_count, dst_buffer, dst_offset, stride, flags) }
}
#[inline(always)]
pub unsafe fn cmd_push_constants(command_buffer: VkCommandBuffer, layout: VkPipelineLayout, stage_flags: VkShaderStageFlags, offset: u32, size: u32, values: *const c_void) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_push_constants.0)(command_buffer, layout, stage_flags, offset, size, values) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdPushConstants(command_buffer, layout, stage_flags, offset, size, values) }
}
#[inline(always)]
pub unsafe fn cmd_begin_render_pass(command_buffer: VkCommandBuffer, render_pass_begin_info: *const VkRenderPassBeginInfo, contents: VkSubpassContents) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_begin_render_pass.0)(command_buffer, render_pass_begin_info, contents) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdBeginRenderPass(command_buffer, render_pass_begin_info, contents) }
}
#[inline(always)]
pub unsafe fn cmd_next_subpass(command_buffer: VkCommandBuffer, contents: VkSubpassContents) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_next_subpass.0)(command_buffer, contents) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdNextSubpass(command_buffer, contents) }
}
#[inline(always)]
pub unsafe fn cmd_end_render_pass(command_buffer: VkCommandBuffer) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_end_render_pass.0)(command_buffer) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdEndRenderPass(command_buffer) }
}
#[inline(always)]
pub unsafe fn cmd_execute_commands(command_buffer: VkCommandBuffer, command_buffer_count: u32, command_buffers: *const VkCommandBuffer) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_execute_commands.0)(command_buffer, command_buffer_count, command_buffers) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdExecuteCommands(command_buffer, command_buffer_count, command_buffers) }
}
#[cfg(feature = "Allow1_1APIs")]
#[inline(always)]
pub unsafe fn enumerate_instance_version(api_version: *mut u32) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.enumerate_instance_version.0)(api_version) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkEnumerateInstanceVersion(api_version) }
}
#[cfg(feature = "VK_KHR_surface")]
#[inline(always)]
pub unsafe fn destroy_surface_khr(instance: VkInstance, surface: VkSurfaceKHR, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_surface_khr.0)(instance, surface, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroySurfaceKHR(instance, surface, allocator) }
}
#[cfg(feature = "VK_KHR_surface")]
#[inline(always)]
pub unsafe fn get_physical_device_surface_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, surface: VkSurfaceKHR, supported_out: *mut VkBool32) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_surface_support_khr.0)(physical_device, queue_family_index, surface, supported_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceSurfaceSupportKHR(physical_device, queue_family_index, surface, supported_out) }
}
#[cfg(feature = "VK_KHR_surface")]
#[inline(always)]
pub unsafe fn get_physical_device_surface_capabilities_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, surface_capabilities_out: *mut VkSurfaceCapabilitiesKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_surface_capabilities_khr.0)(physical_device, surface, surface_capabilities_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, surface, surface_capabilities_out) }
}
#[cfg(feature = "VK_KHR_surface")]
#[inline(always)]
pub unsafe fn get_physical_device_surface_formats_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, surface_format_count_out: *mut u32, surface_formats_out: *mut VkSurfaceFormatKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_surface_formats_khr.0)(physical_device, surface, surface_format_count_out, surface_formats_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, surface, surface_format_count_out, surface_formats_out) }
}
#[cfg(feature = "VK_KHR_surface")]
#[inline(always)]
pub unsafe fn get_physical_device_surface_present_modes_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, present_mode_count_out: *mut u32, present_modes_out: *mut VkPresentModeKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_surface_present_modes_khr.0)(physical_device, surface, present_mode_count_out, present_modes_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device, surface, present_mode_count_out, present_modes_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[inline(always)]
pub unsafe fn create_swapchain_khr(device: VkDevice, create_info: *const VkSwapchainCreateInfoKHR, allocator: *const VkAllocationCallbacks, swapchain_out: *mut VkSwapchainKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_swapchain_khr.0)(device, create_info, allocator, swapchain_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateSwapchainKHR(device, create_info, allocator, swapchain_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[inline(always)]
pub unsafe fn destroy_swapchain_khr(device: VkDevice, swapchain: VkSwapchainKHR, allocator: *const VkAllocationCallbacks) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.destroy_swapchain_khr.0)(device, swapchain, allocator) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkDestroySwapchainKHR(device, swapchain, allocator) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[inline(always)]
pub unsafe fn get_swapchain_images_khr(device: VkDevice, swapchain: VkSwapchainKHR, swapchain_image_count_out: *mut u32, swapchain_images_out: *mut VkImage) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_swapchain_images_khr.0)(device, swapchain, swapchain_image_count_out, swapchain_images_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetSwapchainImagesKHR(device, swapchain, swapchain_image_count_out, swapchain_images_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[inline(always)]
pub unsafe fn acquire_next_image_khr(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, image_index_out: *mut u32) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.acquire_next_image_khr.0)(device, swapchain, timeout, semaphore, fence, image_index_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkAcquireNextImageKHR(device, swapchain, timeout, semaphore, fence, image_index_out) }
}
#[cfg(feature = "VK_KHR_swapchain")]
#[inline(always)]
pub unsafe fn queue_present_khr(queue: VkQueue, present_info: *const VkPresentInfoKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.queue_present_khr.0)(queue, present_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkQueuePresentKHR(queue, present_info) }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[inline(always)]
pub unsafe fn create_xlib_surface_khr(instance: VkInstance, create_info: *const VkXlibSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_xlib_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateXlibSurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[inline(always)]
pub unsafe fn get_physical_device_xlib_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, dpy: *mut x11::xlib::Display, visual_id: x11::xlib::VisualID) -> VkBool32 {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_xlib_presentation_support_khr.0)(physical_device, queue_family_index, dpy, visual_id) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceXlibPresentationSupportKHR(physical_device, queue_family_index, dpy, visual_id) }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[inline(always)]
pub unsafe fn create_xcb_surface_khr(instance: VkInstance, create_info: *const VkXcbSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_xcb_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateXcbSurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[inline(always)]
pub unsafe fn get_physical_device_xcb_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, connection: *mut xcb::ffi::xcb_connection_t, visual_id: xcb::x::VisualId) -> VkBool32 {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_xcb_presentation_support_khr.0)(physical_device, queue_family_index, connection, visual_id) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceXcbPresentationSupportKHR(physical_device, queue_family_index, connection, visual_id) }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[inline(always)]
pub unsafe fn create_wayland_surface_khr(instance: VkInstance, create_info: *const VkWaylandSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_wayland_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateWaylandSurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[inline(always)]
pub unsafe fn get_physical_device_wayland_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, display: *mut c_void) -> VkBool32 {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_wayland_presentation_support_khr.0)(physical_device, queue_family_index, display) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceWaylandPresentationSupportKHR(physical_device, queue_family_index, display) }
}
#[cfg(feature = "VK_KHR_android_surface")]
#[inline(always)]
pub unsafe fn create_android_surface_khr(instance: VkInstance, create_info: *const VkAndroidSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_android_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateAndroidSurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[inline(always)]
pub unsafe fn create_win32_surface_khr(instance: VkInstance, create_info: *const VkWin32SurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_win32_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateWin32SurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[inline(always)]
pub unsafe fn get_physical_device_win32_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32) -> VkBool32 {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_win32_presentation_support_khr.0)(physical_device, queue_family_index) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceWin32PresentationSupportKHR(physical_device, queue_family_index) }
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[inline(always)]
pub unsafe fn create_macos_surface_mvk(instance: VkInstance, create_info: *const VkMacOSSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_macos_surface_mvk.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateMacOSSurfaceMVK(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[inline(always)]
pub unsafe fn get_physical_device_display_properties_khr(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkDisplayPropertiesKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_display_properties_khr.0)(physical_device, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceDisplayPropertiesKHR(physical_device, property_count_out, properties_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[inline(always)]
pub unsafe fn get_physical_device_display_plane_properties_khr(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkDisplayPlanePropertiesKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_physical_device_display_plane_properties_khr.0)(physical_device, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physical_device, property_count_out, properties_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[inline(always)]
pub unsafe fn get_display_plane_supported_displays_khr(physical_device: VkPhysicalDevice, plane_index: u32, display_count_out: *mut u32, displays_out: *mut VkDisplayKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_display_plane_supported_displays_khr.0)(physical_device, plane_index, display_count_out, displays_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetDisplayPlaneSupportedDisplaysKHR(physical_device, plane_index, display_count_out, displays_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[inline(always)]
pub unsafe fn get_display_mode_properties_khr(physical_device: VkPhysicalDevice, display: VkDisplayKHR, property_count_out: *mut u32, properties_out: *mut VkDisplayModePropertiesKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_display_mode_properties_khr.0)(physical_device, display, property_count_out, properties_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetDisplayModePropertiesKHR(physical_device, display, property_count_out, properties_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[inline(always)]
pub unsafe fn create_display_mode_khr(physical_device: VkPhysicalDevice, display: VkDisplayKHR, create_info: *const VkDisplayModeCreateInfoKHR, allocator: *const VkAllocationCallbacks, mode_out: *mut VkDisplayModeKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_display_mode_khr.0)(physical_device, display, create_info, allocator, mode_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateDisplayModeKHR(physical_device, display, create_info, allocator, mode_out) }
}
#[cfg(feature = "VK_KHR_display")]
#[inline(always)]
pub unsafe fn get_display_plane_capabilities_khr(physcial_device: VkPhysicalDevice, mode: VkDisplayModeKHR, plane_index: u32, capabilities_out: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.get_display_plane_capabilities_khr.0)(physcial_device, mode, plane_index, capabilities_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkGetDisplayPlaneCapabilitiesKHR(physcial_device, mode, plane_index, capabilities_out) }
}
#[cfg(all(feature = "VK_KHR_display", feature = "VK_KHR_surface"))]
#[inline(always)]
pub unsafe fn create_display_plane_surface_khr(instance: VkInstance, create_info: *const VkDisplaySurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_display_plane_surface_khr.0)(instance, create_info, allocator, surface_out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateDisplayPlaneSurfaceKHR(instance, create_info, allocator, surface_out) }
}
#[cfg(feature = "Allow1_2APIs")]
#[inline(always)]
pub unsafe fn create_render_pass2(device: VkDevice, create_info: *const VkRenderPassCreateInfo2, allocator: *const VkAllocationCallbacks, out: *mut VkRenderPass) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.create_render_pass2.0)(device, create_info, allocator, out) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCreateRenderPass2(device, create_info, allocator, out) }
}
#[cfg(feature = "Allow1_2APIs")]
#[inline(always)]
pub unsafe fn cmd_begin_render_pass2(command_buffer: VkCommandBuffer, begin_info: *const VkRenderPassBeginInfo, begin_subpass_info: *const VkSubpassBeginInfo) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_begin_render_pass2.0)(command_buffer, begin_info, begin_subpass_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdBeginRenderPass2(command_buffer, begin_info, begin_subpass_info) }
}
#[cfg(feature = "Allow1_2APIs")]
#[inline(always)]
pub unsafe fn cmd_next_subpass2(command_buffer: VkCommandBuffer, begin_subpass_info: *const VkSubpassBeginInfo, end_subpass_info: *const VkSubpassEndInfo) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_next_subpass2.0)(command_buffer, begin_subpass_info, end_subpass_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdNextSubpass2(command_buffer, begin_subpass_info, end_subpass_info) }
}
#[cfg(feature = "Allow1_2APIs")]
#[inline(always)]
pub unsafe fn cmd_end_render_pass2(command_buffer: VkCommandBuffer, end_subpass_info: *const VkSubpassEndInfo) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_end_render_pass2.0)(command_buffer, end_subpass_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdEndRenderPass2(command_buffer, end_subpass_info) }
}
#[cfg(feature = "Allow1_3APIs")]
#[inline(always)]
pub unsafe fn cmd_pipeline_barrier2(command_buffer: VkCommandBuffer, dependency_info: *const VkDependencyInfo) {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.cmd_pipeline_barrier2.0)(command_buffer, dependency_info) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkCmdPipelineBarrier2(command_buffer, dependency_info) }
}
#[cfg(feature = "Allow1_3APIs")]
#[inline(always)]
pub unsafe fn queue_submit2(queue: VkQueue, submit_count: u32, submits: *const VkSubmitInfo2, fence: VkFence) -> VkResult {
    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] { (FPTBL.queue_submit2.0)(queue, submit_count, submits, fence) }
    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] { vkQueueSubmit2(queue, submit_count, submits, fence) }
}

#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
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
    #[cfg(feature = "Allow1_2APIs")]
    create_render_pass2: PFN_vkCreateRenderPass2,
    #[cfg(feature = "Allow1_2APIs")]
    cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
    #[cfg(feature = "Allow1_2APIs")]
    cmd_next_subpass2: PFN_vkCmdNextSubpass2,
    #[cfg(feature = "Allow1_2APIs")]
    cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
    #[cfg(feature = "Allow1_3APIs")]
    cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
    #[cfg(feature = "Allow1_3APIs")]
    queue_submit2: PFN_vkQueueSubmit2
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
static mut FPTBL: FunctionPointerTable = FunctionPointerTable::INIT;

#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
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
        #[cfg(feature = "VK_KHR_surface")]
        destroy_surface: PFN_vkDestroySurfaceKHR(stub_destroy_surface_khr),
        #[cfg(feature = "VK_KHR_surface")]
        get_physical_device_surface_support: PFN_vkGetPhysicalDeviceSurfaceSupportKHR(stub_get_physical_device_surface_support_khr),
        #[cfg(feature = "VK_KHR_surface")]
        get_physical_device_surface_capabilities: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(stub_get_physical_device_surface_capabilities_khr),
        #[cfg(feature = "VK_KHR_surface")]
        get_physical_device_surface_formats: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR(stub_get_physical_device_surface_formats_khr),
        #[cfg(feature = "VK_KHR_surface")]
        get_physical_device_surface_present_modes: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR(stub_get_physical_device_surface_present_modes_khr),
        #[cfg(feature = "VK_KHR_swapchain")]
        create_swapchain: PFN_vkCreateSwapchainKHR(stub_create_swapchain_khr),
        #[cfg(feature = "VK_KHR_swapchain")]
        destroy_swapchain: PFN_vkDestroySwapchainKHR(stub_destroy_swapchain_khr),
        #[cfg(feature = "VK_KHR_swapchain")]
        get_swapchain_images: PFN_vkGetSwapchainImagesKHR(stub_get_swapchain_images_khr),
        #[cfg(feature = "VK_KHR_swapchain")]
        acquire_next_image: PFN_vkAcquireNextImageKHR(stub_acquire_next_image_khr),
        #[cfg(feature = "VK_KHR_swapchain")]
        queue_present: PFN_vkQueuePresentKHR(stub_queue_present_khr),
        #[cfg(feature = "VK_KHR_xlib_surface")]
        create_xlib_surface: PFN_vkCreateXlibSurfaceKHR(stub_create_xlib_surface_khr),
        #[cfg(feature = "VK_KHR_xlib_surface")]
        get_physical_device_xlib_presentation_support: PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR(stub_get_physical_device_xlib_presentation_support_khr),
        #[cfg(feature = "VK_KHR_xcb_surface")]
        create_xcb_surface: PFN_vkCreateXcbSurfaceKHR(stub_create_xcb_surface_khr),
        #[cfg(feature = "VK_KHR_xcb_surface")]
        get_physical_device_xcb_presentation_support: PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR(stub_get_physical_device_xcb_presentation_support_khr),
        #[cfg(feature = "VK_KHR_wayland_surface")]
        create_wayland_surface: PFN_vkCreateWaylandSurfaceKHR(stub_create_wayland_surface_khr),
        #[cfg(feature = "VK_KHR_wayland_surface")]
        get_physical_device_wayland_presentation_support: PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR(stub_get_physical_device_wayland_presentation_support_khr),
        #[cfg(feature = "VK_KHR_android_surface")]
        create_android_surface: PFN_vkCreateAndroidSurfaceKHR(stub_create_android_surface_khr),
        #[cfg(feature = "VK_KHR_win32_surface")]
        create_win32_surface: PFN_vkCreateWin32SurfaceKHR(stub_create_win32_surface_khr),
        #[cfg(feature = "VK_KHR_win32_surface")]
        get_physical_device_win32_presentation_support: PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR(stub_get_physical_device_win32_presentation_support_khr),
        #[cfg(feature = "VK_MVK_macos_surface")]
        create_macos_surface: PFN_vkCreateMacOSSurfaceMVK(stub_create_macos_surface_mvk),
        #[cfg(feature = "VK_KHR_display")]
        get_physical_device_display_properties: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR(stub_get_physical_device_display_properties_khr),
        #[cfg(feature = "VK_KHR_display")]
        get_physical_device_display_plane_properties: PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR(stub_get_physical_device_display_plane_properties_khr),
        #[cfg(feature = "VK_KHR_display")]
        get_display_plane_supported_displays: PFN_vkGetDisplayPlaneSupportedDisplaysKHR(stub_get_display_plane_supported_displays_khr),
        #[cfg(feature = "VK_KHR_display")]
        get_display_mode_properties: PFN_vkGetDisplayModePropertiesKHR(stub_get_display_mode_properties_khr),
        #[cfg(feature = "VK_KHR_display")]
        create_display_mode: PFN_vkCreateDisplayModeKHR(stub_create_display_mode_khr),
        #[cfg(feature = "VK_KHR_display")]
        get_display_plane_capabilities: PFN_vkGetDisplayPlaneCapabilitiesKHR(stub_get_display_plane_capabilities_khr),
        #[cfg(all(feature = "VK_KHR_display", feature = "VK_KHR_surface"))]
        create_display_plane_surface: PFN_vkCreateDisplayPlaneSurfaceKHR(stub_create_display_plane_surface_khr),
        #[cfg(feature = "Allow1_2APIs")]
        create_render_pass2: PFN_vkCreateRenderPass2(stub_create_render_pass2),
        #[cfg(feature = "Allow1_2APIs")]
        cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2(stub_cmd_begin_render_pass2),
        #[cfg(feature = "Allow1_2APIs")]
        cmd_next_subpass2: PFN_vkCmdNextSubpass2(stub_cmd_next_subpass2),
        #[cfg(feature = "Allow1_2APIs")]
        cmd_end_render_pass2: PFN_vkCmdEndRenderPass2(stub_cmd_end_render_pass2),
        #[cfg(feature = "Allow1_3APIs")]
        cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2(stub_cmd_pipeline_barrier2),
        #[cfg(feature = "Allow1_3APIs")]
        queue_submit2: PFN_vkQueueSubmit2(stub_queue_submit2)
    };
    #[inline(always)] pub(crate) fn reset() { unsafe { FPTBL = Self::INIT; } }
}

#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_instance(create_info: *const VkInstanceCreateInfo, allocator: *const VkAllocationCallbacks, instance_out: *mut VkInstance) -> VkResult {
    let fp: PFN_vkCreateInstance = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateInstance>::NAME_CSTR);
    FPTBL.create_instance = fp;
    (fp.0)(create_info, allocator, instance_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_instance(instance: VkInstance, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyInstance = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyInstance>::NAME_CSTR);
    FPTBL.destroy_instance = fp;
    (fp.0)(instance, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_physical_devices(instance: VkInstance, physical_devices_count_out: *mut u32, physical_devices_out: *mut VkPhysicalDevice) -> VkResult {
    let fp: PFN_vkEnumeratePhysicalDevices = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkEnumeratePhysicalDevices>::NAME_CSTR);
    FPTBL.enumerate_physical_devices = fp;
    (fp.0)(instance, physical_devices_count_out, physical_devices_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_features(physical_device: VkPhysicalDevice, features_out: *mut VkPhysicalDeviceFeatures) {
    let fp: PFN_vkGetPhysicalDeviceFeatures = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceFeatures>::NAME_CSTR);
    FPTBL.get_physical_device_features = fp;
    (fp.0)(physical_device, features_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, format_properties_out: *mut VkFormatProperties) {
    let fp: PFN_vkGetPhysicalDeviceFormatProperties = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceFormatProperties>::NAME_CSTR);
    FPTBL.get_physical_device_format_properties = fp;
    (fp.0)(physical_device, format, format_properties_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_image_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, image_type: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, image_format_properties_out: *mut VkImageFormatProperties) -> VkResult {
    let fp: PFN_vkGetPhysicalDeviceImageFormatProperties = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceImageFormatProperties>::NAME_CSTR);
    FPTBL.get_physical_device_image_format_properties = fp;
    (fp.0)(physical_device, format, image_type, tiling, usage, flags, image_format_properties_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_properties(physical_device: VkPhysicalDevice, properties_out: *mut VkPhysicalDeviceProperties) {
    let fp: PFN_vkGetPhysicalDeviceProperties = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceProperties>::NAME_CSTR);
    FPTBL.get_physical_device_properties = fp;
    (fp.0)(physical_device, properties_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_queue_family_properties(physical_device: VkPhysicalDevice, queue_family_properties_count_out: *mut u32, queue_family_properties_out: *mut VkQueueFamilyProperties) {
    let fp: PFN_vkGetPhysicalDeviceQueueFamilyProperties = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceQueueFamilyProperties>::NAME_CSTR);
    FPTBL.get_physical_device_queue_family_properties = fp;
    (fp.0)(physical_device, queue_family_properties_count_out, queue_family_properties_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_memory_properties(physical_device: VkPhysicalDevice, memory_properties_out: *mut VkPhysicalDeviceMemoryProperties) {
    let fp: PFN_vkGetPhysicalDeviceMemoryProperties = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceMemoryProperties>::NAME_CSTR);
    FPTBL.get_physical_device_memory_properties = fp;
    (fp.0)(physical_device, memory_properties_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_instance_proc_addr(instance: VkInstance, name: *const c_char) -> Option<PFN_vkVoidFunction> {
    let fp: PFN_vkGetInstanceProcAddr = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetInstanceProcAddr>::NAME_CSTR);
    FPTBL.get_instance_proc_addr = fp;
    (fp.0)(instance, name)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_device_proc_addr(device: VkDevice, name: *const c_char) -> Option<PFN_vkVoidFunction> {
    let fp: PFN_vkGetDeviceProcAddr = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetDeviceProcAddr>::NAME_CSTR);
    FPTBL.get_device_proc_addr = fp;
    (fp.0)(device, name)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_device(physical_device: VkPhysicalDevice, create_info: *const VkDeviceCreateInfo, allocator: *const VkAllocationCallbacks, device_out: *mut VkDevice) -> VkResult {
    let fp: PFN_vkCreateDevice = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateDevice>::NAME_CSTR);
    FPTBL.create_device = fp;
    (fp.0)(physical_device, create_info, allocator, device_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_device(device: VkDevice, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyDevice = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyDevice>::NAME_CSTR);
    FPTBL.destroy_device = fp;
    (fp.0)(device, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_instance_extension_properties(layer_name: *const c_char, property_count_out: *mut u32, properties_out: *mut VkExtensionProperties) -> VkResult {
    let fp: PFN_vkEnumerateInstanceExtensionProperties = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkEnumerateInstanceExtensionProperties>::NAME_CSTR);
    FPTBL.enumerate_instance_extension_properties = fp;
    (fp.0)(layer_name, property_count_out, properties_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_device_extension_properties(physical_device: VkPhysicalDevice, layer_name: *const c_char, property_count_out: *mut u32, properties_out: *mut VkExtensionProperties) -> VkResult {
    let fp: PFN_vkEnumerateDeviceExtensionProperties = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkEnumerateDeviceExtensionProperties>::NAME_CSTR);
    FPTBL.enumerate_device_extension_properties = fp;
    (fp.0)(physical_device, layer_name, property_count_out, properties_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_instance_layer_properties(property_count_out: *mut u32, properties_out: *mut VkLayerProperties) -> VkResult {
    let fp: PFN_vkEnumerateInstanceLayerProperties = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkEnumerateInstanceLayerProperties>::NAME_CSTR);
    FPTBL.enumerate_instance_layer_properties = fp;
    (fp.0)(property_count_out, properties_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_device_layer_properties(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkLayerProperties) -> VkResult {
    let fp: PFN_vkEnumerateDeviceLayerProperties = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkEnumerateDeviceLayerProperties>::NAME_CSTR);
    FPTBL.enumerate_device_layer_properties = fp;
    (fp.0)(physical_device, property_count_out, properties_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_device_queue(device: VkDevice, queue_family_index: u32, queue_index: u32, queue_out: *mut VkQueue) {
    let fp: PFN_vkGetDeviceQueue = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetDeviceQueue>::NAME_CSTR);
    FPTBL.get_device_queue = fp;
    (fp.0)(device, queue_family_index, queue_index, queue_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_queue_submit(queue: VkQueue, submit_count: u32, submits: *const VkSubmitInfo, fence: VkFence) -> VkResult {
    let fp: PFN_vkQueueSubmit = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkQueueSubmit>::NAME_CSTR);
    FPTBL.queue_submit = fp;
    (fp.0)(queue, submit_count, submits, fence)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_queue_wait_idle(queue: VkQueue) -> VkResult {
    let fp: PFN_vkQueueWaitIdle = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkQueueWaitIdle>::NAME_CSTR);
    FPTBL.queue_wait_idle = fp;
    (fp.0)(queue)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_device_wait_idle(device: VkDevice) -> VkResult {
    let fp: PFN_vkDeviceWaitIdle = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDeviceWaitIdle>::NAME_CSTR);
    FPTBL.device_wait_idle = fp;
    (fp.0)(device)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_allocate_memory(device: VkDevice, allocate_info: *const VkMemoryAllocateInfo, allocator: *const VkAllocationCallbacks, memory_out: *mut VkDeviceMemory) -> VkResult {
    let fp: PFN_vkAllocateMemory = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkAllocateMemory>::NAME_CSTR);
    FPTBL.allocate_memory = fp;
    (fp.0)(device, allocate_info, allocator, memory_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_free_memory(device: VkDevice, memory: VkDeviceMemory, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkFreeMemory = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkFreeMemory>::NAME_CSTR);
    FPTBL.free_memory = fp;
    (fp.0)(device, memory, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_map_memory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, data_ptr_out: *mut *mut c_void) -> VkResult {
    let fp: PFN_vkMapMemory = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkMapMemory>::NAME_CSTR);
    FPTBL.map_memory = fp;
    (fp.0)(device, memory, offset, size, flags, data_ptr_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_unmap_memory(device: VkDevice, memory: VkDeviceMemory) {
    let fp: PFN_vkUnmapMemory = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkUnmapMemory>::NAME_CSTR);
    FPTBL.unmap_memory = fp;
    (fp.0)(device, memory)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_flush_mapped_memory_ranges(device: VkDevice, memory_range_count: u32, memory_ranges: *const VkMappedMemoryRange) -> VkResult {
    let fp: PFN_vkFlushMappedMemoryRanges = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkFlushMappedMemoryRanges>::NAME_CSTR);
    FPTBL.flush_mapped_memory_ranges = fp;
    (fp.0)(device, memory_range_count, memory_ranges)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_invalidate_mapped_memory_ranges(device: VkDevice, memory_range_count: u32, memory_ranges: *const VkMappedMemoryRange) -> VkResult {
    let fp: PFN_vkInvalidateMappedMemoryRanges = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkInvalidateMappedMemoryRanges>::NAME_CSTR);
    FPTBL.invalidate_mapped_memory_ranges = fp;
    (fp.0)(device, memory_range_count, memory_ranges)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_device_memory_commitment(device: VkDevice, memory: VkDeviceMemory, committed_memory_bytes_out: *mut VkDeviceSize) {
    let fp: PFN_vkGetDeviceMemoryCommitment = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetDeviceMemoryCommitment>::NAME_CSTR);
    FPTBL.get_device_memory_commitment = fp;
    (fp.0)(device, memory, committed_memory_bytes_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_bind_buffer_memory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memory_offset: VkDeviceSize) -> VkResult {
    let fp: PFN_vkBindBufferMemory = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkBindBufferMemory>::NAME_CSTR);
    FPTBL.bind_buffer_memory = fp;
    (fp.0)(device, buffer, memory, memory_offset)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_bind_image_memory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memory_offset: VkDeviceSize) -> VkResult {
    let fp: PFN_vkBindImageMemory = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkBindImageMemory>::NAME_CSTR);
    FPTBL.bind_image_memory = fp;
    (fp.0)(device, image, memory, memory_offset)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_buffer_memory_requirements(device: VkDevice, buffer: VkBuffer, memory_requirements_out: *mut VkMemoryRequirements) {
    let fp: PFN_vkGetBufferMemoryRequirements = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetBufferMemoryRequirements>::NAME_CSTR);
    FPTBL.get_buffer_memory_requirements = fp;
    (fp.0)(device, buffer, memory_requirements_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_image_memory_requirements(device: VkDevice, image: VkImage, memory_requirements_out: *mut VkMemoryRequirements) {
    let fp: PFN_vkGetImageMemoryRequirements = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetImageMemoryRequirements>::NAME_CSTR);
    FPTBL.get_image_memory_requirements = fp;
    (fp.0)(device, image, memory_requirements_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_image_sparse_memory_requirements(device: VkDevice, image: VkImage, sparse_memory_requirement_count_out: *mut u32, sparse_memory_requirements_out: *mut VkSparseImageMemoryRequirements) {
    let fp: PFN_vkGetImageSparseMemoryRequirements = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetImageSparseMemoryRequirements>::NAME_CSTR);
    FPTBL.get_image_sparse_memory_requirements = fp;
    (fp.0)(device, image, sparse_memory_requirement_count_out, sparse_memory_requirements_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_sparse_image_format_properties(physical_device: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling, property_count_out: *mut u32, properties_out: *mut VkSparseImageFormatProperties) {
    let fp: PFN_vkGetPhysicalDeviceSparseImageFormatProperties = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceSparseImageFormatProperties>::NAME_CSTR);
    FPTBL.get_physical_device_sparse_image_format_properties = fp;
    (fp.0)(physical_device, format, r#type, samples, usage, tiling, property_count_out, properties_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_queue_bind_sparse(queue: VkQueue, bind_info_count: u32, bind_info: *const VkBindSparseInfo, fence: VkFence) -> VkResult {
    let fp: PFN_vkQueueBindSparse = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkQueueBindSparse>::NAME_CSTR);
    FPTBL.queue_bind_sparse = fp;
    (fp.0)(queue, bind_info_count, bind_info, fence)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_fence(device: VkDevice, create_info: *const VkFenceCreateInfo, allocator: *const VkAllocationCallbacks, fence_out: *mut VkFence) -> VkResult {
    let fp: PFN_vkCreateFence = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateFence>::NAME_CSTR);
    FPTBL.create_fence = fp;
    (fp.0)(device, create_info, allocator, fence_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_fence(device: VkDevice, fence: VkFence, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyFence = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyFence>::NAME_CSTR);
    FPTBL.destroy_fence = fp;
    (fp.0)(device, fence, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_reset_fences(device: VkDevice, fence_count: u32, fences: *const VkFence) -> VkResult {
    let fp: PFN_vkResetFences = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkResetFences>::NAME_CSTR);
    FPTBL.reset_fences = fp;
    (fp.0)(device, fence_count, fences)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_fence_status(device: VkDevice, fence: VkFence) -> VkResult {
    let fp: PFN_vkGetFenceStatus = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetFenceStatus>::NAME_CSTR);
    FPTBL.get_fence_status = fp;
    (fp.0)(device, fence)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_wait_for_fences(device: VkDevice, fence_count: u32, fences: *const VkFence, wait_all: VkBool32, timeout: u64) -> VkResult {
    let fp: PFN_vkWaitForFences = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkWaitForFences>::NAME_CSTR);
    FPTBL.wait_for_fences = fp;
    (fp.0)(device, fence_count, fences, wait_all, timeout)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_semaphore(device: VkDevice, create_info: *const VkSemaphoreCreateInfo, allocator: *const VkAllocationCallbacks, semaphore_out: *mut VkSemaphore) -> VkResult {
    let fp: PFN_vkCreateSemaphore = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateSemaphore>::NAME_CSTR);
    FPTBL.create_semaphore = fp;
    (fp.0)(device, create_info, allocator, semaphore_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_semaphore(device: VkDevice, semaphore: VkSemaphore, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroySemaphore = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroySemaphore>::NAME_CSTR);
    FPTBL.destroy_semaphore = fp;
    (fp.0)(device, semaphore, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_event(device: VkDevice, create_info: *const VkEventCreateInfo, allocator: *const VkAllocationCallbacks, event_out: *mut VkEvent) -> VkResult {
    let fp: PFN_vkCreateEvent = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateEvent>::NAME_CSTR);
    FPTBL.create_event = fp;
    (fp.0)(device, create_info, allocator, event_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_event(device: VkDevice, event: VkEvent, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyEvent = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyEvent>::NAME_CSTR);
    FPTBL.destroy_event = fp;
    (fp.0)(device, event, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_event_status(device: VkDevice, event: VkEvent) -> VkResult {
    let fp: PFN_vkGetEventStatus = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetEventStatus>::NAME_CSTR);
    FPTBL.get_event_status = fp;
    (fp.0)(device, event)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_set_event(device: VkDevice, event: VkEvent) -> VkResult {
    let fp: PFN_vkSetEvent = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkSetEvent>::NAME_CSTR);
    FPTBL.set_event = fp;
    (fp.0)(device, event)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_reset_event(device: VkDevice, event: VkEvent) -> VkResult {
    let fp: PFN_vkResetEvent = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkResetEvent>::NAME_CSTR);
    FPTBL.reset_event = fp;
    (fp.0)(device, event)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_query_pool(device: VkDevice, create_info: *const VkQueryPoolCreateInfo, allocator: *const VkAllocationCallbacks, query_pool_out: *mut VkQueryPool) -> VkResult {
    let fp: PFN_vkCreateQueryPool = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateQueryPool>::NAME_CSTR);
    FPTBL.create_query_pool = fp;
    (fp.0)(device, create_info, allocator, query_pool_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_query_pool(device: VkDevice, query_pool: VkQueryPool, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyQueryPool = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyQueryPool>::NAME_CSTR);
    FPTBL.destroy_query_pool = fp;
    (fp.0)(device, query_pool, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_query_pool_results(device: VkDevice, query_pool: VkQueryPool, first_query: u32, query_count: u32, data_size: usize, data_out: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult {
    let fp: PFN_vkGetQueryPoolResults = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetQueryPoolResults>::NAME_CSTR);
    FPTBL.get_query_pool_results = fp;
    (fp.0)(device, query_pool, first_query, query_count, data_size, data_out, stride, flags)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_buffer(device: VkDevice, create_info: *const VkBufferCreateInfo, allocator: *const VkAllocationCallbacks, buffer_out: *mut VkBuffer) -> VkResult {
    let fp: PFN_vkCreateBuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateBuffer>::NAME_CSTR);
    FPTBL.create_buffer = fp;
    (fp.0)(device, create_info, allocator, buffer_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_buffer(device: VkDevice, buffer: VkBuffer, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyBuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyBuffer>::NAME_CSTR);
    FPTBL.destroy_buffer = fp;
    (fp.0)(device, buffer, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_buffer_view(device: VkDevice, create_info: *const VkBufferViewCreateInfo, allocator: *const VkAllocationCallbacks, view_out: *mut VkBufferView) -> VkResult {
    let fp: PFN_vkCreateBufferView = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateBufferView>::NAME_CSTR);
    FPTBL.create_buffer_view = fp;
    (fp.0)(device, create_info, allocator, view_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_buffer_view(device: VkDevice, buffer_view: VkBufferView, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyBufferView = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyBufferView>::NAME_CSTR);
    FPTBL.destroy_buffer_view = fp;
    (fp.0)(device, buffer_view, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_image(device: VkDevice, create_info: *const VkImageCreateInfo, allocator: *const VkAllocationCallbacks, image_out: *mut VkImage) -> VkResult {
    let fp: PFN_vkCreateImage = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateImage>::NAME_CSTR);
    FPTBL.create_image = fp;
    (fp.0)(device, create_info, allocator, image_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_image(device: VkDevice, image: VkImage, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyImage = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyImage>::NAME_CSTR);
    FPTBL.destroy_image = fp;
    (fp.0)(device, image, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_image_subresource_layout(device: VkDevice, image: VkImage, subresource: *const VkImageSubresource, layout_out: *mut VkSubresourceLayout) {
    let fp: PFN_vkGetImageSubresourceLayout = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetImageSubresourceLayout>::NAME_CSTR);
    FPTBL.get_image_subresource_layout = fp;
    (fp.0)(device, image, subresource, layout_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_image_view(device: VkDevice, create_info: *const VkImageViewCreateInfo, allocator: *const VkAllocationCallbacks, view_out: *mut VkImageView) -> VkResult {
    let fp: PFN_vkCreateImageView = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateImageView>::NAME_CSTR);
    FPTBL.create_image_view = fp;
    (fp.0)(device, create_info, allocator, view_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_image_view(device: VkDevice, image_view: VkImageView, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyImageView = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyImageView>::NAME_CSTR);
    FPTBL.destroy_image_view = fp;
    (fp.0)(device, image_view, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_shader_module(device: VkDevice, create_info: *const VkShaderModuleCreateInfo, allocator: *const VkAllocationCallbacks, shader_module_out: *mut VkShaderModule) -> VkResult {
    let fp: PFN_vkCreateShaderModule = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateShaderModule>::NAME_CSTR);
    FPTBL.create_shader_module = fp;
    (fp.0)(device, create_info, allocator, shader_module_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_shader_module(device: VkDevice, shader_module: VkShaderModule, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyShaderModule = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyShaderModule>::NAME_CSTR);
    FPTBL.destroy_shader_module = fp;
    (fp.0)(device, shader_module, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_pipeline_cache(device: VkDevice, create_info: *const VkPipelineCacheCreateInfo, allocator: *const VkAllocationCallbacks, pipeline_cache_out: *mut VkPipelineCache) -> VkResult {
    let fp: PFN_vkCreatePipelineCache = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreatePipelineCache>::NAME_CSTR);
    FPTBL.create_pipeline_cache = fp;
    (fp.0)(device, create_info, allocator, pipeline_cache_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_pipeline_cache(device: VkDevice, pipeline_cache: VkPipelineCache, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyPipelineCache = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyPipelineCache>::NAME_CSTR);
    FPTBL.destroy_pipeline_cache = fp;
    (fp.0)(device, pipeline_cache, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_pipeline_cache_data(device: VkDevice, pipeline_cache: VkPipelineCache, data_size_out: *mut usize, data_out: *mut c_void) -> VkResult {
    let fp: PFN_vkGetPipelineCacheData = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPipelineCacheData>::NAME_CSTR);
    FPTBL.get_pipeline_cache_data = fp;
    (fp.0)(device, pipeline_cache, data_size_out, data_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_merge_pipeline_caches(device: VkDevice, dst_cache: VkPipelineCache, src_cache_count: u32, src_caches: *const VkPipelineCache) -> VkResult {
    let fp: PFN_vkMergePipelineCaches = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkMergePipelineCaches>::NAME_CSTR);
    FPTBL.merge_pipeline_caches = fp;
    (fp.0)(device, dst_cache, src_cache_count, src_caches)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_graphics_pipelines(device: VkDevice, pipeline_cache: VkPipelineCache, create_info_count: u32, create_infos: *const VkGraphicsPipelineCreateInfo, allocator: *const VkAllocationCallbacks, pipelines_out: *mut VkPipeline) -> VkResult {
    let fp: PFN_vkCreateGraphicsPipelines = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateGraphicsPipelines>::NAME_CSTR);
    FPTBL.create_graphics_pipelines = fp;
    (fp.0)(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_compute_pipelines(device: VkDevice, pipeline_cache: VkPipelineCache, create_info_count: u32, create_infos: *const VkComputePipelineCreateInfo, allocator: *const VkAllocationCallbacks, pipelines_out: *mut VkPipeline) -> VkResult {
    let fp: PFN_vkCreateComputePipelines = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateComputePipelines>::NAME_CSTR);
    FPTBL.create_compute_pipelines = fp;
    (fp.0)(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_pipeline(device: VkDevice, pipeline: VkPipeline, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyPipeline = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyPipeline>::NAME_CSTR);
    FPTBL.destroy_pipeline = fp;
    (fp.0)(device, pipeline, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_pipeline_layout(device: VkDevice, create_info: *const VkPipelineLayoutCreateInfo, allocator: *const VkAllocationCallbacks, pipeline_layout_out: *mut VkPipelineLayout) -> VkResult {
    let fp: PFN_vkCreatePipelineLayout = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreatePipelineLayout>::NAME_CSTR);
    FPTBL.create_pipeline_layout = fp;
    (fp.0)(device, create_info, allocator, pipeline_layout_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_pipeline_layout(device: VkDevice, pipeline_layout: VkPipelineLayout, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyPipelineLayout = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyPipelineLayout>::NAME_CSTR);
    FPTBL.destroy_pipeline_layout = fp;
    (fp.0)(device, pipeline_layout, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_sampler(device: VkDevice, create_info: *const VkSamplerCreateInfo, allocator: *const VkAllocationCallbacks, sampler_out: *mut VkSampler) -> VkResult {
    let fp: PFN_vkCreateSampler = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateSampler>::NAME_CSTR);
    FPTBL.create_sampler = fp;
    (fp.0)(device, create_info, allocator, sampler_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_sampler(device: VkDevice, sampler: VkSampler, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroySampler = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroySampler>::NAME_CSTR);
    FPTBL.destroy_sampler = fp;
    (fp.0)(device, sampler, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_descriptor_set_layout(device: VkDevice, create_info: *const VkDescriptorSetLayoutCreateInfo, allocator: *const VkAllocationCallbacks, set_layout_out: *mut VkDescriptorSetLayout) -> VkResult {
    let fp: PFN_vkCreateDescriptorSetLayout = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateDescriptorSetLayout>::NAME_CSTR);
    FPTBL.create_descriptor_set_layout = fp;
    (fp.0)(device, create_info, allocator, set_layout_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_descriptor_set_layout(device: VkDevice, descriptor_set_layout: VkDescriptorSetLayout, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyDescriptorSetLayout = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyDescriptorSetLayout>::NAME_CSTR);
    FPTBL.destroy_descriptor_set_layout = fp;
    (fp.0)(device, descriptor_set_layout, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_descriptor_pool(device: VkDevice, create_info: *const VkDescriptorPoolCreateInfo, allocator: *const VkAllocationCallbacks, descriptor_pool_out: *mut VkDescriptorPool) -> VkResult {
    let fp: PFN_vkCreateDescriptorPool = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateDescriptorPool>::NAME_CSTR);
    FPTBL.create_descriptor_pool = fp;
    (fp.0)(device, create_info, allocator, descriptor_pool_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_descriptor_pool(device: VkDevice, descriptor_pool: VkDescriptorPool, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyDescriptorPool = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyDescriptorPool>::NAME_CSTR);
    FPTBL.destroy_descriptor_pool = fp;
    (fp.0)(device, descriptor_pool, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_reset_descriptor_pool(device: VkDevice, descriptor_pool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult {
    let fp: PFN_vkResetDescriptorPool = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkResetDescriptorPool>::NAME_CSTR);
    FPTBL.reset_descriptor_pool = fp;
    (fp.0)(device, descriptor_pool, flags)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_allocate_descriptor_sets(device: VkDevice, allocate_info: *const VkDescriptorSetAllocateInfo, descriptor_sets_out: *mut VkDescriptorSet) -> VkResult {
    let fp: PFN_vkAllocateDescriptorSets = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkAllocateDescriptorSets>::NAME_CSTR);
    FPTBL.allocate_descriptor_sets = fp;
    (fp.0)(device, allocate_info, descriptor_sets_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_free_descriptor_sets(device: VkDevice, descriptor_pool: VkDescriptorPool, descriptor_set_count: u32, descriptor_sets: *const VkDescriptorSet) -> VkResult {
    let fp: PFN_vkFreeDescriptorSets = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkFreeDescriptorSets>::NAME_CSTR);
    FPTBL.free_descriptor_sets = fp;
    (fp.0)(device, descriptor_pool, descriptor_set_count, descriptor_sets)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_update_descriptor_sets(device: VkDevice, descriptor_write_count: u32, descriptor_writes: *const VkWriteDescriptorSet, descriptor_copy_count: u32, descriptor_copies: *const VkCopyDescriptorSet) {
    let fp: PFN_vkUpdateDescriptorSets = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkUpdateDescriptorSets>::NAME_CSTR);
    FPTBL.update_descriptor_sets = fp;
    (fp.0)(device, descriptor_write_count, descriptor_writes, descriptor_copy_count, descriptor_copies)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_framebuffer(device: VkDevice, create_info: *const VkFramebufferCreateInfo, allocator: *const VkAllocationCallbacks, framebuffer_out: *mut VkFramebuffer) -> VkResult {
    let fp: PFN_vkCreateFramebuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateFramebuffer>::NAME_CSTR);
    FPTBL.create_framebuffer = fp;
    (fp.0)(device, create_info, allocator, framebuffer_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_framebuffer(device: VkDevice, framebuffer: VkFramebuffer, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyFramebuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyFramebuffer>::NAME_CSTR);
    FPTBL.destroy_framebuffer = fp;
    (fp.0)(device, framebuffer, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_render_pass(device: VkDevice, create_info: *const VkRenderPassCreateInfo, allocator: *const VkAllocationCallbacks, render_pass_out: *mut VkRenderPass) -> VkResult {
    let fp: PFN_vkCreateRenderPass = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateRenderPass>::NAME_CSTR);
    FPTBL.create_render_pass = fp;
    (fp.0)(device, create_info, allocator, render_pass_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_render_pass(device: VkDevice, render_pass: VkRenderPass, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyRenderPass = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyRenderPass>::NAME_CSTR);
    FPTBL.destroy_render_pass = fp;
    (fp.0)(device, render_pass, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_render_area_granularity(device: VkDevice, render_pass: VkRenderPass, granularity_out: *mut VkExtent2D) {
    let fp: PFN_vkGetRenderAreaGranularity = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetRenderAreaGranularity>::NAME_CSTR);
    FPTBL.get_render_area_granularity = fp;
    (fp.0)(device, render_pass, granularity_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_command_pool(device: VkDevice, create_info: *const VkCommandPoolCreateInfo, allocator: *const VkAllocationCallbacks, command_pool_out: *mut VkCommandPool) -> VkResult {
    let fp: PFN_vkCreateCommandPool = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateCommandPool>::NAME_CSTR);
    FPTBL.create_command_pool = fp;
    (fp.0)(device, create_info, allocator, command_pool_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_command_pool(device: VkDevice, command_pool: VkCommandPool, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroyCommandPool = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroyCommandPool>::NAME_CSTR);
    FPTBL.destroy_command_pool = fp;
    (fp.0)(device, command_pool, allocator)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_reset_command_pool(device: VkDevice, command_pool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult {
    let fp: PFN_vkResetCommandPool = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkResetCommandPool>::NAME_CSTR);
    FPTBL.reset_command_pool = fp;
    (fp.0)(device, command_pool, flags)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_allocate_command_buffers(device: VkDevice, allocate_info: *const VkCommandBufferAllocateInfo, command_buffers_out: *mut VkCommandBuffer) -> VkResult {
    let fp: PFN_vkAllocateCommandBuffers = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkAllocateCommandBuffers>::NAME_CSTR);
    FPTBL.allocate_command_buffers = fp;
    (fp.0)(device, allocate_info, command_buffers_out)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_free_command_buffers(device: VkDevice, command_pool: VkCommandPool, command_buffer_count: u32, command_buffers: *const VkCommandBuffer) {
    let fp: PFN_vkFreeCommandBuffers = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkFreeCommandBuffers>::NAME_CSTR);
    FPTBL.free_command_buffers = fp;
    (fp.0)(device, command_pool, command_buffer_count, command_buffers)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_begin_command_buffer(command_buffer: VkCommandBuffer, begin_info: *const VkCommandBufferBeginInfo) -> VkResult {
    let fp: PFN_vkBeginCommandBuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkBeginCommandBuffer>::NAME_CSTR);
    FPTBL.begin_command_buffer = fp;
    (fp.0)(command_buffer, begin_info)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_end_command_buffer(command_buffer: VkCommandBuffer) -> VkResult {
    let fp: PFN_vkEndCommandBuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkEndCommandBuffer>::NAME_CSTR);
    FPTBL.end_command_buffer = fp;
    (fp.0)(command_buffer)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_reset_command_buffer(command_buffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult {
    let fp: PFN_vkResetCommandBuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkResetCommandBuffer>::NAME_CSTR);
    FPTBL.reset_command_buffer = fp;
    (fp.0)(command_buffer, flags)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_bind_pipeline(command_buffer: VkCommandBuffer, pipeline_bind_point: VkPipelineBindPoint, pipeline: VkPipeline) {
    let fp: PFN_vkCmdBindPipeline = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdBindPipeline>::NAME_CSTR);
    FPTBL.cmd_bind_pipeline = fp;
    (fp.0)(command_buffer, pipeline_bind_point, pipeline)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_viewport(command_buffer: VkCommandBuffer, first_viewport: u32, viewport_count: u32, viewports: *const VkViewport) {
    let fp: PFN_vkCmdSetViewport = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdSetViewport>::NAME_CSTR);
    FPTBL.cmd_set_viewport = fp;
    (fp.0)(command_buffer, first_viewport, viewport_count, viewports)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_scissor(command_buffer: VkCommandBuffer, first_scissor: u32, scissor_count: u32, scissors: *const VkRect2D) {
    let fp: PFN_vkCmdSetScissor = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdSetScissor>::NAME_CSTR);
    FPTBL.cmd_set_scissor = fp;
    (fp.0)(command_buffer, first_scissor, scissor_count, scissors)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_line_width(command_buffer: VkCommandBuffer, line_width: c_float) {
    let fp: PFN_vkCmdSetLineWidth = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdSetLineWidth>::NAME_CSTR);
    FPTBL.cmd_set_line_width = fp;
    (fp.0)(command_buffer, line_width)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_depth_bias(command_buffer: VkCommandBuffer, depth_bias_constant_factor: c_float, depth_bias_clamp: c_float, depth_bias_slope_factor: c_float) {
    let fp: PFN_vkCmdSetDepthBias = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdSetDepthBias>::NAME_CSTR);
    FPTBL.cmd_set_depth_bias = fp;
    (fp.0)(command_buffer, depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_blend_constants(command_buffer: VkCommandBuffer, blend_constants: *const c_float) {
    let fp: PFN_vkCmdSetBlendConstants = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdSetBlendConstants>::NAME_CSTR);
    FPTBL.cmd_set_blend_constants = fp;
    (fp.0)(command_buffer, blend_constants)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_depth_bounds(command_buffer: VkCommandBuffer, min_depth_bounds: c_float, max_depth_bounds: c_float) {
    let fp: PFN_vkCmdSetDepthBounds = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdSetDepthBounds>::NAME_CSTR);
    FPTBL.cmd_set_depth_bounds = fp;
    (fp.0)(command_buffer, min_depth_bounds, max_depth_bounds)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_stencil_compare_mask(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, compare_mask: u32) {
    let fp: PFN_vkCmdSetStencilCompareMask = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdSetStencilCompareMask>::NAME_CSTR);
    FPTBL.cmd_set_stencil_compare_mask = fp;
    (fp.0)(command_buffer, face_mask, compare_mask)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_stencil_write_mask(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, write_mask: u32) {
    let fp: PFN_vkCmdSetStencilWriteMask = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdSetStencilWriteMask>::NAME_CSTR);
    FPTBL.cmd_set_stencil_write_mask = fp;
    (fp.0)(command_buffer, face_mask, write_mask)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_stencil_reference(command_buffer: VkCommandBuffer, face_mask: VkStencilFaceFlags, reference: u32) {
    let fp: PFN_vkCmdSetStencilReference = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdSetStencilReference>::NAME_CSTR);
    FPTBL.cmd_set_stencil_reference = fp;
    (fp.0)(command_buffer, face_mask, reference)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_bind_descriptor_sets(command_buffer: VkCommandBuffer, pipeline_bind_point: VkPipelineBindPoint, layout: VkPipelineLayout, first_set: u32, descriptor_set_count: u32, descriptor_sets: *const VkDescriptorSet, dynamic_offset_count: u32, dynamic_offsets: *const u32) {
    let fp: PFN_vkCmdBindDescriptorSets = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdBindDescriptorSets>::NAME_CSTR);
    FPTBL.cmd_bind_descriptor_sets = fp;
    (fp.0)(command_buffer, pipeline_bind_point, layout, first_set, descriptor_set_count, descriptor_sets, dynamic_offset_count, dynamic_offsets)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_bind_index_buffer(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, index_type: VkIndexType) {
    let fp: PFN_vkCmdBindIndexBuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdBindIndexBuffer>::NAME_CSTR);
    FPTBL.cmd_bind_index_buffer = fp;
    (fp.0)(command_buffer, buffer, offset, index_type)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_bind_vertex_buffers(command_buffer: VkCommandBuffer, first_binding: u32, binding_count: u32, buffers: *const VkBuffer, offsets: *const VkDeviceSize) {
    let fp: PFN_vkCmdBindVertexBuffers = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdBindVertexBuffers>::NAME_CSTR);
    FPTBL.cmd_bind_vertex_buffers = fp;
    (fp.0)(command_buffer, first_binding, binding_count, buffers, offsets)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_draw(command_buffer: VkCommandBuffer, vertex_count: u32, instance_count: u32, first_vertex: u32, first_index: u32) {
    let fp: PFN_vkCmdDraw = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdDraw>::NAME_CSTR);
    FPTBL.cmd_draw = fp;
    (fp.0)(command_buffer, vertex_count, instance_count, first_vertex, first_index)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_draw_indexed(command_buffer: VkCommandBuffer, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
    let fp: PFN_vkCmdDrawIndexed = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdDrawIndexed>::NAME_CSTR);
    FPTBL.cmd_draw_indexed = fp;
    (fp.0)(command_buffer, index_count, instance_count, first_index, vertex_offset, first_instance)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_draw_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, draw_count: u32, stride: u32) {
    let fp: PFN_vkCmdDrawIndirect = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdDrawIndirect>::NAME_CSTR);
    FPTBL.cmd_draw_indirect = fp;
    (fp.0)(command_buffer, buffer, offset, draw_count, stride)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_draw_indexed_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, draw_count: u32, stride: u32) {
    let fp: PFN_vkCmdDrawIndexedIndirect = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdDrawIndexedIndirect>::NAME_CSTR);
    FPTBL.cmd_draw_indexed_indirect = fp;
    (fp.0)(command_buffer, buffer, offset, draw_count, stride)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_dispatch(command_buffer: VkCommandBuffer, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
    let fp: PFN_vkCmdDispatch = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdDispatch>::NAME_CSTR);
    FPTBL.cmd_dispatch = fp;
    (fp.0)(command_buffer, group_count_x, group_count_y, group_count_z)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_dispatch_indirect(command_buffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize) {
    let fp: PFN_vkCmdDispatchIndirect = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdDispatchIndirect>::NAME_CSTR);
    FPTBL.cmd_dispatch_indirect = fp;
    (fp.0)(command_buffer, buffer, offset)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_copy_buffer(command_buffer: VkCommandBuffer, src_buffer: VkBuffer, dst_buffer: VkBuffer, region_count: u32, regions: *const VkBufferCopy) {
    let fp: PFN_vkCmdCopyBuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdCopyBuffer>::NAME_CSTR);
    FPTBL.cmd_copy_buffer = fp;
    (fp.0)(command_buffer, src_buffer, dst_buffer, region_count, regions)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_copy_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageCopy) {
    let fp: PFN_vkCmdCopyImage = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdCopyImage>::NAME_CSTR);
    FPTBL.cmd_copy_image = fp;
    (fp.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_blit_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageBlit, filter: VkFilter) {
    let fp: PFN_vkCmdBlitImage = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdBlitImage>::NAME_CSTR);
    FPTBL.cmd_blit_image = fp;
    (fp.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions, filter)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_copy_buffer_to_image(command_buffer: VkCommandBuffer, src_buffer: VkBuffer, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkBufferImageCopy) {
    let fp: PFN_vkCmdCopyBufferToImage = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdCopyBufferToImage>::NAME_CSTR);
    FPTBL.cmd_copy_buffer_to_image = fp;
    (fp.0)(command_buffer, src_buffer, dst_image, dst_image_layout, region_count, regions)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_copy_image_to_buffer(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_buffer: VkBuffer, region_count: u32, regions: *const VkBufferImageCopy) {
    let fp: PFN_vkCmdCopyImageToBuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdCopyImageToBuffer>::NAME_CSTR);
    FPTBL.cmd_copy_image_to_buffer = fp;
    (fp.0)(command_buffer, src_image, src_image_layout, dst_buffer, region_count, regions)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_update_buffer(command_buffer: VkCommandBuffer, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, data_size: VkDeviceSize, data: *const c_void) {
    let fp: PFN_vkCmdUpdateBuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdUpdateBuffer>::NAME_CSTR);
    FPTBL.cmd_update_buffer = fp;
    (fp.0)(command_buffer, dst_buffer, dst_offset, data_size, data)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_fill_buffer(command_buffer: VkCommandBuffer, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, size: VkDeviceSize, data: u32) {
    let fp: PFN_vkCmdFillBuffer = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdFillBuffer>::NAME_CSTR);
    FPTBL.cmd_fill_buffer = fp;
    (fp.0)(command_buffer, dst_buffer, dst_offset, size, data)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_clear_color_image(command_buffer: VkCommandBuffer, image: VkImage, image_layout: VkImageLayout, color: *const VkClearColorValue, range_count: u32, ranges: *const VkImageSubresourceRange) {
    let fp: PFN_vkCmdClearColorImage = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdClearColorImage>::NAME_CSTR);
    FPTBL.cmd_clear_color_image = fp;
    (fp.0)(command_buffer, image, image_layout, color, range_count, ranges)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_clear_depth_stencil_image(command_buffer: VkCommandBuffer, image: VkImage, image_layout: VkImageLayout, depth_stencil: *const VkClearDepthStencilValue, range_count: u32, ranges: *const VkImageSubresourceRange) {
    let fp: PFN_vkCmdClearDepthStencilImage = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdClearDepthStencilImage>::NAME_CSTR);
    FPTBL.cmd_clear_depth_stencil_image = fp;
    (fp.0)(command_buffer, image, image_layout, depth_stencil, range_count, ranges)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_clear_attachments(command_buffer: VkCommandBuffer, attachment_count: u32, attachments: *const VkClearAttachment, rect_count: u32, rects: *const VkClearRect) {
    let fp: PFN_vkCmdClearAttachments = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdClearAttachments>::NAME_CSTR);
    FPTBL.cmd_clear_attachments = fp;
    (fp.0)(command_buffer, attachment_count, attachments, rect_count, rects)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_resolve_image(command_buffer: VkCommandBuffer, src_image: VkImage, src_image_layout: VkImageLayout, dst_image: VkImage, dst_image_layout: VkImageLayout, region_count: u32, regions: *const VkImageResolve) {
    let fp: PFN_vkCmdResolveImage = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdResolveImage>::NAME_CSTR);
    FPTBL.cmd_resolve_image = fp;
    (fp.0)(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_set_event(command_buffer: VkCommandBuffer, event: VkEvent, stage_mask: VkPipelineStageFlags) {
    let fp: PFN_vkCmdSetEvent = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdSetEvent>::NAME_CSTR);
    FPTBL.cmd_set_event = fp;
    (fp.0)(command_buffer, event, stage_mask)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_reset_event(command_buffer: VkCommandBuffer, event: VkEvent, stage_mask: VkPipelineStageFlags) {
    let fp: PFN_vkCmdResetEvent = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdResetEvent>::NAME_CSTR);
    FPTBL.cmd_reset_event = fp;
    (fp.0)(command_buffer, event, stage_mask)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_wait_events(command_buffer: VkCommandBuffer, event_count: u32, events: *const VkEvent, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags, memory_barrier_count: u32, memory_barriers: *const VkMemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const VkBufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const VkImageMemoryBarrier) {
    let fp: PFN_vkCmdWaitEvents = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdWaitEvents>::NAME_CSTR);
    FPTBL.cmd_wait_events = fp;
    (fp.0)(command_buffer, event_count, events, src_stage_mask, dst_stage_mask, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_pipeline_barrier(command_buffer: VkCommandBuffer, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags, dependency_flags: VkDependencyFlags, memory_barrier_count: u32, memory_barriers: *const VkMemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const VkBufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const VkImageMemoryBarrier) {
    let fp: PFN_vkCmdPipelineBarrier = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdPipelineBarrier>::NAME_CSTR);
    FPTBL.cmd_pipeline_barrier = fp;
    (fp.0)(command_buffer, src_stage_mask, dst_stage_mask, dependency_flags, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_begin_query(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, query: u32, flags: VkQueryControlFlags) {
    let fp: PFN_vkCmdBeginQuery = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdBeginQuery>::NAME_CSTR);
    FPTBL.cmd_begin_query = fp;
    (fp.0)(command_buffer, query_pool, query, flags)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_end_query(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, query: u32) {
    let fp: PFN_vkCmdEndQuery = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdEndQuery>::NAME_CSTR);
    FPTBL.cmd_end_query = fp;
    (fp.0)(command_buffer, query_pool, query)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_reset_query_pool(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, first_query: u32, query_count: u32) {
    let fp: PFN_vkCmdResetQueryPool = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdResetQueryPool>::NAME_CSTR);
    FPTBL.cmd_reset_query_pool = fp;
    (fp.0)(command_buffer, query_pool, first_query, query_count)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_write_timestamp(command_buffer: VkCommandBuffer, pipeline_stage: VkPipelineStageFlags, query_pool: VkQueryPool, query: u32) {
    let fp: PFN_vkCmdWriteTimestamp = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdWriteTimestamp>::NAME_CSTR);
    FPTBL.cmd_write_timestamp = fp;
    (fp.0)(command_buffer, pipeline_stage, query_pool, query)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_copy_query_pool_results(command_buffer: VkCommandBuffer, query_pool: VkQueryPool, first_query: u32, query_count: u32, dst_buffer: VkBuffer, dst_offset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags) {
    let fp: PFN_vkCmdCopyQueryPoolResults = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdCopyQueryPoolResults>::NAME_CSTR);
    FPTBL.cmd_copy_query_pool_results = fp;
    (fp.0)(command_buffer, query_pool, first_query, query_count, dst_buffer, dst_offset, stride, flags)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_push_constants(command_buffer: VkCommandBuffer, layout: VkPipelineLayout, stage_flags: VkShaderStageFlags, offset: u32, size: u32, values: *const c_void) {
    let fp: PFN_vkCmdPushConstants = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdPushConstants>::NAME_CSTR);
    FPTBL.cmd_push_constants = fp;
    (fp.0)(command_buffer, layout, stage_flags, offset, size, values)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_begin_render_pass(command_buffer: VkCommandBuffer, render_pass_begin_info: *const VkRenderPassBeginInfo, contents: VkSubpassContents) {
    let fp: PFN_vkCmdBeginRenderPass = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdBeginRenderPass>::NAME_CSTR);
    FPTBL.cmd_begin_render_pass = fp;
    (fp.0)(command_buffer, render_pass_begin_info, contents)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_next_subpass(command_buffer: VkCommandBuffer, contents: VkSubpassContents) {
    let fp: PFN_vkCmdNextSubpass = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdNextSubpass>::NAME_CSTR);
    FPTBL.cmd_next_subpass = fp;
    (fp.0)(command_buffer, contents)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_end_render_pass(command_buffer: VkCommandBuffer) {
    let fp: PFN_vkCmdEndRenderPass = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdEndRenderPass>::NAME_CSTR);
    FPTBL.cmd_end_render_pass = fp;
    (fp.0)(command_buffer)
}
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_execute_commands(command_buffer: VkCommandBuffer, command_buffer_count: u32, command_buffers: *const VkCommandBuffer) {
    let fp: PFN_vkCmdExecuteCommands = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdExecuteCommands>::NAME_CSTR);
    FPTBL.cmd_execute_commands = fp;
    (fp.0)(command_buffer, command_buffer_count, command_buffers)
}
#[cfg(feature = "Allow1_1APIs")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_enumerate_instance_version(api_version: *mut u32) -> VkResult {
    let fp: PFN_vkEnumerateInstanceVersion = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkEnumerateInstanceVersion>::NAME_CSTR);
    FPTBL.enumerate_instance_version = fp;
    (fp.0)(api_version)
}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_surface_khr(instance: VkInstance, surface: VkSurfaceKHR, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroySurfaceKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroySurfaceKHR>::NAME_CSTR);
    FPTBL.destroy_surface_khr = fp;
    (fp.0)(instance, surface, allocator)
}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_surface_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, surface: VkSurfaceKHR, supported_out: *mut VkBool32) -> VkResult {
    let fp: PFN_vkGetPhysicalDeviceSurfaceSupportKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceSurfaceSupportKHR>::NAME_CSTR);
    FPTBL.get_physical_device_surface_support_khr = fp;
    (fp.0)(physical_device, queue_family_index, surface, supported_out)
}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_surface_capabilities_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, surface_capabilities_out: *mut VkSurfaceCapabilitiesKHR) -> VkResult {
    let fp: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>::NAME_CSTR);
    FPTBL.get_physical_device_surface_capabilities_khr = fp;
    (fp.0)(physical_device, surface, surface_capabilities_out)
}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_surface_formats_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, surface_format_count_out: *mut u32, surface_formats_out: *mut VkSurfaceFormatKHR) -> VkResult {
    let fp: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>::NAME_CSTR);
    FPTBL.get_physical_device_surface_formats_khr = fp;
    (fp.0)(physical_device, surface, surface_format_count_out, surface_formats_out)
}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_surface_present_modes_khr(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR, present_mode_count_out: *mut u32, present_modes_out: *mut VkPresentModeKHR) -> VkResult {
    let fp: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>::NAME_CSTR);
    FPTBL.get_physical_device_surface_present_modes_khr = fp;
    (fp.0)(physical_device, surface, present_mode_count_out, present_modes_out)
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_swapchain_khr(device: VkDevice, create_info: *const VkSwapchainCreateInfoKHR, allocator: *const VkAllocationCallbacks, swapchain_out: *mut VkSwapchainKHR) -> VkResult {
    let fp: PFN_vkCreateSwapchainKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateSwapchainKHR>::NAME_CSTR);
    FPTBL.create_swapchain_khr = fp;
    (fp.0)(device, create_info, allocator, swapchain_out)
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_destroy_swapchain_khr(device: VkDevice, swapchain: VkSwapchainKHR, allocator: *const VkAllocationCallbacks) {
    let fp: PFN_vkDestroySwapchainKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkDestroySwapchainKHR>::NAME_CSTR);
    FPTBL.destroy_swapchain_khr = fp;
    (fp.0)(device, swapchain, allocator)
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_swapchain_images_khr(device: VkDevice, swapchain: VkSwapchainKHR, swapchain_image_count_out: *mut u32, swapchain_images_out: *mut VkImage) -> VkResult {
    let fp: PFN_vkGetSwapchainImagesKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetSwapchainImagesKHR>::NAME_CSTR);
    FPTBL.get_swapchain_images_khr = fp;
    (fp.0)(device, swapchain, swapchain_image_count_out, swapchain_images_out)
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_acquire_next_image_khr(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, image_index_out: *mut u32) -> VkResult {
    let fp: PFN_vkAcquireNextImageKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkAcquireNextImageKHR>::NAME_CSTR);
    FPTBL.acquire_next_image_khr = fp;
    (fp.0)(device, swapchain, timeout, semaphore, fence, image_index_out)
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_queue_present_khr(queue: VkQueue, present_info: *const VkPresentInfoKHR) -> VkResult {
    let fp: PFN_vkQueuePresentKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkQueuePresentKHR>::NAME_CSTR);
    FPTBL.queue_present_khr = fp;
    (fp.0)(queue, present_info)
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_xlib_surface_khr(instance: VkInstance, create_info: *const VkXlibSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    let fp: PFN_vkCreateXlibSurfaceKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateXlibSurfaceKHR>::NAME_CSTR);
    FPTBL.create_xlib_surface_khr = fp;
    (fp.0)(instance, create_info, allocator, surface_out)
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_xlib_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, dpy: *mut x11::xlib::Display, visual_id: x11::xlib::VisualID) -> VkBool32 {
    let fp: PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR>::NAME_CSTR);
    FPTBL.get_physical_device_xlib_presentation_support_khr = fp;
    (fp.0)(physical_device, queue_family_index, dpy, visual_id)
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_xcb_surface_khr(instance: VkInstance, create_info: *const VkXcbSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    let fp: PFN_vkCreateXcbSurfaceKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateXcbSurfaceKHR>::NAME_CSTR);
    FPTBL.create_xcb_surface_khr = fp;
    (fp.0)(instance, create_info, allocator, surface_out)
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_xcb_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, connection: *mut xcb::ffi::xcb_connection_t, visual_id: xcb::x::VisualId) -> VkBool32 {
    let fp: PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>::NAME_CSTR);
    FPTBL.get_physical_device_xcb_presentation_support_khr = fp;
    (fp.0)(physical_device, queue_family_index, connection, visual_id)
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_wayland_surface_khr(instance: VkInstance, create_info: *const VkWaylandSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    let fp: PFN_vkCreateWaylandSurfaceKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateWaylandSurfaceKHR>::NAME_CSTR);
    FPTBL.create_wayland_surface_khr = fp;
    (fp.0)(instance, create_info, allocator, surface_out)
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_wayland_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32, display: *mut c_void) -> VkBool32 {
    let fp: PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR>::NAME_CSTR);
    FPTBL.get_physical_device_wayland_presentation_support_khr = fp;
    (fp.0)(physical_device, queue_family_index, display)
}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_android_surface_khr(instance: VkInstance, create_info: *const VkAndroidSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    let fp: PFN_vkCreateAndroidSurfaceKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateAndroidSurfaceKHR>::NAME_CSTR);
    FPTBL.create_android_surface_khr = fp;
    (fp.0)(instance, create_info, allocator, surface_out)
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_win32_surface_khr(instance: VkInstance, create_info: *const VkWin32SurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    let fp: PFN_vkCreateWin32SurfaceKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateWin32SurfaceKHR>::NAME_CSTR);
    FPTBL.create_win32_surface_khr = fp;
    (fp.0)(instance, create_info, allocator, surface_out)
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_win32_presentation_support_khr(physical_device: VkPhysicalDevice, queue_family_index: u32) -> VkBool32 {
    let fp: PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR>::NAME_CSTR);
    FPTBL.get_physical_device_win32_presentation_support_khr = fp;
    (fp.0)(physical_device, queue_family_index)
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_macos_surface_mvk(instance: VkInstance, create_info: *const VkMacOSSurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    let fp: PFN_vkCreateMacOSSurfaceMVK = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateMacOSSurfaceMVK>::NAME_CSTR);
    FPTBL.create_macos_surface_mvk = fp;
    (fp.0)(instance, create_info, allocator, surface_out)
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_display_properties_khr(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkDisplayPropertiesKHR) -> VkResult {
    let fp: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceDisplayPropertiesKHR>::NAME_CSTR);
    FPTBL.get_physical_device_display_properties_khr = fp;
    (fp.0)(physical_device, property_count_out, properties_out)
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_physical_device_display_plane_properties_khr(physical_device: VkPhysicalDevice, property_count_out: *mut u32, properties_out: *mut VkDisplayPlanePropertiesKHR) -> VkResult {
    let fp: PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR>::NAME_CSTR);
    FPTBL.get_physical_device_display_plane_properties_khr = fp;
    (fp.0)(physical_device, property_count_out, properties_out)
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_display_plane_supported_displays_khr(physical_device: VkPhysicalDevice, plane_index: u32, display_count_out: *mut u32, displays_out: *mut VkDisplayKHR) -> VkResult {
    let fp: PFN_vkGetDisplayPlaneSupportedDisplaysKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetDisplayPlaneSupportedDisplaysKHR>::NAME_CSTR);
    FPTBL.get_display_plane_supported_displays_khr = fp;
    (fp.0)(physical_device, plane_index, display_count_out, displays_out)
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_display_mode_properties_khr(physical_device: VkPhysicalDevice, display: VkDisplayKHR, property_count_out: *mut u32, properties_out: *mut VkDisplayModePropertiesKHR) -> VkResult {
    let fp: PFN_vkGetDisplayModePropertiesKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetDisplayModePropertiesKHR>::NAME_CSTR);
    FPTBL.get_display_mode_properties_khr = fp;
    (fp.0)(physical_device, display, property_count_out, properties_out)
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_display_mode_khr(physical_device: VkPhysicalDevice, display: VkDisplayKHR, create_info: *const VkDisplayModeCreateInfoKHR, allocator: *const VkAllocationCallbacks, mode_out: *mut VkDisplayModeKHR) -> VkResult {
    let fp: PFN_vkCreateDisplayModeKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateDisplayModeKHR>::NAME_CSTR);
    FPTBL.create_display_mode_khr = fp;
    (fp.0)(physical_device, display, create_info, allocator, mode_out)
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_get_display_plane_capabilities_khr(physcial_device: VkPhysicalDevice, mode: VkDisplayModeKHR, plane_index: u32, capabilities_out: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult {
    let fp: PFN_vkGetDisplayPlaneCapabilitiesKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkGetDisplayPlaneCapabilitiesKHR>::NAME_CSTR);
    FPTBL.get_display_plane_capabilities_khr = fp;
    (fp.0)(physcial_device, mode, plane_index, capabilities_out)
}
#[cfg(all(feature = "VK_KHR_display", feature = "VK_KHR_surface"))]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_display_plane_surface_khr(instance: VkInstance, create_info: *const VkDisplaySurfaceCreateInfoKHR, allocator: *const VkAllocationCallbacks, surface_out: *mut VkSurfaceKHR) -> VkResult {
    let fp: PFN_vkCreateDisplayPlaneSurfaceKHR = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateDisplayPlaneSurfaceKHR>::NAME_CSTR);
    FPTBL.create_display_plane_surface_khr = fp;
    (fp.0)(instance, create_info, allocator, surface_out)
}
#[cfg(feature = "Allow1_2APIs")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_create_render_pass2(device: VkDevice, create_info: *const VkRenderPassCreateInfo2, allocator: *const VkAllocationCallbacks, out: *mut VkRenderPass) -> VkResult {
    let fp: PFN_vkCreateRenderPass2 = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCreateRenderPass2>::NAME_CSTR);
    FPTBL.create_render_pass2 = fp;
    (fp.0)(device, create_info, allocator, out)
}
#[cfg(feature = "Allow1_2APIs")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_begin_render_pass2(command_buffer: VkCommandBuffer, begin_info: *const VkRenderPassBeginInfo, begin_subpass_info: *const VkSubpassBeginInfo) {
    let fp: PFN_vkCmdBeginRenderPass2 = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdBeginRenderPass2>::NAME_CSTR);
    FPTBL.cmd_begin_render_pass2 = fp;
    (fp.0)(command_buffer, begin_info, begin_subpass_info)
}
#[cfg(feature = "Allow1_2APIs")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_next_subpass2(command_buffer: VkCommandBuffer, begin_subpass_info: *const VkSubpassBeginInfo, end_subpass_info: *const VkSubpassEndInfo) {
    let fp: PFN_vkCmdNextSubpass2 = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdNextSubpass2>::NAME_CSTR);
    FPTBL.cmd_next_subpass2 = fp;
    (fp.0)(command_buffer, begin_subpass_info, end_subpass_info)
}
#[cfg(feature = "Allow1_2APIs")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_end_render_pass2(command_buffer: VkCommandBuffer, end_subpass_info: *const VkSubpassEndInfo) {
    let fp: PFN_vkCmdEndRenderPass2 = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdEndRenderPass2>::NAME_CSTR);
    FPTBL.cmd_end_render_pass2 = fp;
    (fp.0)(command_buffer, end_subpass_info)
}
#[cfg(feature = "Allow1_3APIs")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_cmd_pipeline_barrier2(command_buffer: VkCommandBuffer, dependency_info: *const VkDependencyInfo) {
    let fp: PFN_vkCmdPipelineBarrier2 = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkCmdPipelineBarrier2>::NAME_CSTR);
    FPTBL.cmd_pipeline_barrier2 = fp;
    (fp.0)(command_buffer, dependency_info)
}
#[cfg(feature = "Allow1_3APIs")]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn stub_queue_submit2(queue: VkQueue, submit_count: u32, submits: *const VkSubmitInfo2, fence: VkFence) -> VkResult {
    let fp: PFN_vkQueueSubmit2 = crate::resolver::get_resolver().load_function_unconstrainted(<PFN_vkQueueSubmit2>::NAME_CSTR);
    FPTBL.queue_submit2 = fp;
    (fp.0)(queue, submit_count, submits, fence)
}

