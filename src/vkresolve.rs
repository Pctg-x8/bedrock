//! Vulkan Function Resolver

#![cfg(feature = "Implements")]
#![allow(non_snake_case)]

#[cfg(feature = "DynamicLoaded")] use libloading::*;
use crate::vk::*;
#[cfg(feature = "DynamicLoaded")]
use std::sync::{Once, ONCE_INIT};
#[cfg(feature = "DynamicLoaded")] #[cfg(unix)]
use libloading::os::unix::Symbol as RawSymbol;
#[cfg(feature = "DynamicLoaded")] #[cfg(windows)]
use libloading::os::windows::Symbol as RawSymbol;

#[cfg(feature = "VK_KHR_xlib_surface")] use x11::xlib::{Display, VisualID};
#[cfg(feature = "VK_KHR_xcb_surface")] use xcb::ffi::{xcb_connection_t, xcb_visualid_t};

use libc::*;

macro_rules! WrapAPI {
    ($xt: ident = $n: ident ( $($an: ident : $at: ty),* )) => {
        #[cfg(not(feature = "DynamicLoaded"))]
        #[inline(always)]
        #[allow(clippy::too_many_arguments)]
        unsafe fn $xt(&self, $($an: $at),*) { $n($($an),*); }
        #[cfg(feature = "DynamicLoaded")]
        unsafe fn $xt(&self, $($an: $at),*)
        {
            static mut F: Option<RawSymbol<fn($($at),*)>> = None;
            static ONCE: Once = ONCE_INIT;
            ONCE.call_once(|| F = Some(self.0.get::<fn($($at),*)>(concat!(stringify!($n), "\0").as_bytes()).unwrap().into_raw()));
            (F.as_ref().unwrap())($($an),*);
        }
    };
    ($xt: ident = $n: ident ( $($an: ident : $at: ty),* ) -> $rt: ty) => {
        #[cfg(not(feature = "DynamicLoaded"))]
        #[inline(always)]
        #[allow(clippy::too_many_arguments)]
        unsafe fn $xt(&self, $($an: $at),*) -> $rt { $n($($an),*) }
        #[cfg(feature = "DynamicLoaded")]
        unsafe fn $xt(&self, $($an: $at),*) -> $rt
        {
            static mut F: Option<RawSymbol<fn($($at),*) -> $rt>> = None;
            static ONCE: Once = ONCE_INIT;
            ONCE.call_once(|| F = Some(self.0.get::<fn($($at),*) -> $rt>(concat!(stringify!($n), "\0").as_bytes()).unwrap().into_raw()));
            (F.as_ref().unwrap())($($an),*)
        }
    };
}

// Replacement Formula(RegEx)
// * NoReturn API: pub fn (\w+)\((([^\)]|[\r\n])*)\)\s*; => WrapAPI!($1 = $1($2));
// * Return API: pub fn (\w+)\((([^\)]|[\r\n])*)\)\s*->\s*([^;\s]*)\s*; => WrapAPI!($1 = $1($2) -> $4);

use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::cell::RefCell;
thread_local!(static STATIC_RESOLVER_INITIALIZED: RefCell<bool> = RefCell::new(false));
#[cfg(not(feature = "CustomResolver"))]
static STATIC_RESOLVER: AtomicPtr<Resolver> = AtomicPtr::new(0 as *mut _);
#[cfg(feature = "CustomResolver")]
static STATIC_RESOLVER: AtomicPtr<ResolverInterface> = AtomicPtr::new(0 as *mut _);

#[cfg(feature = "CustomResolver")]
pub fn set_custom_resolver(resv: Box<ResolverInterface>)
{
    STATIC_RESOLVER_INITIALIZED.with(|f| if !*f.borrow() {
        let _ = STATIC_RESOLVER.compare_exchange(0 as *mut _, Box::into_raw(resv), Ordering::SeqCst, Ordering::Relaxed);
    });
}

