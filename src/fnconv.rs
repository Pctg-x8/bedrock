//! FFI Function Conversions

use vk::*;
use std;

pub trait FnTransmute : Sized
{
	/// Transmute unknown function pointer into another function pointer
	/// # Safety
	/// If source pointer is not pointed valid function entry point, calling returned pointer has undefined behavior
	unsafe fn from_fn(p: PFN_vkVoidFunction) -> Self;
}
macro_rules! FnTransmuteImpl
{
	(for $($t: ty),*) =>
	{
		$(impl FnTransmute for $t { unsafe fn from_fn(p: PFN_vkVoidFunction) -> Self { std::mem::transmute(p) } })*
	}
}
FnTransmuteImpl!(for PFN_vkCreateInstance, PFN_vkDestroyInstance, PFN_vkEnumeratePhysicalDevices);
FnTransmuteImpl!(for PFN_vkGetPhysicalDeviceFeatures, PFN_vkGetPhysicalDeviceFormatProperties, PFN_vkGetPhysicalDeviceImageFormatProperties,
	PFN_vkGetPhysicalDeviceProperties, PFN_vkGetPhysicalDeviceQueueFamilyProperties, PFN_vkGetPhysicalDeviceMemoryProperties);
FnTransmuteImpl!(for PFN_vkGetInstanceProcAddr, PFN_vkGetDeviceProcAddr);
FnTransmuteImpl!(for PFN_vkCreateDevice, PFN_vkDestroyDevice);
FnTransmuteImpl!(for PFN_vkEnumerateInstanceExtensionProperties, PFN_vkEnumerateDeviceExtensionProperties,
	PFN_vkEnumerateInstanceLayerProperties, PFN_vkEnumerateDeviceLayerProperties);
FnTransmuteImpl!(for PFN_vkGetDeviceQueue, PFN_vkQueueSubmit, PFN_vkQueueWaitIdle, PFN_vkDeviceWaitIdle);
FnTransmuteImpl!(for PFN_vkAllocateMemory, PFN_vkFreeMemory, PFN_vkMapMemory, PFN_vkUnmapMemory);
FnTransmuteImpl!(for extern "system" fn(VkDevice, u32, *const VkMappedMemoryRange) -> VkResult); // FlushMappedMemoryRanges, InvalidateMappedMemoryRanges
FnTransmuteImpl!(for PFN_vkGetDeviceMemoryCommitment,
	PFN_vkBindBufferMemory, PFN_vkBindImageMemory, PFN_vkGetBufferMemoryRequirements, PFN_vkGetImageMemoryRequirements, PFN_vkGetImageSparseMemoryRequirements);
FnTransmuteImpl!(for PFN_vkGetPhysicalDeviceSparseImageFormatProperties, PFN_vkQueueBindSparse);
FnTransmuteImpl!(for PFN_vkCreateFence, PFN_vkDestroyFence, PFN_vkResetFences, PFN_vkGetFenceStatus, PFN_vkWaitForFences);
FnTransmuteImpl!(for PFN_vkCreateSemaphore, PFN_vkDestroySemaphore, PFN_vkCreateEvent, PFN_vkDestroyEvent);
FnTransmuteImpl!(for extern "system" fn(VkDevice, VkEvent) -> VkResult);	// SetEvent, ResetEvent, GetEventStatus
FnTransmuteImpl!(for PFN_vkCreateQueryPool, PFN_vkDestroyQueryPool, PFN_vkGetQueryPoolResults);
FnTransmuteImpl!(for PFN_vkCreateBuffer, PFN_vkDestroyBuffer, PFN_vkCreateBufferView, PFN_vkDestroyBufferView,
	PFN_vkCreateImage, PFN_vkDestroyImage, PFN_vkCreateImageView, PFN_vkGetImageSubresourceLayout, PFN_vkDestroyImageView);
FnTransmuteImpl!(for PFN_vkCreateShaderModule, PFN_vkDestroyShaderModule, PFN_vkCreatePipelineCache, PFN_vkDestroyPipelineCache, PFN_vkGetPipelineCacheData,
	PFN_vkMergePipelineCaches, PFN_vkCreateGraphicsPipelines, PFN_vkCreateComputePipelines, PFN_vkDestroyPipeline,
	PFN_vkCreatePipelineLayout, PFN_vkDestroyPipelineLayout, PFN_vkCreateSampler, PFN_vkDestroySampler);
FnTransmuteImpl!(for PFN_vkCreateDescriptorSetLayout, PFN_vkDestroyDescriptorSetLayout, PFN_vkCreateDescriptorPool, PFN_vkDestroyDescriptorPool,
	PFN_vkResetDescriptorPool, PFN_vkAllocateDescriptorSets, PFN_vkFreeDescriptorSets, PFN_vkUpdateDescriptorSets);
FnTransmuteImpl!(for PFN_vkCreateFramebuffer, PFN_vkDestroyFramebuffer, PFN_vkCreateRenderPass, PFN_vkDestroyRenderPass, PFN_vkGetRenderAreaGranularity);
FnTransmuteImpl!(for PFN_vkCreateCommandPool, PFN_vkDestroyCommandPool, PFN_vkAllocateCommandBuffers, PFN_vkFreeCommandBuffers, PFN_vkBeginCommandBuffer, PFN_vkEndCommandBuffer, PFN_vkResetCommandBuffer);
FnTransmuteImpl!(for PFN_vkCmdBindPipeline, PFN_vkCmdSetViewport, PFN_vkCmdSetScissor, PFN_vkCmdSetLineWidth, PFN_vkCmdSetDepthBias, PFN_vkCmdSetBlendConstants, PFN_vkCmdSetDepthBounds, PFN_vkCmdSetStencilCompareMask,
	PFN_vkCmdBindDescriptorSets, PFN_vkCmdBindIndexBuffer, PFN_vkCmdBindVertexBuffers, PFN_vkCmdDraw, PFN_vkCmdDrawIndexed, PFN_vkCmdDispatch, PFN_vkCmdDispatchIndirect,
	PFN_vkCmdCopyBuffer, PFN_vkCmdCopyImage, PFN_vkCmdBlitImage, PFN_vkCmdCopyBufferToImage, PFN_vkCmdCopyImageToBuffer, PFN_vkCmdUpdateBuffer,
	PFN_vkCmdFillBuffer, PFN_vkCmdClearColorImage, PFN_vkCmdClearDepthStencilImage, PFN_vkCmdClearAttachments,
	PFN_vkCmdResolveImage, PFN_vkCmdWaitEvents, PFN_vkCmdPipelineBarrier, PFN_vkCmdEndQuery, PFN_vkCmdWriteTimestamp, PFN_vkCmdCopyQueryPoolResults,
	PFN_vkCmdPushConstants, PFN_vkCmdBeginRenderPass, PFN_vkCmdNextSubpass, PFN_vkCmdEndRenderPass, PFN_vkCmdExecuteCommands);
FnTransmuteImpl!(for extern "system" fn(VkCommandBuffer, VkBuffer, VkDeviceSize, u32, u32));	// CmdDrawIndirect, CmdDrawIndexedIndirect
FnTransmuteImpl!(for extern "system" fn(VkCommandBuffer, VkEvent, VkPipelineStageFlags));		// CmdSetEvent, CmdResetEvent
FnTransmuteImpl!(for extern "system" fn(VkCommandBuffer, VkQueryPool, u32, u32));				// CmdBeginQuery, CmdResetQueryPool