pub trait ResolverInterface
{
    unsafe fn create_instance(&self, create_info: *const VkInstanceCreateInfo, allocator: *const VkAllocationCallbacks, instance: *mut VkInstance) -> VkResult;
    unsafe fn destroy_instance(&self, instance: VkInstance, allocator: *const VkAllocationCallbacks);
    unsafe fn enumerate_physical_devices(&self, instance: VkInstance, physical_device_count: *mut u32, physical_devices: *mut VkPhysicalDevice) -> VkResult;
    unsafe fn get_physical_device_features(&self, physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
    unsafe fn get_physical_device_format_properties(&self, physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
    unsafe fn get_physical_device_image_format_properties(&self, physicalDevice: VkPhysicalDevice, format: VkFormat, itype: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;
    unsafe fn get_physical_device_properties(&self, physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
    unsafe fn get_physical_device_queue_family_properties(&self, physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
    unsafe fn get_physical_device_memory_properties(&self, physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
    unsafe fn get_instance_proc_addr(&self, instance: VkInstance, pName: *const c_char) -> Option<PFN_vkVoidFunction>;
    unsafe fn get_device_proc_addr(&self, device: VkDevice, pName: *const c_char) -> Option<PFN_vkVoidFunction>;
    unsafe fn create_device(&self, physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;
    unsafe fn destroy_device(&self, device: VkDevice, pAllocator: *const VkAllocationCallbacks);
    unsafe fn enumerate_instance_extension_properties(&self, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
    unsafe fn enumerate_device_extension_properties(&self, physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
    unsafe fn enumerate_instance_layer_properties(&self, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
    unsafe fn enumerate_device_layer_properties(&self, physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
    unsafe fn get_device_queue(&self, device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
    unsafe fn queue_submit(&self, queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
    unsafe fn queue_wait_idle(&self, queue: VkQueue) -> VkResult;
    unsafe fn device_wait_idle(&self, device: VkDevice) -> VkResult;
    unsafe fn allocate_memory(&self, device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;
    unsafe fn free_memory(&self, device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
    unsafe fn map_memory(&self, device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult;
    unsafe fn unmap_memory(&self, device: VkDevice, memory: VkDeviceMemory);
    unsafe fn flush_mapped_memory_ranges(&self, device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
    unsafe fn invalidate_mapped_memory_ranges(&self, device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
    unsafe fn get_device_memory_commitment(&self, device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);
    unsafe fn bind_buffer_memory(&self, device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
    unsafe fn bind_image_memory(&self, device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
    unsafe fn get_buffer_memory_requirements(&self, device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);
    unsafe fn get_image_memory_requirements(&self, device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
    unsafe fn get_image_sparse_memory_requirements(&self, device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);
    unsafe fn get_physical_device_sparse_image_format_properties(&self, physicalDevice: VkPhysicalDevice, format: VkFormat, _type: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
    unsafe fn queue_bind_sparse(&self, queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;
    unsafe fn create_fence(&self, device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
    unsafe fn destroy_fence(&self, device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
    unsafe fn reset_fences(&self, device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
    unsafe fn get_fence_status(&self, device: VkDevice, fence: VkFence) -> VkResult;
    unsafe fn wait_for_fences(&self, device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;
    unsafe fn create_semaphore(&self, device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;
    unsafe fn destroy_semaphore(&self, device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
    unsafe fn create_event(&self, device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;
    unsafe fn destroy_event(&self, device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
    unsafe fn get_event_status(&self, device: VkDevice, event: VkEvent) -> VkResult;
    unsafe fn set_event(&self, device: VkDevice, event: VkEvent) -> VkResult;
    unsafe fn reset_event(&self, device: VkDevice, event: VkEvent) -> VkResult;
    unsafe fn create_query_pool(&self, device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;
    unsafe fn destroy_query_pool(&self, device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
    unsafe fn get_query_pool_results(&self, device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: size_t, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;
    unsafe fn create_buffer(&self, device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;
    unsafe fn destroy_buffer(&self, device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
    unsafe fn create_buffer_view(&self, device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;
    unsafe fn destroy_buffer_view(&self, device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
    unsafe fn create_image(&self, device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;
    unsafe fn destroy_image(&self, device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
    unsafe fn get_image_subresource_layout(&self, device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);
    unsafe fn create_image_view(&self, device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;
    unsafe fn destroy_image_view(&self, device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
    unsafe fn create_shader_module(&self, device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;
    unsafe fn destroy_shader_module(&self, device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
    unsafe fn create_pipeline_cache(&self, device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;
    unsafe fn destroy_pipeline_cache(&self, device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
    unsafe fn get_pipeline_cache_data(&self, device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut size_t, pData: *mut c_void) -> VkResult;
    unsafe fn merge_pipeline_caches(&self, device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;
    unsafe fn create_graphics_pipelines(&self, device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
    unsafe fn create_compute_pipelines(&self, device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
    unsafe fn destroy_pipeline(&self, device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
    unsafe fn create_pipeline_layout(&self, device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;
    unsafe fn destroy_pipeline_layout(&self, device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
    unsafe fn create_sampler(&self, device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;
    unsafe fn destroy_sampler(&self, device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
    unsafe fn create_descriptor_set_layout(&self, device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;
    unsafe fn destroy_descriptor_set_layout(&self, device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
    unsafe fn create_descriptor_pool(&self, device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;
    unsafe fn destroy_descriptor_pool(&self, device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
    unsafe fn reset_descriptor_pool(&self, device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;
    unsafe fn allocate_descriptor_sets(&self, device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;
    unsafe fn free_descriptor_sets(&self, device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;
    unsafe fn update_descriptor_sets(&self, device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
    unsafe fn create_framebuffer(&self, device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;
    unsafe fn destroy_framebuffer(&self, device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
    unsafe fn create_render_pass(&self, device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
    unsafe fn destroy_render_pass(&self, device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
    unsafe fn get_render_area_granularity(&self, device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
    unsafe fn create_command_pool(&self, device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;
    unsafe fn destroy_command_pool(&self, device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
    unsafe fn reset_command_pool(&self, device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;
    unsafe fn allocate_command_buffers(&self, device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;
    unsafe fn free_command_buffers(&self, device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    unsafe fn begin_command_buffer(&self, commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;
    unsafe fn end_command_buffer(&self, commandBuffer: VkCommandBuffer) -> VkResult;
    unsafe fn reset_command_buffer(&self, commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;

    unsafe fn cmd_bind_pipeline(&self, commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
    unsafe fn cmd_set_viewport(&self, commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);
    unsafe fn cmd_set_scissor(&self, commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);
    unsafe fn cmd_set_line_width(&self, commandBuffer: VkCommandBuffer, lineWidth: c_float);
    unsafe fn cmd_set_depth_bias(&self, commandBuffer: VkCommandBuffer, depthBiasConstantFactor: c_float, depthBiasClamp: c_float, depthBiasSlopeFactor: c_float);
    unsafe fn cmd_set_blend_constants(&self, commandBuffer: VkCommandBuffer, blendConstants: *const c_float);
    unsafe fn cmd_set_depth_bounds(&self, commandBuffer: VkCommandBuffer, minDepthBounds: c_float, maxDepthBounds: c_float);
    unsafe fn cmd_set_stencil_compare_mask(&self, commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
    unsafe fn cmd_set_stencil_write_mask(&self, commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
    unsafe fn cmd_set_stencil_reference(&self, commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
    unsafe fn cmd_bind_descriptor_sets(&self, commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);
    unsafe fn cmd_bind_index_buffer(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
    unsafe fn cmd_bind_vertex_buffers(&self, commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
    unsafe fn cmd_draw(&self, commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
    unsafe fn cmd_draw_indexed(&self, commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
    unsafe fn cmd_draw_indirect(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    unsafe fn cmd_draw_indexed_indirect(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    unsafe fn cmd_dispatch(&self, commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    unsafe fn cmd_dispatch_indirect(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
    unsafe fn cmd_copy_buffer(&self, commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
    unsafe fn cmd_copy_image(&self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);
    unsafe fn cmd_blit_image(&self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);
    unsafe fn cmd_copy_buffer_to_image(&self, commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
    unsafe fn cmd_copy_image_to_buffer(&self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);
    unsafe fn cmd_update_buffer(&self, commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void);
    unsafe fn cmd_fill_buffer(&self, ommandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);
    unsafe fn cmd_clear_color_image(&self, commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    unsafe fn cmd_clear_depth_stencil_image(&self, commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    unsafe fn cmd_clear_attachments(&self, commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);
    unsafe fn cmd_resolve_image(&self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);
    unsafe fn cmd_set_event(&self, commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    unsafe fn cmd_reset_event(&self, commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    unsafe fn cmd_wait_events(&self, commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags,
        memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBariers: *const VkImageMemoryBarrier);
    unsafe fn cmd_pipeline_barrier(&self, commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags,
        memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    unsafe fn cmd_begin_query(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);
    unsafe fn cmd_end_query(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
    unsafe fn cmd_reset_query_pool(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
    unsafe fn cmd_write_timestamp(&self, commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlags, queryPool: VkQueryPool, query: u32);
    unsafe fn cmd_copy_query_pool_results(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);
    unsafe fn cmd_push_constants(&self, commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void);
    unsafe fn cmd_begin_render_pass(&self, commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
    unsafe fn cmd_next_subpass(&self, commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
    unsafe fn cmd_end_render_pass(&self, commandBuffer: VkCommandBuffer);
    unsafe fn cmd_execute_commands(&self, commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    #[cfg(feature = "VK_KHR_push_descriptor")]
    unsafe fn cmd_push_descriptor_set_khr(&self, commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet);
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    unsafe fn push_descriptor_set_with_template_khr(&self, commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: VkPipelineLayout, set: u32, pData: *const c_void);
    #[cfg(feature = "VK_EXT_debug_marker")]
    unsafe fn cmd_debug_marker_begin_ext(&self, commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);
    #[cfg(feature = "VK_EXT_debug_marker")]
    unsafe fn cmd_debug_marker_end_ext(&self, commandBuffer: VkCommandBuffer);
    #[cfg(feature = "VK_EXT_debug_marker")]
    unsafe fn cmd_debug_marker_insert_ext(&self, commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    unsafe fn cmd_draw_indirect_count_amd(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    unsafe fn cmd_draw_indexed_indirect_count_amd(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
    #[cfg(feature = "VK_KHX_device_group")]
    unsafe fn cmd_set_device_mask_khx(&self, commandBuffer: VkCommandBuffer, deviceMask: u32);
    #[cfg(feature = "VK_KHX_device_group")]
    unsafe fn cmd_dispatch_base_khx(&self, commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    unsafe fn cmd_process_commands_nvx(&self, commandBuffer: VkCommandBuffer, pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX);
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    unsafe fn cmd_reserve_space_for_commands_nvx(&self, commandBuffer: VkCommandBuffer, pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX);
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    unsafe fn cmd_set_viewport_w_scaling_nv(&self, commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewportWScalings: *const VkViewportWScalingNV);
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    unsafe fn cmd_discard_rectangle_ext(&self, commandBuffer: VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const VkRect2D);

    // 1,1
    unsafe fn enumerate_instance_version(&self, pApiVersion: *mut u32) -> VkResult;
    unsafe fn bind_buffer_memory2(&self, device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfo) -> VkResult;
    unsafe fn bind_image_memory2(&self, device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo) -> VkResult;
    unsafe fn get_device_group_peer_memory_features(&self, device: VkDevice, heapIndex: u32, localDeviceIndex: u32,
        remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags);
    unsafe fn cmd_set_device_mask(&self, commandBuffer: VkCommandBuffer, deviceMask: u32);
    unsafe fn cmd_dispatch_base(&self, commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32,
        groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    unsafe fn get_image_memory_requirements2(&self, device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2,
        pMemoryRequirements: *mut VkMemoryRequirements2);
    unsafe fn get_buffer_memory_requirements2(&self, device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2,
        pMemoryRequirements: *mut VkMemoryRequirements2);
    unsafe fn get_image_sparse_memory_requirements2(&self, device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2,
        pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2);
    unsafe fn get_physical_device_features2(&self, physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);
    unsafe fn get_physical_device_properties2(&self, physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);
    unsafe fn get_physical_device_format_properties2(&self, physicalDevice: VkPhysicalDevice, format: VkFormat,
        pFormatProperties: *mut VkFormatProperties2);
    unsafe fn get_physical_device_image_format_properties2(&self, physicalDevice: VkPhysicalDevice,
        pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
        pImageFormatProperties: *mut VkImageFormatProperties2) -> VkResult;
    unsafe fn get_physical_device_queue_family_properties2(&self, physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);
    unsafe fn get_physical_device_memory_properties2(&self, physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);
    unsafe fn get_physical_device_sparse_image_format_properties2(&self, physicalDevice: VkPhysicalDevice,
        pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2, pPropertyCount: *mut u32,
        pProperties: *mut VkSparseImageFormatProperties2);
    unsafe fn trim_command_pool(&self, device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags);
    unsafe fn get_device_queue2(&self, device: VkDevice, pQueueInfo: *const VkDeviceQueueInfo2, pQueue: *mut VkQueue);
    unsafe fn create_sampler_ycbcr_conversion(&self, device: VkDevice, pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
        pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSamplerYcbcrConversion) -> VkResult;
    unsafe fn destroy_sampler_ycbcr_conversion(&self, device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion,
        pAllocator: *const VkAllocationCallbacks);
    // unsafe fn create_descriptor_update_template(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    //     pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate) -> VkResult;
    // unsafe fn destroy_descriptor_update_template(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    //     pAllocator: *const VkAllocationCallbacks);
    // khr equivalent
    // unsafe fn vkCreateDescriptorUpdateTemplateKHR(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    //     pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate) -> VkResult;
    // khr equivalent
    // unsafe fn vkDestroyDescriptorUpdateTemplateKHR(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    //     pAllocator: *const VkAllocationCallbacks);
    unsafe fn update_descriptor_set_with_template(&self, device: VkDevice, descriptorSet: VkDescriptorSet,
        descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pData: *const c_void);
    unsafe fn get_physical_device_external_buffer_properties(&self, physicalDevice: VkPhysicalDevice,
        pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
        pExternalBufferProperties: *mut VkExternalBufferProperties);
    unsafe fn get_physical_device_external_fence_properties(&self, physicalDevice: VkPhysicalDevice,
        pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
        pExternalFenceProperties: *mut VkExternalFenceProperties);
    unsafe fn get_physical_device_external_semaphore_properties(&self, physicalDevice: VkPhysicalDevice,
        pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
        pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties);
    unsafe fn get_descriptor_set_layout_support(&self, device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
        pSupport: *mut VkDescriptorSetLayoutSupport);

    #[cfg(feature = "VK_KHR_surface")]
    unsafe fn destroy_surface_khr(&self, instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "VK_KHR_surface")]
    unsafe fn get_physical_device_surface_support_khr(&self, physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;
    #[cfg(feature = "VK_KHR_surface")]
    unsafe fn get_physical_device_surface_capabilities_khr(&self, physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_surface")]
    unsafe fn get_physical_device_surface_formats_khr(&self, physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_surface")]
    unsafe fn get_physical_device_surface_present_modes_khr(&self, physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    unsafe fn get_physical_device_surface_capabilities2_khr(&self, physicalDevice: VkPhysicalDevice, surface_info: *const VkPhysicalDeviceSurfaceInfo2KHR, surface_capabilities: *mut VkSurfaceCapabilities2KHR) -> VkResult;

    #[cfg(feature = "VK_KHR_swapchain")]
    unsafe fn create_swapchain_khr(&self, device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_swapchain")]
    unsafe fn destroy_swapchain_khr(&self, device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);
    #[cfg(feature = "VK_KHR_swapchain")]
    unsafe fn get_swapchain_images_khr(&self, device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult;
    #[cfg(feature = "VK_KHR_swapchain")]
    unsafe fn acquire_next_image_khr(&self, device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32) -> VkResult;
    #[cfg(feature = "VK_KHR_swapchain")]
    unsafe fn queue_present_khr(&self, queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;

    #[cfg(feature = "VK_KHR_xlib_surface")]
    unsafe fn create_xlib_surface_khr(&self, instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_xlib_surface")]
    unsafe fn get_physical_device_xlib_presentation_support_khr(&self, physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut Display, visualID: VisualID) -> VkBool32;

    #[cfg(feature = "VK_KHR_xcb_surface")]
    unsafe fn create_xcb_surface_khr(&self, instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_xcb_surface")]
    unsafe fn get_physical_device_xcb_presentation_support_khr(&self, physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_connection_t, visual_id: xcb_visualid_t) -> VkBool32;
    
    #[cfg(feature = "VK_KHR_android_surface")]
    unsafe fn create_android_surface_khr(&self, instance: VkInstance, pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    
    #[cfg(feature = "VK_KHR_win32_surface")]
    unsafe fn create_win32_surface_khr(&self, instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_win32_surface")]
    unsafe fn get_physical_device_win32_presentation_support_khr(&self, physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;

    #[cfg(feature = "VK_MVK_macos_surface")]
    unsafe fn create_macos_surface_mvk(&self, instance: VkInstance, pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    #[cfg(feature = "VK_KHR_display")]
    unsafe fn get_physical_device_display_properties_khr(&self, physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn get_physical_device_display_plane_properties_khr(&self, physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn get_display_plane_supported_displays_khr(&self, physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn get_display_mode_properties_khr(&self, physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn create_display_mode_khr(&self, physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn get_display_plane_capabilities_khr(&self, physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn create_display_plane_surface_khr(&self, instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    unsafe fn import_semaphore_win32_handle_khr(&self, device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> VkResult;
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    unsafe fn get_semaphore_win32_handle_khr(&self, device: VkDevice, pGetWin32HadleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut winapi::shared::ntdef::HANDLE) -> VkResult;

    #[cfg(feature = "VK_KHR_external_memory_win32")]
    unsafe fn get_memory_win32_handle_khr(&self, device: VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut winapi::shared::ntdef::HANDLE) -> VkResult;
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    unsafe fn get_memory_win32_handle_properties_khr(&self, device: VkDevice, handleType: VkExternalMemoryHandleTypeFlags, handle: winapi::shared::ntdef::HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR) -> VkResult;
}

pub struct Resolver(#[cfg(feature = "DynamicLoaded")] Library);
#[cfg(not(feature = "CustomResolver"))]
impl Resolver
{
    pub fn get<'a>() -> &'a Self
    {
        STATIC_RESOLVER_INITIALIZED.with(|f| if !*f.borrow()
        {
            let _ = STATIC_RESOLVER.compare_exchange(std::ptr::null_mut(), Box::into_raw(Box::new(Self::new())),
                Ordering::SeqCst, Ordering::Relaxed);
            *f.borrow_mut() = true;
        });
        unsafe { &*STATIC_RESOLVER.load(Ordering::Relaxed) }
    }

    #[cfg(feature = "DynamicLoaded")]
    fn new() -> Self
    {
        #[cfg(target_os="macos")] fn libname() -> std::path::PathBuf
        {
            let mut exepath = std::env::current_exe().unwrap(); exepath.pop();
            exepath.push("libvulkan.dylib"); return exepath;
        }
        #[cfg(windows)] fn libname() -> &'static str { "vulkan-1.dll" }
        #[cfg(not(any(target_os="macos", windows)))] fn libname() -> &'static str { "libvulkan.so" }
        Library::new(&libname()).map(Resolver).expect(&format!("Unable to open libvulkan: {:?}", libname()))
    }
    #[cfg(not(feature = "DynamicLoaded"))]
    fn new() -> Self { Resolver() }
}
#[cfg(feature = "CustomRenderer")]
impl Resolver
{
    pub fn get<'a>() -> &'a ResolverInterface
    {
        unsafe { &*STATIC_RESOLVER.load(Ordering::Relaxed) }
    }
}

#[cfg(not(feature = "CustomResolver"))]
impl ResolverInterface for Resolver
{
    WrapAPI!(create_instance = vkCreateInstance(create_info: *const VkInstanceCreateInfo, alloator: *const VkAllocationCallbacks, instance: *mut VkInstance) -> VkResult);
    WrapAPI!(destroy_instance = vkDestroyInstance(instance: VkInstance, allocator: *const VkAllocationCallbacks));
    WrapAPI!(enumerate_physical_devices = vkEnumeratePhysicalDevices(instance: VkInstance, phyical_device_count: *mut u32, physical_devices: *mut VkPhysicalDevice) -> VkResult);
    WrapAPI!(get_physical_device_features = vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures));
    WrapAPI!(get_physical_device_format_properties = vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties));
    WrapAPI!(get_physical_device_image_format_properties = vkGetPhysicalDeviceImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, itype: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult);
    WrapAPI!(get_physical_device_properties = vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties));
    WrapAPI!(get_physical_device_queue_family_properties = vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties));
    WrapAPI!(get_physical_device_memory_properties = vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties));
    WrapAPI!(get_instance_proc_addr = vkGetInstanceProcAddr(instance: VkInstance, pName: *const c_char) -> Option<PFN_vkVoidFunction>);
    WrapAPI!(get_device_proc_addr = vkGetDeviceProcAddr(device: VkDevice, pName: *const c_char) -> Option<PFN_vkVoidFunction>);
    WrapAPI!(create_device = vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult);
    WrapAPI!(destroy_device = vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(enumerate_instance_extension_properties = vkEnumerateInstanceExtensionProperties(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult);
    WrapAPI!(enumerate_device_extension_properties = vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult);
    WrapAPI!(enumerate_instance_layer_properties = vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult);
    WrapAPI!(enumerate_device_layer_properties = vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult);
    WrapAPI!(get_device_queue = vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue));
    WrapAPI!(queue_submit = vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult);
    WrapAPI!(queue_wait_idle = vkQueueWaitIdle(queue: VkQueue) -> VkResult);
    WrapAPI!(device_wait_idle = vkDeviceWaitIdle(device: VkDevice) -> VkResult);
    WrapAPI!(allocate_memory = vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult);
    WrapAPI!(free_memory = vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(map_memory = vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult);
    WrapAPI!(unmap_memory = vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory));
    WrapAPI!(flush_mapped_memory_ranges = vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult);
    WrapAPI!(invalidate_mapped_memory_ranges = vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult);
    WrapAPI!(get_device_memory_commitment = vkGetDeviceMemoryCommitment(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize));
    WrapAPI!(bind_buffer_memory = vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult);
    WrapAPI!(bind_image_memory = vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult);
    WrapAPI!(get_buffer_memory_requirements = vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements));
    WrapAPI!(get_image_memory_requirements = vkGetImageMemoryRequirements(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements));
    WrapAPI!(get_image_sparse_memory_requirements = vkGetImageSparseMemoryRequirements(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements));
    WrapAPI!(get_physical_device_sparse_image_format_properties = vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, _type: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties));
    WrapAPI!(queue_bind_sparse = vkQueueBindSparse(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult);
    WrapAPI!(create_fence = vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult);
    WrapAPI!(destroy_fence = vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(reset_fences = vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult);
    WrapAPI!(get_fence_status = vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult);
    WrapAPI!(wait_for_fences = vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult);
    WrapAPI!(create_semaphore = vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult);
    WrapAPI!(destroy_semaphore = vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(create_event = vkCreateEvent(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult);
    WrapAPI!(destroy_event = vkDestroyEvent(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(get_event_status = vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult);
    WrapAPI!(set_event = vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult);
    WrapAPI!(reset_event = vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult);
    WrapAPI!(create_query_pool = vkCreateQueryPool(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult);
    WrapAPI!(destroy_query_pool = vkDestroyQueryPool(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(get_query_pool_results = vkGetQueryPoolResults(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: size_t, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult);
    WrapAPI!(create_buffer = vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult);
    WrapAPI!(destroy_buffer = vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(create_buffer_view = vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult);
    WrapAPI!(destroy_buffer_view = vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(create_image = vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult);
    WrapAPI!(destroy_image = vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(get_image_subresource_layout = vkGetImageSubresourceLayout(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout));
    WrapAPI!(create_image_view = vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult);
    WrapAPI!(destroy_image_view = vkDestroyImageView(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(create_shader_module = vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult);
    WrapAPI!(destroy_shader_module = vkDestroyShaderModule(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(create_pipeline_cache = vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult);
    WrapAPI!(destroy_pipeline_cache = vkDestroyPipelineCache(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(get_pipeline_cache_data = vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut size_t, pData: *mut c_void) -> VkResult);
    WrapAPI!(merge_pipeline_caches = vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult);
    WrapAPI!(create_graphics_pipelines = vkCreateGraphicsPipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult);
    WrapAPI!(create_compute_pipelines = vkCreateComputePipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult);
    WrapAPI!(destroy_pipeline = vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(create_pipeline_layout = vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult);
    WrapAPI!(destroy_pipeline_layout = vkDestroyPipelineLayout(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(create_sampler = vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult);
    WrapAPI!(destroy_sampler = vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(create_descriptor_set_layout = vkCreateDescriptorSetLayout(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult);
    WrapAPI!(destroy_descriptor_set_layout = vkDestroyDescriptorSetLayout(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(create_descriptor_pool = vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult);
    WrapAPI!(destroy_descriptor_pool = vkDestroyDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(reset_descriptor_pool = vkResetDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult);
    WrapAPI!(allocate_descriptor_sets = vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult);
    WrapAPI!(free_descriptor_sets = vkFreeDescriptorSets(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult);
    WrapAPI!(update_descriptor_sets = vkUpdateDescriptorSets(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet));
    WrapAPI!(create_framebuffer = vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult);
    WrapAPI!(destroy_framebuffer = vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(create_render_pass = vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult);
    WrapAPI!(destroy_render_pass = vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(get_render_area_granularity = vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D));
    WrapAPI!(create_command_pool = vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult);
    WrapAPI!(destroy_command_pool = vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(reset_command_pool = vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult);
    WrapAPI!(allocate_command_buffers = vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult);
    WrapAPI!(free_command_buffers = vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer));
    WrapAPI!(begin_command_buffer = vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult);
    WrapAPI!(end_command_buffer = vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult);
    WrapAPI!(reset_command_buffer = vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult);

    WrapAPI!(cmd_bind_pipeline = vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline));
    WrapAPI!(cmd_set_viewport = vkCmdSetViewport(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport));
    WrapAPI!(cmd_set_scissor = vkCmdSetScissor(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D));
    WrapAPI!(cmd_set_line_width = vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: c_float));
    WrapAPI!(cmd_set_depth_bias = vkCmdSetDepthBias(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: c_float, depthBiasClamp: c_float, depthBiasSlopeFactor: c_float));
    WrapAPI!(cmd_set_blend_constants = vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: *const c_float));
    WrapAPI!(cmd_set_depth_bounds = vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: c_float, maxDepthBounds: c_float));
    WrapAPI!(cmd_set_stencil_compare_mask = vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32));
    WrapAPI!(cmd_set_stencil_write_mask = vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32));
    WrapAPI!(cmd_set_stencil_reference = vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32));
    WrapAPI!(cmd_bind_descriptor_sets = vkCmdBindDescriptorSets(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32));
    WrapAPI!(cmd_bind_index_buffer = vkCmdBindIndexBuffer(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType));
    WrapAPI!(cmd_bind_vertex_buffers = vkCmdBindVertexBuffers(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize));
    WrapAPI!(cmd_draw = vkCmdDraw(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32));
    WrapAPI!(cmd_draw_indexed = vkCmdDrawIndexed(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32));
    WrapAPI!(cmd_draw_indirect = vkCmdDrawIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32));
    WrapAPI!(cmd_draw_indexed_indirect = vkCmdDrawIndexedIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32));
    WrapAPI!(cmd_dispatch = vkCmdDispatch(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32));
    WrapAPI!(cmd_dispatch_indirect = vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize));
    WrapAPI!(cmd_copy_buffer = vkCmdCopyBuffer(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy));
    WrapAPI!(cmd_copy_image = vkCmdCopyImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy));
    WrapAPI!(cmd_blit_image = vkCmdBlitImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter));
    WrapAPI!(cmd_copy_buffer_to_image = vkCmdCopyBufferToImage(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy));
    WrapAPI!(cmd_copy_image_to_buffer = vkCmdCopyImageToBuffer(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy));
    WrapAPI!(cmd_update_buffer = vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void));
    WrapAPI!(cmd_fill_buffer = vkCmdFillBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32));
    WrapAPI!(cmd_clear_color_image = vkCmdClearColorImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange));
    WrapAPI!(cmd_clear_depth_stencil_image = vkCmdClearDepthStencilImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange));
    WrapAPI!(cmd_clear_attachments = vkCmdClearAttachments(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect));
    WrapAPI!(cmd_resolve_image = vkCmdResolveImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve));
    WrapAPI!(cmd_set_event = vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags));
    WrapAPI!(cmd_reset_event = vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags));
    WrapAPI!(cmd_wait_events = vkCmdWaitEvents(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags,
        memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBariers: *const VkImageMemoryBarrier));
    WrapAPI!(cmd_pipeline_barrier = vkCmdPipelineBarrier(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags,
        memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier));
    WrapAPI!(cmd_begin_query = vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags));
    WrapAPI!(cmd_end_query = vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32));
    WrapAPI!(cmd_reset_query_pool = vkCmdResetQueryPool(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32));
    WrapAPI!(cmd_write_timestamp = vkCmdWriteTimestamp(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlags, queryPool: VkQueryPool, query: u32));
    WrapAPI!(cmd_copy_query_pool_results = vkCmdCopyQueryPoolResults(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags));
    WrapAPI!(cmd_push_constants = vkCmdPushConstants(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void));
    WrapAPI!(cmd_begin_render_pass = vkCmdBeginRenderPass(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents));
    WrapAPI!(cmd_next_subpass = vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents));
    WrapAPI!(cmd_end_render_pass = vkCmdEndRenderPass(commandBuffer: VkCommandBuffer));
    WrapAPI!(cmd_execute_commands = vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer));
    #[cfg(feature = "VK_KHR_push_descriptor")]
    WrapAPI!(cmd_push_descriptor_set_khr = vkCmdPushDescriptorSetKHR(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet));
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    WrapAPI!(push_descriptor_set_with_template_khr = vkCmdPushDescriptorSetWithTemplateKHR(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: VkPipelineLayout, set: u32, pData: *const c_void));
    #[cfg(feature = "VK_EXT_debug_marker")]
    WrapAPI!(cmd_debug_marker_begin_ext = vkCmdDebugMarkerBeginEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT));
    #[cfg(feature = "VK_EXT_debug_marker")]
    WrapAPI!(cmd_debug_marker_end_ext = vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer));
    #[cfg(feature = "VK_EXT_debug_marker")]
    WrapAPI!(cmd_debug_marker_insert_ext = vkCmdDebugMarkerInsertEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT));
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    WrapAPI!(cmd_draw_indirect_count_amd = vkCmdDrawIndirectCountAMD(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32));
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    WrapAPI!(cmd_draw_indexed_indirect_count_amd = vkCmdDrawIndexedIndirectCountAMD(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32));
    #[cfg(feature = "VK_KHX_device_group")]
    WrapAPI!(cmd_set_device_mask_khx = vkCmdSetDeviceMaskKHX(commandBuffer: VkCommandBuffer, deviceMask: u32));
    #[cfg(feature = "VK_KHX_device_group")]
    WrapAPI!(cmd_dispatch_base_khx = vkCmdDispatchBaseKHX(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32));
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    WrapAPI!(cmd_process_commands_nvx = vkCmdProcessCommandsNVX(commandBuffer: VkCommandBuffer, pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX));
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    WrapAPI!(cmd_reserve_space_for_commands_nvx = vkCmdReserveSpaceForCommandsNVX(commandBuffer: VkCommandBuffer, pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX));
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    WrapAPI!(cmd_set_viewport_w_scaling_nv = vkCmdSetViewportWScalingNV(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewportWScalings: *const VkViewportWScalingNV));
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    WrapAPI!(cmd_discard_rectangle_ext = vkCmdDiscardRectangleEXT(commandBuffer: VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const VkRect2D));

    // 1,1
    WrapAPI!(enumerate_instance_version = vkEnumerateInstanceVersion(pApiVersion: *mut u32) -> VkResult);
    WrapAPI!(bind_buffer_memory2 = vkBindBufferMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfo) -> VkResult);
    WrapAPI!(bind_image_memory2 = vkBindImageMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo) -> VkResult);
    WrapAPI!(get_device_group_peer_memory_features = vkGetDeviceGroupPeerMemoryFeatures(device: VkDevice, heapIndex: u32, localDeviceIndex: u32,
        remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags));
    WrapAPI!(cmd_set_device_mask = vkCmdSetDeviceMask(commandBuffer: VkCommandBuffer, deviceMask: u32));
    WrapAPI!(cmd_dispatch_base = vkCmdDispatchBase(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32,
        groupCountX: u32, groupCountY: u32, groupCountZ: u32));
    WrapAPI!(get_image_memory_requirements2 = vkGetImageMemoryRequirements2(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2,
        pMemoryRequirements: *mut VkMemoryRequirements2));
    WrapAPI!(get_buffer_memory_requirements2 = vkGetBufferMemoryRequirements2(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2,
        pMemoryRequirements: *mut VkMemoryRequirements2));
    WrapAPI!(get_image_sparse_memory_requirements2 = vkGetImageSparseMemoryRequirements2(device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2,
        pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2));
    WrapAPI!(get_physical_device_features2 = vkGetPhysicalDeviceFeatures2(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2));
    WrapAPI!(get_physical_device_properties2 = vkGetPhysicalDeviceProperties2(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2));
    WrapAPI!(get_physical_device_format_properties2 = vkGetPhysicalDeviceFormatProperties2(physicalDevice: VkPhysicalDevice, format: VkFormat,
        pFormatProperties: *mut VkFormatProperties2));
    WrapAPI!(get_physical_device_image_format_properties2 = vkGetPhysicalDeviceImageFormatProperties2(physicalDevice: VkPhysicalDevice,
        pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
        pImageFormatProperties: *mut VkImageFormatProperties2) -> VkResult);
    WrapAPI!(get_physical_device_queue_family_properties2 = vkGetPhysicalDeviceQueueFamilyProperties2(physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2));
    WrapAPI!(get_physical_device_memory_properties2 = vkGetPhysicalDeviceMemoryProperties2(physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2));
    WrapAPI!(get_physical_device_sparse_image_format_properties2 = vkGetPhysicalDeviceSparseImageFormatProperties2(physicalDevice: VkPhysicalDevice,
        pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2, pPropertyCount: *mut u32,
        pProperties: *mut VkSparseImageFormatProperties2));
    WrapAPI!(trim_command_pool = vkTrimCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags));
    WrapAPI!(get_device_queue2 = vkGetDeviceQueue2(device: VkDevice, pQueueInfo: *const VkDeviceQueueInfo2, pQueue: *mut VkQueue));
    WrapAPI!(create_sampler_ycbcr_conversion = vkCreateSamplerYcbcrConversion(device: VkDevice, pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
        pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSamplerYcbcrConversion) -> VkResult);
    WrapAPI!(destroy_sampler_ycbcr_conversion = vkDestroySamplerYcbcrConversion(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion,
        pAllocator: *const VkAllocationCallbacks));
    // WrapAPI!(create_descriptor_update_template = vkCreateDescriptorUpdateTemplate(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    //     pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate) -> VkResult);
    // WrapAPI!(destroy_descriptor_update_template = vkDestroyDescriptorUpdateTemplate(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    //     pAllocator: *const VkAllocationCallbacks));
    // khr equivalent
    // WrapAPI!(vkCreateDescriptorUpdateTemplateKHR = vkCreateDescriptorUpdateTemplateKHR(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    //     pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate) -> VkResult);
    // khr equivalent
    // WrapAPI!(vkDestroyDescriptorUpdateTemplateKHR = vkDestroyDescriptorUpdateTemplateKHR(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    //     pAllocator: *const VkAllocationCallbacks));
    WrapAPI!(update_descriptor_set_with_template = vkUpdateDescriptorSetWithTemplate(device: VkDevice, descriptorSet: VkDescriptorSet,
        descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pData: *const c_void));
    WrapAPI!(get_physical_device_external_buffer_properties = vkGetPhysicalDeviceExternalBufferProperties(physicalDevice: VkPhysicalDevice,
        pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
        pExternalBufferProperties: *mut VkExternalBufferProperties));
    WrapAPI!(get_physical_device_external_fence_properties = vkGetPhysicalDeviceExternalFenceProperties(physicalDevice: VkPhysicalDevice,
        pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
        pExternalFenceProperties: *mut VkExternalFenceProperties));
    WrapAPI!(get_physical_device_external_semaphore_properties = vkGetPhysicalDeviceExternalSemaphoreProperties(physicalDevice: VkPhysicalDevice,
        pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
        pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties));
    WrapAPI!(get_descriptor_set_layout_support = vkGetDescriptorSetLayoutSupport(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
        pSupport: *mut VkDescriptorSetLayoutSupport));
        
    #[cfg(feature = "VK_KHR_surface")]
    WrapAPI!(destroy_surface_khr = vkDestroySurfaceKHR(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks));
    #[cfg(feature = "VK_KHR_surface")]
    WrapAPI!(get_physical_device_surface_support_khr = vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult);
    #[cfg(feature = "VK_KHR_surface")]
    WrapAPI!(get_physical_device_surface_capabilities_khr = vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_surface")]
    WrapAPI!(get_physical_device_surface_formats_khr = vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_surface")]
    WrapAPI!(get_physical_device_surface_present_modes_khr = vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_surface")]
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    WrapAPI!(get_physical_device_surface_capabilities2_khr = vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice: VkPhysicalDevice, surface_info: *const VkPhysicalDeviceSurfaceInfo2KHR, surface_capabilities: *mut VkSurfaceCapabilities2KHR) -> VkResult);
    
    #[cfg(feature = "VK_KHR_swapchain")]
    WrapAPI!(create_swapchain_khr = vkCreateSwapchainKHR(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_swapchain")]
    WrapAPI!(destroy_swapchain_khr = vkDestroySwapchainKHR(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks));
    #[cfg(feature = "VK_KHR_swapchain")]
    WrapAPI!(get_swapchain_images_khr = vkGetSwapchainImagesKHR(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult);
    #[cfg(feature = "VK_KHR_swapchain")]
    WrapAPI!(acquire_next_image_khr = vkAcquireNextImageKHR(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32) -> VkResult);
    #[cfg(feature = "VK_KHR_swapchain")]
    WrapAPI!(queue_present_khr = vkQueuePresentKHR(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult);

    #[cfg(feature = "VK_KHR_xlib_surface")]
    WrapAPI!(create_xlib_surface_khr = vkCreateXlibSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_xlib_surface")]
    WrapAPI!(get_physical_device_xlib_presentation_support_khr = vkGetPhysicalDeviceXlibPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut Display, visualID: VisualID) -> VkBool32);

    #[cfg(feature = "VK_KHR_xcb_surface")]
    WrapAPI!(create_xcb_surface_khr = vkCreateXcbSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_xcb_surface")]
    WrapAPI!(get_physical_device_xcb_presentation_support_khr = vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_connection_t, visual_id: xcb_visualid_t) -> VkBool32);
    
    #[cfg(feature = "VK_KHR_android_surface")]
    WrapAPI!(create_android_surface_khr = vkCreateAndroidSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
    
    #[cfg(feature = "VK_KHR_win32_surface")]
    WrapAPI!(create_win32_surface_khr = vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_win32_surface")]
    WrapAPI!(get_physical_device_win32_presentation_support_khr = vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32);

    #[cfg(feature = "VK_MVK_macos_surface")]
    WrapAPI!(create_macos_surface_mvk = vkCreateMacOSSurfaceMVK(instance: VkInstance, pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);

    #[cfg(feature = "VK_KHR_display")]
    WrapAPI!(get_physical_device_display_properties_khr = vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_display")]
    WrapAPI!(get_physical_device_display_plane_properties_khr = vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_display")]
    WrapAPI!(get_display_plane_supported_displays_khr = vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_display")]
    WrapAPI!(get_display_mode_properties_khr = vkGetDisplayModePropertiesKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_display")]
    WrapAPI!(create_display_mode_khr = vkCreateDisplayModeKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_display")]
    WrapAPI!(get_display_plane_capabilities_khr = vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_display")]
    WrapAPI!(create_display_plane_surface_khr = vkCreateDisplayPlaneSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);

    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    WrapAPI!(import_semaphore_win32_handle_khr = vkImportSemaphoreWin32HandleKHR(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> VkResult);
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    WrapAPI!(get_semaphore_win32_handle_khr = vkGetSemaphoreWin32HandleKHR(device: VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut winapi::shared::ntdef::HANDLE) -> VkResult);

    #[cfg(feature = "VK_KHR_external_memory_win32")]
    WrapAPI!(get_memory_win32_handle_khr = vkGetMemoryWin32HandleKHR(device: VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut winapi::shared::ntdef::HANDLE) -> VkResult);
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    WrapAPI!(get_memory_win32_handle_properties_khr = vkGetMemoryWin32HandlePropertiesKHR(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlags, handle: winapi::shared::ntdef::HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR) -> VkResult);
}
